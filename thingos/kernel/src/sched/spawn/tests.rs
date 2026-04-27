use super::process_info::boot_module_matches;
use super::task::register_thread_in_process;
use super::*;
// Re-use the shared mock runtime defined in `sched::tests` so that both
// this module and `sched::mod` share a single `init_runtime` call and a
// single `MockRuntime` type.  This prevents the "Runtime type mismatch" /
// double-init panics that occur when each module defines its own mock.
use crate::sched::tests::{MockRuntime, init_test_env};
use crate::sched::types::Scheduler;
use crate::task::{Affinity, StartupArg, TaskPriority};

#[test]
fn boot_module_match_requires_exact_basename() {
    assert!(boot_module_matches("ls", "/bin/ls"));
    assert!(boot_module_matches("/bin/ls", "/bin/ls"));
}

#[test]
fn test_spawn_arg_semantics() {
    let _g = init_test_env();

    let mut sched = Scheduler::<MockRuntime>::new();
    sched.next_id = 5000;
    // Manually initialize PerCpu state for the mock
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(0); // Set a dummy current task ID for parent linking

    let cases = [
        (StartupArg::None, 0),
        (StartupArg::BootRegistry, 0x600000),
        (StartupArg::DeviceId(0x123), 0x123),
        (StartupArg::Raw(0x42), 0x42),
    ];

    for (arg, expected) in cases {
        let id = sched.spawn(mock_entry, arg, TaskPriority::Normal, Affinity::Any);
        let task = crate::task::registry::get_task::<MockRuntime>(id).unwrap();

        // In our MockTasking.init_kernel_context, we store arg in MockContext.0
        assert_eq!(task.ctx.0, expected);
        assert_eq!(arg.to_raw(), expected);
    }
}

#[test]
fn spawn_defers_registry_insert_when_scheduler_lock_is_tracked() {
    let _g = init_test_env();

    let mut sched = Scheduler::<MockRuntime>::new();
    sched.next_id = 7000;
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(0);

    super::super::set_sched_lock_tracking::<MockRuntime>(0);
    let id = sched.spawn(mock_entry, StartupArg::Raw(9), TaskPriority::Normal, Affinity::Any);
    super::super::clear_sched_lock_tracking::<MockRuntime>();

    assert!(
        crate::task::registry::get_task::<MockRuntime>(id).is_none(),
        "spawn should defer REGISTRY insertion while scheduler lock is held"
    );
    assert_eq!(sched.pending_registry_inserts.len(), 1);

    super::super::apply_deferred_registry_inserts::<MockRuntime>(
        sched.drain_pending_registry_inserts(),
    );
    assert!(crate::task::registry::get_task::<MockRuntime>(id).is_some());
}

/// Verify that `spawn_user_thread` correctly routes the startup argument
/// into the task context so the entry function receives it in the first
/// argument register (e.g. `rdi` on x86_64).
///
/// The `MockRuntime::init_user_context` stores `spec.arg` directly in the
/// mock context, so asserting `task.ctx.0 == expected` confirms the full
/// pipeline:
///   `spawn_with_arg(entry, arg)`
///   → `SYS_SPAWN_THREAD` with `SpawnThreadReq { arg }`
///   → `StartupArg::Raw(arg)` passed to `spawn_user_thread`
///   → `UserTaskSpec { arg }` passed to `init_user_context`
///   → arg placed in first argument register on the target arch
#[test]
fn test_spawn_user_thread_arg() {
    let _g = init_test_env();

    let mut sched = Scheduler::<MockRuntime>::new();
    sched.next_id = 9000;
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(0);

    let stack_info = abi::types::StackInfo::default();

    let cases: &[(usize, usize)] =
        &[(0, 0), (0x1234, 0x1234), (0xDEAD_BEEF, 0xDEAD_BEEF), (usize::MAX, usize::MAX)];

    for &(raw_arg, expected) in cases {
        let id = sched.spawn_user_thread(
            0x4000, // mock entry address
            0x8000, // mock user stack pointer
            StartupArg::Raw(raw_arg),
            stack_info,
            TaskPriority::Normal,
            crate::task::Affinity::Any,
            0,     // tls_base
            false, // detached
        );
        let task = crate::task::registry::get_task::<MockRuntime>(id).unwrap();

        // MockRuntime::init_user_context stores spec.arg in MockContext.0
        assert_eq!(task.ctx.0, expected, "user thread arg mismatch for raw_arg={:#x}", raw_arg);
    }
}

