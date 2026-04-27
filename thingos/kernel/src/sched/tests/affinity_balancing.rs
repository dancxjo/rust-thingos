use core::sync::atomic::Ordering;

use super::super::*;
use super::support::*;
use crate::sched::TEST_LEAST_LOADED_ONLINE_CPU_CALLS;
use crate::sched::balancing::{
    STEAL_SCAN_DEPTH_PER_PRIORITY, reset_any_wake_policy_for_tests, select_any_affinity_wake_cpu,
    set_any_wake_policy_for_tests, set_any_wake_policy_for_tests_with_streak,
};
use crate::task::{Affinity, TaskId, TaskPriority, TaskState};
use crate::{BootRuntimeBase, BootTasking};

#[test]
fn test_wakeup_routes_to_last_cpu_for_any_affinity() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..3 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);
    sched.state.mark_cpu_online(2);
    sched.state.per_cpu[0].current = Some(0);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        0,
        TaskState::Running,
        TaskPriority::Normal,
    )));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        9901,
        TaskState::Blocked,
        TaskPriority::Normal,
    )));

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
        tid: 9901,
        runq_location: None,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(2),
        wake_cpu: None,
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    let (_ipi, _deferred) =
        crate::sched::blocking::wake_task_locked::<MockRuntime>(&mut sched, 9901);
    assert!(
        sched.state.per_cpu[2].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 9901),
        "[policy] wakeup should enqueue Any-affinity blocked task onto last_cpu"
    );
    assert_eq!(
        sched.state.get_task(9901).and_then(|sf| sf.wake_cpu),
        Some(2),
        "[policy] wake_cpu should preserve last_cpu routing for Any-affinity wakeup"
    );
}

#[test]
fn test_wakeup_redirects_any_affinity_when_last_cpu_overloaded() {
    let _g = init_test_env();
    set_any_wake_policy_for_tests("redirect", 2);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);
    sched.state.per_cpu[0].current = Some(0);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        0,
        TaskState::Running,
        TaskPriority::Normal,
    )));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        9911,
        TaskState::Blocked,
        TaskPriority::Normal,
    )));

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
        tid: 9911,
        runq_location: None,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: None,
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    // Make CPU 0 overloaded relative to CPU 1.
    const OVERLOAD_TASK_START: u64 = 9912;
    const OVERLOAD_TASK_END: u64 = 9915;
    for id in OVERLOAD_TASK_START..OVERLOAD_TASK_END {
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(make_task(id, TaskState::Runnable, TaskPriority::Low)));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: id,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Low,
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
        sched.state.enqueue_task(0, TaskPriority::Low as usize, id);
    }

    let (_ipi, _deferred) =
        crate::sched::blocking::wake_task_locked::<MockRuntime>(&mut sched, 9911);
    assert!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 9911),
        "[policy] redirect should move Any-affinity wakeup to least-loaded CPU"
    );
    assert_eq!(
        sched.state.get_task(9911).and_then(|sf| sf.wake_cpu),
        Some(1),
        "[policy] wake_cpu should track redirected Any-affinity wakeup target"
    );
}

#[test]
fn test_any_affinity_wakeup_prefers_local_cpu_when_last_cpu_is_busier() {
    let _g = init_test_env();
    reset_any_wake_policy_for_tests();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..3 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);
    sched.state.mark_cpu_online(2);
    sched.state.per_cpu[0].current = Some(0);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        0,
        TaskState::Running,
        TaskPriority::Normal,
    )));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        9916,
        TaskState::Blocked,
        TaskPriority::Normal,
    )));
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
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9916,
        runq_location: None,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(2),
        wake_cpu: None,
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });

    for id in 9917..9920 {
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(make_task(id, TaskState::Runnable, TaskPriority::Low)));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: id,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Low,
            affinity: Affinity::Any,
            last_cpu: Some(2),
            wake_cpu: Some(2),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });
        sched.state.enqueue_task(2, TaskPriority::Low as usize, id);
    }

    let (_ipi, _deferred) =
        crate::sched::blocking::wake_task_locked::<MockRuntime>(&mut sched, 9916);
    assert!(
        sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 9916),
        "[policy] Any-affinity wakeup should prefer local CPU when last_cpu is busier"
    );
    assert_eq!(
        sched.state.get_task(9916).and_then(|sf| sf.wake_cpu),
        Some(0),
        "[policy] wake_cpu should record local routing under local-bias heuristic"
    );
}

