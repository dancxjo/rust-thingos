use core::sync::atomic::Ordering;

use super::super::*;
use super::support::*;
use crate::task::{Affinity, TaskId, TaskPriority, TaskState};
use crate::{BootRuntimeBase, BootTasking};

#[test]
fn test_task_cpu_trace_strings_prefers_scheduler_trace_fields() {
    let sf = crate::sched::state::ThreadSchedFields {
        tid: 1,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(3),
        wake_cpu: Some(2),
        run_cpu: Some(1),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    };

    let (last, wake, run) = task_cpu_trace_strings(Some(9), Some(&sf));
    assert_eq!(last, "3");
    assert_eq!(wake, "2");
    assert_eq!(run, "1");
}

#[test]
fn test_task_cpu_trace_strings_falls_back_to_task_last_cpu() {
    let sf = crate::sched::state::ThreadSchedFields {
        tid: 2,
        runq_location: None,
        state: TaskState::Runnable,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: None,
        wake_cpu: None,
        run_cpu: None,
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        enqueued_at_tick: 0,
        wake_pending: false,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
    };

    let (last, wake, run) = task_cpu_trace_strings(Some(4), Some(&sf));
    assert_eq!(last, "4");
    assert_eq!(wake, "-");
    assert_eq!(run, "-");
}

#[test]
fn test_format_optional_cpu_formats_some_and_none() {
    assert_eq!(format_optional_cpu(Some(7)), "7");
    assert_eq!(format_optional_cpu(None), "-");
}

#[test]
fn test_trylock_miss_records_pending_resched_pressure() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(0);

    let mut lock = SCHEDULER.lock();
    *lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);

    let before_miss = PROF_TRYLOCK_MISS_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);
    let before_pending =
        PROF_TRYLOCK_MISS_PENDING_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);
    let before_timer =
        PROF_TRYLOCK_MISS_TIMER_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);
    let before_ipi = PROF_TRYLOCK_MISS_IPI_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);
    assert!(global_need_resched_slot(0).is_some(), "test requires CPU 0 GLOBAL_NEED_RESCHED slot");
    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);
    assert!(
        !global_need_resched_swap(0, true, core::sync::atomic::Ordering::Release),
        "setup should mark CPU 0 resched flag from clear->set transition"
    );

    // Must run while SCHEDULER lock is held so try_lock path fails.
    try_resched_if_needed::<MockRuntime>(DispatchTrigger::ReschedIpi);

    let after_miss = PROF_TRYLOCK_MISS_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);
    let after_pending =
        PROF_TRYLOCK_MISS_PENDING_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);
    assert!(after_miss > before_miss, "[contention] per-CPU trylock miss counter should increment");
    assert!(
        after_pending > before_pending,
        "[contention] per-CPU pending-resched trylock miss counter should increment"
    );
    assert!(
        PROF_TRYLOCK_MISS_IPI_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed) > before_ipi,
        "[contention] IPI-triggered trylock miss counter should increment"
    );
    assert_eq!(
        PROF_TRYLOCK_MISS_TIMER_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed),
        before_timer,
        "[contention] timer-triggered trylock miss counter must not increment for IPI dispatch"
    );

    drop(lock);
    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_trylock_miss_timer_attribution() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(0);

    let mut lock = SCHEDULER.lock();
    *lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);

    let before_timer =
        PROF_TRYLOCK_MISS_TIMER_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);
    let before_ipi = PROF_TRYLOCK_MISS_IPI_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);

    try_resched_if_needed::<MockRuntime>(DispatchTrigger::TimerTick);

    assert!(
        PROF_TRYLOCK_MISS_TIMER_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed)
            > before_timer,
        "[contention] timer-triggered trylock miss counter should increment"
    );
    assert_eq!(
        PROF_TRYLOCK_MISS_IPI_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed),
        before_ipi,
        "[contention] IPI-triggered trylock miss counter must not increment for timer dispatch"
    );

    drop(lock);
    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_trylock_miss_idle_timer_without_pending_does_not_set_resched_flag() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(123);
    sched.state.per_cpu[0].idle_task = Some(123);

    let mut lock = SCHEDULER.lock();
    *lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);

    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);
    crate::runtime::<MockRuntime>().set_idle_task_current(true);
    let before_window_actionable = TRYLOCK_MISS_WINDOW_COUNT[0].load(Ordering::Relaxed);
    let before_window_idle_timer = TRYLOCK_MISS_WINDOW_IDLE_TIMER_COUNT[0].load(Ordering::Relaxed);
    let before_idle_timer =
        PROF_TRYLOCK_MISS_IDLE_TIMER_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);

    try_resched_if_needed::<MockRuntime>(DispatchTrigger::TimerTick);

    assert!(
        !global_need_resched_load(0, core::sync::atomic::Ordering::Acquire),
        "idle timer-only trylock miss should not force pending resched"
    );
    assert_eq!(
        TRYLOCK_MISS_WINDOW_COUNT[0].load(Ordering::Relaxed),
        before_window_actionable,
        "idle timer-only misses should not count toward actionable warning threshold"
    );
    assert!(
        TRYLOCK_MISS_WINDOW_IDLE_TIMER_COUNT[0].load(Ordering::Relaxed) > before_window_idle_timer,
        "idle timer-only misses should be attributed in the dedicated window counter"
    );
    assert!(
        PROF_TRYLOCK_MISS_IDLE_TIMER_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed)
            > before_idle_timer,
        "idle timer-only misses should increment dedicated attribution counter"
    );

    crate::runtime::<MockRuntime>().set_idle_task_current(false);
    drop(lock);
    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_dispatch_trigger_labels() {
    assert_eq!(DispatchTrigger::TimerTick.as_str(), "timer_tick");
    assert_eq!(DispatchTrigger::ReschedIpi.as_str(), "resched_ipi");
}

