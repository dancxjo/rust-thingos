#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use stem::syscall::vfs::vfs_lstat;
use stem::syscall::{argv_get, exit, vfs_close, vfs_open, vfs_readdir, vfs_write};

const S_IFDIR: u32 = 0o040000;
const S_IFMT: u32 = 0o170000;
const O_RDONLY: u32 = 0;
const READDIR_BUF_SIZE: usize = 4096;

#[derive(Default)]
struct Options {
    show_all: bool,
    dirs_only: bool,
    max_depth: Option<usize>,
    paths: Vec<String>,
}

#[derive(Default)]
struct Counts {
    dirs: usize,
    files: usize,
}

struct Entry {
    name: String,
    path: String,
    is_dir: bool,
}

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
        .filter_map(|arg| core::str::from_utf8(arg).ok().map(String::from))
        .collect()
}

fn write_to_fd(fd: u32, msg: &str) {
    let _ = vfs_write(fd, msg.as_bytes());
}

fn usage() {
    write_to_fd(2, "usage: tree [-a] [-d] [-L depth] [PATH...]\n");
}

fn parse_depth(value: &str) -> Result<usize, &'static str> {
    value.parse::<usize>().map_err(|_| "tree: invalid depth\n")
}

fn parse_args(args: &[String]) -> Result<Options, &'static str> {
    let mut opts = Options::default();
    let mut i = 0usize;

    while i < args.len() {
        let arg = &args[i];
        if arg == "--" {
            opts.paths.extend(args.iter().skip(i + 1).cloned());
            break;
        } else if arg == "-a" {
            opts.show_all = true;
        } else if arg == "-d" {
            opts.dirs_only = true;
        } else if arg == "-L" {
            i += 1;
            if i >= args.len() {
                return Err("tree: missing depth after -L\n");
            }
            opts.max_depth = Some(parse_depth(&args[i])?);
        } else if let Some(value) = arg.strip_prefix("-L") {
            if value.is_empty() {
                return Err("tree: missing depth after -L\n");
            }
            opts.max_depth = Some(parse_depth(value)?);
        } else if arg.starts_with('-') && arg.len() > 1 {
            for flag in arg.chars().skip(1) {
                match flag {
                    'a' => opts.show_all = true,
                    'd' => opts.dirs_only = true,
                    _ => return Err("tree: unsupported option\n"),
                }
            }
        } else {
            opts.paths.push(arg.clone());
        }

        i += 1;
    }

    if opts.paths.is_empty() {
        opts.paths.push(String::from("."));
    }

    Ok(opts)
}

fn path_join(parent: &str, name: &str) -> String {
    if parent == "/" {
        let mut path = String::from("/");
        path.push_str(name);
        return path;
    }

    let mut path = String::from(parent);
    if !path.ends_with('/') {
        path.push('/');
    }
    path.push_str(name);
    path
}

fn is_hidden(name: &str) -> bool {
    name.starts_with('.') && name != "." && name != ".."
}

fn read_entries(path: &str, opts: &Options, had_error: &mut bool) -> Vec<Entry> {
    let fd = match vfs_open(path, O_RDONLY) {
        Ok(fd) => fd,
        Err(e) => {
            write_to_fd(2, &alloc::format!("tree: cannot open '{}': {:?}\n", path, e));
            *had_error = true;
            return Vec::new();
        }
    };

    let mut names = Vec::new();
    let mut buf = alloc::vec![0u8; READDIR_BUF_SIZE];
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

                    if end > offset {
                        if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
                            if name != "." && name != ".." && (opts.show_all || !is_hidden(name)) {
                                names.push(String::from(name));
                            }
                        }
                    }

                    offset = end + 1;
                }
            }
            Err(e) => {
                write_to_fd(2, &alloc::format!("tree: cannot read '{}': {:?}\n", path, e));
                *had_error = true;
                break;
            }
        }
    }

    let _ = vfs_close(fd);
    names.sort();
    names.dedup();

    let mut entries = Vec::new();
    for name in names {
        let child = path_join(path, &name);
        match vfs_lstat(&child) {
            Ok(stat) => {
                let is_dir = (stat.mode & S_IFMT) == S_IFDIR;
                if !opts.dirs_only || is_dir {
                    entries.push(Entry { name, path: child, is_dir });
                }
            }
            Err(e) => {
                write_to_fd(2, &alloc::format!("tree: cannot stat '{}': {:?}\n", child, e));
                *had_error = true;
            }
        }
    }

    entries
}

fn walk_dir(
    path: &str,
    prefix: &str,
    depth: usize,
    opts: &Options,
    counts: &mut Counts,
    had_error: &mut bool,
) {
    if opts.max_depth.is_some_and(|limit| depth >= limit) {
        return;
    }

    let entries = read_entries(path, opts, had_error);
    let last_index = entries.len().saturating_sub(1);

    for (idx, entry) in entries.iter().enumerate() {
        let is_last = idx == last_index;
        let connector = if is_last { "`-- " } else { "|-- " };
        write_to_fd(1, &alloc::format!("{}{}{}\n", prefix, connector, entry.name));

        if entry.is_dir {
            counts.dirs += 1;
            let branch = if is_last { "    " } else { "|   " };
            let mut child_prefix = String::from(prefix);
            child_prefix.push_str(branch);
            walk_dir(&entry.path, &child_prefix, depth + 1, opts, counts, had_error);
        } else {
            counts.files += 1;
        }
    }
}

fn print_tree(path: &str, opts: &Options, counts: &mut Counts, had_error: &mut bool) {
    let stat = match vfs_lstat(path) {
        Ok(stat) => stat,
        Err(e) => {
            write_to_fd(2, &alloc::format!("tree: cannot stat '{}': {:?}\n", path, e));
            *had_error = true;
            return;
        }
    };

    write_to_fd(1, &alloc::format!("{}\n", path));
    if (stat.mode & S_IFMT) == S_IFDIR {
        walk_dir(path, "", 0, opts, counts, had_error);
    } else if !opts.dirs_only {
        counts.files += 1;
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    let opts = match parse_args(&args) {
        Ok(opts) => opts,
        Err(msg) => {
            write_to_fd(2, msg);
            usage();
            exit(1);
        }
    };

    let mut counts = Counts::default();
    let mut had_error = false;
    for (idx, path) in opts.paths.iter().enumerate() {
        if idx > 0 {
            write_to_fd(1, "\n");
        }
        print_tree(path, &opts, &mut counts, &mut had_error);
    }

    write_to_fd(1, &alloc::format!("\n{} directories, {} files\n", counts.dirs, counts.files));

    if had_error {
        exit(1)
    }
    exit(0)
}
