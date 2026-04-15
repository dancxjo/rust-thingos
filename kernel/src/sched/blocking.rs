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

pub fn block_current<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let mut blocked_id = None;
    let switch_params = {
        let wait_start = rt.mono_ticks();
        let lock = SCHEDULER.lock();
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
                rt.irq_restore(_irq);
                return;
            }
        };

        // Move current from Running to Blocked in the canonical REGISTRY.
        if let Some(mut task) = crate::task::registry::get_task_mut::<R>(current_id) {
            if task.wake_pending {
                task.wake_pending = false;
                rt.irq_restore(_irq);
                return;
            }
            task.state = TaskState::Blocked;
            blocked_id = Some(current_id);
        }

        // Keep the scheduler-side hot-field cache in sync.
        if let Some(sf) = sched.state.get_task_mut(current_id) {
            sf.state = TaskState::Blocked;
        }

        // Add to wait queue
        sched.state.wait_queue.push_back(current_id);

        // Schedule next
        let switch = sched.prepare_schedule();
        super::record_sched_lock_hold::<R>(
            &super::PROF_SCHED_LOCK_BLOCK_CURRENT_CALLS,
            &super::PROF_SCHED_LOCK_BLOCK_CURRENT_US_TOTAL,
            &super::PROF_SCHED_LOCK_BLOCK_CURRENT_US_MAX,
            &super::PROF_SCHED_LOCK_BLOCK_CURRENT_HOLD_HIST,
            lock_start,
        );
        switch
    };

    if let Some(_id) = blocked_id {
        // when push_task_state wakes the drain task.
    }

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

/// Wake a blocked task.  Returns the CPU to which a reschedule IPI should be
/// sent, or `None` if no cross-CPU IPI is needed (either the task is on the
/// current CPU, or a previous IPI for that CPU is already in flight).
///
/// **The caller must drop the `SCHEDULER` lock before sending any IPI** to
/// avoid holding the lock during IPI delivery (reduces lock-convoy churn on
/// SMP during boot and service launch — issue #130).
pub fn wake_task_locked<R: BootRuntime>(sched: &mut Scheduler<R>, id: u64) -> Option<usize> {
    let mut wake_info: Option<(usize, usize)> = None;

    // 1. Read scheduling fields from the hot-field cache — no REGISTRY lock
    //    needed for the read path.  Write the new state to REGISTRY afterwards.
    if let Some(sf) = sched.state.get_thread(id) {
        if sf.state == TaskState::Blocked {
            let task_priority = sf.priority as usize;
            let target_cpu = match sf.affinity {
                crate::task::Affinity::Pinned(cpu) => cpu,
                crate::task::Affinity::Any => sf
                    .last_cpu
                    .unwrap_or_else(|| super::current_cpu_index::<R>()),
            };
            wake_info = Some((target_cpu, task_priority));
        }
    }

    if wake_info.is_some() {
        // Update the canonical REGISTRY state and profiling counter.
        if let Some(mut task) = crate::task::registry::get_task_mut::<R>(id) {
            task.state = TaskState::Runnable;
            task.enqueued_at_tick = super::TICK_COUNT.load(core::sync::atomic::Ordering::Relaxed);
        }
        super::PROF_RUNNABLE_TRANSITIONS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        // Keep the scheduler-side cache in sync.
        if let Some(sf) = sched.state.get_thread_mut(id) {
            sf.state = TaskState::Runnable;
        }
    } else {
        // Task is not blocked — set wake_pending so the next block_current
        // returns immediately without actually blocking.
        if let Some(mut task) = crate::task::registry::get_task_mut::<R>(id) {
            task.wake_pending = true;
        }
    }

    // 2. Update scheduler queues after dropping the registry lock taken above.
    if let Some((target_cpu, task_priority)) = wake_info {
        let mut safe_cpu = target_cpu;
        if let Some(pos) = sched.state.wait_queue.iter().position(|&wid| wid == id) {
            sched.state.wait_queue.remove(pos);
        }
        sched.state.sleep_queue.retain(|_, tids| {
            tids.retain(|&sleep_tid| sleep_tid != id);
            !tids.is_empty()
        });

        if safe_cpu >= sched.state.per_cpu.len() {
            safe_cpu = 0;
        }

        sched.state.enqueue_task(safe_cpu, task_priority, id);

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
                // Use the coalescing helper: only return an IPI request if the
                // pending flag was not already set.  A set flag means a
                // previous IPI is in flight; that CPU will pick up this task.
                let already_pending = super::set_global_need_resched(safe_cpu);
                if !already_pending {
                    // Return the IPI target CPU so the caller can send it after
                    // dropping the SCHEDULER lock (issue #130).
                    return Some(safe_cpu);
                } else {
                    super::PROF_IPI_SUPPRESSED
                        .fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                }
            }
        }
    }

    None
}

pub fn wake_task<R: BootRuntime>(id: u64) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let wait_start = rt.mono_ticks();
    // Collect any pending IPI target inside the lock, then send it after
    // the lock is dropped to prevent holding SCHEDULER during IPI delivery
    // (issue #130).
    let ipi_cpu = {
        let lock_sched = SCHEDULER.lock();
        super::record_sched_lock_wait::<R>(
            &super::PROF_SCHED_WAIT_WAKE_TASK_CALLS,
            &super::PROF_SCHED_WAIT_WAKE_TASK_US_TOTAL,
            &super::PROF_SCHED_WAIT_WAKE_TASK_US_MAX,
            &super::PROF_SCHED_WAIT_WAKE_TASK_HIST,
            wait_start,
        );
        let lock_start = rt.mono_ticks();
        let ipi = if let Some(ptr) = *lock_sched {
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
            wake_task_locked::<R>(sched, id)
        } else {
            None
        };

        super::record_sched_lock_hold::<R>(
            &super::PROF_SCHED_LOCK_WAKE_TASK_CALLS,
            &super::PROF_SCHED_LOCK_WAKE_TASK_US_TOTAL,
            &super::PROF_SCHED_LOCK_WAKE_TASK_US_MAX,
            &super::PROF_SCHED_LOCK_WAKE_TASK_HOLD_HIST,
            lock_start,
        );
        ipi
        // SCHEDULER lock dropped here.
    };

    // Send the reschedule IPI *after* the SCHEDULER lock is released so the
    // target CPU's IPI handler can acquire the lock immediately without
    // spinning (issue #130).
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
