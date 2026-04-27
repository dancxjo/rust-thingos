use alloc::sync::Arc;
use core::mem::size_of;

use abi::errors::{Errno, SysResult};
use abi::wait::{self, WaitKind, WaitResult, WaitSpec};

use crate::syscall::validate::validate_user_range;

#[derive(Clone)]
enum Registration {
    Fd(Arc<dyn crate::vfs::VfsNode>),
    TaskExit(u64),
}

pub fn sys_wait_many(
    specs_ptr: usize,
    spec_count: usize,
    results_ptr: usize,
    results_cap: usize,
    timeout_ns: u64,
) -> SysResult<usize> {
    let tid = unsafe { crate::sched::current_tid_current() };
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ESRCH)?;

    // Debug: Check for GS corruption early
    {
        let cpu_idx = crate::runtime_base().current_cpu_index();
        let real_cpu_id = crate::runtime_base().current_cpu_id().0 as usize;
        if cpu_idx != real_cpu_id {
            panic!(
                "GS CORRUPTION DETECTED in sys_wait_many: Core {} thinks it is index {} via GS!",
                real_cpu_id, cpu_idx
            );
        }
    }

    if spec_count == 0 || spec_count > wait::WAIT_MANY_MAX_ITEMS {
        return Err(Errno::EINVAL);
    }
    if results_cap == 0 || results_cap > wait::WAIT_MANY_MAX_ITEMS {
        return Err(Errno::EINVAL);
    }

    let mut specs_buf = [WaitSpec::default(); wait::WAIT_MANY_MAX_ITEMS];
    unsafe {
        let dst = core::slice::from_raw_parts_mut(
            specs_buf.as_mut_ptr() as *mut u8,
            spec_count * size_of::<WaitSpec>(),
        );
        super::copyin(dst, specs_ptr)?;
    }
    let specs = &specs_buf[..spec_count];

    let mut results = [WaitResult::default(); wait::WAIT_MANY_MAX_ITEMS];
    let results_cap = results_cap.min(wait::WAIT_MANY_MAX_ITEMS);

    let timeout_deadline_ns = if timeout_ns == u64::MAX {
        None
    } else {
        Some(crate::time::monotonic_now_ns().saturating_add(timeout_ns))
    };
    let timeout_wake_tick = timeout_wake_tick(timeout_ns);

    let regs = {
        let lock = pinfo_arc.lock();
        let r = register_all(&lock, specs, tid)?;
        if let Some(wake_tick) = timeout_wake_tick {
            crate::sched::register_timeout_wake_current(tid, wake_tick);
        }
        r
    };

    let mut ready = 0;
    loop {
        ready = collect_ready(&pinfo_arc, specs, &mut results[..results_cap])?;
        if ready > 0 || timeout_expired(timeout_deadline_ns) {
            break;
        }

        if crate::sched::take_pending_interrupt_current() {
            cleanup_all(&regs, tid, timeout_wake_tick)?;
            return Err(Errno::EINTR);
        }

        unsafe {
            crate::sched::block_current_erased();
        }
    }

    cleanup_all(&regs, tid, timeout_wake_tick)?;

    let count = if ready > 0 {
        ready
    } else {
        results[0] = WaitResult {
            kind: WaitKind::Timeout as u32,
            flags: wait::ready::TIMEOUT,
            object: 0,
            token: 0,
            value: 0,
            reserved: 0,
        };
        1
    };

    unsafe {
        let src = core::slice::from_raw_parts(
            results.as_ptr() as *const u8,
            count * size_of::<WaitResult>(),
        );
        super::copyout(results_ptr, src)?;
    }
    Ok(count)
}

fn timeout_expired(timeout_deadline_ns: Option<u64>) -> bool {
    match timeout_deadline_ns {
        Some(deadline) => crate::time::monotonic_now_ns() >= deadline,
        None => false,
    }
}

fn timeout_wake_tick(timeout_ns: u64) -> Option<u64> {
    if timeout_ns == u64::MAX {
        return None;
    }
    let ticks = crate::time::duration_to_sleep_ticks(timeout_ns);
    Some(crate::sched::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed).saturating_add(ticks))
}

fn collect_ready(
    pinfo_arc: &Arc<spin::Mutex<crate::task::ProcessInfo>>,
    specs: &[WaitSpec],
    out: &mut [WaitResult],
) -> SysResult<usize> {
    let mut count = 0usize;
    for spec in specs {
        if count >= out.len() {
            break;
        }
        // crate::kinfo!("collect_ready: polling spec[{}] kind={} object={:x} token={:x}", count, spec.kind, spec.object, spec.token);
        if let Some(result) = poll_spec(pinfo_arc, spec)? {
            out[count] = result;
            count += 1;
        }
    }
    Ok(count)
}