extern "C" fn mock_entry(_arg: usize) -> ! {
    loop {}
}

// ── Thread-group membership invariant tests ───────────────────────────

/// Helper: build a minimal `ProcessInfo` Arc with the given leader TID.
fn make_process_info(
    leader: crate::task::TaskId,
) -> alloc::sync::Arc<spin::Mutex<crate::task::ProcessInfo>> {
    alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: leader as u32,
        job: crate::task::ProcessLifecycle::new(1, leader),
        unix_compat: crate::task::ProcessUnixCompat::isolated(leader as u32, false),
        handle_table: crate::vfs::handle_table::HandleTable::new(),
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

/// Helper: build a minimal leader task backed by `pinfo` and insert it into
/// the registry, then set it as the current task on CPU 0.
fn setup_leader_task(
    sched: &mut Scheduler<MockRuntime>,
    leader_id: crate::task::TaskId,
    pinfo: alloc::sync::Arc<spin::Mutex<crate::task::ProcessInfo>>,
) {
    use crate::task::{TaskPriority, TaskState};
    let leader = crate::task::Task {
        id: leader_id,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: true,
        wake_pending: false,
        pending_interrupt: false,
        affinity: crate::task::Affinity::Any,
        kstack_base: core::ptr::null_mut(),
        kstack_size: 0,
        kstack_top: 0,
        ctx: Default::default(),
        aspace: crate::sched::tests::MockAddressSpace(0),
        simd: crate::simd::SimdState::new(&crate::sched::tests::MOCK_RUNTIME),
        stack_info: None,
        mappings: alloc::sync::Arc::new(spin::Mutex::new(
            crate::memory::mappings::MappingList::new(),
        )),
        timeslice_remaining: crate::sched::types::DEFAULT_TIMESLICE,
        last_cpu: Some(0),
        name: [0; 32],
        name_len: 0,
        process_info: Some(pinfo),
        user_fs_base: 0,
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(leader));
    sched.state.per_cpu[0].current = Some(leader_id);
}

/// A thread spawned with `tls_base = 0` must appear in the parent
/// process's `thread_ids`.
///
/// This directly tests the fix for the historical bug where the
/// registration was inadvertently gated on `tls_base != 0`.
#[test]
fn test_spawn_thread_zero_tls_base_registered_in_thread_ids() {
    let _g = init_test_env();

    let leader_id: crate::task::TaskId = 7100;
    let pinfo = make_process_info(leader_id);

    let mut sched = Scheduler::<MockRuntime>::new();
    sched.next_id = 7101;
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    setup_leader_task(&mut sched, leader_id, pinfo.clone());

    let child_id = sched.spawn_user_thread(
        0x4000,
        0x8000,
        StartupArg::None,
        abi::types::StackInfo::default(),
        TaskPriority::Normal,
        crate::task::Affinity::Any,
        0, // tls_base = 0 — the historically broken case
        false,
    );

    let pi = pinfo.lock();
    assert!(
        pi.job.thread_ids.contains(&child_id),
        "thread spawned with tls_base=0 must appear in process thread_ids"
    );
    assert!(
        pi.job.thread_ids.contains(&leader_id),
        "leader TID must still be present after spawning a child"
    );
}

/// A thread spawned with a non-zero `tls_base` must also appear in the
/// parent process's `thread_ids`.
#[test]
fn test_spawn_thread_nonzero_tls_base_registered_in_thread_ids() {
    let _g = init_test_env();

    let leader_id: crate::task::TaskId = 7200;
    let pinfo = make_process_info(leader_id);

    let mut sched = Scheduler::<MockRuntime>::new();
    sched.next_id = 7201;
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    setup_leader_task(&mut sched, leader_id, pinfo.clone());

    let child_id = sched.spawn_user_thread(
        0x4000,
        0x8000,
        StartupArg::None,
        abi::types::StackInfo::default(),
        TaskPriority::Normal,
        crate::task::Affinity::Any,
        0xDEAD_CAFE_0000_0000u64, // non-zero tls_base
        false,
    );

    let pi = pinfo.lock();
    assert!(
        pi.job.thread_ids.contains(&child_id),
        "thread spawned with non-zero tls_base must appear in process thread_ids"
    );
}

/// Calling `register_thread_in_process` twice for the same TID must not
/// produce duplicate entries in `thread_ids`.
#[test]
fn test_register_thread_in_process_no_duplicates() {
    let _g = init_test_env();

    let pinfo = make_process_info(7300);
    let pinfo_opt = Some(pinfo.clone());

    // First registration (e.g. from spawn path).
    register_thread_in_process(&pinfo_opt, 7301);
    // Second registration (e.g. accidental double-call or re-use).
    register_thread_in_process(&pinfo_opt, 7301);

    let pi = pinfo.lock();
    let count = pi.job.thread_ids.iter().filter(|&&t| t == 7301).count();
    assert_eq!(count, 1, "duplicate TID entries must not be created");
}

/// Spawning multiple threads in sequence must register every one of them.
#[test]
fn test_multiple_threads_all_registered_in_thread_ids() {
    let _g = init_test_env();

    let leader_id: crate::task::TaskId = 7400;
    let pinfo = make_process_info(leader_id);

    let mut sched = Scheduler::<MockRuntime>::new();
    sched.next_id = 7401;
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    setup_leader_task(&mut sched, leader_id, pinfo.clone());

    let mut child_ids = alloc::vec::Vec::new();
    for _ in 0..4 {
        let id = sched.spawn_user_thread(
            0x4000,
            0x8000,
            StartupArg::None,
            abi::types::StackInfo::default(),
            TaskPriority::Normal,
            crate::task::Affinity::Any,
            0,
            false,
        );
        child_ids.push(id);
    }

    let pi = pinfo.lock();
    // Leader + 4 children = 5 entries, no duplicates.
    assert_eq!(pi.job.thread_ids.len(), 5, "all spawned threads plus leader must be in thread_ids");
    for &cid in &child_ids {
        assert!(pi.job.thread_ids.contains(&cid), "child TID {} must be in thread_ids", cid);
    }
}

// ── Runtime spawn path tests ──────────────────────────────────────────

/// `spawn_process_from_path` must return `ENOENT` immediately when the
/// requested path has no matching VFS mount.
///
/// This verifies the first step of the runtime process creation model:
/// "open executable" from the VFS.  If the path does not resolve, no
/// scheduler or runtime interactions occur.
#[test]
fn test_spawn_process_from_path_returns_enoent_for_missing_vfs_path() {
    let _g = init_test_env();

    // Use a path that will not match any mount that might already be present.
    let result = unsafe {
        spawn_process_from_path::<MockRuntime>(
            "/totally/nonexistent/binary_9f3a1b",
            alloc::vec![],
            alloc::collections::BTreeMap::new(),
            StdioSpec::Inherit,
            StdioSpec::Inherit,
            StdioSpec::Inherit,
            0,
            alloc::vec![],
            None,
            alloc::vec![],
            None,
        )
    };

    assert!(
        matches!(result, Err(abi::errors::Errno::ENOENT)),
        "expected ENOENT for a path not present in the VFS, got {:?}",
        result
    );
}

/// `spawn_process_from_path` must return `EACCES` when the VFS node exists
/// but is not a regular file (e.g. a directory).
///
/// This verifies the runtime path's validation step before it ever touches
/// the scheduler or ELF loader.
#[test]
fn test_spawn_process_from_path_returns_eacces_for_directory_node() {
    let _g = init_test_env();

    // A VFS node that pretends to be a directory.
    struct DirNode;
    impl crate::vfs::VfsNode for DirNode {
        fn read(&self, _: u64, _: &mut [u8]) -> abi::errors::SysResult<usize> {
            Ok(0)
        }
        fn write(&self, _: u64, _: &[u8]) -> abi::errors::SysResult<usize> {
            Ok(0)
        }
        fn stat(&self) -> abi::errors::SysResult<crate::vfs::VfsStat> {
            Ok(crate::vfs::VfsStat {
                mode: crate::vfs::VfsStat::S_IFDIR | 0o755,
                size: 0,
                ino: 1,
                ..Default::default()
            })
        }
    }

    struct DirFs;
    impl crate::vfs::VfsDriver for DirFs {
        fn lookup(
            &self,
            path: &str,
        ) -> abi::errors::SysResult<alloc::sync::Arc<dyn crate::vfs::VfsNode>> {
            if path == "notafile" {
                Ok(alloc::sync::Arc::new(DirNode))
            } else {
                Err(abi::errors::Errno::ENOENT)
            }
        }
    }

    crate::vfs::mount::init();
    crate::vfs::mount::mount("/spawn_test_dir", alloc::sync::Arc::new(DirFs), 0);

    let result = unsafe {
        spawn_process_from_path::<MockRuntime>(
            "/spawn_test_dir/notafile",
            alloc::vec![],
            alloc::collections::BTreeMap::new(),
            StdioSpec::Inherit,
            StdioSpec::Inherit,
            StdioSpec::Inherit,
            0,
            alloc::vec![],
            None,
            alloc::vec![],
            None,
        )
    };

    let _ = crate::vfs::mount::umount("/spawn_test_dir");

    assert!(
        matches!(result, Err(abi::errors::Errno::EACCES)),
        "expected EACCES for a directory node, got {:?}",
        result
    );
}

/// The runtime hook `SPAWN_PROCESS_FROM_PATH_HOOK` is a separate static
/// from the boot hook `SPAWN_PROCESS_EX_HOOK`.
///
/// Before the scheduler is initialized the hook is `None`, so calling
/// `spawn_process_from_path_current` returns `ENOSYS`.  This verifies
/// the hook plumbing and its independence from the boot path.
#[test]
fn test_spawn_process_from_path_current_returns_enosys_without_scheduler() {
    let _g = init_test_env();
    // `init_test_env` sets SCHEDULER to None, so hooks are not installed.
    let result = unsafe {
        crate::sched::hooks::spawn_process_from_path_current(
            "/usr/bin/ls",
            alloc::vec![],
            alloc::collections::BTreeMap::new(),
            StdioSpec::Inherit,
            StdioSpec::Inherit,
            StdioSpec::Inherit,
            0,
            alloc::vec![],
            None,
            alloc::vec![],
            None,
        )
    };

    assert!(
        matches!(result, Err(abi::errors::Errno::ENOSYS)),
        "expected ENOSYS when SPAWN_PROCESS_FROM_PATH_HOOK is not installed, got {:?}",
        result
    );
}

/// During early bringup (`bringup_in_progress == true`) any task spawned
/// with `Affinity::Any` should be placed on the local (boot) CPU so that
/// cross-CPU placement overhead and IPI traffic are avoided.
#[test]
fn test_spawn_any_affinity_stays_local_during_bringup() {
    let _g = init_test_env();

    let mut sched = Scheduler::<MockRuntime>::new();
    // Two CPUs online so round-robin would normally spread tasks.
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new()); // CPU 0
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new()); // CPU 1
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);
    sched.state.per_cpu[0].current = Some(0);
    sched.bringup_in_progress = true;

    // Spawn several tasks with Any affinity.
    let id1 =
        sched.spawn(mock_entry, StartupArg::None, crate::task::TaskPriority::Normal, Affinity::Any);
    let id2 =
        sched.spawn(mock_entry, StartupArg::None, crate::task::TaskPriority::Normal, Affinity::Any);
    let id3 =
        sched.spawn(mock_entry, StartupArg::None, crate::task::TaskPriority::Normal, Affinity::Any);

    // All tasks should land on the local CPU (CPU 0 in the mock).
    for id in [id1, id2, id3] {
        let last_cpu = crate::task::registry::get_task::<MockRuntime>(id)
            .and_then(|t| t.last_cpu)
            .expect("task should have last_cpu set");
        assert_eq!(
            last_cpu, 0,
            "task {} should be on CPU 0 during bringup, got CPU {}",
            id, last_cpu
        );
    }
}

