#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use stem::abi::attrs::{AttrListEntryHeader, AttrType};
use stem::syscall::{argv_get, exit, vfs_attr_list, vfs_close, vfs_open, vfs_write};

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
    if args.len() != 1 {
        print("usage: attr_list <path>\n");
        exit(1)
    }
    let fd = match vfs_open(&args[0], 0) {
        Ok(v) => v,
        Err(e) => {
            print(&alloc::format!("attr_list: open failed: {:?}\n", e));
            exit(1)
        }
    };
    let mut buf = [0u8; 8192];
    let total_bytes = match vfs_attr_list(fd, &mut buf) {
        Ok(v) => v,
        Err(e) => {
            print(&alloc::format!("attr_list: failed: {:?}\n", e));
            let _ = vfs_close(fd);
            exit(1)
        }
    };
    let mut off = 0usize;
    let mut seen = 0usize;
    let header_len = core::mem::size_of::<AttrListEntryHeader>();
    while off < total_bytes && off + header_len <= buf.len() {
        let name_len = u16::from_le_bytes([buf[off], buf[off + 1]]) as usize;
        let raw_ty = buf[off + 2];
        let ty = AttrType::from_u8(raw_ty);
        let value_len =
            u32::from_le_bytes([buf[off + 4], buf[off + 5], buf[off + 6], buf[off + 7]]);
        off += header_len;
        if off + name_len > buf.len() {
            break;
        }
        if name_len == 0 {
            break;
        }
        let name = match core::str::from_utf8(&buf[off..off + name_len]) {
            Ok(v) => v,
            Err(_) => break,
        };
        off += name_len;
        if let Some(ty) = ty {
            print(&alloc::format!("{}\t{:?}\t{}\n", name, ty, value_len));
        } else {
            print(&alloc::format!("{}\tunknown({})\t{}\n", name, raw_ty, value_len));
        }
        seen += 1;
    }
    print(&alloc::format!("count={}\n", seen));
    let _ = vfs_close(fd);
    exit(0)
}
