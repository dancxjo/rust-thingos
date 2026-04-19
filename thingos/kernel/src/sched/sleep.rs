//! Timing and yield functions.

use super::SCHEDULER;
use super::types::{ScheduleReason, Scheduler};
use crate::{BootRuntime, BootTasking};

/// Cooperative yield: attempt to switch to the next runnable task.
///
/// Returns `true` if a context switch occurred **or** there is runnable work
/// in the queues (i.e. the CPU should stay awake). Returns `false` when all
/// queues are empty and the caller may safely halt (HLT / WFI).
pub fn yield_now<R: BootRuntime>() -> bool {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let cpu_idx = super::current_cpu_index::<R>();

    let (switch_decision, has_work, deferred_prepare_ipis, deferred_registry_syncs) = {
        let wait_start = rt.mono_ticks();
        let lock = SCHEDULER.lock();
        super::set_sched_lock_tracking::<R>(cpu_idx);
        super::record_sched_lock_wait::<R>(
            &super::PROF_SCHED_WAIT_YIELD_NOW_CALLS,
            &super::PROF_SCHED_WAIT_YIELD_NOW_US_TOTAL,
            &super::PROF_SCHED_WAIT_YIELD_NOW_US_MAX,
            &super::PROF_SCHED_WAIT_YIELD_NOW_HIST,
            wait_start,
        );
        let lock_start = rt.mono_ticks();
        let ptr = lock.expect("Scheduler not initialized");
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
        let sp = sched.schedule_point(ScheduleReason::CooperativeYield);
        let work = sched.has_runnable_work(cpu_idx);
        let deferred_prepare_ipis = sched.drain_pending_prepare_schedule_ipis();
        let deferred_registry_syncs = core::mem::take(&mut sched.pending_registry_syncs);
        super::record_sched_lock_hold::<R>(
            &super::PROF_SCHED_LOCK_YIELD_NOW_CALLS,
            &super::PROF_SCHED_LOCK_YIELD_NOW_US_TOTAL,
            &super::PROF_SCHED_LOCK_YIELD_NOW_US_MAX,
            &super::PROF_SCHED_LOCK_YIELD_NOW_HOLD_HIST,
            lock_start,
        );
        super::clear_sched_lock_tracking::<R>();
        (sp, work, deferred_prepare_ipis, deferred_registry_syncs)
    };
    super::apply_deferred_registry_syncs::<R>(deferred_registry_syncs);
    super::send_deferred_prepare_schedule_ipis::<R>(deferred_prepare_ipis);

    if let Some(decision) = switch_decision {
        let mut ghost_ctx = <R::Tasking as BootTasking>::Context::default();
        let mut ghost_fs_base = 0;
        let Some(switch) =
            super::resolve_switch_params::<R>(decision, &mut ghost_ctx, &mut ghost_fs_base)
        else {
            rt.irq_restore(_irq);
            return has_work;
        };
        rt.tasking().activate_address_space(switch.to_aspace);

        unsafe {
            rt.tasking().switch_with_tls(
                &mut *switch.from_ctx,
                &*switch.to_ctx,
                switch.to_tid,
                switch.from_user_fs_base,
                switch.to_user_fs_base,
            );
        }

        rt.irq_restore(_irq);
        return true; // switched
    }

    rt.irq_restore(_irq);
    has_work
}