#[test]
fn test_try_resched_timer_tick_updates_tick_stats_and_timeslice() {
    let _g = init_test_env();

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    let tid = 77;
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid,
        runq_location: None,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        affinity: Affinity::Any,
        last_cpu: Some(0),
        wake_cpu: Some(0),
        run_cpu: Some(0),
        timeslice_remaining: 2,
        enqueued_at_tick: 0,
        voluntary_yields: 0,
        migration_state: MigrationState::Local,
        wake_pending: false,
    });
    sched.state.per_cpu[0].current = Some(tid);
    sched.state.per_cpu[0].idle_task = Some(999);

    let mut lock = SCHEDULER.lock();
    *lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(lock);

    try_resched_if_needed::<MockRuntime>(DispatchTrigger::TimerTick);

    let pc = &sched.state.per_cpu[0];
    assert_eq!(pc.stats.timer_interrupts, 1, "timer dispatch should update tick stats");
    assert_eq!(pc.stats.resched_ipi_received, 0, "timer dispatch must not count as an IPI");
    assert_eq!(
        sched.state.get_thread(tid).map(|sf| sf.timeslice_remaining),
        Some(1),
        "timer dispatch should execute preempt-tick bookkeeping"
    );

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
}

#[test]
fn test_set_global_need_resched_invalid_cpu_returns_conservative_fallback() {
    let _g = init_test_env();
    let invalid_cpu = types::MAX_CPUS;

    assert!(
        set_global_need_resched(invalid_cpu),
        "invalid cpu should conservatively report an already-pending resched"
    );
}

#[test]
fn test_global_need_resched_load_invalid_cpu_returns_conservative_fallback() {
    let _g = init_test_env();
    let invalid_cpu = types::MAX_CPUS;

    assert!(
        global_need_resched_load(invalid_cpu, core::sync::atomic::Ordering::Acquire),
        "invalid cpu should conservatively report pending resched"
    );
}

// ── per-CPU preemption tests ──────────────────────────────────────────────

/// `need_resched_pending` returns false when the per-CPU atomic flag is clear.
#[test]
fn test_need_resched_pending_returns_false_when_clear() {
    let _g = init_test_env();
    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);
    assert!(
        !need_resched_pending(0),
        "need_resched_pending should return false when the per-CPU flag is clear"
    );
}

/// `need_resched_pending` returns true after `set_global_need_resched` is called.
#[test]
fn test_need_resched_pending_returns_true_after_set() {
    let _g = init_test_env();
    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);
    set_global_need_resched(0);
    assert!(
        need_resched_pending(0),
        "need_resched_pending should return true after set_global_need_resched"
    );
    // Cleanup
    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);
}

