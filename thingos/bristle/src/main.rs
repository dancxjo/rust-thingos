//! Bristle: Unified HID Broker
//!
//! The sole input authority. Drivers send raw reports, apps receive
//! normalized events. Apps never see scancodes, drivers never see apps.
//!
//! # Architecture (ServiceLoop edition)
//!
//! Bristle starts with no packed argument.  At runtime:
//!
//! 1. It creates two port pairs for keyboard and mouse input, and publishes
//!    the write handles to `/run/bristle/kbd_in` and `/run/bristle/mouse_in`
//!    so that device drivers can discover them.
//! 2. It opens a `ServiceLoop` backed by its process inbox.  The inbox is the
//!    **control plane**: consumers (bloom, echo) send a `RegisterSink` message
//!    (KindId = `KIND_BRISTLE_REGISTER_SINK`) to register a port write handle.
//! 3. Device read FDs are registered as secondary sources on the ServiceLoop.
//!    When a device FD becomes readable, bristle reads, accumulates, and
//!    dispatches normalized events to all registered sinks.
#![no_std]
#![no_main]
extern crate alloc;

use core::sync::atomic::{AtomicU64, Ordering};

use abi::hid::{
    BRISTLE_EVENT_CLASS_KEYBOARD, BRISTLE_EVENT_CLASS_POINTER, BRISTLE_SINK_TAG_BLOOM,
    BRISTLE_SINK_TAG_ECHO, BristleEventHeader, EventType, KIND_BRISTLE_DEVICE_EVENT,
    KIND_BRISTLE_REGISTER_SINK, Key, KeyEventPayload, decode_register_sink_with_mask,
};
use abi::syscall::vfs_flags::{O_CREAT, O_RDWR, O_TRUNC};
use abi::wire::KindId;
use stem::service_loop::{ServiceEvent, ServiceLoop};
use stem::syscall::vfs::{
    vfs_close, vfs_handle_from_port, vfs_mkdir, vfs_open, vfs_read, vfs_write,
};
use stem::syscall::{port_close, port_create, port_send_all};
use stem::wait_set::WaitToken;
use stem::{debug, info, trace, warn};

const INPUT_TRACE_INITIAL: u64 = 24;
const INPUT_TRACE_INTERVAL: u64 = 128;
static BRISTLE_DISPATCH_COUNT: AtomicU64 = AtomicU64::new(0);
static BRISTLE_FORWARD_DROP_COUNT: AtomicU64 = AtomicU64::new(0);

fn ensure_session_roots() {
    let _ = vfs_mkdir("/session");
    let _ = vfs_mkdir("/session/seat0");
    let _ = vfs_mkdir("/session/seat0/keyboard");
    let _ = vfs_mkdir("/session/seat0/pointer");
}

fn update_active_ui(target: &str) {
    if let Ok(fd) = vfs_open("/session/active_ui", O_RDWR | O_CREAT | O_TRUNC) {
        let _ = vfs_write(fd, target.as_bytes());
        let _ = vfs_close(fd);
        stem::trace!("bristle: active_ui set to '{}'", target);
    }
}

#[derive(Clone, Copy)]
struct Sink {
    handle: u32,
    event_mask: u8,
}

/// Publish bristle's PID to `/run/bristle/pid` so consumers can find us.
fn publish_pid() {
    let _ = vfs_mkdir("/run");
    let _ = vfs_mkdir("/run/bristle");
    let pid = stem::syscall::getpid();
    if let Ok(fd) = vfs_open("/run/bristle/pid", O_RDWR | O_CREAT | O_TRUNC) {
        let text = alloc::format!("{}\n", pid);
        let _ = vfs_write(fd, text.as_bytes());
        let _ = vfs_close(fd);
        info!("bristle: published pid {} to /run/bristle/pid", pid);
    }
}

