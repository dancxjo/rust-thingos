#![no_std]
#![no_main]
use alloc::string::{String, ToString};
extern crate alloc;

use abi::driver_interface::{
    DRIVER_DESCRIPTOR_ABI_VERSION, DeviceInfo, DriverClass, DriverDescriptor, DriverEntryCtx,
    ProbeResult, Status,
};
use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind, device_kind_bytes};
use stem::syscall::{device_claim, vm_map, vm_unmap};
use stem::{debug, info, warn};

const THINGOS_DRIVER_NAME: &[u8] = b"rtc_cmos";

#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    fn thingos_driver_start_safe(ctx: *const DriverEntryCtx) -> Status;
}

#[unsafe(no_mangle)]
#[used]
pub static THINGOS_DRIVER: DriverDescriptor = DriverDescriptor {
    abi_version: DRIVER_DESCRIPTOR_ABI_VERSION,
    driver_name_ptr: THINGOS_DRIVER_NAME.as_ptr(),
    driver_name_len: THINGOS_DRIVER_NAME.len(),
    driver_class: DriverClass::Other,
    flags: 0,
    probe: thingos_driver_probe,
    #[cfg(target_arch = "x86_64")]
    start: thingos_driver_start_safe as unsafe extern "C" fn(ctx: *const DriverEntryCtx) -> Status,
    #[cfg(not(target_arch = "x86_64"))]
    start: thingos_driver_start_rust,
};

unsafe extern "C" fn thingos_driver_probe(dev: *const DeviceInfo, out: *mut ProbeResult) -> Status {
    if dev.is_null() || out.is_null() {
        return Status::InvalidArgument;
    }
    let dev = &*dev;
    let out = &mut *out;

    // Check for "Other" class or specifically System/RTC (0x080001) if we ever use it.
    // For now, if cambium matched us via device_kind, it will spawn us.
    // If we are here via a probe scan, we check if the device looks like an RTC.

    // Match if class is Other (0x00) and it's a legacy/platform device.
    // (In our current system, legacy devices have vendor=0, device=0 on bus 0)
    if dev.vendor_id == 0 && dev.device_id == 0 {
        out.matched = 1;
        out.score = 100;
        out.claimed_class = DriverClass::Other;
        out.flags = 0;
        return Status::Ok;
    }

    out.matched = 0;
    out.score = 0;
    out.claimed_class = DriverClass::Other;
    out.flags = 0;
    Status::NoMatch
}

#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(
    r#"
    .section .text
    .global thingos_driver_start_safe
    thingos_driver_start_safe:
        // RSP = 16n (kernel spawn)
        sub rsp, 8
        push rdi
        // Call std initialization (TLS, etc)
        call thingos_runtime_setup
        // Restore RDI and realign for the next call.
        pop rdi
        add rsp, 8
        // CALL will push 8 bytes, so inside Rust entry RSP = 16n + 8.
        call thingos_driver_start_rust
        ret
"#
);

#[unsafe(no_mangle)]
unsafe extern "C" fn thingos_driver_start_rust(ctx: *const DriverEntryCtx) -> Status {
    main(ctx as usize)
}

#[link_section = ".thing_manifest"]
#[no_mangle]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: device_kind_bytes(b"dev.rtc.Cmos"),
    version: 1,
    _reserved: 0,
};

// CMOS RTC I/O ports
const CMOS_ADDR: u16 = 0x70;
const CMOS_DATA: u16 = 0x71;

