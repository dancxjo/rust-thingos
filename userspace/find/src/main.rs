#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use stem::syscall::{argv_get, exit, vfs_close, vfs_open, vfs_readdir, vfs_stat, vfs_write};

const S_IFDIR: u32 = 0o040000;
const S_IFMT: u32 = 0o170000;
const READDIR_BUF_SIZE: usize = 4096;

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
        .filter_map(|arg| core::str::from_utf8(arg).ok().map(String::from))
        .collect()
}

fn write_to_fd(fd: u32, msg: &str) {
    let _ = vfs_write(fd, msg.as_bytes());
}

fn path_join(parent: &str, name: &str) -> String {
    if parent == "/" {
        let mut path = String::from("/");
        path.push_str(name);
        return path;
    }

    let mut path = String::from(parent);
    if !path.ends_with('/') {
        path.push('/');
    }
    path.push_str(name);
    path
}

fn walk(path: &str, has_error: &mut bool) {
    let fd = match vfs_open(path, 0) {
        Ok(fd) => fd,
        Err(e) => {
            write_to_fd(2, &alloc::format!("find: cannot open '{}': {:?}\n", path, e));
            *has_error = true;
            return;
        }
    };

    let stat = match vfs_stat(fd) {
        Ok(stat) => stat,
        Err(e) => {
            write_to_fd(2, &alloc::format!("find: cannot stat '{}': {:?}\n", path, e));
            let _ = vfs_close(fd);
            *has_error = true;
            return;
        }
    };

    write_to_fd(1, &alloc::format!("{}\n", path));

    if (stat.mode & S_IFMT) != S_IFDIR {
        let _ = vfs_close(fd);
        return;
    }

    let mut entries = Vec::new();
    let mut buf = [0u8; READDIR_BUF_SIZE];
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
                            if name != "." && name != ".." {
                                entries.push(path_join(path, name));
                            }
                        }
                    }

                    offset = end + 1;
                }
            }
            Err(e) => {
                write_to_fd(2, &alloc::format!("find: cannot read directory '{}': {:?}\n", path, e));
                *has_error = true;
                break;
            }
        }
    }

    let _ = vfs_close(fd);
    entries.sort();
    for child in entries {
        walk(&child, has_error);
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let mut args = get_args();
    if args.is_empty() {
        args.push(String::from("."));
    }

    let mut had_error = false;
    for path in &args {
        walk(path, &mut had_error);
    }

    if had_error {
        exit(1)
    }
    exit(0)
}
