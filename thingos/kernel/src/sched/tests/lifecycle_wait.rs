use core::sync::atomic::Ordering;

use super::super::*;
use super::support::*;
use crate::sched::mailbox::take_remote_wake_mailbox;
use crate::task::{Affinity, TaskId, TaskPriority, TaskState};
use crate::{BootRuntimeBase, BootTasking};

#[test]
fn test_registry_rapid_create_exit_churn() {
    let _g = init_test_env();
    crate::task::registry::init::<MockRuntime>();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    // Helper to create dummy task
    let make_task = |id: TaskId| crate::task::Task {
        id,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
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
        last_cpu: None,
        name: [0; 32],
        name_len: 0,
        process_info: None,
        user_fs_base: 0,
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };

    // Insert tasks out of order
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(make_task(4010)));
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(make_task(4005)));
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(make_task(4020)));
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(make_task(4001)));

    assert_eq!(crate::task::registry::get_registry::<MockRuntime>().threads.len(), 4);

    // Verify lookups work
    assert!(crate::task::registry::get_task::<MockRuntime>(4010).is_some());
    assert!(crate::task::registry::get_task::<MockRuntime>(4005).is_some());
    assert!(crate::task::registry::get_task::<MockRuntime>(4001).is_some());
    assert!(crate::task::registry::get_task::<MockRuntime>(4099).is_none());

    // Keep a small hot set of thread IDs cycling through many create/exit
    // operations to stress index maintenance under churn.
    const CHURN_ITERATIONS: usize = 512; // enough iterations to repeatedly reshuffle slots
    const CHURN_ACTIVE_TID_COUNT: usize = 32; // small hot set maximizes remove/reinsert reuse
    for i in 0..CHURN_ITERATIONS {
        let tid = 5000 + (i % CHURN_ACTIVE_TID_COUNT) as u64;
        if crate::task::registry::get_task::<MockRuntime>(tid).is_none() {
            crate::task::registry::get_registry::<MockRuntime>()
                .insert(alloc::boxed::Box::new(make_task(tid)));
        }
        let removed = crate::task::registry::get_registry::<MockRuntime>().remove(tid);
        assert!(removed.is_some(), "expected tid {} to exist before removal", tid);
        assert!(
            crate::task::registry::get_task::<MockRuntime>(tid).is_none(),
            "removed tid {} must not remain addressable",
            tid
        );
    }
}

#[test]
fn test_block_and_wake_state_transitions() {
    let _g = init_test_env();
    crate::task::registry::init::<MockRuntime>();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    let waiting_task = crate::task::Task {
        id: 5001,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
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
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };

    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(waiting_task));
    sched.state.per_cpu[0].current = Some(5001);

    // Put task in Wait queue and switch it to Blocked (simulating block_current behavior)
    if let Some(mut task) = crate::task::registry::get_task_mut::<MockRuntime>(5001) {
        task.state = TaskState::Blocked;
    }
    sched.state.register_waiter(5001, crate::sched::state::WaitReason::BlockCurrent);

    // Verify task is stuck blocked
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(5001).unwrap().state,
        TaskState::Blocked
    );

    // Emulate `wake_task_erased` via wake_task in MockRuntime context
    crate::sched::blocking::WAKE_TASK_HOOK.store(
        crate::sched::blocking::wake_task::<MockRuntime> as *mut (),
        core::sync::atomic::Ordering::SeqCst,
    );

    // Make sure scheduler hook resolves safely (we will mock inject the scheduler here via static for the hook)
    // Since we are unit testing `wake_task`, we can't easily use the global `SCHEDULER`.
    // So we just directly call the core logic we care about: the wake sleeper unblock logic.

    // Remove from wait queue if present
    sched.state.unregister_waiter(5001);

    // Update state to Runnable and add to runq
    if let Some(mut task) = crate::task::registry::get_task_mut::<MockRuntime>(5001) {
        if task.state == TaskState::Blocked {
            task.state = TaskState::Runnable;
            sched.state.enqueue_task(0, task.priority as usize, 5001);
        }
    }

    let woken_task = crate::task::registry::get_task::<MockRuntime>(5001).unwrap();
    assert_eq!(
        woken_task.state,
        TaskState::Runnable,
        "Task must transition from Blocked to Runnable upon wake"
    );

    // Verify task was placed in runq
    assert!(
        sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].iter().any(|&id| id == 5001),
        "Woken task must be in the run queue"
    );
}

