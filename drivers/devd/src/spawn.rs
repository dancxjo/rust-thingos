#![no_std]
extern crate alloc;
use abi::driver_interface::DriverEntryCtx;
use abi::types::TaskStatus;
use alloc::string::{String, ToString};
use stem::syscall::{task_poll, vfs_umount};
use stem::time::monotonic_ns;
use stem::{debug, warn};

use crate::binding::Binding;
use crate::sysfs::{device_present, SysDevice};

const INITIAL_BACKOFF_MS: u64 = 100;
const MAX_BACKOFF_MS: u64 = 5_000;

/// Whether a `ManagedDriver` was created from the symbol-based catalog or from
/// the legacy static binding table.
enum SpawnMode {
    /// Legacy mode: spawn via `main`, use `binding.driver` path.
    Legacy { driver: &'static str },
    /// Catalog mode: spawn via driver entrypoint symbol.
    Catalog { driver_path: String, entry_symbol: String },
}

pub struct ManagedDriver {
    pub slot: String,
    mode: SpawnMode,
    pub mount_path: Option<String>,
    pub pid: Option<u64>,
    /// PCI identifiers, carried for building the DriverEntryCtx.
    vendor_id: u16,
    device_id: u16,
    class_code: u32,
    restarts: u32,
    restart_after_ns: u64,
}

impl ManagedDriver {
    /// Create from the legacy static binding table (pre-catalog path).
    pub fn new(device: &SysDevice, binding: Binding, mount_path: Option<String>) -> Self {
        Self {
            slot: device.slot.clone(),
            mode: SpawnMode::Legacy { driver: binding.driver },
            mount_path,
            pid: None,
            vendor_id: device.vendor_id,
            device_id: device.device_id,
            class_code: device.class_code,
            restarts: 0,
            restart_after_ns: 0,
        }
    }

    /// Create from the symbol-based driver catalog.
    ///
    /// `driver_path` is the absolute VFS path to the binary; `entry_symbol`
    /// is the name of the driver entrypoint symbol (e.g. `thing_driver_entry_v1`).
    pub fn new_from_catalog(
        device: &SysDevice,
        driver_path: String,
        entry_symbol: String,
        mount_path: Option<String>,
    ) -> Self {
        Self {
            slot: device.slot.clone(),
            mode: SpawnMode::Catalog { driver_path, entry_symbol },
            mount_path,
            pid: None,
            vendor_id: device.vendor_id,
            device_id: device.device_id,
            class_code: device.class_code,
            restarts: 0,
            restart_after_ns: 0,
        }
    }

    pub fn ensure_running(&mut self) {
        if self.pid.is_some() {
            return;
        }
        if !device_present(&self.slot) {
            return;
        }
        if monotonic_ns() < self.restart_after_ns {
            return;
        }

        match &self.mode {
            SpawnMode::Legacy { driver } => self.spawn_legacy(driver),
            SpawnMode::Catalog { driver_path, entry_symbol } => {
                // Clone to satisfy borrow checker before calling &mut self method.
                let path = driver_path.clone();
                let sym = entry_symbol.clone();
                self.spawn_via_entrypoint(&path, &sym);
            }
        }
    }

    /// Legacy spawn: write device path into a memfd and exec via `main`.
    fn spawn_legacy(&mut self, driver: &str) {
        let full_path = alloc::format!("/sys/devices/{}", self.slot);
        let boot_fd = stem::syscall::memfd_create("driver.boot", 4096).unwrap_or(0);

        if boot_fd != 0 {
            use stem::syscall::vfs::{vfs_seek, vfs_write};
            let _ = vfs_write(boot_fd, full_path.as_bytes());
            let _ = vfs_write(boot_fd, &[0]);
            let _ = vfs_seek(boot_fd, 0, 0);
        }

        let driver_path =
            if driver.starts_with('/') { driver.to_string() } else { alloc::format!("/bin/{}", driver) };

        let boot_fd_str = alloc::format!("{}", boot_fd);
        let argv: &[&[u8]] = &[driver_path.as_bytes(), boot_fd_str.as_bytes()];

        let spawn_res = stem::syscall::spawn_process_ex(
            &driver_path,
            argv,
            &alloc::collections::BTreeMap::new(),
            stem::abi::types::stdio_mode::INHERIT,
            stem::abi::types::stdio_mode::INHERIT,
            stem::abi::types::stdio_mode::INHERIT,
            boot_fd as u64,
            &[],
        );

        match spawn_res {
            Ok(resp) => {
                debug!(
                    "DEVD: launched driver {} for {} (legacy, boot_fd={}, pid={})",
                    driver, self.slot, boot_fd, resp.child_tid
                );
                self.pid = Some(resp.child_tid);
            }
            Err(err) => {
                warn!("DEVD: failed to launch {} for {}: {:?}", driver, self.slot, err);
                self.schedule_restart();
                if boot_fd != 0 {
                    let _ = stem::syscall::vfs::vfs_close(boot_fd);
                }
            }
        }
    }

