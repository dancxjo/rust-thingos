//! VFS syscall handlers: open, close, read, write, stat, readdir,
//!                       unlink, mkdir, dup, dup2, pipe, poll.
//!
//! These handlers implement the thingos VFS syscall interface:
//!
//! - [`sys_fs_open`]   — open (or create) a path, return a file descriptor
//! - [`sys_fs_close`]  — release a file descriptor (all fds, including 0-2)
//! - [`sys_fs_read`]   — read from a file descriptor into a user buffer
//! - [`sys_fs_write`]  — write from a user buffer to a file descriptor
//! - [`sys_fs_unlink`] — remove a file or empty directory
//! - [`sys_fs_mkdir`]  — create a directory
//! - [`SYS_FS_DUP`]        — duplicate a file descriptor to the lowest free slot
//! - [`SYS_FS_DUP2`]       — duplicate a file descriptor to a specific slot
//! - [`sys_pipe`]       — create an anonymous pipe, allocating two fds
//! - [`sys_fs_poll`]   — poll a set of fds for readiness (POSIX-style)

use alloc::sync::Arc;
use alloc::vec;

use abi::errors::{Errno, SysResult};
use abi::syscall::{PollHandle, fcntl_cmd, handle_flags, poll_flags, vfs_flags};

use crate::syscall::validate::{copyin, copyout, validate_user_range};
use crate::vfs::{self, OpenFlags};

const MODE_READ_ANY: u32 = 0o444;
const MODE_WRITE_ANY: u32 = 0o222;

/// Returns whether `mode` permits the requested read/write access.
///
/// Transitional behavior: this currently checks whether any owner/group/other
/// class bit grants the requested access.
fn mode_allows_requested_access(mode: u32, want_read: bool, want_write: bool) -> bool {
    // Transitional coarse gate: enforce requested read/write against any
    // corresponding permission class bit. Caller-vs-owner/group class matching
    // is deferred until full uid/gid ownership propagation is in place.
    // This means if any class bit grants the requested access, the open is
    // currently allowed.
    let read_ok = !want_read || (mode & MODE_READ_ANY) != 0;
    let write_ok = !want_write || (mode & MODE_WRITE_ANY) != 0;
    read_ok && write_ok
}

/// Transitional VFS open-time access gate.
///
/// Non-root callers are denied when node mode bits do not permit the requested
/// read/write access. This currently uses coarse mode-bit checks; owner/group
/// class matching is deferred until uid/gid ownership propagation is complete.
fn enforce_open_access(node: &Arc<dyn vfs::VfsNode>, open_flags: OpenFlags) -> SysResult<()> {
    // Open requests with no read/write access mode do not perform data access
    // and remain allowed.
    if !open_flags.is_readable() && !open_flags.is_writable() {
        return Ok(());
    }

    // Root remains the privileged principal and bypasses file mode checks.
    if crate::sched::process_info_current().map(|p| p.lock().authority.uid).unwrap_or(0) == 0 {
        return Ok(());
    }

    let authority = crate::authority::bridge::authority_for_current();
    let stat = node.stat()?;
    if mode_allows_requested_access(stat.mode, open_flags.is_readable(), open_flags.is_writable()) {
        Ok(())
    } else {
        Err(Errno::EACCES)
    }
}

mod attrs;
mod descriptors;
mod io;
mod namespace;
mod path_ops;
mod poll;
#[cfg(test)]
mod tests;

pub use attrs::*;
pub use descriptors::*;
pub use io::*;
pub use namespace::*;
pub use path_ops::*;
pub use poll::*;

// ── Helpers ──────────────────────────────────────────────────────────────────

fn split_parent(path: &str) -> (&str, &str) {
    let trimmed = path.trim_end_matches('/');
    match trimmed.rfind('/') {
        Some(0) => ("/", &trimmed[1..]),
        Some(idx) => (&trimmed[..idx], &trimmed[idx + 1..]),
        None => ("/", trimmed),
    }
}

fn require_namespace_mount_privilege() -> SysResult<()> {
    if let Some(pinfo) = crate::sched::process_info_current() {
        if pinfo.lock().namespace.is_isolated() {
            return Ok(());
        }
    }

    let authority = crate::authority::bridge::authority_for_current();
    crate::authority::bridge::check_privilege(&authority, "mount")
}

pub fn resolve_path(path: &str) -> SysResult<alloc::string::String> {
    let abs = if path.starts_with('/') {
        alloc::string::String::from(path)
    } else {
        let pinfo = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let cwd = pinfo.lock().cwd.clone();

        if cwd.ends_with('/') {
            alloc::format!("{}{}", cwd, path)
        } else {
            alloc::format!("{}/{}", cwd, path)
        }
    };

    // Ensure all paths are canonical (handle . and ..)
    vfs::path::normalise(&abs)
}
