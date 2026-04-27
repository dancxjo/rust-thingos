use super::*;

// ── open ────────────────────────────────────────────────────────────────────

pub fn sys_fs_open(path_ptr: usize, path_len: usize, flags: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }

    // Copy path from userspace.
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;

    let tid = unsafe { crate::sched::current_tid_current() };
    crate::ktrace!("VFS: sys_fs_open path='{}' tid={}", path, tid);

    if path == "/dev/fb0" {
        crate::kdebug!("sys_fs_open: path='{}' len={} flags=0x{:x}", path, path_len, flags);
    }

    let open_flags = OpenFlags::from_open_call(flags as u32);
    let want_creat = (flags as u32) & vfs_flags::O_CREAT != 0;
    let want_trunc = (flags as u32) & vfs_flags::O_TRUNC != 0;
    let want_excl = (flags as u32) & vfs_flags::O_EXCL != 0;

    let abs_path = resolve_path(path)?;

    // Resolve path through the mount table, creating the file if O_CREAT is set.
    let node = if want_creat {
        // Try lookup first; fall back to create if the file doesn't exist.
        match vfs::mount::lookup(&abs_path) {
            Ok(existing) => {
                if want_excl {
                    // O_CREAT | O_EXCL: file must not pre-exist.
                    return Err(Errno::EEXIST);
                }
                if want_trunc {
                    // Truncate the file to zero length.
                    let _ = existing.truncate(0);
                }
                existing
            }
            Err(Errno::ENOENT) => vfs::mount::create(&abs_path)?,
            Err(e) => return Err(e),
        }
    } else {
        vfs::mount::lookup(&abs_path)?
    };
    if path == "/sys/devices" {
        crate::kdebug!("sys_fs_open: /sys/devices lookup ok");
    }

    enforce_open_access(&node, open_flags)?;
    if path == "/sys/devices" {
        crate::kdebug!("sys_fs_open: /sys/devices access ok");
    }

    if path == "/dev/fb0" {
        match node.stat() {
            Ok(stat) => crate::kdebug!(
                "sys_fs_open: resolved node for /dev/fb0 mode=0o{:o} size={} ino={}",
                stat.mode,
                stat.size,
                stat.ino
            ),
            Err(err) => crate::kwarn!("sys_fs_open: resolved /dev/fb0 but stat failed: {:?}", err),
        }
    }

    // Insert into the per-process fd table.
    let pinfo_arc = match crate::sched::process_info_current() {
        Some(pinfo_arc) => pinfo_arc,
        None => {
            if path == "/dev/fb0" {
                let tid = unsafe { crate::sched::current_tid_current() };
                let direct = crate::sched::process_info_for_tid_current(tid).is_some();
                crate::kwarn!(
                    "sys_fs_open: no process info for /dev/fb0 current_tid={} direct_lookup={}",
                    tid,
                    direct
                );
            }
            return Err(Errno::ENOENT);
        }
    };
    if path == "/dev/fb0" {
        crate::kdebug!("sys_fs_open: process info present for /dev/fb0");
    }
    let fd = pinfo_arc.lock().handle_table.open(node, open_flags, abs_path)?;
    if path == "/sys/devices" {
        crate::kdebug!("sys_fs_open: /sys/devices fd={}", fd);
    }

    if path == "/dev/fb0" {
        crate::kdebug!("sys_fs_open: handle_table.open('/dev/fb0') -> {}", fd);
    }

    Ok(fd as usize)
}

// ── close ───────────────────────────────────────────────────────────────────