#[test]
fn test_any_wake_hysteresis_requires_persistent_overload_before_rebalance() {
    let _g = init_test_env();
    set_any_wake_policy_for_tests_with_streak("redirect", 2, 3);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    for tid in 20_000..20_003 {
        sched.state.enqueue_task(0, TaskPriority::Low as usize, tid);
    }

    assert_eq!(select_any_affinity_wake_cpu::<MockRuntime>(&sched, 0), 0);
    assert_eq!(select_any_affinity_wake_cpu::<MockRuntime>(&sched, 0), 0);
    assert_eq!(select_any_affinity_wake_cpu::<MockRuntime>(&sched, 0), 1);
}

#[test]
fn test_any_wake_hysteresis_resets_when_overload_clears() {
    let _g = init_test_env();
    set_any_wake_policy_for_tests_with_streak("redirect", 2, 3);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    for tid in 21_000..21_003 {
        sched.state.enqueue_task(0, TaskPriority::Low as usize, tid);
    }
    assert_eq!(select_any_affinity_wake_cpu::<MockRuntime>(&sched, 0), 0);

    sched.state.per_cpu[0].runq[TaskPriority::Low as usize].clear();
    assert_eq!(select_any_affinity_wake_cpu::<MockRuntime>(&sched, 0), 0);

    for tid in 21_003..21_006 {
        sched.state.enqueue_task(0, TaskPriority::Low as usize, tid);
    }
    assert_eq!(select_any_affinity_wake_cpu::<MockRuntime>(&sched, 0), 0);
    assert_eq!(select_any_affinity_wake_cpu::<MockRuntime>(&sched, 0), 0);
    assert_eq!(select_any_affinity_wake_cpu::<MockRuntime>(&sched, 0), 1);
}

// ── choose_wake_cpu tests ─────────────────────────────────────────────

/// `choose_wake_cpu` should route an Any-affinity wakeup to the idle CPU
/// when the preferred CPU (last_cpu) is loaded beyond the overload gap.
#[test]
fn test_choose_wake_cpu_prefers_idle_cpu_when_preferred_is_loaded() {
    let _g = init_test_env();
    set_any_wake_policy_for_tests("redirect", 2);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    // CPU 0: loaded, current is a non-idle task.
    // CPU 1: idle  (current == idle_task).
    // CPU 2: online but not idle.
    for _ in 0..3 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);
    sched.state.mark_cpu_online(2);

    // Make CPU 1 idle: current == idle_task == 80_001.
    let idle_tid: u64 = 80_001;
    sched.state.per_cpu[1].idle_task = Some(idle_tid);
    sched.state.per_cpu[1].current = Some(idle_tid);

    // Load CPU 0 past the overload gap (gap = 2, add 3 tasks).
    for tid in 80_010..80_013 {
        sched.state.enqueue_task(0, TaskPriority::Low as usize, tid);
    }

    // Task that was last on CPU 0.
    let result = choose_wake_cpu::<MockRuntime>(&sched, Some(0));
    assert_eq!(result, 1, "choose_wake_cpu should route to idle CPU 1 when CPU 0 is overloaded");
}

/// `choose_wake_cpu` should stay on the preferred CPU when the preferred CPU
/// is itself the idle CPU (i.e. it is lightly loaded).
#[test]
fn test_choose_wake_cpu_stays_on_preferred_when_preferred_is_idle() {
    let _g = init_test_env();
    set_any_wake_policy_for_tests("redirect", 2);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    // Make CPU 0 idle: current == idle_task == 80_020.
    let idle_tid: u64 = 80_020;
    sched.state.per_cpu[0].idle_task = Some(idle_tid);
    sched.state.per_cpu[0].current = Some(idle_tid);

    // Task with last_cpu = 0 (the idle CPU).
    let result = choose_wake_cpu::<MockRuntime>(&sched, Some(0));
    // CPU 0 is the idle CPU and its depth is 0 (below overload_gap of 2),
    // so no redirection should happen.
    assert_eq!(result, 0, "choose_wake_cpu should keep preferred CPU when it is already idle");
}

