//! Blocking primitives for task synchronization.

use crate::BootRuntime;
use crate::BootTasking;
use crate::task::TaskState;

use super::SCHEDULER;

use super::types::Scheduler;

pub(crate) static BLOCK_CURRENT_HOOK: core::sync::atomic::AtomicPtr<()> =
    core::sync::atomic::AtomicPtr::new(core::ptr::null_mut());
pub(crate) static WAKE_TASK_HOOK: core::sync::atomic::AtomicPtr<()> =
    core::sync::atomic::AtomicPtr::new(core::ptr::null_mut());

/// Deferred writes to the canonical REGISTRY that must be applied **after**
/// releasing the SCHEDULER lock.
///
/// `wake_task_locked` populates this struct instead of calling `get_task_mut`
/// directly.  The caller (`wake_task`) applies the writes once the SCHEDULER
/// lock is no longer held, breaking the nested-lock pattern that was the
/// primary source of lock-convoy under SMP.
pub(crate) struct DeferredWakeUpdate {
    pub tid: u64,
    /// If `Some`, write this state to `Thread::state` in REGISTRY.
    pub new_state: Option<TaskState>,
    /// If `Some`, write this tick to `Thread::enqueued_at_tick` in REGISTRY.
    pub new_enqueued_at_tick: Option<u64>,
    /// If `true`, set `Thread::wake_pending = true` in REGISTRY.
    pub set_wake_pending: bool,
}