/// Write a port write handle as decimal text to `path` so device drivers can
/// open the file and convert the handle to their own VFS FD.
fn publish_device_handle(path: &str, handle: u32) {
    if let Ok(fd) = vfs_open(path, O_RDWR | O_CREAT | O_TRUNC) {
        let text = alloc::format!("{}\n", handle);
        let _ = vfs_write(fd, text.as_bytes());
        let _ = vfs_close(fd);
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    ensure_session_roots();
    update_active_ui("bloom");

    // ── Publish PID so consumers can register via inbox ───────────────────
    publish_pid();

    // ── Create device-facing port pairs ───────────────────────────────────
    // Device drivers discover the write handles via VFS files and write raw
    // bristle events into them.  We retain the read ends and poll them below.
    let (kbd_write, kbd_read) = match port_create(4096) {
        Ok(pair) => pair,
        Err(e) => {
            warn!("bristle: kbd port_create failed: {:?}", e);
            (0, 0)
        }
    };
    let (mouse_write, mouse_read) = match port_create(4096) {
        Ok(pair) => pair,
        Err(e) => {
            warn!("bristle: mouse port_create failed: {:?}", e);
            (0, 0)
        }
    };

    let _ = vfs_mkdir("/run/bristle");
    publish_device_handle("/run/bristle/kbd_in", kbd_write);
    publish_device_handle("/run/bristle/mouse_in", mouse_write);
    info!("bristle: published device handles kbd_in={} mouse_in={}", kbd_write, mouse_write);

    // ── Open ServiceLoop ──────────────────────────────────────────────────
    let mut svc = match ServiceLoop::new(64) {
        Ok(s) => s,
        Err(e) => {
            warn!("bristle: ServiceLoop::new failed: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    // ── Bridge device port read ends to VFS FDs ───────────────────────────
    // Keyboard input path.
    let kbd_fd: Option<u32> = if kbd_read != 0 {
        match vfs_handle_from_port(kbd_read) {
            Ok(fd) => Some(fd),
            Err(e) => {
                debug!("bristle: kbd fd bridge failed ({:?}), keyboard disabled", e);
                None
            }
        }
    } else {
        None
    };
    let kbd_tok: Option<WaitToken> = kbd_fd.and_then(|fd| svc.add_fd_readable(fd).ok());

    // Mouse input path.
    let mouse_fd: Option<u32> = if mouse_read != 0 {
        match vfs_handle_from_port(mouse_read) {
            Ok(fd) => Some(fd),
            Err(e) => {
                debug!("bristle: mouse fd bridge failed ({:?}), mouse disabled", e);
                None
            }
        }
    } else {
        None
    };
    let mouse_tok: Option<WaitToken> = mouse_fd.and_then(|fd| svc.add_fd_readable(fd).ok());

    info!("bristle: online (kbd_tok={:?}, mouse_tok={:?})", kbd_tok, mouse_tok);

    // ── Event-dispatch state ──────────────────────────────────────────────
    // Registered event sinks — registered via inbox RegisterSink messages.
    let mut bloom_sink: Option<Sink> = None;
    let mut echo_sink: Option<Sink> = None;

    let mut recv_buf = [0u8; 128];
    let mut kbd_event_accum = [0u8; 64];
    let mut kbd_accum_len = 0usize;
    let mut mouse_event_accum = [0u8; 64];
    let mut mouse_accum_len = 0usize;
    let mut drop_counter: u32 = 0;

    // ── Main service loop ─────────────────────────────────────────────────
    loop {
        let event = match svc.next_event(None::<stem::time::Duration>) {
            Ok(ev) => ev,
            Err(_) => {
                stem::time::sleep_ms(10);
                continue;
            }
        };

        match event {
            // ── Control plane: sink registration via inbox ─────────────
            ServiceEvent::Message { kind, payload, .. } => {
                if kind == KindId(KIND_BRISTLE_REGISTER_SINK) {
                    handle_register_sink(payload, &mut bloom_sink, &mut echo_sink);
                } else if kind == KindId(KIND_BRISTLE_DEVICE_EVENT) {
                    let (event_accum, accum_len) = if is_pointer_event_payload(payload) {
                        (&mut mouse_event_accum, &mut mouse_accum_len)
                    } else {
                        (&mut kbd_event_accum, &mut kbd_accum_len)
                    };
                    accumulate_and_dispatch(
                        payload,
                        "inbox",
                        event_accum,
                        accum_len,
                        bloom_sink,
                        echo_sink,
                        &mut drop_counter,
                    );
                } else {
                    trace!("bristle: unknown inbox message kind {:?}", kind.0);
                }
            }

            // ── Data plane: device input ───────────────────────────────
            ServiceEvent::Ready { token, event: ev } if ev.is_readable() => {
                let is_kbd = Some(token) == kbd_tok;
                let is_mouse = Some(token) == mouse_tok;
                let n_result = if is_kbd {
                    if let Some(fd) = kbd_fd {
                        vfs_read(fd, &mut recv_buf)
                    } else {
                        continue;
                    }
                } else if is_mouse {
                    if let Some(fd) = mouse_fd {
                        vfs_read(fd, &mut recv_buf)
                    } else {
                        continue;
                    }
                } else {
                    continue;
                };

                if let Ok(n) = n_result {
                    if n > 0 {
                        let (event_accum, accum_len) = if is_mouse {
                            (&mut mouse_event_accum, &mut mouse_accum_len)
                        } else {
                            (&mut kbd_event_accum, &mut kbd_accum_len)
                        };
                        accumulate_and_dispatch(
                            &recv_buf[..n],
                            if is_mouse { "mouse_fd" } else { "kbd_fd" },
                            event_accum,
                            accum_len,
                            bloom_sink,
                            echo_sink,
                            &mut drop_counter,
                        );
                    }
                }
            }

            ServiceEvent::Ready { .. } => {}

            ServiceEvent::InboxClosed => {
                warn!("bristle: inbox closed — entering degraded loop");
                loop {
                    stem::time::sleep_ms(1000);
                }
            }

            ServiceEvent::Timeout => {}
        }

        if drop_counter > 0 && drop_counter % 100 == 0 {
            warn!("bristle: dropped {} events (sink full)", drop_counter);
        }
    }
}

fn is_pointer_event_payload(payload: &[u8]) -> bool {
    if payload.len() < BristleEventHeader::SIZE {
        return false;
    }
    let mut hdr_bytes = [0u8; BristleEventHeader::SIZE];
    hdr_bytes.copy_from_slice(&payload[..BristleEventHeader::SIZE]);
    let Ok(header) = BristleEventHeader::from_bytes(&hdr_bytes) else {
        return false;
    };
    matches!(
        EventType::from_raw(header.event_type),
        Ok(EventType::PointerMove
            | EventType::PointerButtonDown
            | EventType::PointerButtonUp
            | EventType::Scroll)
    )
}

/// Handle a `RegisterSink` inbox message.
fn handle_register_sink(
    payload: &[u8],
    bloom_sink: &mut Option<Sink>,
    echo_sink: &mut Option<Sink>,
) {
    let Some((tag, handle, event_mask)) = decode_register_sink_with_mask(payload) else {
        warn!("bristle: RegisterSink payload too short ({} bytes)", payload.len());
        return;
    };
    let sink = Sink { handle, event_mask };

    match tag {
        BRISTLE_SINK_TAG_BLOOM => {
            if let Some(old) = bloom_sink.replace(sink) {
                if old.handle != handle {
                    let _ = port_close(old.handle);
                }
            }
            info!("bristle: bloom sink registered (handle={} mask=0x{:02x})", handle, event_mask);
        }
        BRISTLE_SINK_TAG_ECHO => {
            if let Some(old) = echo_sink.replace(sink) {
                if old.handle != handle {
                    let _ = port_close(old.handle);
                }
            }
            info!("bristle: echo sink registered (handle={} mask=0x{:02x})", handle, event_mask);
        }
        _ => {
            warn!("bristle: RegisterSink unknown tag {}", tag);
            let _ = port_close(handle);
        }
    }
}

/// Accumulate raw bytes into `event_accum`, parse complete bristle events,
/// apply hotkey handling, and forward to registered sinks.
fn accumulate_and_dispatch(
    input: &[u8],
    source: &str,
    event_accum: &mut [u8; 64],
    accum_len: &mut usize,
    bloom_sink: Option<Sink>,
    echo_sink: Option<Sink>,
    drop_counter: &mut u32,
) {
    let start_ns = stem::monotonic_ns();
    let entry_depth = *accum_len;
    let mut cursor = 0;
    let mut dispatched = 0u64;
    while cursor < input.len() {
        let to_copy = (input.len() - cursor).min(64 - *accum_len);
        event_accum[*accum_len..*accum_len + to_copy]
            .copy_from_slice(&input[cursor..cursor + to_copy]);
        *accum_len += to_copy;
        cursor += to_copy;

        while *accum_len >= BristleEventHeader::SIZE {
            let mut header_bytes = [0u8; BristleEventHeader::SIZE];
            header_bytes.copy_from_slice(&event_accum[..BristleEventHeader::SIZE]);

            if let Ok(header) = BristleEventHeader::from_bytes(&header_bytes) {
                let event_type = header.event_type;
                let payload_len = header.payload_len;
                let total_len = BristleEventHeader::SIZE + payload_len as usize;
                if *accum_len >= total_len {
                    let event_bytes = &event_accum[..total_len];
                    let event_no = BRISTLE_DISPATCH_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
                    dispatched = dispatched.wrapping_add(1);
                    if should_log_input(event_no) {
                        stem::info!(
                            "bristle: dispatch entry source={} event={} type={} payload_len={} accum_depth={} input_len={}",
                            source,
                            event_no,
                            event_type,
                            payload_len,
                            *accum_len,
                            input.len()
                        );
                    }

                    // Hotkey handling
                    if event_type == EventType::KeyDown as u16 && payload_len >= 4 {
                        let mut p = [0u8; 4];
                        p.copy_from_slice(&event_bytes[20..24]);
                        let payload = KeyEventPayload::from_bytes(&p);
                        stem::info!("bristle: KeyDown received: {:?}", payload.key());

                        match payload.key() {
                            Key::F2 => {
                                stem::info!("bristle: F2 pressed - dumping tasks...");
                                stem::syscall::task_dump();
                            }
                            Key::Delete
                                if payload.mods().has_ctrl() && payload.mods().has_alt() =>
                            {
                                stem::info!("bristle: Ctrl+Alt+Del - rebooting...");
                                stem::syscall::reboot();
                            }
                            Key::F11 => {
                                update_active_ui("bloom");
                            }
                            Key::F12 => {
                                update_active_ui("terminal");
                            }
                            _ => {}
                        }
                    }

                    // Forward to registered sinks.
                    let event_class = event_class(event_type);
                    if let Some(sink) = bloom_sink {
                        if sink.accepts(event_class)
                            && port_send_all(sink.handle, event_bytes).is_err()
                        {
                            *drop_counter += 1;
                            BRISTLE_FORWARD_DROP_COUNT.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    if let Some(sink) = echo_sink {
                        if sink.accepts(event_class)
                            && port_send_all(sink.handle, event_bytes).is_err()
                        {
                            *drop_counter += 1;
                            BRISTLE_FORWARD_DROP_COUNT.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    if should_log_input(event_no) {
                        stem::info!(
                            "bristle: dispatch exit source={} event={} class={} accum_depth={} drops={}",
                            source,
                            event_no,
                            event_class,
                            *accum_len,
                            *drop_counter
                        );
                    }

                    // Shift remaining bytes to the start of the accumulator.
                    *accum_len -= total_len;
                    if *accum_len > 0 {
                        event_accum.copy_within(total_len..total_len + *accum_len, 0);
                    }
                } else {
                    break; // Need more data.
                }
            } else {
                // Invalid header — resync by dropping one byte.
                *accum_len -= 1;
                if *accum_len > 0 {
                    event_accum.copy_within(1..1 + *accum_len, 0);
                }
            }
        }
    }
    if dispatched != 0 {
        let total = BRISTLE_DISPATCH_COUNT.load(Ordering::Relaxed);
        if should_log_input(total) {
            stem::info!(
                "bristle: input_rate source={} bytes={} events={} total_events={} entry_depth={} exit_depth={} elapsed_ns={} drops={}",
                source,
                input.len(),
                dispatched,
                total,
                entry_depth,
                *accum_len,
                stem::monotonic_ns().saturating_sub(start_ns),
                BRISTLE_FORWARD_DROP_COUNT.load(Ordering::Relaxed)
            );
        }
    }
}

impl Sink {
    fn accepts(self, event_class: u8) -> bool {
        event_class == 0 || (self.event_mask & event_class) != 0
    }
}

fn event_class(event_type: u16) -> u8 {
    match EventType::from_raw(event_type) {
        Ok(EventType::KeyDown | EventType::KeyUp) => BRISTLE_EVENT_CLASS_KEYBOARD,
        Ok(
            EventType::PointerMove
            | EventType::PointerButtonDown
            | EventType::PointerButtonUp
            | EventType::Scroll,
        ) => BRISTLE_EVENT_CLASS_POINTER,
        _ => 0,
    }
}

fn should_log_input(count: u64) -> bool {
    count <= INPUT_TRACE_INITIAL || count % INPUT_TRACE_INTERVAL == 0
}
