#![no_std]
#![no_main]
use alloc::string::ToString;
use core::default::Default;
use core::sync::atomic::{AtomicU64, Ordering};
extern crate alloc;

use abi::driver_interface::{
    DRIVER_DESCRIPTOR_ABI_VERSION, DeviceInfo, DriverClass, DriverDescriptor, DriverEntryCtx,
    ProbeResult, Status,
};
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind, device_kind_bytes};
use stem::syscall::message::msg_send;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};
use stem::syscall::{ioport_read, irq_subscribe};
use stem::time::Duration;
use stem::wait_set::WaitSet;
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
const IRQ_ASSIST_POLL_MS: u64 = 4;
const INPUT_TRACE_INITIAL: u64 = 24;
const INPUT_TRACE_INTERVAL: u64 = 128;

/// Counts how many times drain_keyboard_data has found at least one scancode.
static PS2_KBD_DRAIN_COUNT: AtomicU64 = AtomicU64::new(0);
static PS2_KBD_BYTE_COUNT: AtomicU64 = AtomicU64::new(0);

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
    stem::debug!("PS/2 keyboard service online; waiting for bristle pid");

    if let Err(e) = stem::fs::wait_until_exists(BRISTLE_PID_PATH) {
        stem::error!("Failed waiting for {}: {:?}", BRISTLE_PID_PATH, e);
        loop {
            stem::time::sleep_ms(1000);
        }
    }

    let bristle_pid = match read_u32_file(BRISTLE_PID_PATH) {
        Some(pid) if pid != 0 => pid,
        _ => {
            stem::error!("Failed to read bristle pid from {}", BRISTLE_PID_PATH);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    stem::debug!("Connected PS/2 keyboard events to bristle pid={}.", bristle_pid);

    // Subscribe to keyboard interrupt
    match irq_subscribe(KBD_VECTOR) {
        Ok(()) => {
            stem::debug!("Subscribed to PS/2 keyboard IRQ1: vector=0x{:02x}", KBD_VECTOR);
            interrupt_loop(bristle_pid);
        }
        Err(e) => {
            debug!("PS/2 keyboard IRQ subscribe failed ({:?}); falling back to polling", e);
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
fn drain_keyboard_data(
    bristle_pid: u32,
    state: &mut KeyboardState,
    drop_counter: &mut u32,
) -> usize {
    let start_ns = stem::monotonic_ns();
    let mut bytes_read = 0usize;
    let mut events_sent = 0usize;
    // Read while data is available (handle burst of scancodes)
    for _ in 0..16 {
        let status = ioport_read(PS2_STATUS, 1);

        if status & STATUS_OUTPUT_FULL == 0 {
            break; // No more data
        }

        if status & STATUS_AUX_DATA == 0 {
            // Keyboard data - read and send
            let scancode = ioport_read(PS2_DATA, 1) as u8;
            bytes_read += 1;
            if bytes_read == 1 {
                let drain_no = PS2_KBD_DRAIN_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
                if should_log_input(drain_no) {
                    stem::trace!(
                        "PS/2 keyboard drain={} first_scancode=0x{:02x} status=0x{:02x}",
                        drain_no,
                        scancode,
                        status
                    );
                }
            }
            let byte_no = PS2_KBD_BYTE_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
            if should_log_input(byte_no) {
                stem::trace!(
                    "PS/2 keyboard take_scancode: scancode=0x{:02x} status=0x{:02x} drain_depth={}",
                    scancode,
                    status,
                    bytes_read
                );
            }
            if let Some(edge) = state.process_ps2(scancode) {
                stem::trace!("PS/2 keyboard edge detected: {:?}", edge);
                send_key_event(bristle_pid, edge, drop_counter);
                events_sent += 1;
            }
        } else {
            // If aux data (mouse), stop draining - let ps2_mouse handle it
            stem::trace!("PS/2 keyboard yielded on AUX data");
            break;
        }
    }
    let elapsed_ns = stem::monotonic_ns().saturating_sub(start_ns);
    if bytes_read != 0 {
        let drain_no = PS2_KBD_DRAIN_COUNT.load(Ordering::Relaxed);
        if should_log_input(drain_no) {
            stem::trace!(
                "PS/2 keyboard drain exit: bytes={} events={} elapsed_ns={} dropped={}",
                bytes_read,
                events_sent,
                elapsed_ns,
                *drop_counter
            );
        }
    }
    bytes_read
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
    if send_ok {
        stem::trace!("Sent PS/2 key {:?} to bristle pid={}", key, bristle_pid);
    } else {
        *drop_counter = drop_counter.wrapping_add(1);
        if *drop_counter <= 4 || *drop_counter % 100 == 0 {
            warn!("Dropped {} key events: send to pid={} failed", *drop_counter, bristle_pid);
        }
    }
}

/// IRQ-assisted service loop (primary path).
///
/// Waits briefly for IRQ1, then drains all pending scancodes even if the wait
/// timed out. The timeout is a safety net for the kernel IRQ path: if an input
/// interrupt arrives while the scheduler lock is held, the IRQ handler records
/// the pending count but deliberately skips the blocking wake.
fn interrupt_loop(bristle_pid: u32) -> ! {
    stem::debug!(
        "Using PS/2 keyboard IRQ-assisted loop: vector=0x{:02x} poll={}ms",
        KBD_VECTOR,
        IRQ_ASSIST_POLL_MS
    );
    let mut waitset = WaitSet::new();
    let irq_token = match waitset.add_irq(KBD_VECTOR as u64) {
        Ok(token) => token,
        Err(e) => {
            warn!("Failed to add PS/2 keyboard IRQ wait source ({:?}); switching to polling", e);
            polling_loop(bristle_pid);
        }
    };

    let mut state = KeyboardState::new();
    let mut drop_counter = 0u32;
    let mut irq_wake_count = 0u64;
    let mut timeout_count = 0u64;
    let mut input_count = 0u64;
    let mut rate_window_start_ns = stem::monotonic_ns();
    let mut rate_window_input = 0u64;
    loop {
        if should_log_input(irq_wake_count + timeout_count + 1) {
            stem::trace!(
                "ps2_kbd IRQ wait: vector=0x{:02x} wakes={} timeouts={} input_total={} dropped={}.",
                KBD_VECTOR,
                irq_wake_count,
                timeout_count,
                input_count,
                drop_counter
            );
        }
        match waitset.wait(Some(Duration::from_millis(IRQ_ASSIST_POLL_MS))) {
            Ok(events) => {
                let mut saw_irq = false;
                let mut pending = 0i64;
                for event in events {
                    if event.token() == irq_token && event.is_irq() {
                        irq_wake_count = irq_wake_count.wrapping_add(1);
                        pending = event.value();
                        saw_irq = true;
                    }
                }
                if saw_irq && should_log_input(irq_wake_count) {
                    stem::trace!(
                        "PS/2 IRQ wake entry: vector=0x{:02x} wakes={} pending={}",
                        KBD_VECTOR,
                        irq_wake_count,
                        pending
                    );
                }
                if !saw_irq {
                    timeout_count = timeout_count.wrapping_add(1);
                }
                if saw_irq && should_log_input(irq_wake_count) {
                    stem::trace!(
                        "PS/2 keyboard irq_wait exit: vector=0x{:02x} wakes={} timeouts={} pending={}",
                        KBD_VECTOR,
                        irq_wake_count,
                        timeout_count,
                        pending
                    );
                }
                let drained = drain_keyboard_data(bristle_pid, &mut state, &mut drop_counter);
                input_count = input_count.wrapping_add(drained as u64);
                rate_window_input = rate_window_input.wrapping_add(drained as u64);
                let now_ns = stem::monotonic_ns();
                let window_ns = now_ns.saturating_sub(rate_window_start_ns);
                if window_ns >= 1_000_000_000 {
                    stem::debug!(
                        "ps2_kbd input rate: bytes_per_sec={} total={} irq_wakes={} dropped={}.",
                        rate_window_input,
                        input_count,
                        irq_wake_count,
                        drop_counter
                    );
                    rate_window_start_ns = now_ns;
                    rate_window_input = 0;
                }
            }
            Err(e) => {
                warn!(
                    "PS/2 keyboard IRQ wait error ({:?}) after {} wakes; switching to polling fallback",
                    e, irq_wake_count
                );
                break;
            }
        }
    }
    polling_loop(bristle_pid)
}

fn should_log_input(count: u64) -> bool {
    count <= INPUT_TRACE_INITIAL || count % INPUT_TRACE_INTERVAL == 0
}

/// Fallback polling loop – used only when IRQ subscription is unavailable.
fn polling_loop(bristle_pid: u32) -> ! {
    stem::debug!("Using PS/2 keyboard fallback polling loop: interval={}ms", POLLING_INTERVAL_MS);
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
