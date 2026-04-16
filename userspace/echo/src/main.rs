#![no_std]
#![no_main]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use stem::syscall::{argv_get, vfs_write};

fn get_args() -> Vec<String> {
    let mut len = 0;
    // First call to get the size
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
            let str_len = u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap()) as usize;
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

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    let mut interpret_escapes = false;
    let mut suppress_newline = false;
    let mut start = 1usize;

    while start < args.len() {
        let Some(flag) = args.get(start) else { break };
        if !flag.starts_with('-') || flag.len() <= 1 {
            break;
        }

        let mut valid = true;
        for option in flag.as_bytes().iter().skip(1).copied() {
            match option {
                b'e' => interpret_escapes = true,
                b'E' => interpret_escapes = false,
                b'n' => suppress_newline = true,
                _ => {
                    valid = false;
                    break;
                }
            }
        }
        if !valid {
            break;
        }
        start += 1;
    }

    let text = args[start..].join(" ");
    let mut out = if interpret_escapes { expand_escapes(&text) } else { text };
    if !suppress_newline {
        out.push('\n');
    }

    let _ = vfs_write(1, out.as_bytes());

    stem::syscall::exit(0)
}

fn expand_escapes(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }

        match chars.next() {
            Some('a') => out.push('\u{0007}'),
            Some('b') => out.push('\u{0008}'),
            Some('c') => break,
            Some('f') => out.push('\u{000C}'),
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('v') => out.push('\u{000B}'),
            Some('\\') => out.push('\\'),
            Some(next) => {
                out.push('\\');
                out.push(next);
            }
            None => out.push('\\'),
        }
    }

    out
}
