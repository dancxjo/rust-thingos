#![no_std]
#![no_main]
extern crate alloc;

mod binding;
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
use binding::{match_binding, mount_hint};
use catalog::Catalog;
use spawn::ManagedDriver;
use stem::kinds::{
    KIND_ID_THINGOS_JOB_EXIT, KIND_ID_THINGOS_SHUTDOWN_READY, KIND_ID_THINGOS_SHUTDOWN_REQUEST,
    ShutdownReadyV1, ShutdownRequestV1,
};
use stem::service_loop::{ServiceEvent, ServiceLoop};
use stem::syscall::message::{KindId, msg_recv, msg_send};
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_watch_path, vfs_write};
use stem::time::{Duration, monotonic_ns};
use stem::{debug, error, trace, warn};
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
const SHUTDOWN_TERM_GRACE_MS: u64 = 1_000;
const SHUTDOWN_KILL_GRACE_MS: u64 = 1_000;
const SHUTDOWN_POLL_MS: u64 = 25;

#[stem::main]
fn main(_arg: usize) -> ! {
    stem::debug!("Cambium main started");
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
    debug!("Manual mode: driver={}", driver_path);

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
            error!("'{}' does not export THING_DRIVER_V1 or could not be read", abs_path);
            stem::syscall::exit(1);
        }
    };

    debug!(
        "Driver '{}' inspected: name='{}' class={:?} vendor=0x{:04x} device=0x{:04x} class_code=0x{:06x} start='{}'",
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
            error!("Failed to scan /sys/devices: {:?}", e);
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
            ManagedDriver::new_from_catalog(
                &d,
                abs_path.clone(),
                entry.start_symbol.clone(),
                entry.driver_class,
                None,
            )
        })
        .collect();

    if managed.is_empty() {
        if let Some(slot) = slot_filter {
            // If an explicit slot was given, try to run the driver for it
            // regardless of the marker's match hints — the user said to do it.
            debug!("No match by hints for slot '{}'; spawning anyway by explicit override", slot);
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
                entry.driver_class,
                None,
            ));
        } else {
            warn!("No matching devices found for driver '{}'", abs_path);
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
    stem::info!("Starting device discovery manager...");

    let fallback_bootfb = is_fallback_bootfb();
    if fallback_bootfb {
        stem::debug!(
            "Fallback boot framebuffer is active; alternate display drivers will be skipped"
        );
    }

    let mut drivers: BTreeMap<String, ManagedDriver> = BTreeMap::new();
    let mut catalog = Catalog::new();
    let mut observed_pids: BTreeMap<u64, ()> = BTreeMap::new();

    // Initial catalog scan.
    catalog.scan();

    match scan_devices() {
        Ok(devices) => reconcile_devices(&mut drivers, &catalog, devices, fallback_bootfb),
        Err(err) => warn!("Initial scan of /sys/devices failed: {:?}", err),
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
                "Failed to construct ServiceLoop ({:?}); running degraded monitor-only loop",
                err
            );
            run_degraded_monitor_loop(&mut drivers);
        }
    };

    let devices_watch_token = add_watch_path(&mut svc, "/sys/devices", "device topology");
    let drivers_watch_token = add_watch_path(&mut svc, "/drivers", "driver catalog");
    let mounts_watch_token = add_watch_path(&mut svc, "/proc/mounts", "mount table");

    let timeout = Some(Duration::from_millis(RECONCILE_TIMEOUT_MS));

    loop {
        let mut reconcile_due = false;
        let mut catalog_rescan_due = false;
        let mut messages_drained = false;

        match svc.next_event(timeout) {
            Ok(ServiceEvent::Message { kind, payload, .. }) => {
                stem::trace!("ServiceLoop woke for inbox message");
                if kind.0 == KIND_ID_THINGOS_SHUTDOWN_REQUEST {
                    handle_shutdown_request_and_exit(&mut drivers, payload);
                } else {
                    handle_job_exit_message(&mut drivers, &mut observed_pids, kind, payload);
                }
                messages_drained = true;
            }
            Ok(ServiceEvent::Ready { token, event }) => {
                if event.is_error() || event.is_hangup() {
                    warn!("ServiceLoop secondary source error or hangup: token={:?}", token);
                    reconcile_due = true;
                }
                if let Some((dev_token, dev_fd)) = devices_watch_token {
                    if token == dev_token {
                        stem::trace!("ServiceLoop woke for readable /sys/devices watch");
                        if event.is_readable() {
                            drain_watch_fd(dev_fd);
                        }
                        reconcile_due = true;
                    }
                }
                if let Some((catalog_token, catalog_fd)) = drivers_watch_token {
                    if token == catalog_token {
                        stem::debug!("/drivers changed; rescanning driver catalog...");
                        if event.is_readable() {
                            drain_watch_fd(catalog_fd);
                        }
                        catalog_rescan_due = true;
                        reconcile_due = true;
                    }
                }
                if let Some((mounts_token, mounts_fd)) = mounts_watch_token {
                    if token == mounts_token {
                        stem::debug!("Mount table changed; rescanning driver catalog...");
                        if event.is_readable() {
                            drain_watch_fd(mounts_fd);
                        }
                        catalog_rescan_due = true;
                        reconcile_due = true;
                    }
                }
            }
            Ok(ServiceEvent::Timeout) => {
                stem::trace!("ServiceLoop woke for reconcile tick");
                reconcile_due = true;
            }
            Ok(ServiceEvent::InboxClosed) => {
                warn!("Inbox closed; falling back to degraded monitor-only loop");
                run_degraded_monitor_loop(&mut drivers);
            }
            Err(err) => {
                warn!("ServiceLoop next_event error: {:?}", err);
                reconcile_due = true;
            }
        }

        // Batch any additional queued inbox messages so the loop matches
        // the original "drain inbox per wake" semantics.
        if messages_drained {
            let _ = svc.drain_inbox(|kind, payload| {
                if kind.0 == KIND_ID_THINGOS_SHUTDOWN_REQUEST {
                    handle_shutdown_request_and_exit(&mut drivers, payload);
                } else {
                    handle_job_exit_message(&mut drivers, &mut observed_pids, kind, payload);
                }
                ControlFlow::Continue(())
            });
            register_observers_for_running(&mut drivers, &mut observed_pids);
        }

        if catalog_rescan_due {
            catalog.scan();
        }

        if reconcile_due {
            match scan_devices() {
                Ok(devices) => {
                    reconcile_devices(&mut drivers, &catalog, devices, fallback_bootfb);
                    register_observers_for_running(&mut drivers, &mut observed_pids);
                }
                Err(err) => warn!("Scan of /sys/devices failed: {:?}", err),
            }
        }
    }
}

