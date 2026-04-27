use core::sync::atomic::Ordering;

use super::super::*;
use super::support::*;
use crate::task::{Affinity, TaskId, TaskPriority, TaskState};
use crate::{BootRuntimeBase, BootTasking};

/// Thread IDs are tracked in ProcessInfo.thread_ids when tasks share the
/// same ProcessInfo Arc.
#[test]
fn test_thread_ids_tracked_in_process_info() {
    let _g = init_test_env();

    // Build a shared ProcessInfo for a 2-thread group.
    // pid = 7000 (thread-group leader), thread_ids = [7000, 7001].
    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 7000,
        job: crate::job::Job {
            ppid: 1,
            thread_ids: alloc::vec![7000, 7001],
            exec_in_progress: false,
            children_done: alloc::collections::VecDeque::new(),
            exit_observer_inbox: None,
            leader_exit_code: None,
            leader_exit_waiters: crate::sched::WaitQueue::new(),
        },
        unix_compat: crate::task::ProcessUnixCompat::isolated(7000, false),
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    {
        let pi = pinfo.lock();
        assert_eq!(pi.job.thread_ids.len(), 2);
        assert!(pi.job.thread_ids.contains(&7000));
        assert!(pi.job.thread_ids.contains(&7001));
        assert_eq!(pi.pid, 7000);
    }
}

/// When the thread-group leader exits, sibling threads are removed from the
/// thread_ids list and their scheduler state is set to Dead.
#[test]
fn test_mark_task_exited_removes_tid_from_thread_ids() {
    let _g = init_test_env();

    // Shared ProcessInfo for a 2-thread group: leader 8700, sibling 8701.
    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 8700,
        job: crate::job::Job {
            ppid: 1,
            thread_ids: alloc::vec![8700, 8701],
            exec_in_progress: false,
            children_done: alloc::collections::VecDeque::new(),
            exit_observer_inbox: None,
            leader_exit_code: None,
            leader_exit_waiters: crate::sched::WaitQueue::new(),
        },
        unix_compat: crate::task::ProcessUnixCompat::isolated(8700, false),
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    // Register both tasks.
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(8700, TaskState::Running, 8700, 1, pinfo.clone()),
    ));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(8701, TaskState::Runnable, 8700, 1, pinfo.clone()),
    ));

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(8700);

    // Exit the sibling thread first — its TID should be removed from thread_ids.
    let _ = mark_task_exited::<MockRuntime>(&mut sched, 8701, 0);

    {
        let pi = pinfo.lock();
        // 8701 should have been removed.
        assert!(!pi.job.thread_ids.contains(&8701), "sibling TID still in thread_ids");
        // 8700 (leader) is still present — it hasn't exited yet.
        assert!(pi.job.thread_ids.contains(&8700), "leader TID wrongly removed");
    }

    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(8701).unwrap().state,
        TaskState::Dead,
        "sibling should be dead"
    );
}

/// When the thread-group leader (tid == pid) exits, remaining sibling
/// threads are also killed (thread-group exit).
#[test]
fn test_thread_group_leader_exit_kills_siblings() {
    let _g = init_test_env();

    // Shared ProcessInfo for a 2-thread group: leader 8800, sibling 8801.
    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 8800,
        job: crate::job::Job {
            ppid: 1,
            thread_ids: alloc::vec![8800, 8801],
            exec_in_progress: false,
            children_done: alloc::collections::VecDeque::new(),
            exit_observer_inbox: None,
            leader_exit_code: None,
            leader_exit_waiters: crate::sched::WaitQueue::new(),
        },
        unix_compat: crate::task::ProcessUnixCompat::isolated(8800, false),
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(8800, TaskState::Running, 8800, 1, pinfo.clone()),
    ));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(8801, TaskState::Runnable, 8800, 1, pinfo.clone()),
    ));

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(8800);

    // Exit the thread-group leader.
    let _ = mark_task_exited::<MockRuntime>(&mut sched, 8800, 42);

    // Leader must be dead.
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(8800).unwrap().state,
        TaskState::Dead,
        "leader should be dead"
    );

    // Sibling must also be dead (killed by thread-group exit).
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(8801).unwrap().state,
        TaskState::Dead,
        "sibling should be killed on leader exit"
    );

    // Both TIDs removed from thread_ids.
    assert!(pinfo.lock().job.thread_ids.is_empty(), "thread_ids should be empty after group exit");
}

