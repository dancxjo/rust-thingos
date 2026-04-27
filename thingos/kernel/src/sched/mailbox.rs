//! Per-CPU cross-CPU wakeup mailboxes.
use alloc::collections::VecDeque;
use core::sync::atomic::{AtomicU64, Ordering};

use super::metrics::*;
use super::policy::SchedPolicy;
use super::profiling::*;
use super::{state, types};
use crate::task::{TaskPriority, TaskState};

/// Per-CPU cross-CPU wakeup mailboxes.
///
/// Each slot is a [`state::WakeMailbox`] that remote CPUs push tasks into when
/// they need to wake a thread whose target CPU is not their own.  The receiving
/// CPU drains its slot at the start of `schedule_point` and inside the timer
/// tick handler.
///
/// Stored as a module-level static so that remote CPUs can push to the target
/// slot **without** holding the global `SCHEDULER` lock (the mailbox uses its
/// own internal [`spin::Mutex`] for mutual exclusion).
#[allow(clippy::declare_interior_mutable_const)]
pub(super) static REMOTE_WAKE_MAILBOXES: [state::WakeMailbox; types::MAX_CPUS] =
    [const { state::WakeMailbox::new() }; types::MAX_CPUS];

pub(super) static REMOTE_WAKE_MAILBOX_ENQUEUE_EPOCH: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};
pub(super) static REMOTE_WAKE_MAILBOX_LAST_IPI_EPOCH: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};

#[inline]
pub fn enqueue_remote_wake_mailbox(
    target_cpu: usize,
    entry: types::RemoteWakeMailboxEntry,
) -> usize {
    let safe_cpu = target_cpu.min(types::MAX_CPUS.saturating_sub(1));
    REMOTE_WAKE_MAILBOXES[safe_cpu].push(entry);
    REMOTE_WAKE_MAILBOX_ENQUEUE_EPOCH[safe_cpu].fetch_add(1, Ordering::Release);
    PROF_MAILBOX_PUSHES_PER_CPU[safe_cpu].fetch_add(1, Ordering::Relaxed);
    safe_cpu
}

#[inline]
pub(crate) fn claim_remote_wake_mailbox_ipi_epoch(cpu: usize) -> bool {
    if cpu >= types::MAX_CPUS {
        return false;
    }
    let mut last_ipi_epoch = REMOTE_WAKE_MAILBOX_LAST_IPI_EPOCH[cpu].load(Ordering::Acquire);
    loop {
        let enqueue_epoch = REMOTE_WAKE_MAILBOX_ENQUEUE_EPOCH[cpu].load(Ordering::Acquire);
        if enqueue_epoch <= last_ipi_epoch {
            DIAG_REMOTE_WAKE_MAILBOX_NO_IPI.fetch_add(1, Ordering::Relaxed);
            return false;
        }
        match REMOTE_WAKE_MAILBOX_LAST_IPI_EPOCH[cpu].compare_exchange_weak(
            last_ipi_epoch,
            enqueue_epoch,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => return true,
            Err(actual) => last_ipi_epoch = actual,
        }
    }
}

#[inline]
pub(super) fn take_remote_wake_mailbox(cpu: usize) -> VecDeque<types::RemoteWakeMailboxEntry> {
    if cpu >= types::MAX_CPUS {
        return VecDeque::new();
    }
    REMOTE_WAKE_MAILBOXES[cpu].drain()
}

#[cfg(test)]
pub(super) fn reset_remote_wake_mailboxes_for_tests() {
    for cpu in 0..types::MAX_CPUS {
        // Drain and clear any leftover entries (also resets the pending flag).
        let _ = REMOTE_WAKE_MAILBOXES[cpu].drain();
        REMOTE_WAKE_MAILBOX_ENQUEUE_EPOCH[cpu].store(0, Ordering::Relaxed);
        REMOTE_WAKE_MAILBOX_LAST_IPI_EPOCH[cpu].store(0, Ordering::Relaxed);
    }
    DIAG_REMOTE_WAKE_MAILBOX_NO_IPI.store(0, Ordering::Relaxed);
    for bucket in &PROF_REMOTE_WAKE_MAILBOX_AGE_HIST {
        bucket.store(0, Ordering::Relaxed);
    }
}

