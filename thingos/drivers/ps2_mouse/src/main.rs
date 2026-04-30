//! PS/2 Mouse Driver (Interrupt-driven)
//!
//! Subscribes to IRQ12 via IOAPIC and blocks on `irq_wait` for each interrupt.
//! All pending PS/2 bytes are drained on each IRQ wake before waiting again.
//! Falls back to a time-bounded polling loop only when IRQ delivery is unavailable.
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
use stem::syscall::port::port_send_all;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};
use stem::syscall::{ioport_read, ioport_write, irq_subscribe, irq_wait};
use stem::{debug, error, info, warn};

const THINGOS_DRIVER_NAME: &[u8] = b"ps2_mouse";

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
    start: thingos_driver_start_safe,
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
        // Store RDI (boot_fd) and align stack to 16n + 8 for SysV.
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
    device_kind: device_kind_bytes(b"drv.Ps2Mouse"),
    version: 1,
    _reserved: 0,
};

const PS2_DATA: usize = 0x60;
const PS2_STATUS: usize = 0x64;
const PS2_CMD: usize = 0x64;

const STATUS_OUTPUT_FULL: usize = 0x01;
const STATUS_AUX_DATA: usize = 0x20;

const CMD_ENABLE_AUX: u8 = 0xA8;
const CMD_READ_CFG: u8 = 0x20;
const CMD_WRITE_CFG: u8 = 0x60;
const CMD_WRITE_AUX: u8 = 0xD4;
const MOUSE_ENABLE: u8 = 0xF4;

/// IRQ12 vector (mouse) - legacy IRQ12 maps to vector 0x2C after IOAPIC remap
const MOUSE_VECTOR: u8 = 0x2C;
const POLLING_INTERVAL_MS: u64 = 8;

fn wait_input_empty() {
    for _ in 0..10000 {
        if ioport_read(PS2_STATUS, 1) & 0x02 == 0 {
            return;
        }
        stem::yield_now();
    }
}

fn flush_output_buffer() {
    // Drain up to 16 bytes of garbage
    for _ in 0..16 {
        if ioport_read(PS2_STATUS, 1) & STATUS_OUTPUT_FULL != 0 {
            let b = ioport_read(PS2_DATA, 1);
            debug!("ps2_mouse: flushed garbage byte: 0x{:02x}", b);
        } else {
            break;
        }
        stem::yield_now();
    }
}

fn read_data_filtered(expect_aux: bool, label: &str) -> Option<u8> {
    let discarded_aux: u32 = 0;
    let discarded_non_aux: u32 = 0;
    for _ in 0..20_000 {
        let status = ioport_read(PS2_STATUS, 1);
        if status & STATUS_OUTPUT_FULL == 0 {
            stem::yield_now();
            continue;
        }
        let is_aux = (status & STATUS_AUX_DATA) != 0;

        if is_aux != expect_aux {
            // Leave bytes for the matching side of the shared controller.
            stem::yield_now();
            continue;
        }
        let byte = ioport_read(PS2_DATA, 1) as u8;
        return Some(byte);
    }
    debug!(
        "ps2_mouse: timed out waiting for {} (discarded_aux={}, discarded_non_aux={})",
        label, discarded_aux, discarded_non_aux
    );
    None
}

fn read_controller_config() -> u8 {
    wait_input_empty();
    // Flush any pending data (e.g. key scancodes) before asking for config
    flush_output_buffer();
    ioport_write(PS2_CMD, CMD_READ_CFG as usize, 1);
    if let Some(val) = read_data_filtered(false, "controller cfg") {
        return val;
    }
    // Fallback if we keep getting garbage
    debug!("ps2_mouse: read_cfg failed, assuming default safe config (0x47)");
    0x47 // IRQ1, IRQ12, SysFlag, Translation
}

fn write_controller_config(cfg: u8) {
    wait_input_empty();
    ioport_write(PS2_CMD, CMD_WRITE_CFG as usize, 1);
    wait_input_empty();
    ioport_write(PS2_DATA, cfg as usize, 1);
}

fn send_aux_byte(byte: u8) {
    wait_input_empty();
    ioport_write(PS2_CMD, CMD_WRITE_AUX as usize, 1);
    wait_input_empty();
    ioport_write(PS2_DATA, byte as usize, 1);
}

