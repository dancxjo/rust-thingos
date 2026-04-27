use core::sync::atomic::Ordering;

use super::super::*;
use super::support::*;
use crate::task::{Affinity, TaskId, TaskPriority, TaskState};
use crate::{BootRuntimeBase, BootTasking};

#[test]
fn test_wake_preempts_lower_priority() {
    let _g = init_test_env();
    crate::task::registry::init::<MockRuntime>();
    use core::sync::atomic::Ordering;

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    // Task 1: Normal priority, currently running
    let normal_task = crate::task::Task {
        id: 3001,
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

    // Task 2: Realtime priority, sleeping (about to wake)
    let rt_task = crate::task::Task {
        id: 3002,
        state: TaskState::Runnable,
        priority: TaskPriority::Realtime,
        base_priority: TaskPriority::Realtime,
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
        .insert(alloc::boxed::Box::new(normal_task));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(rt_task));
    sched.state.per_cpu[0].current = Some(3001); // Normal task is running
    // Scheduler state entries for both tasks.
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 3001,
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
        tid: 3002,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Realtime,
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

    // Put RT task in sleep queue with wake_tick in the past
    TICK_COUNT.store(100, Ordering::Relaxed);
    sched.state.add_task_to_sleep_queue(3002, 50);

    // Before: need_resched should be false
    assert!(!sched.state.per_cpu[0].need_resched, "need_resched should start false");

    // Wake sleepers — should detect RT > Normal and set need_resched
    sched.wake_sleepers();

    // Verify need_resched was set
    assert!(
        sched.state.per_cpu[0].need_resched,
        "need_resched should be true after waking a higher-priority task"
    );

    // Verify RT task was enqueued to the Realtime runq
    assert!(
        sched.state.per_cpu[0].runq[TaskPriority::Realtime as usize].iter().any(|&id| id == 3002),
        "RT task should be in the Realtime run queue"
    );

    // Now simulate schedule: prepare_yield should pick the RT task
    sched.state.per_cpu[0].need_resched = false; // clear so prepare_yield runs clean
    let switch = sched.prepare_yield();
    assert!(switch.is_some(), "Should produce a context switch");
    let switch = switch.unwrap();
    assert_eq!(switch.to_tid, 3002, "Scheduler should switch to the RT task");
    assert_eq!(switch.from_tid, 3001, "Scheduler should switch away from the Normal task");
}

/// Verify that `wake_sleepers` defers cross-CPU IPIs to `pending_wake_ipis`
/// instead of calling `send_ipi` while the SCHEDULER lock is held.
///
/// The fix for issue #131 changed `wake_sleepers` to push target CPUs onto
/// `self.pending_wake_ipis` rather than invoking `send_ipi` directly. This
/// test checks that, after a cross-CPU sleeper is woken:
///   1. The IPI target is present in `pending_wake_ipis`.
///   2. `pending_wake_ipis` is empty once the caller drains it.
#[test]
fn test_wake_sleepers_defers_ipi_to_pending_wake_ipis() {
    let _g = init_test_env();
    use core::sync::atomic::Ordering;

    let mut sched = types::Scheduler::<MockRuntime>::new();
    // Add two CPUs: CPU 0 (running this test) and CPU 1 (the IPI target).
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    // Mark CPU 1 as online so it appears in the online_cpus list.
    sched.state.mark_cpu_online(1);

    // Task running on CPU 0 (the "current" task for MockRuntime).
    let current_task = make_task(9001, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    sched.state.per_cpu[0].current = Some(9001);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9001,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(0),
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    // Sleeping task pinned to CPU 1 (a different CPU from current_cpu_index == 0).
    let sleeping_task = make_task(9002, TaskState::Blocked, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(sleeping_task));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9002,
        runq_location: None,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(1),
        last_cpu: Some(1),
        wake_cpu: Some(1),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });

    // Put the sleeping task in the sleep queue with a wake_tick in the past.
    TICK_COUNT.store(100, Ordering::Relaxed);
    sched.state.add_task_to_sleep_queue(9002, 50);

    // `pending_wake_ipis` should be empty before wake_sleepers runs.
    assert!(
        sched.pending_wake_ipis.is_empty(),
        "pending_wake_ipis should be empty before wake_sleepers"
    );

    sched.wake_sleepers();

    // After wake_sleepers, the IPI for CPU 1 must be in pending_wake_ipis,
    // not already sent (send_ipi is a no-op in MockRuntime).
    assert_eq!(
        sched.pending_wake_ipis,
        alloc::vec![1usize],
        "wake_sleepers should defer cross-CPU IPI to pending_wake_ipis (got {:?})",
        sched.pending_wake_ipis
    );

    // Simulate the caller draining pending_wake_ipis after releasing the lock.
    let drained = core::mem::take(&mut sched.pending_wake_ipis);
    assert_eq!(drained, alloc::vec![1usize]);
    assert!(sched.pending_wake_ipis.is_empty(), "pending_wake_ipis should be empty after drain");
}

