#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use stem::abi::attrs::AttrType;
use stem::syscall::{argv_get, exit, vfs_attr_get, vfs_close, vfs_open, vfs_write};

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
        print("usage: attr_get <path> <name>\n");
        exit(1)
    }
    let fd = match vfs_open(&args[0], 0) {
        Ok(v) => v,
        Err(e) => {
            print(&alloc::format!("attr_get: open failed: {:?}\n", e));
            exit(1)
        }
    };
    let mut buf = [0u8; 4096];
    match vfs_attr_get(fd, &args[1], &mut buf) {
        Ok((ty, len)) => {
            let n = len.min(buf.len());
            print(&alloc::format!("type={:?} len={}\n", ty, len));
            match ty {
                AttrType::Utf8 => {
                    if let Ok(s) = core::str::from_utf8(&buf[..n]) {
                        print(&alloc::format!("value={}\n", s));
                    }
                }
                AttrType::Bool => {
                    print(&alloc::format!("value={}\n", if n > 0 && buf[0] != 0 { "true" } else { "false" }));
                }
                AttrType::U64 if n >= 8 => {
                    let v = u64::from_le_bytes(buf[..8].try_into().unwrap());
                    print(&alloc::format!("value={}\n", v));
                }
                AttrType::I64 if n >= 8 => {
                    let v = i64::from_le_bytes(buf[..8].try_into().unwrap());
                    print(&alloc::format!("value={}\n", v));
                }
                _ => {}
            }
        }
        Err(e) => {
            print(&alloc::format!("attr_get: failed: {:?}\n", e));
            let _ = vfs_close(fd);
            exit(1)
        }
    }
    let _ = vfs_close(fd);
    exit(0)
}