fn init_mouse() {
    debug!("ps2_mouse: enabling aux port");

    // Clear any initial garbage
    flush_output_buffer();

    // Enable aux port
    wait_input_empty();
    ioport_write(PS2_CMD, CMD_ENABLE_AUX as usize, 1);
    for _ in 0..10 {
        stem::yield_now();
    }

    // Ensure IRQ12 is enabled (Bit 1) and Mouse Disabled (Bit 5) is CLEARED.
    // Bit 5: 1 = Mouse Disabled, 0 = Mouse Enabled.
    let cfg = read_controller_config();

    // Force: Set Bit 1 (IRQ12), Clear Bit 5 (Mouse Disable)
    let new_cfg = (cfg | 0x02) & !0x20;

    if new_cfg != cfg {
        write_controller_config(new_cfg);
        debug!("ps2_mouse: updated controller cfg 0x{:02x} -> 0x{:02x}", cfg, new_cfg);
    } else {
        debug!("ps2_mouse: controller cfg already correct (0x{:02x})", cfg);
    }

    // Reset mouse (0xFF)
    debug!("ps2_mouse: sending RESET (0xFF)");
    send_aux_byte(0xFF);
    let ack = read_data_filtered(true, "reset ACK (0xfa)").unwrap_or(0);
    if ack == 0xFA {
        debug!("ps2_mouse: reset ACK received (0xfa)");
        let bat = read_data_filtered(true, "BAT byte (0xAA)").unwrap_or(0);
        let id = read_data_filtered(true, "Device ID (0x00)").unwrap_or(1);
        debug!("ps2_mouse: BAT passed (0x{:02x}), ID 0x{:02x} confirmed", bat, id);
    }

    debug!("ps2_mouse: setting sample rate (100)");
    send_aux_byte(0xF3);
    read_data_filtered(true, "sample rate ACK");
    send_aux_byte(100);
    read_data_filtered(true, "sample rate set ACK");

    debug!("ps2_mouse: setting resolution (3)");
    send_aux_byte(0xE8);
    read_data_filtered(true, "resolution ACK");
    send_aux_byte(3);
    read_data_filtered(true, "resolution set ACK");

    send_aux_byte(0xE9);
    let _s_ack = read_data_filtered(true, "status request ACK");
    let b1 = read_data_filtered(true, "status byte 1").unwrap_or(0);
    let b2 = read_data_filtered(true, "status byte 2").unwrap_or(0);
    let b3 = read_data_filtered(true, "status byte 3").unwrap_or(0);
    debug!("ps2_mouse: status result = Some({}) Some({}) Some({})", b1, b2, b3);

    // Bit 5 indicates Enable/Disable status (1 = Enabled, 0 = Disabled).
    // If it is 0, data reporting is disabled, so we must enable it.
    if b1 & 0x20 == 0 {
        // Enable mouse data reporting (0xF4)
        debug!("ps2_mouse: sending enable command (0xF4)");
        send_aux_byte(MOUSE_ENABLE);
        let e_ack = read_data_filtered(true, "enable ACK (0xFA)").unwrap_or(0);
        debug!("ps2_mouse: enable ACK received (0x{:02x})", e_ack);
    } else {
        debug!("ps2_mouse: already enabled, skipping 0xF4 command");
    }

    stem::sleep_ms(100);

    // Drain any lingering response bytes.
    for _ in 0..10 {
        if ioport_read(PS2_STATUS, 1) & STATUS_OUTPUT_FULL != 0 {
            let byte = ioport_read(PS2_DATA, 1) as u8;
            debug!("ps2_mouse: drained 0x{:02x}", byte);
        }
        stem::sleep_ms(10);
    }

    debug!("ps2_mouse: init done");
}

