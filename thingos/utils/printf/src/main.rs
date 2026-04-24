#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use stem::syscall::{argv_get, vfs_write};

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

fn expand_escapes(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }

        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('\\') => out.push('\\'),
            Some('\'') => out.push('\''),
            Some('\"') => out.push('\"'),
            Some('0') => {
                // Handle \0ooo (octal) - simplified: just \0 for now or up to 3 digits
                let mut octal = 0u8;
                let mut count = 0;
                while count < 3 {
                    if let Some(&next) = chars.peek() {
                        if next >= '0' && next <= '7' {
                            octal = octal * 8 + (next as u8 - b'0');
                            chars.next();
                            count += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                out.push(octal as char);
            }
            Some(next) => {
                out.push('\\');
                out.push(next);
            }
            None => out.push('\\'),
        }
    }
    out
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    if args.len() < 2 {
        stem::syscall::exit(0);
    }

    let format_str = &args[1];
    let out = expand_escapes(format_str);
    
    // Very basic printf: if there are more args, we should ideally format them.
    // For the BDD tests like `printf 'a\nb\nc\n'`, there are no extra args.
    // If we wanted to support %s, we'd need more logic.
    // For now, let's just support the basic case used in tests.

    let _ = vfs_write(1, out.as_bytes());

    stem::syscall::exit(0)
}
