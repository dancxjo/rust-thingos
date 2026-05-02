#![no_std]
#![no_main]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use abi::errors::Errno;
use abi::signal::{
    SIG_IGN, SIGCONT, SIGINT, SIGTSTP, SIGTTIN, SIGTTOU, SigAction, SigSet, wexitstatus,
    wifcontinued, wifexited, wifsignaled, wifstopped, wstopsig, wtermsig,
};
use abi::syscall::{fcntl_cmd, handle_flags, vfs_flags};
use abi::termios;
use abi::types::{stdio_mode, waitpid_flags};
use stem::syscall;
use stem::syscall::{signal, vfs};

const TTY_FD: u32 = 0;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ProcessState {
    Running,
    Stopped(u8),
    Exited(u8),
    Signaled(u8),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum JobState {
    Running,
    Stopped,
    Completed,
}

struct ProcessEntry {
    pid: u32,
    state: ProcessState,
}

struct Job {
    id: usize,
    pgid: u32,
    command: String,
    background: bool,
    processes: Vec<ProcessEntry>,
    state: JobState,
    last_status: Option<i32>,
    notify: bool,
}

impl Job {
    fn new(id: usize, pgid: u32, command: String, background: bool, pids: Vec<u32>) -> Self {
        let processes = pids
            .into_iter()
            .map(|pid| ProcessEntry { pid, state: ProcessState::Running })
            .collect();
        Self {
            id,
            pgid,
            command,
            background,
            processes,
            state: JobState::Running,
            last_status: None,
            notify: false,
        }
    }

    fn recompute_state(&self) -> JobState {
        let mut any_stopped = false;
        let mut any_running = false;
        let mut all_completed = true;

        for process in &self.processes {
            match process.state {
                ProcessState::Running => {
                    any_running = true;
                    all_completed = false;
                }
                ProcessState::Stopped(_) => {
                    any_stopped = true;
                    all_completed = false;
                }
                ProcessState::Exited(_) | ProcessState::Signaled(_) => {}
            }
        }

        if all_completed {
            JobState::Completed
        } else if any_running {
            JobState::Running
        } else if any_stopped {
            JobState::Stopped
        } else {
            JobState::Running
        }
    }

    fn update_process(&mut self, pid: u32, status: i32) -> bool {
        let old_state = self.state;
        for process in &mut self.processes {
            if process.pid != pid {
                continue;
            }
            process.state = if wifexited(status) {
                ProcessState::Exited(wexitstatus(status))
            } else if wifsignaled(status) {
                ProcessState::Signaled(wtermsig(status))
            } else if wifstopped(status) {
                ProcessState::Stopped(wstopsig(status))
            } else if wifcontinued(status) {
                ProcessState::Running
            } else {
                process.state
            };
            self.last_status = Some(status);
            break;
        }

        self.state = self.recompute_state();
        old_state != self.state || wifcontinued(status)
    }

    fn mark_running(&mut self) {
        for process in &mut self.processes {
            if matches!(process.state, ProcessState::Stopped(_)) {
                process.state = ProcessState::Running;
            }
        }
        self.state = JobState::Running;
        self.notify = false;
    }
}

struct Cmd<'a> {
    program: &'a str,
    args: Vec<&'a str>,
    stdin_file: Option<&'a str>,
    stdout_file: Option<&'a str>,
    stdout_append: bool,
}

impl<'a> Cmd<'a> {
    fn parse(tokens: &[&'a str]) -> Option<Self> {
        if tokens.is_empty() {
            return None;
        }

        let mut program = None;
        let mut args = Vec::new();
        let mut stdin_file = None;
        let mut stdout_file = None;
        let mut stdout_append = false;
        let mut idx = 0;

        while idx < tokens.len() {
            match tokens[idx] {
                "<" => {
                    idx += 1;
                    if idx < tokens.len() {
                        stdin_file = Some(tokens[idx]);
                    }
                }
                ">" => {
                    idx += 1;
                    if idx < tokens.len() {
                        stdout_file = Some(tokens[idx]);
                        stdout_append = false;
                    }
                }
                ">>" => {
                    idx += 1;
                    if idx < tokens.len() {
                        stdout_file = Some(tokens[idx]);
                        stdout_append = true;
                    }
                }
                token => {
                    if program.is_none() {
                        program = Some(token);
                    } else {
                        args.push(token);
                    }
                }
            }
            idx += 1;
        }

        program.map(|program| Self { program, args, stdin_file, stdout_file, stdout_append })
    }
}

enum ReadLineResult {
    Line(String),
    Interrupted,
    Eof,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReapResult {
    StatusUpdated,
    NoChild,
    NoStatus,
}

struct Shell {
    shell_pgid: u32,
    jobs: Vec<Job>,
    next_job_id: usize,
    last_foreground_status: Option<i32>,
    aliases: BTreeMap<String, String>,
    env: BTreeMap<String, String>,
    history: Vec<String>,
}

impl Shell {
    fn new() -> Self {
        let shell_pid = syscall::getpid();
        // Avoid hard dependency on early job-control setup during bootstrap.
        // The shell remains usable for BDD command execution even if session
        // leadership is not established immediately.
        let shell_pgid = signal::getpgrp().unwrap_or(shell_pid as i32) as u32;
        Self {
            shell_pgid,
            jobs: Vec::new(),
            next_job_id: 1,
            last_foreground_status: None,
            aliases: BTreeMap::new(),
            env: BTreeMap::new(),
            history: Vec::new(),
        }
    }

