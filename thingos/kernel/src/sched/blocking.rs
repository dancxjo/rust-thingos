//! Blocking primitives for task synchronization.

use super::SCHEDULER;
use super::state::WaitReason;
use super::types::Scheduler;
use crate::task::TaskState;
use crate::{BootRuntime, BootTasking};

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

fn recover_failed_block_current<R: BootRuntime>(tid: u64) {
    let rt = crate::runtime::<R>();

    {
        let lock = SCHEDULER.lock();
        super::set_sched_lock_tracking::<R>(rt.current_cpu_index());
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
            let cpu = super::current_cpu_index::<R>();
            sched.state.unregister_waiter(tid);
            if let Some(sf) = sched.state.get_task_mut(tid) {
                sf.state = TaskState::Running;
                sf.run_cpu = Some(cpu);
                sf.last_cpu = Some(cpu);
                sf.wake_pending = false;
            }
        }
        super::clear_sched_lock_tracking::<R>();
    }

    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(tid) {
        task.state = TaskState::Running;
        task.last_cpu = Some(rt.current_cpu_index());
        task.wake_pending = false;
    }
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

    let (switch_decision, deferred_prepare_ipis, deferred_registry_syncs) = {
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
                super::clear_sched_lock_tracking::<R>();
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
            super::clear_sched_lock_tracking::<R>();
            // Return None (no context switch); deferred REGISTRY clear handled below.
            let deferred_prepare_ipis = sched.drain_pending_prepare_schedule_ipis();
            let deferred_registry_syncs = core::mem::take(&mut sched.pending_registry_syncs);
            (None, deferred_prepare_ipis, deferred_registry_syncs)
        } else {
            // Add to wait queue and pick next task to run.
            let _ = sched.state.register_waiter(current_id, WaitReason::BlockCurrent);
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
            super::clear_sched_lock_tracking::<R>();
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

    if let Some(decision) = switch_decision {
        let mut ghost_ctx = <R::Tasking as BootTasking>::Context::default();
        let mut ghost_fs_base = 0;
        let Some(switch) =
            super::resolve_switch_params::<R>(decision, &mut ghost_ctx, &mut ghost_fs_base)
        else {
            if let Some(tid) = deferred_tid.filter(|_| !was_wake_pending) {
                crate::kwarn!(
                    "SCHED: block_current failed to resolve switch params for tid {}; restoring task state",
                    tid
                );
                recover_failed_block_current::<R>(tid);
            }
            rt.irq_restore(_irq);
            return;
        };
        while crate::sched::is_task_on_any_cpu(switch.to_tid) {
            core::hint::spin_loop();
        }

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

        crate::sched::set_cpu_current_task(
            crate::runtime::<R>().current_cpu_index(),
            crate::runtime::<R>().current_tid(),
        );
    } else if let Some(tid) = deferred_tid.filter(|_| !was_wake_pending) {
        crate::kwarn!(
            "SCHED: block_current produced no switch for tid {}; restoring task state",
            tid
        );
        recover_failed_block_current::<R>(tid);
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
                    super::choose_wake_cpu::<R>(sched, sf.last_cpu)
                }
                crate::task::Affinity::Restricted(ref aff) => {
                    let cpu_count = sched.state.per_cpu.len().max(1);
                    // Prefer last_cpu within the allowed set; fall back to
                    // the affinity's own pick which respects preferred + last_cpu.
                    aff.pick_cpu(cpu_count)
                        .unwrap_or_else(|| super::choose_wake_cpu::<R>(sched, sf.last_cpu))
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
        sched.state.note_enqueue_cause(id, crate::sched::state::EnqueueCause::Wake);

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

        sched.state.unregister_waiter(id);

        // Best-effort cleanup: the target may be blocked on non-timeout paths.
        let _ = sched.state.remove_task_from_sleep_queue(id);
        sched.sleep_duration_ticks_by_tid.remove(&id);

        if safe_cpu >= sched.state.per_cpu.len() {
            safe_cpu = 0;
        }
        if let Some(sf) = sched.state.get_thread_mut(id) {
            // Track cross-CPU wake routing using MigrationState.
            // When the task is woken onto a different CPU than it last ran on,
            // record a Requested transition so the migration is observable.
            let last = sf.last_cpu;
            if last.is_some_and(|c| c != safe_cpu) {
                match sf.migration_state.try_transition(
                    crate::sched::state::MigrationState::Requested { target: safe_cpu },
                ) {
                    Ok(new_state) => {
                        crate::kdebug!(
                            "MIGRATE[tid={}]: {:?} → Requested {{ target: cpu{} }} (wake from cpu{:?})",
                            id,
                            sf.migration_state,
                            safe_cpu,
                            last,
                        );
                        sf.migration_state = new_state;
                    }
                    Err(bad_state) => {
                        // Non-fatal: migration state already reflects a
                        // pending transition (e.g. already InTransit or
                        // Requested from a parallel path).  Log so the
                        // situation is visible for debugging.
                        crate::kdebug!(
                            "MIGRATE[tid={}]: wake routing to cpu{} blocked by state {:?} \
                             (last_cpu={:?}) — transition skipped",
                            id,
                            safe_cpu,
                            bad_state,
                            last,
                        );
                    }
                }
            }
            sf.wake_cpu = Some(safe_cpu);
        }

        let current_cpu = super::current_cpu_index::<R>();
        if safe_cpu == current_cpu {
            // Local CPU: enqueue directly into the local run queue.
            sched.state.enqueue_task(safe_cpu, task_priority, id);
        } else {
            // Remote CPU: route through the per-CPU wake mailbox so the owning
            // CPU enqueues the task itself.  This preserves the per-CPU
            // ownership invariant: only the owning CPU mutates its run queue.
            let now_tick = super::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed);
            let wake_mono = crate::runtime::<R>().mono_ticks();
            super::enqueue_remote_wake_mailbox(
                safe_cpu,
                crate::sched::types::RemoteWakeMailboxEntry {
                    tid: id,
                    priority: task_priority,
                    enqueued_at_tick: now_tick,
                    wake_mono,
                },
            );
        }
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
                    super::PROF_IPI_SUPPRESSED.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                }
            }
        }
    }

    (None, deferred)
}