/// True blocking sleep - puts task in sleep queue and reschedules
pub fn sleep_ticks<R: BootRuntime>(ticks: u64) {
    if ticks == 0 {
        // Zero sleep = just yield once
        yield_now::<R>();
        return;
    }

    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    // Both switch_decision and the deferred REGISTRY state update are returned
    // from the SCHEDULER lock scope so that REGISTRY is written outside the
    // lock, avoiding the nested SCHEDULER → REGISTRY lock ordering.
    let (switch_decision, deferred_state, deferred_prepare_ipis, deferred_registry_syncs) = {
        let wait_start = rt.mono_ticks();
        let lock = SCHEDULER.lock();
        super::set_sched_lock_tracking::<R>(super::current_cpu_index::<R>());
        super::record_sched_lock_wait::<R>(
            &super::PROF_SCHED_WAIT_SLEEP_TICKS_CALLS,
            &super::PROF_SCHED_WAIT_SLEEP_TICKS_US_TOTAL,
            &super::PROF_SCHED_WAIT_SLEEP_TICKS_US_MAX,
            &super::PROF_SCHED_WAIT_SLEEP_TICKS_HIST,
            wait_start,
        );
        let lock_start = rt.mono_ticks();
        let ptr = lock.expect("Scheduler not initialized");
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };

        // Get current task ID
        let current_id = {
            let cpu = super::current_cpu_index::<R>();
            match sched.state.per_cpu.get(cpu).and_then(|pc| pc.current) {
                Some(id) => {
                    // crate::ktrace!(
                    //     "SCHED: CPU {} task {} sleeping for {} ticks",
                    //     cpu, id, ticks
                    // );
                    id
                }
                None => {
                    // No current task (shouldn't happen). Restore interrupts
                    // before returning so we do not leave the CPU with IRQs
                    // permanently disabled on this early-exit path.
                    rt.irq_restore(_irq);
                    return;
                }
            }
        };

        // Calculate wake time and add to sleep queue
        let wake_tick = super::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed) + ticks;
        sched.state.add_task_to_sleep_queue(current_id, wake_tick);

        // Update the scheduler-side hot-field cache immediately; the canonical
        // REGISTRY write is deferred to after the SCHEDULER lock is released to
        // avoid the nested SCHEDULER → REGISTRY lock pattern.
        if let Some(sf) = sched.state.get_task_mut(current_id) {
            sf.state = crate::task::TaskState::Blocked;
        }

        // Do NOT push current task to runq - it's now sleeping
        // Just call prepare_schedule to pick next task
        let switch = sched.prepare_schedule();
        let deferred_prepare_ipis = sched.drain_pending_prepare_schedule_ipis();
        let deferred_registry_syncs = core::mem::take(&mut sched.pending_registry_syncs);

        // Determine the final REGISTRY state to write after the lock is released.
        let final_state = if switch.is_none() {
            // If no context switch is available (e.g. no idle task during early
            // boot, or the only runnable task is the current one), undo the sleep
            // enrollment so the task does not stay Blocked while still running on
            // the CPU.  The caller will wait for the next timer interrupt below
            // instead of spinning on the SCHEDULER lock.
            let _ = sched.state.remove_task_from_sleep_queue(current_id);
            if let Some(sf) = sched.state.get_task_mut(current_id) {
                sf.state = crate::task::TaskState::Running;
            }
            crate::task::TaskState::Running
        } else {
            crate::task::TaskState::Blocked
        };

        super::record_sched_lock_hold::<R>(
            &super::PROF_SCHED_LOCK_SLEEP_TICKS_CALLS,
            &super::PROF_SCHED_LOCK_SLEEP_TICKS_US_TOTAL,
            &super::PROF_SCHED_LOCK_SLEEP_TICKS_US_MAX,
            &super::PROF_SCHED_LOCK_SLEEP_TICKS_HOLD_HIST,
            lock_start,
        );
        super::clear_sched_lock_tracking::<R>();
        (switch, (current_id, final_state), deferred_prepare_ipis, deferred_registry_syncs)
    };
    // SCHEDULER lock released here.
    super::apply_deferred_registry_syncs::<R>(deferred_registry_syncs);
    super::send_deferred_prepare_schedule_ipis::<R>(deferred_prepare_ipis);

    // Apply the deferred REGISTRY write outside the SCHEDULER lock.
    let (deferred_tid, deferred_task_state) = deferred_state;
    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(deferred_tid) {
        task.state = deferred_task_state;
    }

    if let Some(decision) = switch_decision {
        let mut ghost_ctx = <R::Tasking as BootTasking>::Context::default();
        let mut ghost_fs_base = 0;
        let Some(switch) =
            super::resolve_switch_params::<R>(decision, &mut ghost_ctx, &mut ghost_fs_base)
        else {
            rt.irq_restore(_irq);
            return;
        };
        rt.tasking().activate_address_space(switch.to_aspace);

        unsafe {
            rt.tasking().switch_with_tls(
                &mut *switch.from_ctx,
                &*switch.to_ctx,
                switch.to_tid,
                switch.from_user_fs_base,
                switch.to_user_fs_base,
            );
        }
        // crate::ktrace!("SCHED: task woke up on CPU");
        rt.irq_restore(_irq);
    } else {
        // No context switch was possible.  Restore interrupts and halt until
        // the next timer interrupt fires.  This prevents the outer loop in
        // sleep_until (and other callers) from busy-spinning on the SCHEDULER
        // lock when no other task is available to run.
        rt.irq_restore(_irq);
        rt.wait_for_interrupt();
    }
}

/// Sleep until an absolute deadline expressed in `rt.mono_ticks()` units.
///
/// Delegates to `sleep_ticks`, which enqueues the task in the timer-backed
/// sleep queue and blocks it with a proper context switch (or halts the CPU
/// via `wait_for_interrupt` when no other task is runnable).  This avoids
/// the lock-acquire storm that a busy-spin loop would cause under SMP.
///
/// # Accuracy
/// Resolution is bounded by the 100 Hz timer tick (≈ 10 ms).  The function
/// may sleep slightly longer than the requested deadline (up to one extra
/// tick) because it rounds up the remaining-tick count to prevent
/// undersleeping.  The outer loop re-checks the deadline on wakeup to handle
/// early wakeups (e.g. due to `wait_for_interrupt` returning early on a
/// non-timer interrupt) without re-acquiring the SCHEDULER lock in a tight spin.
pub fn sleep_until<R: BootRuntime>(deadline_ticks: u64) {
    let rt = crate::runtime::<R>();
    loop {
        let now = rt.mono_ticks();
        if now >= deadline_ticks {
            break;
        }

        // Convert the remaining monotonic-clock ticks to 100 Hz scheduler
        // ticks so we can delegate to sleep_ticks (queue-based, non-spinning).
        let remaining_mono = deadline_ticks - now;
        let freq = rt.mono_freq_hz().max(1);
        // ticks_per_sched ≈ freq / 100 (i.e. mono ticks per scheduler tick)
        let ticks_per_sched = freq / 100;
        let sched_ticks = if ticks_per_sched > 0 {
            // Round up to avoid undersleeping.
            (remaining_mono + ticks_per_sched - 1) / ticks_per_sched
        } else {
            1
        }
        .max(1);

        sleep_ticks::<R>(sched_ticks);
        // After wakeup, re-check the deadline.  This also handles the case
        // where sleep_ticks returned early (e.g. interrupted or rounded down).
    }
}

pub fn sleep_ms<R: BootRuntime>(ms: u64) {
    // 1 tick is 10ms (100Hz timer)
    // Round up to avoid undersleeping.
    // ms=0 converts to ticks=0, which sleep_ticks handles as a yield.
    let ticks = (ms + 9) / 10;
    sleep_ticks::<R>(ticks);
}
