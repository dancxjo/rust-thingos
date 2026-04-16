#![no_std]
#![no_main]
extern crate alloc;


mod binding;
mod catalog;
mod spawn;
mod sysfs;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use binding::{match_binding, mount_hint};
use catalog::Catalog;
use spawn::ManagedDriver;
use stem::{debug, warn};
use sysfs::scan_devices;

/// How many main-loop ticks between full catalog rescans.
/// At 100 ms per tick this is ~30 seconds.
const CATALOG_RESCAN_TICKS: u32 = 300;

#[stem::main]
fn main(_arg: usize) -> ! {
    debug!("DEVD: starting device discovery manager");

    let mut drivers: BTreeMap<alloc::string::String, ManagedDriver> = BTreeMap::new();
    let mut catalog = Catalog::new();
    let mut tick: u32 = 0;

    // Initial catalog scan.
    catalog.scan();

    loop {
        match scan_devices() {
            Ok(devices) => reconcile_devices(&mut drivers, &catalog, devices),
            Err(err) => warn!("DEVD: scan of /sys/devices failed: {:?}", err),
        }

        for managed in drivers.values_mut() {
            managed.monitor();
        }

        tick = tick.wrapping_add(1);
        if tick % CATALOG_RESCAN_TICKS == 0 {
            debug!("DEVD: rescanning driver catalog");
            catalog.scan();
        }

        stem::time::sleep_ms(100);
    }
}

fn reconcile_devices(
    drivers: &mut BTreeMap<alloc::string::String, ManagedDriver>,
    catalog: &Catalog,
    devices: Vec<sysfs::SysDevice>,
) {
    let mut seen = BTreeMap::new();

    for device in devices {
        seen.insert(device.slot.clone(), ());
        if !device.present {
            continue;
        }

        // First try the symbol-based catalog (new path).
        if let Some(entry) = catalog.find_for_pci(
            device.vendor_id,
            device.device_id,
            device.class_code,
        ) {
            let managed = drivers
                .entry(device.slot.clone())
                .or_insert_with(|| {
                    ManagedDriver::new_from_catalog(
                        &device,
                        entry.path.clone(),
                        entry.entry_symbol.clone(),
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

    let stale_slots: Vec<_> = drivers
        .keys()
        .filter(|slot| !seen.contains_key(*slot))
        .cloned()
        .collect();

    for slot in stale_slots {
        if let Some(mut managed) = drivers.remove(&slot) {
            warn!("DEVD: device {} disappeared", slot);
            managed.mark_removed();
        }
    }
}