fn poll_spec(
    pinfo_arc: &Arc<spin::Mutex<crate::task::ProcessInfo>>,
    spec: &WaitSpec,
) -> SysResult<Option<WaitResult>> {
    match WaitKind::from_u32(spec.kind).ok_or(Errno::EINVAL)? {
        WaitKind::Fd => poll_fd(pinfo_arc, spec),
        WaitKind::TaskExit => poll_task_exit(spec),
        WaitKind::Irq => Ok(poll_irq(spec)),
        WaitKind::Timeout => Err(Errno::EINVAL),
    }
}

fn error_result(spec: &WaitSpec, errno: Errno) -> WaitResult {
    WaitResult {
        kind: spec.kind,
        flags: wait::ready::ERROR,
        object: spec.object,
        token: spec.token,
        value: errno as i64,
        reserved: 0,
    }
}

fn poll_fd(
    pinfo_arc: &Arc<spin::Mutex<crate::task::ProcessInfo>>,
    spec: &WaitSpec,
) -> SysResult<Option<WaitResult>> {
    let node = {
        let pinfo = pinfo_arc.lock();
        let entry = pinfo.handle_table.get(spec.object as u32)?;
        entry.node.clone()
    };

    // let raw_ptr: *const dyn crate::vfs::VfsNode = Arc::as_ptr(&node);
    // let (ptr, vtable) = unsafe { core::mem::transmute::<*const dyn crate::vfs::VfsNode, (usize, usize)>(raw_ptr) };
    // crate::kinfo!("poll_fd: fd={} node_ptr={:x} vtable={:x}", spec.object, ptr, vtable);

    let revents = node.poll();
    let mut ready_flags = 0u32;
    if (spec.flags & wait::interest::READABLE) != 0
        && (revents & abi::syscall::poll_flags::POLLIN) != 0
    {
        ready_flags |= wait::ready::READABLE;
    }
    if (spec.flags & wait::interest::WRITABLE) != 0
        && (revents & abi::syscall::poll_flags::POLLOUT) != 0
    {
        ready_flags |= wait::ready::WRITABLE;
    }

    if (revents & abi::syscall::poll_flags::POLLHUP) != 0 {
        ready_flags |= wait::ready::HANGUP;
    }
    if (revents & abi::syscall::poll_flags::POLLERR) != 0 {
        ready_flags |= wait::ready::ERROR;
    }

    if ready_flags == 0 {
        Ok(None)
    } else {
        Ok(Some(WaitResult {
            kind: spec.kind,
            flags: ready_flags,
            object: spec.object,
            token: spec.token,
            value: 0,
            reserved: 0,
        }))
    }
}

fn poll_task_exit(spec: &WaitSpec) -> SysResult<Option<WaitResult>> {
    match unsafe { crate::sched::poll_task_exit_current(spec.object) } {
        Ok(Some(code)) => Ok(Some(WaitResult {
            kind: spec.kind,
            flags: wait::ready::EXITED,
            object: spec.object,
            token: spec.token,
            value: code as i64,
            reserved: 0,
        })),
        Ok(None) => Ok(None),
        Err(err) => Ok(Some(error_result(spec, err))),
    }
}

fn poll_irq(spec: &WaitSpec) -> Option<WaitResult> {
    if spec.object > u8::MAX as u64 {
        return Some(error_result(spec, Errno::EINVAL));
    }
    match crate::irq::poll(spec.object as u8) {
        Some(0) => None,
        Some(count) => Some(WaitResult {
            kind: spec.kind,
            flags: wait::ready::IRQ,
            object: spec.object,
            token: spec.token,
            value: count as i64,
            reserved: 0,
        }),
        None => Some(error_result(spec, Errno::ENODEV)),
    }
}

fn register_all(
    pinfo: &crate::task::ProcessInfo,
    specs: &[WaitSpec],
    tid: u64,
) -> SysResult<alloc::vec::Vec<Registration>> {
    let mut regs = alloc::vec::Vec::with_capacity(specs.len());
    for spec in specs {
        match WaitKind::from_u32(spec.kind).ok_or(Errno::EINVAL)? {
            WaitKind::Fd => {
                let node = {
                    let file =
                        pinfo.handle_table.get(spec.object as u32).map_err(|_| Errno::EBADF)?;
                    file.node.clone()
                };
                node.add_waiter(tid);
                regs.push(Registration::Fd(node));
            }
            WaitKind::TaskExit => {
                match unsafe { crate::sched::register_task_exit_waiter_current(spec.object, tid) } {
                    Ok(Some(_)) | Ok(None) => regs.push(Registration::TaskExit(spec.object)),
                    Err(_) => {}
                }
            }
            WaitKind::Irq => {}
            WaitKind::Timeout => return Err(Errno::EINVAL),
        }
    }
    Ok(regs)
}