pub fn sys_fs_close(fd: usize) -> SysResult<usize> {
    crate::ktrace!("sys_fs_close: fd={}", fd);
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;

    // Before closing, check if this fd has an advisory lock and release it.
    // We must NOT call node.stat() while holding the process-info lock: for
    // provider-backed files, stat() triggers a synchronous RPC to the
    // userland provider.  If the provider is busy servicing another RPC
    // (e.g. a vfs_poll), the calling task would block indefinitely while
    // holding the spin-lock — deadlocking any other operation that needs
    // the same ProcessInfo.
    //
    // Optimisation: skip the (potentially expensive) stat entirely when the
    // process holds no advisory locks, which is the common case.
    let (maybe_node, pid) = {
        let lock = pinfo_arc.lock();
        let pid = lock.pid;
        let node = lock.handle_table.get(fd as u32).ok().map(|f| f.node.clone());
        (node, pid)
    }; // pinfo_arc lock released — safe to perform I/O now

    if let Some(node) = maybe_node {
        if crate::vfs::flock::process_has_locks(pid) {
            crate::ktrace!("sys_fs_close: fd={} checking flock", fd);
            if let Ok(s) = node.stat() {
                crate::vfs::flock::release(s.ino, pid);
            }
        }
    }

    let entry = pinfo_arc.lock().handle_table.take(fd as u32)?;
    crate::vfs::handle_table::close_open_handle(entry);
    Ok(0)
}

// ── sync (fsync) ─────────────────────────────────────────────────────────────

/// Flush the VFS node associated with `fd` to its backing store.
///
/// For RAM-backed filesystems this is a no-op that always succeeds.
/// Returns `Ok(0)` on success, or an errno on failure.
pub fn sys_fs_sync(fd: usize) -> SysResult<usize> {
    let node = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        lock.handle_table.get(fd as u32)?.node.clone()
    };
    node.sync()?;
    Ok(0)
}

/// Truncate the file associated with `fd` to exactly `size` bytes (ftruncate).
///
/// If `size` is greater than the current file length, the file is extended with
/// zero bytes.  If `size` is smaller, the excess data is discarded.
/// Returns `Ok(0)` on success, or an errno on failure.
pub fn sys_fs_ftruncate(fd: usize, size: usize) -> SysResult<usize> {
    let node = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        lock.handle_table.get(fd as u32)?.node.clone()
    };
    node.truncate(size as u64)?;
    Ok(0)
}

/// File-descriptor control.
pub fn sys_fs_fcntl(fd: usize, cmd: usize, arg: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let mut lock = pinfo_arc.lock();

    match cmd as u32 {
        fcntl_cmd::F_GETFD => Ok(lock.handle_table.get_handle_flags(fd as u32)? as usize),
        fcntl_cmd::F_SETFD => {
            lock.handle_table
                .set_handle_flags(fd as u32, (arg as u32) & handle_flags::HANDLE_CLOEXEC)?;
            Ok(0)
        }
        fcntl_cmd::F_GETFL => {
            let file = lock.handle_table.get(fd as u32)?;
            Ok(file.status_flags.lock().0 as usize)
        }
        fcntl_cmd::F_SETFL => {
            let file = lock.handle_table.get(fd as u32)?;
            let mut status_flags = file.status_flags.lock();
            *status_flags = status_flags.with_mutable_status(arg as u32);
            Ok(0)
        }
        _ => Err(Errno::EINVAL),
    }
}

// ── dup ─────────────────────────────────────────────────────────────────────

/// Duplicate `old_fd` to the lowest available file descriptor.
///
/// Returns the new file descriptor, or an error if `old_fd` is not open.
pub fn SYS_FS_DUP(old_fd: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let new_fd = pinfo_arc.lock().handle_table.dup(old_fd as u32)?;
    Ok(new_fd as usize)
}

// ── dup2 ────────────────────────────────────────────────────────────────────

/// Duplicate `old_fd` to `new_fd`.
///
/// If `new_fd` is already open it is closed first.  If `old_fd == new_fd`
/// this is a no-op.  Returns `new_fd` on success.
pub fn SYS_FS_DUP2(old_fd: usize, new_fd: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let result = pinfo_arc.lock().handle_table.dup2(old_fd as u32, new_fd as u32)?;
    Ok(result as usize)
}

// ── pipe ────────────────────────────────────────────────────────────────────

