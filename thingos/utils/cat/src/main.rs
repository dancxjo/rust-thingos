#![no_std]
#![no_main]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use stem::abi::syscall::vfs_flags;
use stem::syscall::{argv_get, vfs_close, vfs_open, vfs_read, vfs_write};

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

fn print_error(msg: &str) {
    let out = alloc::format!("cat: {}\n", msg);
    let _ = vfs_write(1, out.as_bytes());
}

/// Stream bytes from in_fd to out_fd.
/// If `number_lines` is true, prefixes each line with its 1-indexed line number.
fn stream(
    in_fd: u32,
    out_fd: u32,
    buf: &mut [u8],
    number_lines: bool,
    lineno: &mut usize,
    at_line_start: &mut bool,
) -> Result<(), ()> {
    loop {
        match vfs_read(in_fd, buf) {
            Ok(0) => break,
            Ok(n) => {
                if !number_lines {
                    let mut written = 0;
                    while written < n {
                        match vfs_write(out_fd, &buf[written..n]) {
                            Ok(0) => return Err(()),
                            Ok(nw) => written += nw,
                            Err(_) => return Err(()),
                        }
                    }
                } else {
                    // Line numbering mode: write each numbered line atomically to
                    // prevent kernel log messages from interleaving between the
                    // line-number prefix and the line content.
                    let chunk = &buf[..n];
                    let mut pos = 0;

                    while pos < chunk.len() {
                        if let Some(newline_offset) = chunk[pos..].iter().position(|&b| b == b'\n') {
                            let nl = pos + newline_offset;
                            if *at_line_start {
                                // Write prefix + line content as one atomic write.
                                *lineno += 1;
                                let mut out =
                                    alloc::format!("{:6}  ", *lineno).into_bytes();
                                out.extend_from_slice(&chunk[pos..=nl]);
                                let _ = vfs_write(out_fd, &out);
                            } else {
                                // Continuation of a line started in a previous chunk.
                                let _ = vfs_write(out_fd, &chunk[pos..=nl]);
                            }
                            *at_line_start = true;
                            pos = nl + 1;
                        } else {
                            // Remaining bytes form a partial line (no newline yet).
                            if *at_line_start {
                                *lineno += 1;
                                let mut out =
                                    alloc::format!("{:6}  ", *lineno).into_bytes();
                                out.extend_from_slice(&chunk[pos..]);
                                let _ = vfs_write(out_fd, &out);
                                *at_line_start = false;
                            } else {
                                let _ = vfs_write(out_fd, &chunk[pos..]);
                            }
                            break;
                        }
                    }
                }
            }
            Err(_) => return Err(()),
        }
    }
    Ok(())
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    let mut buf = alloc::vec![0u8; 32768]; // 32KB buffer

    let mut number_lines = false;
    let mut file_args = Vec::new();

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg.starts_with('-') && arg.len() > 1 && arg != "--" {
            for ch in arg.chars().skip(1) {
                match ch {
                    'n' => number_lines = true,
                    _ => {
                        print_error(&alloc::format!("invalid option -- '{}'", ch));
                        stem::syscall::exit(1);
                    }
                }
            }
        } else if arg == "--" {
            i += 1;
            while i < args.len() {
                file_args.push(&args[i]);
                i += 1;
            }
            break;
        } else {
            file_args.push(arg);
        }
        i += 1;
    }

    let mut lineno = 0;
    let mut at_line_start = true;

    if file_args.is_empty() {
        // No files specified, read from stdin (fd 0)
        if stream(0, 1, &mut buf, number_lines, &mut lineno, &mut at_line_start).is_err() {
            print_error("error reading from stdin");
        }
    } else {
        for path in file_args {
            if path == "-" {
                // Special case: read from stdin
                if stream(0, 1, &mut buf, number_lines, &mut lineno, &mut at_line_start).is_err() {
                    print_error("error reading from stdin");
                }
                continue;
            }

            stem::info!("cat: opening '{}'", path);
            match vfs_open(path, vfs_flags::O_RDONLY) {
                Ok(fd) => {
                    stem::info!("cat: opened '{}' fd={}", path, fd);
                    if stream(fd, 1, &mut buf, number_lines, &mut lineno, &mut at_line_start).is_err()
                    {
                        print_error(&alloc::format!("error reading {}", path));
                    }
                    let _ = vfs_close(fd);
                }
                Err(_) => {
                    print_error(&alloc::format!("failed to open {}", path));
                }
            }
        }
    }

    stem::syscall::exit(0)
}