#[test]
fn test_wake_sleepers_coalesces_same_cpu_remote_ipi_within_batch() {
    let _g = init_test_env();
    use core::sync::atomic::Ordering;

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.mark_cpu_online(1);

    PROF_IPI_SUPPRESSED.store(0, Ordering::Relaxed);
    clear_global_need_resched(1, Ordering::Relaxed);

    let current_task = make_task(9_101, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    sched.state.per_cpu[0].current = Some(9_101);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9_101,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(0),
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    for tid in [9_102, 9_103] {
        let sleeping_task = make_task(tid, TaskState::Blocked, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(sleeping_task));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Blocked,
            priority: TaskPriority::Normal,
            affinity: Affinity::Pinned(1),
            last_cpu: Some(1),
            wake_cpu: Some(1),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.add_task_to_sleep_queue(tid, 50);
    }
    TICK_COUNT.store(100, Ordering::Relaxed);

    sched.wake_sleepers();

    assert_eq!(
        sched.pending_wake_ipis,
        alloc::vec![1usize],
        "waking multiple sleepers for the same remote CPU should queue only one deferred IPI"
    );
    assert_eq!(
        PROF_IPI_SUPPRESSED.load(Ordering::Relaxed),
        1,
        "duplicate remote wake in one wake_sleepers batch should be counted as suppressed"
    );
}

#[test]
fn test_wake_sleepers_enforces_budget_and_carries_remainder() {
    let _g = init_test_env();
    use core::sync::atomic::Ordering;

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    let total_sleepers = types::WAKE_SLEEPERS_BUDGET_PER_TICK + 5;
    for tid in 10_000..(10_000 + total_sleepers as u64) {
        let sleeping_task = make_task(tid, TaskState::Blocked, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(sleeping_task));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Blocked,
            priority: TaskPriority::Normal,
            affinity: Affinity::Pinned(0),
            last_cpu: Some(0),
            wake_cpu: Some(0),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });
        sched.state.add_task_to_sleep_queue(tid, 50);
    }
    TICK_COUNT.store(100, Ordering::Relaxed);

    sched.wake_sleepers();
    assert_eq!(
        sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].len(),
        types::WAKE_SLEEPERS_BUDGET_PER_TICK,
        "wake_sleepers should honor per-tick wake budget"
    );
    assert_eq!(
        sched.state.sleep_queue.get(&50).map(|v| v.len()),
        Some(5),
        "sleep queue should retain remaining sleepers after budget is exhausted"
    );
    assert_eq!(
        sched.wake_sleepers_budget_carry, 0,
        "budget carry should be empty after fully consuming available budget"
    );

    sched.wake_sleepers();
    assert_eq!(
        sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].len(),
        total_sleepers,
        "second wake pass should process remaining sleepers"
    );
    assert!(
        !sched.state.sleep_queue.contains_key(&50),
        "sleep queue entry should be removed once all sleepers wake"
    );
    assert_eq!(
        sched.wake_sleepers_budget_carry,
        types::WAKE_SLEEPERS_BUDGET_PER_TICK - 5,
        "unused budget should carry forward to later ticks"
    );
}

