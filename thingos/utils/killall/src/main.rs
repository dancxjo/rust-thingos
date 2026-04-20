//! `killall` — send signals to processes selected by wildcard pattern.
//!
//! Usage: killall [-<signal>] <pattern> [<pattern> ...]

#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::collections::BTreeSet;

use abi::signal::*;
use stem::syscall::{
    argv_get, exit, getpid, kill, vfs_close, vfs_open, vfs_read, vfs_readdir, vfs_write,
};

#[derive(Clone)]
struct ProcEntry {
    pid: i32,
    name: String,
    cmdline: String,
}

fn write_stderr(msg: &str) {
    let _ = unsafe { vfs_write(2, msg.as_bytes()) };
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
                let _ = vfs_close(fd);
                return None;
            }
        }
    }
    let _ = vfs_close(fd);
    Some(String::from_utf8_lossy(&out).into_owned())
}

fn parse_status_name(status: &str) -> String {
    for line in status.lines() {
        if let Some(v) = line.strip_prefix("Name:\t") {
            return v.trim().to_string();
        }
    }
    String::new()
}

fn normalize_cmdline(cmdline: &str) -> String {
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
    normalized
}

fn basename(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

fn wildcard_match(pattern: &str, text: &str) -> bool {
    let p = pattern.as_bytes();
    let t = text.as_bytes();
    let mut pattern_index = 0usize;
    let mut text_index = 0usize;
    let mut star_pattern_index: Option<usize> = None;
    let mut star_text_index = 0usize;

    while text_index < t.len() {
        if pattern_index < p.len()
            && (p[pattern_index] == t[text_index] || p[pattern_index] == b'?')
        {
            pattern_index += 1;
            text_index += 1;
        } else if pattern_index < p.len() && p[pattern_index] == b'*' {
            star_pattern_index = Some(pattern_index);
            pattern_index += 1;
            star_text_index = text_index;
        } else if let Some(star_index) = star_pattern_index {
            pattern_index = star_index + 1;
            star_text_index += 1;
            text_index = star_text_index;
        } else {
            return false;
        }
    }

    while pattern_index < p.len() && p[pattern_index] == b'*' {
        pattern_index += 1;
    }
    pattern_index == p.len()
}

fn pattern_matches_process(pattern: &str, proc: &ProcEntry) -> bool {
    if wildcard_match(pattern, &proc.name) || wildcard_match(pattern, &proc.cmdline) {
        return true;
    }

    for part in proc.cmdline.split_whitespace() {
        if wildcard_match(pattern, part) || wildcard_match(pattern, basename(part)) {
            return true;
        }
    }

    false
}

fn parse_signal(s: &str) -> Option<u8> {
    let s = s.strip_prefix("SIG").unwrap_or(s);
    if let Ok(n) = s.parse::<u8>() {
        return (n < NSIG).then_some(n);
    }
    match s {
        "HUP" => Some(SIGHUP),
        "INT" => Some(SIGINT),
        "QUIT" => Some(SIGQUIT),
        "ILL" => Some(SIGILL),
        "TRAP" => Some(SIGTRAP),
        "ABRT" | "IOT" => Some(SIGABRT),
        "BUS" => Some(SIGBUS),
        "FPE" => Some(SIGFPE),
        "KILL" => Some(SIGKILL),
        "USR1" => Some(SIGUSR1),
        "SEGV" => Some(SIGSEGV),
        "USR2" => Some(SIGUSR2),
        "PIPE" => Some(SIGPIPE),
        "ALRM" => Some(SIGALRM),
        "TERM" => Some(SIGTERM),
        "CHLD" | "CLD" => Some(SIGCHLD),
        "CONT" => Some(SIGCONT),
        "STOP" => Some(SIGSTOP),
        "TSTP" => Some(SIGTSTP),
        "TTIN" => Some(SIGTTIN),
        "TTOU" => Some(SIGTTOU),
        "URG" => Some(SIGURG),
        "XCPU" => Some(SIGXCPU),
        "XFSZ" => Some(SIGXFSZ),
        "VTALRM" => Some(SIGVTALRM),
        "PROF" => Some(SIGPROF),
        "WINCH" => Some(SIGWINCH),
        "IO" | "POLL" => Some(SIGIO),
        "PWR" => Some(SIGPWR),
        "SYS" => Some(SIGSYS),
        _ => None,
    }
}

fn collect_processes() -> Vec<ProcEntry> {
    let mut out = Vec::new();
    let fd = match vfs_open("/proc", 0) {
        Ok(fd) => fd,
        Err(_) => return out,
    };

    let mut buf = [0u8; 4096];
    loop {
        match vfs_readdir(fd, &mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let mut offset = 0usize;
                while offset < n {
                    let mut end = offset;
                    while end < n && buf[end] != 0 {
                        end += 1;
                    }

                    if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
                        if let Ok(pid) = name.parse::<i32>() {
                            let status_path = alloc::format!("/proc/{name}/status");
                            let cmdline_path = alloc::format!("/proc/{name}/cmdline");
                            let status = read_file(&status_path).unwrap_or_default();
                            let process_name = parse_status_name(&status);
                            let cmdline_raw = read_file(&cmdline_path).unwrap_or_default();
                            let cmdline = normalize_cmdline(&cmdline_raw);
                            out.push(ProcEntry { pid, name: process_name, cmdline });
                        }
                    }

                    offset = end + 1;
                }
            }
            Err(_) => break,
        }
    }

    let _ = vfs_close(fd);
    out
}

