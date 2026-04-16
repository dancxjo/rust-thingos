#![no_std]
#![no_main]

extern crate alloc;

use abi::syscall::vfs_flags::{O_CREAT, O_RDONLY, O_RDWR, O_TRUNC};
use abi::syscall::{PollThing, poll_flags};
use stem::syscall::vfs::{
    vfs_close, vfs_ftruncate, vfs_mkdir, vfs_open, vfs_poll, vfs_read, vfs_readdir, vfs_seek,
    vfs_watch_path, vfs_write,
};
use stem::{info, warn};

/// Watch poll timeout in milliseconds.
const WATCH_POLL_TIMEOUT_MS: u64 = 1000;
/// Sleep interval in milliseconds when watch mode is unavailable.
const FALLBACK_SLEEP_MS: u64 = 250;
const READDIR_BUFFER_SIZE: usize = 4096;
const WATCH_BUFFER_SIZE: usize = 1024;

pub fn derive_policy(presence_count: usize) -> (&'static str, &'static str) {
    if presence_count > 0 {
        ("true", "inhabited")
    } else {
        ("false", "ambient")
    }
}

fn count_presences(dir_fd: u32) -> Result<usize, abi::errors::Errno> {
    let mut presence_count = 0;
    vfs_seek(dir_fd, 0, 0)?;
    let mut buf = [0u8; READDIR_BUFFER_SIZE];
    let n = vfs_readdir(dir_fd, &mut buf)?;
    let mut offset = 0;
    while offset < n {
        if buf[offset] == 0 {
            offset += 1;
            continue;
        }
        let mut end = offset;
        while end < n && buf[end] != 0 {
            end += 1;
        }
        if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
            if name != "." && name != ".." {
                presence_count += 1;
            }
        }
        offset = end + 1;
    }
    Ok(presence_count)
}

fn write_policy_value(fd: u32, value: &'static str) -> Result<(), abi::errors::Errno> {
    vfs_seek(fd, 0, 0)?;
    vfs_ftruncate(fd, 0)?;
    vfs_write(fd, value.as_bytes())?;
    Ok(())
}

#[stem::main]
fn main(_arg: usize) -> ! {
    stem::info!("placed: Place presentation policy daemon starting");

    // Ensure session roots and our policy leaves exist
    let _ = vfs_mkdir("/session");
    let _ = vfs_mkdir("/session/seat0");
    let _ = vfs_mkdir("/session/seat0/presences");

    // Default wallpaper config
    if let Ok(fd) = vfs_open("/session/seat0/wallpaper", O_CREAT | O_TRUNC | O_RDWR) {
        let _ = vfs_write(fd, b"/share/wallpapers/flower.bmp");
        let _ = vfs_close(fd);
    }

    let watch_mask = abi::vfs_watch::mask::CREATE
        | abi::vfs_watch::mask::REMOVE
        | abi::vfs_watch::mask::MODIFY
        | abi::vfs_watch::mask::MOVE;
    let mut watch_fd = match vfs_watch_path("/session/seat0/presences", watch_mask, 0) {
        Ok(fd) => Some(fd),
        Err(e) => {
            warn!("placed: failed to watch presences dir: {:?}", e);
            None
        }
    };
    if watch_fd.is_some() {
        info!("placed: using watch-driven presence updates");
    } else {
        info!("placed: using fallback periodic presence polling");
    }

    let mut presences_fd = vfs_open("/session/seat0/presences", O_RDONLY).ok();
    let mut mode_fd = vfs_open("/session/seat0/presentation_mode", O_CREAT | O_RDWR).ok();
    let mut occupied_fd = vfs_open("/session/seat0/occupied", O_CREAT | O_RDWR).ok();

    let mut last_mode: Option<&'static str> = None;
    let mut last_occupied: Option<&'static str> = None;

    loop {
        if let Some(fd) = watch_fd {
            let mut fds = [PollThing {
                thing: fd as i32,
                events: poll_flags::POLLIN as u16,
                revents: 0,
            }];
            match vfs_poll(&mut fds, WATCH_POLL_TIMEOUT_MS) {
                Ok(n) if n > 0 => {
                    // Drain pending watch payload so future poll calls can block again.
                    // We intentionally ignore the payload and do a full recount because
                    // policy depends only on aggregate presence count.
                    let mut watch_buf = [0u8; WATCH_BUFFER_SIZE];
                    if let Err(e) = vfs_read(fd, &mut watch_buf) {
                        warn!("placed: failed draining watch events: {:?}", e);
                        let _ = vfs_close(fd);
                        watch_fd = None;
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    warn!("placed: watch poll failed: {:?}, falling back to sleep", e);
                    let _ = vfs_close(fd);
                    watch_fd = None;
                    stem::time::sleep_ms(FALLBACK_SLEEP_MS);
                }
            }
        } else {
            stem::time::sleep_ms(FALLBACK_SLEEP_MS);
        }

        if presences_fd.is_none() {
            presences_fd = vfs_open("/session/seat0/presences", O_RDONLY).ok();
        }

        let presence_count = if let Some(fd) = presences_fd {
            match count_presences(fd) {
                Ok(count) => count,
                Err(e) => {
                    warn!("placed: failed reading presences: {:?}", e);
                    let _ = vfs_close(fd);
                    presences_fd = None;
                    0
                }
            }
        } else {
            0
        };

        let (occupied_str, mode_str) = derive_policy(presence_count);

        if last_mode != Some(mode_str) {
            if mode_fd.is_none() {
                mode_fd = vfs_open("/session/seat0/presentation_mode", O_CREAT | O_RDWR).ok();
            }
            if let Some(fd) = mode_fd {
                if write_policy_value(fd, mode_str).is_ok() {
                    last_mode = Some(mode_str);
                } else {
                    warn!("placed: failed writing presentation_mode");
                    let _ = vfs_close(fd);
                    mode_fd = None;
                }
            }
        }

        if last_occupied != Some(occupied_str) {
            if occupied_fd.is_none() {
                occupied_fd = vfs_open("/session/seat0/occupied", O_CREAT | O_RDWR).ok();
            }
            if let Some(fd) = occupied_fd {
                if write_policy_value(fd, occupied_str).is_ok() {
                    last_occupied = Some(occupied_str);
                } else {
                    warn!("placed: failed writing occupied");
                    let _ = vfs_close(fd);
                    occupied_fd = None;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_derivation() {
        assert_eq!(derive_policy(0), ("false", "ambient"));
        assert_eq!(derive_policy(1), ("true", "inhabited"));
        assert_eq!(derive_policy(5), ("true", "inhabited"));
    }
}
