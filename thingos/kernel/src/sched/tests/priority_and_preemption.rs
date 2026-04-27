use core::sync::atomic::Ordering;

use super::super::*;
use super::support::*;
use crate::task::{Affinity, TaskId, TaskPriority, TaskState};
use crate::{BootRuntimeBase, BootTasking};

#[test]
fn set_priority_uses_hot_cache_runnable_state_for_runq_move() {
    let _g = init_test_env();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    // Registry carries stale state (Blocked), while scheduler cache has the
    // hot-path truth (Runnable + currently enqueued).
    let task = make_task(42, TaskState::Blocked, TaskPriority::Low);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 42,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Low,
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
    sched.state.enqueue_task(0, TaskPriority::Low as usize, 42);

    sched.set_priority(42, TaskPriority::High);

    assert_eq!(sched.state.get_task(42).unwrap().priority, TaskPriority::High);
    assert_eq!(
        crate::task::registry::get_registry::<MockRuntime>().threads[0].priority,
        TaskPriority::Low,
        "scheduler-only set_priority keeps REGISTRY sync out of the hot path"
    );
    assert!(sched.state.per_cpu[0].runq[TaskPriority::Low as usize].is_empty());
    assert_eq!(sched.state.per_cpu[0].runq[TaskPriority::High as usize].front().copied(), Some(42));
}

