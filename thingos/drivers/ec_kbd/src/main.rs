#![no_std]
#![no_main]
extern crate alloc;

use core::sync::atomic::{AtomicU64, Ordering};

use abi::driver_interface::{
    DRIVER_DESCRIPTOR_ABI_VERSION, DeviceInfo, DriverClass, DriverDescriptor, DriverEntryCtx,
    ProbeResult, Status,
};
use abi::hid::{
    BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION, BristleEventHeader, EventType,
    KIND_BRISTLE_DEVICE_EVENT, KeyEventPayload,
};
use abi::syscall::vfs_flags::O_RDONLY;
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind, device_kind_bytes};
use stem::syscall::message::msg_send;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};
use stem::{debug, info, trace, warn};

mod ec;
mod keyboard;
mod normalizer;

use ec::EcStatus;
use keyboard::{KeyEdge, KeyboardState};

const THINGOS_DRIVER_NAME: &[u8] = b"ec_kbd";
const KIND_DRV_EC_KBD: &str = "drv.EcKeyboard";
const BRISTLE_PID_PATH: &str = "/run/bristle/pid";

/// Path to the EC core service status file.
const EC_SERVICE_STATUS: &str = "/services/ec/status";
/// Path to the EC core service data (OBF byte) file.
const EC_SERVICE_DATA: &str = "/services/ec/data";

const POLL_INTERVAL_MS: u64 = 25;
const DRAIN_LIMIT: usize = 16;
const INPUT_TRACE_INITIAL: u64 = 24;
const INPUT_TRACE_INTERVAL: u64 = 128;

static EC_EVENT_COUNT: AtomicU64 = AtomicU64::new(0);

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
    driver_class: DriverClass::Input,
    flags: 0,
    probe: thingos_driver_probe,
    #[cfg(target_arch = "x86_64")]
    start: thingos_driver_start_safe as unsafe extern "C" fn(ctx: *const DriverEntryCtx) -> Status,
    #[cfg(not(target_arch = "x86_64"))]
    start: thingos_driver_start,
};

#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(
    r#"
    .section .text
    .global thingos_driver_start_safe
    thingos_driver_start_safe:
        sub rsp, 8
        push rdi
        call thingos_runtime_setup
        pop rdi
        add rsp, 8
        call thingos_driver_start_rust
        ret
"#
);

#[unsafe(no_mangle)]
unsafe extern "C" fn thingos_driver_start_rust(ctx: *const DriverEntryCtx) -> Status {
    thingos_driver_start(ctx)
}

unsafe extern "C" fn thingos_driver_probe(
    _dev: *const DeviceInfo,
    out: *mut ProbeResult,
) -> Status {
    if out.is_null() {
        return Status::InvalidArgument;
    }
    let out = unsafe { &mut *out };
    out.matched = 0;
    out.score = 0;
    out.claimed_class = DriverClass::Input;
    out.flags = 0;
    Status::NoMatch
}

unsafe extern "C" fn thingos_driver_start(_ctx: *const DriverEntryCtx) -> Status {
    main(0)
}

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: device_kind_bytes(KIND_DRV_EC_KBD.as_bytes()),
    version: 1,
    _reserved: 0,
};

// ── Entry point ───────────────────────────────────────────────────────────────

