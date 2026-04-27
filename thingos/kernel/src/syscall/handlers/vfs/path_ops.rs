use super::*;

pub fn sys_fs_unlink(path_ptr: usize, path_len: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;

    let abs_path = resolve_path(path)?;
    let target = vfs::mount::lookup(&abs_path)?;
    if target.stat()?.is_dir() {
        return Err(Errno::EISDIR);
    }

    // Resolve parent to emit event
    let (parent_path, name) = split_parent(&abs_path);
    let parent_node = vfs::mount::lookup(parent_path).ok();
    let parent_mount_id = vfs::mount::mount_id_for_path(parent_path);

    vfs::mount::unlink(&abs_path)?;

    if let Some(parent) = parent_node {
        crate::vfs::watch::emit_event(
            &*parent,
            abi::vfs_watch::mask::REMOVE,
            Some(name),
            0,
            parent_mount_id,
        );
    }

    Ok(0)
}

/// Remove an empty directory at `path` (`rmdir` semantics).
pub fn sys_fs_rmdir(path_ptr: usize, path_len: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;

    let abs_path = resolve_path(path)?;
    let target = vfs::mount::lookup(&abs_path)?;
    if !target.stat()?.is_dir() {
        return Err(Errno::ENOTDIR);
    }

    // Resolve parent to emit event
    let (parent_path, name) = split_parent(&abs_path);
    let parent_node = vfs::mount::lookup(parent_path).ok();
    let parent_mount_id = vfs::mount::mount_id_for_path(parent_path);

    vfs::mount::unlink(&abs_path)?;

    if let Some(parent) = parent_node {
        crate::vfs::watch::emit_event(
            &*parent,
            abi::vfs_watch::mask::REMOVE,
            Some(name),
            0,
            parent_mount_id,
        );
    }

    Ok(0)
}

// ── mkdir ───────────────────────────────────────────────────────────────────

/// Create a directory at `path`.
pub fn sys_fs_mkdir(path_ptr: usize, path_len: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;

    let abs_path = resolve_path(path)?;

    // Resolve parent to emit event
    let (parent_path, name) = split_parent(&abs_path);
    let parent_node = vfs::mount::lookup(parent_path).ok();
    let parent_mount_id = vfs::mount::mount_id_for_path(parent_path);

    vfs::mount::mkdir(&abs_path)?;

    if let Some(parent) = parent_node {
        crate::vfs::watch::emit_event(
            &*parent,
            abi::vfs_watch::mask::CREATE,
            Some(name),
            0,
            parent_mount_id,
        );
    }

    Ok(0)
}

// ── rename ──────────────────────────────────────────────────────────────────

static NEXT_COOKIE: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(1);

pub fn sys_fs_rename(
    _old_dirfd: usize,
    old_const_ptr: usize,
    old_len: usize,
    _new_dirfd: usize,
    new_const_ptr: usize,
    new_len: usize,
) -> SysResult<usize> {
    validate_user_range(old_const_ptr, old_len, false)?;
    validate_user_range(new_const_ptr, new_len, false)?;

    let mut old_path_buf = vec![0u8; old_len];
    let mut new_path_buf = vec![0u8; new_len];
    unsafe {
        copyin(&mut old_path_buf, old_const_ptr)?;
        copyin(&mut new_path_buf, new_const_ptr)?;
    }
    let old_path = core::str::from_utf8(&old_path_buf).map_err(|_| Errno::EINVAL)?;
    let new_path = core::str::from_utf8(&new_path_buf).map_err(|_| Errno::EINVAL)?;

    let old_abs = resolve_path(old_path)?;
    let new_abs = resolve_path(new_path)?;

    // Resolve parents for events
    let (old_parent_path, old_name) = split_parent(&old_abs);
    let (new_parent_path, new_name) = split_parent(&new_abs);
    let old_parent_node = vfs::mount::lookup(old_parent_path).ok();
    let new_parent_node = vfs::mount::lookup(new_parent_path).ok();
    let old_parent_mount_id = vfs::mount::mount_id_for_path(old_parent_path);
    let new_parent_mount_id = vfs::mount::mount_id_for_path(new_parent_path);

    // Perform rename (VFS mount layer needs a rename method too, which redirects to driver)
    vfs::mount::rename(&old_abs, &new_abs)?;

    // Emit MOVE events
    let cookie = NEXT_COOKIE.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    if let Some(parent) = old_parent_node {
        crate::vfs::watch::emit_event(
            &*parent,
            abi::vfs_watch::mask::MOVE_FROM,
            Some(old_name),
            cookie,
            old_parent_mount_id,
        );
    }
    if let Some(parent) = new_parent_node {
        crate::vfs::watch::emit_event(
            &*parent,
            abi::vfs_watch::mask::MOVE_TO,
            Some(new_name),
            cookie,
            new_parent_mount_id,
        );
    }

    Ok(0)
}