pub fn block_current<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    // Outcome of the scheduler decision made under the SCHEDULER lock.
    // Applied to the canonical REGISTRY *after* the lock is released.
    let mut deferred_tid: Option<u64> = None;
    // true  → had wake_pending; just clear it in REGISTRY and return.
    // false → task is blocking; write Blocked state to REGISTRY.
    let mut was_wake_pending = false;

    let (switch_params, deferred_prepare_ipis, deferred_registry_syncs) = {
        let wait_start = rt.mono_ticks();
        let lock = SCHEDULER.lock();
        super::set_sched_lock_tracking::<R>(rt.current_cpu_index());
        super::record_sched_lock_wait::<R>(
            &super::PROF_SCHED_WAIT_BLOCK_CURRENT_CALLS,
            &super::PROF_SCHED_WAIT_BLOCK_CURRENT_US_TOTAL,
            &super::PROF_SCHED_WAIT_BLOCK_CURRENT_US_MAX,
            &super::PROF_SCHED_WAIT_BLOCK_CURRENT_HIST,
            wait_start,
        );
        let lock_start = rt.mono_ticks();
        let ptr = lock.expect("Scheduler not initialized");
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };

        let cpu = super::current_cpu_index::<R>();
        let current_id = match sched.state.per_cpu.get(cpu).and_then(|pc| pc.current) {
            Some(id) => id,
            None => {
                super::record_sched_lock_hold::<R>(
                    &super::PROF_SCHED_LOCK_BLOCK_CURRENT_CALLS,
                    &super::PROF_SCHED_LOCK_BLOCK_CURRENT_US_TOTAL,
                    &super::PROF_SCHED_LOCK_BLOCK_CURRENT_US_MAX,
                    &super::PROF_SCHED_LOCK_BLOCK_CURRENT_HOLD_HIST,
                    lock_start,
                );
                rt.irq_restore(_irq);
                return;
            }
        };

        // Check and update wake_pending from the hot-field cache.
        // This avoids a nested REGISTRY lock on the check-and-early-return path
        // (the primary source of SCHEDULER↔REGISTRY lock contention under SMP).
        if let Some(sf) = sched.state.get_task_mut(current_id) {
            // Record tid for deferred REGISTRY write regardless of which path is taken.
            deferred_tid = Some(current_id);
            if sf.wake_pending {
                sf.wake_pending = false;
                was_wake_pending = true;
            } else {
                sf.state = TaskState::Blocked;
                if current_id == 6 {
                    crate::kdebug!("SCHED[TID6]: blocked");
                }
            }
        }

        if was_wake_pending {
            super::record_sched_lock_hold::<R>(
                &super::PROF_SCHED_LOCK_BLOCK_CURRENT_CALLS,
                &super::PROF_SCHED_LOCK_BLOCK_CURRENT_US_TOTAL,
                &super::PROF_SCHED_LOCK_BLOCK_CURRENT_US_MAX,
                &super::PROF_SCHED_LOCK_BLOCK_CURRENT_HOLD_HIST,
                lock_start,
            );
            // Return None (no context switch); deferred REGISTRY clear handled below.
            let deferred_prepare_ipis = sched.drain_pending_prepare_schedule_ipis();
            let deferred_registry_syncs = core::mem::take(&mut sched.pending_registry_syncs);
            (None, deferred_prepare_ipis, deferred_registry_syncs)
        } else {
            // Add to wait queue and pick next task to run.
            sched.state.wait_queue.push_back(current_id);
            let switch = sched.prepare_schedule();
            let deferred_prepare_ipis = sched.drain_pending_prepare_schedule_ipis();
            let deferred_registry_syncs = core::mem::take(&mut sched.pending_registry_syncs);
            super::record_sched_lock_hold::<R>(
                &super::PROF_SCHED_LOCK_BLOCK_CURRENT_CALLS,
                &super::PROF_SCHED_LOCK_BLOCK_CURRENT_US_TOTAL,
                &super::PROF_SCHED_LOCK_BLOCK_CURRENT_US_MAX,
                &super::PROF_SCHED_LOCK_BLOCK_CURRENT_HOLD_HIST,
                lock_start,
            );
            super::clear_sched_lock_tracking();
            (switch, deferred_prepare_ipis, deferred_registry_syncs)
        }
    };
    // SCHEDULER lock is released here.
    super::apply_deferred_registry_syncs::<R>(deferred_registry_syncs);

    // Apply deferred REGISTRY write outside SCHEDULER lock to avoid nesting.
    if let Some(tid) = deferred_tid {
        if was_wake_pending {
            // Wake was pending: task should not block.  Clear the flag in REGISTRY.
            if let Some(mut task) = crate::task::registry::get_task_mut::<R>(tid) {
                task.wake_pending = false;
            }
            rt.irq_restore(_irq);
            return;
        } else {
            // Task is genuinely blocking.  Persist Blocked state to REGISTRY.
            if let Some(mut task) = crate::task::registry::get_task_mut::<R>(tid) {
                task.state = TaskState::Blocked;
            }
        }
    }
    super::send_deferred_prepare_schedule_ipis::<R>(deferred_prepare_ipis);

    if let Some(switch) = switch_params {
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
    }

    rt.irq_restore(_irq);
}

