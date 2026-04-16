#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use stem::abi::syscall::vfs_flags;
use stem::syscall::{argv_get, exit, vfs_close, vfs_open, vfs_read, vfs_write};

fn get_args() -> Vec<String> {
    let mut len = 0;
    if let Ok(l) = argv_get(&mut []) {
        len = l;
    }
    if len == 0 {
        return Vec::new();
    }

    let mut buf = vec![0u8; len];
    if argv_get(&mut buf).is_err() {
        return Vec::new();
    }

    stem::utils::parse_argv(&buf)
        .into_iter()
        .skip(1)
        .filter_map(|b| core::str::from_utf8(b).ok().map(String::from))
        .collect()
}

fn print(fd: u32, msg: &str) {
    let _ = vfs_write(fd, msg.as_bytes());
}

fn write_all(fd: u32, mut data: &[u8]) -> Result<(), ()> {
    while !data.is_empty() {
        match vfs_write(fd, data) {
            Ok(0) => return Err(()),
            Ok(n) => data = &data[n..],
            Err(_) => return Err(()),
        }
    }
    Ok(())
}

fn open_output(path: &str, append: bool) -> Result<u32, ()> {
    let mut flags = vfs_flags::O_WRONLY | vfs_flags::O_CREAT;
    if append {
        flags |= vfs_flags::O_APPEND;
    } else {
        flags |= vfs_flags::O_TRUNC;
    }
    vfs_open(path, flags).map_err(|_| ())
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    let mut append = false;
    let mut file_args: Vec<&str> = Vec::new();

    for arg in &args {
        if arg == "-a" {
            append = true;
        } else if arg == "--help" {
            print(1, "usage: tee [-a] [FILE...]\n");
            exit(0);
        } else if arg.starts_with('-') {
            print(2, "tee: unsupported option\n");
            print(2, "usage: tee [-a] [FILE...]\n");
            exit(1);
        } else {
            file_args.push(arg);
        }
    }

    let mut out_fds: Vec<u32> = Vec::new();
    let mut had_error = false;

    for path in &file_args {
        match open_output(path, append) {
            Ok(fd) => out_fds.push(fd),
            Err(_) => {
                print(2, "tee: cannot open ");
                print(2, path);
                print(2, "\n");
                had_error = true;
            }
        }
    }

    let mut buf = vec![0u8; 4096];
    loop {
        let n = match vfs_read(0, &mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => {
                print(2, "tee: read error\n");
                had_error = true;
                break;
            }
        };

        if write_all(1, &buf[..n]).is_err() {
            print(2, "tee: write error on stdout\n");
            had_error = true;
            break;
        }

        for &fd in &out_fds {
            if write_all(fd, &buf[..n]).is_err() {
                print(2, "tee: write error\n");
                had_error = true;
            }
        }
    }

    for fd in out_fds {
        let _ = vfs_close(fd);
    }

    if had_error {
        exit(1)
    } else {
        exit(0)
    }
}
