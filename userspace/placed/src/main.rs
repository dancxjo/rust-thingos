#![no_std]
#![no_main]

extern crate alloc;

use abi::syscall::vfs_flags::{O_CREAT, O_RDONLY, O_RDWR, O_TRUNC};
use stem::syscall::vfs::{vfs_close, vfs_mkdir, vfs_open, vfs_readdir, vfs_write};
use stem::{info, warn};

pub fn derive_policy(presence_count: usize) -> (&'static str, &'static str) {
    if presence_count > 0 {
        ("true", "inhabited")
    } else {
        ("false", "ambient")
    }
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

    loop {
        let mut presence_count = 0;
        
        // Poll presences/
        if let Ok(dir_fd) = vfs_open("/session/seat0/presences", O_RDONLY) {
            let mut buf = [0u8; 4096];
            if let Ok(n) = vfs_readdir(dir_fd, &mut buf) {
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
            }
            let _ = vfs_close(dir_fd);
        }

        let (occ_str, mode_str) = derive_policy(presence_count);

        if let Ok(fd) = vfs_open("/session/seat0/presentation_mode", O_CREAT | O_TRUNC | O_RDWR) {
            let _ = vfs_write(fd, mode_str.as_bytes());
            let _ = vfs_close(fd);
        }

        if let Ok(fd) = vfs_open("/session/seat0/occupied", O_CREAT | O_TRUNC | O_RDWR) {
            let _ = vfs_write(fd, occ_str.as_bytes());
            let _ = vfs_close(fd);
        }

        stem::time::sleep_ms(500); // Wait between polls
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
