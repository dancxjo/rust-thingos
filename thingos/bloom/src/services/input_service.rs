//! `InputService` — drains normalized HID events from the bristle event FD
//! and routes them through [`BloomWorld::handle_bristle_event`].

use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

use abi::KindId;
use abi::hid::{
    BRISTLE_EVENT_CLASS_ALL, BRISTLE_SINK_TAG_BLOOM, BristleEventHeader, EventType,
    KIND_BRISTLE_REGISTER_SINK, encode_register_sink_with_mask,
};
use abi::syscall::vfs_flags::O_RDONLY;
use stem::syscall::message::msg_send;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};

use crate::loop_types::{BloomService, Interest, LoopAction, LoopEvent};
use crate::world::BloomWorld;

const REGISTER_TIMER_ID: u64 = 1;
const BRISTLE_PID_PATH: &str = "/run/bristle/pid";
const INPUT_READ_CHUNK: usize = 64;
const MAX_READS_PER_WAKE: usize = 4;
const INPUT_TRACE_INITIAL: u64 = 24;
const INPUT_TRACE_INTERVAL: u64 = 128;
static BLOOM_INPUT_READ_COUNT: AtomicU64 = AtomicU64::new(0);
static BLOOM_INPUT_EVENT_COUNT: AtomicU64 = AtomicU64::new(0);

/// Handles normalized HID events forwarded by the bristle input service.
///
/// `fd` is the read end of the port pair that bloom created and registered
/// with bristle via `msg_send(bristle_pid, RegisterSink, ...)`.
pub struct InputService {
    fd: u32,
    sink_write: u32,
    registered: bool,
    event_accum: [u8; 64],
    accum_len: usize,
    interests: Vec<Interest>,
}

impl InputService {
    /// Create an `InputService` that reads from `fd`.
    pub fn new(fd: u32, sink_write: u32) -> Self {
        let interests = vec![Interest::FdReadable(fd)];
        Self { fd, sink_write, registered: false, event_accum: [0; 64], accum_len: 0, interests }
    }
}

