#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use stem::syscall::{argv_get, exit, vfs_write};

const SYSNAME: &str = "ThingOS";
const NODENAME: &str = "thingos";
const RELEASE: &str = "0.1.0";
const VERSION: &str = "dev";

#[cfg(target_arch = "x86_64")]
const MACHINE: &str = "x86_64";
#[cfg(target_arch = "aarch64")]
const MACHINE: &str = "aarch64";
#[cfg(target_arch = "riscv64")]
const MACHINE: &str = "riscv64";
#[cfg(target_arch = "loongarch64")]
const MACHINE: &str = "loongarch64";
#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "riscv64",
    target_arch = "loongarch64"
)))]
const MACHINE: &str = "unknown";

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

fn print_err(msg: &str) {
    let _ = vfs_write(2, msg.as_bytes());
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    let mut show_s = false;
    let mut show_n = false;
    let mut show_r = false;
    let mut show_v = false;
    let mut show_m = false;

    if args.is_empty() {
        show_s = true;
    } else {
        for arg in &args {
            match arg.as_str() {
                "-a" => {
                    show_s = true;
                    show_n = true;
                    show_r = true;
                    show_v = true;
                    show_m = true;
                }
                "-s" => show_s = true,
                "-n" => show_n = true,
                "-r" => show_r = true,
                "-v" => show_v = true,
                "-m" => show_m = true,
                _ => {
                    print_err("usage: uname [-a] [-s] [-n] [-r] [-v] [-m]\n");
                    exit(1)
                }
            }
        }
    }

    let mut fields: Vec<&str> = Vec::new();
    if show_s {
        fields.push(SYSNAME);
    }
    if show_n {
        fields.push(NODENAME);
    }
    if show_r {
        fields.push(RELEASE);
    }
    if show_v {
        fields.push(VERSION);
    }
    if show_m {
        fields.push(MACHINE);
    }

    for (i, field) in fields.iter().enumerate() {
        if i != 0 {
            print(" ");
        }
        print(field);
    }
    print("\n");

    exit(0)
}
