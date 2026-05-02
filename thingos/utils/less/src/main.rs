#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use stem::abi::device::{DeviceCall, DeviceKind};
use stem::abi::syscall::vfs_flags;
use stem::abi::termios;
use stem::syscall::{argv_get, exit, vfs, vfs_close, vfs_open, vfs_read, vfs_write};

const TTY_FD: u32 = 0;
const DEFAULT_ROWS: usize = 24;
const DEFAULT_COLS: usize = 80;
const READ_BUF_SIZE: usize = 4096;

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
        raw.c_iflag &= !(termios::ICRNL | termios::IXON);
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

#[derive(Default)]
struct Args {
    quit_if_one_screen: bool,
    files: Vec<String>,
}

fn print(fd: u32, msg: &str) {
    let _ = vfs_write(fd, msg.as_bytes());
}

fn write_all(fd: u32, mut data: &[u8]) -> Result<(), ()> {
    while !data.is_empty() {
        match vfs_write(fd, data) {
            Ok(0) => return Err(()),
            Ok(n) => data = &data[n..],
            Err(_) => return Err(()),
        }
    }
    Ok(())
}

fn get_args() -> Vec<String> {
    let mut len = 0;
    if let Ok(l) = argv_get(&mut []) {
        len = l;
    }
    if len == 0 {
        return Vec::new();
    }

    let mut buf = vec![0u8; len];
    if argv_get(&mut buf).is_err() {
        return Vec::new();
    }

    stem::utils::parse_argv(&buf)
        .into_iter()
        .skip(1)
        .filter_map(|b| core::str::from_utf8(b).ok().map(String::from))
        .collect()
}

fn usage() {
    print(1, "usage: less [-F] [FILE...]\n");
    print(1, "commands: q quit, space/f page down, b page up, j/k line, g/G top/bottom\n");
}

fn parse_args(args: &[String]) -> Result<Args, ()> {
    let mut parsed = Args::default();
    let mut literal_files = false;

    for arg in args {
        if literal_files {
            parsed.files.push(arg.clone());
            continue;
        }

        if arg == "--" {
            literal_files = true;
        } else if arg == "--help" {
            usage();
            exit(0);
        } else if arg.starts_with('-') && arg.len() > 1 && arg != "-" {
            for ch in arg.chars().skip(1) {
                match ch {
                    'F' => parsed.quit_if_one_screen = true,
                    'h' => {
                        usage();
                        exit(0);
                    }
                    _ => {
                        print(2, "less: unsupported option -- ");
                        let mut buf = [0u8; 4];
                        print(2, ch.encode_utf8(&mut buf));
                        print(2, "\n");
                        return Err(());
                    }
                }
            }
        } else {
            parsed.files.push(arg.clone());
        }
    }

    Ok(parsed)
}

fn read_fd(fd: u32) -> Result<Vec<u8>, ()> {
    let mut data = Vec::new();
    let mut buf = vec![0u8; READ_BUF_SIZE];
    loop {
        match vfs_read(fd, &mut buf) {
            Ok(0) => break,
            Ok(n) => data.extend_from_slice(&buf[..n]),
            Err(_) => return Err(()),
        }
    }
    Ok(data)
}

fn read_path(path: &str) -> Result<Vec<u8>, ()> {
    if path == "-" {
        return read_fd(0);
    }

    match vfs_open(path, vfs_flags::O_RDONLY) {
        Ok(fd) => {
            let result = read_fd(fd);
            let _ = vfs_close(fd);
            result
        }
        Err(_) => Err(()),
    }
}

fn collect_input(files: &[String]) -> Result<Vec<u8>, ()> {
    if files.is_empty() {
        return read_fd(0);
    }

    let mut data = Vec::new();
    for (idx, path) in files.iter().enumerate() {
        match read_path(path) {
            Ok(mut bytes) => {
                if files.len() > 1 {
                    if idx > 0 && !data.ends_with(b"\n") {
                        data.push(b'\n');
                    }
                    data.extend_from_slice(b"::::::::::::::\n");
                    data.extend_from_slice(path.as_bytes());
                    data.extend_from_slice(b"\n::::::::::::::\n");
                }
                data.append(&mut bytes);
            }
            Err(_) => {
                print(2, "less: cannot open ");
                print(2, path);
                print(2, "\n");
                return Err(());
            }
        }
    }
    Ok(data)
}

fn line_starts(data: &[u8]) -> Vec<usize> {
    let mut starts = Vec::new();
    starts.push(0);
    for (idx, byte) in data.iter().enumerate() {
        if *byte == b'\n' && idx + 1 < data.len() {
            starts.push(idx + 1);
        }
    }
    starts
}

fn line_end(data: &[u8], starts: &[usize], line: usize) -> usize {
    if line + 1 < starts.len() { starts[line + 1] } else { data.len() }
}

fn terminal_size() -> (usize, usize) {
    let mut winsize = termios::Winsize::default();
    let call = DeviceCall {
        kind: DeviceKind::Terminal,
        op: termios::TERMINAL_OP_TIOCGWINSZ,
        in_ptr: 0,
        in_len: 0,
        out_ptr: &mut winsize as *mut termios::Winsize as u64,
        out_len: core::mem::size_of::<termios::Winsize>() as u32,
    };

    if vfs::vfs_device_call_raw(TTY_FD, &call).is_ok() {
        let rows = if winsize.ws_row == 0 { DEFAULT_ROWS } else { winsize.ws_row as usize };
        let cols = if winsize.ws_col == 0 { DEFAULT_COLS } else { winsize.ws_col as usize };
        (rows, cols)
    } else {
        (DEFAULT_ROWS, DEFAULT_COLS)
    }
}

