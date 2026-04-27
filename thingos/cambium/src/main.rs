#![no_std]
#![no_main]
extern crate alloc;

mod catalog;
mod spawn;
mod sysfs;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::ops::ControlFlow;

use abi::driver_interface::DriverClass;
use abi::errors::Errno;
use abi::syscall::vfs_flags::{O_RDONLY, O_WRONLY};
use abi::vfs_watch::{flags as watch_flags, mask as watch_mask};
use catalog::Catalog;
use spawn::ManagedDriver;
use stem::kinds::KIND_ID_THINGOS_JOB_EXIT;
use stem::service_loop::{ServiceEvent, ServiceLoop};
use stem::syscall::message::KindId;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_watch_path, vfs_write};
use stem::time::Duration;
use stem::{debug, error, info, warn};
use sysfs::{SysDevice, scan_devices};

/// Periodic fallback rescan interval (milliseconds) when no events arrive.
const RECONCILE_TIMEOUT_MS: u64 = 30_000;
/// Maximum inbox payload size for `ServiceLoop` (covers `THINGOS_JOB_EXIT`
/// notifications with comfortable headroom for future control messages).
const INBOX_MAX_PAYLOAD: usize = 256;
/// `THINGOS_JOB_EXIT` notification payload layout:
/// - bytes 0-3: exited process PID / job_id (u32 LE)
/// - byte 4: state (2 = exited)
/// - byte 5: exit-code present flag (0/1)
/// - bytes 6-9: exit code (i32 LE, valid when present=1)
const JOB_EXIT_NOTIFICATION_LEN: usize = 10;
const JOB_EXIT_JOB_ID_BYTES: usize = 4;
const JOB_EXIT_STATE_OFFSET: usize = 4;
const JOB_EXIT_CODE_PRESENT_OFFSET: usize = 5;
const JOB_EXIT_CODE_OFFSET: usize = 6;
const JOB_EXIT_CODE_BYTES: usize = 4;
const JOB_EXIT_STATE_EXITED: u8 = 2;
const SPROUT_EARLY_AUDIO_MARKER: &str = "/run/sprout/audio-early";

#[stem::main]
fn main(_arg: usize) -> ! {
    stem::info!("CAMBIUM: main started");
    // ── Parse argv ──────────────────────────────────────────────────────────
    let args = get_args();

    if args.len() >= 2 {
        // Manual mode: `cambium <driver-path> [<device-slot>]`
        //
        // Reads the named binary, verifies it exports THING_DRIVER_V1, finds
        // all matching devices in /sys/devices (or the explicitly named slot),
        // spawns it via thing_driver_entry_v1, and monitors the process.
        let driver_path = &args[1];
        let slot_filter: Option<&str> = if args.len() >= 3 { Some(&args[2]) } else { None };
        run_manual_mode(driver_path, slot_filter);
    } else {
        // Daemon mode: autonomously scan /drivers and bind devices.
        run_daemon_mode();
    }
}

// ── Manual mode ─────────────────────────────────────────────────────────────

