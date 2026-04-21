#![no_std]
#![no_main]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use abi::errors::Errno;
use stem::syscall::{argv_get, exit, spawn_driver_ex, vfs_close, vfs_open, vfs_read, vfs_write};

const MOUNT_VERIFICATION_ATTEMPTS: usize = 50;
const MOUNT_VERIFICATION_DELAY_MS: u64 = 100;
const READ_FILE_CHUNK_SIZE: usize = 1024;

fn get_args() -> Vec<String> {
    let len = match argv_get(&mut []) {
        Ok(l) if l > 0 => l,
        _ => return Vec::new(),
    };
    let mut buf = alloc::vec![0u8; len];
    if argv_get(&mut buf).is_err() {
        return Vec::new();
    }

    stem::utils::parse_argv(&buf)
        .into_iter()
        .skip(1)
        .filter_map(|b| core::str::from_utf8(b).ok().map(String::from))
        .collect()
}

fn out(msg: &str) {
    let _ = vfs_write(1, msg.as_bytes());
}

fn err(msg: &str) {
    let _ = vfs_write(2, msg.as_bytes());
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    if args.is_empty() {
        print_usage();
        exit(2);
    }

    if args.len() == 1 && args[0] == "-a" {
        let status = mount_all_from_fstab("/etc/fstab");
        exit(status);
    }

    if (args.len() != 3 && args.len() != 4) || args[0] != "-t" {
        print_usage();
        exit(2);
    }

    if args.len() == 4 {
        let fs_type = args[1].as_str();
        let device = args[2].as_str();
        let target = args[3].as_str();
        match mount_one(fs_type, device, target) {
            Ok(()) => {
                exit(0);
            }
            Err(e) => {
                err(&alloc::format!(
                    "mount: failed to mount {} (device {}) on {}: {:?}\n",
                    fs_type, device, target, e
                ));
                exit(1);
            }
        }
    } else {
        let fs_type = args[1].as_str();
        let target = args[2].as_str();
        match mount_one(fs_type, "none", target) {
            Ok(()) => {
                exit(0);
            }
            Err(e) => {
                err(&alloc::format!("mount: failed to mount {} on {}: {:?}\n", fs_type, target, e));
                exit(1);
            }
        }
    }
}

fn print_usage() {
    err("usage: mount -t <type> [device] <target>\n       mount -a\n");
}

fn mount_all_from_fstab(path: &str) -> i32 {
    let Some(data) = read_file(path, 64 * 1024) else {
        err("mount: cannot read /etc/fstab\n");
        return 1;
    };
    let Ok(text) = core::str::from_utf8(&data) else {
        err("mount: /etc/fstab is not valid UTF-8\n");
        return 1;
    };

    let mut had_error = false;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.split_whitespace().collect();
        let (device, target, fs_type) = if fields.len() >= 3 {
            (fields[0], fields[1], fields[2])
        } else if fields.len() >= 2 {
            ("none", fields[1], fields[0])
        } else {
            continue;
        };

        if let Err(e) = mount_one(fs_type, device, target) {
            had_error = true;
            err(&alloc::format!(
                "mount: fstab entry failed for type={} device={} target={}: {:?}\n",
                fs_type,
                device,
                target,
                e
            ));
        }
    }

    if had_error { 1 } else { 0 }
}

fn mount_one(fs_type: &str, device: &str, target: &str) -> Result<(), Errno> {
    let provider_path = resolve_provider_binary(fs_type).ok_or(Errno::ENOENT)?;
    let argv = [provider_path.as_bytes(), device.as_bytes(), target.as_bytes()];
    let _resp = spawn_driver_ex(&provider_path, &argv, &BTreeMap::new(), 0, &[], None)?;

    wait_for_mount(target, MOUNT_VERIFICATION_ATTEMPTS, MOUNT_VERIFICATION_DELAY_MS)?;
    out(&alloc::format!("mounted type={} device={} target={}\n", fs_type, device, target));
    Ok(())
}

fn wait_for_mount(target: &str, attempts: usize, delay_ms: u64) -> Result<(), Errno> {
    for _ in 0..attempts {
        if let Ok(fd) = vfs_open(target, abi::syscall::vfs_flags::O_RDONLY) {
            let _ = vfs_close(fd);
            return Ok(());
        }
        stem::time::sleep_ms(delay_ms);
    }
    Err(Errno::ETIMEDOUT)
}

fn resolve_provider_binary(fs_type: &str) -> Option<String> {
    if fs_type.starts_with('/') {
        if path_exists(fs_type) {
            return Some(fs_type.to_string());
        }
        return None;
    }

    let direct = alloc::format!("/bin/{}", fs_type);
    if path_exists(&direct) {
        return Some(direct);
    }
    let daemon = alloc::format!("/bin/{}d", fs_type);
    if path_exists(&daemon) {
        return Some(daemon);
    }
    let driver = alloc::format!("/drivers/{}", fs_type);
    if path_exists(&driver) {
        return Some(driver);
    }
    None
}

fn path_exists(path: &str) -> bool {
    match vfs_open(path, abi::syscall::vfs_flags::O_RDONLY) {
        Ok(fd) => {
            let _ = vfs_close(fd);
            true
        }
        Err(_) => false,
    }
}

fn read_file(path: &str, max_bytes: usize) -> Option<Vec<u8>> {
    let fd = vfs_open(path, abi::syscall::vfs_flags::O_RDONLY).ok()?;
    let mut out = Vec::new();
    let mut buf = [0u8; READ_FILE_CHUNK_SIZE];
    loop {
        let n = vfs_read(fd, &mut buf).ok()?;
        if n == 0 {
            break;
        }
        let next_len = match out.len().checked_add(n) {
            Some(v) => v,
            None => {
                let _ = vfs_close(fd);
                return None;
            }
        };
        if next_len > max_bytes {
            let _ = vfs_close(fd);
            return None;
        }
        out.extend_from_slice(&buf[..n]);
    }
    let _ = vfs_close(fd);
    Some(out)
}