impl<R: crate::BootRuntime> types::Scheduler<R> {
    pub(super) fn drain_remote_wake_mailbox(&mut self, cpu_idx: usize) {
        // This function must only be called by the CPU that owns cpu_idx.
        // Verify this invariant in non-test debug builds.
        super::debug_assert_runq_cpu_is_local::<R>(cpu_idx);

        let pending = take_remote_wake_mailbox(cpu_idx);
        if pending.is_empty() {
            return;
        }
        let task_count = pending.len() as u64;
        let now_mono = crate::runtime::<R>().mono_ticks();

        // Snapshot the global push counter into the per-CPU stats and update
        // drain counters under the scheduler lock.
        if let Some(pc) = self.state.per_cpu.get_mut(cpu_idx) {
            let global_pushes = if cpu_idx < types::MAX_CPUS {
                PROF_MAILBOX_PUSHES_PER_CPU[cpu_idx].load(Ordering::Relaxed)
            } else {
                0
            };
            pc.stats.mailbox_pushes = global_pushes;
            pc.stats.mailbox_drains = pc.stats.mailbox_drains.saturating_add(1);
            pc.stats.mailbox_tasks_drained =
                pc.stats.mailbox_tasks_drained.saturating_add(task_count);
        }

        for wake in pending {
            let tid = wake.tid;
            let priority = wake.priority.min(TaskPriority::Realtime as usize);
            let age_us = ticks_to_us::<R>(now_mono.wrapping_sub(wake.wake_mono));
            let age_bucket = us_latency_hist_bucket(age_us);
            PROF_REMOTE_WAKE_MAILBOX_AGE_HIST[age_bucket].fetch_add(1, Ordering::Relaxed);

            self.state.unregister_waiter(tid);
            let _ = self.state.remove_task_from_sleep_queue(tid);

            if let Some(sf) = self.state.get_thread_mut(tid) {
                if sf.state == TaskState::Dead
                    || sf.state == TaskState::Running
                    || sf.runq_location.is_some()
                {
                    continue;
                }
                if sf.last_cpu.is_some_and(|c| c != cpu_idx) {
                    if sf.migration_state == state::MigrationState::Local {
                        let _ = sf
                            .migration_state
                            .try_transition(state::MigrationState::Requested { target: cpu_idx });
                    }
                    if let state::MigrationState::Requested { .. } = sf.migration_state {
                        let _ = sf.migration_state.try_transition(state::MigrationState::InTransit);
                    }
                }
                sf.state = TaskState::Runnable;
                sf.enqueued_at_tick = wake.enqueued_at_tick;
                sf.wake_cpu = Some(cpu_idx);
                sf.wake_pending = false;
            } else {
                continue;
            }

            self.state.wake_enqueued_at_mono.insert(tid, wake.wake_mono);
            self.state.note_enqueue_cause(tid, state::EnqueueCause::Wake);
            self.state.enqueue_task(cpu_idx, priority, tid);
            if let Some(pc) = self.state.per_cpu.get_mut(cpu_idx) {
                pc.stats.wakeups = pc.stats.wakeups.saturating_add(1);
            }

            // Policy: decide whether the incoming task should preempt the
            // currently running task on this CPU.
            if self.policy.should_preempt_on_wake(&self.state, cpu_idx, priority) {
                self.state.per_cpu[cpu_idx].need_resched = true;
                // Mirror to the per-CPU atomic flag so the lockless fast-path
                // in resched_if_needed / preempt_enable can see it.
                super::set_global_need_resched(cpu_idx);
            }
        }
    }
}
