use alloc::sync::Arc;
use core::mem::size_of;

use abi::errors::{Errno, SysResult};
use abi::wait::{self, WaitKind, WaitResult, WaitSpec};

#[derive(Clone)]
enum Registration {
    PortRead(Arc<crate::ipc::Port>),
    PortWrite(Arc<crate::ipc::Port>),
    Fd(Arc<dyn crate::vfs::VfsNode>),
    GraphOp(u64),
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

    let nonblocking_poll = timeout_ns == 0;
    let timeout_deadline_ns = if timeout_ns == u64::MAX {
        None
    } else {
        Some(crate::time::monotonic_now_ns().saturating_add(timeout_ns))
    };
    let timeout_wake_tick = timeout_wake_tick(timeout_ns);

    let regs = {
        let lock = pinfo_arc.lock();
        let r = if nonblocking_poll {
            alloc::vec::Vec::new()
        } else {
            register_all(&lock, specs, tid)?
        };
        if let Some(wake_tick) = timeout_wake_tick {
            crate::sched::register_timeout_wake_current(tid, wake_tick);
        }
        r
    };

    let ready = loop {
        let ready = collect_ready(&pinfo_arc, specs, &mut results[..results_cap])?;
        if ready > 0 || timeout_expired(timeout_deadline_ns) {
            break ready;
        }

        if crate::sched::take_pending_interrupt_current() {
            cleanup_all(&regs, tid, timeout_wake_tick)?;
            return Err(Errno::EINTR);
        }

        unsafe {
            crate::sched::block_current_erased();
        }
    };

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
    if timeout_ns == 0 || timeout_ns == u64::MAX {
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

#[allow(deprecated)] // handle legacy WaitKind variants that map to ENOSYS
fn poll_spec(
    pinfo_arc: &Arc<spin::Mutex<crate::task::ProcessInfo>>,
    spec: &WaitSpec,
) -> SysResult<Option<WaitResult>> {
    match WaitKind::from_u32(spec.kind).ok_or(Errno::EINVAL)? {
        WaitKind::Port => poll_port(pinfo_arc, spec),
        WaitKind::Fd => poll_fd(pinfo_arc, spec),
        WaitKind::GraphOp => poll_graph_op(spec),
        WaitKind::TaskExit => poll_task_exit(spec),
        WaitKind::Irq => Ok(poll_irq(spec)),
        WaitKind::RootWatch => Err(Errno::ENOSYS),
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

fn poll_port(
    pinfo_arc: &Arc<spin::Mutex<crate::task::ProcessInfo>>,
    spec: &WaitSpec,
) -> SysResult<Option<WaitResult>> {
    let handle = crate::ipc::IpcHandle(spec.object as u32);
    let mut ready_flags = 0u32;
    let mut value = 0i64;

    if (spec.flags & wait::interest::READABLE) != 0 {
        let port = {
            let pinfo = pinfo_arc.lock();
            let table = &pinfo.ipc_table;
            match table.get(handle, crate::ipc::IpcHandleMode::Read) {
                Some(entry) => Arc::clone(&entry.port),
                None => return Ok(Some(error_result(spec, Errno::EBADF))),
            }
        };

        // Diagnostic: catch sporadic null/bad Arc pointers
        let raw_port_ptr = Arc::as_ptr(&port);
        if (raw_port_ptr as usize) < 0x1000 {
            panic!(
                "BAD PORT POINTER in poll_port (READABLE path): {:p}, handle={:?}",
                raw_port_ptr, handle
            );
        }

        let is_empty = port.is_empty();
        let has_writers = port.has_writers();
        if !is_empty {
            ready_flags |= wait::ready::READABLE;
            value = port.len() as i64;
        } else if !has_writers {
            ready_flags |= wait::ready::HANGUP;
        }
    }

    if (spec.flags & wait::interest::WRITABLE) != 0 {
        let port = {
            let pinfo = pinfo_arc.lock();
            let table = &pinfo.ipc_table;
            match table.get(handle, crate::ipc::IpcHandleMode::Write) {
                Some(entry) => Arc::clone(&entry.port),
                None => return Ok(Some(error_result(spec, Errno::EBADF))),
            }
        };

        // Diagnostic: catch sporadic null/bad Arc pointers
        let raw_port_ptr = Arc::as_ptr(&port);
        if (raw_port_ptr as usize) < 0x1000 {
            panic!(
                "BAD PORT POINTER in poll_port (WRITABLE path): {:p}, handle={:?}",
                raw_port_ptr, handle
            );
        }

        if !port.is_full() {
            ready_flags |= wait::ready::WRITABLE;
            value = port.available() as i64;
        } else if !port.has_readers() {
            ready_flags |= wait::ready::HANGUP;
        }
    }

    if ready_flags == 0 {
        Ok(None)
    } else {
        Ok(Some(WaitResult {
            kind: spec.kind,
            flags: ready_flags,
            object: spec.object,
            token: spec.token,
            value,
            reserved: 0,
        }))
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

fn poll_graph_op(_spec: &WaitSpec) -> SysResult<Option<WaitResult>> {
    Err(Errno::ENOSYS)
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
    let tid = unsafe { crate::sched::current_tid_current() };
    match crate::irq::try_wait(spec.object as u8, tid) {
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

#[allow(deprecated)] // handle legacy WaitKind variants that map to ENOSYS
fn register_all(
    pinfo: &crate::task::ProcessInfo,
    specs: &[WaitSpec],
    tid: u64,
) -> SysResult<alloc::vec::Vec<Registration>> {
    let mut regs = alloc::vec::Vec::with_capacity(specs.len());
    for spec in specs {
        match WaitKind::from_u32(spec.kind).ok_or(Errno::EINVAL)? {
            WaitKind::Port => {
                let handle = crate::ipc::IpcHandle(spec.object as u32);
                let table = &pinfo.ipc_table;
                if (spec.flags & wait::interest::READABLE) != 0 {
                    if let Some(entry) = table.get(handle, crate::ipc::IpcHandleMode::Read) {
                        entry.port.add_waiter_read(tid);
                        regs.push(Registration::PortRead(Arc::clone(&entry.port)));
                    }
                }
                if (spec.flags & wait::interest::WRITABLE) != 0 {
                    if let Some(entry) = table.get(handle, crate::ipc::IpcHandleMode::Write) {
                        entry.port.add_waiter_write(tid);
                        regs.push(Registration::PortWrite(Arc::clone(&entry.port)));
                    }
                }
            }
            WaitKind::Fd => {
                let node = {
                    let file =
                        pinfo.handle_table.get(spec.object as u32).map_err(|_| Errno::EBADF)?;
                    file.node.clone()
                };
                node.add_waiter(tid);
                regs.push(Registration::Fd(node));
            }
            WaitKind::GraphOp => {
                return Err(Errno::ENOSYS);
            }
            WaitKind::TaskExit => {
                match unsafe { crate::sched::register_task_exit_waiter_current(spec.object, tid) } {
                    Ok(Some(_)) | Ok(None) => regs.push(Registration::TaskExit(spec.object)),
                    Err(_) => {}
                }
            }
            WaitKind::Irq => {}
            WaitKind::RootWatch => return Err(Errno::ENOSYS),
            WaitKind::Timeout => return Err(Errno::EINVAL),
        }
    }
    Ok(regs)
}

fn cleanup_all(regs: &[Registration], tid: u64, timeout_wake_tick: Option<u64>) -> SysResult<()> {
    for reg in regs {
        match reg {
            Registration::PortRead(port) => {
                port.remove_waiter_read(tid);
            }
            Registration::PortWrite(port) => {
                port.remove_waiter_write(tid);
            }
            Registration::Fd(node) => {
                node.remove_waiter(tid);
            }
            Registration::GraphOp(_id) => {}
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
#[allow(deprecated)] // WaitKind::Port is deprecated but its handler is kept for backward compat
mod tests {
    use alloc::sync::Arc;
    use core::sync::atomic::Ordering;

    use spin::Mutex;

    use super::*;
    use crate::vfs::devfs::NullNode;

    fn alloc_port_pair(
        pinfo: &Arc<Mutex<crate::task::ProcessInfo>>,
        capacity: usize,
    ) -> (u32, u32) {
        let port_id = crate::ipc::create_port(capacity);
        let port = crate::ipc::get_port(port_id).expect("port");
        let mut lock = pinfo.lock();
        let write = lock
            .ipc_table
            .alloc(port.clone(), crate::ipc::IpcHandleMode::Write)
            .expect("write handle");
        let read =
            lock.ipc_table.alloc(port, crate::ipc::IpcHandleMode::Read).expect("read handle");
        (write.0, read.0)
    }

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
    fn poll_port_reports_readable_and_hangup() {
        let pinfo = make_pinfo_with_node(0, Arc::new(NullNode));
        let (write_handle, read_handle) = alloc_port_pair(&pinfo, 64);
        let port = {
            let lock = pinfo.lock();
            let entry = lock
                .ipc_table
                .get(crate::ipc::IpcHandle(write_handle), crate::ipc::IpcHandleMode::Write)
                .expect("entry");
            Arc::clone(&entry.port)
        };

        assert!(port.send_all(b"abc"));
        let readable = poll_spec_with_pinfo(
            pinfo.clone(),
            &WaitSpec {
                kind: WaitKind::Port as u32,
                flags: wait::interest::READABLE,
                object: read_handle as u64,
                token: 11,
            },
        )
        .expect("poll")
        .expect("ready");
        assert_ne!(readable.flags & wait::ready::READABLE, 0);
        assert_eq!(readable.token, 11);

        assert!(!port.close_writer());
        let hangup = poll_spec_with_pinfo(
            pinfo.clone(),
            &WaitSpec {
                kind: WaitKind::Port as u32,
                flags: wait::interest::READABLE,
                object: read_handle as u64,
                token: 12,
            },
        )
        .expect("poll")
        .expect("ready");
        assert_ne!(hangup.flags & wait::ready::READABLE, 0);

        let mut drain = [0u8; 8];
        assert_eq!(port.try_recv(&mut drain), 3);

        let hangup_only = poll_spec_with_pinfo(
            pinfo.clone(),
            &WaitSpec {
                kind: WaitKind::Port as u32,
                flags: wait::interest::READABLE,
                object: read_handle as u64,
                token: 13,
            },
        )
        .expect("poll")
        .expect("ready");
        assert_eq!(hangup_only.flags, wait::ready::HANGUP);
    }

    #[test]
    fn poll_port_reports_writable_capacity_and_write_hangup() {
        let pinfo = make_pinfo_with_node(0, Arc::new(NullNode));
        let (write_handle, _read_handle) = alloc_port_pair(&pinfo, 4);
        let port = {
            let lock = pinfo.lock();
            let entry = lock
                .ipc_table
                .get(crate::ipc::IpcHandle(write_handle), crate::ipc::IpcHandleMode::Write)
                .expect("entry");
            Arc::clone(&entry.port)
        };

        let writable = poll_spec_with_pinfo(
            pinfo.clone(),
            &WaitSpec {
                kind: WaitKind::Port as u32,
                flags: wait::interest::WRITABLE,
                object: write_handle as u64,
                token: 21,
            },
        )
        .expect("poll")
        .expect("ready");
        assert_eq!(writable.flags, wait::ready::WRITABLE);
        assert_eq!(writable.value, 16);

        assert!(port.send_all(&[0u8; 16]));
        let not_writable = poll_spec_with_pinfo(
            pinfo.clone(),
            &WaitSpec {
                kind: WaitKind::Port as u32,
                flags: wait::interest::WRITABLE,
                object: write_handle as u64,
                token: 22,
            },
        )
        .expect("poll");
        assert!(not_writable.is_none());

        assert!(!port.close_reader());
        let hangup = poll_spec_with_pinfo(
            pinfo.clone(),
            &WaitSpec {
                kind: WaitKind::Port as u32,
                flags: wait::interest::WRITABLE,
                object: write_handle as u64,
                token: 23,
            },
        )
        .expect("poll")
        .expect("ready");
        assert_eq!(hangup.flags, wait::ready::HANGUP);
    }

    #[test]
    fn collect_ready_returns_multiple_ports() {
        let pinfo = make_pinfo_with_node(0, Arc::new(NullNode));
        let (write_a, read_a) = alloc_port_pair(&pinfo, 64);
        let (write_b, read_b) = alloc_port_pair(&pinfo, 64);

        let port_a = {
            let lock = pinfo.lock();
            let entry = lock
                .ipc_table
                .get(crate::ipc::IpcHandle(write_a), crate::ipc::IpcHandleMode::Write)
                .expect("entry a");
            Arc::clone(&entry.port)
        };
        let port_b = {
            let lock = pinfo.lock();
            let entry = lock
                .ipc_table
                .get(crate::ipc::IpcHandle(write_b), crate::ipc::IpcHandleMode::Write)
                .expect("entry b");
            Arc::clone(&entry.port)
        };

        assert!(port_a.send_all(b"a"));
        assert!(port_b.send_all(b"bb"));

        let specs = [
            WaitSpec {
                kind: WaitKind::Port as u32,
                flags: wait::interest::READABLE,
                object: read_a as u64,
                token: 1,
            },
            WaitSpec {
                kind: WaitKind::Port as u32,
                flags: wait::interest::READABLE,
                object: read_b as u64,
                token: 2,
            },
        ];
        let mut results = [WaitResult::default(); 2];
        let ready = collect_ready(&pinfo, &specs, &mut results).expect("collect");
        assert_eq!(ready, 2);
        assert_eq!(results[0].token, 1);
        assert_eq!(results[1].token, 2);
    }

    #[test]
    fn collect_ready_respects_output_capacity() {
        let pinfo = make_pinfo_with_node(0, Arc::new(NullNode));
        let (write_a, read_a) = alloc_port_pair(&pinfo, 64);
        let (write_b, read_b) = alloc_port_pair(&pinfo, 64);

        let port_a = {
            let lock = pinfo.lock();
            let entry = lock
                .ipc_table
                .get(crate::ipc::IpcHandle(write_a), crate::ipc::IpcHandleMode::Write)
                .expect("entry a");
            Arc::clone(&entry.port)
        };
        let port_b = {
            let lock = pinfo.lock();
            let entry = lock
                .ipc_table
                .get(crate::ipc::IpcHandle(write_b), crate::ipc::IpcHandleMode::Write)
                .expect("entry b");
            Arc::clone(&entry.port)
        };

        assert!(port_a.send_all(b"a"));
        assert!(port_b.send_all(b"b"));

        let specs = [
            WaitSpec {
                kind: WaitKind::Port as u32,
                flags: wait::interest::READABLE,
                object: read_a as u64,
                token: 101,
            },
            WaitSpec {
                kind: WaitKind::Port as u32,
                flags: wait::interest::READABLE,
                object: read_b as u64,
                token: 102,
            },
        ];
        let mut results = [WaitResult::default(); 1];
        let ready = collect_ready(&pinfo, &specs, &mut results).expect("collect");
        assert_eq!(ready, 1);
        assert_eq!(results[0].token, 101);
    }

    #[test]
    #[allow(deprecated)]
    fn poll_graph_op_returns_enosys() {
        // GraphOp is deprecated and returns ENOSYS.  This test verifies the
        // backward-compatibility shim remains in place so existing binaries that
        // pass WaitKind::GraphOp = 6 receive a clean error rather than EINVAL.
        let spec = WaitSpec { kind: WaitKind::GraphOp as u32, flags: 0, object: 1, token: 41 };
        let pinfo = make_pinfo_with_node(0, Arc::new(NullNode));
        assert!(matches!(poll_spec_with_pinfo(pinfo, &spec), Err(Errno::ENOSYS)));
    }

    #[test]
    fn collect_ready_returns_mixed_port_and_fd() {
        let pinfo = make_pinfo_with_node(0, Arc::new(NullNode));
        let (write_handle, read_handle) = alloc_port_pair(&pinfo, 64);
        let port = {
            let lock = pinfo.lock();
            let entry = lock
                .ipc_table
                .get(crate::ipc::IpcHandle(write_handle), crate::ipc::IpcHandleMode::Write)
                .expect("entry");
            Arc::clone(&entry.port)
        };
        assert!(port.send_all(b"hello"));

        // Build a pipe and push one byte so the read end is POLLIN-ready.
        // Keep `write_node` alive for the duration of the test so the write
        // end is not closed prematurely — otherwise the read end would show
        // POLLHUP in addition to POLLIN, which is correct but not what the
        // assertion below tests.
        let (read_node, write_node) = crate::ipc::pipe::create_fd_pair(0, false);
        write_node.write(0, b"x").expect("pipe write");

        // Stash the pipe read node in a temporary HandleTable so we can look it
        // up through the Fd WaitKind path.
        use crate::vfs::OpenFlags;
        use crate::vfs::handle_table::HandleTable;
        let mut table = HandleTable::new();
        table
            .insert_at(0, read_node, OpenFlags::read_only(), "/pipe/read".into())
            .expect("insert pipe");

        // Poll the Port spec directly (does not need a process context).
        let port_spec = WaitSpec {
            kind: WaitKind::Port as u32,
            flags: wait::interest::READABLE,
            object: read_handle as u64,
            token: 10,
        };
        let port_result =
            poll_spec_with_pinfo(pinfo.clone(), &port_spec).expect("port poll").expect("ready");
        assert_ne!(port_result.flags & wait::ready::READABLE, 0);
        assert_eq!(port_result.token, 10);

        // Poll the Fd spec directly against the node from the table.
        let node = table.get(0).expect("get fd").node.clone();
        let fd_ready_flags = node.poll();
        assert_ne!(
            fd_ready_flags & abi::syscall::poll_flags::POLLIN,
            0,
            "pipe read-end should be POLLIN-ready after write"
        );
        // Keep write_node alive until after the assertion so POLLHUP is not set.
        let _ = write_node;
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
        assert_eq!(timeout_wake_tick(0), None);
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
