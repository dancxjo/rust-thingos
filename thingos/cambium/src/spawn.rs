extern crate alloc;
use alloc::string::{String, ToString};

use abi::driver_interface::DriverEntryCtx;
use abi::types::TaskStatus;
use stem::kinds::{DriverReadyV1, KIND_ID_THINGOS_DRIVER_READY};
use stem::syscall::message::{KindId, msg_send};
use stem::syscall::{task_poll, vfs_umount};
use stem::time::monotonic_ns;
use stem::{debug, warn};

use crate::sysfs::{SysDevice, device_present};

const INITIAL_BACKOFF_MS: u64 = 100;
const MAX_BACKOFF_MS: u64 = 5_000;

pub struct ManagedDriver {
    pub slot: String,
    driver_path: String,
    start_symbol: String,
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
    pub fn pid(&self) -> Option<u64> {
        self.pid
    }

    /// Create from the symbol-based driver catalog.
    ///
    /// `driver_path` is the absolute VFS path to the binary; `start_symbol`
    /// is the name of the driver start entrypoint symbol.
    pub fn new_from_catalog(
        device: &SysDevice,
        driver_path: String,
        start_symbol: String,
        mount_path: Option<String>,
    ) -> Self {
        Self {
            slot: device.slot.clone(),
            driver_path,
            start_symbol,
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

        // Clone to satisfy borrow checker before calling &mut self method.
        let path = self.driver_path.clone();
        let sym = self.start_symbol.clone();
        self.spawn_via_entrypoint(&path, &sym);
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
                    "CAMBIUM: launched driver {} for {} (entry='{}', pid={})",
                    driver_path, self.slot, entry_symbol, resp.child_tid
                );
                self.pid = Some(resp.child_tid);
                send_driver_ready(resp.child_tid);
            }
            Err(err) => {
                warn!(
                    "CAMBIUM: failed to launch {} for {} via '{}': {:?}",
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

        match stem::syscall::waitpid(pid as i64, abi::types::system::waitpid_flags::WNOHANG) {
            Ok((child_pid, status)) if child_pid > 0 => {
                let code = abi::signal::w_exit_status(status as u8);
                self.handle_exit(code);
            }
            Ok(_) => {}
            Err(err) => {
                warn!("CAMBIUM: lost pid {} for {}: {:?}", pid, self.slot, err);
                self.pid = None;
                self.cleanup_mount();
                if device_present(&self.slot) {
                    self.schedule_restart();
                    self.ensure_running();
                }
            }
        }
    }

    pub fn handle_exit(&mut self, code: i32) {
        warn!("CAMBIUM: driver for {} exited with code {}", self.slot, code);
        self.pid = None;
        self.cleanup_mount();
        if device_present(&self.slot) {
            self.schedule_restart();
            self.ensure_running();
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

/// Send a `DRIVER_READY` inbox message to the Sprout supervisor (our parent
/// process) so it can mark the corresponding `ManagedTask` as ready.
///
/// # Assumption
///
/// Cambium is always spawned directly by Sprout (PID 1 / the supervisor) and
/// is never reparented.  `getppid()` therefore reliably returns Sprout's PID.
/// If Cambium were ever launched from a different parent, this would send the
/// notification to the wrong process — a log warning would indicate the
/// failure.
fn send_driver_ready(driver_pid: u64) {
    let sprout_pid = stem::syscall::getppid();
    if sprout_pid == 0 {
        warn!("CAMBIUM: send_driver_ready: getppid() returned 0, cannot notify supervisor");
        return;
    }
    let msg = DriverReadyV1::new_ok(driver_pid as u32);
    let kind = KindId(KIND_ID_THINGOS_DRIVER_READY);
    match msg_send(sprout_pid, kind, msg.as_bytes()) {
        Ok(()) => {
            debug!(
                "CAMBIUM: sent DRIVER_READY to Sprout (pid={}) for driver pid={}",
                sprout_pid, driver_pid
            );
        }
        Err(err) => {
            warn!(
                "CAMBIUM: failed to send DRIVER_READY to Sprout (pid={}) for driver pid={}: {:?}",
                sprout_pid, driver_pid, err
            );
        }
    }
}