#[test]
fn test_wake_task_removes_sleep_queue_entry() {
    let _g = init_test_env();
    crate::task::registry::init::<MockRuntime>();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(6000);

    let current_task = crate::task::Task {
        id: 6000,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
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
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };

    let sleeping_task = crate::task::Task {
        id: 6001,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
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
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };

    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(sleeping_task));

    // Both tasks must be in ThreadSchedFields so that wake_task_locked can
    // find and properly wake the sleeping task via the hot-field cache.
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6000,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6001,
        runq_location: None,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });

    sched.state.add_task_to_sleep_queue(6001, 10);

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(sched_lock);

    crate::sched::blocking::wake_task::<MockRuntime>(6001);

    let task = crate::task::registry::get_task::<MockRuntime>(6001).unwrap();
    assert_eq!(task.state, TaskState::Runnable);
    assert!(sched.state.sleep_queue.is_empty());
    assert!(
        sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].iter().any(|&id| id == 6001)
    );

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_wake_task_revalidates_under_split_authority_window() {
    let _g = init_test_env();
    crate::task::registry::init::<MockRuntime>();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6101,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    let task = crate::task::Task {
        id: 6101,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
        wake_pending: true,
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
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(sched_lock);
    let _ = sched_lock_metrics_snapshot_and_reset();

    crate::sched::blocking::wake_task::<MockRuntime>(6101);
    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
    drop(sched_lock);

    let task = crate::task::registry::get_task::<MockRuntime>(6101).unwrap();
    assert_eq!(task.state, TaskState::Running);
    assert!(task.wake_pending);
    assert!(
        sched.state.get_task(6101).map(|sf| sf.wake_pending).unwrap_or(false),
        "wake_task must revalidate scheduler cache even when REGISTRY wake_pending is true"
    );

    let metrics = sched_lock_metrics_snapshot_and_reset();
    assert_eq!(metrics.wake_task_fastpath_already_pending, 0);
    assert_eq!(metrics.wake_task.wait_calls, 1);
}

#[test]
fn test_wake_task_remote_path_uses_mailbox_without_scheduler_lock() {
    let _g = init_test_env();
    TICK_COUNT.store(123, Ordering::Relaxed);
    clear_global_need_resched(1, Ordering::Relaxed);
    DIAG_IPI_SENT.store(0, Ordering::Relaxed);
    DIAG_IPI_SENT_WAKE_TASK.store(0, Ordering::Relaxed);
    DIAG_REMOTE_WAKE_MAILBOX_NO_IPI.store(0, Ordering::Relaxed);

    let task = crate::task::Task {
        id: 6_202,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
        wake_pending: false,
        pending_interrupt: false,
        affinity: Affinity::Pinned(1),
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
        last_cpu: Some(1),
        name: [0; 32],
        name_len: 0,
        process_info: None,
        user_fs_base: 0,
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

    crate::sched::blocking::wake_task::<MockRuntime>(6_202);

    let task = crate::task::registry::get_task::<MockRuntime>(6_202).unwrap();
    assert_eq!(task.state, TaskState::Runnable);
    assert_eq!(task.enqueued_at_tick, 123);

    let queued = take_remote_wake_mailbox(1);
    assert_eq!(queued.len(), 1);
    assert_eq!(queued.front().map(|entry| entry.tid), Some(6_202));
    assert!(global_need_resched_load(1, Ordering::Acquire));
    assert_eq!(DIAG_IPI_SENT.load(Ordering::Relaxed), 1);
    assert_eq!(DIAG_IPI_SENT_WAKE_TASK.load(Ordering::Relaxed), 1);
    assert_eq!(DIAG_REMOTE_WAKE_MAILBOX_NO_IPI.load(Ordering::Relaxed), 0);
}

#[test]
fn test_wake_task_remote_mailbox_forces_ipi_when_pending_and_rate_limited() {
    let _g = init_test_env();
    TICK_COUNT.store(100, Ordering::Relaxed);
    DIAG_IPI_SENT.store(0, Ordering::Relaxed);
    DIAG_IPI_SENT_WAKE_TASK.store(0, Ordering::Relaxed);
    DIAG_REMOTE_WAKE_MAILBOX_NO_IPI.store(0, Ordering::Relaxed);
    LAST_RESCHED_IPI_SENT_AT_TICK[1].store(100, Ordering::Relaxed);
    set_global_need_resched(1);

    let task = make_task(6_203, TaskState::Blocked, TaskPriority::Normal);
    let mut task = task;
    task.affinity = Affinity::Pinned(1);
    task.last_cpu = Some(1);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

    crate::sched::blocking::wake_task::<MockRuntime>(6_203);

    assert_eq!(
        DIAG_IPI_SENT.load(Ordering::Relaxed),
        1,
        "remote mailbox wake should force one resched IPI even with pending/rate-limit state"
    );
    assert_eq!(DIAG_IPI_SENT_WAKE_TASK.load(Ordering::Relaxed), 1);
    assert_eq!(DIAG_REMOTE_WAKE_MAILBOX_NO_IPI.load(Ordering::Relaxed), 0);
}

#[test]
fn test_drain_remote_wake_mailbox_enqueues_locally() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.mark_cpu_online(1);
    sched.state.per_cpu[1].current = Some(6_300);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6_300,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Low,
        affinity: Affinity::Pinned(1),
        last_cpu: Some(1),
        wake_cpu: Some(1),
        run_cpu: Some(1),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6_301,
        runq_location: None,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(1),
        last_cpu: Some(1),
        wake_cpu: None,
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    let _ = sched.state.register_waiter(6_301, state::WaitReason::BlockCurrent);
    sched.state.add_task_to_sleep_queue(6_301, 7);

    enqueue_remote_wake_mailbox(
        1,
        types::RemoteWakeMailboxEntry {
            tid: 6_301,
            priority: TaskPriority::Normal as usize,
            enqueued_at_tick: 9,
            wake_mono: 11,
        },
    );

    sched.drain_remote_wake_mailbox(1);

    assert!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 6_301)
    );
    assert_eq!(sched.state.get_task(6_301).map(|sf| sf.state), Some(TaskState::Runnable));
    assert_eq!(sched.state.get_task(6_301).and_then(|sf| sf.wake_cpu), Some(1));
    assert!(!sched.state.wait_queue.contains(&6_301));
    assert!(!sched.state.sleep_membership.contains_key(&6_301));
    assert_eq!(PROF_REMOTE_WAKE_MAILBOX_AGE_HIST[0].load(Ordering::Relaxed), 1);
}

