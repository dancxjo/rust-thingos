use super::*;

// ── read ────────────────────────────────────────────────────────────────────

pub fn sys_fs_read(fd: usize, buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    validate_user_range(buf_ptr, buf_len, true)?;
    if buf_len == 0 {
        return Ok(0);
    }

    // Clone the node Arc and the shared offset so we don't hold the process lock
    // during the read.
    let (node, offset_cell, status_flags) = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.handle_table.get(fd as u32)?;
        let status_flags = *file.status_flags.lock();
        if !status_flags.is_readable() {
            return Err(Errno::EBADF);
        }
        (file.node.clone(), file.offset.clone(), status_flags)
    };

    if status_flags.read_would_block(node.poll()) {
        return Err(Errno::EAGAIN);
    }

    let offset = *offset_cell.lock();
    let mut kbuf = vec![0u8; buf_len];
    let n = node.read(offset, &mut kbuf)?;

    if n > 0 {
        *offset_cell.lock() = offset.saturating_add(n as u64);
    }

    unsafe { copyout(buf_ptr, &kbuf[..n])? };
    Ok(n)
}

pub fn sys_fs_stat(fd: usize, stat_ptr: usize, _a2: usize, _a3: usize) -> SysResult<usize> {
    let stat_size = core::mem::size_of::<abi::fs::FileStat>();
    validate_user_range(stat_ptr, stat_size, true)?;

    let node = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.handle_table.get(fd as u32)?;
        file.node.clone()
    };

    let stat = node.stat()?;
    let file_stat = stat.to_abi_stat();
    // SAFETY: `file_stat` is a plain repr(C) struct on the stack; we read it as bytes.
    let bytes = unsafe {
        core::slice::from_raw_parts(&file_stat as *const abi::fs::FileStat as *const u8, stat_size)
    };
    unsafe { copyout(stat_ptr, bytes)? };

    Ok(0)
}

pub fn sys_fs_isatty(fd: usize) -> SysResult<usize> {
    let node = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.handle_table.get(fd as u32)?;
        file.node.clone()
    };
    Ok(if node.is_tty() { 1 } else { 0 })
}

pub fn sys_fs_readdir(fd: usize, buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    crate::ktrace!("sys_fs_readdir: fd={} len={}", fd, buf_len);
    validate_user_range(buf_ptr, buf_len, true)?;
    if buf_len == 0 {
        return Ok(0);
    }

    let (node, offset_cell, path) = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.handle_table.get(fd as u32)?;
        (file.node.clone(), file.offset.clone(), file.path.clone())
    };

    let mut kbuf = vec![0u8; buf_len];
    let offset: u64 = *offset_cell.lock();
    let is_mounts_phase = (offset & (1 << 63)) != 0;
    let mut n = 0;

    // 1. Natural entries phase
    if !is_mounts_phase {
        crate::ktrace!("sys_fs_readdir: natural phase begin offset={}", offset);
        n = node.readdir(offset, &mut kbuf)?;
        if n > 0 {
            *offset_cell.lock() = offset.saturating_add(n as u64);
        } else {
            // Reached EOF of natural entries. Switch to mounts phase.
            *offset_cell.lock() = 1 << 63;
        }
    }

    let current_offset = *offset_cell.lock();
    let is_mounts_phase_now = (current_offset & (1 << 63)) != 0;

    // 2. Mount points supplement phase
    // Only supplement if we are in the mounts phase AND we have room in the buffer
    if is_mounts_phase_now && n < buf_len {
        let mounts = crate::vfs::mount::get_mounts_under(&path);
        if !mounts.is_empty() {
            let mounts_offset = current_offset & !(1 << 63);
            let m_n = crate::vfs::write_readdir_entries(
                mounts.iter().map(|s| s.as_str()),
                mounts_offset,
                &mut kbuf[n..],
            )?;
            if m_n > 0 {
                *offset_cell.lock() = current_offset.saturating_add(m_n as u64);
                n += m_n;
            }
        }
    }

    if n > 0 {
        unsafe { copyout(buf_ptr, &kbuf[..n])? };
    }

    Ok(n)
}

// ── write ───────────────────────────────────────────────────────────────────

pub fn sys_fs_write(fd: usize, buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    let tid = unsafe { crate::sched::current_tid_current() };
    crate::ktrace!("sys_fs_write: tid={} fd={} len={}", tid, fd, buf_len);
    validate_user_range(buf_ptr, buf_len, false)?;
    if buf_len == 0 {
        return Ok(0);
    }

    let mut kbuf = vec![0u8; buf_len];
    unsafe {
        crate::kdebug!("sys_fs_write: tid={} fd={} starting copyin", tid, fd);
        copyin(&mut kbuf, buf_ptr)?;
        crate::kdebug!("sys_fs_write: tid={} fd={} copyin ok", tid, fd);
    };

    let (node, offset_cell, status_flags, mount_id) = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.handle_table.get(fd as u32)?;
        let status_flags = *file.status_flags.lock();
        if !status_flags.is_writable() {
            crate::kwarn!("VFS: sys_fs_write fd={} EBADF (not writable, path={:?})", fd, file.path);
            return Err(Errno::EBADF);
        }
        let mid = vfs::mount::mount_id_for_path_in_namespace(lock.namespace.id(), &file.path);
        (file.node.clone(), file.offset.clone(), status_flags, mid)
    };

    if status_flags.write_would_block(node.poll()) {
        return Err(Errno::EAGAIN);
    }

    let write_offset = status_flags.effective_write_offset(*offset_cell.lock(), node.stat()?.size);
    let n = node.write(write_offset, &kbuf)?;

    if n > 0 {
        *offset_cell.lock() = write_offset.saturating_add(n as u64);
        // Emit MODIFY event
        crate::vfs::watch::emit_event(&*node, abi::vfs_watch::mask::MODIFY, None, 0, mount_id);
    }

    Ok(n)
}