    fn load_profile(&mut self, path: &str) {
        let fd = match vfs::vfs_open(path, vfs_flags::O_RDONLY) {
            Ok(fd) => fd,
            Err(_) => return,
        };
        let mut buf = [0u8; 4096];
        if let Ok(n) = syscall::vfs_read(fd, &mut buf) {
            if let Ok(content) = core::str::from_utf8(&buf[..n]) {
                for line in content.lines() {
                    let line = line.trim();
                    if line.starts_with("alias ") {
                        let rest = &line[6..];
                        if let Some((name, value)) = rest.split_once('=') {
                            let name = name.trim();
                            let value = value.trim().trim_matches('\'').trim_matches('"');
                            self.aliases.insert(String::from(name), String::from(value));
                        }
                    } else if line.starts_with("export ") {
                        let rest = &line[7..];
                        if let Some((name, value)) = rest.split_once('=') {
                            let name = name.trim();
                            let value = value.trim().trim_matches('\'').trim_matches('"');
                            self.env.insert(String::from(name), String::from(value));
                        }
                    }
                }
            }
        }
        let _ = syscall::vfs_close(fd);
    }

    fn expand_aliases(&self, line: &str) -> String {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if let Some(first) = tokens.first().copied() {
            if let Some(expansion) = self.aliases.get(first) {
                if tokens.len() > 1 {
                    return format!("{} {}", expansion, tokens[1..].join(" "));
                } else {
                    return format!("{}", expansion);
                }
            }
        }
        String::from(line)
    }

    fn reap_children(&mut self, nohang: bool) -> ReapResult {
        let flags = (if nohang { waitpid_flags::WNOHANG } else { 0 })
            | waitpid_flags::WUNTRACED
            | waitpid_flags::WCONTINUED;

        loop {
            match syscall::waitpid(-1, flags) {
                Ok((0, _)) => return ReapResult::NoStatus,
                Ok((pid, status)) if pid > 0 => {
                    self.handle_child_status(pid as u32, status);
                    if !nohang {
                        return ReapResult::StatusUpdated;
                    }
                }
                Ok(_) => return ReapResult::NoStatus,
                Err(Errno::ECHILD) => return ReapResult::NoChild,
                Err(Errno::EINTR) => continue,
                Err(_) => return ReapResult::NoStatus,
            }
        }
    }

    fn handle_child_status(&mut self, pid: u32, status: i32) {
        let Some(idx) =
            self.jobs.iter().position(|job| job.processes.iter().any(|process| process.pid == pid))
        else {
            return;
        };

        let state_changed = self.jobs[idx].update_process(pid, status);
        if state_changed {
            self.jobs[idx].notify = true;
        }
    }

    fn print_job_notifications(&mut self) {
        let mut idx = 0;
        while idx < self.jobs.len() {
            let should_remove = if self.jobs[idx].notify && self.jobs[idx].background {
                print_job_update(&self.jobs[idx]);
                self.jobs[idx].notify = false;
                self.jobs[idx].state == JobState::Completed
            } else {
                self.jobs[idx].state == JobState::Completed && self.jobs[idx].background
            };

            if should_remove {
                self.jobs.remove(idx);
            } else {
                idx += 1;
            }
        }
    }

