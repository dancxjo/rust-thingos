//! `WaylandService` — reads native Bloom protocol messages from the service
//! port FD and dispatches them to [`BloomWorld`].

use alloc::vec;
use alloc::vec::Vec;

use stem::syscall::vfs::vfs_read;

use crate::loop_types::{BloomService, Interest, LoopAction, LoopEvent};
use crate::world::BloomWorld;

/// Handles incoming Bloom protocol requests from compositor clients.
///
/// The service FD is the read end of the port pair published at
/// `/run/services/bloom`, bridged to a VFS file descriptor via
/// `vfs_handle_from_port`.  On each dispatch call the service reads one
/// message chunk and forwards it to [`BloomWorld::handle_wayland_message`].
pub struct WaylandService {
    fd: u32,
    interests: Vec<Interest>,
}

impl WaylandService {
    /// Create a `WaylandService` that reads from `fd`.
    pub fn new(fd: u32) -> Self {
        let interests = vec![Interest::FdReadable(fd)];
        Self { fd, interests }
    }
}

impl BloomService for WaylandService {
    fn name(&self) -> &'static str {
        "wayland"
    }

    fn interests(&self) -> &[Interest] {
        &self.interests
    }

    fn dispatch(&mut self, _event: LoopEvent, world: &mut BloomWorld) -> LoopAction {
        let mut buf = [0u8; 512];
        match vfs_read(self.fd, &mut buf) {
            Ok(n) if n > 0 => {
                let changed = world.handle_wayland_message(&buf[..n]);
                if changed { LoopAction::RequestRepaint } else { LoopAction::None }
            }
            _ => LoopAction::None,
        }
    }
}