fn visible_line_count(rows: usize) -> usize {
    rows.saturating_sub(1).max(1)
}

fn write_display_line(data: &[u8], width: usize) {
    let mut col = 0usize;
    for &byte in data {
        if byte == b'\n' || byte == b'\r' {
            break;
        }

        match byte {
            b'\t' => {
                let spaces = 4 - (col % 4);
                for _ in 0..spaces {
                    if col >= width {
                        break;
                    }
                    let _ = vfs_write(1, b" ");
                    col += 1;
                }
            }
            0x20..=0x7e => {
                if col >= width {
                    break;
                }
                let _ = vfs_write(1, &[byte]);
                col += 1;
            }
            _ => {
                if col + 1 >= width {
                    break;
                }
                let shown = [b'^', byte ^ 0x40];
                let _ = vfs_write(1, &shown);
                col += 2;
            }
        }
    }
    let _ = vfs_write(1, b"\r\n");
}

fn render(data: &[u8], starts: &[usize], top_line: usize, rows: usize, cols: usize) {
    let page_lines = visible_line_count(rows);
    let width = cols.max(1);
    let _ = vfs_write(1, b"\x1B[2J\x1B[H");

    for y in 0..page_lines {
        let line = top_line + y;
        if line < starts.len() {
            let start = starts[line];
            let end = line_end(data, starts, line);
            write_display_line(&data[start..end], width);
        } else {
            let _ = vfs_write(1, b"~\r\n");
        }
    }

    let percent = if starts.is_empty() {
        100
    } else {
        ((top_line + 1).saturating_mul(100) / starts.len()).min(100)
    };
    let prompt =
        alloc::format!("less: line {}/{} ({}%)", top_line + 1, starts.len().max(1), percent);
    let mut prompt_bytes = prompt.into_bytes();
    if prompt_bytes.len() > width {
        prompt_bytes.truncate(width);
    }
    let _ = write_all(1, &prompt_bytes);
}

fn read_command() -> Option<u8> {
    let mut one = [0u8; 1];
    match vfs_read(0, &mut one) {
        Ok(0) | Err(_) => None,
        Ok(_) if one[0] == 0x1b => {
            if vfs_read(0, &mut one).unwrap_or(0) == 0 || one[0] != b'[' {
                return Some(0x1b);
            }
            if vfs_read(0, &mut one).unwrap_or(0) == 0 {
                return Some(0x1b);
            }
            match one[0] {
                b'A' => Some(b'k'),
                b'B' => Some(b'j'),
                b'5' => {
                    let _ = vfs_read(0, &mut one);
                    Some(b'b')
                }
                b'6' => {
                    let _ = vfs_read(0, &mut one);
                    Some(b' ')
                }
                _ => Some(0x1b),
            }
        }
        Ok(_) => Some(one[0]),
    }
}

fn page(data: &[u8], starts: &[usize], quit_if_one_screen: bool) {
    let (rows, cols) = terminal_size();
    let page_lines = visible_line_count(rows);
    if quit_if_one_screen && starts.len() <= page_lines {
        let _ = write_all(1, data);
        return;
    }

    let tty_guard = TtyModeGuard::raw(TTY_FD);
    if !tty_guard.is_active() {
        let _ = write_all(1, data);
        return;
    }
    let _tty_guard = tty_guard;

    let mut top_line = 0usize;
    loop {
        render(data, starts, top_line, rows, cols);
        match read_command() {
            Some(b'q') | Some(0x03) | Some(0x04) => break,
            Some(b' ') | Some(b'f') => {
                top_line = top_line.saturating_add(page_lines).min(starts.len().saturating_sub(1));
            }
            Some(b'b') => top_line = top_line.saturating_sub(page_lines),
            Some(b'j') | Some(b'\n') | Some(b'\r') => {
                top_line = top_line.saturating_add(1).min(starts.len().saturating_sub(1));
            }
            Some(b'k') => top_line = top_line.saturating_sub(1),
            Some(b'g') => top_line = 0,
            Some(b'G') => top_line = starts.len().saturating_sub(page_lines),
            Some(b'h') | Some(b'?') => {
                let _ = vfs_write(
                    1,
                    b"\r\x1B[Kq quit  space/f page down  b page up  j/k line  g/G top/bottom",
                );
                let _ = read_command();
            }
            None => break,
            _ => {}
        }
    }

    let _ = vfs_write(1, b"\r\x1B[K\x1B[?25h");
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    let parsed = match parse_args(&args) {
        Ok(parsed) => parsed,
        Err(_) => exit(1),
    };

    let data = match collect_input(&parsed.files) {
        Ok(data) => data,
        Err(_) => exit(1),
    };

    let stdin_is_tty = vfs::vfs_isatty(0).unwrap_or(false);
    let stdout_is_tty = vfs::vfs_isatty(1).unwrap_or(false);
    if !stdin_is_tty || !stdout_is_tty {
        let _ = write_all(1, &data);
        exit(0);
    }

    let starts = line_starts(&data);
    page(&data, &starts, parsed.quit_if_one_screen);
    exit(0)
}
