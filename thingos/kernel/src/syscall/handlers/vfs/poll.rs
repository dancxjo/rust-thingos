use super::*;

// ── notify ──────────────────────────────────────────────────────────────────

/// Notify the kernel that a provider-backed node is ready.
///
/// `req_handle` is the handle to the request port of the provider.
pub fn sys_fs_notify(req_handle: usize, node_handle: usize, revents: usize) -> SysResult<usize> {
    let handle = crate::ipc::IpcHandle(req_handle as u32);
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let entry = {
        let lock = pinfo_arc.lock();
        lock.ipc_table.get_any(handle).cloned().ok_or(Errno::EBADF)?
    };

    let port_id = crate::ipc::find_port_id(&entry.port).ok_or(Errno::EBADF)?;
    vfs::provider::notify_by_port(port_id.0, node_handle as u64, revents as u16)?;
    Ok(0)
}

// ── poll ────────────────────────────────────────────────────────────────────

/// POSIX-style poll over VFS file descriptors.
///
/// Examines each entry in the `pollfds` array and sets `revents` on those
/// that are ready.  Uses a three-phase no-lost-wakeup algorithm:
///
/// 1. **Probe** — call `VfsNode::poll()` on every entry.  If anything is
///    ready or `timeout_ms == 0`, copy `revents` back and return immediately.
/// 2. **Register** — call `VfsNode::add_waiter(tid)` on every node, then
///    optionally arm a scheduler timeout.
/// 3. **Re-probe** — repeat the probe after registration.  If still nothing
///    is ready, call `block_current_erased()` to park the calling task until
///    a node wakes it or the timeout expires.
///
/// # Arguments
/// - `pollfds_ptr` — pointer to a `[PollHandle; nfds]` in user memory (read/write)
/// - `nfds`         — number of entries in the array (max 256)
/// - `timeout_ms`   — `0` = non-blocking; `usize::MAX` = block indefinitely;
///                    any other value = maximum wait in milliseconds
///
/// # Returns
/// The number of entries with non-zero `revents`, or an errno on error.
/// Returns `Ok(0)` when the timeout expires with no events.
pub fn sys_fs_poll(pollfds_ptr: usize, nfds: usize, timeout_ms: usize) -> SysResult<usize> {
    const MAX_POLLFDS: usize = 256;
    if nfds == 0 {
        return Ok(0);
    }
    if nfds > MAX_POLLFDS {
        return Err(Errno::EINVAL);
    }

    let byte_len = nfds * core::mem::size_of::<PollHandle>();
    validate_user_range(pollfds_ptr, byte_len, true)?;

    // Copy all PollHandle entries from userspace.
    let mut kfds = vec![PollHandle::default(); nfds];
    unsafe {
        copyin(
            core::slice::from_raw_parts_mut(kfds.as_mut_ptr() as *mut u8, byte_len),
            pollfds_ptr,
        )?
    };

    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let tid = unsafe { crate::sched::current_tid_current() };

    // Resolve all entries once to avoid repeated FD table locking in the loop.
    #[derive(Clone)]
    struct Entry {
        node: Option<Arc<dyn vfs::VfsNode>>,
        events: u16,
    }
    let mut entries = vec![Entry { node: None, events: 0 }; nfds];
    {
        let lock = pinfo_arc.lock();
        for (i, kfd) in kfds.iter().enumerate() {
            if kfd.handle >= 0 {
                entries[i].node =
                    lock.handle_table.get(kfd.handle as u32).ok().map(|f| f.node.clone());
                entries[i].events = kfd.events;
            }
        }
    }

    let deadline = if timeout_ms == usize::MAX {
        None
    } else {
        let ticks = (timeout_ms as u64).saturating_add(9) / 10;
        Some(crate::sched::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed) + ticks)
    };

    loop {
        if crate::sched::take_pending_interrupt_current() {
            unsafe {
                copyout(
                    pollfds_ptr,
                    core::slice::from_raw_parts(kfds.as_ptr() as *const u8, byte_len),
                )?
            };
            return Err(Errno::EINTR);
        }

        // Pass 1: Probe current state
        let mut ready_count = 0;
        for (i, entry) in entries.iter().enumerate() {
            let revents = if let Some(ref node) = entry.node {
                node.poll() & (entry.events | poll_flags::POLLERR | poll_flags::POLLHUP)
            } else if kfds[i].handle >= 0 {
                poll_flags::POLLNVAL
            } else {
                0
            };

            kfds[i].revents = revents;
            if revents != 0 {
                ready_count += 1;
            }
        }

        // Immediate return if something is ready or if it's a non-blocking poll.
        if ready_count > 0 || timeout_ms == 0 {
            unsafe {
                copyout(
                    pollfds_ptr,
                    core::slice::from_raw_parts(kfds.as_ptr() as *const u8, byte_len),
                )?
            };
            return Ok(ready_count);
        }

        // Check for timeout.
        if let Some(d) = deadline {
            if crate::sched::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed) >= d {
                unsafe {
                    copyout(
                        pollfds_ptr,
                        core::slice::from_raw_parts(kfds.as_ptr() as *const u8, byte_len),
                    )?
                };
                return Ok(0);
            }
        }

        // Pass 2: Register as waiter on all nodes.
        for entry in entries.iter() {
            if let Some(ref node) = entry.node {
                node.add_waiter(tid);
            }
        }

        if let Some(d) = deadline {
            crate::sched::register_timeout_wake_current(tid, d);
        }

        // Pass 3: Re-probe after registration to avoid the missed-wakeup race.
        let mut ready_count = 0;
        for (i, entry) in entries.iter().enumerate() {
            let revents = if let Some(ref node) = entry.node {
                node.poll() & (entry.events | poll_flags::POLLERR | poll_flags::POLLHUP)
            } else if kfds[i].handle >= 0 {
                poll_flags::POLLNVAL
            } else {
                0
            };

            kfds[i].revents = revents;
            if revents != 0 {
                ready_count += 1;
            }
        }

        if ready_count > 0
            || (deadline.is_some()
                && crate::sched::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed)
                    >= deadline.unwrap())
        {
            for entry in entries.iter() {
                if let Some(ref node) = entry.node {
                    node.remove_waiter(tid);
                }
            }
            if deadline.is_some() {
                crate::sched::unregister_timeout_wake_current(tid);
            }
            unsafe {
                copyout(
                    pollfds_ptr,
                    core::slice::from_raw_parts(kfds.as_ptr() as *const u8, byte_len),
                )?
            };
            return Ok(ready_count);
        }

        // Wait for an event.
        unsafe { crate::sched::block_current_erased() };

        // Pass 4: Unregister waiters and repeat.
        for entry in entries.iter() {
            if let Some(ref node) = entry.node {
                node.remove_waiter(tid);
            }
        }
        if deadline.is_some() {
            crate::sched::unregister_timeout_wake_current(tid);
        }

        if crate::sched::take_pending_interrupt_current() {
            unsafe {
                copyout(
                    pollfds_ptr,
                    core::slice::from_raw_parts(kfds.as_ptr() as *const u8, byte_len),
                )?
            };
            return Err(Errno::EINTR);
        }
    }
}