// ── lstat ────────────────────────────────────────────────────────────────────

/// Stat a path without following the final symlink (`lstat` semantics).
///
/// Signature: `SYS_FS_LSTAT(path_ptr, path_len, stat_ptr) → 0`
///
/// - Resolves `path` without following the final path component if it is a
///   symlink, so symlink metadata (mode `S_IFLNK`, size = target length, etc.)
///   is returned instead of the target's metadata.
/// - Returns `ENOENT` if the path does not exist.
pub fn sys_fs_lstat(path_ptr: usize, path_len: usize, stat_ptr: usize) -> SysResult<usize> {
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    let stat_size = core::mem::size_of::<abi::fs::FileStat>();
    validate_user_range(path_ptr, path_len, false)?;
    validate_user_range(stat_ptr, stat_size, true)?;

    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;

    let abs_path = resolve_path(path)?;

    // Use no-follow lookup so the symlink node itself is returned.
    let node = vfs::path::resolve_no_follow(&abs_path)?;

    let stat = node.stat()?;
    let file_stat = stat.to_abi_stat();
    // SAFETY: `file_stat` is a plain repr(C) struct on the stack; we read it as bytes.
    let bytes = unsafe {
        core::slice::from_raw_parts(&file_stat as *const abi::fs::FileStat as *const u8, stat_size)
    };
    unsafe { copyout(stat_ptr, bytes)? };

    Ok(0)
}

// ── symlink ──────────────────────────────────────────────────────────────────
///
/// Signature: `SYS_FS_SYMLINK(target_ptr, target_len, link_ptr, link_len) → 0`
///
/// - `target` is the path the symlink will point to (not validated for existence).
/// - `link_path` is the path at which the symlink entry is created.
/// - Returns `Ok(0)` on success.
/// - Returns `ENOENT` if the parent directory of `link_path` does not exist.
/// - Returns `EEXIST` if an entry already exists at `link_path`.
/// - Returns `EROFS` if the underlying filesystem is read-only.
pub fn sys_fs_symlink(
    target_ptr: usize,
    target_len: usize,
    link_ptr: usize,
    link_len: usize,
) -> SysResult<usize> {
    if target_len == 0 || target_len > 4096 {
        return Err(Errno::EINVAL);
    }
    if link_len == 0 || link_len > 4096 {
        return Err(Errno::EINVAL);
    }
    validate_user_range(target_ptr, target_len, false)?;
    validate_user_range(link_ptr, link_len, false)?;

    let mut target_buf = vec![0u8; target_len];
    let mut link_buf = vec![0u8; link_len];
    unsafe {
        copyin(&mut target_buf, target_ptr)?;
        copyin(&mut link_buf, link_ptr)?;
    }
    let target = core::str::from_utf8(&target_buf).map_err(|_| Errno::EINVAL)?;
    let link_path = core::str::from_utf8(&link_buf).map_err(|_| Errno::EINVAL)?;

    let abs_link = resolve_path(link_path)?;

    vfs::mount::symlink(target, &abs_link)?;
    Ok(0)
}

