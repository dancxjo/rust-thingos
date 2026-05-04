//! `InputService` — drains normalized HID events from the bristle event FD
//! and routes them through [`BloomWorld::handle_bristle_event`].

use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

use abi::KindId;
use abi::hid::{
    BRISTLE_EVENT_CLASS_ALL, BRISTLE_SINK_TAG_BLOOM, BristleEventHeader,
    KIND_BRISTLE_REGISTER_SINK, encode_register_sink_with_mask,
};
use abi::syscall::vfs_flags::O_RDONLY;
use abi::trace::input_source;
use stem::syscall::message::msg_send;
use stem::syscall::trace_mark_input;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};

use crate::loop_types::{BloomService, Interest, LoopAction, LoopEvent};
use crate::world::BloomWorld;

const REGISTER_TIMER_ID: u64 = 1;
const BRISTLE_PID_PATH: &str = "/run/bristle/pid";
const INPUT_READ_CHUNK: usize = 64;
const MAX_READS_PER_WAKE: usize = 4;
const INPUT_TRACE_INITIAL: u64 = 24;
const INPUT_TRACE_INTERVAL: u64 = 128;
const INPUT_SUMMARY_INTERVAL_NS: u64 = 1_000_000_000;
static BLOOM_INPUT_READ_COUNT: AtomicU64 = AtomicU64::new(0);
static BLOOM_INPUT_EVENT_COUNT: AtomicU64 = AtomicU64::new(0);
static BLOOM_INPUT_VFS_READ_COUNT: AtomicU64 = AtomicU64::new(0);

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
    rate_window_start_ns: u64,
    rate_window_wakes: u64,
    rate_window_reads: u64,
    rate_window_events: u64,
    rate_window_bytes: u64,
}

impl InputService {
    /// Create an `InputService` that reads from `fd`.
    pub fn new(fd: u32, sink_write: u32) -> Self {
        let interests = vec![Interest::FdReadable(fd)];
        Self {
            fd,
            sink_write,
            registered: false,
            event_accum: [0; 64],
            accum_len: 0,
            interests,
            rate_window_start_ns: 0,
            rate_window_wakes: 0,
            rate_window_reads: 0,
            rate_window_events: 0,
            rate_window_bytes: 0,
        }
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
                    stem::debug!("Registered bristle pointer sink.");
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
                self.rate_window_wakes = self.rate_window_wakes.wrapping_add(1);

                if should_log_input(read_wake) {
                    stem::trace!(
                        "bloom: input fd wake entry wake={} accum_depth={} event_total={}",
                        read_wake,
                        self.accum_len,
                        BLOOM_INPUT_EVENT_COUNT.load(Ordering::Relaxed)
                    );
                }

                for read_iter in 0..MAX_READS_PER_WAKE {
                    let read_no = BLOOM_INPUT_VFS_READ_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
                    let read_start_ns = stem::monotonic_ns();
                    if should_log_input(read_wake) || should_log_input(read_no) {
                        stem::trace!(
                            "bloom: input vfs_read entry wake={} read={} iter={} fd={} accum_depth={}",
                            read_wake,
                            read_no,
                            read_iter,
                            self.fd,
                            self.accum_len
                        );
                    }
                    let read_result = vfs_read(self.fd, &mut buf);
                    let read_elapsed_ns = stem::monotonic_ns().saturating_sub(read_start_ns);
                    self.rate_window_reads = self.rate_window_reads.wrapping_add(1);
                    if should_log_input(read_wake) || should_log_input(read_no) {
                        match &read_result {
                            Ok(n) => stem::trace!(
                                "bloom: input vfs_read exit wake={} read={} iter={} fd={} result=ok bytes={} elapsed_ns={}",
                                read_wake,
                                read_no,
                                read_iter,
                                self.fd,
                                *n,
                                read_elapsed_ns
                            ),
                            Err(e) => stem::trace!(
                                "bloom: input vfs_read exit wake={} read={} iter={} fd={} result=err err={:?} elapsed_ns={}",
                                read_wake,
                                read_no,
                                read_iter,
                                self.fd,
                                e,
                                read_elapsed_ns
                            ),
                        }
                    }
                    match read_result {
                        Ok(0) => break,
                        Ok(n) => {
                            self.rate_window_bytes = self.rate_window_bytes.wrapping_add(n as u64);
                            handled |= self.drain_bristle_bytes(&buf[..n], world);
                            if n < buf.len() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
                if should_log_input(read_wake) {
                    stem::trace!(
                        "bloom: input fd wake exit wake={} handled={} accum_depth={} elapsed_ns={}",
                        read_wake,
                        handled,
                        self.accum_len,
                        stem::monotonic_ns().saturating_sub(start_ns)
                    );
                }
                self.maybe_log_input_summary();

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
                let payload_len = header.payload_len as usize;
                let total_len = BristleEventHeader::SIZE.saturating_add(payload_len);

                if total_len > self.event_accum.len() || total_len < BristleEventHeader::SIZE {
                    self.resync_accumulator();
                    continue;
                }
                if self.accum_len < total_len {
                    break;
                }

                let event_no = BLOOM_INPUT_EVENT_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
                self.rate_window_events = self.rate_window_events.wrapping_add(1);
                if should_log_input(event_no) {
                    stem::trace!(
                        "bloom: input event entry event={} type={:x} payload_len={} accum_depth={}",
                        event_no,
                        event_type,
                        total_len - BristleEventHeader::SIZE,
                        self.accum_len
                    );
                }
                handled |= world.handle_bristle_event(&self.event_accum[..total_len]);
                if should_log_input(event_no) {
                    stem::trace!(
                        "bloom: input event exit event={} type={:x} handled={} accum_depth={}",
                        event_no,
                        event_type,
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

    fn maybe_log_input_summary(&mut self) {
        let now = stem::monotonic_ns();
        if self.rate_window_start_ns == 0 {
            self.rate_window_start_ns = now;
            return;
        }
        if now.saturating_sub(self.rate_window_start_ns) < INPUT_SUMMARY_INTERVAL_NS {
            return;
        }
        trace_mark_input(input_source::BLOOM, self.rate_window_bytes, self.rate_window_events, 0);
        stem::debug!(
            "Input summary: wakes={} reads={} events={} bytes={} accum_depth={}.",
            self.rate_window_wakes,
            self.rate_window_reads,
            self.rate_window_events,
            self.rate_window_bytes,
            self.accum_len
        );
        self.rate_window_start_ns = now;
        self.rate_window_wakes = 0;
        self.rate_window_reads = 0;
        self.rate_window_events = 0;
        self.rate_window_bytes = 0;
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