fn cleanup_all(regs: &[Registration], tid: u64, timeout_wake_tick: Option<u64>) -> SysResult<()> {
    for reg in regs {
        match reg {
            Registration::Fd(node) => {
                node.remove_waiter(tid);
            }
            Registration::TaskExit(target) => {
                let _ = unsafe { crate::sched::unregister_task_exit_waiter_current(*target, tid) };
            }
        }
    }
    if timeout_wake_tick.is_some() {
        crate::sched::unregister_timeout_wake_current(tid);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use alloc::sync::Arc;
    use core::sync::atomic::Ordering;

    use spin::Mutex;

    use super::*;
    use crate::vfs::devfs::NullNode;

    // ── FD-semantics test helpers ─────────────────────────────────────────────
    //
    // These helpers mirror the pattern used in vfs.rs tests: they install a
    // process-info hook so that `poll_fd` (which calls
    // `crate::sched::process_info_current()`) can find a real FD table.

    static FD_TEST_GUARD: spin::Mutex<()> = spin::Mutex::new(());
    static FD_TEST_PINFO: spin::Mutex<Option<Arc<Mutex<crate::task::ProcessInfo>>>> =
        spin::Mutex::new(None);

    fn fd_test_process_info_hook() -> Option<Arc<Mutex<crate::task::ProcessInfo>>> {
        FD_TEST_PINFO.lock().clone()
    }

    fn fd_test_tid() -> u64 {
        99
    }

    fn make_pinfo_with_node(
        fd: u32,
        node: Arc<dyn crate::vfs::VfsNode>,
    ) -> Arc<Mutex<crate::task::ProcessInfo>> {
        use crate::vfs::OpenFlags;
        use crate::vfs::handle_table::HandleTable;
        let mut table = HandleTable::new();
        table.insert_at(fd, node, OpenFlags::read_write(), "/test".into()).expect("insert_at");
        Arc::new(Mutex::new(crate::task::ProcessInfo {
            pid: 1,
            job: crate::job::Job::new(0, 1),
            unix_compat: crate::task::ProcessUnixCompat::isolated(1, false),
            handle_table: table,
            ipc_table: crate::ipc::IpcHandleTable::new(),
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            root: alloc::string::String::from("/"),
            exec_path: alloc::string::String::new(),
            authority: crate::task::ProcessAuthority::root(),
            space: crate::task::ProcessAddressSpace::empty(),
            service_loop: None,
        }))
    }

    /// Run `poll_spec` with a process-info hook installed.
    fn poll_spec_with_pinfo(
        pinfo: Arc<Mutex<crate::task::ProcessInfo>>,
        spec: &WaitSpec,
    ) -> SysResult<Option<WaitResult>> {
        let _guard = FD_TEST_GUARD.lock();
        unsafe {
            crate::sched::hooks::CURRENT_TID_HOOK = Some(fd_test_tid);
            crate::sched::hooks::PROCESS_INFO_HOOK = Some(fd_test_process_info_hook);
        }
        FD_TEST_PINFO.lock().replace(pinfo.clone());
        let result = poll_spec(&pinfo, spec);
        unsafe {
            crate::sched::hooks::PROCESS_INFO_HOOK = None;
            crate::sched::hooks::CURRENT_TID_HOOK = None;
        }
        FD_TEST_PINFO.lock().take();
        result
    }

    #[test]
    fn removed_wait_kind_numbers_are_invalid() {
        let pinfo = make_pinfo_with_node(0, Arc::new(NullNode));
        for kind in [1u32, 2, 6] {
            let spec = WaitSpec { kind, flags: wait::interest::READABLE, object: 1, token: 41 };
            assert_eq!(poll_spec_with_pinfo(pinfo.clone(), &spec), Err(Errno::EINVAL));
        }
    }

    #[test]
    fn timeout_expired_handles_none_and_deadline_boundaries() {
        assert!(!timeout_expired(None));
        let now = crate::time::monotonic_now_ns();
        assert!(timeout_expired(Some(now)));
        assert!(timeout_expired(Some(now.saturating_sub(1))));
        assert!(!timeout_expired(Some(now.saturating_add(1))));
    }

    #[test]
    fn timeout_wake_tick_converts_duration_to_scheduler_ticks() {
        crate::sched::TICK_COUNT.store(50, Ordering::Relaxed);
        assert_eq!(timeout_wake_tick(u64::MAX), None);
        assert_eq!(timeout_wake_tick(0), Some(50));
        assert_eq!(timeout_wake_tick(crate::time::SCHED_TICK_NANOS), Some(51));
        assert_eq!(timeout_wake_tick(crate::time::SCHED_TICK_NANOS + 1), Some(52));
    }

    #[test]
    fn poll_irq_rejects_invalid_vectors() {
        let invalid_spec = WaitSpec {
            kind: WaitKind::Irq as u32,
            flags: 0,
            object: (u8::MAX as u64) + 1,
            token: 1,
        };
        let invalid = poll_irq(&invalid_spec).expect("invalid vector");
        assert_eq!(invalid.flags, wait::ready::ERROR);
        assert_eq!(invalid.value, Errno::EINVAL as i64);
        assert_eq!(invalid.token, invalid_spec.token);
    }

    // ── WaitKind::Fd tests ────────────────────────────────────────────────────
    //
    // These tests verify the FD-centric readiness model: poll_spec routes
    // WaitKind::Fd through poll_fd(), which translates VfsNode::poll() flags
    // into wait::ready flags.

    /// A pipe read-end with data reports ready::READABLE when interest::READABLE.
    #[test]
    fn poll_fd_pipe_read_ready_reports_readable() {
        let (read_node, write_node) = crate::ipc::pipe::create_fd_pair(0, false);
        // Write data so the read end has POLLIN.
        write_node.write(0, b"hello").expect("pipe write");
        let pinfo = make_pinfo_with_node(5, read_node);
        let spec = WaitSpec {
            kind: WaitKind::Fd as u32,
            flags: wait::interest::READABLE,
            object: 5,
            token: 200,
        };
        let result =
            poll_spec_with_pinfo(pinfo, &spec).expect("poll_spec ok").expect("fd should be ready");
        assert_ne!(
            result.flags & wait::ready::READABLE,
            0,
            "READABLE must be set on pipe read-end with data"
        );
        assert_eq!(result.token, 200);
        let _ = write_node; // keep write end alive
    }

    /// A pipe read-end with no data and a live write end is not ready.
    #[test]
    fn poll_fd_empty_pipe_read_end_not_ready() {
        let (read_node, write_node) = crate::ipc::pipe::create_fd_pair(0, false);
        let pinfo = make_pinfo_with_node(6, read_node);
        let spec = WaitSpec {
            kind: WaitKind::Fd as u32,
            flags: wait::interest::READABLE,
            object: 6,
            token: 201,
        };
        let result = poll_spec_with_pinfo(pinfo, &spec).expect("poll_spec ok");
        assert!(result.is_none(), "empty pipe with live writer must not be ready");
        let _ = write_node;
    }

    /// A pipe write-end with buffer space reports ready::WRITABLE.
    #[test]
    fn poll_fd_pipe_write_end_ready_reports_writable() {
        let (read_node, write_node) = crate::ipc::pipe::create_fd_pair(0, false);
        let pinfo = make_pinfo_with_node(7, write_node);
        let spec = WaitSpec {
            kind: WaitKind::Fd as u32,
            flags: wait::interest::WRITABLE,
            object: 7,
            token: 202,
        };
        let result = poll_spec_with_pinfo(pinfo, &spec)
            .expect("poll_spec ok")
            .expect("write end should be ready");
        assert_ne!(
            result.flags & wait::ready::WRITABLE,
            0,
            "WRITABLE must be set on pipe write-end with free space"
        );
        assert_eq!(result.token, 202);
        let _ = read_node;
    }

    /// Closing the write end causes the read end to report HANGUP.
    #[test]
    fn poll_fd_pipe_read_hangup_when_writer_closed() {
        let (read_node, write_node) = crate::ipc::pipe::create_fd_pair(0, false);
        // VfsNode::close() is the fd-close path that decrements the peer
        // refcount.  Dropping the Arc alone does not change the pipe state.
        write_node.close();
        let pinfo = make_pinfo_with_node(8, read_node);
        let spec = WaitSpec {
            kind: WaitKind::Fd as u32,
            flags: wait::interest::READABLE,
            object: 8,
            token: 203,
        };
        let result = poll_spec_with_pinfo(pinfo, &spec)
            .expect("poll_spec ok")
            .expect("read end should be ready after writer closed");
        assert_ne!(
            result.flags & wait::ready::HANGUP,
            0,
            "HANGUP must be set after the write end is closed"
        );
    }

    /// An FD that does not exist in the process table returns EBADF.
    #[test]
    fn poll_fd_missing_fd_returns_ebadf() {
        let (read_node, _write_node) = crate::ipc::pipe::create_fd_pair(0, false);
        // Install FD at 3 but poll FD 99.
        let pinfo = make_pinfo_with_node(3, read_node);
        let spec = WaitSpec {
            kind: WaitKind::Fd as u32,
            flags: wait::interest::READABLE,
            object: 99,
            token: 204,
        };
        let result = poll_spec_with_pinfo(pinfo, &spec);
        assert!(matches!(result, Err(Errno::EBADF)), "missing fd must return EBADF");
    }
}
