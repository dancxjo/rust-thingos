#![no_std]
#![no_main]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use abi::errors::Errno;
use stem::syscall::vfs::vfs_bind;
use stem::syscall::{
    argv_get, exit, spawn_driver_ex, vfs_close, vfs_open, vfs_read, vfs_readdir, vfs_write,
};

const MOUNT_VERIFICATION_ATTEMPTS: usize = 50;
const MOUNT_VERIFICATION_DELAY_MS: u64 = 100;
const ROOT_SOURCE_POLL_DELAY_MS: u64 = 100;
const READ_FILE_CHUNK_SIZE: usize = 1024;
const READDIR_BUFFER_SIZE: usize = 4096;

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

    if args.len() == 1 && (args[0] == "--roots" || args[0] == "-R") {
        let status = mount_all_from_roots("/etc/roots");
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
                    fs_type,
                    device,
                    target,
                    e
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
    err("usage: mount -t <type> [device] <target>\n       mount -a\n       mount --roots\n");
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
    if mount_table_contains(target) {
        out(&alloc::format!("mounted type={} device={} target={}\n", fs_type, device, target));
        return Ok(());
    }

    let provider_path = resolve_provider_binary(fs_type).ok_or(Errno::ENOENT)?;
    let argv = [provider_path.as_bytes(), device.as_bytes(), target.as_bytes()];
    let _resp = spawn_driver_ex(&provider_path, &argv, &BTreeMap::new(), 0, &[], None)?;

    wait_for_mount(target, MOUNT_VERIFICATION_ATTEMPTS, MOUNT_VERIFICATION_DELAY_MS)?;
    out(&alloc::format!("mounted type={} device={} target={}\n", fs_type, device, target));
    Ok(())
}

#[derive(Default)]
struct RootSpec {
    fs_type: String,
    device: String,
    source: String,
    target: String,
    flags: u32,
    flags_text: String,
    wait_ms: u64,
}

fn mount_all_from_roots(dir: &str) -> i32 {
    let entries = match read_dir_entries(dir) {
        Some(entries) => entries,
        None => {
            err("mount: cannot read /etc/roots\n");
            return 1;
        }
    };

    let mut had_error = false;
    for name in entries {
        if name == "." || name == ".." || name.starts_with('.') {
            continue;
        }
        let path = alloc::format!("{}/{}", dir.trim_end_matches('/'), name);
        match read_root_spec(&path) {
            Some(spec) => {
                if let Err(e) = activate_root_spec(&spec) {
                    had_error = true;
                    err(&alloc::format!(
                        "mount: root overlay failed source={} target={}: {:?}\n",
                        spec.source,
                        spec.target,
                        e
                    ));
                }
            }
            None => {
                had_error = true;
                err(&alloc::format!("mount: root spec {} is invalid\n", path));
            }
        }
    }

    if had_error { 1 } else { 0 }
}

fn read_root_spec(path: &str) -> Option<RootSpec> {
    let data = read_file(path, 16 * 1024)?;
    let text = core::str::from_utf8(&data).ok()?;
    let mut spec = RootSpec { flags: abi::syscall::mount_flags::MREPL, ..RootSpec::default() };

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        match key {
            "type" | "fs_type" => spec.fs_type = value.to_string(),
            "device" => spec.device = value.to_string(),
            "source" => spec.source = value.to_string(),
            "target" => spec.target = value.to_string(),
            "flags" => {
                spec.flags_text = value.to_string();
                spec.flags = parse_mount_flags(value)?;
            }
            "wait_ms" => spec.wait_ms = value.parse::<u64>().ok()?,
            _ => {}
        }
    }

    if spec.source.is_empty() || spec.target.is_empty() {
        return None;
    }
    if spec.flags_text.is_empty() {
        spec.flags_text = "repl".to_string();
    }
    Some(spec)
}