// ── readv ───────────────────────────────────────────────────────────────────

/// Scatter-gather read: read from `fd` into multiple buffers described by the
/// `iovec` array at `iovec_ptr` (count = `iovec_count`).
///
/// Each element of the array is `abi::syscall::IoVec { base: usize, len: usize }`.
/// Returns the total number of bytes read across all buffers.
pub fn sys_fs_readv(fd: usize, iovec_ptr: usize, iovec_count: usize) -> SysResult<usize> {
    use abi::syscall::IoVec;

    if iovec_count == 0 {
        return Ok(0);
    }
    if iovec_count > 1024 {
        return Err(Errno::EINVAL);
    }

    let iov_size = iovec_count.checked_mul(core::mem::size_of::<IoVec>()).ok_or(Errno::EINVAL)?;
    validate_user_range(iovec_ptr, iov_size, false)?;

    // Copy the iovec array from userspace.
    let mut iovecs = vec![IoVec::default(); iovec_count];
    unsafe {
        copyin(
            core::slice::from_raw_parts_mut(iovecs.as_mut_ptr() as *mut u8, iov_size),
            iovec_ptr,
        )?
    };

    let (node, offset_cell, status_flags) = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.handle_table.get(fd as u32)?;
        let status_flags = *file.status_flags.lock();
        if !status_flags.is_readable() {
            return Err(Errno::EBADF);
        }
        (file.node.clone(), file.offset.clone(), status_flags)
    };

    if status_flags.read_would_block(node.poll()) {
        return Err(Errno::EAGAIN);
    }

    let mut total = 0usize;
    for iov in &iovecs {
        if iov.len == 0 {
            continue;
        }
        validate_user_range(iov.base, iov.len, true)?;

        let offset = *offset_cell.lock();
        let mut kbuf = vec![0u8; iov.len];
        let n = node.read(offset, &mut kbuf)?;
        if n > 0 {
            *offset_cell.lock() = offset.saturating_add(n as u64);
            unsafe { copyout(iov.base, &kbuf[..n])? };
            total += n;
        }
        if n < iov.len {
            // Short read — stop filling further buffers.
            break;
        }
    }

    Ok(total)
}

// ── writev ──────────────────────────────────────────────────────────────────

/// Scatter-gather write: write to `fd` from multiple buffers described by the
/// `iovec` array at `iovec_ptr` (count = `iovec_count`).
///
/// Returns the total number of bytes written across all buffers.
pub fn sys_fs_writev(fd: usize, iovec_ptr: usize, iovec_count: usize) -> SysResult<usize> {
    use abi::syscall::IoVec;

    if iovec_count == 0 {
        return Ok(0);
    }
    if iovec_count > 1024 {
        return Err(Errno::EINVAL);
    }

    let iov_size = iovec_count.checked_mul(core::mem::size_of::<IoVec>()).ok_or(Errno::EINVAL)?;
    validate_user_range(iovec_ptr, iov_size, false)?;

    // Copy the iovec array from userspace.
    let mut iovecs = vec![IoVec::default(); iovec_count];
    unsafe {
        copyin(
            core::slice::from_raw_parts_mut(iovecs.as_mut_ptr() as *mut u8, iov_size),
            iovec_ptr,
        )?
    };

    let (node, offset_cell, status_flags, mount_id) = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.handle_table.get(fd as u32)?;
        let status_flags = *file.status_flags.lock();
        if !status_flags.is_writable() {
            return Err(Errno::EBADF);
        }
        let mid = vfs::mount::mount_id_for_path_in_namespace(lock.namespace.id(), &file.path);
        (file.node.clone(), file.offset.clone(), status_flags, mid)
    };

    if status_flags.write_would_block(node.poll()) {
        return Err(Errno::EAGAIN);
    }

    let mut total = 0usize;
    for iov in &iovecs {
        if iov.len == 0 {
            continue;
        }
        validate_user_range(iov.base, iov.len, false)?;

        let mut kbuf = vec![0u8; iov.len];
        unsafe { copyin(&mut kbuf, iov.base)? };

        let write_offset =
            status_flags.effective_write_offset(*offset_cell.lock(), node.stat()?.size);
        let n = node.write(write_offset, &kbuf)?;
        if n > 0 {
            *offset_cell.lock() = write_offset.saturating_add(n as u64);
            crate::vfs::watch::emit_event(&*node, abi::vfs_watch::mask::MODIFY, None, 0, mount_id);
            total += n;
        }
        if n < iov.len {
            // Short write — stop filling further buffers.
            break;
        }
    }

    Ok(total)
}
