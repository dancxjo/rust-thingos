//! `ThemeService` — watches the session theme selector and applies compositor
//! chrome themes without restarting Bloom.

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::time::Duration;

use abi::syscall::vfs_flags::{O_CREAT, O_RDONLY, O_RDWR, O_TRUNC};
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_stat, vfs_write};

use crate::loop_types::{BloomService, Interest, LoopAction, LoopEvent};
use crate::theme::{DEFAULT_THEME_NAME, theme_by_name};
use crate::world::BloomWorld;

pub const DEFAULT_THEME_CONFIG_PATH: &str = "/session/desktop/theme";
pub const WALLPAPER_CONFIG_PATH: &str = "/session/desktop/wallpaper";
const THEME_FALLBACK_POLL_TIMER: u64 = 11;
const THEME_FALLBACK_POLL_MS: u64 = 1000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ThemeStamp {
    size: u64,
    mtime_sec: u64,
    mtime_nsec: u32,
    ctime_sec: u64,
    ctime_nsec: u32,
}

pub fn read_theme_target(config_path: &str) -> Option<String> {
    let fd = vfs_open(config_path, O_RDONLY).ok()?;
    let mut buf = [0u8; 128];
    let n = vfs_read(fd, &mut buf).ok()?;
    let _ = vfs_close(fd);
    if n == 0 {
        return None;
    }

    let name = core::str::from_utf8(&buf[..n]).ok()?.trim();
    if name.is_empty() { None } else { Some(name.to_string()) }
}

pub fn theme_target_or_default(config_path: &str) -> String {
    read_theme_target(config_path).unwrap_or_else(|| DEFAULT_THEME_NAME.to_string())
}

pub fn ensure_theme_config(config_path: &str) -> String {
    if let Some(name) = read_theme_target(config_path) {
        return name;
    }

    if let Ok(fd) = vfs_open(config_path, O_CREAT | O_TRUNC | O_RDWR) {
        let _ = vfs_write(fd, DEFAULT_THEME_NAME.as_bytes());
        let _ = vfs_write(fd, b"\n");
        let _ = vfs_close(fd);
    }

    DEFAULT_THEME_NAME.to_string()
}

/// Write `wallpaper_path` to the wallpaper config file, overwriting any
/// previous value.  Called by `apply_theme` and bloom's startup to keep the
/// wallpaper config in sync with the active theme.
pub fn write_themed_wallpaper(config_path: &str, wallpaper_path: &str) {
    match vfs_open(config_path, O_CREAT | O_TRUNC | O_RDWR) {
        Ok(fd) => {
            let ok = vfs_write(fd, wallpaper_path.as_bytes()).is_ok()
                && vfs_write(fd, b"\n").is_ok();
            let _ = vfs_close(fd);
            if !ok {
                stem::warn!("Failed to write wallpaper config {}", config_path);
            }
        }
        Err(e) => {
            stem::warn!("Could not open wallpaper config {}: {:?}", config_path, e);
        }
    }
}

fn theme_config_stamp(config_path: &str) -> Option<ThemeStamp> {
    let fd = vfs_open(config_path, O_RDONLY).ok()?;
    let stat = vfs_stat(fd).ok()?;
    let _ = vfs_close(fd);
    Some(ThemeStamp {
        size: stat.size,
        mtime_sec: stat.mtime.sec,
        mtime_nsec: stat.mtime.nsec,
        ctime_sec: stat.ctime.sec,
        ctime_nsec: stat.ctime.nsec,
    })
}

fn write_wallpaper_target(config_path: &str, wallpaper_path: &str) -> bool {
    if let Ok(fd) = vfs_open(config_path, O_CREAT | O_TRUNC | O_RDWR) {
        let wrote_path = vfs_write(fd, wallpaper_path.as_bytes()).is_ok();
        let wrote_newline = vfs_write(fd, b"\n").is_ok();
        let _ = vfs_close(fd);
        wrote_path && wrote_newline
    } else {
        false
    }
}

pub struct ThemeService {
    fd: Option<u32>,
    config_path: &'static str,
    wallpaper_config_path: Option<&'static str>,
    last_stamp: Option<ThemeStamp>,
    interests: Vec<Interest>,
}

impl ThemeService {
    pub fn new(
        fd: Option<u32>,
        config_path: &'static str,
        wallpaper_config_path: Option<&'static str>,
    ) -> Self {
        let interests = if let Some(fd) = fd { vec![Interest::FdReadable(fd)] } else { Vec::new() };
        Self {
            fd,
            config_path,
            wallpaper_config_path,
            last_stamp: theme_config_stamp(config_path),
            interests,
        }
    }

    fn arm_fallback_poll_timer() -> LoopAction {
        LoopAction::ArmTimer {
            delay: Duration::from_millis(THEME_FALLBACK_POLL_MS),
            id: THEME_FALLBACK_POLL_TIMER,
        }
    }

    fn arm_change_detection(&self) -> LoopAction {
        Self::arm_fallback_poll_timer()
    }

    fn repaint_and_arm_change_detection(&self) -> LoopAction {
        LoopAction::RequestRepaintAndArmTimer {
            delay: Duration::from_millis(THEME_FALLBACK_POLL_MS),
            id: THEME_FALLBACK_POLL_TIMER,
        }
    }

    fn apply_theme(&mut self, world: &mut BloomWorld) {
        self.last_stamp = theme_config_stamp(self.config_path);
        let requested = theme_target_or_default(self.config_path);
        let theme = theme_by_name(&requested);
        let applied = world.visuals.set_theme_by_name(&requested);
        stem::info!("Applying theme {}", applied);
        if let (Some(config_path), Some(wallpaper_path)) =
            (self.wallpaper_config_path, theme.wallpaper_path)
        {
            let _ = write_wallpaper_target(config_path, wallpaper_path);
            stem::info!("Applying theme wallpaper {}", wallpaper_path);
            world.visuals.start_background_load(&world.display, wallpaper_path);
        }
        world.damage.mark_full(world.primary.width, world.primary.height);
        // Write the theme's wallpaper path so blossom picks it up via its watch.
        let theme = theme_by_name(&requested);
        write_themed_wallpaper(WALLPAPER_CONFIG_PATH, theme.wallpaper_path);
    }

    fn poll_config(&mut self, world: &mut BloomWorld) -> bool {
        let stamp = theme_config_stamp(self.config_path);
        if stamp.is_some() && stamp != self.last_stamp {
            self.apply_theme(world);
            true
        } else {
            false
        }
    }
}

impl BloomService for ThemeService {
    fn name(&self) -> &'static str {
        "theme"
    }

    fn interests(&self) -> &[Interest] {
        &self.interests
    }

    fn on_added(&mut self) -> LoopAction {
        self.arm_change_detection()
    }

    fn dispatch(&mut self, event: LoopEvent, world: &mut BloomWorld) -> LoopAction {
        match event {
            LoopEvent::FdReady(_) => {
                if let Some(fd) = self.fd {
                    let mut drain = [0u8; 1024];
                    let _ = vfs_read(fd, &mut drain);
                }
                self.apply_theme(world);
                LoopAction::RequestRepaint
            }
            LoopEvent::Timer(THEME_FALLBACK_POLL_TIMER) => {
                if self.poll_config(world) {
                    self.repaint_and_arm_change_detection()
                } else {
                    self.arm_change_detection()
                }
            }
            _ => self.arm_change_detection(),
        }
    }
}