fn try_remote_wake_via_mailbox<R: BootRuntime>(id: u64) -> bool {
    let rt = crate::runtime::<R>();
    let current_cpu = rt.current_cpu_index();
    let now_tick = super::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed);
    let wake_mono = rt.mono_ticks();

    let mut remote_wake: Option<(usize, usize)> = None;

    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(id) {
        if task.state == TaskState::Runnable || task.state == TaskState::Dead {
            return true;
        }
        if task.state != TaskState::Blocked {
            return false;
        }

        let target_cpu = match task.affinity {
            crate::task::Affinity::Pinned(cpu) => cpu,
            crate::task::Affinity::Any => task.last_cpu.unwrap_or(current_cpu),
            crate::task::Affinity::Restricted(ref aff) => {
                let cpu_total = rt.cpu_total_count().max(1);
                aff.pick_cpu(cpu_total)
                    .unwrap_or_else(|| task.last_cpu.unwrap_or(current_cpu))
            }
        };
        let cpu_total = rt.cpu_total_count().max(1);
        let safe_cpu = target_cpu.min(cpu_total.saturating_sub(1));
        if safe_cpu == current_cpu {
            return false;
        }

        task.state = TaskState::Runnable;
        task.enqueued_at_tick = now_tick;
        task.wake_pending = false;
        remote_wake = Some((safe_cpu, task.priority as usize));
    } else {
        return true;
    }

    let Some((safe_cpu, priority)) = remote_wake else {
        return false;
    };

    super::PROF_RUNNABLE_TRANSITIONS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    let mailbox_cpu = super::enqueue_remote_wake_mailbox(
        safe_cpu,
        crate::sched::types::RemoteWakeMailboxEntry {
            tid: id,
            priority,
            enqueued_at_tick: now_tick,
            wake_mono,
        },
    );
    let already_pending = super::set_global_need_resched(mailbox_cpu);
    if super::claim_remote_wake_mailbox_ipi_epoch(mailbox_cpu) {
        super::DIAG_IPI_SENT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        super::DIAG_IPI_SENT_WAKE_TASK.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        rt.send_ipi(mailbox_cpu, 0x30);
    } else {
        // Track suppressions attributable to an already-pending resched signal.
        if already_pending {
            super::PROF_IPI_SUPPRESSED.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        }
    }
    true
}

pub fn wake_task<R: BootRuntime>(id: u64) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    if try_remote_wake_via_mailbox::<R>(id) {
        rt.irq_restore(_irq);
        return;
    }

    // Keep the scheduler hot-cache as the source of truth for wake coalescing.
    // A lockless REGISTRY-only early return can observe stale state during the
    // block_current deferred-clear window (`REGISTRY.wake_pending = true` while
    // scheduler cache already consumed/cleared), which can lose a concurrent wake.

    let wait_start = rt.mono_ticks();

    crate::ktrace!("WAKE_TASK: ID={} taking SCHEDULER lock", id);
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
        super::clear_sched_lock_tracking::<R>();

        result
    };
    // SCHEDULER lock released here.
    crate::ktrace!("WAKE_TASK: ID={} wake_task_locked returned IPI_CPU={:?}", id, ipi_cpu);

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
        if !super::should_send_remote_resched_ipi(cpu) {
            rt.irq_restore(_irq);
            return;
        }
        super::DIAG_IPI_SENT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        super::DIAG_IPI_SENT_WAKE_TASK.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        crate::ktrace!("WAKE_TASK: Sending IPI 0x30 to CPU {}", cpu);
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
    BLOCK_CURRENT_HOOK.store(block_current::<R> as *mut (), core::sync::atomic::Ordering::SeqCst);
    WAKE_TASK_HOOK.store(wake_task::<R> as *mut (), core::sync::atomic::Ordering::SeqCst);
}
