#![no_std]
#![no_main]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use abi::signal::{wexitstatus, wifexited, wifsignaled, wtermsig};
use abi::types::stdio_mode;
use stem::abi::errors::Errno;
use stem::syscall::{argv_get, env_get, spawn_process_ex, vfs_open, vfs_read, vfs_write, waitpid};

const MAX_ARGV_BYTES: usize = 24 * 1024;
const ARG_ENTRY_OVERHEAD: usize = 1 + core::mem::size_of::<usize>();

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

fn write_err(msg: &str) {
    let _ = vfs_write(2, msg.as_bytes());
}

fn read_stdin() -> Vec<u8> {
    let mut out = Vec::new();
    let mut buf = alloc::vec![0u8; 4096];
    loop {
        match vfs_read(0, &mut buf) {
            Ok(0) => break,
            Ok(n) => out.extend_from_slice(&buf[..n]),
            Err(_) => break,
        }
    }
    out
}

fn split_whitespace_bytes(input: &[u8]) -> Vec<Vec<u8>> {
    let mut tokens = Vec::new();
    let mut index = 0usize;
    while index < input.len() {
        while index < input.len() && input[index].is_ascii_whitespace() {
            index += 1;
        }
        if index >= input.len() {
            break;
        }
        let start = index;
        while index < input.len() && !input[index].is_ascii_whitespace() {
            index += 1;
        }
        tokens.push(input[start..index].to_vec());
    }
    tokens
}

fn get_path_entries() -> Vec<String> {
    let needed = env_get(b"PATH", &mut []).unwrap_or(0);
    if needed == 0 {
        return alloc::vec![String::from("/bin"), String::from("/drivers")];
    }

    let mut buf = alloc::vec![0u8; needed];
    let n = env_get(b"PATH", &mut buf).unwrap_or(0).min(buf.len());
    if n == 0 {
        return alloc::vec![String::from("/bin"), String::from("/drivers")];
    }

    let path = match core::str::from_utf8(&buf[..n]) {
        Ok(s) => s,
        Err(_) => return alloc::vec![String::from("/bin"), String::from("/drivers")],
    };

    let mut entries = Vec::new();
    for part in path.split(':') {
        if !part.is_empty() {
            entries.push(String::from(part));
        }
    }
    if entries.is_empty() {
        entries.push(String::from("/bin"));
        entries.push(String::from("/drivers"));
    }
    entries
}

fn resolve_program(cmd: &str, path_entries: &[String]) -> Option<String> {
    if cmd.contains('/') {
        return Some(String::from(cmd));
    }

    for dir in path_entries {
        let mut candidate = dir.clone();
        if !candidate.ends_with('/') {
            candidate.push('/');
        }
        candidate.push_str(cmd);
        if vfs_open(&candidate, 0).is_ok() {
            return Some(candidate);
        }
    }
    None
}

fn wait_for_child(pid: u32) -> Result<i32, Errno> {
    loop {
        match waitpid(pid as i64, 0) {
            Ok((_, status)) => return Ok(status),
            Err(Errno::EINTR) => continue,
            Err(e) => return Err(e),
        }
    }
}

fn run_command(path: &str, argv: &[Vec<u8>]) -> i32 {
    let argv_refs: Vec<&[u8]> = argv.iter().map(|a| a.as_slice()).collect();
    match spawn_process_ex(
        path,
        &argv_refs,
        &BTreeMap::new(),
        stdio_mode::INHERIT,
        stdio_mode::INHERIT,
        stdio_mode::INHERIT,
        0,
        &[],
    ) {
        Ok(resp) => match wait_for_child(resp.child_pid) {
            Ok(status) => {
                if wifexited(status) {
                    wexitstatus(status) as i32
                } else if wifsignaled(status) {
                    128 + wtermsig(status) as i32
                } else {
                    1
                }
            }
            Err(_) => 1,
        },
        Err(_) => {
            write_err("xargs: failed to spawn command\n");
            127
        }
    }
}

fn build_base_argv(cmd: &str, fixed_args: &[String]) -> Vec<Vec<u8>> {
    let mut argv = Vec::new();
    argv.push(cmd.as_bytes().to_vec());
    for arg in fixed_args {
        argv.push(arg.as_bytes().to_vec());
    }
    argv
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    let (cmd, fixed_args): (&str, &[String]) =
        if args.is_empty() { ("echo", &[]) } else { (args[0].as_str(), &args[1..]) };

    let path_entries = get_path_entries();
    let path = match resolve_program(cmd, &path_entries) {
        Some(p) => p,
        None => {
            write_err("xargs: command not found\n");
            stem::syscall::exit(127);
        }
    };

    let stdin_bytes = read_stdin();
    let items = split_whitespace_bytes(&stdin_bytes);
    let base_argv = build_base_argv(cmd, fixed_args);
    let base_bytes: usize = base_argv.iter().map(|v| v.len() + ARG_ENTRY_OVERHEAD).sum();

    let mut exit_code = 0;
    let mut child_failed = false;
    if items.is_empty() {
        let status = run_command(&path, &base_argv);
        if status != 0 {
            child_failed = true;
        }
    } else {
        let mut idx = 0usize;
        while idx < items.len() {
            let mut argv = base_argv.clone();
            let mut current_bytes = base_bytes;
            let mut added = 0usize;

            while idx < items.len() {
                let item_bytes = items[idx].len() + ARG_ENTRY_OVERHEAD;
                if added > 0 && current_bytes + item_bytes > MAX_ARGV_BYTES {
                    break;
                }
                if added == 0 && current_bytes + item_bytes > MAX_ARGV_BYTES {
                    write_err("xargs: argument too long\n");
                    if exit_code == 0 {
                        exit_code = 124;
                    }
                    idx += 1;
                    break;
                }
                argv.push(items[idx].clone());
                current_bytes += item_bytes;
                idx += 1;
                added += 1;
            }

            if added == 0 {
                continue;
            }

            let status = run_command(&path, &argv);
            if status != 0 {
                child_failed = true;
            }
        }
    }

    if child_failed && exit_code == 0 {
        exit_code = 123;
    }

    stem::syscall::exit(exit_code)
}