impl BloomService for InputService {
    fn name(&self) -> &'static str {
        "input"
    }

    fn interests(&self) -> &[Interest] {
        &self.interests
    }

    fn on_added(&mut self) -> LoopAction {
        LoopAction::ArmTimer {
            delay: core::time::Duration::from_millis(250),
            id: REGISTER_TIMER_ID,
        }
    }

    fn dispatch(&mut self, event: LoopEvent, world: &mut BloomWorld) -> LoopAction {
        match event {
            LoopEvent::Timer(REGISTER_TIMER_ID) => {
                if self.registered {
                    return LoopAction::None;
                }
                if self.try_register_with_bristle() {
                    self.registered = true;
                    stem::info!("bloom: registered bristle pointer sink");
                    LoopAction::None
                } else {
                    LoopAction::ArmTimer {
                        delay: core::time::Duration::from_millis(500),
                        id: REGISTER_TIMER_ID,
                    }
                }
            }
            LoopEvent::FdReady(_) => {
                let mut buf = [0u8; INPUT_READ_CHUNK];
                let mut handled = false;
                let read_wake = BLOOM_INPUT_READ_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
                let start_ns = stem::monotonic_ns();

                if should_log_input(read_wake) {
                    stem::info!(
                        "bloom: input fd wake entry wake={} accum_depth={} event_total={}",
                        read_wake,
                        self.accum_len,
                        BLOOM_INPUT_EVENT_COUNT.load(Ordering::Relaxed)
                    );
                }

                for _ in 0..MAX_READS_PER_WAKE {
                    match vfs_read(self.fd, &mut buf) {
                        Ok(0) => break,
                        Ok(n) => {
                            handled |= self.drain_bristle_bytes(&buf[..n], world);
                            if n < buf.len() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
                if should_log_input(read_wake) {
                    stem::info!(
                        "bloom: input fd wake exit wake={} handled={} accum_depth={} elapsed_ns={}",
                        read_wake,
                        handled,
                        self.accum_len,
                        stem::monotonic_ns().saturating_sub(start_ns)
                    );
                }

                if handled {
                    LoopAction::RequestImmediateRepaint
                } else if world.input.has_pending_cursor_motion() {
                    LoopAction::RequestRepaint
                } else {
                    LoopAction::None
                }
            }
            _ => LoopAction::None,
        }
    }
}

impl InputService {
    fn try_register_with_bristle(&self) -> bool {
        let Some(pid) = read_bristle_pid() else {
            return false;
        };
        let payload = encode_register_sink_with_mask(
            BRISTLE_SINK_TAG_BLOOM,
            self.sink_write,
            BRISTLE_EVENT_CLASS_ALL,
        );
        msg_send(pid, KindId(KIND_BRISTLE_REGISTER_SINK), &payload).is_ok()
    }

    fn drain_bristle_bytes(&mut self, input: &[u8], world: &mut BloomWorld) -> bool {
        let mut cursor = 0usize;
        let mut handled = false;

        while cursor < input.len() {
            let room = self.event_accum.len().saturating_sub(self.accum_len);
            if room == 0 {
                self.resync_accumulator();
                continue;
            }

            let to_copy = (input.len() - cursor).min(room);
            self.event_accum[self.accum_len..self.accum_len + to_copy]
                .copy_from_slice(&input[cursor..cursor + to_copy]);
            self.accum_len += to_copy;
            cursor += to_copy;

            while self.accum_len >= BristleEventHeader::SIZE {
                let mut header_bytes = [0u8; BristleEventHeader::SIZE];
                header_bytes.copy_from_slice(&self.event_accum[..BristleEventHeader::SIZE]);
                let Ok(header) = BristleEventHeader::from_bytes(&header_bytes) else {
                    self.resync_accumulator();
                    continue;
                };

                let event_type = header.event_type;
                let payload_len = header.payload_len;
                let total_len = BristleEventHeader::SIZE + payload_len as usize;
                if total_len > self.event_accum.len() {
                    self.resync_accumulator();
                    continue;
                }
                if self.accum_len < total_len {
                    break;
                }

                let event_no = BLOOM_INPUT_EVENT_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
                if should_log_input(event_no) {
                    stem::info!(
                        "bloom: input event entry event={} type={} payload_len={} accum_depth={}",
                        event_no,
                        event_type_name(event_type),
                        payload_len,
                        self.accum_len
                    );
                }
                handled |= world.handle_bristle_event(&self.event_accum[..total_len]);
                if should_log_input(event_no) {
                    stem::info!(
                        "bloom: input event exit event={} type={} handled={} accum_depth={}",
                        event_no,
                        event_type_name(event_type),
                        handled,
                        self.accum_len
                    );
                }

                self.accum_len -= total_len;
                if self.accum_len > 0 {
                    self.event_accum.copy_within(total_len..total_len + self.accum_len, 0);
                }
            }
        }

        handled
    }

    fn resync_accumulator(&mut self) {
        if self.accum_len > 0 {
            self.accum_len -= 1;
            if self.accum_len > 0 {
                self.event_accum.copy_within(1..1 + self.accum_len, 0);
            }
        }
    }
}

fn read_bristle_pid() -> Option<u32> {
    let fd = vfs_open(BRISTLE_PID_PATH, O_RDONLY).ok()?;
    let mut buf = [0u8; 16];
    let n = vfs_read(fd, &mut buf).ok()?;
    let _ = vfs_close(fd);
    let text = core::str::from_utf8(&buf[..n]).ok()?.trim();
    parse_u32(text.as_bytes())
}

fn parse_u32(bytes: &[u8]) -> Option<u32> {
    let mut value = 0u32;
    let mut saw_digit = false;
    for b in bytes {
        if !b.is_ascii_digit() {
            break;
        }
        saw_digit = true;
        value = value.checked_mul(10)?.checked_add((b - b'0') as u32)?;
    }
    if saw_digit { Some(value) } else { None }
}

fn should_log_input(count: u64) -> bool {
    count <= INPUT_TRACE_INITIAL || count % INPUT_TRACE_INTERVAL == 0
}

fn event_type_name(raw: u16) -> &'static str {
    match EventType::from_raw(raw) {
        Ok(EventType::KeyDown) => "KeyDown",
        Ok(EventType::KeyUp) => "KeyUp",
        Ok(EventType::PointerMove) => "PointerMove",
        Ok(EventType::PointerButtonDown) => "PointerButtonDown",
        Ok(EventType::PointerButtonUp) => "PointerButtonUp",
        Ok(EventType::Scroll) => "Scroll",
        _ => "Unknown",
    }
}