// CMOS registers
const RTC_SECONDS: u8 = 0x00;
const RTC_MINUTES: u8 = 0x02;
const RTC_HOURS: u8 = 0x04;
const RTC_DAY: u8 = 0x07;
const RTC_MONTH: u8 = 0x08;
const RTC_YEAR: u8 = 0x09;
const RTC_STATUS_A: u8 = 0x0A;
const RTC_STATUS_B: u8 = 0x0B;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RtcSample {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

fn cmos_read(reg: u8) -> u8 {
    stem::syscall::ioport_write(CMOS_ADDR as usize, reg as usize, 1);
    stem::syscall::ioport_read(CMOS_DATA as usize, 1) as u8
}

fn is_updating() -> bool {
    cmos_read(RTC_STATUS_A) & 0x80 != 0
}

fn bcd_to_binary(bcd: u8) -> u8 {
    (bcd & 0x0F) + ((bcd >> 4) * 10)
}

fn read_cmos_raw() -> (u8, u8, u8, u8, u8, u8, u8) {
    while is_updating() {
        stem::yield_now();
    }

    let seconds = cmos_read(RTC_SECONDS);
    let minutes = cmos_read(RTC_MINUTES);
    let hours = cmos_read(RTC_HOURS);
    let day = cmos_read(RTC_DAY);
    let month = cmos_read(RTC_MONTH);
    let year = cmos_read(RTC_YEAR);
    let status_b = cmos_read(RTC_STATUS_B);

    (year, month, day, hours, minutes, seconds, status_b)
}

fn decode_rtc_sample(raw: (u8, u8, u8, u8, u8, u8, u8)) -> Option<RtcSample> {
    let (year, month, day, hours, minutes, seconds, status_b) = raw;
    let is_bcd = (status_b & 0x04) == 0;
    let is_24h = (status_b & 0x02) != 0;
    let pm = !is_24h && (hours & 0x80) != 0;
    let hour_without_pm = hours & 0x7F;

    let (second, minute, mut hour, day, month, year) = if is_bcd {
        (
            bcd_to_binary(seconds),
            bcd_to_binary(minutes),
            bcd_to_binary(hour_without_pm),
            bcd_to_binary(day),
            bcd_to_binary(month),
            bcd_to_binary(year),
        )
    } else {
        (seconds, minutes, hour_without_pm, day, month, year)
    };

    if !is_24h {
        if hour == 12 {
            hour = 0;
        }
        if pm {
            hour = hour.saturating_add(12);
        }
    }

    // Assume century is 20xx for year < 70, 19xx for >= 70
    let full_year = if year < 70 { 2000 + year as u16 } else { 1900 + year as u16 };

    let sample = RtcSample { year: full_year, month, day, hour, minute, second };
    if validate_rtc_sample(sample) { Some(sample) } else { None }
}

fn read_rtc() -> Option<RtcSample> {
    for _ in 0..8 {
        let first = read_cmos_raw();
        let second = read_cmos_raw();
        if first == second {
            return decode_rtc_sample(second);
        }
        stem::sleep_ms(2);
    }
    None
}

fn validate_rtc_sample(sample: RtcSample) -> bool {
    sample.year >= 2020
        && (1..=12).contains(&sample.month)
        && (1..=days_in_month(sample.year, sample.month)).contains(&sample.day)
        && sample.hour < 24
        && sample.minute < 60
        && sample.second < 60
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

/// Convert calendar time to Unix timestamp (seconds since 1970-01-01 00:00:00 UTC)
fn rtc_to_unix(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> u64 {
    const MONTH_DAYS: [u32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

    let year = year as u64;
    let month = (month as u64).clamp(1, 12);
    let day = (day as u64).clamp(1, 31);

    let years = year.saturating_sub(1970);

    // Count leap years since 1970
    let leap_years = if year > 1970 {
        let y = year - 1;
        let from_base = |y: u64| (y / 4) - (y / 100) + (y / 400);
        from_base(y) - from_base(1969)
    } else {
        0
    };

    let mut days = years * 365 + leap_years;
    days += MONTH_DAYS[(month - 1) as usize] as u64;

    let is_leap = (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0));
    if is_leap && month > 2 {
        days += 1;
    }

    days += day - 1;

    days * 86400 + hour as u64 * 3600 + minute as u64 * 60 + second as u64
}

fn anchor_from_rtc() -> bool {
    let Some(sample) = read_rtc() else {
        warn!("Failed to read a stable CMOS timestamp; system clock remains unanchored");
        return false;
    };
    let unix_secs = rtc_to_unix(
        sample.year,
        sample.month,
        sample.day,
        sample.hour,
        sample.minute,
        sample.second,
    );

    debug!(
        "RTC sample: {:04}-{:02}-{:02} {:02}:{:02}:{:02} unix_secs={}",
        sample.year, sample.month, sample.day, sample.hour, sample.minute, sample.second, unix_secs
    );

    stem::syscall::time_anchor(unix_secs);
    info!("System clock anchored from RTC");
    true
}

fn device_path_from_ctx_memfd(boot_fd: usize) -> Option<String> {
    if boot_fd == 0 {
        return None;
    }

    let len = core::mem::size_of::<DriverEntryCtx>();
    let req = VmMapReq {
        addr_hint: 0,
        len,
        prot: VmProt::READ | VmProt::USER,
        flags: VmMapFlags::empty(),
        backing: VmBacking::File { thing: boot_fd as u32, offset: 0 },
    };
    let Ok(resp) = vm_map(&req) else {
        return None;
    };

    let path = unsafe {
        let ctx = &*(resp.addr as *const DriverEntryCtx);
        if ctx.version == 1 {
            let path = ctx.device_path_str();
            if path.is_empty() { None } else { Some(path.to_string()) }
        } else {
            None
        }
    };
    let _ = vm_unmap(resp.addr, resp.len);
    path
}

fn claim_device_from_boot_arg(boot_arg: usize) -> Option<usize> {
    let Some(path) = device_path_from_ctx_memfd(boot_arg) else {
        debug!("Starting without a DriverEntryCtx device path");
        return None;
    };

    match device_claim(&path) {
        Ok(claim) => {
            debug!("Device claimed at {} with handle={}", path, claim);
            Some(claim)
        }
        Err(err) => {
            warn!("Failed to claim {}: {:?}", path, err);
            None
        }
    }
}

#[stem::main]
fn main(arg: usize) -> ! {
    let cpu = stem::arch::whoami();
    debug!(
        "whoami: cs=0x{:x} ss=0x{:x} cpl={} rsp=0x{:x} rip=0x{:x} rflags=0x{:x}",
        cpu.cs, cpu.ss, cpu.cpl, cpu.rsp, cpu.rip, cpu.rflags
    );

    debug!("Starting with arg={:x}...", arg);
    let _claim = claim_device_from_boot_arg(arg);

    for attempt in 1..=20 {
        if anchor_from_rtc() {
            break;
        }
        warn!("Clock anchor attempt {} failed; retrying", attempt);
        stem::sleep_ms(100);
    }

    debug!("Entering RTC maintenance loop");

    loop {
        stem::sleep(core::time::Duration::from_secs(3600)); // Update once per hour
        let _ = anchor_from_rtc();
    }
}