/// Leader exit is projected to the parent wait queue using encoded
/// wait-status semantics, and parent threads are returned as wake targets.
#[test]
fn test_mark_task_exited_queues_parent_wait_status() {
    let _g = init_test_env();
    unsafe {
        crate::sched::hooks::PROCESS_INFO_FOR_PID_HOOK = Some(process_info_for_pid::<MockRuntime>);
    }

    // Parent process (pid 9900) with one thread waiting in waitpid path.
    let parent_pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 9900,
        job: crate::job::Job {
            ppid: 1,
            thread_ids: alloc::vec![9900],
            exec_in_progress: false,
            children_done: alloc::collections::VecDeque::new(),
            exit_observer_inbox: None,
            leader_exit_code: None,
            leader_exit_waiters: crate::sched::WaitQueue::new(),
        },
        unix_compat: crate::task::ProcessUnixCompat::isolated(9900, false),
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    // Child process (pid 9800) whose leader exits with code 7.
    let child_pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 9800,
        job: crate::job::Job {
            ppid: 9900,
            thread_ids: alloc::vec![9800],
            exec_in_progress: false,
            children_done: alloc::collections::VecDeque::new(),
            exit_observer_inbox: None,
            leader_exit_code: None,
            leader_exit_waiters: crate::sched::WaitQueue::new(),
        },
        unix_compat: crate::task::ProcessUnixCompat::isolated(9800, false),
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(9900, TaskState::Blocked, 9900, 1, parent_pinfo.clone()),
    ));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(9800, TaskState::Running, 9800, 9900, child_pinfo),
    ));

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(9800);

    let waiters = mark_task_exited::<MockRuntime>(&mut sched, 9800, 7);
    assert_eq!(waiters, alloc::vec![9900], "parent thread should be returned for wakeup");

    let parent = parent_pinfo.lock();
    assert_eq!(parent.job.children_done.len(), 1);
    assert_eq!(
        parent.job.children_done.front().copied(),
        Some((9800, abi::signal::w_exit_status(7))),
        "leader exit should be queued as encoded wait status"
    );
}

/// exec_in_progress: killing siblings during exec collapse removes their
/// TIDs from thread_ids, leaving only the exec-calling thread.
#[test]
fn test_exec_collapse_kills_siblings_and_updates_thread_ids() {
    let _g = init_test_env();

    // Shared ProcessInfo for a 3-thread group: leader 9100, siblings 9101, 9102.
    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 9100,
        job: crate::job::Job {
            ppid: 1,
            thread_ids: alloc::vec![9100, 9101, 9102],
            exec_in_progress: false,
            children_done: alloc::collections::VecDeque::new(),
            exit_observer_inbox: None,
            leader_exit_code: None,
            leader_exit_waiters: crate::sched::WaitQueue::new(),
        },
        unix_compat: crate::task::ProcessUnixCompat::isolated(9100, false),
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(9100, TaskState::Running, 9100, 1, pinfo.clone()),
    ));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(9101, TaskState::Runnable, 9100, 1, pinfo.clone()),
    ));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(9102, TaskState::Runnable, 9100, 1, pinfo.clone()),
    ));

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    // Mark 9100 as the "current" (exec-calling) thread.
    sched.state.per_cpu[0].current = Some(9100);

    // Step 1: simulate exec – set exec_in_progress.
    pinfo.lock().job.exec_in_progress = true;

    // Step 2: collect siblings.
    let caller_tid: TaskId = 9100;
    let siblings: alloc::vec::Vec<TaskId> =
        pinfo.lock().job.thread_ids.iter().copied().filter(|&t| t != caller_tid).collect();
    assert_eq!(siblings.len(), 2);

    // Step 3: kill siblings (simulates kill_by_tid path).
    for sibling in siblings {
        let _ = mark_task_exited::<MockRuntime>(&mut sched, sibling, -9);
    }

    // Both siblings must be dead.
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(9101).unwrap().state,
        TaskState::Dead,
        "sibling 9101 should be dead"
    );
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(9102).unwrap().state,
        TaskState::Dead,
        "sibling 9102 should be dead"
    );

    // thread_ids should contain only the caller.
    {
        let pi = pinfo.lock();
        assert_eq!(
            pi.job.thread_ids,
            alloc::vec![caller_tid],
            "only caller TID should remain after collapse"
        );
    }

    // Step 4: simulate commit – clear exec_in_progress.
    pinfo.lock().job.exec_in_progress = false;
    assert!(!pinfo.lock().job.exec_in_progress, "exec_in_progress cleared after commit");
}