/// Path where bristle publishes its mouse input port write handle.
const BRISTLE_MOUSE_IN_PATH: &str = "/run/bristle/mouse_in";

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
    stem::debug!("ps2_mouse: online — waiting for bristle mouse_in port");

    if let Err(e) = stem::fs::wait_until_exists(BRISTLE_MOUSE_IN_PATH) {
        stem::error!("ps2_mouse: failed waiting for {}: {:?}", BRISTLE_MOUSE_IN_PATH, e);
        loop {
            stem::time::sleep_ms(1000);
        }
    }

    let mouse_port = match read_u32_file(BRISTLE_MOUSE_IN_PATH) {
        Some(h) if h != 0 => h,
        _ => {
            stem::error!(
                "ps2_mouse: failed to read mouse port handle from {}",
                BRISTLE_MOUSE_IN_PATH
            );
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    stem::info!(
        "ps2_mouse: routing events via /run/bristle/mouse_in (port handle={})",
        mouse_port
    );

    init_mouse();

    // Subscribe to mouse interrupt
    match irq_subscribe(MOUSE_VECTOR) {
        Ok(()) => {
            debug!("ps2_mouse: subscribed to IRQ12 (vector 0x{:02x})", MOUSE_VECTOR);
            interrupt_loop(mouse_port);
        }
        Err(e) => {
            debug!("ps2_mouse: IRQ subscribe failed ({:?}), falling back to polling", e);
            polling_loop(mouse_port);
        }
    }
}

mod mouse;

use abi::hid::{
    BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION, BristleEventHeader, EventType,
    PointerButtonPayload, PointerMovePayload,
};
use mouse::{MouseState, PointerEvent};

fn send_mouse_events(
    mouse_port: u32,
    state: &mut MouseState,
    packet: &[u8; 3],
    drop_counter: &mut u32,
    send_counter: &mut u64,
) {
    let (events, count) = state.process_packet(packet);
    for i in 0..count {
        if let Some(evt) = events[i] {
            let timestamp_ns = stem::monotonic_ns();
            let mut buf = [0u8; 24]; // Max size is header + 4 byte payload
            let len;

            match evt {
                PointerEvent::Move { dx, dy } => {
                    let header = BristleEventHeader {
                        magic: BRISTLE_EVENT_MAGIC,
                        version: BRISTLE_EVENT_VERSION,
                        event_type: EventType::PointerMove as u16,
                        timestamp_ns,
                        payload_len: PointerMovePayload::SIZE as u32,
                    };
                    let payload = PointerMovePayload { dx, dy };
                    buf[0..20].copy_from_slice(&header.to_bytes());
                    buf[20..24].copy_from_slice(&payload.to_bytes());
                    len = 24;
                }
                PointerEvent::ButtonDown { button } => {
                    let header = BristleEventHeader {
                        magic: BRISTLE_EVENT_MAGIC,
                        version: BRISTLE_EVENT_VERSION,
                        event_type: EventType::PointerButtonDown as u16,
                        timestamp_ns,
                        payload_len: PointerButtonPayload::SIZE as u32,
                    };
                    let payload = PointerButtonPayload { button, _pad: 0 };
                    buf[0..20].copy_from_slice(&header.to_bytes());
                    buf[20..22].copy_from_slice(&payload.to_bytes());
                    len = 22;
                }
                PointerEvent::ButtonUp { button } => {
                    let header = BristleEventHeader {
                        magic: BRISTLE_EVENT_MAGIC,
                        version: BRISTLE_EVENT_VERSION,
                        event_type: EventType::PointerButtonUp as u16,
                        timestamp_ns,
                        payload_len: PointerButtonPayload::SIZE as u32,
                    };
                    let payload = PointerButtonPayload { button, _pad: 0 };
                    buf[0..20].copy_from_slice(&header.to_bytes());
                    buf[20..22].copy_from_slice(&payload.to_bytes());
                    len = 22;
                }
            }
            if len > 0 {
                match port_send_all(mouse_port, &buf[..len]) {
                    Ok(_) => {
                        *send_counter = send_counter.wrapping_add(1);
                        // Periodic log (every 256 events) to confirm port usage.
                        if *send_counter % 256 == 0 {
                            debug!(
                                "ps2_mouse: {} events sent via /run/bristle/mouse_in (port={})",
                                *send_counter, mouse_port
                            );
                        }
                    }
                    Err(_) => {
                        *drop_counter = drop_counter.wrapping_add(1);
                        if *drop_counter <= 4 || *drop_counter % 100 == 0 {
                            debug!(
                                "ps2_mouse: dropped {} mouse events (port_send port={} failed)",
                                *drop_counter, mouse_port
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Drain all pending mouse data and assemble packets.
///
/// Returns the number of raw bytes consumed from the PS/2 FIFO.
fn drain_mouse_data(
    mouse_port: u32,
    state: &mut MouseState,
    packet: &mut [u8; 3],
    idx: &mut usize,
    drop_counter: &mut u32,
    send_counter: &mut u64,
) -> usize {
    let mut bytes_read = 0usize;
    for _ in 0..16 {
        let status = ioport_read(PS2_STATUS, 1);

        if status & STATUS_OUTPUT_FULL == 0 {
            break;
        }

        if status & STATUS_AUX_DATA != 0 {
            let byte = ioport_read(PS2_DATA, 1) as u8;
            bytes_read += 1;

            // First byte must have bit 3 set (sync)
            if *idx == 0 && (byte & 0x08) == 0 {
                continue;
            }

            packet[*idx] = byte;
            *idx += 1;

            if *idx == 3 {
                send_mouse_events(mouse_port, state, packet, drop_counter, send_counter);
                *idx = 0;
            }
        } else {
            // Shared i8042 controller: leave keyboard bytes for ps2_kbd.
            // Consuming them here makes keyboard input appear dead.
            break;
        }
    }
    bytes_read
}

/// Interrupt-driven service loop (primary path).
///
/// Blocks on `irq_wait` until IRQ12 fires, then drains all pending PS/2 bytes.
/// This avoids the previous polling cadence (8 ms interval + 1 ms unconditional
/// sleep per iteration) and removes pointer jitter during normal operation.
/// If `irq_wait` returns an error the driver falls back to the bounded polling
/// loop so the cursor never becomes completely unresponsive.
fn interrupt_loop(mouse_port: u32) -> ! {
    debug!(
        "ps2_mouse: using interrupt-driven loop (IRQ vector 0x{:02x})",
        MOUSE_VECTOR
    );
    let mut packet = [0u8; 3];
    let mut idx = 0usize;
    let mut mouse_state = MouseState::new();
    let mut drop_counter = 0u32;
    let mut send_counter = 0u64;
    let mut irq_wake_count = 0u64;

    loop {
        match irq_wait(MOUSE_VECTOR) {
            Ok(pending) => {
                irq_wake_count += 1;
                let drained = drain_mouse_data(
                    mouse_port,
                    &mut mouse_state,
                    &mut packet,
                    &mut idx,
                    &mut drop_counter,
                    &mut send_counter,
                );
                // Periodic diagnostic log (every 256 wakes) to confirm IRQ delivery.
                if irq_wake_count % 256 == 0 {
                    debug!(
                        "ps2_mouse: irq_wakes={} pending={} last_drain_bytes={} port_sends={}",
                        irq_wake_count, pending, drained, send_counter
                    );
                }
            }
            Err(e) => {
                // irq_wait should not fail once subscribed; if it does, switch to
                // the fallback polling loop so the driver keeps functioning.
                warn!(
                    "ps2_mouse: irq_wait error ({:?}) after {} wakes, switching to polling fallback",
                    e, irq_wake_count
                );
                break;
            }
        }
    }
    polling_loop(mouse_port)
}

/// Fallback polling loop – used only when IRQ subscription or wait fails.
fn polling_loop(mouse_port: u32) -> ! {
    debug!(
        "ps2_mouse: using fallback polling loop ({}ms interval)",
        POLLING_INTERVAL_MS
    );

    let mut packet = [0u8; 3];
    let mut idx = 0usize;
    let mut mouse_state = MouseState::new();
    let mut drop_counter = 0u32;
    let mut send_counter = 0u64;

    loop {
        let status = ioport_read(PS2_STATUS, 1);

        if status & STATUS_OUTPUT_FULL != 0 {
            if status & STATUS_AUX_DATA != 0 {
                drain_mouse_data(
                    mouse_port,
                    &mut mouse_state,
                    &mut packet,
                    &mut idx,
                    &mut drop_counter,
                    &mut send_counter,
                );
            } else {
                // Leave keyboard bytes queued for ps2_kbd.
                stem::sleep_ms(POLLING_INTERVAL_MS);
            }
        } else {
            stem::sleep_ms(POLLING_INTERVAL_MS);
        }
        // No unconditional sleep here - the per-branch sleeps above are sufficient.
    }
}