fn add_watch_path(
    svc: &mut ServiceLoop,
    path: &str,
    purpose: &str,
) -> Option<(stem::wait_set::WaitToken, u32)> {
    let flags = if path == "/proc/mounts" {
        watch_flags::NONBLOCK
    } else {
        watch_flags::NONBLOCK | watch_flags::ONLYDIR
    };
    match vfs_watch_path(path, watch_mask::ALL_EVENTS, flags) {
        Ok(fd) => match svc.add_vfs_watch(fd) {
            Ok(token) => {
                stem::debug!("Watching {} for {} updates", path, purpose);
                Some((token, fd))
            }
            Err(err) => {
                warn!("Failed to register {} watch with ServiceLoop: {:?}", path, err);
                let _ = vfs_close(fd);
                None
            }
        },
        Err(err) => {
            warn!("Failed to watch {}: {:?}", path, err);
            None
        }
    }
}

/// Fallback loop used when no readiness primitive is available (no inbox,
/// no `/sys/devices` watch, or an unrecoverable inbox closure).  It mirrors
/// the original degraded path so behavior is preserved end-to-end.
fn run_degraded_monitor_loop(drivers: &mut BTreeMap<String, ManagedDriver>) -> ! {
    loop {
        let mut kind = KindId([0u8; 16]);
        let mut payload = [0u8; INBOX_MAX_PAYLOAD];
        match msg_recv(&mut kind, &mut payload) {
            Ok(len) if kind.0 == KIND_ID_THINGOS_SHUTDOWN_REQUEST => {
                handle_shutdown_request_and_exit(drivers, &payload[..len]);
            }
            Ok(_) | Err(Errno::EAGAIN) => {}
            Err(err) => warn!("Degraded loop inbox receive failed: {:?}", err),
        }
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
                    warn!("Failed to register job observer for pid {}: {:?}", pid, err);
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
                warn!("Failed reading /sys/devices watch events: {:?}", err);
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

fn handle_shutdown_request_and_exit(
    drivers: &mut BTreeMap<String, ManagedDriver>,
    payload: &[u8],
) -> ! {
    let requester_pid = match ShutdownRequestV1::from_bytes(payload) {
        Some(request) => request.requester_pid,
        None => {
            warn!(
                "Malformed shutdown request payload ({} bytes); replying to parent",
                payload.len()
            );
            stem::syscall::getppid()
        }
    };

    stem::info!(
        "Shutdown requested by PID {}; quiescing {} managed driver(s)",
        requester_pid,
        drivers.len()
    );

    for managed in drivers.values() {
        managed.request_shutdown();
    }
    let mut remaining = wait_for_driver_shutdown(drivers, SHUTDOWN_TERM_GRACE_MS);
    if remaining > 0 {
        warn!("{} drivers still running after SIGTERM grace; forcing shutdown", remaining);
        for managed in drivers.values() {
            managed.force_shutdown();
        }
        remaining = wait_for_driver_shutdown(drivers, SHUTDOWN_KILL_GRACE_MS);
    }

    let status = if remaining == 0 { 0 } else { 1 };
    let ready = ShutdownReadyV1::new(stem::syscall::getpid(), status, remaining);
    match msg_send(requester_pid, KindId(KIND_ID_THINGOS_SHUTDOWN_READY), ready.as_bytes()) {
        Ok(()) => stem::debug!(
            "Sent shutdown-ready to PID {} status={} active_children={}",
            requester_pid,
            status,
            remaining
        ),
        Err(err) => {
            warn!("Failed to send shutdown-ready to PID {}: {:?}", requester_pid, err)
        }
    }

    stem::syscall::exit(status);
}

fn wait_for_driver_shutdown(drivers: &mut BTreeMap<String, ManagedDriver>, grace_ms: u64) -> u32 {
    let deadline_ns = monotonic_ns().saturating_add(grace_ms.saturating_mul(1_000_000));
    loop {
        let mut remaining = 0u32;
        for managed in drivers.values_mut() {
            if !managed.poll_shutdown() {
                remaining = remaining.saturating_add(1);
            }
        }
        if remaining == 0 || monotonic_ns() >= deadline_ns {
            return remaining;
        }
        stem::time::sleep_ms(SHUTDOWN_POLL_MS);
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
    fallback_bootfb: bool,
) {
    let mut seen = BTreeMap::new();
    for device in &devices {
        seen.insert(device.slot.clone(), ());
    }

    for bootfb_pass in [false, true] {
        for device in &devices {
            if is_boot_framebuffer_device(device) != bootfb_pass {
                continue;
            }

            stem::trace!(
                "Discovered device: slot={} kind={} vendor=0x{:04x} device=0x{:04x} class=0x{:06x} present={}",
                device.slot,
                device.kind,
                device.vendor_id,
                device.device_id,
                device.class_code,
                device.present
            );
            let is_xhci_pci = device.slot.starts_with("pci-") && device.class_code == 0x0c0330;
            if is_xhci_pci {
                stem::debug!(
                    "Discovered xHCI PCI device: slot={} vendor=0x{:04x} device=0x{:04x} class=0x{:06x} present={}",
                    device.slot,
                    device.vendor_id,
                    device.device_id,
                    device.class_code,
                    device.present
                );
            }
            if !device.present {
                continue;
            }

            // First try the symbol-based catalog (new path).
            let maybe_entry = if device.kind != "unknown" {
                catalog.find_for_kind(&device.kind).filter(|entry| {
                    // `display_bootfb` serves the platform framebuffer and must not
                    // be auto-bound to PCI display slots.
                    !(device.slot.starts_with("pci-") && entry.path.ends_with("/display_bootfb"))
                })
            } else {
                None
            };

            if let Some(entry) = maybe_entry.or_else(|| {
                catalog.find_for_pci(device.vendor_id, device.device_id, device.class_code).filter(
                    |entry| {
                        !(device.slot.starts_with("pci-")
                            && entry.path.ends_with("/display_bootfb"))
                    },
                )
            }) {
                if is_xhci_pci {
                    stem::debug!(
                        "Starting xHCI driver '{}' for PCI device {}...",
                        entry.path,
                        device.slot
                    );
                }
                if fallback_bootfb
                    && entry.driver_class == DriverClass::Display
                    && !entry.path.ends_with("/display_bootfb")
                {
                    stem::debug!(
                        "Fallback boot framebuffer mode skipped display driver '{}' for {}",
                        entry.path,
                        device.slot
                    );
                    continue;
                }
                if entry.driver_class == DriverClass::Audio
                    && path_exists(SPROUT_EARLY_AUDIO_MARKER)
                {
                    stem::trace!(
                        "Skipped audio driver '{}' for {}; Sprout owns early audio",
                        entry.path,
                        device.slot
                    );
                    continue;
                }
                let mount_path = catalog_mount_path(drivers, entry.driver_class);
                let managed = drivers.entry(device.slot.clone()).or_insert_with(|| {
                    ManagedDriver::new_from_catalog(
                        device,
                        entry.path.clone(),
                        entry.start_symbol.clone(),
                        entry.driver_class,
                        mount_path,
                    )
                });
                managed.ensure_running();
                continue;
            }

            // Fall back to the legacy static binding table.
            let Some(binding) = match_binding(device) else {
                if is_xhci_pci {
                    stem::warn!(
                        "No driver matched xHCI PCI device {} vendor=0x{:04x} device=0x{:04x} class=0x{:06x}",
                        device.slot,
                        device.vendor_id,
                        device.device_id,
                        device.class_code
                    );
                }
                continue;
            };
            let mount_path = mount_hint(binding, device);
            let managed = drivers
                .entry(device.slot.clone())
                .or_insert_with(|| ManagedDriver::new(device, binding, mount_path));
            managed.ensure_running();
        }
    }

    let stale_slots: Vec<_> =
        drivers.keys().filter(|slot| !seen.contains_key(*slot)).cloned().collect();

    for slot in stale_slots {
        if let Some(mut managed) = drivers.remove(&slot) {
            warn!("Device {} disappeared", slot);
            managed.mark_removed();
        }
    }
}

fn is_boot_framebuffer_device(device: &SysDevice) -> bool {
    device.kind == "dev.display.Framebuffer"
}

fn catalog_mount_path(
    drivers: &BTreeMap<String, ManagedDriver>,
    driver_class: DriverClass,
) -> Option<String> {
    match driver_class {
        DriverClass::Display => allocate_indexed_dev_path(drivers, "/dev/display/card"),
        DriverClass::Net => allocate_indexed_dev_path(drivers, "/dev/net/card"),
        _ => None,
    }
}

fn allocate_indexed_dev_path(
    drivers: &BTreeMap<String, ManagedDriver>,
    prefix: &str,
) -> Option<String> {
    for index in 0..32u32 {
        let candidate = alloc::format!("{}{}", prefix, index);
        let in_use =
            drivers.values().any(|driver| driver.mount_path.as_deref() == Some(candidate.as_str()));
        if !in_use {
            return Some(candidate);
        }
    }
    None
}

fn is_fallback_bootfb() -> bool {
    match vfs_open("/sys/boot/cmdline", O_RDONLY) {
        Ok(fd) => {
            let mut buf = [0u8; 512];
            let res = vfs_read(fd, &mut buf);
            let _ = vfs_close(fd);
            match res {
                Ok(len) => {
                    let cmdline = core::str::from_utf8(&buf[..len]).unwrap_or("");
                    cmdline.contains("display=bootfb")
                }
                Err(_) => false,
            }
        }
        Err(_) => false,
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
