#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use stem::syscall::{argv_get, exit, vfs_attr_remove, vfs_close, vfs_open, vfs_write};

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

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    if args.len() != 2 {
        print("usage: attr_rm <path> <name>\n");
        exit(1)
    }
    let fd = match vfs_open(&args[0], 0) {
        Ok(v) => v,
        Err(e) => {
            print(&alloc::format!("attr_rm: open failed: {:?}\n", e));
            exit(1)
        }
    };
    match vfs_attr_remove(fd, &args[1]) {
        Ok(()) => {}
        Err(e) => {
            print(&alloc::format!("attr_rm: failed: {:?}\n", e));
            let _ = vfs_close(fd);
            exit(1)
        }
    }
    let _ = vfs_close(fd);
    exit(0)
}