// ── link (hard link) ─────────────────────────────────────────────────────────

/// Create a hard link at `dst` that refers to the same inode as `src`.
///
/// Signature: `SYS_FS_LINK(src_ptr, src_len, dst_ptr, dst_len) → 0`
///
/// - `src` must be an existing regular file (hard-linking directories is not
///   supported).
/// - `dst` is the path at which the new directory entry is created.
/// - Returns `Ok(0)` on success.
/// - Returns `ENOENT` if `src` does not exist or the parent of `dst` does not exist.
/// - Returns `EEXIST` if an entry already exists at `dst`.
/// - Returns `EPERM` if `src` is a directory or symlink.
/// - Returns `EXDEV` if `src` and `dst` are on different mount points.
/// - Returns `EOPNOTSUPP` if the underlying filesystem does not support hard links.
pub fn sys_fs_link(
    src_ptr: usize,
    src_len: usize,
    dst_ptr: usize,
    dst_len: usize,
) -> SysResult<usize> {
    if src_len == 0 || src_len > 4096 {
        return Err(Errno::EINVAL);
    }
    if dst_len == 0 || dst_len > 4096 {
        return Err(Errno::EINVAL);
    }
    validate_user_range(src_ptr, src_len, false)?;
    validate_user_range(dst_ptr, dst_len, false)?;

    let mut src_buf = vec![0u8; src_len];
    let mut dst_buf = vec![0u8; dst_len];
    unsafe {
        copyin(&mut src_buf, src_ptr)?;
        copyin(&mut dst_buf, dst_ptr)?;
    }
    let src = core::str::from_utf8(&src_buf).map_err(|_| Errno::EINVAL)?;
    let dst = core::str::from_utf8(&dst_buf).map_err(|_| Errno::EINVAL)?;

    let abs_src = resolve_path(src)?;
    let abs_dst = resolve_path(dst)?;

    vfs::mount::link(&abs_src, &abs_dst)?;
    Ok(0)
}

// ── readlink ─────────────────────────────────────────────────────────────────

/// Read the target of the symbolic link at `path`.
///
/// Signature: `SYS_FS_READLINK(path_ptr, path_len, buf_ptr, buf_len) → len`
///
/// - Writes the symlink target (without a NUL terminator) into the caller
///   buffer.  The actual target bytes written is `min(target_len, buf_len)`.
/// - Returns the number of bytes in the symlink target (which may be larger
///   than `buf_len` if the buffer was too small).
/// - Returns `EINVAL` if the path does not refer to a symlink.
/// - Returns `ENOENT` if the path does not exist.
pub fn sys_fs_readlink(
    path_ptr: usize,
    path_len: usize,
    buf_ptr: usize,
    buf_len: usize,
) -> SysResult<usize> {
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    validate_user_range(path_ptr, path_len, false)?;

    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;

    let abs_path = resolve_path(path)?;

    // Use no-follow lookup so we get the symlink node itself.
    let node = vfs::path::resolve_no_follow(&abs_path)?;
    let target = node.readlink()?;

    let target_bytes = target.as_bytes();
    let needed = target_bytes.len();

    if buf_ptr != 0 && buf_len > 0 {
        let copy_len = buf_len.min(needed);
        validate_user_range(buf_ptr, copy_len, true)?;
        unsafe { copyout(buf_ptr, &target_bytes[..copy_len])? };
    }

    Ok(needed)
}

// ── chmod ────────────────────────────────────────────────────────────────────

/// Set the permission bits for the file at `path` (path-based chmod).
///
/// `mode` contains the lower 12 bits of the POSIX permission mask
/// (`0o7777`); the file-type bits are ignored.
/// Returns `Ok(0)` on success, or an errno on failure.
pub fn sys_fs_chmod(path_ptr: usize, path_len: usize, mode: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;
    let abs_path = resolve_path(path)?;
    let node = vfs::mount::lookup(&abs_path)?;
    node.chmod((mode as u32) & 0o7777)?;
    Ok(0)
}

