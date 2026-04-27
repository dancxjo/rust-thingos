//! `InputService` — drains normalized HID events from the bristle event FD
//! and routes them through [`BloomWorld::handle_bristle_event`].

use alloc::vec;
use alloc::vec::Vec;

use abi::KindId;
use abi::hid::{BRISTLE_SINK_TAG_BLOOM, KIND_BRISTLE_REGISTER_SINK, encode_register_sink};
use abi::syscall::vfs_flags::O_RDONLY;
use stem::syscall::message::msg_send;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};

use crate::loop_types::{BloomService, Interest, LoopAction, LoopEvent};
use crate::world::BloomWorld;

const REGISTER_TIMER_ID: u64 = 1;
const BRISTLE_PID_PATH: &str = "/run/bristle/pid";

/// Handles normalized HID events forwarded by the bristle input service.
///
/// `fd` is the read end of the port pair that bloom created and registered
/// with bristle via `msg_send(bristle_pid, RegisterSink, ...)`.
pub struct InputService {
    fd: u32,
    sink_write: u32,
    registered: bool,
    interests: Vec<Interest>,
}

impl InputService {
    /// Create an `InputService` that reads from `fd`.
    pub fn new(fd: u32, sink_write: u32) -> Self {
        let interests = vec![Interest::FdReadable(fd)];
        Self { fd, sink_write, registered: false, interests }
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
        LoopAction::ArmTimer { delay: core::time::Duration::from_millis(250), id: REGISTER_TIMER_ID }
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
                let mut buf = [0u8; 512];
                match vfs_read(self.fd, &mut buf) {
                    Ok(n) if n > 0 => {
                        world.handle_bristle_event(&buf[..n]);
                        LoopAction::RequestRepaint
                    }
                    _ => LoopAction::None,
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
        let payload = encode_register_sink(BRISTLE_SINK_TAG_BLOOM, self.sink_write);
        msg_send(pid, KindId(KIND_BRISTLE_REGISTER_SINK), &payload).is_ok()
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
