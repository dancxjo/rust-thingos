//! `WallpaperService` — reacts to VFS watch events on the wallpaper config
//! path and asks [`CompositorVisuals::start_background_load`] to reload the
//! compositor background.

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::time::Duration;

use abi::syscall::vfs_flags::{O_CREAT, O_RDONLY, O_RDWR, O_TRUNC};
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_stat, vfs_write};

use crate::loop_types::{BloomService, Interest, LoopAction, LoopEvent};
use crate::world::BloomWorld;

pub const DEFAULT_WALLPAPER_PATH: &str = "/share/wallpapers/flower.png";
const WALLPAPER_FALLBACK_POLL_TIMER: u64 = 1;
const WALLPAPER_INITIAL_LOAD_TIMER: u64 = 2;
const WALLPAPER_FALLBACK_POLL_MS: u64 = 1000;
const WALLPAPER_INITIAL_LOAD_MS: u64 = 25;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WallpaperStamp {
    size: u64,
    mtime_sec: u64,
    mtime_nsec: u32,
    ctime_sec: u64,
    ctime_nsec: u32,
}

pub fn read_wallpaper_target(config_path: &str) -> Option<String> {
    let fd = vfs_open(config_path, O_RDONLY).ok()?;
    let mut buf = [0u8; 256];
    let n = vfs_read(fd, &mut buf).ok()?;
    let _ = vfs_close(fd);
    if n == 0 {
        return None;
    }

    let path = core::str::from_utf8(&buf[..n]).ok()?.trim();
    if path.is_empty() { None } else { Some(path.to_string()) }
}

pub fn wallpaper_target_or_default(config_path: &str) -> String {
    read_wallpaper_target(config_path).unwrap_or_else(|| DEFAULT_WALLPAPER_PATH.to_string())
}

pub fn ensure_wallpaper_config(config_path: &str) -> String {
    if let Some(path) = read_wallpaper_target(config_path) {
        return path;
    }

    if let Ok(fd) = vfs_open(config_path, O_CREAT | O_TRUNC | O_RDWR) {
        let _ = vfs_write(fd, DEFAULT_WALLPAPER_PATH.as_bytes());
        let _ = vfs_write(fd, b"\n");
        let _ = vfs_close(fd);
    }

    DEFAULT_WALLPAPER_PATH.to_string()
}

fn wallpaper_config_stamp(config_path: &str) -> Option<WallpaperStamp> {
    let fd = vfs_open(config_path, O_RDONLY).ok()?;
    let stat = vfs_stat(fd).ok()?;
    let _ = vfs_close(fd);
    Some(WallpaperStamp {
        size: stat.size,
        mtime_sec: stat.mtime.sec,
        mtime_nsec: stat.mtime.nsec,
        ctime_sec: stat.ctime.sec,
        ctime_nsec: stat.ctime.nsec,
    })
}

/// Watches `/session/desktop/wallpaper` and triggers background reloads.
///
/// `fd` is the watch FD returned by `vfs_watch_path`.  When the watch fires,
/// the service drains the event bytes (to keep the FD clear) and delegates
/// the actual reload to [`CompositorVisuals::start_background_load`].  If the
/// watch could not be registered, the service falls back to a slow stat poll.
pub struct WallpaperService {
    fd: Option<u32>,
    config_path: &'static str,
    last_stamp: Option<WallpaperStamp>,
    initial_loaded: bool,
    interests: Vec<Interest>,
}

impl WallpaperService {
    /// Create a `WallpaperService` that reacts to watch events on `fd`.
    ///
    /// `config_path` is the session file containing the wallpaper path passed
    /// to [`CompositorVisuals::start_background_load`].
    pub fn new(fd: Option<u32>, config_path: &'static str) -> Self {
        let interests = if let Some(fd) = fd { vec![Interest::FdReadable(fd)] } else { Vec::new() };
        Self {
            fd,
            config_path,
            last_stamp: wallpaper_config_stamp(config_path),
            initial_loaded: false,
            interests,
        }
    }

    fn arm_initial_load_timer() -> LoopAction {
        LoopAction::ArmTimer {
            delay: Duration::from_millis(WALLPAPER_INITIAL_LOAD_MS),
            id: WALLPAPER_INITIAL_LOAD_TIMER,
        }
    }

    fn arm_fallback_poll_timer() -> LoopAction {
        LoopAction::ArmTimer {
            delay: Duration::from_millis(WALLPAPER_FALLBACK_POLL_MS),
            id: WALLPAPER_FALLBACK_POLL_TIMER,
        }
    }

    fn arm_change_detection(&self) -> LoopAction {
        if self.fd.is_some() { LoopAction::None } else { Self::arm_fallback_poll_timer() }
    }

    fn reload_background(&mut self, world: &mut BloomWorld) {
        self.initial_loaded = true;
        self.last_stamp = wallpaper_config_stamp(self.config_path);
        let wallpaper_path = wallpaper_target_or_default(self.config_path);
        stem::info!("bloom: reacting to wallpaper change: {}", wallpaper_path);
        world.visuals.start_background_load(&world.display, &wallpaper_path);
        world.damage.mark_full(world.primary.width, world.primary.height);
    }

    fn load_initial_background(&mut self, world: &mut BloomWorld) {
        if self.initial_loaded {
            return;
        }
        self.initial_loaded = true;
        self.last_stamp = wallpaper_config_stamp(self.config_path);
        let wallpaper_path = wallpaper_target_or_default(self.config_path);
        stem::info!("bloom: preparing background {}", wallpaper_path);
        world.visuals.start_background_load(&world.display, &wallpaper_path);
        world.damage.mark_full(world.primary.width, world.primary.height);
    }

    fn poll_config(&mut self, world: &mut BloomWorld) {
        let stamp = wallpaper_config_stamp(self.config_path);
        if stamp.is_some() && stamp != self.last_stamp {
            self.reload_background(world);
        }
    }
}

impl BloomService for WallpaperService {
    fn name(&self) -> &'static str {
        "wallpaper"
    }

    fn interests(&self) -> &[Interest] {
        &self.interests
    }

    fn on_added(&mut self) -> LoopAction {
        Self::arm_initial_load_timer()
    }

    fn dispatch(&mut self, event: LoopEvent, world: &mut BloomWorld) -> LoopAction {
        match event {
            LoopEvent::FdReady(_) => {
                // Drain the watch event bytes so the FD doesn't stay
                // permanently readable.
                if let Some(fd) = self.fd {
                    let mut drain = [0u8; 1024];
                    let _ = vfs_read(fd, &mut drain);
                }
                self.reload_background(world);
                LoopAction::RequestRepaint
            }
            LoopEvent::Timer(WALLPAPER_FALLBACK_POLL_TIMER) => {
                self.poll_config(world);
                self.arm_change_detection()
            }
            LoopEvent::Timer(WALLPAPER_INITIAL_LOAD_TIMER) => {
                self.load_initial_background(world);
                self.arm_change_detection()
            }
            _ => self.arm_change_detection(),
        }
    }
}
