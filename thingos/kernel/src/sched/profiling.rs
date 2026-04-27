//! Scheduler profiling counters and telemetry atomics.
use core::sync::atomic::AtomicU64;

use super::{state, types};

/// Global tick counter for debugging scheduler health
pub static TICK_COUNT: AtomicU64 = AtomicU64::new(0);

pub static PROF_RESCHED_TRYLOCK_MISS: AtomicU64 = AtomicU64::new(0);

// Diagnostic counters for IPI delivery chain
pub static DIAG_IPI_SENT: AtomicU64 = AtomicU64::new(0);
pub static DIAG_IPI_HANDLER: AtomicU64 = AtomicU64::new(0);
pub static DIAG_HLT_WAKE: AtomicU64 = AtomicU64::new(0);

// Per-source IPI sent counters (remote reschedule interrupt counts by callsite)
pub static DIAG_IPI_SENT_WAKE_TASK: AtomicU64 = AtomicU64::new(0);
pub static DIAG_IPI_SENT_WAKE_SLEEPERS: AtomicU64 = AtomicU64::new(0);
pub static DIAG_IPI_SENT_SPAWN: AtomicU64 = AtomicU64::new(0);
pub static DIAG_IPI_SENT_PREPARE_SCHEDULE: AtomicU64 = AtomicU64::new(0);
pub(super) static LAST_DEBUG_SUMMARY_MONO: AtomicU64 = AtomicU64::new(0);
pub static PROF_TRYLOCK_MISS_PER_CPU: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; types::MAX_CPUS]
};
pub static PROF_TRYLOCK_MISS_PENDING_PER_CPU: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; types::MAX_CPUS]
};
pub static PROF_TRYLOCK_MISS_TIMER_PER_CPU: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; types::MAX_CPUS]
};
pub static PROF_TRYLOCK_MISS_IPI_PER_CPU: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; types::MAX_CPUS]
};
pub static PROF_TRYLOCK_MISS_IDLE_TIMER_PER_CPU: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; types::MAX_CPUS]
};

/// Count of reschedule requests that were coalesced (flag was already set).
pub static PROF_RESCHED_COALESCED: AtomicU64 = AtomicU64::new(0);

/// Count of remote IPI sends that were suppressed because the per-CPU pending
/// flag was already set (i.e. a previous IPI is already in flight or pending).
pub static PROF_IPI_SUPPRESSED: AtomicU64 = AtomicU64::new(0);

/// Count of `task_status` / poll calls (task-state poll count by caller).
pub static PROF_TASK_STATUS_POLLS: AtomicU64 = AtomicU64::new(0);

/// Count of task transitions into the Runnable state (runnable transitions).
pub static PROF_RUNNABLE_TRANSITIONS: AtomicU64 = AtomicU64::new(0);
/// Count of observed scheduler lock-order violations on this CPU.
pub static PROF_LOCK_ORDER_VIOLATIONS: AtomicU64 = AtomicU64::new(0);

/// Count of wake calls that skipped the scheduler lock because the target task
/// already had `wake_pending = true`.
pub static PROF_WAKE_TASK_FASTPATH_ALREADY_PENDING: AtomicU64 = AtomicU64::new(0);

/// Per-CPU last-sampled run-queue length.
pub static PROF_RUNQ_LEN_LAST: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; types::MAX_CPUS]
};

/// Per-CPU maximum observed run-queue length.
pub static PROF_RUNQ_LEN_MAX: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; types::MAX_CPUS]
};
pub static PROF_RUNQ_SAMPLE_COUNT: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; types::MAX_CPUS]
};
pub static PROF_RUNQ_SAMPLE_TOTAL: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; types::MAX_CPUS]
};
pub static PROF_IDLE_TICKS_PER_CPU: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; types::MAX_CPUS]
};
pub static PROF_WAKE_TO_RUN_COUNT: AtomicU64 = AtomicU64::new(0);
pub static PROF_WAKE_TO_RUN_TICKS_TOTAL: AtomicU64 = AtomicU64::new(0);
pub static PROF_WAKE_TO_RUN_TICKS_MAX: AtomicU64 = AtomicU64::new(0);
pub static PROF_WAKE_TO_RUN_HIST: [AtomicU64; state::WAKE_LATENCY_HIST_BUCKETS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; state::WAKE_LATENCY_HIST_BUCKETS]
};
pub static PROF_RUNQ_DEPTH_VARIANCE_LAST: AtomicU64 = AtomicU64::new(0);
pub static PROF_RUNQ_DEPTH_VARIANCE_MAX: AtomicU64 = AtomicU64::new(0);
pub static PROF_RUNQ_DEPTH_VARIANCE_SAMPLE_COUNT: AtomicU64 = AtomicU64::new(0);
pub static PROF_RUNQ_DEPTH_VARIANCE_TOTAL: AtomicU64 = AtomicU64::new(0);
pub static PROF_IMBALANCE_TOTAL_US: AtomicU64 = AtomicU64::new(0);
pub static PROF_IMBALANCE_EPISODES: AtomicU64 = AtomicU64::new(0);
pub static PROF_IMBALANCE_LONGEST_US: AtomicU64 = AtomicU64::new(0);

/// Total tasks migrated by the periodic load balancer (slow-path balancing).
pub static PROF_PERIODIC_BALANCE_MIGRATIONS: AtomicU64 = AtomicU64::new(0);

/// Count of cross-CPU direct run-queue enqueue attempts that violated per-CPU
/// ownership rules.  These are expected to be zero after all callers correctly
/// route remote wakeups through the mailbox.  A non-zero value in production
/// indicates a regression in locking discipline.
pub static PROF_CROSS_CPU_RUNQ_DIRECT_ENQUEUE: AtomicU64 = AtomicU64::new(0);

/// Per-CPU count of entries pushed into the wake mailbox by remote CPUs.
///
/// Incremented by [`enqueue_remote_wake_mailbox`] without holding the
/// scheduler lock.  Complements `PerCpuSchedStats::mailbox_pushes` which is
/// updated from the drain path (under the scheduler lock) for precise
/// per-drain accounting.
pub static PROF_MAILBOX_PUSHES_PER_CPU: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; types::MAX_CPUS]
};
pub(super) static LAST_RESCHED_IPI_SENT_AT_TICK: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_INIT: AtomicU64 = AtomicU64::new(super::RESCHED_IPI_NEVER_SENT);
    [ATOMIC_INIT; types::MAX_CPUS]
};

pub static CPU_CURRENT_TASK: [core::sync::atomic::AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const NONE: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(u64::MAX);
    [NONE; types::MAX_CPUS]
};

#[inline]
pub(crate) fn is_task_on_any_cpu(tid: crate::task::TaskId) -> bool {
    for cpu in 0..types::MAX_CPUS {
        if CPU_CURRENT_TASK[cpu].load(core::sync::atomic::Ordering::Acquire) == tid as u64 {
            return true;
        }
    }
    false
}

#[inline]
pub(crate) fn set_cpu_current_task(cpu_idx: usize, tid: crate::task::TaskId) {
    if cpu_idx < types::MAX_CPUS {
        CPU_CURRENT_TASK[cpu_idx].store(tid as u64, core::sync::atomic::Ordering::Release);
    }
}