enum ParseArgsError {
    InvalidArgs,
    InvalidSignal(String),
}

fn parse_args() -> Result<(u8, Vec<String>), ParseArgsError> {
    let len = argv_get(&mut []).map_err(|_| ParseArgsError::InvalidArgs)?;
    let mut buf = alloc::vec![0u8; len];
    argv_get(&mut buf).map_err(|_| ParseArgsError::InvalidArgs)?;
    let raw_args = stem::utils::parse_argv(&buf);
    if raw_args.len() < 2 {
        return Err(ParseArgsError::InvalidArgs);
    }

    let mut args = Vec::new();
    for raw in raw_args.iter().skip(1) {
        let arg = core::str::from_utf8(raw).map_err(|_| ParseArgsError::InvalidArgs)?;
        args.push(arg.to_string());
    }

    let (sig, start) = if let Some(sig_str) = args[0].strip_prefix('-') {
        let sig = parse_signal(sig_str)
            .ok_or_else(|| ParseArgsError::InvalidSignal(sig_str.to_string()))?;
        (sig, 1usize)
    } else {
        (SIGTERM, 0usize)
    };

    if start >= args.len() {
        return Err(ParseArgsError::InvalidArgs);
    }

    Ok((sig, args[start..].to_vec()))
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let (sig, patterns) = match parse_args() {
        Ok(v) => v,
        Err(ParseArgsError::InvalidSignal(sig)) => {
            let msg = alloc::format!("killall: invalid signal: -{sig}\n");
            write_stderr(&msg);
            write_stderr("Usage: killall [-<signal>] <pattern> [<pattern> ...]\n");
            exit(1);
        }
        Err(ParseArgsError::InvalidArgs) => {
            write_stderr("killall: at least one pattern is required\n");
            write_stderr("Usage: killall [-<signal>] <pattern> [<pattern> ...]\n");
            exit(1);
        }
    };

    let self_pid = getpid() as i32;
    let procs = collect_processes();
    let mut matched_pids = BTreeSet::new();

    for pattern in &patterns {
        let mut matched_for_pattern = false;
        for proc in &procs {
            if proc.pid == self_pid {
                continue;
            }
            if pattern_matches_process(pattern, proc) {
                matched_for_pattern = true;
                matched_pids.insert(proc.pid);
            }
        }
        if !matched_for_pattern {
            let msg = alloc::format!("killall: no process matched pattern: {pattern}\n");
            write_stderr(&msg);
        }
    }

    if matched_pids.is_empty() {
        exit(1);
    }

    let mut exit_code = 0;
    for pid in matched_pids {
        if let Err(e) = kill(pid, sig) {
            let msg = alloc::format!("killall: {pid}: {e:?}\n");
            write_stderr(&msg);
            exit_code = 1;
        }
    }

    exit(exit_code);
}
