#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use stem::syscall::{argv_get, env_get, exit, vfs_close, vfs_open, vfs_stat, vfs_write};

fn get_args() -> Vec<String> {
    let mut len = 0;
    if let Ok(l) = argv_get(&mut []) {
        len = l;
    }
    if len == 0 {
        return Vec::new();
    }

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

fn print(msg: &str) {
    let _ = vfs_write(1, msg.as_bytes());
}

fn print_err(msg: &str) {
    let _ = vfs_write(2, msg.as_bytes());
}

fn is_executable_file(path: &str) -> bool {
    let fd = match vfs_open(path, 0) {
        Ok(fd) => fd,
        Err(_) => return false,
    };
    let stat = match vfs_stat(fd) {
        Ok(s) => s,
        Err(_) => {
            let _ = vfs_close(fd);
            return false;
        }
    };
    let _ = vfs_close(fd);

    let kind = stat.mode & 0o170000;
    kind == 0o100000 && (stat.mode & 0o111) != 0
}

fn get_path_entries() -> Vec<String> {
    let needed = match env_get(b"PATH", &mut []) {
        Ok(n) => n,
        Err(_) => 0,
    };
    if needed == 0 {
        return vec![String::from("/bin")];
    }

    let mut buf = alloc::vec![0u8; needed];
    let n = match env_get(b"PATH", &mut buf) {
        Ok(n) => n.min(buf.len()),
        Err(_) => 0,
    };
    if n == 0 {
        return vec![String::from("/bin")];
    }

    let path = match core::str::from_utf8(&buf[..n]) {
        Ok(s) => s,
        Err(_) => return vec![String::from("/bin")],
    };

    let mut entries: Vec<String> = path
        .split(':')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    if entries.is_empty() {
        entries.push(String::from("/bin"));
    }
    entries
}

fn resolve_command(cmd: &str, path_entries: &[String]) -> Option<String> {
    if cmd.contains('/') {
        if is_executable_file(cmd) {
            return Some(cmd.to_string());
        }
        return None;
    }

    for dir in path_entries {
        let mut candidate = dir.clone();
        if !candidate.ends_with('/') {
            candidate.push('/');
        }
        candidate.push_str(cmd);
        if is_executable_file(&candidate) {
            return Some(candidate);
        }
    }
    None
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    if args.is_empty() {
        print_err("usage: which <command> [command...]\n");
        exit(2)
    }

    let path_entries = get_path_entries();
    let mut all_found = true;

    for cmd in &args {
        if let Some(found) = resolve_command(cmd, &path_entries) {
            print(&found);
            print("\n");
        } else {
            all_found = false;
            print_err("which: ");
            print_err(cmd);
            print_err(": not found\n");
        }
    }

    if all_found { exit(0) } else { exit(1) }
}