// ── watch ───────────────────────────────────────────────────────────────────

pub fn sys_watch_fd(fd: usize, mask: usize, flags: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;

    let (node, mount_id) = {
        let lock = pinfo_arc.lock();
        let file = lock.handle_table.get(fd as u32)?;
        let mid = vfs::mount::mount_id_for_path_in_namespace(lock.namespace.id(), &file.path);
        (file.node.clone(), mid)
    };

    let watch = Arc::new(crate::vfs::watch::Watch::new(mask as u32, flags as u32));
    crate::vfs::watch::register_watch(&node, watch.clone(), mount_id)?;

    // Return the watch as a new file descriptor
    let watch_fd = pinfo_arc.lock().handle_table.open(
        watch,
        crate::vfs::OpenFlags::read_only(),
        "watch:fd".into(),
    )?;
    Ok(watch_fd as usize)
}

pub fn sys_watch_path(
    path_ptr: usize,
    path_len: usize,
    mask: usize,
    flags: usize,
) -> SysResult<usize> {
    validate_user_range(path_ptr, path_len, false)?;
    let mut path_buf = vec![0u8; path_len];
    unsafe { copyin(&mut path_buf, path_ptr)? };
    let path = core::str::from_utf8(&path_buf).map_err(|_| Errno::EINVAL)?;

    let abs_path = resolve_path(path)?;
    let node = vfs::mount::lookup(&abs_path)?;
    let mount_id = vfs::mount::mount_id_for_path(&abs_path);

    let watch = Arc::new(crate::vfs::watch::Watch::new(mask as u32, flags as u32));
    crate::vfs::watch::register_watch(&node, watch.clone(), mount_id)?;

    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let watch_fd = pinfo_arc.lock().handle_table.open(
        watch,
        crate::vfs::OpenFlags::read_only(),
        alloc::format!("watch:{}", abs_path),
    )?;
    Ok(watch_fd as usize)
}