/// Set the permission bits for the file associated with `fd` (fd-based fchmod).
///
/// `mode` contains the lower 12 bits of the POSIX permission mask
/// (`0o7777`); the file-type bits are ignored.
/// Returns `Ok(0)` on success, or an errno on failure.
pub fn sys_fs_fchmod(fd: usize, mode: usize) -> SysResult<usize> {
    let node = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        lock.handle_table.get(fd as u32)?.node.clone()
    };
    node.chmod((mode as u32) & 0o7777)?;
    Ok(0)
}

// ── utimes helpers ───────────────────────────────────────────────────────────

/// Copy a [`abi::fs::UtimesRequest`] from userspace and decode it into
/// optional `(sec, nsec)` pairs.
fn read_utimes_request(times_ptr: usize) -> SysResult<(Option<(u64, u32)>, Option<(u64, u32)>)> {
    use abi::fs::UtimesRequest;
    let size = core::mem::size_of::<UtimesRequest>();
    validate_user_range(times_ptr, size, false)?;
    let mut req = UtimesRequest::default();
    let buf =
        unsafe { core::slice::from_raw_parts_mut(&mut req as *mut UtimesRequest as *mut u8, size) };
    unsafe { copyin(buf, times_ptr)? };
    let atime = if req.atime_sec == UtimesRequest::OMIT {
        None
    } else {
        Some((req.atime_sec, req.atime_nsec))
    };
    let mtime = if req.mtime_sec == UtimesRequest::OMIT {
        None
    } else {
        Some((req.mtime_sec, req.mtime_nsec))
    };
    Ok((atime, mtime))
}

// ── utimes ───────────────────────────────────────────────────────────────────

/// Set the access and modification timestamps for the file at `path`.
///
/// `times_ptr` points to an [`abi::fs::UtimesRequest`] struct.
/// `flags`: bit 0 = `AT_SYMLINK_NOFOLLOW` — if set, operate on a symlink
/// node itself rather than its target (reserved; currently returns `ENOTSUP`
/// if a symlink is found at the target path and this flag is set).
/// Returns `Ok(0)` on success, or an errno on failure.
pub fn sys_fs_utimes(
    path_ptr: usize,
    path_len: usize,
    times_ptr: usize,
    _flags: usize,
) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;
    let (atime, mtime) = read_utimes_request(times_ptr)?;
    let abs_path = resolve_path(path)?;
    let node = vfs::mount::lookup(&abs_path)?;
    node.utimes(atime, mtime)?;
    Ok(0)
}

/// Set the access and modification timestamps for the file at `path` without
/// following symlinks (lutimes).
///
/// `times_ptr` points to an [`abi::fs::UtimesRequest`] struct.
/// Returns `Ok(0)` on success, or an errno on failure.
pub fn sys_fs_lutimes(path_ptr: usize, path_len: usize, times_ptr: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;
    let (atime, mtime) = read_utimes_request(times_ptr)?;
    let abs_path = resolve_path(path)?;

    // Use no-follow lookup so the symlink node itself is returned.
    let node = vfs::path::resolve_no_follow(&abs_path)?;
    node.utimes(atime, mtime)?;
    Ok(0)
}

/// Set the access and modification timestamps for the file associated with `fd`.
///
/// `times_ptr` points to an [`abi::fs::UtimesRequest`] struct.
/// Returns `Ok(0)` on success, or an errno on failure.
pub fn sys_fs_futimes(fd: usize, times_ptr: usize) -> SysResult<usize> {
    let (atime, mtime) = read_utimes_request(times_ptr)?;
    let node = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        lock.handle_table.get(fd as u32)?.node.clone()
    };
    node.utimes(atime, mtime)?;
    Ok(0)
}