#[stem::main]
fn main(_raw_arg: usize) -> ! {
    debug!("EC keyboard service online; waiting for bristle pid");

    if let Err(e) = stem::fs::wait_until_exists(BRISTLE_PID_PATH) {
        warn!("Failed waiting for {}: {:?}", BRISTLE_PID_PATH, e);
        idle_forever();
    }

    let bristle_pid = match read_u32_file(BRISTLE_PID_PATH) {
        Some(pid) if pid != 0 => pid,
        _ => {
            warn!("Failed to read bristle pid from {}", BRISTLE_PID_PATH);
            idle_forever();
        }
    };

    debug!("Connected EC keyboard events to bristle pid={}", bristle_pid);

    // Wait for the EC core driver to publish its service.
    if let Err(e) = stem::fs::wait_until_exists(EC_SERVICE_STATUS) {
        warn!("EC service unavailable ({}): {:?}", EC_SERVICE_STATUS, e);
        idle_forever();
    }

    let status_fd = match vfs_open(EC_SERVICE_STATUS, O_RDONLY) {
        Ok(fd) => fd,
        Err(e) => {
            warn!("Failed to open {}: {:?}", EC_SERVICE_STATUS, e);
            idle_forever();
        }
    };

    let data_fd = match vfs_open(EC_SERVICE_DATA, O_RDONLY) {
        Ok(fd) => fd,
        Err(e) => {
            warn!("Failed to open {}: {:?}", EC_SERVICE_DATA, e);
            let _ = vfs_close(status_fd);
            idle_forever();
        }
    };

    if !ec_present(status_fd) {
        info!("EC keyboard controller unavailable");
        let _ = vfs_close(status_fd);
        let _ = vfs_close(data_fd);
        idle_forever();
    }

    debug!("EC keyboard polling via /services/ec (interval={}ms)", POLL_INTERVAL_MS);
    polling_loop(bristle_pid, status_fd, data_fd)
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn read_u32_file(path: &str) -> Option<u32> {
    let fd = vfs_open(path, O_RDONLY).ok()?;
    let mut buf = [0u8; 32];
    let n = vfs_read(fd, &mut buf).unwrap_or(0);
    let _ = vfs_close(fd);
    if n == 0 { return None; }
    let s = core::str::from_utf8(&buf[..n]).ok()?.trim();
    s.parse::<u32>().ok()
}

fn idle_forever() -> ! {
    loop {
        stem::time::sleep_ms(1000);
    }
}

/// Read a single byte from an open EC service file descriptor.
fn read_ec_byte(fd: u32) -> Option<u8> {
    let mut buf = [0u8; 1];
    match vfs_read(fd, &mut buf) {
        Ok(1) => Some(buf[0]),
        _ => None,
    }
}

fn ec_present(status_fd: u32) -> bool {
    for _ in 0..4 {
        if let Some(s) = read_ec_byte(status_fd) {
            if !EcStatus(s).looks_absent() {
                return true;
            }
        }
        stem::time::sleep_ms(1);
    }
    false
}

// ── Main polling loop ─────────────────────────────────────────────────────────

fn polling_loop(bristle_pid: u32, status_fd: u32, data_fd: u32) -> ! {
    let mut state = KeyboardState::new();
    let mut drop_counter = 0u32;

    loop {
        drain_ec_events(bristle_pid, status_fd, data_fd, &mut state, &mut drop_counter);
        stem::time::sleep_ms(POLL_INTERVAL_MS);
    }
}

fn drain_ec_events(
    bristle_pid: u32,
    status_fd: u32,
    data_fd: u32,
    state: &mut KeyboardState,
    drop_counter: &mut u32,
) -> usize {
    let mut events = 0usize;

    for _ in 0..DRAIN_LIMIT {
        // Read EC status from the core service (never touches hardware directly).
        let status = EcStatus(read_ec_byte(status_fd).unwrap_or(0xff));
        if status.looks_absent() {
            break;
        }
        if !status.output_full() {
            break;
        }

        // OBF is set — read the keyboard scancode.
        let byte = match read_ec_byte(data_fd) {
            Some(b) => b,
            None    => break,
        };

        let event_no = EC_EVENT_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
        if should_log_input(event_no) {
            trace!("EC keyboard byte event={} data=0x{:02x}", event_no, byte);
        }

        if let Some(edge) = state.process_set1(byte) {
            send_key_event(bristle_pid, edge, drop_counter);
            events += 1;
        } else if should_log_input(event_no) {
            trace!("Unmapped EC keyboard byte event={} data=0x{:02x}", event_no, byte);
        }
    }

    events
}

// ── Key event sender ──────────────────────────────────────────────────────────

fn send_key_event(bristle_pid: u32, edge: KeyEdge, drop_counter: &mut u32) {
    let timestamp_ns = stem::monotonic_ns();
    let mut buf = [0u8; 24];

    let (event_type, key, mods, repeat) = match edge {
        KeyEdge::Down { key, mods, repeat } => (EventType::KeyDown, key, mods, repeat),
        KeyEdge::Up { key, mods }           => (EventType::KeyUp,   key, mods, false),
    };

    let header = BristleEventHeader {
        magic: BRISTLE_EVENT_MAGIC,
        version: BRISTLE_EVENT_VERSION,
        event_type: event_type as u16,
        timestamp_ns,
        payload_len: KeyEventPayload::SIZE as u32,
    };
    let payload =
        KeyEventPayload { key: key as u16, mods: mods.0, flags: if repeat { 1 } else { 0 } };

    buf[0..20].copy_from_slice(&header.to_bytes());
    buf[20..24].copy_from_slice(&payload.to_bytes());

    if msg_send(bristle_pid, abi::KindId(KIND_BRISTLE_DEVICE_EVENT), &buf).is_err() {
        *drop_counter = drop_counter.wrapping_add(1);
        if *drop_counter <= 4 || *drop_counter % 100 == 0 {
            warn!("Dropped {} EC key events: send to pid={} failed", *drop_counter, bristle_pid);
        }
    }
}

fn should_log_input(count: u64) -> bool {
    count <= INPUT_TRACE_INITIAL || count % INPUT_TRACE_INTERVAL == 0
}
