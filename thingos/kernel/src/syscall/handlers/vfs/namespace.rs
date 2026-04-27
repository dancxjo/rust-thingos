use super::*;

// ── mount ───────────────────────────────────────────────────────────────────

/// Mount a userland VFS provider at the given path prefix.
///
/// `provider_write_handle` is the *write* end of a port pair owned by the
/// calling process.  The kernel will send VFS RPC messages (see
/// [`abi::vfs_rpc`]) to that port whenever a path under `path` is accessed.
///
/// The kernel creates a private response port and registers its write-handle
/// in the global handle table so the provider can call `SYS_port_send` to
/// deliver replies.
pub fn sys_fs_mount(
    provider_write_handle: usize,
    path_ptr: usize,
    path_len: usize,
) -> SysResult<usize> {
    sys_fs_mount_ex(provider_write_handle, path_ptr, path_len, abi::syscall::mount_flags::MREPL)
}

/// Mount a userland VFS provider at the given path prefix, with extended Plan 9 style flags.
pub fn sys_fs_mount_ex(
    provider_write_handle: usize,
    path_ptr: usize,
    path_len: usize,
    flags: u32,
) -> SysResult<usize> {
    require_namespace_mount_privilege()?;

    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }

    // Copy path from userspace.
    let mut path_buf = alloc::vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;
    let abs_path = resolve_path(path)?;

    // Resolve the provider's write handle to a port Arc.
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;

    // Attempt 1: Check if it's a VFS FD pointing to a PortNode
    let req_port = crate::handle::bridge::resolve_write_port_compat(
        &pinfo_arc,
        crate::handle::bridge::Handle(provider_write_handle as u32),
    )?;

    // Create the kernel response port.
    // Large capacity to hold multiple concurrent responses (though we serialise
    // requests, responses may vary in size).
    let resp_port_id = crate::ipc::create_port(abi::vfs_rpc::VFS_RPC_MAX_RESP * 4);
    let resp_port = crate::ipc::get_port(resp_port_id).ok_or(Errno::ENOMEM)?;

    // Identify the provider process (the primary reader of the request port).
    let provider_pid = req_port.primary_reader_pid();
    let provider_pinfo = if provider_pid != 0 {
        crate::sched::process_info_for_pid_current(provider_pid as u32)
    } else {
        None
    };

    // Register the write end of the response port.
    // We prefer to inject this into the provider process's own handle table
    // so it can call SYS_port_send on it directly.
    let resp_write_handle = if let Some(provider_arc) = provider_pinfo {
        let mut lock = provider_arc.lock();
        lock.ipc_table
            .alloc(resp_port.clone(), crate::ipc::IpcHandleMode::Write)
            .ok_or(Errno::ENOMEM)?
    } else {
        // Fallback: use caller's table (legacy/compatibility)
        let mut lock = pinfo_arc.lock();
        lock.ipc_table
            .alloc(resp_port.clone(), crate::ipc::IpcHandleMode::Write)
            .ok_or(Errno::ENOMEM)?
    };

    let req_port_id = crate::ipc::find_port_id(&req_port).ok_or(Errno::EBADF)?;

    // Build and mount the provider filesystem.
    let provider_fs = vfs::provider::ProviderFs::new(
        req_port.clone(),
        resp_port,
        resp_write_handle.0,
        req_port_id.0,
    );

    let driver: Arc<dyn vfs::VfsDriver> = if (flags & abi::syscall::mount_flags::MCOR) != 0 {
        Arc::new(vfs::overlay::OverlayFs::new(provider_fs as Arc<dyn vfs::VfsDriver>))
    } else {
        provider_fs as Arc<dyn vfs::VfsDriver>
    };

    vfs::mount::mount(&abs_path, driver, flags);

    crate::kdebug!(
        "vfs: mounted userland provider at {} (flags: {:#x}) port={:p}",
        abs_path,
        flags,
        Arc::as_ptr(&req_port)
    );
    Ok(0)
}