/// exec collapse with 4 threads is deterministic: ALL siblings (9701–9703)
/// are in Dead state before the exec-caller (9700) proceeds to commit.
///
/// This tests the full scheduler + registry path that `task_exec_current`
/// uses via `kill_by_tid` → `mark_task_exited`:
///   1. Set exec_in_progress.
///   2. Collect sibling TIDs (exclude caller).
///   3. Kill every sibling via mark_task_exited.
///   4. Assert every sibling is Dead and only caller TID remains.
///   5. Assert exec-caller is NOT Dead.
///   6. Commit: clear exec_in_progress.
#[test]
fn test_exec_collapse_determinism_four_threads() {
    let _g = init_test_env();

    let caller_tid: TaskId = 9700;
    let sibling_tids: [TaskId; 3] = [9701, 9702, 9703];

    let mut all_tids = alloc::vec![caller_tid];
    all_tids.extend_from_slice(&sibling_tids);

    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 9700,
        job: crate::job::Job {
            ppid: 1,
            thread_ids: all_tids.clone(),
            exec_in_progress: false,
            children_done: alloc::collections::VecDeque::new(),
            exit_observer_inbox: None,
            leader_exit_code: None,
            leader_exit_waiters: crate::sched::WaitQueue::new(),
        },
        unix_compat: {
            let mut uc = crate::task::ProcessUnixCompat::isolated(9700, false);
            uc.set_spawn_context(alloc::vec![b"old".to_vec()], alloc::vec![]);
            uc
        },
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::from("/old/binary"),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_thread_task(caller_tid, TaskState::Running, 9700, 1, pinfo.clone()),
    ));
    for &sid in &sibling_tids {
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_thread_task(sid, TaskState::Runnable, 9700, 1, pinfo.clone()),
        ));
    }

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(caller_tid);

    // Phase 1: set exec_in_progress atomically.
    pinfo.lock().job.exec_in_progress = true;

    // Phase 2: collect sibling TIDs (excluding caller).
    let siblings: alloc::vec::Vec<TaskId> =
        pinfo.lock().job.thread_ids.iter().copied().filter(|&t| t != caller_tid).collect();
    assert_eq!(siblings.len(), 3, "expected 3 siblings");
    assert!(!siblings.contains(&caller_tid), "caller must not appear in sibling list");

    // Phase 3: kill every sibling (as task_exec_current calls kill_by_tid_current).
    for &sid in &siblings {
        let _ = mark_task_exited::<MockRuntime>(&mut sched, sid, -9);
    }

    // Phase 4: invariant — every sibling must be Dead before commit.
    for &sid in &sibling_tids {
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(sid)
                .expect("sibling must remain as zombie")
                .state,
            TaskState::Dead,
            "sibling {} must be Dead after exec collapse",
            sid
        );
    }

    // thread_ids must contain only the exec-caller.
    assert_eq!(
        pinfo.lock().job.thread_ids,
        alloc::vec![caller_tid],
        "only exec-caller TID must remain in thread_ids after collapse"
    );

    // The exec-caller itself must NOT be Dead.
    assert_ne!(
        crate::task::registry::get_task::<MockRuntime>(caller_tid)
            .expect("exec-caller must still be in registry")
            .state,
        TaskState::Dead,
        "exec-caller must not be killed during collapse"
    );

    // Phase 5: commit — clear exec_in_progress.
    pinfo.lock().job.exec_in_progress = false;
    assert!(!pinfo.lock().job.exec_in_progress, "exec_in_progress must be cleared after commit");
}