/// `choose_wake_cpu` should use last_cpu as the target when no idle CPU
/// exists and the preferred CPU is not overloaded.
#[test]
fn test_choose_wake_cpu_uses_last_cpu_when_no_idle_cpu_and_not_overloaded() {
    let _g = init_test_env();
    reset_any_wake_policy_for_tests();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..3 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);
    sched.state.mark_cpu_online(2);
    // No idle CPU: current is None on all CPUs.

    // Task with last_cpu = 2.
    let result = choose_wake_cpu::<MockRuntime>(&sched, Some(2));
    assert_eq!(
        result, 2,
        "choose_wake_cpu should return last_cpu when not overloaded and no idle CPU exists"
    );
}

/// When multiple Any-affinity tasks wake at once and one idle CPU exists,
/// the first wakeup should land on the idle CPU; subsequent wakeups should
/// spread to other CPUs rather than all piling on the idle one.
#[test]
fn test_choose_wake_cpu_distributes_fanout_across_cpus() {
    let _g = init_test_env();
    set_any_wake_policy_for_tests("redirect", 1);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..4 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);
    sched.state.mark_cpu_online(2);
    sched.state.mark_cpu_online(3);

    // Load CPUs 0, 2, 3 so they are above overload gap.
    for cpu in [0usize, 2, 3] {
        for tid_offset in 0..3u64 {
            let tid = (cpu as u64) * 100 + 80_100 + tid_offset;
            sched.state.enqueue_task(cpu, TaskPriority::Low as usize, tid);
        }
    }
    // CPU 1 is idle.
    let idle_tid: u64 = 80_050;
    sched.state.per_cpu[1].idle_task = Some(idle_tid);
    sched.state.per_cpu[1].current = Some(idle_tid);

    // All tasks have last_cpu = 0 (overloaded).
    let targets: alloc::vec::Vec<usize> =
        (0..4).map(|_| choose_wake_cpu::<MockRuntime>(&sched, Some(0))).collect();

    // At least one call should have routed to an idle or less-loaded CPU.
    assert!(
        targets.iter().any(|&cpu| cpu != 0),
        "fanout should distribute wakeups away from overloaded CPU 0, got {:?}",
        targets
    );
}

#[test]
fn test_wake_sleepers_redirects_any_affinity_when_last_cpu_overloaded() {
    let _g = init_test_env();
    set_any_wake_policy_for_tests("redirect", 2);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    let current_task = make_task(9920, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    sched.state.per_cpu[0].current = Some(9920);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9920,
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

    let sleeping = make_task(9921, TaskState::Blocked, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(sleeping));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9921,
        runq_location: None,
        state: TaskState::Blocked,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: None,
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });

    const OVERLOAD_TASK_START: u64 = 9922;
    const OVERLOAD_TASK_END: u64 = 9925;
    for id in OVERLOAD_TASK_START..OVERLOAD_TASK_END {
        let runnable = make_task(id, TaskState::Runnable, TaskPriority::Low);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(runnable));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: id,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Low,
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
        sched.state.enqueue_task(0, TaskPriority::Low as usize, id);
    }

    TICK_COUNT.store(100, Ordering::Relaxed);
    sched.state.add_task_to_sleep_queue(9921, 50);
    sched.wake_sleepers();

    assert!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 9921),
        "[policy] wake_sleepers should redirect Any-affinity wakeup under overload"
    );
    assert_eq!(
        sched.state.get_task(9921).and_then(|sf| sf.wake_cpu),
        Some(1),
        "[policy] wake_sleepers should record redirected wake_cpu for Any-affinity task"
    );
}

#[test]
fn test_wake_sleepers_batches_any_affinity_load_scans() {
    let _g = init_test_env();
    set_any_wake_policy_for_tests("redirect", 2);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    let current_task = make_task(9926, TaskState::Running, TaskPriority::Normal);
    crate::task::registry::get_registry::<MockRuntime>()
        .insert(alloc::boxed::Box::new(current_task));
    sched.state.per_cpu[0].current = Some(9926);
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9926,
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

    // Keep CPU 0 overloaded for Any-affinity wake routing.
    for id in 9927..9930 {
        let runnable = make_task(id, TaskState::Runnable, TaskPriority::Low);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(runnable));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: id,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Low,
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
        sched.state.enqueue_task(0, TaskPriority::Low as usize, id);
    }

    for id in 9930..9933 {
        let sleeping = make_task(id, TaskState::Blocked, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(sleeping));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: id,
            runq_location: None,
            state: TaskState::Blocked,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(0),
            wake_cpu: None,
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });
        sched.state.add_task_to_sleep_queue(id, 50);
    }

    TEST_LEAST_LOADED_ONLINE_CPU_CALLS.store(0, Ordering::Relaxed);
    TICK_COUNT.store(100, Ordering::Relaxed);
    sched.wake_sleepers();

    let cpu0_wakes = sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].len();
    let cpu1_wakes = sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].len();
    assert_eq!(cpu0_wakes + cpu1_wakes, 3, "all Any-affinity sleepers should wake in this batch");
    assert!(
        cpu0_wakes > 0 && cpu1_wakes > 0,
        "batch load snapshot should spread wakeups after redirecting early tasks (cpu0={}, cpu1={})",
        cpu0_wakes,
        cpu1_wakes
    );

    let least_scan_calls = TEST_LEAST_LOADED_ONLINE_CPU_CALLS.load(Ordering::Relaxed);
    assert!(
        least_scan_calls == 0,
        "wake batch should avoid least-loaded queue scans via per-batch snapshots (calls={})",
        least_scan_calls
    );
}