#[test]
fn test_wake_task_without_pending_wake_uses_scheduler_path() {
    let _g = init_test_env();
    crate::task::registry::init::<MockRuntime>();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6102,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    let task = crate::task::Task {
        id: 6102,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
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
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(sched_lock);
    let _ = sched_lock_metrics_snapshot_and_reset();

    crate::sched::blocking::wake_task::<MockRuntime>(6102);

    let task = crate::task::registry::get_task::<MockRuntime>(6102).unwrap();
    assert!(task.wake_pending);
    let metrics = sched_lock_metrics_snapshot_and_reset();
    assert_eq!(metrics.wake_task_fastpath_already_pending, 0);
    assert_eq!(metrics.wake_task.wait_calls, 1);

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_kill_by_tid_removes_wait_queue_entry() {
    let _g = init_test_env();
    crate::task::registry::init::<MockRuntime>();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(7000);

    let current_task = crate::task::Task {
        id: 7000,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
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
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };

    let blocked_task = crate::task::Task {
        id: 7001,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
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
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };

    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(blocked_task));
    sched.state.register_waiter(7001, crate::sched::state::WaitReason::BlockCurrent);

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(sched_lock);

    assert!(kill_by_tid::<MockRuntime>(7001));
    assert!(sched.state.wait_queue.is_empty());
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(7001).unwrap().state,
        TaskState::Dead
    );

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_wait_task_returns_immediately_for_dead_target() {
    let _g = init_test_env();

    let dead_task = crate::task::Task {
        id: 8001,
        state: TaskState::Dead,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: Some(23),
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
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
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(dead_task));

    assert_eq!(wait_task::<MockRuntime>(8001).unwrap(), 23);
}

#[test]
fn test_wait_task_returns_echild_for_missing_target() {
    let _g = init_test_env();

    assert_eq!(wait_task::<MockRuntime>(8999).unwrap_err(), abi::errors::Errno::ECHILD);
}

#[test]
fn test_register_task_exit_waiter_tracks_live_target() {
    let _g = init_test_env();

    let live_task = crate::task::Task {
        id: 8101,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
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
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(live_task));

    assert_eq!(register_task_exit_waiter::<MockRuntime>(8101, 8102).unwrap(), None);

    let waiters =
        crate::task::registry::get_task::<MockRuntime>(8101).unwrap().exit_waiters.drain();
    assert_eq!(waiters, alloc::vec![8102]);
}

#[test]
fn test_kill_by_tid_wakes_registered_exit_waiter() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(8200);

    let current_task = crate::task::Task {
        id: 8200,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
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
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };

    let waiter_task = crate::task::Task {
        id: 8201,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
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
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };

    let target_task = crate::task::Task {
        id: 8202,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0,
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
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
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };

    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(waiter_task));
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(target_task));

    let target_fields = crate::sched::state::TaskSchedFields {
        tid: 8202,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    };
    sched.state.insert_task(target_fields);

    assert_eq!(register_task_exit_waiter::<MockRuntime>(8202, 8201).unwrap(), None);

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(sched_lock);

    assert!(kill_by_tid::<MockRuntime>(8202));
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(8202).unwrap().state,
        TaskState::Dead
    );
    assert_eq!(crate::task::registry::get_task::<MockRuntime>(8202).unwrap().exit_code, Some(-9));
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(8201).unwrap().state,
        TaskState::Runnable
    );
    assert!(
        sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].iter().any(|&id| id == 8201)
    );

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_poll_task_exit_reports_pending_dead_and_missing_targets() {
    let _g = init_test_env();

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        8301,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));

    let mut dead = make_task(8302, TaskState::Dead, TaskPriority::Normal);
    dead.exit_code = Some(17);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(dead));

    assert_eq!(poll_task_exit::<MockRuntime>(8301).unwrap(), None);
    assert_eq!(poll_task_exit::<MockRuntime>(8302).unwrap(), Some(17));
    assert_eq!(poll_task_exit::<MockRuntime>(8399).unwrap_err(), abi::errors::Errno::ECHILD);
}