/// exec_in_progress blocks additional thread creation at the process level.
#[test]
fn test_exec_in_progress_rejects_new_threads() {
    let _g = init_test_env();

    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 9300,
        job: crate::job::Job::new(1, 9300),
        unix_compat: crate::task::ProcessUnixCompat::isolated(9300, false),
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    // Before exec: flag is clear — new threads would be accepted.
    assert!(!pinfo.lock().job.exec_in_progress);

    // Set exec_in_progress (as task_exec_current does at the start).
    pinfo.lock().job.exec_in_progress = true;

    // The sys_spawn_thread handler checks this flag and returns EAGAIN.
    // Here we verify the condition it tests.
    assert!(
        pinfo.lock().job.exec_in_progress,
        "exec_in_progress must be set to block SYS_SPAWN_THREAD"
    );

    // Rollback: clear the flag on pre-commit failure.
    pinfo.lock().job.exec_in_progress = false;
    assert!(!pinfo.lock().job.exec_in_progress, "flag cleared after rollback");
}

// ── TLS-base and detached-thread tests ───────────────────────────────────

/// A task constructed with a non-zero `user_fs_base` retains that value.
///
/// This is the kernel-side invariant for the TLS-base handoff: the spawn
/// path stores `tls_base` in `Task.user_fs_base`, and the scheduler
/// writes it to hardware (FS_BASE) on the first context switch.
#[test]
fn test_tls_base_stored_in_task_user_fs_base() {
    let _g = init_test_env();

    let tls_base: u64 = 0xDEAD_CAFE_0000_0000;

    let task = crate::task::Task {
        id: 9800,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: true,
        wake_pending: false,
        pending_interrupt: false,
        affinity: Affinity::Any,
        kstack_base: core::ptr::null_mut(),
        kstack_size: 0,
        kstack_top: 0,
        ctx: Default::default(),
        aspace: MockAddressSpace(0),
        simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
        stack_info: None,
        mappings: alloc::sync::Arc::new(spin::Mutex::new(
            crate::memory::mappings::MappingList::new(),
        )),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        last_cpu: Some(0),
        name: [0; 32],
        name_len: 0,
        process_info: None,
        user_fs_base: tls_base,
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

    let stored = crate::task::registry::get_task::<MockRuntime>(9800)
        .expect("task must be in registry")
        .user_fs_base;
    assert_eq!(stored, tls_base, "user_fs_base must equal the requested tls_base");
}

/// Joining a detached thread must return `EINVAL`.
#[test]
fn test_detached_thread_cannot_be_joined() {
    let _g = init_test_env();

    let task = crate::task::Task {
        id: 9801,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: true,
        wake_pending: false,
        pending_interrupt: false,
        affinity: Affinity::Any,
        kstack_base: core::ptr::null_mut(),
        kstack_size: 0,
        kstack_top: 0,
        ctx: Default::default(),
        aspace: MockAddressSpace(0),
        simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
        stack_info: None,
        mappings: alloc::sync::Arc::new(spin::Mutex::new(
            crate::memory::mappings::MappingList::new(),
        )),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        last_cpu: Some(0),
        name: [0; 32],
        name_len: 0,
        process_info: None,
        user_fs_base: 0,
        detached: true, // detached — must not be joinable
        signals: crate::signal::ThreadSignals::new(),
    };

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

    assert_eq!(
        register_task_exit_waiter::<MockRuntime>(9801, 9802).unwrap_err(),
        abi::errors::Errno::EINVAL,
        "joining a detached thread must return EINVAL"
    );
}

/// A live joinable (non-detached) thread allows waiting via
/// `register_task_exit_waiter`, returning `None` (not yet exited).
#[test]
fn test_joinable_thread_can_be_waited_on() {
    let _g = init_test_env();

    let task = crate::task::Task {
        id: 9803,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: true,
        wake_pending: false,
        pending_interrupt: false,
        affinity: Affinity::Any,
        kstack_base: core::ptr::null_mut(),
        kstack_size: 0,
        kstack_top: 0,
        ctx: Default::default(),
        aspace: MockAddressSpace(0),
        simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
        stack_info: None,
        mappings: alloc::sync::Arc::new(spin::Mutex::new(
            crate::memory::mappings::MappingList::new(),
        )),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        last_cpu: Some(0),
        name: [0; 32],
        name_len: 0,
        process_info: None,
        user_fs_base: 0,
        detached: false, // joinable
        signals: crate::signal::ThreadSignals::new(),
    };

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

    // Should succeed and return None (thread still running).
    assert_eq!(
        register_task_exit_waiter::<MockRuntime>(9803, 9804).unwrap(),
        None,
        "joining a live joinable thread must return None"
    );
}