#[test]
fn test_steal_task_scans_bounded_depth_per_priority() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        9930,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        9931,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));

    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9930,
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
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: 9931,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(1),
        wake_cpu: Some(1),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });

    sched.state.enqueue_task(1, TaskPriority::Normal as usize, 9930);
    sched.state.enqueue_task(1, TaskPriority::Normal as usize, 9931);

    let stolen = sched.steal_task_for(0);
    assert_eq!(
        stolen,
        Some(9931),
        "bounded scan should skip unstealable head and steal stealable follower"
    );
    assert_eq!(
        sched.state.get_task(9931).and_then(|sf| sf.wake_cpu),
        Some(0),
        "stolen task wake_cpu should track destination CPU"
    );
    assert_eq!(
        sched.state.get_task(9931).and_then(|sf| sf.runq_location),
        None,
        "stolen task should no longer be tracked in donor runq"
    );
    assert!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 9930),
        "unstealable pinned head should remain queued on donor CPU"
    );
}

#[test]
fn test_steal_task_respects_scan_depth_limit() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    for offset in 0..STEAL_SCAN_DEPTH_PER_PRIORITY {
        let tid = 9940 + offset as u64;
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(tid, TaskState::Runnable, TaskPriority::Normal),
        ));
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
        sched.state.enqueue_task(1, TaskPriority::Normal as usize, tid);
    }

    let stealable_tid = 9950;
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        stealable_tid,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: stealable_tid,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(1),
        wake_cpu: Some(1),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });
    sched.state.enqueue_task(1, TaskPriority::Normal as usize, stealable_tid);

    let stolen = sched.steal_task_for(0);
    assert_eq!(
        stolen, None,
        "steal scan must stay bounded and not inspect beyond configured depth"
    );
    assert!(
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize]
            .iter()
            .any(|&tid| tid == stealable_tid),
        "stealable task beyond scan-depth cap should remain on donor queue"
    );
}

// ── try_steal_one tests ────────────────────────────────────────────────

#[test]
fn test_try_steal_one_returns_none_when_victim_below_min_depth() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    // Only one task on CPU 1 — below the min_depth=2 threshold.
    let tid = 10_001u64;
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        tid,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(1),
        wake_cpu: Some(1),
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    });
    sched.state.enqueue_task(1, TaskPriority::Normal as usize, tid);

    // min_depth=2 but victim has only 1 task — should return None.
    let result = sched.try_steal_one(0, 1, 2);
    assert_eq!(result, None, "should not steal when victim depth < min_depth");
}

#[test]
fn test_try_steal_one_steals_when_victim_meets_min_depth() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    // Two tasks on CPU 1.
    for tid in [10_010u64, 10_011u64] {
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(tid, TaskState::Runnable, TaskPriority::Normal),
        ));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(1),
            wake_cpu: Some(1),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(1, TaskPriority::Normal as usize, tid);
    }

    let stolen = sched.try_steal_one(0, 1, 2);
    assert!(stolen.is_some(), "should steal when victim has >= min_depth tasks");
    let stolen_id = stolen.unwrap();
    assert!(stolen_id == 10_010 || stolen_id == 10_011, "stolen task must come from victim CPU 1");
    assert_eq!(
        sched.state.get_task(stolen_id).and_then(|sf| sf.wake_cpu),
        Some(0),
        "stolen task wake_cpu must point to local CPU"
    );
    assert_eq!(
        sched.state.get_task(stolen_id).and_then(|sf| sf.runq_location),
        None,
        "stolen task must be removed from victim run queue"
    );
}

