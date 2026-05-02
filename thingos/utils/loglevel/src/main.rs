#![no_std]
#![no_main]
use alloc::string::String;
use core::fmt::{self, Write};
extern crate alloc;

use stem::syscall::{argv_get, log_set_level, log_set_serial_level, write as fd_write};

fn console(args: fmt::Arguments) {
    let mut text = String::new();
    let _ = text.write_fmt(args);
    let _ = text.write_str("\n");
    let _ = fd_write(1, text.as_bytes());
}

macro_rules! consoleln {
    ($($arg:tt)*) => {
        console(format_args!($($arg)*))
    };
}

fn get_args() -> alloc::vec::Vec<alloc::string::String> {
    let mut len = 0;
    if let Ok(l) = argv_get(&mut []) {
        len = l;
    }
    if len == 0 {
        return alloc::vec::Vec::new();
    }
    let mut buf = alloc::vec![0u8; len];
    if argv_get(&mut buf).is_err() {
        return alloc::vec::Vec::new();
    }

    let mut args = alloc::vec::Vec::new();
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
                args.push(alloc::string::String::from(s));
            }
            offset += str_len;
        }
    }
    args
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    if args.len() < 2 {
        consoleln!("Usage: loglevel [kernel|serial] <0-5>");
        consoleln!("  0: Off");
        consoleln!("  1: Error");
        consoleln!("  2: Warn");
        consoleln!("  3: Info (Default)");
        consoleln!("  4: Debug");
        consoleln!("  5: Trace");
        consoleln!("No target sets both kernel retention and serial mirroring.");
        stem::syscall::exit(0);
    }

    let (target, level_str) =
        if args.len() >= 3 { (Some(args[1].as_str()), &args[2]) } else { (None, &args[1]) };
    let level: u8 = match level_str.as_str() {
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        _ => {
            consoleln!("Invalid log level: {}", level_str);
            stem::syscall::exit(1);
        }
    };

    match target {
        Some("kernel") => match log_set_level(level) {
            Ok(_) => consoleln!("Kernel and serial log level set to {}", level),
            Err(e) => consoleln!("Failed to set kernel log level: {:?}", e),
        },
        Some("serial") => match log_set_serial_level(level) {
            Ok(_) => consoleln!("Serial log level set to {}", level),
            Err(e) => consoleln!("Failed to set serial log level: {:?}", e),
        },
        Some(other) => {
            consoleln!("Invalid log target: {}", other);
            stem::syscall::exit(1);
        }
        None => {
            let kernel_result = log_set_level(level);
            let serial_result = log_set_serial_level(level);
            match (kernel_result, serial_result) {
                (Ok(_), Ok(_)) => consoleln!("Kernel and serial log level set to {}", level),
                (Err(e), _) => consoleln!("Failed to set kernel log level: {:?}", e),
                (_, Err(e)) => consoleln!("Failed to set serial log level: {:?}", e),
            }
        }
    }

    stem::syscall::exit(0);
}