    /// Catalog spawn: build a `DriverEntryCtx`, write it into a memfd, and
    /// spawn the binary entering at `entry_symbol` instead of `main`.
    fn spawn_via_entrypoint(&mut self, driver_path: &str, entry_symbol: &str) {
        // Build the stable context payload.
        let ctx = self.build_driver_entry_ctx();
        let ctx_size = core::mem::size_of::<DriverEntryCtx>();

        let boot_fd = stem::syscall::memfd_create("driver.ctx", ctx_size).unwrap_or(0);
        if boot_fd != 0 {
            use stem::syscall::vfs::{vfs_seek, vfs_write};
            // SAFETY: ctx is a plain repr(C) struct; we serialize as raw bytes.
            let ctx_bytes = unsafe {
                core::slice::from_raw_parts(&ctx as *const DriverEntryCtx as *const u8, ctx_size)
            };
            let _ = vfs_write(boot_fd, ctx_bytes);
            let _ = vfs_seek(boot_fd, 0, 0);
        }

        let argv: &[&[u8]] = &[driver_path.as_bytes()];

        let spawn_res = stem::syscall::spawn_driver_ex(
            driver_path,
            argv,
            &alloc::collections::BTreeMap::new(),
            boot_fd as u64,
            &[],
            Some(entry_symbol),
        );

        match spawn_res {
            Ok(resp) => {
                debug!(
                    "DEVD: launched driver {} for {} (entry='{}', pid={})",
                    driver_path, self.slot, entry_symbol, resp.child_tid
                );
                self.pid = Some(resp.child_tid);
            }
            Err(err) => {
                warn!(
                    "DEVD: failed to launch {} for {} via '{}': {:?}",
                    driver_path, self.slot, entry_symbol, err
                );
                self.schedule_restart();
                if boot_fd != 0 {
                    let _ = stem::syscall::vfs::vfs_close(boot_fd);
                }
            }
        }
    }

    fn build_driver_entry_ctx(&self) -> DriverEntryCtx {
        let mut ctx = DriverEntryCtx {
            version: 1,
            vendor_id: self.vendor_id,
            device_id: self.device_id,
            class_code: self.class_code,
            _reserved0: 0,
            device_path: [0u8; 128],
        };
        let path = alloc::format!("/sys/devices/{}", self.slot);
        let bytes = path.as_bytes();
        let len = bytes.len().min(127);
        ctx.device_path[..len].copy_from_slice(&bytes[..len]);
        ctx
    }

    pub fn monitor(&mut self) {
        let Some(pid) = self.pid else {
            self.ensure_running();
            return;
        };

        match task_poll(pid) {
            Ok((TaskStatus::Dead, code)) => {
                warn!(
                    "DEVD: driver for {} exited with code {}",
                    self.slot, code
                );
                self.pid = None;
                self.cleanup_mount();
                if device_present(&self.slot) {
                    self.schedule_restart();
                    self.ensure_running();
                }
            }
            Ok(_) => {}
            Err(err) => {
                warn!(
                    "DEVD: lost pid {} for {}: {:?}",
                    pid, self.slot, err
                );
                self.pid = None;
                self.cleanup_mount();
                if device_present(&self.slot) {
                    self.schedule_restart();
                    self.ensure_running();
                }
            }
        }
    }

    pub fn mark_removed(&mut self) {
        self.pid = None;
        self.restart_after_ns = 0;
        self.cleanup_mount();
    }

    fn schedule_restart(&mut self) {
        self.restarts = self.restarts.saturating_add(1);
        let shift = self.restarts.saturating_sub(1).min(5);
        let delay_ms = (INITIAL_BACKOFF_MS << shift).min(MAX_BACKOFF_MS);
        self.restart_after_ns = monotonic_ns() + delay_ms * 1_000_000;
    }

    fn cleanup_mount(&self) {
        if let Some(path) = self.mount_path.as_deref() {
            let _ = vfs_umount(path);
        }
    }
}
