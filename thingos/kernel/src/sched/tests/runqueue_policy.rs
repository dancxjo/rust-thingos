use core::sync::atomic::Ordering;

use super::super::*;
use super::support::*;
use crate::task::{Affinity, TaskId, TaskPriority, TaskState};
use crate::{BootRuntimeBase, BootTasking};

#[test]
fn test_priority_aging_boost() {
    let _g = init_test_env();
    // Test that tasks waiting too long get priority boost when scheduling
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(0); // Dummy current task

    // The dummy current task must be in BOTH the global registry and the
    // scheduler's local sched-state so that `do_context_switch` can index
    // into both consistently (both vectors are sorted by TID).
    let dummy_current = make_task(0, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(dummy_current));
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
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });
    let task_normal = crate::task::Task {
        id: 1001,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 600, // Recent (now - 600 = 400 < AGING_THRESHOLD, so no boost)
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

    // Create a low-priority task enqueued a long time ago
    let task_low = crate::task::Task {
        id: 1002,
        state: TaskState::Runnable,
        priority: TaskPriority::Low,
        base_priority: TaskPriority::Low,
        enqueued_at_tick: 0, // Very old
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
        .insert(alloc::boxed::Box::new(task_normal));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task_low));

    // Scheduler state entries are separate from the global registry and must
    // be inserted explicitly so `prepare_schedule` can locate them.
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 1001,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        // Must match `task_normal.enqueued_at_tick` above (600) so the
        // hot-field cache reflects the correct wait time for aging.
        enqueued_at_tick: 600,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 1002,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Low,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        // Must match `task_low.enqueued_at_tick` above (0) so the
        // hot-field cache reflects the correct wait time for aging.
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 1001);
    sched.state.enqueue_task(0, TaskPriority::Low as usize, 1002);

    // Simulate time advancing enough to give the Low task a boost of +2 (eff = High=3),
    // while the Normal task, enqueued at tick 600, only waits 400 ticks → no boost (eff = Normal=2).
    let now = types::AGING_THRESHOLD_TICKS * 2;
    TICK_COUNT.store(now, core::sync::atomic::Ordering::Relaxed);

    // Request schedule. The Low task should be selected because its effective priority is higher
    // than Normal due to wait time.
    let next_switch = sched.prepare_schedule().expect("Should find a task");
    assert_eq!(next_switch.to_tid, 1002, "Low priority task with aging should preempt normal task");

    // Verify it was popped from the Low queue, not moved to High queue
    assert!(sched.state.per_cpu[0].runq[TaskPriority::Low as usize].is_empty());
    assert!(!sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].is_empty());
}

#[test]
fn test_vruntime_prefers_least_served_on_effective_priority_tie() {
    let _g = init_test_env();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(0);

    let dummy_current = make_task(0, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(dummy_current));
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
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });

    let mut task_a = make_task(4001, TaskState::Runnable, TaskPriority::Normal);
    task_a.enqueued_at_tick = 0;
    let mut task_b = make_task(4002, TaskState::Runnable, TaskPriority::Normal);
    task_b.enqueued_at_tick = 0;
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task_a));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task_b));

    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 4001,
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
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 4002,
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
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 4001);
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 4002);

    sched.state.task_runtime_stats_mut(4001).fair_vruntime = 100;
    sched.state.task_runtime_stats_mut(4002).fair_vruntime = 10;

    let next_switch = sched.prepare_schedule().expect("Should pick a runnable task");
    assert_eq!(
        next_switch.to_tid, 4002,
        "least-served task should be preferred when effective priority ties"
    );
}

