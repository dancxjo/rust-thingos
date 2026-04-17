#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use stem::syscall::{argv_get, exit, vfs_write};

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

fn write_all(fd: u32, mut data: &[u8]) -> bool {
    while !data.is_empty() {
        match vfs_write(fd, data) {
            Ok(0) => return false,
            Ok(n) => data = &data[n.min(data.len())..],
            Err(_) => return false,
        }
    }
    true
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    let mut line = if args.is_empty() {
        String::from("y")
    } else {
        args.join(" ")
    };
    line.push('\n');
    let line_bytes = line.as_bytes();

    loop {
        if !write_all(1, line_bytes) {
            exit(1);
        }
    }
}