/// `cambium <driver-path> [slot]`
///
/// 1. Inspect the binary at `driver_path` for the `THING_DRIVER_V1` marker.
/// 2. Enumerate `/sys/devices` and find devices that match the driver's hints
///    (or use `slot` directly when provided).
/// 3. Spawn the driver via its `thing_driver_entry_v1` entrypoint for each
///    matching device and monitor it — restarting on exit while the device is
///    still present.
fn run_manual_mode(driver_path: &str, slot_filter: Option<&str>) -> ! {
    debug!("CAMBIUM: manual mode, driver={}", driver_path);

    // Resolve to an absolute path if needed.
    let abs_path: String = if driver_path.starts_with('/') {
        driver_path.to_string()
    } else {
        let in_drivers = alloc::format!("/drivers/{}", driver_path);
        if path_exists(&in_drivers) { in_drivers } else { alloc::format!("/bin/{}", driver_path) }
    };

    // Inspect the binary.
    let mut catalog = Catalog::new();
    let entry = match catalog.inspect_binary_path(&abs_path) {
        Some(e) => e.clone(),
        None => {
            error!("CAMBIUM: '{}' does not export THING_DRIVER_V1 or could not be read", abs_path);
            stem::syscall::exit(1);
        }
    };

    debug!(
        "CAMBIUM: driver '{}' name='{}' class={:?} vendor=0x{:04x} device=0x{:04x} class_code=0x{:06x} start='{}'",
        abs_path,
        entry.driver_name,
        entry.driver_class,
        entry.vendor_id,
        entry.device_id,
        entry.class_code,
        entry.start_symbol
    );

    // Find matching devices.
    let devices = match scan_devices() {
        Ok(d) => d,
        Err(e) => {
            error!("CAMBIUM: failed to scan /sys/devices: {:?}", e);
            stem::syscall::exit(1);
        }
    };

    let mut managed: Vec<ManagedDriver> = devices
        .into_iter()
        .filter(|d| {
            d.present
                && slot_filter.map_or(true, |s| d.slot == s)
                && entry.matches_pci(d.vendor_id, d.device_id, d.class_code)
        })
        .map(|d| {
            ManagedDriver::new_from_catalog(&d, abs_path.clone(), entry.start_symbol.clone(), None)
        })
        .collect();

    if managed.is_empty() {
        if let Some(slot) = slot_filter {
            // If an explicit slot was given, try to run the driver for it
            // regardless of the marker's match hints — the user said to do it.
            debug!(
                "CAMBIUM: no match by hints for slot '{}'; spawning anyway (explicit override)",
                slot
            );
            // Create a synthetic device record from what we know.
            let fake_device = SysDevice {
                slot: slot.to_string(),
                kind: "unknown".into(),
                vendor_id: 0,
                device_id: 0,
                class_code: 0,
                present: true,
            };
            managed.push(ManagedDriver::new_from_catalog(
                &fake_device,
                abs_path.clone(),
                entry.start_symbol.clone(),
                None,
            ));
        } else {
            warn!("CAMBIUM: no matching devices found for driver '{}'", abs_path);
            stem::syscall::exit(0);
        }
    }

    // Start all matched drivers.
    for m in &mut managed {
        m.ensure_running();
    }

    // Monitor loop — restart on exit while device is still present.
    loop {
        for m in &mut managed {
            m.monitor();
        }
        stem::time::sleep_ms(100);
    }
}

fn path_exists(path: &str) -> bool {
    match vfs_open(path, O_RDONLY) {
        Ok(fd) => {
            let _ = vfs_close(fd);
            true
        }
        Err(_) => false,
    }
}

// ── Daemon mode ──────────────────────────────────────────────────────────────

