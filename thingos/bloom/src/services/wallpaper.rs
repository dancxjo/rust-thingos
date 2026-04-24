//! `WallpaperService` — reacts to VFS watch events on the wallpaper path and
//! kicks off an async background decode via
//! [`CompositorVisuals::start_background_load`].

use alloc::vec;
use alloc::vec::Vec;

use stem::syscall::vfs::vfs_read;

use crate::loop_types::{BloomService, Interest, LoopAction, LoopEvent};
use crate::world::BloomWorld;

/// Watches `/session/desktop/wallpaper` and triggers async background reloads.
///
/// `fd` is the watch FD returned by `vfs_watch_path`.  When the watch fires,
/// the service drains the event bytes (to keep the FD clear) and delegates
/// the actual decode to [`CompositorVisuals::start_background_load`], which
/// spawns a worker thread so the compositor loop is never blocked by image
/// decoding.  The completed texture is picked up in the main loop by
/// `poll_ready_background`.
pub struct WallpaperService {
    fd: u32,
    wp_path: &'static str,
    interests: Vec<Interest>,
}

impl WallpaperService {
    /// Create a `WallpaperService` that reacts to watch events on `fd`.
    ///
    /// `wp_path` is the wallpaper path passed to
    /// [`CompositorVisuals::start_background_load`].
    pub fn new(fd: u32, wp_path: &'static str) -> Self {
        let interests = vec![Interest::FdReadable(fd)];
        Self { fd, wp_path, interests }
    }
}

impl BloomService for WallpaperService {
    fn name(&self) -> &'static str {
        "wallpaper"
    }

    fn interests(&self) -> &[Interest] {
        &self.interests
    }

    fn dispatch(&mut self, _event: LoopEvent, world: &mut BloomWorld) -> LoopAction {
        // Drain the watch event bytes so the FD doesn't stay permanently
        // readable.
        let mut drain = [0u8; 1024];
        let _ = vfs_read(self.fd, &mut drain);

        stem::info!("bloom: reacting to wallpaper change (async)");
        world.visuals.start_background_load(&world.display, self.wp_path);

        // The decode is async; the main loop will call poll_ready_background
        // and schedule a repaint when the texture is ready.
        LoopAction::None
    }
}
