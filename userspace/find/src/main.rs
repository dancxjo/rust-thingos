#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use stem::syscall::{argv_get, exit, vfs_close, vfs_open, vfs_readdir, vfs_write};
use stem::syscall::vfs::vfs_lstat;

const S_IFDIR: u32 = 0o040000;
const S_IFMT: u32 = 0o170000;
const READDIR_BUF_SIZE: usize = 4096;
const O_RDONLY: u32 = 0;

#[derive(Clone, Copy, PartialEq, Eq)]
enum TypeFilter {
    Any,
    Directory,
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

fn type_matches(filter: TypeFilter, mode: u32) -> bool {
    match filter {
        TypeFilter::Any => true,
        TypeFilter::Directory => (mode & S_IFMT) == S_IFDIR,
    }
}

fn walk(path: &str, filter: TypeFilter, has_error: &mut bool) {
    let stat = match vfs_lstat(path) {
        Ok(stat) => stat,
        Err(e) => {
            write_to_fd(2, &alloc::format!("find: cannot stat '{}': {:?}\n", path, e));
            *has_error = true;
            return;
        }
    };

    if type_matches(filter, stat.mode) {
        write_to_fd(1, &alloc::format!("{}\n", path));
    }

    if (stat.mode & S_IFMT) != S_IFDIR {
        return;
    }

    let fd = match vfs_open(path, O_RDONLY) {
        Ok(fd) => fd,
        Err(e) => {
            write_to_fd(
                2,
                &alloc::format!("find: cannot open directory '{}': {:?}\n", path, e),
            );
            *has_error = true;
            return;
        }
    };

    let mut entries = Vec::new();
    let mut buf = alloc::vec![0u8; READDIR_BUF_SIZE];
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
        walk(&child, filter, has_error);
    }
}

fn parse_args(args: &[String]) -> Result<(TypeFilter, Vec<String>), &'static str> {
    let mut filter = TypeFilter::Any;
    let mut paths = Vec::new();
    let mut i = 0usize;
    while i < args.len() {
        let arg = &args[i];
        if arg == "-t" || arg == "--type" {
            i += 1;
            if i >= args.len() {
                return Err("find: missing type after -t/--type\n");
            }
            filter = match args[i].as_str() {
                "d" => TypeFilter::Directory,
                _ => return Err("find: unsupported type (supported: d)\n"),
            };
        } else {
            paths.push(args[i].clone());
        }
        i += 1;
    }

    if paths.is_empty() {
        paths.push(String::from("."));
    }
    Ok((filter, paths))
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    let (filter, paths) = match parse_args(&args) {
        Ok(parsed) => parsed,
        Err(msg) => {
            write_to_fd(2, msg);
            write_to_fd(2, "usage: find [PATH...] [-t d]\n");
            exit(1);
        }
    };

    let mut has_error = false;
    for path in &paths {
        walk(path, filter, &mut has_error);
    }

    if has_error {
        exit(1)
    }
    exit(0)
}