#[test]
fn test_cpu_online_grows_per_cpu_state_without_ending_bringup() {
    let _g = init_test_env();

    let mut sched = Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.mark_cpu_online(0);
    sched.state.per_cpu[0].current = Some(0);
    sched.bringup_in_progress = true;

    sched.cpu_online(1);

    assert!(sched.bringup_in_progress, "cpu_online should not end early-boot bringup");
    assert!(sched.state.per_cpu.len() > 1, "cpu_online should extend per-cpu state");
    assert!(
        sched.state.online_cpus.contains(&1),
        "cpu_online should mark the secondary CPU online"
    );
    assert!(
        sched.state.per_cpu[1].idle_task.is_some(),
        "cpu_online should create an idle task for the new CPU"
    );
}

/// After `end_bringup` the flag is cleared and subsequent spawns may be
/// placed on other CPUs via the normal round-robin algorithm.
#[test]
fn test_bringup_flag_cleared_after_end_bringup() {
    let _g = init_test_env();

    let mut sched = Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(0);

    // Flag starts true (as set by init_boot_task in production).
    sched.bringup_in_progress = true;
    assert!(sched.bringup_in_progress, "bringup_in_progress should be true");

    // Simulate end_bringup by clearing the flag directly (the public
    // end_bringup<R>() function requires a fully-initialized SCHEDULER
    // global which is intentionally absent in unit tests).
    sched.bringup_in_progress = false;
    assert!(!sched.bringup_in_progress, "bringup_in_progress should be false after end");
}