    fn launch_job(&mut self, cmds: &[Cmd<'_>], background: bool, command: &str) {
        match spawn_job(cmds, background, &self.env) {
            Ok((pgid, pids)) => {
                let job_id = self.next_job_id;
                self.next_job_id += 1;
                self.jobs.push(Job::new(job_id, pgid, String::from(command), background, pids));

                if background {
                    let msg = format!("[{}] {}\n", job_id, pgid);
                    write_str(&msg);
                } else {
                    stem::debug!(
                        "sh: foreground handoff start job_id={} pgid={} shell_pgid={} cmd='{}'",
                        job_id,
                        pgid,
                        self.shell_pgid,
                        command
                    );
                    let _ = vfs::tcsetpgrp(TTY_FD, pgid);
                    stem::debug!(
                        "sh: foreground handoff done job_id={} pgid={} cmd='{}'",
                        job_id,
                        pgid,
                        command
                    );
                    write_str("\x1B[?25l"); // hide cursor while child runs
                    self.wait_for_foreground_job(job_id);
                    write_str("\x1B[?25h"); // show cursor when shell regains control
                }
            }
            Err(err) => {
                let msg = format!("sh: spawn failed: {}\n", err);
                write_str(&msg);
            }
        }
    }

    fn wait_for_foreground_job(&mut self, job_id: usize) {
        loop {
            let Some(idx) = self.job_index(job_id) else {
                break;
            };
            if self.jobs[idx].state != JobState::Running {
                break;
            }
            if self.reap_children(false) == ReapResult::NoChild {
                // Defensive fallback: if waitpid reports no children while this
                // foreground job is still marked running, avoid spinning forever
                // and return control to the shell.
                self.jobs[idx].state = JobState::Completed;
                if self.jobs[idx].last_status.is_none() {
                    self.jobs[idx].last_status = Some(0);
                }
                break;
            }
        }

        let _ = vfs::tcsetpgrp(TTY_FD, self.shell_pgid);

        let Some(idx) = self.job_index(job_id) else {
            return;
        };

        self.jobs[idx].background = false;
        self.jobs[idx].notify = false;
        self.last_foreground_status = self.jobs[idx].last_status;

        match self.jobs[idx].state {
            JobState::Stopped => {
                if let Some(status) = self.jobs[idx].last_status {
                    let msg = format!("stopped by signal {}\n", wstopsig(status));
                    write_str(&msg);
                }
            }
            JobState::Completed => {
                if let Some(status) = self.jobs[idx].last_status {
                    if wifsignaled(status) {
                        let msg = format!("terminated by signal {}\n", wtermsig(status));
                        write_str(&msg);
                    }
                }
                self.jobs.remove(idx);
            }
            JobState::Running => {}
        }
    }

    fn list_jobs(&self) {
        for job in &self.jobs {
            let state = match job.state {
                JobState::Running => "running",
                JobState::Stopped => "stopped",
                JobState::Completed => "done",
            };
            let msg = format!("[{}] {} {}\n", job.id, state, job.command);
            write_str(&msg);
        }
    }

    fn resume_job(&mut self, arg: Option<&str>, foreground: bool) {
        let Some(idx) = self.resolve_job(arg) else {
            write_str("sh: job not found\n");
            return;
        };

        let pgid = self.jobs[idx].pgid;
        self.jobs[idx].background = !foreground;
        self.jobs[idx].mark_running();

        if foreground {
            let _ = vfs::tcsetpgrp(TTY_FD, pgid);
        }

        let _ = signal::kill(-(pgid as i32), SIGCONT);

        if foreground {
            let job_id = self.jobs[idx].id;
            self.wait_for_foreground_job(job_id);
        } else {
            let msg = format!("[{}] {}\n", self.jobs[idx].id, pgid);
            write_str(&msg);
        }
    }

    fn resolve_job(&self, arg: Option<&str>) -> Option<usize> {
        if let Some(id) = arg.and_then(parse_job_id) {
            return self.job_index(id);
        }

        self.jobs.iter().rposition(|job| job.state != JobState::Completed)
    }

    fn job_index(&self, job_id: usize) -> Option<usize> {
        self.jobs.iter().position(|job| job.id == job_id)
    }

    fn terminate_jobs(&mut self) {
        for job in &self.jobs {
            let _ = signal::kill(-(job.pgid as i32), SIGCONT);
            let _ = signal::kill(-(job.pgid as i32), abi::signal::SIGTERM);
        }
        let _ = self.reap_children(true);
    }
}

fn install_signal_handlers() {
    let ignore = SigAction { handler: SIG_IGN, mask: SigSet::EMPTY, flags: 0, restorer: 0 };
    let _ = signal::sigaction(SIGINT, Some(&ignore), None);
    let _ = signal::sigaction(SIGTSTP, Some(&ignore), None);
    let _ = signal::sigaction(SIGTTIN, Some(&ignore), None);
    let _ = signal::sigaction(SIGTTOU, Some(&ignore), None);
}

fn prompt(last_status: Option<i32>) {
    let status = match last_status {
        Some(code) if wifexited(code) && wexitstatus(code) == 0 => "\x1B[1;92mOK\x1B[0m",
        Some(_) => "\x1B[1;91mERR\x1B[0m",
        None => "\x1B[1;95mBOOT\x1B[0m",
    };

    let mut buf = [0u8; 256];
    match syscall::vfs_getcwd(&mut buf) {
        Ok(n) => {
            let cwd = core::str::from_utf8(&buf[..n]).unwrap_or("/");
            let out = format!(
                "\x1B[1;95mTHING\x1B[0m\x1B[1;96m-OS\x1B[0m \x1B[2;94m[\x1B[0m{}\x1B[2;94m]\x1B[0m \x1B[1;93m{}\x1B[0m \x1B[1;96m>\x1B[0m ",
                status, cwd
            );
            write_str(&out);
        }
        Err(_) => {
            let out = format!(
                "\x1B[1;95mTHING\x1B[0m\x1B[1;96m-OS\x1B[0m \x1B[2;94m[\x1B[0m{}\x1B[2;94m]\x1B[0m \x1B[1;96m>\x1B[0m ",
                status
            );
            write_str(&out);
        }
    }
}

struct TtyModeGuard {
    original: Option<termios::Termios>,
}

impl TtyModeGuard {
    fn raw(fd: u32) -> Self {
        let mut original = termios::Termios::default();
        if vfs::tcgetattr(fd, &mut original).is_err() {
            return Self { original: None };
        }

        let mut raw = original;
        raw.c_lflag &=
            !(termios::ICANON | termios::ECHO | termios::ECHOE | termios::ECHONL | termios::ISIG);
        raw.c_cc[termios::VMIN] = 1;
        raw.c_cc[termios::VTIME] = 0;
        if vfs::tcsetattr(fd, &raw).is_err() {
            return Self { original: None };
        }

        Self { original: Some(original) }
    }

    fn is_active(&self) -> bool {
        self.original.is_some()
    }
}

impl Drop for TtyModeGuard {
    fn drop(&mut self) {
        if let Some(original) = self.original.as_ref() {
            let _ = vfs::tcsetattr(TTY_FD, original);
        }
    }
}

fn env_var_equals(key: &[u8], expected: &[u8]) -> bool {
    let mut buf = [0u8; 32];
    match syscall::env_get(key, &mut buf) {
        Ok(n) if n == expected.len() && n <= buf.len() => &buf[..n] == expected,
        _ => false,
    }
}

fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn trailing_word_start(bytes: &[u8]) -> Option<usize> {
    if bytes.is_empty() || !is_word_byte(*bytes.last()?) {
        return None;
    }

    let mut idx = bytes.len() - 1;
    while idx > 0 && is_word_byte(bytes[idx - 1]) {
        idx -= 1;
    }
    Some(idx)
}

fn common_prefix_len(candidates: &[String]) -> usize {
    let Some(first) = candidates.first() else {
        return 0;
    };

    let first_bytes = first.as_bytes();
    let mut shared = first_bytes.len();
    for candidate in candidates.iter().skip(1) {
        let candidate_bytes = candidate.as_bytes();
        let mut idx = 0;
        while idx < shared
            && idx < candidate_bytes.len()
            && candidate_bytes[idx] == first_bytes[idx]
        {
            idx += 1;
        }
        shared = idx;
        if shared == 0 {
            break;
        }
    }
    shared
}

fn complete_from_current_dir(fragment: &str) -> Option<String> {
    if fragment.is_empty() {
        return None;
    }

    let fd = vfs::vfs_open(".", vfs_flags::O_RDONLY).ok()?;
    let mut matches = Vec::new();
    let mut buf = [0u8; 1024];

    loop {
        match vfs::vfs_readdir(fd, &mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let mut offset = 0;
                while offset < n {
                    let mut end = offset;
                    while end < n && buf[end] != 0 {
                        end += 1;
                    }

                    if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
                        if !name.is_empty() && name.starts_with(fragment) {
                            matches.push(String::from(name));
                        }
                    }

                    offset = end + 1;
                }
            }
            Err(_) => break,
        }
    }

