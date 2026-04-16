#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use stem::abi::syscall::vfs_flags;
use stem::syscall::{argv_get, exit, vfs_close, vfs_open, vfs_read, vfs_write};

const BUF_SIZE: usize = 4096;

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

struct Output<'a> {
    fd: u32,
    path: &'a str,
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

    let mut outputs: Vec<Output<'_>> = Vec::new();
    let mut had_error = false;
    let mut stdout_ok = true;

    for path in &file_args {
        match open_output(path, append) {
            Ok(fd) => outputs.push(Output { fd, path }),
            Err(_) => {
                print(2, "tee: cannot open ");
                print(2, path);
                print(2, "\n");
                had_error = true;
            }
        }
    }

    let mut buf = vec![0u8; BUF_SIZE];
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

        if stdout_ok && write_all(1, &buf[..n]).is_err() {
            print(2, "tee: write error on stdout\n");
            had_error = true;
            stdout_ok = false;
        }

        let mut i = 0;
        while i < outputs.len() {
            if write_all(outputs[i].fd, &buf[..n]).is_err() {
                print(2, "tee: write error on ");
                print(2, outputs[i].path);
                print(2, "\n");
                had_error = true;
                let _ = vfs_close(outputs[i].fd);
                outputs.swap_remove(i);
                continue;
            }
            i += 1;
        }
    }

    for output in outputs {
        let _ = vfs_close(output.fd);
    }

    if had_error {
        exit(1)
    } else {
        exit(0)
    }
}