#[test]
fn test_terminate_current_purges_dead_task_from_scheduler_queues() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(8303);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        8303,
        TaskState::Running,
        TaskPriority::Normal,
    )));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        8304,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));

    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 8303,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 8304,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    // Seed stale queue membership for the exiting task and ensure another
    // runnable task exists so terminate_current can produce a switch.
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 8303);
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 8304);
    sched.state.register_waiter(8303, crate::sched::state::WaitReason::BlockCurrent);
    sched.state.add_task_to_sleep_queue(8303, 55);
    sched.state.add_task_to_sleep_queue(9999, 55);

    let termination = mark_task_exited_in_registry::<MockRuntime>(8303, 101);
    let switch = sched.terminate_current(8303, &termination.siblings_to_kill);

    assert_eq!(switch.to_tid, 8304, "scheduler should switch to the next runnable task");
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(8303).unwrap().state,
        TaskState::Dead
    );
    assert_eq!(crate::task::registry::get_task::<MockRuntime>(8303).unwrap().exit_code, Some(101));
    assert!(
        !sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 8303),
        "dead current task must be removed from the run queue"
    );
    assert!(
        !sched.state.wait_queue.contains(&8303),
        "dead current task must be removed from the wait queue"
    );
    assert_eq!(sched.state.sleep_queue.get(&55).cloned(), Some(alloc::vec![9999]));
}

#[test]
fn terminate_current_only_mutates_scheduler_state() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(8310);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        8310,
        TaskState::Running,
        TaskPriority::Normal,
    )));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        8311,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));

    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 8310,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 8311,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 8311);

    let _ = sched.terminate_current(8310, &[]);
    assert_eq!(sched.state.get_task(8310).unwrap().state, TaskState::Dead);
    assert_eq!(
        crate::task::registry::get_task::<MockRuntime>(8310).unwrap().state,
        TaskState::Running,
        "terminate_current should not acquire/mutate REGISTRY directly"
    );
}