/// Wake a blocked task. Returns the CPU to which a reschedule IPI should be
/// sent, or `None` if no cross-CPU IPI is needed.
///
/// **The caller must drop the `SCHEDULER` lock before sending any IPI** to
/// avoid holding the lock during IPI delivery.
///
/// Also returns a [`DeferredWakeUpdate`] that the caller **must** apply to
/// the canonical REGISTRY after releasing the SCHEDULER lock.  This breaks
/// the nested SCHEDULER → REGISTRY lock ordering that was the primary source
/// of lock-convoy behaviour under SMP.
pub fn wake_task_locked<R: BootRuntime>(
    sched: &mut Scheduler<R>,
    id: u64,
) -> (Option<usize>, Option<DeferredWakeUpdate>) {
    let mut wake_info: Option<(usize, usize)> = None;

    // Read scheduling fields from the scheduler-side hot-field cache. If the
    // task is currently blocked, compute the target CPU/priority from the cache
    // and update REGISTRY afterwards (via DeferredWakeUpdate).
    if let Some(sf) = sched.state.get_thread(id) {
        if sf.state == TaskState::Blocked {
            let task_priority = sf.priority as usize;
            let target_cpu = match sf.affinity {
                crate::task::Affinity::Pinned(cpu) => cpu,
                crate::task::Affinity::Any => {
                    let preferred = sf.last_cpu.unwrap_or_else(|| super::current_cpu_index::<R>());
                    super::select_any_affinity_wake_cpu::<R>(sched, preferred)
                }
            };
            wake_info = Some((target_cpu, task_priority));
        }
    }

    let deferred = if wake_info.is_some() {
        let tick = super::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed);
        let wake_mono = crate::runtime::<R>().mono_ticks();
        // Increment the profiling counter before updating the hot-field cache.
        // The counter tracks Runnable transitions regardless of whether the
        // cache update succeeds, so ordering relative to the cache write does
        // not affect correctness.  The tick snapshot and the cache write use
        // the same `tick` value to keep both consistent.
        super::PROF_RUNNABLE_TRANSITIONS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);

        // Keep the scheduler-side cache in sync without touching REGISTRY.
        if let Some(sf) = sched.state.get_thread_mut(id) {
            sf.state = TaskState::Runnable;
            sf.enqueued_at_tick = tick;
        }
        sched.state.wake_enqueued_at_mono.insert(id, wake_mono);
        sched
            .state
            .note_enqueue_cause(id, crate::sched::state::EnqueueCause::Wake);

        // Defer the canonical REGISTRY writes to the caller (outside SCHEDULER lock).
        Some(DeferredWakeUpdate {
            tid: id,
            new_state: Some(TaskState::Runnable),
            new_enqueued_at_tick: Some(tick),
            set_wake_pending: false,
        })
    } else {
        // Task is not blocked. Set wake_pending in the hot-field cache so the
        // next block_current returns immediately without actually blocking.
        if let Some(sf) = sched.state.get_thread_mut(id) {
            sf.wake_pending = true;
        }

        // Defer the canonical REGISTRY wake_pending write to the caller.
        Some(DeferredWakeUpdate {
            tid: id,
            new_state: None,
            new_enqueued_at_tick: None,
            set_wake_pending: true,
        })
    };

    if let Some((target_cpu, task_priority)) = wake_info {
        let mut safe_cpu = target_cpu;

        if let Some(pos) = sched.state.wait_queue.iter().position(|&wid| wid == id) {
            sched.state.wait_queue.remove(pos);
        }

        // Best-effort cleanup: the target may be blocked on non-timeout paths.
        let _ = sched.state.remove_task_from_sleep_queue(id);

        if safe_cpu >= sched.state.per_cpu.len() {
            safe_cpu = 0;
        }
        if let Some(sf) = sched.state.get_thread_mut(id) {
            sf.wake_cpu = Some(safe_cpu);
        }

        sched.state.enqueue_task(safe_cpu, task_priority, id);
        if let Some(pc) = sched.state.per_cpu.get_mut(safe_cpu) {
            pc.stats.wakeups = pc.stats.wakeups.saturating_add(1);
        }

        // Use the cached priority of the current task to avoid a nested REGISTRY lock.
        let current_prio = sched.state.per_cpu[safe_cpu]
            .current
            .and_then(|cid| sched.state.get_thread(cid))
            .map(|sf| sf.priority as usize)
            .unwrap_or(0);

        // Nudge logic:
        // - Higher priority than current
        // - Equal priority (to trigger round-robin preemption)
        // - Current is idle_task
        let is_idle =
            sched.state.per_cpu[safe_cpu].current == sched.state.per_cpu[safe_cpu].idle_task;

        if task_priority >= current_prio || is_idle {
            if safe_cpu == super::current_cpu_index::<R>() {
                sched.state.per_cpu[safe_cpu].need_resched = true;
            } else {
                // Only request a remote IPI if the pending flag was not already set.
                // A set flag means a previous IPI is already in flight.
                let already_pending = super::set_global_need_resched(safe_cpu);
                if !already_pending {
                    return (Some(safe_cpu), deferred);
                } else {
                    super::PROF_IPI_SUPPRESSED
                        .fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                }
            }
        }
    }

    (None, deferred)
}

