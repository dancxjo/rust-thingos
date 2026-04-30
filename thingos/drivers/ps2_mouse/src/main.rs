//! PS/2 Mouse Driver (IRQ-assisted)
//!
//! Subscribes to IRQ12 via IOAPIC and drains pending PS/2 bytes when the IRQ
//! fires. The wait is deliberately time-bounded so the mouse still works when
//! virtual hardware or interrupt routing fails to deliver IRQ12.
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
use stem::syscall::{ioport_read, ioport_write, irq_subscribe};
use stem::time::Duration;
use stem::wait_set::WaitSet;
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
const IRQ_ASSIST_POLL_MS: u64 = 4;
const POINTER_MOTION_MIN_INTERVAL_NS: u64 = 16_666_666;

/// Preferred PS/2 mouse sample rate (Hz). Higher rates give smoother pointer motion.
const PREFERRED_SAMPLE_RATE: u8 = 200;
/// Fallback sample rate used when the device rejects `PREFERRED_SAMPLE_RATE`.
const FALLBACK_SAMPLE_RATE: u8 = 100;

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

/// Send the Set Sample Rate command (0xF3) followed by `rate`.
///
/// Returns `true` if the device acknowledged both bytes with 0xFA, `false`
/// otherwise. Callers should fall back to a lower rate on failure.
fn set_sample_rate(rate: u8) -> bool {
    send_aux_byte(0xF3);
    let cmd_ack = read_data_filtered(true, "sample rate cmd ACK").unwrap_or(0);
    if cmd_ack != 0xFA {
        debug!("ps2_mouse: sample rate cmd NACK (0x{:02x}) for rate={}", cmd_ack, rate);
        return false;
    }
    send_aux_byte(rate);
    let rate_ack = read_data_filtered(true, "sample rate set ACK").unwrap_or(0);
    if rate_ack != 0xFA {
        debug!("ps2_mouse: sample rate set NACK (0x{:02x}) for rate={}", rate_ack, rate);
        return false;
    }
    true
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

    debug!("ps2_mouse: requesting sample rate {} Hz", PREFERRED_SAMPLE_RATE);
    let _ = if set_sample_rate(PREFERRED_SAMPLE_RATE) {
        info!("ps2_mouse: sample rate set to {} Hz", PREFERRED_SAMPLE_RATE);
        PREFERRED_SAMPLE_RATE
    } else {
        debug!(
            "ps2_mouse: {} Hz rejected, falling back to {} Hz",
            PREFERRED_SAMPLE_RATE, FALLBACK_SAMPLE_RATE
        );
        if set_sample_rate(FALLBACK_SAMPLE_RATE) {
            info!("ps2_mouse: sample rate set to {} Hz (fallback)", FALLBACK_SAMPLE_RATE);
        } else {
            warn!(
                "ps2_mouse: fallback sample rate {} Hz also failed; continuing with device default",
                FALLBACK_SAMPLE_RATE
            );
        }
        FALLBACK_SAMPLE_RATE
    };

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
    stem::debug!("ps2_mouse: online — waiting for bristle pid");

    if let Err(e) = stem::fs::wait_until_exists(BRISTLE_PID_PATH) {
        stem::error!("ps2_mouse: failed waiting for {}: {:?}", BRISTLE_PID_PATH, e);
        loop {
            stem::time::sleep_ms(1000);
        }
    }

    let bristle_pid = match read_u32_file(BRISTLE_PID_PATH) {
        Some(pid) if pid != 0 => pid,
        _ => {
            stem::error!("ps2_mouse: failed to read bristle pid from {}", BRISTLE_PID_PATH);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    stem::info!("ps2_mouse: bristle pid={}", bristle_pid);

    init_mouse();

    // Subscribe to mouse interrupt
    match irq_subscribe(MOUSE_VECTOR) {
        Ok(()) => {
            debug!("ps2_mouse: subscribed to IRQ12 (vector 0x{:02x})", MOUSE_VECTOR);
            interrupt_loop(bristle_pid);
        }
        Err(e) => {
            debug!("ps2_mouse: IRQ subscribe failed ({:?}), falling back to polling", e);
            polling_loop(bristle_pid);
        }
    }
}

mod mouse;

use abi::hid::{
    BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION, BristleEventHeader, EventType,
    KIND_BRISTLE_DEVICE_EVENT, PointerButtonPayload, PointerMovePayload,
};
use mouse::{MouseState, PointerEvent};

#[derive(Default)]
struct MotionCoalescer {
    pending_dx: i32,
    pending_dy: i32,
    last_sent_ns: u64,
}

impl MotionCoalescer {
    fn add(&mut self, dx: i16, dy: i16) {
        self.pending_dx = self.pending_dx.saturating_add(dx as i32);
        self.pending_dy = self.pending_dy.saturating_add(dy as i32);
    }

    fn has_pending(&self) -> bool {
        self.pending_dx != 0 || self.pending_dy != 0
    }

    fn due(&self, now_ns: u64) -> bool {
        self.has_pending()
            && (self.last_sent_ns == 0
                || now_ns.saturating_sub(self.last_sent_ns) >= POINTER_MOTION_MIN_INTERVAL_NS)
    }

    fn take(&mut self, now_ns: u64) -> (i16, i16) {
        let dx = clamp_i16(self.pending_dx);
        let dy = clamp_i16(self.pending_dy);
        self.pending_dx = self.pending_dx.saturating_sub(dx as i32);
        self.pending_dy = self.pending_dy.saturating_sub(dy as i32);
        self.last_sent_ns = now_ns;
        (dx, dy)
    }
}

fn clamp_i16(value: i32) -> i16 {
    value.clamp(i16::MIN as i32, i16::MAX as i32) as i16
}

fn send_pointer_event(bristle_pid: u32, event_type: EventType, payload: &[u8]) -> bool {
    let timestamp_ns = stem::monotonic_ns();
    let mut buf = [0u8; 24];
    let header = BristleEventHeader {
        magic: BRISTLE_EVENT_MAGIC,
        version: BRISTLE_EVENT_VERSION,
        event_type: event_type as u16,
        timestamp_ns,
        payload_len: payload.len() as u32,
    };
    let len = BristleEventHeader::SIZE + payload.len();
    buf[..BristleEventHeader::SIZE].copy_from_slice(&header.to_bytes());
    buf[BristleEventHeader::SIZE..len].copy_from_slice(payload);

    msg_send(bristle_pid, abi::KindId(KIND_BRISTLE_DEVICE_EVENT), &buf[..len]).is_ok()
}

fn record_drop(drop_counter: &mut u32, bristle_pid: u32) {
    *drop_counter = drop_counter.wrapping_add(1);
    if *drop_counter <= 4 || *drop_counter % 100 == 0 {
        debug!(
            "ps2_mouse: dropped {} mouse events (send pid={} failed)",
            *drop_counter, bristle_pid
        );
    }
}

fn flush_motion_if_due(
    bristle_pid: u32,
    motion: &mut MotionCoalescer,
    drop_counter: &mut u32,
) {
    let now_ns = stem::monotonic_ns();
    if !motion.due(now_ns) {
        return;
    }

    let (dx, dy) = motion.take(now_ns);
    if dx == 0 && dy == 0 {
        return;
    }

    let payload = PointerMovePayload { dx, dy };
    if !send_pointer_event(
        bristle_pid,
        EventType::PointerMove,
        &payload.to_bytes(),
    ) {
        record_drop(drop_counter, bristle_pid);
    }
}

fn send_mouse_events(
    bristle_pid: u32,
    state: &mut MouseState,
    motion: &mut MotionCoalescer,
    packet: &[u8; 3],
    drop_counter: &mut u32,
) {
    let (events, count) = state.process_packet(packet);
    for i in 0..count {
        if let Some(evt) = events[i] {
            match evt {
                PointerEvent::Move { dx, dy } => {
                    motion.add(dx, dy);
                    flush_motion_if_due(bristle_pid, motion, drop_counter);
                }
                PointerEvent::ButtonDown { button } => {
                    flush_motion_if_due(bristle_pid, motion, drop_counter);
                    let payload = PointerButtonPayload { button, _pad: 0 };
                    if !send_pointer_event(
                        bristle_pid,
                        EventType::PointerButtonDown,
                        &payload.to_bytes(),
                    ) {
                        record_drop(drop_counter, bristle_pid);
                    }
                }
                PointerEvent::ButtonUp { button } => {
                    flush_motion_if_due(bristle_pid, motion, drop_counter);
                    let payload = PointerButtonPayload { button, _pad: 0 };
                    if !send_pointer_event(
                        bristle_pid,
                        EventType::PointerButtonUp,
                        &payload.to_bytes(),
                    ) {
                        record_drop(drop_counter, bristle_pid);
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
    bristle_pid: u32,
    state: &mut MouseState,
    motion: &mut MotionCoalescer,
    packet: &mut [u8; 3],
    idx: &mut usize,
    drop_counter: &mut u32,
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
                send_mouse_events(bristle_pid, state, motion, packet, drop_counter);
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

/// IRQ-assisted service loop (primary path).
///
/// Waits briefly for IRQ12, then drains all pending PS/2 bytes even if the wait
/// timed out. This keeps normal interrupt latency low without making pointer
/// updates depend on a perfectly routed IRQ line.
fn interrupt_loop(bristle_pid: u32) -> ! {
    debug!(
        "ps2_mouse: using IRQ-assisted loop (IRQ vector 0x{:02x}, poll={}ms)",
        MOUSE_VECTOR, IRQ_ASSIST_POLL_MS
    );

    let mut waitset = WaitSet::new();
    let irq_token = match waitset.add_irq(MOUSE_VECTOR as u64) {
        Ok(token) => token,
        Err(e) => {
            warn!("ps2_mouse: failed to add IRQ wait source ({:?}), switching to polling", e);
            polling_loop(bristle_pid);
        }
    };

    let mut packet = [0u8; 3];
    let mut idx = 0usize;
    let mut mouse_state = MouseState::new();
    let mut motion = MotionCoalescer::default();
    let mut drop_counter = 0u32;
    let mut irq_wake_count = 0u64;
    let mut timeout_count = 0u64;

    loop {
        match waitset.wait(Some(Duration::from_millis(IRQ_ASSIST_POLL_MS))) {
            Ok(events) => {
                let mut saw_irq = false;
                for event in events {
                    if event.token() == irq_token && event.is_irq() {
                        irq_wake_count = irq_wake_count.wrapping_add(1);
                        saw_irq = true;
                    }
                }
                if !saw_irq {
                    timeout_count = timeout_count.wrapping_add(1);
                }
            }
            Err(e) => {
                warn!(
                    "ps2_mouse: IRQ wait error ({:?}) after {} wakes, switching to polling fallback",
                    e, irq_wake_count
                );
                break;
            }
        }

        let drained = drain_mouse_data(
            bristle_pid,
            &mut mouse_state,
            &mut motion,
            &mut packet,
            &mut idx,
            &mut drop_counter,
        );
        flush_motion_if_due(bristle_pid, &mut motion, &mut drop_counter);

        if (irq_wake_count + timeout_count) % 256 == 0 {
            debug!(
                "ps2_mouse: irq_wakes={} poll_timeouts={} last_drain_bytes={}",
                irq_wake_count, timeout_count, drained
            );
        }
    }
    polling_loop(bristle_pid)
}

/// Fallback polling loop – used only when IRQ subscription or wait fails.
fn polling_loop(bristle_pid: u32) -> ! {
    debug!("ps2_mouse: using fallback polling loop ({}ms interval)", POLLING_INTERVAL_MS);

    let mut packet = [0u8; 3];
    let mut idx = 0usize;
    let mut mouse_state = MouseState::new();
    let mut motion = MotionCoalescer::default();
    let mut drop_counter = 0u32;

    loop {
        let status = ioport_read(PS2_STATUS, 1);

        if status & STATUS_OUTPUT_FULL != 0 {
            if status & STATUS_AUX_DATA != 0 {
                drain_mouse_data(
                    bristle_pid,
                    &mut mouse_state,
                    &mut motion,
                    &mut packet,
                    &mut idx,
                    &mut drop_counter,
                );
                flush_motion_if_due(bristle_pid, &mut motion, &mut drop_counter);
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