fn parse_mount_flags(value: &str) -> Option<u32> {
    let mut flags = abi::syscall::mount_flags::MREPL;
    for token in value.split(',').map(str::trim).filter(|token| !token.is_empty()) {
        match token {
            "repl" | "replace" => {}
            "before" => flags |= abi::syscall::mount_flags::MBEFORE,
            "after" => flags |= abi::syscall::mount_flags::MAFTER,
            "create" => flags |= abi::syscall::mount_flags::MCREATE,
            "cor" => flags |= abi::syscall::mount_flags::MCOR,
            "cow" => flags |= abi::syscall::mount_flags::MCOW,
            _ => return None,
        }
    }
    Some(flags)
}

fn activate_root_spec(spec: &RootSpec) -> Result<(), Errno> {
    if !spec.fs_type.is_empty() && !mount_table_contains(&spec.source) {
        let device = if spec.device.is_empty() { "none" } else { spec.device.as_str() };
        match mount_one(&spec.fs_type, device, &spec.source) {
            Ok(()) | Err(Errno::ETIMEDOUT) => {}
            Err(e) => return Err(e),
        }
    }
    wait_for_root_source(&spec.source, spec.wait_ms)?;
    vfs_bind(&spec.source, &spec.target, spec.flags)?;
    out(&alloc::format!(
        "mount: root overlay mounted source={} target={} flags={}\n",
        spec.source,
        spec.target,
        spec.flags_text
    ));
    Ok(())
}

fn wait_for_root_source(source: &str, wait_ms: u64) -> Result<(), Errno> {
    let attempts = (wait_ms / ROOT_SOURCE_POLL_DELAY_MS).max(1);
    for _ in 0..attempts {
        if mount_table_contains(source) || dir_has_entries(source) {
            return Ok(());
        }
        stem::time::sleep_ms(ROOT_SOURCE_POLL_DELAY_MS);
    }
    Err(Errno::ETIMEDOUT)
}

fn wait_for_mount(target: &str, attempts: usize, delay_ms: u64) -> Result<(), Errno> {
    for _ in 0..attempts {
        if mount_table_contains(target) {
            return Ok(());
        }
        stem::time::sleep_ms(delay_ms);
    }
    Err(Errno::ETIMEDOUT)
}

fn mount_table_contains(target: &str) -> bool {
    let Some(data) = read_file("/proc/mounts", 64 * 1024) else {
        return false;
    };
    let Ok(text) = core::str::from_utf8(&data) else {
        return false;
    };
    let target = normalize_mount_path(target);
    text.lines()
        .filter_map(|line| line.split_whitespace().next())
        .any(|path| normalize_mount_path(path) == target)
}

fn dir_has_entries(path: &str) -> bool {
    read_dir_entries(path).is_some_and(|entries| {
        entries.into_iter().any(|entry| !entry.is_empty() && entry != "." && entry != "..")
    })
}

fn normalize_mount_path(path: &str) -> String {
    let trimmed = path.trim_end_matches('/');
    if trimmed.is_empty() { "/".to_string() } else { trimmed.to_string() }
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
    let service_direct = alloc::format!("/services/{}", fs_type);
    if path_exists(&service_direct) {
        return Some(service_direct);
    }
    let service_daemon = alloc::format!("/services/{}d", fs_type);
    if path_exists(&service_daemon) {
        return Some(service_daemon);
    }
    let app_direct = alloc::format!("/applications/{}", fs_type);
    if path_exists(&app_direct) {
        return Some(app_direct);
    }
    let app_daemon = alloc::format!("/applications/{}d", fs_type);
    if path_exists(&app_daemon) {
        return Some(app_daemon);
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

fn read_dir_entries(path: &str) -> Option<Vec<String>> {
    let fd = vfs_open(path, abi::syscall::vfs_flags::O_RDONLY).ok()?;
    let mut entries = Vec::new();
    let mut buf = [0u8; READDIR_BUFFER_SIZE];
    loop {
        match vfs_readdir(fd, &mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let mut offset = 0;
                while offset < n {
                    let mut end = offset;
                    while end < n && buf[end] != 0 {
                        end += 1;
                    }
                    if end > offset {
                        if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
                            entries.push(name.to_string());
                        }
                    }
                    offset = end + 1;
                }
            }
            Err(_) => {
                let _ = vfs_close(fd);
                return None;
            }
        }
    }
    let _ = vfs_close(fd);
    entries.sort();
    Some(entries)
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