    let _ = syscall::vfs_close(fd);

    if matches.is_empty() {
        return None;
    }

    matches.sort();
    if matches.len() == 1 {
        let mut completion = matches.remove(0);
        completion.push(' ');
        return Some(completion);
    }

    let shared = common_prefix_len(&matches);
    if shared > fragment.len() {
        return Some(String::from(&matches[0][..shared]));
    }

    None
}

/// Move a byte index backward past one complete UTF-8 character.
fn utf8_prev_boundary(bytes: &[u8], pos: usize) -> usize {
    if pos == 0 {
        return 0;
    }
    let mut i = pos - 1;
    // Skip continuation bytes (10xxxxxx).
    while i > 0 && bytes[i] & 0xC0 == 0x80 {
        i -= 1;
    }
    i
}

/// Move a byte index forward past one complete UTF-8 character.
fn utf8_next_boundary(bytes: &[u8], pos: usize) -> usize {
    if pos >= bytes.len() {
        return bytes.len();
    }
    let mut i = pos + 1;
    while i < bytes.len() && bytes[i] & 0xC0 == 0x80 {
        i += 1;
    }
    i
}

/// Count approximate display columns for a byte slice (1 column per character).
fn utf8_char_count(bytes: &[u8]) -> usize {
    if let Ok(s) = core::str::from_utf8(bytes) { s.chars().count() } else { bytes.len() }
}

fn redraw_line_at_cursor(bytes: &[u8], cursor: usize, last_status: Option<i32>) {
    write_str("\r\x1B[K");
    prompt(last_status);
    if let Ok(text) = core::str::from_utf8(bytes) {
        write_str(text);
    }
    // Move the terminal cursor back by the number of *characters* after the
    // insertion point, not the number of bytes.
    let cols = utf8_char_count(&bytes[cursor..]);
    if cols > 0 {
        let seq = format!("\x1B[{}D", cols);
        write_str(&seq);
    }
}

fn read_line(last_status: Option<i32>, history: &mut Vec<String>) -> ReadLineResult {
    let tty_guard = TtyModeGuard::raw(TTY_FD);
    let interactive_pipe =
        !tty_guard.is_active() && env_var_equals(b"THINGOS_INTERACTIVE_PIPE", b"1");
    if !tty_guard.is_active() && !interactive_pipe {
        let mut buf = [0u8; 512];
        let mut bytes = Vec::new();
        loop {
            match syscall::vfs_read(0, &mut buf) {
                Ok(0) => return ReadLineResult::Eof,
                Ok(n) => {
                    for &b in &buf[..n] {
                        bytes.push(b);
                        if b == b'\n' {
                            return ReadLineResult::Line(
                                String::from_utf8(bytes).unwrap_or_default(),
                            );
                        }
                    }
                }
                Err(Errno::EINTR) => return ReadLineResult::Interrupted,
                Err(_) => return ReadLineResult::Eof,
            }
        }
    }

    let _tty_guard = tty_guard;
    write_str("\x1B[?25h"); // ensure cursor is visible for input
    let mut one = [0u8; 1];
    let mut bytes: Vec<u8> = Vec::new();
    // Byte index of the insertion point within `bytes`.
    let mut cursor: usize = 0;
    // History navigation state: None = editing a fresh line, Some(i) = viewing history[i].
    let mut hist_idx: Option<usize> = None;
    // Snapshot of the fresh line saved when history browsing begins.
    let mut saved_line: Vec<u8> = Vec::new();

    loop {
        match syscall::vfs_read(0, &mut one) {
            Ok(0) => return ReadLineResult::Eof,
            Ok(_) => match one[0] {
                b'\r' | b'\n' => {
                    write_str("\n");
                    // Persist non-empty, non-duplicate-of-last lines in history.
                    if let Ok(s) = core::str::from_utf8(&bytes) {
                        let trimmed = s.trim();
                        if !trimmed.is_empty()
                            && history.last().map(|h| h.as_str()) != Some(trimmed)
                        {
                            history.push(String::from(trimmed));
                        }
                    }
                    bytes.push(b'\n');
                    return ReadLineResult::Line(String::from_utf8(bytes).unwrap_or_default());
                }
                0x03 => {
                    // Ctrl-C: signal interrupt
                    write_str("^C\n");
                    return ReadLineResult::Interrupted;
                }
                0x04 => {
                    // Ctrl-D: EOF on empty line
                    if bytes.is_empty() {
                        return ReadLineResult::Eof;
                    }
                }
                0x01 => {
                    // Ctrl-A: move cursor to beginning of line
                    cursor = 0;
                    redraw_line_at_cursor(&bytes, cursor, last_status);
                }
                0x05 => {
                    // Ctrl-E: move cursor to end of line
                    cursor = bytes.len();
                    redraw_line_at_cursor(&bytes, cursor, last_status);
                }
                0x0b => {
                    // Ctrl-K: kill from cursor to end of line
                    bytes.truncate(cursor);
                    redraw_line_at_cursor(&bytes, cursor, last_status);
                }
                0x15 => {
                    // Ctrl-U: kill entire line
                    bytes.clear();
                    cursor = 0;
                    redraw_line_at_cursor(&bytes, cursor, last_status);
                }
                0x17 => {
                    // Ctrl-W: kill the word immediately before the cursor
                    if cursor > 0 {
                        let word_start = trailing_word_start(&bytes[..cursor]).unwrap_or(0);
                        bytes.drain(word_start..cursor);
                        cursor = word_start;
                        redraw_line_at_cursor(&bytes, cursor, last_status);
                    }
                }
                0x08 | 0x7f => {
                    // Backspace / DEL: delete the character before the cursor
                    if cursor > 0 {
                        let prev = utf8_prev_boundary(&bytes, cursor);
                        bytes.drain(prev..cursor);
                        cursor = prev;
                        redraw_line_at_cursor(&bytes, cursor, last_status);
                    }
                }
                b'\t' => {
                    // Tab: complete the word immediately before the cursor
                    let Some(start) = trailing_word_start(&bytes[..cursor]) else {
                        continue;
                    };
                    let Ok(fragment) = core::str::from_utf8(&bytes[start..cursor]) else {
                        continue;
                    };
                    let Some(completion) = complete_from_current_dir(fragment) else {
                        continue;
                    };
                    // Replace [start..cursor] with the completion and preserve any suffix.
                    let suffix: Vec<u8> = bytes[cursor..].to_vec();
                    bytes.truncate(start);
                    bytes.extend_from_slice(completion.as_bytes());
                    cursor = bytes.len();
                    bytes.extend_from_slice(&suffix);
                    redraw_line_at_cursor(&bytes, cursor, last_status);
                }
                0x1b => {
                    // ESC: beginning of a VT/ANSI escape sequence.
                    // Read the next byte; we only handle CSI sequences (ESC [).
                    if syscall::vfs_read(0, &mut one).unwrap_or(0) == 0 {
                        continue;
                    }
                    if one[0] != b'[' {
                        // Not a CSI sequence (e.g. bare ESC, or ESC O for SS3); ignore.
                        continue;
                    }
                    // Accumulate optional decimal parameter, then the final byte.
                    let mut param: u32 = 0;
                    let mut final_byte: u8 = 0;
                    loop {
                        if syscall::vfs_read(0, &mut one).unwrap_or(0) == 0 {
                            break;
                        }
                        let b2 = one[0];
                        if b2.is_ascii_digit() {
                            param = param.saturating_mul(10).saturating_add((b2 - b'0') as u32);
                        } else {
                            final_byte = b2;
                            break;
                        }
                    }
                    match final_byte {
                        b'A' => {
                            // Up arrow: move to previous history entry
                            if history.is_empty() {
                                continue;
                            }
                            let new_idx = match hist_idx {
                                None => {
                                    saved_line = bytes.clone();
                                    history.len() - 1
                                }
                                Some(i) if i > 0 => i - 1,
                                Some(_) => {
                                    // Already at the oldest entry; nothing to do.
                                    continue;
                                }
                            };
                            hist_idx = Some(new_idx);
                            bytes = history[new_idx].as_bytes().to_vec();
                            cursor = bytes.len();
                            redraw_line_at_cursor(&bytes, cursor, last_status);
                        }
                        b'B' => {
                            // Down arrow: move to next history entry or restore the fresh line
                            match hist_idx {
                                None => {
                                    // Already on the fresh line; nothing to do.
                                    continue;
                                }
                                Some(i) if i + 1 < history.len() => {
                                    hist_idx = Some(i + 1);
                                    bytes = history[i + 1].as_bytes().to_vec();
                                    cursor = bytes.len();
                                    redraw_line_at_cursor(&bytes, cursor, last_status);
                                }
                                Some(_) => {
                                    // Past the most recent entry: restore the fresh line.
                                    hist_idx = None;
                                    bytes = saved_line.clone();
                                    cursor = bytes.len();
                                    redraw_line_at_cursor(&bytes, cursor, last_status);
                                }
                            }
                        }
                        b'C' => {
                            // Right arrow: move cursor one character to the right
                            if cursor < bytes.len() {
                                cursor = utf8_next_boundary(&bytes, cursor);
                                write_str("\x1B[C");
                            }
                        }
                        b'D' => {
                            // Left arrow: move cursor one character to the left
                            if cursor > 0 {
                                cursor = utf8_prev_boundary(&bytes, cursor);
                                write_str("\x1B[D");
                            }
                        }
                        b'H' => {
                            // Home (xterm)
                            cursor = 0;
                            redraw_line_at_cursor(&bytes, cursor, last_status);
                        }
                        b'F' => {
                            // End (xterm)
                            cursor = bytes.len();
                            redraw_line_at_cursor(&bytes, cursor, last_status);
                        }
                        b'~' => {
                            // VT-style extended keys: ESC [ N ~
                            match param {
                                1 | 7 => {
                                    // Home
                                    cursor = 0;
                                    redraw_line_at_cursor(&bytes, cursor, last_status);
                                }
                                3 => {
                                    // Delete: remove the character at the cursor
                                    if cursor < bytes.len() {
                                        let next = utf8_next_boundary(&bytes, cursor);
                                        bytes.drain(cursor..next);
                                        redraw_line_at_cursor(&bytes, cursor, last_status);
                                    }
                                }
                                4 | 8 => {
                                    // End
                                    cursor = bytes.len();
                                    redraw_line_at_cursor(&bytes, cursor, last_status);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                b => {
                    // Accept printable ASCII (0x20–0x7E) and high bytes (UTF-8 lead/continuation).
                    // Values below 0x20 are control characters handled by explicit arms above;
                    // anything that reaches here unhandled is silently discarded.
                    if b >= 0x20 {
                        bytes.insert(cursor, b);
                        cursor += 1;
                        if cursor == bytes.len() {
                            // Cursor is at EOL: just echo the byte.
                            let _ = syscall::vfs_write(1, &one);
                        } else {
                            // Cursor is mid-line: redraw to keep display consistent.
                            redraw_line_at_cursor(&bytes, cursor, last_status);
                        }
                    }
                    // Unrecognised control characters (< 0x20) are silently ignored.
                }
            },
            Err(Errno::EINTR) => return ReadLineResult::Interrupted,
            Err(_) => return ReadLineResult::Eof,
        }
    }
}

fn tokenize_line<'a>(line: &'a str) -> Vec<&'a str> {
    let bytes = line.as_bytes();
    let mut tokens = Vec::new();
    let mut idx = 0;

    while idx < bytes.len() {
        while idx < bytes.len() && bytes[idx].is_ascii_whitespace() {
            idx += 1;
        }
        if idx >= bytes.len() {
            break;
        }

        match bytes[idx] {
            b'|' | b'<' | b'&' => {
                tokens.push(&line[idx..idx + 1]);
                idx += 1;
            }
            b'>' => {
                if idx + 1 < bytes.len() && bytes[idx + 1] == b'>' {
                    tokens.push(&line[idx..idx + 2]);
                    idx += 2;
                } else {
                    tokens.push(&line[idx..idx + 1]);
                    idx += 1;
                }
            }
            b'\'' | b'"' => {
                // Quoted arguments preserve internal whitespace and drop quote marks.
                let quote = bytes[idx];
                idx += 1;
                let start = idx;
                while idx < bytes.len() && bytes[idx] != quote {
                    idx += 1;
                }
                tokens.push(&line[start..idx]);
                if idx < bytes.len() {
                    idx += 1;
                }
            }
            _ => {
                let start = idx;
                while idx < bytes.len()
                    && !bytes[idx].is_ascii_whitespace()
                    && !matches!(bytes[idx], b'|' | b'<' | b'>' | b'&')
                {
                    idx += 1;
                }
                tokens.push(&line[start..idx]);
            }
        }
    }

    tokens
}

fn parse_line<'a>(line: &'a str) -> (Vec<Cmd<'a>>, bool) {
    let mut tokens = tokenize_line(line);
    let background = matches!(tokens.last().copied(), Some("&"));
    if background {
        tokens.pop();
    }

    let mut segments = Vec::new();
    let mut current = Vec::new();
    for token in tokens {
        if token == "|" {
            segments.push(current);
            current = Vec::new();
        } else {
            current.push(token);
        }
    }
    segments.push(current);

    let cmds = segments.iter().filter_map(|segment| Cmd::parse(segment)).collect();
    (cmds, background)
}

fn parse_job_id(text: &str) -> Option<usize> {
    text.strip_prefix('%').unwrap_or(text).parse().ok()
}

fn open_read(path: &str) -> Result<u32, Errno> {
    vfs::vfs_open(path, vfs_flags::O_RDONLY)
}

fn open_write(path: &str, append: bool) -> Result<u32, Errno> {
    let flags = if append {
        vfs_flags::O_WRONLY | vfs_flags::O_CREAT | vfs_flags::O_APPEND
    } else {
        vfs_flags::O_WRONLY | vfs_flags::O_CREAT | vfs_flags::O_TRUNC
    };
    vfs::vfs_open(path, flags)
}

fn canonicalize_path(path: &str) -> Result<String, Errno> {
    let mut buf = [0u8; 4096];
    let len = vfs::vfs_realpath(path, &mut buf)?;
    if len > buf.len() {
        return Err(Errno::ENAMETOOLONG);
    }
    let resolved = core::str::from_utf8(&buf[..len]).map_err(|_| Errno::EINVAL)?;
    Ok(String::from(resolved))
}

fn resolve_executable_path(program: &str, path_prefixes: &[&str]) -> Result<String, Errno> {
    if program.contains('/') {
        return canonicalize_path(program);
    }

    for prefix in path_prefixes {
        let mut candidate = prefix.to_string();
        if !candidate.ends_with('/') {
            candidate.push('/');
        }
        candidate.push_str(program);
        match vfs::vfs_open(&candidate, abi::syscall::vfs_flags::O_RDONLY) {
            Ok(probe_fd) => {
                let _ = syscall::vfs_close(probe_fd);
                return Ok(candidate);
            }
            Err(Errno::ENOENT) | Err(Errno::ENOTDIR) => {}
            Err(err) => return Err(err),
        }
    }

    Ok(format!("/bin/{}", program))
}

fn spawn_job(
    cmds: &[Cmd<'_>],
    background: bool,
    env_map: &BTreeMap<String, String>,
) -> Result<(u32, Vec<u32>), Errno> {
    if cmds.is_empty() {
        return Err(Errno::EINVAL);
    }

    #[cfg(feature = "spawn-timing")]
    let t_job_start = stem::syscall::monotonic_ns();

    let mut pipes = Vec::new();
    for _ in 0..cmds.len().saturating_sub(1) {
        let mut pair = [0u32; 2];
        syscall::pipe(&mut pair)?;
        let _ = syscall::vfs_fcntl(pair[0], fcntl_cmd::F_SETFD, handle_flags::HANDLE_CLOEXEC);
        let _ = syscall::vfs_fcntl(pair[1], fcntl_cmd::F_SETFD, handle_flags::HANDLE_CLOEXEC);
        pipes.push(pair);
    }

    let bg_in = if background { Some(open_read("/dev/null")?) } else { None };
    let bg_out = if background { Some(open_write("/dev/null", false)?) } else { None };

    let path_env = env_map.get("PATH").map(|s| s.as_str()).unwrap_or("/bin:/applications:/drivers");
    let path_prefixes: Vec<&str> = path_env.split(':').collect();

    let mut spawned = Vec::new();
    let mut transient_fds = Vec::new();
    let mut pgid = 0u32;

    let mut spawn_env = BTreeMap::new();
    for (k, v) in env_map {
        spawn_env.insert(k.as_bytes().to_vec(), v.as_bytes().to_vec());
    }

    for (idx, cmd) in cmds.iter().enumerate() {
        stem::debug!("sh: spawning job cmd='{}'", cmd.program);
        #[cfg(feature = "spawn-timing")]
        let t_path_probe_start = stem::syscall::monotonic_ns();
        let path = resolve_executable_path(cmd.program, &path_prefixes)?;
        #[cfg(feature = "spawn-timing")]
        let t_path_probe_end = stem::syscall::monotonic_ns();

        let mut argv = Vec::new();
        argv.push(path.as_bytes().to_vec());
        for arg in &cmd.args {
            argv.push(arg.as_bytes().to_vec());
        }
        let argv_slices: Vec<&[u8]> = argv.iter().map(|arg| arg.as_slice()).collect();

        let stdin_mode = if idx == 0 {
            if let Some(path) = cmd.stdin_file {
                let fd = open_read(path)?;
                transient_fds.push(fd);
                stdio_mode::handle(fd)
            } else if background {
                stdio_mode::handle(bg_in.unwrap())
            } else {
                stdio_mode::INHERIT
            }
        } else {
            stdio_mode::handle(pipes[idx - 1][0])
        };

        let stdout_mode = if idx + 1 < cmds.len() {
            stdio_mode::handle(pipes[idx][1])
        } else if let Some(path) = cmd.stdout_file {
            let fd = open_write(path, cmd.stdout_append)?;
            transient_fds.push(fd);
            stdio_mode::handle(fd)
        } else if background {
            stdio_mode::handle(bg_out.unwrap())
        } else {
            stdio_mode::INHERIT
        };

        let stderr_mode =
            if background { stdio_mode::handle(bg_out.unwrap()) } else { stdio_mode::INHERIT };

        let argv_display: Vec<_> = argv.iter().map(|arg| String::from_utf8_lossy(arg)).collect();
        stem::debug!("sh: spawning '{}' with argv={:?}", path, argv_display);
        #[cfg(feature = "spawn-timing")]
        let t_spawn_call = stem::syscall::monotonic_ns();
        match syscall::spawn_process_ex(
            &path,
            &argv_slices,
            &spawn_env,
            stdin_mode,
            stdout_mode,
            stderr_mode,
            0,
            &[],
        ) {
            Ok(resp) => {
                #[cfg(feature = "spawn-timing")]
                let t_spawn_return = stem::syscall::monotonic_ns();
                let pid = resp.child_pid;
                stem::debug!(
                    "sh: spawned '{}' pid={} idx={} background={} pending_pgid={}",
                    path,
                    pid,
                    idx,
                    background,
                    pgid
                );
                #[cfg(feature = "spawn-timing")]
                let t_before_setpgid = stem::syscall::monotonic_ns();
                if pgid == 0 {
                    pgid = pid;
                    let _ = signal::setpgid(pid as i32, pid as i32);
                    stem::debug!("sh: setpgid leader pid={} -> pgid={}", pid, pid);
                } else {
                    let _ = signal::setpgid(pid as i32, pgid as i32);
                    stem::debug!("sh: setpgid member pid={} -> pgid={}", pid, pgid);
                }
                #[cfg(feature = "spawn-timing")]
                let t_after_setpgid = stem::syscall::monotonic_ns();
                spawned.push(pid);
                #[cfg(feature = "spawn-timing")]
                {
                    let path_probe_us = t_path_probe_end.saturating_sub(t_path_probe_start) / 1_000;
                    let spawn_syscall_us = t_spawn_return.saturating_sub(t_spawn_call) / 1_000;
                    let setpgid_us = t_after_setpgid.saturating_sub(t_before_setpgid) / 1_000;
                    let elapsed_us = t_after_setpgid.saturating_sub(t_job_start) / 1_000;
                    stem::debug!(
                        "SPAWN_TIMING sh '{}': elapsed={}µs | \
                         path_probe={}µs spawn_syscall={}µs setpgid={}µs",
                        path,
                        elapsed_us,
                        path_probe_us,
                        spawn_syscall_us,
                        setpgid_us,
                    );
                }
            }
            Err(err) => {
                #[cfg(feature = "spawn-timing")]
                {
                    let t_err = stem::syscall::monotonic_ns();
                    let spawn_syscall_us = t_err.saturating_sub(t_spawn_call) / 1_000;
                    stem::debug!(
                        "SPAWN_TIMING sh '{}': spawn_syscall={}µs -> error {:?}",
                        path,
                        spawn_syscall_us,
                        err
                    );
                }
                cleanup_fds(&pipes, &transient_fds, bg_in, bg_out);
                if pgid != 0 {
                    let _ = signal::kill(-(pgid as i32), abi::signal::SIGTERM);
                }
                return Err(err);
            }
        }
    }

    #[cfg(feature = "spawn-timing")]
    let t_cleanup_start = stem::syscall::monotonic_ns();
    cleanup_fds(&pipes, &transient_fds, bg_in, bg_out);
    #[cfg(feature = "spawn-timing")]
    {
        let t_cleanup_end = stem::syscall::monotonic_ns();
        let cleanup_us = t_cleanup_end.saturating_sub(t_cleanup_start) / 1_000;
        let total_us = t_cleanup_end.saturating_sub(t_job_start) / 1_000;
        stem::debug!("SPAWN_TIMING sh job: total={}µs | cleanup_fds={}µs", total_us, cleanup_us);
    }
    Ok((pgid, spawned))
}

fn cleanup_fds(pipes: &[[u32; 2]], transient_fds: &[u32], bg_in: Option<u32>, bg_out: Option<u32>) {
    stem::debug!("sh: cleaning up {} pipes", pipes.len());
    for (idx, pair) in pipes.iter().enumerate() {
        stem::trace!("sh: closing pipe {} ends: read={} write={}", idx, pair[0], pair[1]);
        let _ = syscall::vfs_close(pair[0]);
        let _ = syscall::vfs_close(pair[1]);
    }
    for &fd in transient_fds {
        let _ = syscall::vfs_close(fd);
    }
    if let Some(fd) = bg_in {
        let _ = syscall::vfs_close(fd);
    }
    if let Some(fd) = bg_out {
        let _ = syscall::vfs_close(fd);
    }
}

fn print_job_update(job: &Job) {
    let state = match job.state {
        JobState::Running => "continued",
        JobState::Stopped => "stopped",
        JobState::Completed => "done",
    };

    let detail = match job.last_status {
        Some(status) if wifexited(status) => format!("exit {}", wexitstatus(status)),
        Some(status) if wifsignaled(status) => format!("signal {}", wtermsig(status)),
        Some(status) if wifstopped(status) => format!("signal {}", wstopsig(status)),
        Some(status) if wifcontinued(status) => String::from("continued"),
        _ => String::new(),
    };

    let msg = if detail.is_empty() {
        format!("[{}] {} {}\n", job.id, state, job.command)
    } else {
        format!("[{}] {} ({}) {}\n", job.id, state, detail, job.command)
    };
    write_str(&msg);
}

fn write_str(s: &str) {
    let _ = syscall::vfs_write(1, s.as_bytes());
}

fn print_motd() {
    let mut buf = [0u8; 1024];
    for path in ["/run/motd", "/etc/motd"] {
        let fd = match vfs::vfs_open(path, vfs_flags::O_RDONLY) {
            Ok(fd) => fd,
            Err(_) => continue,
        };

        let res = syscall::vfs_read(fd, &mut buf);
        let _ = syscall::vfs_close(fd);

        let Ok(n) = res else {
            continue;
        };
        if n == 0 {
            continue;
        }

        let _ = syscall::vfs_write(1, &buf[..n]);
        if buf[n - 1] != b'\n' {
            write_str("\n");
        }
        return;
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    stem::info!("SH: starting v0.1.0-debug");
    install_signal_handlers();
    print_motd();

    let mut shell = Shell::new();
    shell.load_profile("/etc/profile");

    loop {
        let _ = shell.reap_children(true);
        shell.print_job_notifications();

        prompt(shell.last_foreground_status);
        let line = match read_line(shell.last_foreground_status, &mut shell.history) {
            ReadLineResult::Line(line) => line,
            ReadLineResult::Interrupted => {
                write_str("\n");
                continue;
            }
            ReadLineResult::Eof => break,
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let expanded = shell.expand_aliases(trimmed);
        let (cmds, background) = parse_line(&expanded);
        if cmds.is_empty() {
            continue;
        }

        if cmds.len() == 1 {
            match cmds[0].program {
                "exit" => break,
                "cd" => {
                    let target = cmds[0].args.first().copied().unwrap_or("/");
                    if let Err(err) = syscall::vfs_chdir(target) {
                        let msg = format!("cd: {}: {}\n", target, err);
                        write_str(&msg);
                    }
                    continue;
                }
                "jobs" => {
                    shell.list_jobs();
                    continue;
                }
                "fg" => {
                    shell.resume_job(cmds[0].args.first().copied(), true);
                    continue;
                }
                "bg" => {
                    shell.resume_job(cmds[0].args.first().copied(), false);
                    continue;
                }
                "export" => {
                    let arg = cmds[0].args.first().copied().unwrap_or("");
                    if let Some((name, value)) = arg.split_once('=') {
                        let name = name.trim();
                        let value = value.trim().trim_matches('\'').trim_matches('"');
                        shell.env.insert(String::from(name), String::from(value));
                    } else if !arg.is_empty() {
                        // Just 'export NAME' (present in env but no value change? or just list?)
                        // For now we only support 'export NAME=VALUE'
                    } else {
                        // List all env vars
                        for (k, v) in &shell.env {
                            write_str(&format!("{}={}\n", k, v));
                        }
                    }
                    continue;
                }
                _ => {}
            }
        }

        shell.launch_job(&cmds, background, &expanded);
    }

    let _ = vfs::tcsetpgrp(TTY_FD, shell.shell_pgid);
    write_str("\x1B[?25h"); // restore cursor visibility on exit
    shell.terminate_jobs();
    syscall::exit(0)
}