/// Setting `CpuScheduler.need_resched = true` when preempt is disabled also
/// mirrors the flag to the per-CPU atomic so the lockless fast-path sees it.
///
/// Flow under test:
/// 1. `set_global_need_resched(0)` raises the per-CPU atomic flag.
/// 2. `schedule_point(PreemptTick)` atomically *clears* the flag via
///    `global_need_resched_swap(cpu_idx, false)` and records it as
///    `global_requested = true`.
/// 3. Because `preempt_disable_depth > 0`, the reschedule is deferred:
///    `need_resched = true` is set AND `set_global_need_resched(cpu_idx)`
///    re-raises the per-CPU atomic flag.
/// 4. After returning, `need_resched_pending(0)` must be `true`.
///    Without the mirroring call added in step 3, the flag would remain
///    cleared (step 2) and this assertion would fail.
#[test]
fn test_schedule_point_preempt_disabled_mirrors_need_resched_to_atomic() {
    let _g = init_test_env();
    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    sched.state.per_cpu[0].current = Some(1);
    // Disable preemption on CPU 0
    sched.state.per_cpu[0].preempt_disable_depth = 1;

    let mut lock = SCHEDULER.lock();
    *lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
    drop(lock);

    // Simulate a global reschedule request arriving while preemption is disabled.
    set_global_need_resched(0);

    // Verify the flag is set before the call.
    assert!(need_resched_pending(0), "pre-condition: flag should be set before schedule_point");

    {
        let l = SCHEDULER.lock();
        if let Some(ptr) = *l {
            let s = unsafe { &mut *(ptr as *mut types::Scheduler<MockRuntime>) };
            // schedule_point internally clears the atomic flag via
            // global_need_resched_swap(cpu_idx, false), then — because
            // preempt_disable_depth > 0 — calls set_global_need_resched to
            // re-raise it.  The assertion below catches the case where the
            // mirroring call is absent (flag stays cleared after the swap).
            s.schedule_point(ScheduleReason::PreemptTick);
        }
        drop(l);
    }

    // The per-CPU lock-free flag must be re-raised by the mirroring in
    // schedule_point.  Without mirroring the swap(false) above would have
    // left it cleared.
    assert!(
        need_resched_pending(0),
        "schedule_point with preempt disabled should mirror need_resched back to the atomic flag"
    );
    // The local (in-lock) need_resched field must also be set.
    assert!(
        sched.state.per_cpu[0].need_resched,
        "CpuScheduler.need_resched should be set when preempt is disabled and a resched was requested"
    );

    let mut sched_lock = SCHEDULER.lock();
    *sched_lock = None;
    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);
}

/// Draining the remote wake mailbox with a higher-priority task sets
/// `need_resched` and mirrors it to the per-CPU atomic.
#[test]
fn test_drain_wake_mailbox_mirrors_need_resched_to_atomic() {
    let _g = init_test_env();
    TICK_COUNT.store(50, Ordering::Relaxed);
    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);

    let mut sched = types::Scheduler::<MockRuntime>::new();
    sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

    // Current task on CPU 0 at Normal priority
    let current_tid = 500u64;
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: current_tid,
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
    sched.state.per_cpu[0].current = Some(current_tid);

    // Push a Realtime task into the mailbox for CPU 0
    let rt_tid = 501u64;
    sched.state.insert_task(crate::sched::state::ThreadSchedFields {
        tid: rt_tid,
        runq_location: None,
        state: TaskState::Blocked,
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
    enqueue_remote_wake_mailbox(
        0,
        types::RemoteWakeMailboxEntry {
            tid: rt_tid,
            priority: TaskPriority::Realtime as usize,
            wake_mono: 0,
            enqueued_at_tick: 50,
        },
    );

    sched.drain_remote_wake_mailbox(0);

    assert!(
        sched.state.per_cpu[0].need_resched,
        "need_resched should be set after draining a higher-priority wake"
    );
    assert!(
        need_resched_pending(0),
        "per-CPU atomic flag should be mirrored after drain sets need_resched"
    );

    clear_global_need_resched(0, core::sync::atomic::Ordering::Release);
}
