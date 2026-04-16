#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use stem::syscall::{argv_get, exit, vfs_link, vfs_symlink, vfs_write};

#[derive(Default)]
struct Flags {
    symbolic: bool,
}

fn get_args() -> (Flags, Vec<String>) {
    let mut len = 0;
    if let Ok(l) = argv_get(&mut []) {
        len = l;
    }
    if len == 0 {
        return (Flags::default(), Vec::new());
    }

    let mut buf = alloc::vec![0u8; len];
    if argv_get(&mut buf).is_err() {
        return (Flags::default(), Vec::new());
    }

    let mut flags = Flags::default();
    let mut paths = Vec::new();

    for arg_bytes in stem::utils::parse_argv(&buf).into_iter().skip(1) {
        if let Ok(arg) = core::str::from_utf8(arg_bytes) {
            if arg == "-s" || arg == "--symbolic" {
                flags.symbolic = true;
            } else {
                paths.push(String::from(arg));
            }
        }
    }

    (flags, paths)
}

fn print(msg: &str) {
    let _ = vfs_write(1, msg.as_bytes());
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let (flags, args) = get_args();

    if args.len() != 2 {
        print("usage: ln [-s] <target> <link_name>\n");
        exit(1)
    }

    let target = &args[0];
    let link_name = &args[1];

    let result = if flags.symbolic {
        vfs_symlink(target, link_name)
    } else {
        vfs_link(target, link_name)
    };

    if let Err(e) = result {
        print(&alloc::format!(
            "ln: failed to create {}link '{}' -> '{}': {:?}\n",
            if flags.symbolic { "symbolic " } else { "" },
            link_name,
            target,
            e
        ));
        exit(1)
    }

    exit(0)
}