/// Bind an existing path in the namespace to a new path (Plan 9 style bind).
pub fn sys_fs_bind(
    src_ptr: usize,
    src_len: usize,
    dst_ptr: usize,
    dst_len: usize,
    flags: u32,
) -> SysResult<usize> {
    require_namespace_mount_privilege()?;

    validate_user_range(src_ptr, src_len, false)?;
    validate_user_range(dst_ptr, dst_len, false)?;
    if src_len == 0 || src_len > 4096 || dst_len == 0 || dst_len > 4096 {
        return Err(Errno::EINVAL);
    }

    let mut src_buf = alloc::vec![0u8; src_len];
    unsafe { copyin(&mut src_buf, src_ptr)? };
    let src_str = core::str::from_utf8(&src_buf).map_err(|_| Errno::EINVAL)?;
    let src_path = resolve_path(src_str)?;

    let mut dst_buf = alloc::vec![0u8; dst_len];
    unsafe { copyin(&mut dst_buf, dst_ptr)? };
    let dst_str = core::str::from_utf8(&dst_buf).map_err(|_| Errno::EINVAL)?;
    let dst_path = resolve_path(dst_str)?;

    let mut driver = vfs::mount::get_driver_for_path(&src_path)?;

    if (flags & abi::syscall::mount_flags::MCOR) != 0 {
        driver = Arc::new(vfs::overlay::OverlayFs::new(driver));
    }

    vfs::mount::mount(&dst_path, driver, flags);

    crate::kdebug!("vfs: bound {} to {} (flags: {:#x})", src_path, dst_path, flags);
    Ok(0)
}

// ── umount ──────────────────────────────────────────────────────────────────

/// Unmount the VFS provider at the given path prefix.
pub fn sys_fs_umount(path_ptr: usize, path_len: usize) -> SysResult<usize> {
    require_namespace_mount_privilege()?;

    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;
    let abs_path = resolve_path(path)?;
    vfs::mount::umount(&abs_path)?;
    crate::kdebug!("vfs: unmounted userland provider at {}", abs_path);
    Ok(0)
}

pub fn sys_fs_chdir(path_ptr: usize, path_len: usize) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;

    let abs_path = resolve_path(path)?;

    // Verify it exists and is a directory
    let node = vfs::mount::lookup(&abs_path)?;
    let stat = node.stat()?;
    if !stat.is_dir() {
        return Err(Errno::ENOTDIR);
    }

    // Normalise is now handled by resolve_path
    let pinfo = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    pinfo.lock().cwd = abs_path;

    Ok(0)
}

pub fn sys_fs_getcwd(buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    let pinfo = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let cwd = pinfo.lock().cwd.clone();

    let needed = cwd.len();
    if buf_ptr != 0 && buf_len > 0 {
        let copy_len = buf_len.min(needed);
        validate_user_range(buf_ptr, copy_len, true)?;
        unsafe { copyout(buf_ptr, &cwd.as_bytes()[..copy_len])? };
    }
    Ok(needed)
}

/// Resolve `path` (relative or absolute) to its canonical absolute form,
/// writing the result into the caller-supplied buffer.
///
/// Signature: `SYS_FS_REALPATH(path_ptr, path_len, buf_ptr, buf_len) → len`
///
/// - On success returns the length of the canonical path (excluding NUL).
/// - If `buf_len` is smaller than the canonical path length, the output
///   buffer is not written; the needed length is still returned so the
///   caller can retry with a suitably sized buffer.
/// - `buf_ptr` may be `0` (null) to query the required size without
///   writing any output; `buf_len` is ignored in that case.
/// - Returns `EINVAL` if `path` is empty or too long.
/// - Returns `ENOENT` if no process context is available (relative path + no cwd).
pub fn sys_fs_realpath(
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

    let canonical = resolve_path(path)?;
    let canonical_bytes = canonical.as_bytes();
    let needed = canonical_bytes.len();

    if buf_ptr != 0 && buf_len >= needed {
        validate_user_range(buf_ptr, needed, true)?;
        unsafe { copyout(buf_ptr, canonical_bytes)? };
    }

    Ok(needed)
}
