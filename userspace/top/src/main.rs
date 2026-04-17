#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use stem::syscall::{argv_get, exit, sleep_ns, vfs_close, vfs_open, vfs_read, vfs_readdir, vfs_write};

const NANOS_PER_SECOND: u64 = 1_000_000_000;
const MAX_DELAY_SECS: u64 = 86_400;

#[derive(Clone)]
struct ProcRow {
    pid: u32,
    ppid: String,
    state: String,
    cmd: String,
}

fn print(msg: &str) {
    let _ = vfs_write(1, msg.as_bytes()).ok();
}

fn print_err(msg: &str) {
    let _ = vfs_write(2, msg.as_bytes()).ok();
}

fn read_file(path: &str) -> Option<String> {
    let fd = vfs_open(path, 0).ok()?;
    let mut out = Vec::new();
    let mut buf = [0u8; 1024];
    loop {
        match vfs_read(fd, &mut buf) {
            Ok(0) => break,
            Ok(n) => out.extend_from_slice(&buf[..n]),
            Err(_) => {
                let _ = vfs_close(fd).ok();
                return None;
            }
        }
    }
    let _ = vfs_close(fd).ok();
    Some(String::from_utf8_lossy(&out).into_owned())
}

fn parse_status(status: &str) -> (String, String, String) {
    let mut pid = String::new();
    let mut ppid = String::new();
    let mut state = String::new();
    for line in status.lines() {
        if let Some(v) = line.strip_prefix("Pid:\t") {
            pid = v.trim().to_string();
        } else if let Some(v) = line.strip_prefix("PPid:\t") {
            ppid = v.trim().to_string();
        } else if let Some(v) = line.strip_prefix("State:\t") {
            state = v.trim().to_string();
        }
    }

    let state = if state.is_empty() { "?".to_string() } else { state };
    (pid, ppid, state)
}

fn collect_processes() -> Vec<ProcRow> {
    let mut rows = Vec::new();
    let fd = match vfs_open("/proc", 0) {
        Ok(fd) => fd,
        Err(_) => return rows,
    };

    let mut buf = [0u8; 4096];
    loop {
        match vfs_readdir(fd, &mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let mut offset = 0;
                while offset < n {
                    let mut end = offset;
                    while end < n && buf[end] != 0 {
                        end += 1;
                    }

                    if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
                        if !name.is_empty() && name.chars().all(|c| c.is_ascii_digit()) {
                            let status_path = alloc::format!("/proc/{name}/status");
                            if let Some(status) = read_file(&status_path) {
                                let (pid_str, ppid, state) = parse_status(&status);
                                if let Ok(pid) = pid_str.parse::<u32>() {
                                    let cmdline_path = alloc::format!("/proc/{name}/cmdline");
                                    let cmd = if let Some(cmdline) = read_file(&cmdline_path) {
                                        let mut normalized = String::new();
                                        for part in cmdline.split('\0') {
                                            let part = part.trim();
                                            if part.is_empty() {
                                                continue;
                                            }
                                            if !normalized.is_empty() {
                                                normalized.push(' ');
                                            }
                                            normalized.push_str(part);
                                        }
                                        if normalized.is_empty() { name.to_string() } else { normalized }
                                    } else {
                                        name.to_string()
                                    };
                                    rows.push(ProcRow { pid, ppid, state, cmd });
                                }
                            }
                        }
                    }
                    offset = end + 1;
                }
            }
            Err(_) => {
                let _ = vfs_close(fd).ok();
                return rows;
            }
        }
    }

    let _ = vfs_close(fd).ok();
    rows.sort_by_key(|r| r.pid);
    rows
}

fn read_uptime() -> String {
    if let Some(text) = read_file("/proc/uptime") {
        if let Some(uptime_value) = text.split_whitespace().next() {
            return uptime_value.to_string();
        }
    }
    "0".to_string()
}

fn read_meminfo() -> (String, String) {
    let mut total = "?".to_string();
    let mut free = "?".to_string();

    if let Some(text) = read_file("/proc/meminfo") {
        for line in text.lines() {
            if let Some(v) = line.strip_prefix("MemTotal:") {
                total = v.trim().to_string();
            } else if let Some(v) = line.strip_prefix("MemFree:") {
                free = v.trim().to_string();
            }
        }
    }

    (total, free)
}

fn parse_args() -> Result<(u64, u64), ()> {
    let mut len = 0;
    if let Ok(l) = argv_get(&mut []) {
        len = l;
    }
    if len == 0 {
        return Ok((0, NANOS_PER_SECOND));
    }

    let mut buf = alloc::vec![0u8; len];
    if argv_get(&mut buf).is_err() {
        return Err(());
    }

    let args = stem::utils::parse_argv(&buf);
    let mut iterations = 0u64;
    let mut delay_ns = NANOS_PER_SECOND;
    let mut i = 1usize;

    while i < args.len() {
        let arg = core::str::from_utf8(args[i]).map_err(|_| ())?;
        if arg == "-n" {
            i += 1;
            if i >= args.len() {
                return Err(());
            }
            let v = core::str::from_utf8(args[i]).map_err(|_| ())?;
            iterations = v.parse::<u64>().map_err(|_| ())?;
        } else if arg == "-d" {
            i += 1;
            if i >= args.len() {
                return Err(());
            }
            let v = core::str::from_utf8(args[i]).map_err(|_| ())?;
            let secs = v.parse::<u64>().map_err(|_| ())?;
            if secs > MAX_DELAY_SECS {
                return Err(());
            }
            delay_ns = secs.checked_mul(NANOS_PER_SECOND).ok_or(())?;
        } else if arg == "-h" || arg == "--help" {
            return Err(());
        } else {
            return Err(());
        }
        i += 1;
    }

    Ok((iterations, delay_ns))
}

#[stem::main]
fn main(_argc: usize) -> ! {
    let (iterations, delay_ns) = match parse_args() {
        Ok(v) => v,
        Err(_) => {
            print_err("usage: top [-n count] [-d seconds]\n");
            print_err("  -n count     number of refreshes (default: infinite)\n");
            print_err("  -d seconds   refresh interval in whole seconds (default: 1)\n");
            exit(1);
        }
    };

    let mut refresh_count = 0u64;
    loop {
        let rows = collect_processes();
        let uptime = read_uptime();
        let (mem_total, mem_free) = read_meminfo();

        print("\x1B[2J\x1B[H");
        print(&alloc::format!(
            "ThingOS top  uptime: {}s  tasks: {}  mem: total {} free {}\n",
            uptime,
            rows.len(),
            mem_total,
            mem_free
        ));
        print("  PID  PPID STAT COMMAND\n");

        for row in &rows {
            print(&alloc::format!(
                "{:>5} {:>5} {:<4} {}\n",
                row.pid, row.ppid, row.state, row.cmd
            ));
        }

        refresh_count = refresh_count.saturating_add(1);
        if iterations != 0 && refresh_count >= iterations {
            break;
        }
        sleep_ns(delay_ns);
    }

    exit(0)
}