#[test]
fn test_prepare_schedule_defers_misroute_repair_until_maintenance() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.mark_cpu_online(1);

    // Current task on CPU 0.
    let current_task = make_task(9101, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    sched.state.per_cpu[0].current = Some(9101);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9101,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(0),
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    // Runnable task incorrectly queued on CPU 0 but pinned to CPU 1.
    let misrouted = make_task(9102, TaskState::Runnable, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(misrouted));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9102,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(1),
        last_cpu: Some(1),
        wake_cpu: Some(1),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 9102);

    // No local idle task exists, so prepare_schedule returns None and defers
    // misroute repair to the maintenance pass.
    let switch = sched.prepare_schedule();
    assert!(
        switch.is_none(),
        "prepare_schedule should return None when only misrouted work exists and no idle task"
    );
    assert!(
        sched.drain_pending_prepare_schedule_ipis().is_empty(),
        "prepare_schedule picker path should not perform deferred misroute repair or queue repair IPIs"
    );
    assert!(
        sched.pending_wake_ipis.is_empty(),
        "misroute IPIs should not be mixed into pending_wake_ipis"
    );
    assert!(
        sched
            .pending_misrouted_requeues
            .iter()
            .any(|&(_, target_cpu, tid)| target_cpu == 1 && tid == 9102),
        "misrouted task should be queued for deferred repair maintenance"
    );
    assert!(
        !sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 9102),
        "target CPU runq should remain unchanged until maintenance runs"
    );

    sched.run_pending_misroute_repair_maintenance();

    assert_eq!(
        sched.drain_pending_prepare_schedule_ipis(),
        alloc::vec![1usize],
        "misroute maintenance should queue the deferred repair nudge"
    );
    assert!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 9102),
        "misroute maintenance should move the task to the target CPU runq"
    );
}

#[test]
fn test_pending_prepare_schedule_ipi_bitmap_dedups_targets() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    sched.queue_pending_prepare_schedule_ipi(1);
    sched.queue_pending_prepare_schedule_ipi(1);
    sched.queue_pending_prepare_schedule_ipi(2);

    assert_eq!(
        sched.drain_pending_prepare_schedule_ipis(),
        alloc::vec![1usize, 2usize],
        "bitmap-backed pending_prepare_schedule_ipis should dedup repeated CPU targets"
    );
    assert!(
        sched.drain_pending_prepare_schedule_ipis().is_empty(),
        "drain should clear the bitmap-backed pending queue"
    );
}

#[test]
fn test_pending_prepare_schedule_ipi_bitmap_drain_preserves_all_pending_targets() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    sched.queue_pending_prepare_schedule_ipi(1);
    sched.queue_pending_prepare_schedule_ipi(3);
    sched.queue_pending_prepare_schedule_ipi(5);
    sched.queue_pending_prepare_schedule_ipi(3);

    assert_eq!(
        sched.drain_pending_prepare_schedule_ipis(),
        alloc::vec![1usize, 3usize, 5usize],
        "bitmap-backed drain should return all queued CPU targets without dropping pending bits or duplicating targets"
    );
}

