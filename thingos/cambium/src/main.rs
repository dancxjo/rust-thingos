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

use abi::driver_interface::DriverClass;
use abi::syscall::vfs_flags::O_RDONLY;
use binding::{match_binding, mount_hint};
use catalog::Catalog;
use spawn::ManagedDriver;
use stem::syscall::vfs::{vfs_close, vfs_open};
use stem::{debug, error, warn};
use sysfs::{SysDevice, scan_devices};

/// How many main-loop ticks between full catalog rescans.
/// At 100 ms per tick this is ~30 seconds.
const CATALOG_RESCAN_TICKS: u32 = 300;
/// How many main-loop ticks between full device topology rescans.
/// Cambium does not have topology-change notifications yet, so avoid walking
/// `/sys/devices` every tick when the machine is idle.
const DEVICE_RESCAN_TICKS: u32 = 50;

fn is_network_device(device: &SysDevice) -> bool {
    if device.kind.starts_with("dev.net") {
        return true;
    }

    // PCI class major 0x02 == network controller.
    ((device.class_code >> 16) & 0xff) == 0x02
}

fn is_network_catalog_entry(entry: &catalog::DriverEntry) -> bool {
    if entry.driver_class == DriverClass::Net {
        return true;
    }

    // Legacy hint fallback when class enum is not populated.
    ((entry.class_code >> 16) & 0xff) == 0x02
}

#[stem::main]
fn main(_arg: usize) -> ! {
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

    if !is_network_catalog_entry(&entry) {
        error!(
            "CAMBIUM: refusing to run non-network driver '{}' while network-only mode is active",
            abs_path
        );
        stem::syscall::exit(1);
    }

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
                && is_network_device(d)
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
    debug!("CAMBIUM: starting device discovery manager (daemon mode)");

    let mut drivers: BTreeMap<String, ManagedDriver> = BTreeMap::new();
    let mut catalog = Catalog::new();
    let mut tick: u32 = 0;

    // Initial catalog scan.
    catalog.scan();

    match scan_devices() {
        Ok(devices) => reconcile_devices(&mut drivers, &catalog, devices),
        Err(err) => warn!("CAMBIUM: initial scan of /sys/devices failed: {:?}", err),
    }

    loop {
        for managed in drivers.values_mut() {
            managed.monitor();
        }

        tick = tick.wrapping_add(1);
        if tick % DEVICE_RESCAN_TICKS == 0 {
            match scan_devices() {
                Ok(devices) => reconcile_devices(&mut drivers, &catalog, devices),
                Err(err) => warn!("CAMBIUM: scan of /sys/devices failed: {:?}", err),
            }
        }
        if tick % CATALOG_RESCAN_TICKS == 0 {
            debug!("CAMBIUM: rescanning driver catalog");
            catalog.scan();
        }

        stem::time::sleep_ms(100);
    }
}

fn reconcile_devices(
    drivers: &mut BTreeMap<String, ManagedDriver>,
    catalog: &Catalog,
    devices: Vec<SysDevice>,
) {
    let mut seen = BTreeMap::new();

    for device in devices {
        seen.insert(device.slot.clone(), ());
        if !device.present {
            continue;
        }

        if !is_network_device(&device) {
            continue;
        }

        // First try the symbol-based catalog (new path).
        let maybe_entry =
            if device.kind != "unknown" { catalog.find_for_kind(&device.kind) } else { None };

        if let Some(entry) = maybe_entry
            .or_else(|| catalog.find_for_pci(device.vendor_id, device.device_id, device.class_code))
        {
            if !is_network_catalog_entry(entry) {
                debug!(
                    "CAMBIUM: ignoring non-network catalog driver '{}' for slot {}",
                    entry.path, device.slot
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

        // Fall back to the legacy static binding table.
        let Some(binding) = match_binding(&device) else {
            continue;
        };
        let mount_path = mount_hint(binding, &device);
        let managed = drivers
            .entry(device.slot.clone())
            .or_insert_with(|| ManagedDriver::new(&device, binding, mount_path));
        managed.ensure_running();
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