#[test]
fn test_try_steal_one_returns_none_for_same_cpu() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.mark_cpu_online(0);

    let tid = 10_020u64;
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(make_task(
        tid,
        TaskState::Runnable,
        TaskPriority::Normal,
    )));
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid,
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
    sched.state.enqueue_task(0, TaskPriority::Normal as usize, tid);

    // local_cpu == victim_cpu → always None (cannot steal from self).
    assert_eq!(sched.try_steal_one(0, 0, 1), None);
}

#[test]
fn test_try_steal_one_skips_pinned_tasks() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..2 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    sched.state.mark_cpu_online(0);
    sched.state.mark_cpu_online(1);

    // Two tasks on CPU 1, both pinned to CPU 1.
    for tid in [10_030u64, 10_031u64] {
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(tid, TaskState::Runnable, TaskPriority::Normal),
        ));
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
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(1, TaskPriority::Normal as usize, tid);
    }

    assert_eq!(sched.try_steal_one(0, 1, 2), None, "pinned tasks must never be stolen");
}

// ── idle_steal tests ───────────────────────────────────────────────────

#[test]
fn test_idle_steal_prefers_nearby_cpu_over_distant() {
    let _g = init_test_env();

    // CPU layout: local=0, nearby=1 (within radius), far=10.
    // We need at least 11 per_cpu slots.
    let num_cpus = 11usize;
    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..num_cpus {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    for i in [0usize, 1, 10] {
        sched.state.mark_cpu_online(i);
    }

    // Place two tasks on nearby CPU 1 and two tasks on far CPU 10.
    let nearby_tids = [10_100u64, 10_101u64];
    let far_tids = [10_110u64, 10_111u64];

    for &tid in &nearby_tids {
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(tid, TaskState::Runnable, TaskPriority::Normal),
        ));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(1),
            wake_cpu: Some(1),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(1, TaskPriority::Normal as usize, tid);
    }

    for &tid in &far_tids {
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(tid, TaskState::Runnable, TaskPriority::Normal),
        ));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(10),
            wake_cpu: Some(10),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(10, TaskPriority::Normal as usize, tid);
    }

    // idle_steal from CPU 0 should pick from nearby CPU 1 (within
    // STEAL_NEARBY_RADIUS) before trying distant CPU 10.
    let stolen = sched.idle_steal(0);
    assert!(stolen.is_some(), "idle_steal should find work");
    let stolen_id = stolen.unwrap();
    assert!(
        nearby_tids.contains(&stolen_id),
        "idle_steal should prefer nearby CPU 1 over far CPU 10 (stolen={stolen_id})"
    );
}

#[test]
fn test_idle_steal_falls_back_to_distant_when_nearby_empty() {
    let _g = init_test_env();

    let num_cpus = 11usize;
    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..num_cpus {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    for i in [0usize, 10] {
        sched.state.mark_cpu_online(i);
    }

    // Only far CPU 10 has work.
    let far_tids = [10_200u64, 10_201u64];
    for &tid in &far_tids {
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(tid, TaskState::Runnable, TaskPriority::Normal),
        ));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(10),
            wake_cpu: Some(10),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(10, TaskPriority::Normal as usize, tid);
    }

    let stolen = sched.idle_steal(0);
    assert!(
        stolen.is_some(),
        "idle_steal should fall back to distant CPUs when nearby CPUs have no work"
    );
    let stolen_id = stolen.unwrap();
    assert!(
        far_tids.contains(&stolen_id),
        "should steal from far CPU 10 when it is the only loaded CPU"
    );
}

#[test]
fn test_idle_steal_returns_none_when_all_cpus_have_single_task() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    for _ in 0..3 {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }
    for i in 0..3 {
        sched.state.mark_cpu_online(i);
    }

    // Each peer CPU has exactly 1 task — below the min_depth=2 threshold.
    for cpu in 1..3usize {
        let tid = 10_300u64 + cpu as u64;
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(tid, TaskState::Runnable, TaskPriority::Normal),
        ));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(cpu),
            wake_cpu: Some(cpu),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(cpu, TaskPriority::Normal as usize, tid);
    }

    assert_eq!(
        sched.idle_steal(0),
        None,
        "idle_steal must not steal when no CPU has >= 2 runnable tasks (anti-thrash)"
    );
}
