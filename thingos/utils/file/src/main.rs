#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use abi::fs::FileStat;
use stem::syscall::{argv_get, exit, vfs_close, vfs_open, vfs_read, vfs_stat, vfs_write};

const S_IFMT: u32 = 0o170000;
const S_IFDIR: u32 = 0o040000;
const S_IFCHR: u32 = 0o020000;
const S_IFBLK: u32 = 0o060000;
const S_IFREG: u32 = 0o100000;
const S_IFIFO: u32 = 0o010000;
const S_IFLNK: u32 = 0o120000;
const S_IFSOCK: u32 = 0o140000;

fn print(msg: &str) {
    let _ = vfs_write(1, msg.as_bytes());
}

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

fn detect_type(path: &str) -> String {
    let fd = match vfs_open(path, 0) {
        Ok(fd) => fd,
        Err(_) => return "cannot open".to_string(),
    };

    let stat = match vfs_stat(fd as u32) {
        Ok(s) => s,
        Err(_) => {
            let _ = vfs_close(fd as u32);
            return "cannot stat".to_string();
        }
    };

    let base_type = match stat.mode & S_IFMT {
        S_IFDIR => "directory".to_string(),
        S_IFCHR => "character device".to_string(),
        S_IFBLK => "block device".to_string(),
        S_IFIFO => "fifo".to_string(),
        S_IFLNK => "symbolic link".to_string(),
        S_IFSOCK => "socket".to_string(),
        S_IFREG => detect_regular_file(fd as u32, &stat),
        _ => "unknown".to_string(),
    };

    let _ = vfs_close(fd as u32);
    base_type
}

fn detect_regular_file(fd: u32, stat: &FileStat) -> String {
    if stat.size == 0 {
        return "empty".to_string();
    }

    let mut buffer = [0u8; 512];
    let n = match vfs_read(fd, &mut buffer) {
        Ok(n) => n,
        Err(_) => return "regular file (read error)".to_string(),
    };

    if n >= 4 && &buffer[0..4] == b"\x7fELF" {
        return "ELF executable".to_string();
    }

    if n >= 2 && &buffer[0..2] == b"#!" {
        // Try to find the interpreter
        let line = buffer[0..n].split(|&b| b == b'\n').next().unwrap_or(&buffer[0..n]);
        let script_info = core::str::from_utf8(line).unwrap_or("script");
        return alloc::format!("{} script", script_info);
    }

    // Heuristic for text
    let mut is_text = true;
    for &b in &buffer[0..n] {
        if b < 7 || (b > 13 && b < 32) {
            is_text = false;
            break;
        }
    }

    if is_text {
        if core::str::from_utf8(&buffer[0..n]).is_ok() {
            return "text".to_string();
        } else {
            return "binary data".to_string();
        }
    }

    "data".to_string()
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    if args.is_empty() {
        print("usage: file <path>...\n");
        exit(1);
    }

    let mut had_error = false;
    for path in args {
        let file_type = detect_type(&path);
        print(&alloc::format!("{}: {}\n", path, file_type));
        if file_type.contains("cannot") {
            had_error = true;
        }
    }

    if had_error {
        exit(1);
    }
    exit(0);
}