#[test]
fn test_misroute_repair_maintenance_is_bounded_per_call() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.mark_cpu_online(1);

    // Current task on CPU 0.
    let current_task = make_task(9200, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    sched.state.per_cpu[0].current = Some(9200);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9200,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(0),
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    // Fill CPU 0's normal queue with misrouted tasks pinned to CPU 1.
    let misrouted_total = PREPARE_SCHEDULE_PICK_BUDGET + 4;
    for i in 0..misrouted_total {
        let tid = 9201 + i as u64;
        let task = make_task(tid, TaskState::Runnable, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Pinned(1),
            last_cpu: Some(1),
            wake_cpu: Some(1),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });
        sched.state.enqueue_task(0, TaskPriority::Normal as usize, tid);
    }

    let switch = sched.prepare_schedule();
    assert!(switch.is_none());
    assert!(
        !sched.pending_misrouted_requeues.is_empty(),
        "prepare_schedule should defer misroute repairs into maintenance backlog"
    );
    assert_eq!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].len(),
        0,
        "prepare_schedule picker path should not directly repair deferred misroutes"
    );

    sched.run_pending_misroute_repair_maintenance();

    assert!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].len()
            <= PREPARE_SCHEDULE_MISROUTE_REPAIR_BUDGET,
        "maintenance should only repair up to PREPARE_SCHEDULE_MISROUTE_REPAIR_BUDGET misroutes per call"
    );

    // Repeated maintenance passes should drain deferred work.
    for _ in 0..4 {
        sched.run_pending_misroute_repair_maintenance();
    }
    assert!(
        sched.pending_misrouted_requeues.is_empty(),
        "deferred misroute backlog should drain across maintenance passes"
    );
}

#[test]
fn test_misroute_repair_maintenance_drops_non_runnable_entries() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.mark_cpu_online(1);

    // Current task on CPU 0.
    let current_task = make_task(9300, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    sched.state.per_cpu[0].current = Some(9300);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9300,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(0),
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    // Runnable task incorrectly queued on CPU 0 but pinned to CPU 1.
    let misrouted = make_task(9301, TaskState::Runnable, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(misrouted));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9301,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Pinned(1),
        last_cpu: Some(1),
        wake_cpu: Some(1),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, 9301);

    let switch = sched.prepare_schedule();
    assert!(switch.is_none());
    assert_eq!(sched.pending_misrouted_requeues.len(), 1);

    if let Some(sf) = sched.state.get_task_mut(9301) {
        sf.state = TaskState::Dead;
    }

    sched.run_pending_misroute_repair_maintenance();

    assert!(
        sched.pending_misrouted_requeues.is_empty(),
        "maintenance prevalidation should drop non-runnable deferred misroutes"
    );
    assert!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].iter().all(|&tid| tid != 9301),
        "dropped deferred entry should not be re-enqueued"
    );
    assert!(
        sched.drain_pending_prepare_schedule_ipis().is_empty(),
        "dropped deferred entry should not queue an IPI"
    );
}

#[test]
fn test_send_deferred_prepare_schedule_ipis_updates_counters() {
    let _g = init_test_env();
    use core::sync::atomic::Ordering;

    DIAG_IPI_SENT.store(0, Ordering::Relaxed);
    DIAG_IPI_SENT_PREPARE_SCHEDULE.store(0, Ordering::Relaxed);
    LAST_RESCHED_IPI_SENT_AT_TICK[1].store(RESCHED_IPI_NEVER_SENT, Ordering::Relaxed);
    LAST_RESCHED_IPI_SENT_AT_TICK[2].store(RESCHED_IPI_NEVER_SENT, Ordering::Relaxed);
    TICK_COUNT.store(1, Ordering::Relaxed);

    send_deferred_prepare_schedule_ipis::<MockRuntime>(alloc::vec![1usize, 2usize]);

    assert_eq!(
        DIAG_IPI_SENT.load(Ordering::Relaxed),
        2,
        "generic IPI counter should increase for each deferred prepare_schedule send"
    );
    assert_eq!(
        DIAG_IPI_SENT_PREPARE_SCHEDULE.load(Ordering::Relaxed),
        2,
        "prepare_schedule source counter should increase for each deferred send"
    );
}

