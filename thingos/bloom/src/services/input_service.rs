//! `InputService` — drains normalized HID events from the bristle event FD
//! and routes them through [`BloomWorld::handle_bristle_event`].

use alloc::vec;
use alloc::vec::Vec;

use stem::syscall::vfs::vfs_read;

use crate::loop_types::{BloomService, Interest, LoopAction, LoopEvent};
use crate::world::BloomWorld;

/// Handles normalized HID events forwarded by the bristle input service.
///
/// `fd` is the read end of the port pair that bloom created and registered
/// with bristle via `msg_send(bristle_pid, RegisterSink, ...)`.
pub struct InputService {
    fd: u32,
    interests: Vec<Interest>,
}

impl InputService {
    /// Create an `InputService` that reads from `fd`.
    pub fn new(fd: u32) -> Self {
        let interests = vec![Interest::FdReadable(fd)];
        Self { fd, interests }
    }
}

impl BloomService for InputService {
    fn name(&self) -> &'static str {
        "input"
    }

    fn interests(&self) -> &[Interest] {
        &self.interests
    }

    fn dispatch(&mut self, _event: LoopEvent, world: &mut BloomWorld) -> LoopAction {
        let mut buf = [0u8; 512];
        match vfs_read(self.fd, &mut buf) {
            Ok(n) if n > 0 => {
                world.handle_bristle_event(&buf[..n]);
                LoopAction::RequestRepaint
            }
            _ => LoopAction::None,
        }
    }
}