#[test]
fn public_set_priority_syncs_registry_after_scheduler_unlock() {
    let _g = init_test_env();

    let mut sched = alloc::boxed::Box::new(types::Scheduler::<MockRuntime>::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    let task = make_task(43, TaskState::Runnable, TaskPriority::Low);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 43,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Low,
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
    sched.state.enqueue_task(0, TaskPriority::Low as usize, 43);

    let sched_ptr = alloc::boxed::Box::into_raw(sched);
    *SCHEDULER.lock() = Some(sched_ptr as usize);

    set_priority::<MockRuntime>(43, TaskPriority::High);

    let sched_ref = unsafe { &mut *(sched_ptr as *mut types::Scheduler<MockRuntime>) };
    assert_eq!(sched_ref.state.get_task(43).unwrap().priority, TaskPriority::High);
    assert!(sched_ref.state.per_cpu[0].runq[TaskPriority::Low as usize].is_empty());
    assert_eq!(
        sched_ref.state.per_cpu[0].runq[TaskPriority::High as usize].front().copied(),
        Some(43)
    );
    assert_eq!(
        crate::task::registry::get_registry::<MockRuntime>().threads[0].priority,
        TaskPriority::High,
        "public set_priority should sync REGISTRY outside scheduler lock"
    );

    *SCHEDULER.lock() = None;
    unsafe {
        drop(alloc::boxed::Box::from_raw(sched_ptr));
    }
}

#[test]
fn current_priority_reads_scheduler_hot_cache() {
    let _g = init_test_env();

    let mut sched = alloc::boxed::Box::new(types::Scheduler::<MockRuntime>::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(44);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 44,
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
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    let sched_ptr = alloc::boxed::Box::into_raw(sched);
    *SCHEDULER.lock() = Some(sched_ptr as usize);

    assert_eq!(current_priority::<MockRuntime>(), TaskPriority::High);

    *SCHEDULER.lock() = None;
    unsafe {
        drop(alloc::boxed::Box::from_raw(sched_ptr));
    }
}

#[test]
fn current_priority_defaults_when_no_current_task() {
    let _g = init_test_env();

    let mut sched = alloc::boxed::Box::new(types::Scheduler::<MockRuntime>::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    let sched_ptr = alloc::boxed::Box::into_raw(sched);
    *SCHEDULER.lock() = Some(sched_ptr as usize);

    assert_eq!(current_priority::<MockRuntime>(), TaskPriority::Normal);

    *SCHEDULER.lock() = None;
    unsafe {
        drop(alloc::boxed::Box::from_raw(sched_ptr));
    }
}

#[test]
fn current_priority_defaults_when_current_task_missing_from_hot_cache() {
    let _g = init_test_env();

    let mut sched = alloc::boxed::Box::new(types::Scheduler::<MockRuntime>::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(99);
    let sched_ptr = alloc::boxed::Box::into_raw(sched);
    *SCHEDULER.lock() = Some(sched_ptr as usize);

    assert_eq!(current_priority::<MockRuntime>(), TaskPriority::Normal);

    *SCHEDULER.lock() = None;
    unsafe {
        drop(alloc::boxed::Box::from_raw(sched_ptr));
    }
}

#[test]
fn preempt_disable_depth_is_cpu_local() {
    let _g = init_test_env();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    sched.state.per_cpu[1].preempt_disable_depth = 3;
    sched.state.per_cpu[1].preempt_disable_since = 42;
    sched.state.per_cpu[1].preempt_watchdog_warned = true;

    TICK_COUNT.store(99, Ordering::Relaxed);
    sched.preempt_disable();
    sched.preempt_disable();
    assert_eq!(sched.state.per_cpu[0].preempt_disable_depth, 2);
    assert_eq!(sched.state.per_cpu[0].preempt_disable_since, 99);
    assert!(!sched.state.per_cpu[0].preempt_watchdog_warned);

    sched.preempt_enable();
    assert_eq!(sched.state.per_cpu[0].preempt_disable_depth, 1);
    sched.preempt_enable();
    assert_eq!(sched.state.per_cpu[0].preempt_disable_depth, 0);

    assert_eq!(sched.state.per_cpu[1].preempt_disable_depth, 3);
    assert_eq!(sched.state.per_cpu[1].preempt_disable_since, 42);
    assert!(sched.state.per_cpu[1].preempt_watchdog_warned);
}

#[test]
fn clear_sched_lock_tracking_only_clears_when_called_by_owner_cpu() {
    let _g = init_test_env();

    set_sched_lock_tracking::<MockRuntime>(1);
    SCHEDULER_LOCK_ACQUIRED_AT.store(123, Ordering::Release);
    clear_sched_lock_tracking::<MockRuntime>();
    assert_eq!(SCHEDULER_LOCK_OWNER.load(Ordering::Acquire), 1);
    assert_eq!(SCHEDULER_LOCK_ACQUIRED_AT.load(Ordering::Acquire), 123);

    set_sched_lock_tracking::<MockRuntime>(0);
    clear_sched_lock_tracking::<MockRuntime>();
    assert_eq!(SCHEDULER_LOCK_OWNER.load(Ordering::Acquire), -1);
    assert_eq!(SCHEDULER_LOCK_ACQUIRED_AT.load(Ordering::Acquire), 0);
}

#[test]
fn sched_lock_tracking_guard_clears_on_drop() {
    let _g = init_test_env();

    {
        let _tracking = sched_lock_tracking_guard::<MockRuntime>(0);
        assert_eq!(SCHEDULER_LOCK_OWNER.load(Ordering::Acquire), 0);
    }

    assert_eq!(SCHEDULER_LOCK_OWNER.load(Ordering::Acquire), -1);
    assert_eq!(SCHEDULER_LOCK_ACQUIRED_AT.load(Ordering::Acquire), 0);
}

#[test]
fn dump_stats_clears_sched_lock_tracking() {
    let _g = init_test_env();
    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    let sched_ptr = &mut sched as *mut types::Scheduler<MockRuntime> as usize;
    *SCHEDULER.lock() = Some(sched_ptr);
    dump_stats::<MockRuntime>();

    assert_eq!(SCHEDULER_LOCK_OWNER.load(Ordering::Acquire), -1);
    assert_eq!(SCHEDULER_LOCK_ACQUIRED_AT.load(Ordering::Acquire), 0);
    *SCHEDULER.lock() = None;
}
