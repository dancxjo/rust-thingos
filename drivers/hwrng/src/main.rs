#![no_std]
#![no_main]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;

use abi::driver_interface::{
    DRIVER_DESCRIPTOR_ABI_VERSION, DeviceInfo, DriverClass, DriverDescriptor, DriverStartContext,
    ProbeResult, Status,
};
use abi::motor::{
    MOTOR_HOST_DRIVER, MOTOR_INTERFACE_DRIVER_V1, SEED_ABI_VERSION, Seed, SeedInterface,
};
use stem::abi::driver_ctx::DriverCtx;
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind};
use stem::{debug, info};
const THINGOS_DRIVER_NAME: &[u8] = b"hwrng";
const SEED_NAME: &[u8] = b"hwrng";

#[unsafe(no_mangle)]
#[used]
pub static THINGOS_MOTOR: Seed = Seed {
    abi_version: SEED_ABI_VERSION,
    interface_count: 1,
    hosting_modes: MOTOR_HOST_DRIVER,
    capabilities: 0,
    motor_name_ptr: SEED_NAME.as_ptr(),
    motor_name_len: SEED_NAME.len(),
    interfaces: [
        SeedInterface {
            interface_id: MOTOR_INTERFACE_DRIVER_V1,
            interface_version: 1,
            flags: 0,
            reserved: 0,
            entry_symbol_ptr: core::ptr::null(),
            entry_symbol_len: 0,
        },
        SeedInterface::default(),
        SeedInterface::default(),
        SeedInterface::default(),
    ],
};

#[unsafe(no_mangle)]
#[used]
pub static THINGOS_DRIVER: DriverDescriptor = DriverDescriptor {
    abi_version: DRIVER_DESCRIPTOR_ABI_VERSION,
    driver_name_ptr: THINGOS_DRIVER_NAME.as_ptr(),
    driver_name_len: THINGOS_DRIVER_NAME.len(),
    driver_class: DriverClass::Other,
    flags: 0,
    probe: thingos_driver_probe,
    start: thingos_driver_start,
};

unsafe extern "C" fn thingos_driver_probe(
    _dev: *const DeviceInfo,
    out: *mut ProbeResult,
) -> Status {
    if out.is_null() {
        return Status::InvalidArgument;
    }
    let out = &mut *out;
    out.matched = 0;
    out.score = 0;
    out.claimed_class = DriverClass::Other;
    out.flags = 0;
    Status::NoMatch
}

unsafe extern "C" fn thingos_driver_start(_ctx: *const DriverStartContext) -> Status {
    main(0)
}

#[link_section = ".thing_manifest"]
#[no_mangle]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    // "dev.rng.HwRng"
    device_kind: [
        0x64, 0x65, 0x76, 0x2e, 0x72, 0x6e, 0x67, 0x2e, 0x48, 0x77, 0x52, 0x6e, 0x67, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    version: 1,
    _reserved: 0,
};

/// Collect entropy from timing jitter between monotonic reads and mix in the
/// current counter to ensure each sample is unique.
fn collect_entropy(buf: &mut [u8; 64], counter: u64) {
    // Sample the monotonic clock several times; the jitter between samples
    // contains genuine hardware timing noise.
    let mut accum: u64 = counter.wrapping_mul(0x9e3779b97f4a7c15);
    for i in 0..8u64 {
        let t = stem::syscall::monotonic_ns();
        // Mix the timing sample with a position-dependent rotation.
        accum ^= t.rotate_left((i * 7 + 13) as u32 & 63);
        accum = accum.wrapping_mul(0xbf58476d1ce4e5b9);
        accum ^= accum >> 27;
        accum = accum.wrapping_mul(0x94d049bb133111eb);
        accum ^= accum >> 31;
        stem::yield_now();
    }

    // Fill 8 bytes per state word, covering the full 64-byte buffer.
    for (i, chunk) in buf.chunks_mut(8).enumerate() {
        let word = accum
            .wrapping_add(counter)
            .wrapping_add(i as u64)
            .rotate_left((i as u32 * 11 + 5) & 63);
        let word = word ^ word.wrapping_mul(0x517cc1b727220a95);
        let bytes = word.to_ne_bytes();
        let n = chunk.len().min(8);
        chunk[..n].copy_from_slice(&bytes[..n]);
        // Re-mix for next word
        accum ^= word;
        accum = accum.wrapping_mul(0x94d049bb133111eb);
    }
}

#[stem::main]
fn main(arg: usize) -> ! {
    let cpu = stem::arch::whoami();
    debug!(
        "hwrng: cs=0x{:x} ss=0x{:x} cpl={} rsp=0x{:x} rip=0x{:x}",
        cpu.cs, cpu.ss, cpu.cpl, cpu.rsp, cpu.rip
    );

    debug!("hwrng: Starting... arg={:x}", arg);

    if arg != 0 {
        let ctx = DriverCtx::from_raw(arg);
        let id = stem::thing::ThingId(ctx.device_id.0);
        debug!("hwrng: Serving device ID: {:?}", id);
    } else {
        debug!("hwrng: Starting without explicit context (phased boot mode).");
    }

    // Perform an initial seeding of the kernel entropy pool.
    let mut entropy_buf = [0u8; 64];
    collect_entropy(&mut entropy_buf, 0);
    stem::syscall::entropy_seed(&entropy_buf);
    info!("hwrng: Kernel entropy pool seeded.");

    // Enter a maintenance loop, periodically refreshing the entropy pool.
    // This ensures the pool accumulates timing jitter over the system lifetime.
    let mut counter: u64 = 1;
    loop {
        stem::sleep(core::time::Duration::from_secs(60));
        collect_entropy(&mut entropy_buf, counter);
        stem::syscall::entropy_seed(&entropy_buf);
        debug!("hwrng: Entropy pool refreshed (cycle {})", counter);
        counter = counter.wrapping_add(1);
    }
}