#[test]
fn test_reset_priority_aging_on_schedule() {
    let _g = init_test_env();
    // Test that enqueued_at_tick resets when task is preempted/yields
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    // Task 1 is running
    let mut task1 = crate::task::Task {
        id: 2001,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 0, // Very old
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
        timeslice_remaining: 0, // timeslice expired
        last_cpu: Some(0),
        name: [0; 32],
        name_len: 0,
        process_info: None,
        user_fs_base: 0,
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };

    // Task 2 is runnable
    let task2 = crate::task::Task {
        id: 2002,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        base_priority: TaskPriority::Normal,
        enqueued_at_tick: 500, // Newer
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

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task1));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task2));

    sched.state.per_cpu[0].current = Some(2001);
    // Scheduler state entries for both tasks.
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 2001,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 2002,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 500,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 2002);

    // Time moves forward
    TICK_COUNT.store(1000, core::sync::atomic::Ordering::Relaxed);

    // Trigger a timer tick to cause preemption
    let switch = sched.prepare_yield().expect("Should preempt to task2");
    assert_eq!(switch.to_tid, 2002);
    let t1_before_sync = crate::task::registry::get_task::<MockRuntime>(2001).unwrap();
    assert_eq!(
        t1_before_sync.enqueued_at_tick, 0,
        "prepare_yield should defer REGISTRY sync out of prepare_schedule hot path"
    );
    assert_eq!(
        t1_before_sync.state,
        TaskState::Running,
        "REGISTRY state should remain unchanged until deferred sync is applied"
    );
    apply_deferred_registry_syncs::<MockRuntime>(core::mem::take(
        &mut sched.pending_registry_syncs,
    ));

    // Verify task1 was placed back in runq and its enqueued_at_tick was updated to TICK_COUNT
    let t1 = crate::task::registry::get_task::<MockRuntime>(2001).unwrap();
    assert_eq!(t1.enqueued_at_tick, 1000);
    assert_eq!(t1.state, TaskState::Runnable);
}

#[test]
fn test_prepare_yield_keeps_current_runnable_when_no_peer_exists() {
    let _g = init_test_env();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        7001,
        TaskState::Running,
        TaskPriority::Normal,
    )));

    sched.state.per_cpu[0].current = Some(7001);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 7001,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });

    let switch = sched.prepare_yield();

    assert!(switch.is_none(), "yield with no runnable peer should keep running current task");
    let sf = sched.state.get_task(7001).expect("current task should remain in scheduler state");
    assert_eq!(sf.state, TaskState::Running);
    assert_eq!(sf.runq_location, None);
    assert_eq!(sched.state.per_cpu[0].current, Some(7001));
}

#[test]
fn test_prepare_yield_penalizes_spin_yield_requeue_band() {
    let _g = init_test_env();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        6001,
        TaskState::Running,
        TaskPriority::High,
    )));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        6002,
        TaskState::Runnable,
        TaskPriority::Realtime,
    )));

    sched.state.per_cpu[0].current = Some(6001);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6001,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::High,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: types::SPIN_YIELD_PENALTY_THRESHOLD,
        migration_state: MigrationState::Local,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6002,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Realtime,
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
    sched.state.note_enqueue_cause(6001, crate::sched::state::EnqueueCause::YieldRequeue);
    sched.state.enqueue_task(0, TaskPriority::Realtime as usize, 6002);

    let switch = sched.prepare_yield().expect("high-priority peer should run");
    assert_eq!(switch.to_tid, 6002);
    assert_eq!(
        sched.state.get_task(6001).and_then(|sf| sf.runq_location),
        Some((0, TaskPriority::Normal as usize)),
        "spin-yielding task should be demoted one runnable band on requeue",
    );
    assert_eq!(
        sched.state.get_task(6001).map(|sf| sf.voluntary_yields),
        Some(types::SPIN_YIELD_PENALTY_THRESHOLD + 1),
    );
}

#[test]
fn test_prepare_yield_does_not_penalize_without_prior_yield_requeue() {
    let _g = init_test_env();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        6101,
        TaskState::Running,
        TaskPriority::Normal,
    )));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        6102,
        TaskState::Runnable,
        TaskPriority::High,
    )));

    sched.state.per_cpu[0].current = Some(6101);
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
        voluntary_yields: types::SPIN_YIELD_PENALTY_THRESHOLD + 5,
        migration_state: MigrationState::Local,
    });
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 6102,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::High,
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
    sched.state.note_enqueue_cause(6101, crate::sched::state::EnqueueCause::Wake);
    sched.state.enqueue_task(0, TaskPriority::High as usize, 6102);

    let switch = sched.prepare_yield().expect("high-priority peer should run");
    assert_eq!(switch.to_tid, 6102);
    assert_eq!(
        sched.state.get_task(6101).and_then(|sf| sf.runq_location),
        Some((0, TaskPriority::Normal as usize)),
        "without prior yield-requeue cause, no spin penalty should be applied",
    );
}
