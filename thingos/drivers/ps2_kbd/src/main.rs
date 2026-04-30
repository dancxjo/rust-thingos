#![no_std]
#![no_main]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;

use abi::driver_interface::{
    DRIVER_DESCRIPTOR_ABI_VERSION, DeviceInfo, DriverClass, DriverDescriptor, DriverEntryCtx,
    ProbeResult, Status,
};
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind, device_kind_bytes};
use stem::syscall::message::msg_send;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};
use stem::syscall::{ioport_read, irq_subscribe, irq_wait};
use stem::{debug, error, info, warn};
const THINGOS_DRIVER_NAME: &[u8] = b"ps2_kbd";

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
    thingos_driver_start(ctx)
}

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
    device_kind: device_kind_bytes(b"drv.Ps2Keyboard"),
    version: 1,
    _reserved: 0,
};

/// PS/2 controller status register
const PS2_STATUS: usize = 0x64;
/// PS/2 controller data register
const PS2_DATA: usize = 0x60;

/// Status: output buffer full
const STATUS_OUTPUT_FULL: usize = 0x01;
/// Status: data from aux port (mouse) - skip
const STATUS_AUX_DATA: usize = 0x20;

/// IRQ1 vector (keyboard) - legacy IRQ1 maps to vector 0x21 after IOAPIC remap
const KBD_VECTOR: u8 = 0x21;

/// Polling interval in milliseconds – used only in the fallback path when IRQ
/// subscription fails.  25 ms is sufficient to catch any stray scancodes that
/// arrive without an interrupt and low enough to avoid noticeable latency.
const POLLING_INTERVAL_MS: u64 = 25;

/// Driver state node kind
const KIND_DRV_PS2_KBD: &str = "drv.Ps2Keyboard";

/// Path where bristle publishes its inbox-owning PID.
const BRISTLE_PID_PATH: &str = "/run/bristle/pid";

fn read_u32_file(path: &str) -> Option<u32> {
    let fd = vfs_open(path, abi::syscall::vfs_flags::O_RDONLY).ok()?;
    let mut buf = [0u8; 32];
    let n = vfs_read(fd, &mut buf).unwrap_or(0);
    let _ = vfs_close(fd);
    if n == 0 {
        return None;
    }
    let s = core::str::from_utf8(&buf[..n]).ok()?.trim();
    s.parse::<u32>().ok()
}

#[stem::main]
fn main(_raw_arg: usize) -> ! {
    stem::debug!("ps2_kbd: online — waiting for bristle pid");

    if let Err(e) = stem::fs::wait_until_exists(BRISTLE_PID_PATH) {
        stem::error!("ps2_kbd: failed waiting for {}: {:?}", BRISTLE_PID_PATH, e);
        loop {
            stem::time::sleep_ms(1000);
        }
    }

    let bristle_pid = match read_u32_file(BRISTLE_PID_PATH) {
        Some(pid) if pid != 0 => pid,
        _ => {
            stem::error!("ps2_kbd: failed to read bristle pid from {}", BRISTLE_PID_PATH);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    stem::info!("ps2_kbd: bristle pid={}", bristle_pid);

    // Subscribe to keyboard interrupt
    match irq_subscribe(KBD_VECTOR) {
        Ok(()) => {
            stem::debug!("ps2_kbd: subscribed to IRQ1 (vector 0x{:02x})", KBD_VECTOR);
            interrupt_loop(bristle_pid);
        }
        Err(e) => {
            debug!("ps2_kbd: IRQ subscribe failed ({:?}), falling back to polling", e);
            polling_loop(bristle_pid);
        }
    }
}

mod normalizer;
mod thigmonasty;

use abi::hid::{
    BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION, BristleEventHeader, EventType,
    KIND_BRISTLE_DEVICE_EVENT, KeyEventPayload,
};
use thigmonasty::{KeyEdge, KeyboardState};

/// Drain all pending keyboard data from the controller
fn drain_keyboard_data(bristle_pid: u32, state: &mut KeyboardState, drop_counter: &mut u32) {
    // Read while data is available (handle burst of scancodes)
    for _ in 0..16 {
        let status = ioport_read(PS2_STATUS, 1);

        if status & STATUS_OUTPUT_FULL == 0 {
            break; // No more data
        }

        if status & STATUS_AUX_DATA == 0 {
            // Keyboard data - read and send
            let scancode = ioport_read(PS2_DATA, 1) as u8;
            if let Some(edge) = state.process_ps2(scancode) {
                send_key_event(bristle_pid, edge, drop_counter);
            }
        } else {
            // If aux data (mouse), stop draining - let ps2_mouse handle it
            stem::debug!("ps2_kbd: yield on AUX data (mouse packet)");
            break;
        }
    }
}

fn send_key_event(bristle_pid: u32, edge: KeyEdge, drop_counter: &mut u32) {
    let timestamp_ns = stem::monotonic_ns();
    let mut buf = [0u8; 24]; // Max size is header + 4 byte payload

    let (event_type, key, mods, repeat) = match edge {
        KeyEdge::Down { key, mods, repeat } => (EventType::KeyDown, key, mods, repeat),
        KeyEdge::Up { key, mods } => (EventType::KeyUp, key, mods, false),
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

    let send_ok = msg_send(bristle_pid, abi::KindId(KIND_BRISTLE_DEVICE_EVENT), &buf[..24]).is_ok();
    if !send_ok {
        *drop_counter = drop_counter.wrapping_add(1);
        if *drop_counter <= 4 || *drop_counter % 100 == 0 {
            warn!(
                "ps2_kbd: dropped {} key events (send pid={} failed)",
                *drop_counter, bristle_pid
            );
        }
    }
}

/// Interrupt-driven service loop (primary path).
///
/// Blocks on `irq_wait` until IRQ1 fires, then drains all pending scancodes.
/// This avoids runnable-task churn during idle/low-input periods because the
/// task is only scheduled when the hardware actually signals new data.
fn interrupt_loop(bristle_pid: u32) -> ! {
    stem::debug!("ps2_kbd: using interrupt-driven loop (IRQ vector 0x{:02x})", KBD_VECTOR);
    let mut state = KeyboardState::new();
    let mut drop_counter = 0u32;
    loop {
        match irq_wait(KBD_VECTOR) {
            Ok(_pending) => {
                drain_keyboard_data(bristle_pid, &mut state, &mut drop_counter);
            }
            Err(e) => {
                // irq_wait should not fail once subscribed; if it does, fall
                // back to polling so the driver keeps functioning.
                warn!("ps2_kbd: irq_wait error ({:?}), switching to polling fallback", e);
                break;
            }
        }
    }
    polling_loop(bristle_pid)
}

/// Fallback polling loop – used only when IRQ subscription is unavailable.
fn polling_loop(bristle_pid: u32) -> ! {
    stem::debug!("ps2_kbd: using fallback polling loop ({}ms interval)", POLLING_INTERVAL_MS);
    let mut state = KeyboardState::new();
    let mut drop_counter = 0u32;
    loop {
        let status = ioport_read(PS2_STATUS, 1);

        if status & STATUS_OUTPUT_FULL != 0 {
            if status & STATUS_AUX_DATA == 0 {
                drain_keyboard_data(bristle_pid, &mut state, &mut drop_counter);
            } else {
                // Leave mouse bytes queued for ps2_mouse.
                stem::sleep_ms(POLLING_INTERVAL_MS);
            }
        } else {
            // Rate limit the polling to avoid burning CPU
            stem::sleep_ms(POLLING_INTERVAL_MS);
        }
    }
}
