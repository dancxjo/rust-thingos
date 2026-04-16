#![no_std]
#![no_main]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use stem::abi::syscall::vfs_flags;
use stem::syscall::{argv_get, vfs_close, vfs_open, vfs_read, vfs_write};

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
    let mut args = Vec::new();
    if buf.len() >= 4 {
        let count = u32::from_le_bytes(buf[0..4].try_into().unwrap()) as usize;
        let mut offset = 4;
        for _ in 0..count {
            if offset + 4 > buf.len() {
                break;
            }
            let str_len =
                u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            if offset + str_len > buf.len() {
                break;
            }
            if let Ok(s) = core::str::from_utf8(&buf[offset..offset + str_len]) {
                args.push(String::from(s));
            }
            offset += str_len;
        }
    }
    args
}

fn write_str(fd: u32, s: &str) {
    let _ = vfs_write(fd, s.as_bytes());
}

fn read_all(fd: u32) -> Vec<u8> {
    let mut data = Vec::new();
    let mut buf = alloc::vec![0u8; 4096];
    loop {
        match vfs_read(fd, &mut buf) {
            Ok(0) => break,
            Ok(n) => data.extend_from_slice(&buf[..n]),
            Err(_) => break,
        }
    }
    data
}

fn collect_lines_from_bytes(data: &[u8], lines: &mut Vec<String>) {
    for line in data.split(|&b| b == b'\n') {
        let mut s = String::from_utf8_lossy(line).into_owned();
        if s.ends_with('\r') {
            s.pop();
        }
        lines.push(s);
    }
}

fn write_sorted_iter<'a, I: Iterator<Item = &'a String>>(iter: I, unique: bool) {
    let mut first = true;
    let mut previous: Option<&str> = None;
    for line in iter {
        if unique && previous == Some(line.as_str()) {
            continue;
        }
        if !first {
            let _ = vfs_write(1, b"\n");
        }
        write_str(1, line);
        first = false;
        previous = Some(line.as_str());
    }
    if !first {
        let _ = vfs_write(1, b"\n");
    }
}

fn sort_and_write_lines(lines: &mut Vec<String>, reverse: bool, unique: bool) {
    lines.sort();
    if reverse {
        write_sorted_iter(lines.iter().rev(), unique);
    } else {
        write_sorted_iter(lines.iter(), unique);
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();

    let mut reverse = false;
    let mut unique = false;
    let mut files: Vec<&str> = Vec::new();

    let mut arg_index = 1;
    while arg_index < args.len() {
        let arg = &args[arg_index];
        if arg.starts_with('-') && arg.len() > 1 && arg != "--" {
            for ch in arg[1..].chars() {
                match ch {
                    'r' => reverse = true,
                    'u' => unique = true,
                    _ => {
                        let msg = alloc::format!("sort: invalid option -- '{}'\n", ch);
                        write_str(2, &msg);
                        stem::syscall::exit(1);
                    }
                }
            }
        } else {
            files.push(arg.as_str());
        }
        arg_index += 1;
    }

    let mut lines = Vec::new();

    if files.is_empty() {
        let data = read_all(0);
        collect_lines_from_bytes(&data, &mut lines);
    } else {
        let mut had_error = false;
        for path in &files {
            if *path == "-" {
                let data = read_all(0);
                collect_lines_from_bytes(&data, &mut lines);
                continue;
            }

            match vfs_open(path, vfs_flags::O_RDONLY) {
                Ok(fd) => {
                    let data = read_all(fd);
                    let _ = vfs_close(fd);
                    collect_lines_from_bytes(&data, &mut lines);
                }
                Err(_) => {
                    let msg = alloc::format!("sort: {}: No such file or directory\n", path);
                    write_str(2, &msg);
                    had_error = true;
                }
            }
        }

        if had_error && lines.is_empty() {
            stem::syscall::exit(1);
        }
    }

    sort_and_write_lines(&mut lines, reverse, unique);
    stem::syscall::exit(0)
}
