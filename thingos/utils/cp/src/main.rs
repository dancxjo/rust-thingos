#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use stem::abi::syscall::vfs_flags;
use stem::syscall::{argv_get, exit, vfs_close, vfs_open, vfs_read, vfs_write};

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

fn copy_file(src: &str, dst: &str) -> Result<(), ()> {
    let in_fd = vfs_open(src, vfs_flags::O_RDONLY).map_err(|_| {
        print(&stem::tf!("cp.error.open", "cp: ne povas malfermi '{}'\n", src));
    })?;

    let out_fd = vfs_open(dst, vfs_flags::O_WRONLY | vfs_flags::O_CREAT | vfs_flags::O_TRUNC)
        .map_err(|_| {
            let _ = vfs_close(in_fd);
            print(&stem::tf!("cp.error.create", "cp: ne povas krei '{}'\n", dst));
        })?;

    let mut buf = alloc::vec![0u8; 32768];
    let result = 'copy: loop {
        match vfs_read(in_fd, &mut buf) {
            Ok(0) => break Ok(()),
            Ok(n) => {
                let mut written = 0;
                while written < n {
                    match vfs_write(out_fd, &buf[written..n]) {
                        Ok(0) => {
                            print(&stem::tf!("cp.error.write", "cp: skriberaro ĉe '{}'\n", dst));
                            break 'copy Err(());
                        }
                        Ok(nw) => written += nw,
                        Err(_) => {
                            print(&stem::tf!("cp.error.write", "cp: skriberaro ĉe '{}'\n", dst));
                            break 'copy Err(());
                        }
                    }
                }
            }
            Err(_) => {
                print(&stem::tf!("cp.error.read", "cp: legeraro ĉe '{}'\n", src));
                break Err(());
            }
        }
    };

    let _ = vfs_close(in_fd);
    let _ = vfs_close(out_fd);
    result
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    if args.len() < 2 {
        print(stem::tr!("cp.usage", "uzo: cp <fonto> <celo>\n"));
        exit(1)
    }
    let src = &args[0];
    let dst = &args[1];
    if copy_file(src, dst).is_err() {
        exit(1)
    }
    exit(0)
}