fn run_daemon_mode() -> ! {
    stem::info!("CAMBIUM: starting device discovery manager (daemon mode)");

    let mut drivers: BTreeMap<String, ManagedDriver> = BTreeMap::new();
    let mut catalog = Catalog::new();
    let mut observed_pids: BTreeMap<u64, ()> = BTreeMap::new();

    // Initial catalog scan.
    catalog.scan();

    match scan_devices() {
        Ok(devices) => reconcile_devices(&mut drivers, &catalog, devices),
        Err(err) => warn!("CAMBIUM: initial scan of /sys/devices failed: {:?}", err),
    }
    register_observers_for_running(&mut drivers, &mut observed_pids);

    // Construct the canonical Layer 3 service loop (inbox-backed).  When
    // construction fails (e.g. procfs unavailable), fall back to the
    // degraded monitor loop so that manual-mode-style behavior is still
    // preserved.
    let mut svc = match ServiceLoop::new(INBOX_MAX_PAYLOAD) {
        Ok(s) => s,
        Err(err) => {
            warn!(
                "CAMBIUM: failed to construct ServiceLoop ({:?}); running degraded monitor-only loop",
                err
            );
            run_degraded_monitor_loop(&mut drivers);
        }
    };

    let devices_watch_token =
        match vfs_watch_path("/sys/devices", watch_mask::ALL_EVENTS, watch_flags::NONBLOCK) {
            Ok(fd) => match svc.add_vfs_watch(fd) {
                Ok(token) => Some((token, fd)),
                Err(err) => {
                    warn!(
                        "CAMBIUM: failed to register /sys/devices watch with ServiceLoop: {:?}",
                        err
                    );
                    let _ = vfs_close(fd);
                    None
                }
            },
            Err(err) => {
                warn!("CAMBIUM: failed to watch /sys/devices: {:?}", err);
                None
            }
        };

    let timeout = Some(Duration::from_millis(RECONCILE_TIMEOUT_MS));

    loop {
        let mut reconcile_due = false;
        let mut messages_drained = false;

        match svc.next_event(timeout) {
            Ok(ServiceEvent::Message { kind, payload }) => {
                stem::debug!("CAMBIUM: ServiceLoop wake — inbox message");
                handle_job_exit_message(&mut drivers, &mut observed_pids, kind, payload);
                messages_drained = true;
            }
            Ok(ServiceEvent::Ready { token, event }) => {
                if event.is_error() || event.is_hangup() {
                    warn!("CAMBIUM: ServiceLoop secondary source error/hangup (token={:?})", token);
                    reconcile_due = true;
                }
                if let Some((dev_token, dev_fd)) = devices_watch_token {
                    if token == dev_token {
                        stem::debug!("CAMBIUM: ServiceLoop wake — /sys/devices watch readable");
                        if event.is_readable() {
                            drain_watch_fd(dev_fd);
                        }
                        reconcile_due = true;
                    }
                }
            }
            Ok(ServiceEvent::Timeout) => {
                stem::debug!("CAMBIUM: ServiceLoop wake — reconcile tick");
                reconcile_due = true;
            }
            Ok(ServiceEvent::InboxClosed) => {
                warn!("CAMBIUM: inbox closed; falling back to degraded monitor-only loop");
                run_degraded_monitor_loop(&mut drivers);
            }
            Err(err) => {
                warn!("CAMBIUM: ServiceLoop next_event error: {:?}", err);
                reconcile_due = true;
            }
        }

        // Batch any additional queued inbox messages so the loop matches
        // the original "drain inbox per wake" semantics.
        if messages_drained {
            let _ = svc.drain_inbox(|kind, payload| {
                handle_job_exit_message(&mut drivers, &mut observed_pids, kind, payload);
                ControlFlow::Continue(())
            });
            register_observers_for_running(&mut drivers, &mut observed_pids);
        }

        if reconcile_due {
            match scan_devices() {
                Ok(devices) => {
                    reconcile_devices(&mut drivers, &catalog, devices);
                    register_observers_for_running(&mut drivers, &mut observed_pids);
                }
                Err(err) => warn!("CAMBIUM: scan of /sys/devices failed: {:?}", err),
            }
        }
    }
}

/// Fallback loop used when no readiness primitive is available (no inbox,
/// no `/sys/devices` watch, or an unrecoverable inbox closure).  It mirrors
/// the original degraded path so behavior is preserved end-to-end.
fn run_degraded_monitor_loop(drivers: &mut BTreeMap<String, ManagedDriver>) -> ! {
    loop {
        for managed in drivers.values_mut() {
            managed.monitor();
        }
        stem::time::sleep_ms(100);
    }
}

fn register_observers_for_running(
    drivers: &mut BTreeMap<String, ManagedDriver>,
    observed_pids: &mut BTreeMap<u64, ()>,
) {
    for managed in drivers.values_mut() {
        if let Some(pid) = managed.pid() {
            if observed_pids.contains_key(&pid) {
                continue;
            }
            match register_job_observer(pid) {
                Ok(()) => {
                    observed_pids.insert(pid, ());
                }
                Err(err) => {
                    warn!("CAMBIUM: failed to register job observer for pid {}: {:?}", pid, err);
                }
            }
        }
    }
}

fn register_job_observer(pid: u64) -> Result<(), Errno> {
    let path = alloc::format!("/proc/{}/job_observer", pid);
    let fd = vfs_open(&path, O_WRONLY)?;
    let write_res = vfs_write(fd, b"1").map(|_| ());
    let _ = vfs_close(fd);
    write_res
}

fn drain_watch_fd(fd: u32) {
    let mut buf = [0u8; 512];
    loop {
        match vfs_read(fd, &mut buf) {
            Ok(0) => break,
            Ok(_) => {}
            Err(Errno::EAGAIN) => break,
            Err(err) => {
                warn!("CAMBIUM: failed reading /sys/devices watch events: {:?}", err);
                break;
            }
        }
    }
}