/// Create an anonymous pipe and allocate two file descriptors.
///
/// Writes the read-end fd and write-end fd into the user buffer pointed to by
/// `pipefd_ptr` (which must point to a `[u32; 2]`).  Returns 0 on success.
pub fn sys_pipe(pipefd_ptr: usize) -> SysResult<usize> {
    // Validate: we need to write 8 bytes (two u32s).
    validate_user_range(pipefd_ptr, 8, true)?;

    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;

    let (pipe_id, read_node, write_node) = crate::ipc::pipe::create_fd_pair_with_id(4096, false);
    let (read_fd, write_fd) = {
        let mut lock = pinfo_arc.lock();
        let read_fd = lock.handle_table.open(
            read_node,
            crate::vfs::OpenFlags::read_only(),
            alloc::format!("pipe:{}", pipe_id),
        )?;
        match lock.handle_table.open(
            write_node,
            crate::vfs::OpenFlags::write_only(),
            alloc::format!("pipe:{}", pipe_id),
        ) {
            Ok(wfd) => (read_fd, wfd),
            Err(e) => {
                let _ = lock.handle_table.close(read_fd);
                return Err(e);
            }
        }
    };

    // Write [read_fd, write_fd] to userspace as two consecutive u32 values (8 bytes).
    let read_fd_bytes = read_fd.to_ne_bytes();
    let write_fd_bytes = write_fd.to_ne_bytes();
    let mut fds_bytes = [0u8; 8];
    fds_bytes[..4].copy_from_slice(&read_fd_bytes);
    fds_bytes[4..].copy_from_slice(&write_fd_bytes);
    unsafe { copyout(pipefd_ptr, &fds_bytes)? };

    Ok(0)
}

/// Explicitly bridge an IPC handle into the VFS world as a file descriptor.
///
/// This allows standard `poll()` to be used across both files and ports.
pub fn sys_fd_from_handle(handle_val: usize) -> SysResult<usize> {
    let handle = crate::handle::bridge::Handle(handle_val as u32);
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    Ok(crate::handle::bridge::install_fd_compat_for_port_handle(&pinfo_arc, handle)? as usize)
}

// ── seek ────────────────────────────────────────────────────────────────────
pub fn sys_fs_seek(fd: usize, offset: usize, whence: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let mut lock = pinfo_arc.lock();
    let file = lock.handle_table.get_mut(fd as u32)?;

    let node = file.node.clone();
    let stat = node.stat()?;
    let size = stat.size;

    // The raw syscall argument carries the offset as pointer-sized bits.  On
    // 64-bit targets the bit representation of `i64` and `usize` are the same,
    // so reinterpreting as `i64` recovers the signed value for SEEK_CUR /
    // SEEK_END (which may receive negative offsets).
    let offset_signed = offset as i64;

    // Hold the offset lock across the read+compute+write to prevent lost-update
    // races when two threads share the same open-file description via dup/dup2.
    let mut off = file.offset.lock();
    let current_offset = *off;

    let new_offset: u64 = match whence {
        0 => {
            // SEEK_SET – absolute position; negative is invalid.
            if offset_signed < 0 {
                return Err(Errno::EINVAL);
            }
            offset as u64
        }
        1 => {
            // SEEK_CUR – relative to current position.
            let new = (current_offset as i64).checked_add(offset_signed).ok_or(Errno::EINVAL)?;
            if new < 0 {
                return Err(Errno::EINVAL);
            }
            new as u64
        }
        2 => {
            // SEEK_END – relative to end of file.
            let new = (size as i64).checked_add(offset_signed).ok_or(Errno::EINVAL)?;
            if new < 0 {
                return Err(Errno::EINVAL);
            }
            new as u64
        }
        _ => return Err(Errno::EINVAL),
    };

    *off = new_offset;
    Ok(new_offset as usize)
}

/// Advisory file lock or unlock for the file associated with `fd`.
///
/// `how` is a combination of [`abi::syscall::flock_flags`] constants:
/// * `LOCK_SH` (1) — acquire shared lock
/// * `LOCK_EX` (2) — acquire exclusive lock
/// * `LOCK_NB` (4) — non-blocking (return `EAGAIN` instead of blocking)
/// * `LOCK_UN` (8) — release any lock on the file
///
/// Returns `Ok(0)` on success.
pub fn sys_fs_flock(fd: usize, how: usize) -> SysResult<usize> {
    let (ino, pid) = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let pid = lock.pid;
        let node = lock.handle_table.get(fd as u32)?.node.clone();
        // Release the process lock before calling stat() to avoid deadlocks.
        drop(lock);
        (node.stat()?.ino, pid)
    };
    crate::vfs::flock::flock(ino, pid, how as u32)?;
    Ok(0)
}