pub fn wake_task<R: BootRuntime>(id: u64) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    // Fast path: if wake is already pending, there is no additional scheduler
    // work to do and we can avoid taking SCHEDULER.
    // `wake_pending` is level-triggered; a concurrent clear means a blocker has
    // consumed the wake and therefore does not require another enqueue here.
    let already_pending = crate::task::registry::get_task::<R>(id)
        .map(|task| task.wake_pending)
        .unwrap_or(false);
    if already_pending {
        super::PROF_WAKE_TASK_FASTPATH_ALREADY_PENDING
            .fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        rt.irq_restore(_irq);
        return;
    }

    let wait_start = rt.mono_ticks();

    // Collect any pending IPI target and deferred REGISTRY update inside the
    // lock, then apply both *after* the lock is dropped to avoid holding
    // SCHEDULER during IPI delivery and to eliminate the nested REGISTRY lock.
    let (ipi_cpu, deferred) = {
        let lock_sched = SCHEDULER.lock();
        super::set_sched_lock_tracking::<R>(rt.current_cpu_index());
        super::record_sched_lock_wait::<R>(
            &super::PROF_SCHED_WAIT_WAKE_TASK_CALLS,
            &super::PROF_SCHED_WAIT_WAKE_TASK_US_TOTAL,
            &super::PROF_SCHED_WAIT_WAKE_TASK_US_MAX,
            &super::PROF_SCHED_WAIT_WAKE_TASK_HIST,
            wait_start,
        );
        let lock_start = rt.mono_ticks();

        let result = if let Some(ptr) = *lock_sched {
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
            wake_task_locked::<R>(sched, id)
        } else {
            (None, None)
        };

        super::record_sched_lock_hold::<R>(
            &super::PROF_SCHED_LOCK_WAKE_TASK_CALLS,
            &super::PROF_SCHED_LOCK_WAKE_TASK_US_TOTAL,
            &super::PROF_SCHED_LOCK_WAKE_TASK_US_MAX,
            &super::PROF_SCHED_LOCK_WAKE_TASK_HOLD_HIST,
            lock_start,
        );
        super::clear_sched_lock_tracking();

        result
    };
    // SCHEDULER lock released here.

    // Apply the deferred REGISTRY update outside the SCHEDULER lock to avoid
    // the nested SCHEDULER → REGISTRY lock ordering that caused contention.
    if let Some(update) = deferred {
        if let Some(mut task) = crate::task::registry::get_task_mut::<R>(update.tid) {
            if let Some(state) = update.new_state {
                task.state = state;
            }
            if let Some(tick) = update.new_enqueued_at_tick {
                task.enqueued_at_tick = tick;
            }
            if update.set_wake_pending {
                task.wake_pending = true;
            }
        }
    }

    if let Some(cpu) = ipi_cpu {
        super::DIAG_IPI_SENT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        super::DIAG_IPI_SENT_WAKE_TASK.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        rt.send_ipi(cpu, 0x30);
    }

    rt.irq_restore(_irq);
}

/// Type-erased block for use from IRQ module
pub unsafe fn block_current_erased() {
    let ptr = BLOCK_CURRENT_HOOK.load(core::sync::atomic::Ordering::SeqCst);
    if !ptr.is_null() {
        let hook: fn() = unsafe { core::mem::transmute(ptr) };
        hook();
    }
}

/// Type-erased wake for use from IRQ module
pub unsafe fn wake_task_erased(id: u64) {
    let ptr = WAKE_TASK_HOOK.load(core::sync::atomic::Ordering::SeqCst);
    if !ptr.is_null() {
        let hook: fn(u64) = unsafe { core::mem::transmute(ptr) };
        hook(id);
    }
}

pub fn init_blocking_hooks<R: BootRuntime>() {
    BLOCK_CURRENT_HOOK.store(
        block_current::<R> as *mut (),
        core::sync::atomic::Ordering::SeqCst,
    );
    WAKE_TASK_HOOK.store(
        wake_task::<R> as *mut (),
        core::sync::atomic::Ordering::SeqCst,
    );
}