#[test]
fn test_send_deferred_prepare_schedule_ipis_suppresses_same_tick_duplicates() {
    let _g = init_test_env();
    use core::sync::atomic::Ordering;

    DIAG_IPI_SENT.store(0, Ordering::Relaxed);
    DIAG_IPI_SENT_PREPARE_SCHEDULE.store(0, Ordering::Relaxed);
    PROF_IPI_SUPPRESSED.store(0, Ordering::Relaxed);
    LAST_RESCHED_IPI_SENT_AT_TICK[1].store(RESCHED_IPI_NEVER_SENT, Ordering::Relaxed);
    TICK_COUNT.store(42, Ordering::Relaxed);

    send_deferred_prepare_schedule_ipis::<MockRuntime>(alloc::vec![1usize, 1usize]);

    assert_eq!(DIAG_IPI_SENT.load(Ordering::Relaxed), 1);
    assert_eq!(DIAG_IPI_SENT_PREPARE_SCHEDULE.load(Ordering::Relaxed), 1);
    assert_eq!(PROF_IPI_SUPPRESSED.load(Ordering::Relaxed), 1);
}

#[test]
fn test_should_send_remote_resched_ipi_suppresses_adjacent_tick_ping_pong() {
    let _g = init_test_env();
    use core::sync::atomic::Ordering;

    PROF_IPI_SUPPRESSED.store(0, Ordering::Relaxed);
    LAST_RESCHED_IPI_SENT_AT_TICK[1].store(RESCHED_IPI_NEVER_SENT, Ordering::Relaxed);

    TICK_COUNT.store(100, Ordering::Relaxed);
    assert!(should_send_remote_resched_ipi(1));

    TICK_COUNT.store(101, Ordering::Relaxed);
    assert!(!should_send_remote_resched_ipi(1));
    assert_eq!(PROF_IPI_SUPPRESSED.load(Ordering::Relaxed), 1);

    TICK_COUNT.store(102, Ordering::Relaxed);
    assert!(should_send_remote_resched_ipi(1));

    TICK_COUNT.store(103, Ordering::Relaxed);
    assert!(!should_send_remote_resched_ipi(1));
    assert_eq!(PROF_IPI_SUPPRESSED.load(Ordering::Relaxed), 2);
}

#[test]
fn test_sample_runq_len_downsamples_global_telemetry_scan() {
    let _g = init_test_env();
    use core::sync::atomic::Ordering;

    use crate::sched::state::PerCpu;

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(PerCpu::new());
    sched.state.per_cpu.push(PerCpu::new());
    sched.state.online_cpus = alloc::vec![0, 1];
    sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].push_back(7001);
    sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].push_back(7002);

    PROF_RUNQ_SAMPLE_COUNT[0].store(0, Ordering::Relaxed);
    PROF_RUNQ_SAMPLE_TOTAL[0].store(0, Ordering::Relaxed);
    PROF_RUNQ_DEPTH_VARIANCE_SAMPLE_COUNT.store(0, Ordering::Relaxed);
    PROF_RUNQ_DEPTH_VARIANCE_TOTAL.store(0, Ordering::Relaxed);
    PROF_RUNQ_DEPTH_VARIANCE_LAST.store(0, Ordering::Relaxed);
    PROF_RUNQ_DEPTH_VARIANCE_MAX.store(0, Ordering::Relaxed);

    for _ in 0..(RUNQ_GLOBAL_TELEMETRY_SAMPLE_STRIDE - 1) {
        sample_runq_len(&mut sched, 0);
    }
    assert_eq!(
        PROF_RUNQ_DEPTH_VARIANCE_SAMPLE_COUNT.load(Ordering::Relaxed),
        0,
        "global variance scan should be skipped until the configured stride is reached"
    );

    sample_runq_len(&mut sched, 0);
    assert_eq!(
        PROF_RUNQ_DEPTH_VARIANCE_SAMPLE_COUNT.load(Ordering::Relaxed),
        1,
        "global variance scan should run when the stride boundary is reached"
    );
}
