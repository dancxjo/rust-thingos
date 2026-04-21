#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use stem::abi::attrs::AttrType;
use stem::syscall::{argv_get, exit, vfs_attr_set, vfs_close, vfs_open, vfs_write};

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

fn parse_hex_bytes(s: &str) -> Option<Vec<u8>> {
    let raw = s.strip_prefix("0x").unwrap_or(s);
    if raw.len() % 2 != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(raw.len() / 2);
    let bytes = raw.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let hi = (bytes[i] as char).to_digit(16)? as u8;
        let lo = (bytes[i + 1] as char).to_digit(16)? as u8;
        out.push((hi << 4) | lo);
        i += 2;
    }
    Some(out)
}

fn parse_value(ty: &str, value: &str) -> Option<(AttrType, Vec<u8>)> {
    match ty {
        "bytes" => parse_hex_bytes(value).map(|v| (AttrType::Bytes, v)),
        "utf8" => Some((AttrType::Utf8, value.as_bytes().to_vec())),
        "bool" => {
            let v = matches!(value, "1" | "true" | "yes" | "on");
            Some((AttrType::Bool, alloc::vec![if v { 1 } else { 0 }]))
        }
        "i64" => value.parse::<i64>().ok().map(|v| (AttrType::I64, v.to_le_bytes().to_vec())),
        "u64" => value.parse::<u64>().ok().map(|v| (AttrType::U64, v.to_le_bytes().to_vec())),
        "f64" => value.parse::<f64>().ok().map(|v| (AttrType::F64, v.to_le_bytes().to_vec())),
        _ => None,
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    if args.len() != 4 {
        print("usage: attr_set <path> <name> <type: bytes|utf8|bool|i64|u64|f64> <value>\n");
        exit(1)
    }
    let (ty, value) = match parse_value(&args[2], &args[3]) {
        Some(v) => v,
        None => {
            print("attr_set: invalid type/value\n");
            exit(1)
        }
    };
    let fd = match vfs_open(&args[0], 0) {
        Ok(v) => v,
        Err(e) => {
            print(&alloc::format!("attr_set: open failed: {:?}\n", e));
            exit(1)
        }
    };
    match vfs_attr_set(fd, &args[1], ty, &value) {
        Ok(n) => print(&alloc::format!("attr_set: wrote {}\n", n)),
        Err(e) => {
            print(&alloc::format!("attr_set: failed: {:?}\n", e));
            let _ = vfs_close(fd);
            exit(1)
        }
    }
    let _ = vfs_close(fd);
    exit(0)
}