fn handle_job_exit_message(
    drivers: &mut BTreeMap<String, ManagedDriver>,
    observed_pids: &mut BTreeMap<u64, ()>,
    kind: KindId,
    payload: &[u8],
) {
    if kind.0 != KIND_ID_THINGOS_JOB_EXIT {
        return;
    }
    let Some((pid, code)) = decode_job_exit_notification(payload) else {
        return;
    };
    observed_pids.remove(&(pid as u64));
    for managed in drivers.values_mut() {
        if managed.pid() == Some(pid as u64) {
            managed.handle_exit(code);
            break;
        }
    }
}

fn decode_job_exit_notification(bytes: &[u8]) -> Option<(u32, i32)> {
    if bytes.len() < JOB_EXIT_NOTIFICATION_LEN {
        return None;
    }
    let pid = u32::from_le_bytes(bytes.get(..JOB_EXIT_JOB_ID_BYTES)?.try_into().ok()?);
    if bytes[JOB_EXIT_STATE_OFFSET] != JOB_EXIT_STATE_EXITED {
        return None;
    }
    let code = if bytes[JOB_EXIT_CODE_PRESENT_OFFSET] == 1 {
        i32::from_le_bytes(
            bytes
                .get(JOB_EXIT_CODE_OFFSET..JOB_EXIT_CODE_OFFSET + JOB_EXIT_CODE_BYTES)?
                .try_into()
                .ok()?,
        )
    } else {
        0
    };
    Some((pid, code))
}

fn reconcile_devices(
    drivers: &mut BTreeMap<String, ManagedDriver>,
    catalog: &Catalog,
    devices: Vec<SysDevice>,
) {
    let mut seen = BTreeMap::new();

    for device in devices {
        stem::debug!(
            "CAMBIUM: discovered device slot={} kind={} vendor=0x{:04x} device=0x{:04x} class=0x{:06x} present={}",
            device.slot,
            device.kind,
            device.vendor_id,
            device.device_id,
            device.class_code,
            device.present
        );
        seen.insert(device.slot.clone(), ());
        if !device.present {
            continue;
        }

        // First try the symbol-based catalog (new path).
        let maybe_entry = if device.kind != "unknown" {
            catalog.find_for_kind(&device.kind).filter(|entry| {
                // `display_bootfb` is a bootstrap display path started by sprout with a
                // dedicated memfd handshake, and must not be auto-bound to PCI display slots.
                !(device.slot.starts_with("pci-") && entry.path.ends_with("/display_bootfb"))
            })
        } else {
            None
        };

        if let Some(entry) = maybe_entry.or_else(|| {
            catalog.find_for_pci(device.vendor_id, device.device_id, device.class_code).filter(
                |entry| {
                    !(device.slot.starts_with("pci-") && entry.path.ends_with("/display_bootfb"))
                },
            )
        }) {
            if entry.driver_class == DriverClass::Audio && path_exists(SPROUT_EARLY_AUDIO_MARKER) {
                stem::debug!(
                    "CAMBIUM: skipping audio driver '{}' for {}; Sprout owns early audio",
                    entry.path,
                    device.slot
                );
                continue;
            }
            let managed = drivers.entry(device.slot.clone()).or_insert_with(|| {
                ManagedDriver::new_from_catalog(
                    &device,
                    entry.path.clone(),
                    entry.start_symbol.clone(),
                    None,
                )
            });
            managed.ensure_running();
            continue;
        }

        debug!("CAMBIUM: no catalog driver matched {}", device.slot);
    }

    let stale_slots: Vec<_> =
        drivers.keys().filter(|slot| !seen.contains_key(*slot)).cloned().collect();

    for slot in stale_slots {
        if let Some(mut managed) = drivers.remove(&slot) {
            warn!("CAMBIUM: device {} disappeared", slot);
            managed.mark_removed();
        }
    }
}

// ── argv helper ──────────────────────────────────────────────────────────────

fn get_args() -> Vec<String> {
    let len = match stem::syscall::argv_get(&mut []) {
        Ok(l) if l > 0 => l,
        _ => return Vec::new(),
    };
    let mut buf = alloc::vec![0u8; len];
    if stem::syscall::argv_get(&mut buf).is_err() {
        return Vec::new();
    }
    stem::utils::parse_argv(&buf)
        .into_iter()
        .map(|b| core::str::from_utf8(b).unwrap_or("").to_string())
        .collect()
}