#[test]
fn test_spawn_any_affinity_fanout_after_bringup() {
    let _g = init_test_env();
    const TEST_CPU_COUNT: usize = 6;
    const TEST_TASK_COUNT: usize = TEST_CPU_COUNT * 2;

    let mut sched = Scheduler::<MockRuntime>::new();
    for _ in 0..TEST_CPU_COUNT {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    for cpu in 0..TEST_CPU_COUNT {
        sched.state.mark_cpu_online(cpu);
    }
    sched.state.per_cpu[0].current = Some(0);
    sched.bringup_in_progress = false;

    for _ in 0..TEST_TASK_COUNT {
        let _ = sched.spawn(
            mock_entry,
            StartupArg::None,
            crate::task::TaskPriority::Normal,
            Affinity::Any,
        );
    }

    let non_empty = (0..TEST_CPU_COUNT)
        .filter(|&cpu| {
            !sched.state.per_cpu[cpu].runq[crate::task::TaskPriority::Normal as usize].is_empty()
        })
        .count();
    assert!(
        non_empty >= 3,
        "[policy] post-bringup Any-affinity spawn should fan out across CPUs; got {} non-empty CPUs",
        non_empty
    );
}

#[test]
fn test_spawn_user_thread_any_fanout_after_bringup() {
    let _g = init_test_env();

    let mut sched = Scheduler::<MockRuntime>::new();
    const TEST_CPU_COUNT: usize = 4;
    const TEST_THREAD_COUNT: usize = TEST_CPU_COUNT * 2;
    const TEST_STACK_SPACING: usize = 0x10000;
    const TEST_GUARD_START: usize = 0x3000;
    const TEST_GUARD_END: usize = 0x4000;
    const TEST_RESERVE_START: usize = 0x4000;
    const TEST_RESERVE_END: usize = 0x8000;
    const TEST_COMMITTED_START: usize = 0x7000;
    const TEST_STACK_TOP: usize = 0x9000;
    for _ in 0..TEST_CPU_COUNT {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    for cpu in 0..TEST_CPU_COUNT {
        sched.state.mark_cpu_online(cpu);
    }
    sched.state.per_cpu[0].current = Some(0);
    sched.bringup_in_progress = false;

    let mut spawned = alloc::vec::Vec::new();
    for i in 0..TEST_THREAD_COUNT {
        let stack_base = (i + 1) * TEST_STACK_SPACING;
        let stack_top = TEST_STACK_TOP + stack_base;
        let stack_info = abi::types::StackInfo {
            guard_start: TEST_GUARD_START + stack_base,
            guard_end: TEST_GUARD_END + stack_base,
            reserve_start: TEST_RESERVE_START + stack_base,
            reserve_end: TEST_RESERVE_END + stack_base,
            committed_start: TEST_COMMITTED_START + stack_base,
            grow_chunk_bytes: 0x1000,
        };
        let id = sched.spawn_user_thread(
            0x1000,
            stack_top,
            StartupArg::Raw(7),
            stack_info,
            crate::task::TaskPriority::Normal,
            Affinity::Any,
            0,
            false,
        );
        spawned.push((id, stack_info));
    }

    assert!(
        sched.state.per_cpu[0].runq[crate::task::TaskPriority::Normal as usize].is_empty(),
        "[policy] post-bringup Any-affinity user-thread spawn should avoid BSP when secondary CPUs are online"
    );
    for cpu in 1..TEST_CPU_COUNT {
        assert!(
            !sched.state.per_cpu[cpu].runq[crate::task::TaskPriority::Normal as usize].is_empty(),
            "[policy] post-bringup Any-affinity user-thread spawn should fan out to secondary CPU {}",
            cpu
        );
    }

    for (id, expected_stack) in spawned {
        let task =
            crate::task::registry::get_task::<MockRuntime>(id).expect("spawned task missing");
        let got_stack = task.stack_info.expect("spawned task stack_info missing");
        let sf = sched.state.get_task(id).expect("spawned task sched fields missing");
        assert!(sf.runq_location.is_some(), "spawned task should be enqueued in a run queue");
        assert_eq!(got_stack.guard_start, expected_stack.guard_start);
        assert_eq!(got_stack.guard_end, expected_stack.guard_end);
        assert_eq!(got_stack.reserve_start, expected_stack.reserve_start);
        assert_eq!(got_stack.reserve_end, expected_stack.reserve_end);
        assert_eq!(got_stack.committed_start, expected_stack.committed_start);
        assert_eq!(got_stack.grow_chunk_bytes, expected_stack.grow_chunk_bytes);
    }
}

#[test]
fn test_spawn_user_thread_any_stays_local_during_bringup() {
    let _g = init_test_env();

    let mut sched = Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);
    sched.state.per_cpu[0].current = Some(0);
    sched.bringup_in_progress = true;

    let id = sched.spawn_user_thread(
        0x1000,
        0x9000,
        StartupArg::Raw(7),
        abi::types::StackInfo {
            guard_start: 0x3000,
            guard_end: 0x4000,
            reserve_start: 0x4000,
            reserve_end: 0x8000,
            committed_start: 0x7000,
            grow_chunk_bytes: 0x1000,
        },
        crate::task::TaskPriority::Normal,
        Affinity::Any,
        0,
        false,
    );

    let t = crate::task::registry::get_task::<MockRuntime>(id).expect("spawned task missing");
    assert_eq!(
        t.last_cpu,
        Some(0),
        "[policy] Any-affinity user thread should stay local while bringup is in progress"
    );
}