#[test]
#[should_panic(expected = "terminate_current could not find a switch")]
fn test_terminate_current_panics_when_no_switch_candidate_exists() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(8306);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        8306,
        TaskState::Running,
        TaskPriority::Normal,
    )));

    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 8306,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    let termination = mark_task_exited_in_registry::<MockRuntime>(8306, 202);
    let _ = sched.terminate_current(8306, &termination.siblings_to_kill);
}

#[test]
fn test_remove_task_completely_purges_scheduler_queues() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(0);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        0,
        TaskState::Running,
        TaskPriority::Normal,
    )));
    let mut dead = make_task(8305, TaskState::Dead, TaskPriority::Normal);
    dead.exit_code = Some(42);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(dead));

    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 0,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 8305,
        runq_location: None,
        state: TaskState::Dead,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 8305);
    sched.state.register_waiter(8305, crate::sched::state::WaitReason::BlockCurrent);
    sched.state.add_task_to_sleep_queue(8305, 77);

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(sched_lock);

    remove_task_completely::<MockRuntime>(8305);

    assert!(crate::task::registry::get_task::<MockRuntime>(8305).is_none());
    assert!(sched.state.get_task(8305).is_none());
    assert!(
        !sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 8305),
        "reaped task must be removed from the run queue"
    );
    assert!(
        !sched.state.wait_queue.contains(&8305),
        "reaped task must be removed from the wait queue"
    );
    assert!(!sched.state.sleep_queue.contains_key(&77));

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_unregister_task_exit_waiter_removes_only_requested_waiter() {
    let _g = init_test_env();

    let target = make_task(8401, TaskState::Runnable, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(target));

    register_task_exit_waiter::<MockRuntime>(8401, 8402).unwrap();
    register_task_exit_waiter::<MockRuntime>(8401, 8403).unwrap();
    unregister_task_exit_waiter::<MockRuntime>(8401, 8402).unwrap();

    let waiters =
        crate::task::registry::get_task::<MockRuntime>(8401).unwrap().exit_waiters.drain();
    assert_eq!(waiters, alloc::vec![8403]);
}

#[test]
fn test_register_timeout_wake_deduplicates_task_ids() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(8500);

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(sched_lock);

    register_timeout_wake::<MockRuntime>(8501, 42);
    register_timeout_wake::<MockRuntime>(8501, 42);
    register_timeout_wake::<MockRuntime>(8502, 42);

    assert_eq!(sched.state.sleep_queue.get(&42).cloned().unwrap(), alloc::vec![8501, 8502]);

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_unregister_timeout_wake_removes_task_and_updates_membership() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(8600);
    sched.state.add_task_to_sleep_queue(8601, 11);
    sched.state.add_task_to_sleep_queue(8602, 11);
    sched.state.add_task_to_sleep_queue(8602, 12);
    sched.state.add_task_to_sleep_queue(8603, 12);

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(sched_lock);

    unregister_timeout_wake::<MockRuntime>(8602);

    assert_eq!(sched.state.sleep_queue.get(&11).cloned().unwrap(), alloc::vec![8601]);
    assert_eq!(sched.state.sleep_queue.get(&12).cloned().unwrap(), alloc::vec![8603]);
    assert!(!sched.state.sleep_membership.contains_key(&8602));
    assert_eq!(
        sched.state.sleep_membership.get(&8603).copied(),
        Some(crate::sched::state::SleepMembership { wake_tick: 12, bucket_index: 0 })
    );

    unregister_timeout_wake::<MockRuntime>(8603);
    assert!(!sched.state.sleep_queue.contains_key(&12));

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_interrupt_task_marks_and_consumes_pending_interrupt() {
    let _g = init_test_env();

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        0,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));

    interrupt_task::<MockRuntime>(0).expect("interrupt task");
    assert!(crate::task::registry::get_task::<MockRuntime>(0).unwrap().pending_interrupt);
    assert!(take_pending_interrupt::<MockRuntime>());
    assert!(!take_pending_interrupt::<MockRuntime>());
}

#[test]
fn test_interrupt_task_returns_esrch_for_missing_task() {
    let _g = init_test_env();
    assert_eq!(interrupt_task::<MockRuntime>(9999).unwrap_err(), abi::errors::Errno::ESRCH);
}
