//! Scheduler telemetry, metrics and histogram utilities.
use core::sync::atomic::{AtomicU64, Ordering};
use super::{state, types};
use super::profiling::*;

/// Number of histogram buckets used for hold/wait time distributions.
/// Boundaries (µs): <1, 1–10, 10–100, 100–1000, ≥1000
pub const SCHED_HIST_BUCKETS: usize = 5;

/// Map a microsecond duration to a histogram bucket index.
///
/// Bucket 0 covers 0µs, which typically means the measurement rounded down to
/// zero due to clock granularity (i.e. sub-microsecond hold/wait times).
/// Bucket indices: 0 = <1µs, 1 = 1–9µs, 2 = 10–99µs, 3 = 100–999µs, 4 = ≥1ms.
#[inline]
pub fn hist_bucket(us: u64) -> usize {
    match us {
        0 => 0,
        1..=9 => 1,
        10..=99 => 2,
        100..=999 => 3,
        _ => 4,
    }
}

#[inline]
pub fn us_latency_hist_bucket(us: u64) -> usize {
    match us {
        0..=4 => 0,
        5..=19 => 1,
        20..=99 => 2,
        100..=499 => 3,
        _ => 4,
    }
}

#[inline]
pub fn idle_episode_hist_bucket(us: u64) -> usize {
    match us {
        0..=4 => 0,
        5..=49 => 1,
        50..=499 => 2,
        _ => 3,
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SchedLockMetrics {
    pub hold_calls: u64,
    pub hold_us_total: u64,
    pub hold_us_max: u64,
    /// Hold-time histogram buckets (µs): [<1, 1–10, 10–100, 100–1000, ≥1000]
    #[cfg(feature = "sched_telemetry")]
    pub hold_hist: [u64; SCHED_HIST_BUCKETS],
    /// Number of lock-acquisition waits recorded (may differ from hold_calls if
    /// the wait is measured separately by a callsite that tracks both).
    pub wait_calls: u64,
    pub wait_us_total: u64,
    pub wait_us_max: u64,
    /// Wait-time histogram buckets (µs): [<1, 1–10, 10–100, 100–1000, ≥1000]
    #[cfg(feature = "sched_telemetry")]
    pub wait_hist: [u64; SCHED_HIST_BUCKETS],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SchedLockSiteMetrics {
    pub block_current: SchedLockMetrics,
    pub wake_task: SchedLockMetrics,
    pub yield_now: SchedLockMetrics,
    pub sleep_ticks: SchedLockMetrics,
    pub wake_sleepers: SchedLockMetrics,
    /// Reschedule requests that were suppressed because the flag was already set.
    pub resched_coalesced: u64,
    /// Remote IPI sends that were suppressed because the pending flag was already
    /// set (a previous IPI is already in flight for that CPU).
    pub ipi_suppressed: u64,
    /// Total IPI-sends broken down by originating callsite.
    pub ipi_sent_wake_task: u64,
    pub ipi_sent_wake_sleepers: u64,
    pub ipi_sent_spawn: u64,
    pub ipi_sent_prepare_schedule: u64,
    /// Per-CPU last / max run-queue depths at the most recent sample point.
    pub runq_len_last: [u64; types::MAX_CPUS],
    pub runq_len_max: [u64; types::MAX_CPUS],
    /// Per-CPU idle ticks observed at timer interrupts.
    pub idle_ticks_per_cpu: [u64; types::MAX_CPUS],
    /// Cross-CPU run-queue depth variance (population variance, in depth²).
    pub runq_depth_variance_last: u64,
    pub runq_depth_variance_max: u64,
    pub runq_depth_variance_avg: u64,
    /// Wake-to-run latency (ticks) aggregated over tasks that were woken from Blocked.
    pub wake_to_run_count: u64,
    pub wake_to_run_ticks_total: u64,
    pub wake_to_run_ticks_max: u64,
    pub wake_to_run_hist: [u64; state::WAKE_LATENCY_HIST_BUCKETS],
    pub imbalance_total_us: u64,
    pub imbalance_episodes: u64,
    pub imbalance_longest_us: u64,
    /// Number of `task_status` polls since the last snapshot.
    pub task_status_polls: u64,
    /// Number of Blocked → Runnable transitions since the last snapshot.
    pub runnable_transitions: u64,
    /// Wake calls that avoided the SCHEDULER lock because a wake was already
    /// pending for the target task.
    pub wake_task_fastpath_already_pending: u64,
    /// Observed lock-order violations (`SCHEDULER` held while taking deferred-only locks).
    pub lock_order_violations: u64,
}

// ---------------------------------------------------------------------------
// Per-callsite hold-time atomics
// ---------------------------------------------------------------------------
pub(super) static PROF_SCHED_LOCK_BLOCK_CURRENT_CALLS: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_LOCK_BLOCK_CURRENT_US_TOTAL: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_LOCK_BLOCK_CURRENT_US_MAX: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_LOCK_WAKE_TASK_CALLS: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_LOCK_WAKE_TASK_US_TOTAL: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_LOCK_WAKE_TASK_US_MAX: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_LOCK_YIELD_NOW_CALLS: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_LOCK_YIELD_NOW_US_TOTAL: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_LOCK_YIELD_NOW_US_MAX: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_LOCK_SLEEP_TICKS_CALLS: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_LOCK_SLEEP_TICKS_US_TOTAL: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_LOCK_SLEEP_TICKS_US_MAX: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_LOCK_WAKE_SLEEPERS_CALLS: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_LOCK_WAKE_SLEEPERS_US_TOTAL: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_LOCK_WAKE_SLEEPERS_US_MAX: AtomicU64 = AtomicU64::new(0);

// ---------------------------------------------------------------------------
// Per-callsite hold-time histogram atomics.
// Statics are always defined (negligible memory); bucket increments are gated
// on sched_telemetry inside record_sched_lock_hold / record_sched_lock_wait.
// ---------------------------------------------------------------------------
#[allow(clippy::declare_interior_mutable_const)]
const HIST_ZERO: AtomicU64 = AtomicU64::new(0);

pub(super) static PROF_SCHED_LOCK_BLOCK_CURRENT_HOLD_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];
pub(super) static PROF_SCHED_LOCK_WAKE_TASK_HOLD_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];
pub(super) static PROF_SCHED_LOCK_YIELD_NOW_HOLD_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];
pub(super) static PROF_SCHED_LOCK_SLEEP_TICKS_HOLD_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];
pub(super) static PROF_SCHED_LOCK_WAKE_SLEEPERS_HOLD_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];

// ---------------------------------------------------------------------------
// Per-callsite acquisition wait-time atomics
// ---------------------------------------------------------------------------
pub(super) static PROF_SCHED_WAIT_BLOCK_CURRENT_CALLS: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_WAIT_BLOCK_CURRENT_US_TOTAL: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_WAIT_BLOCK_CURRENT_US_MAX: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_WAIT_WAKE_TASK_CALLS: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_WAIT_WAKE_TASK_US_TOTAL: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_WAIT_WAKE_TASK_US_MAX: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_WAIT_YIELD_NOW_CALLS: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_WAIT_YIELD_NOW_US_TOTAL: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_WAIT_YIELD_NOW_US_MAX: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_WAIT_SLEEP_TICKS_CALLS: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_WAIT_SLEEP_TICKS_US_TOTAL: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_WAIT_SLEEP_TICKS_US_MAX: AtomicU64 = AtomicU64::new(0);

// ---------------------------------------------------------------------------
// Per-callsite wait-time histogram atomics (same always-defined strategy)
// ---------------------------------------------------------------------------
pub(super) static PROF_SCHED_WAIT_BLOCK_CURRENT_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];
pub(super) static PROF_SCHED_WAIT_WAKE_TASK_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];
pub(super) static PROF_SCHED_WAIT_YIELD_NOW_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];
pub(super) static PROF_SCHED_WAIT_SLEEP_TICKS_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];

// Dedicated zero-valued wait statics for wake_sleepers (called while lock is
// already held, so there is no acquisition wait to track).
pub(super) static PROF_SCHED_WAIT_WAKE_SLEEPERS_CALLS: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_WAIT_WAKE_SLEEPERS_US_TOTAL: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_WAIT_WAKE_SLEEPERS_US_MAX: AtomicU64 = AtomicU64::new(0);
pub(super) static PROF_SCHED_WAIT_WAKE_SLEEPERS_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];

pub static DIAG_REMOTE_WAKE_MAILBOX_NO_IPI: AtomicU64 = AtomicU64::new(0);
pub const REMOTE_WAKE_MAILBOX_AGE_HIST_BUCKETS: usize = 5;
pub static PROF_REMOTE_WAKE_MAILBOX_AGE_HIST: [AtomicU64; REMOTE_WAKE_MAILBOX_AGE_HIST_BUCKETS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; REMOTE_WAKE_MAILBOX_AGE_HIST_BUCKETS]
};

#[inline]
pub fn ticks_to_us<R: crate::BootRuntime>(ticks: u64) -> u64 {
    let rt = crate::runtime::<R>();
    let freq = rt.mono_freq_hz().max(1);
    ticks.saturating_mul(1_000_000) / freq
}

#[inline]
pub fn update_max_u64(slot: &AtomicU64, val: u64) {
    let mut prev = slot.load(Ordering::Relaxed);
    while val > prev {
        match slot.compare_exchange_weak(prev, val, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(actual) => prev = actual,
        }
    }
}

#[inline]
pub fn snapshot_sched_lock_metric(
    calls: &AtomicU64,
    total: &AtomicU64,
    max: &AtomicU64,
    _hold_hist: &[AtomicU64; SCHED_HIST_BUCKETS],
    wait_calls: &AtomicU64,
    wait_total: &AtomicU64,
    wait_max: &AtomicU64,
    _wait_hist: &[AtomicU64; SCHED_HIST_BUCKETS],
) -> SchedLockMetrics {
    #[cfg(feature = "sched_telemetry")]
    let hold_hist_snapshot = {
        let mut b = [0u64; SCHED_HIST_BUCKETS];
        for (i, a) in _hold_hist.iter().enumerate() {
            b[i] = a.swap(0, Ordering::Relaxed);
        }
        b
    };
    #[cfg(feature = "sched_telemetry")]
    let wait_hist_snapshot = {
        let mut b = [0u64; SCHED_HIST_BUCKETS];
        for (i, a) in _wait_hist.iter().enumerate() {
            b[i] = a.swap(0, Ordering::Relaxed);
        }
        b
    };
    SchedLockMetrics {
        hold_calls: calls.swap(0, Ordering::Relaxed),
        hold_us_total: total.swap(0, Ordering::Relaxed),
        hold_us_max: max.swap(0, Ordering::Relaxed),
        #[cfg(feature = "sched_telemetry")]
        hold_hist: hold_hist_snapshot,
        wait_calls: wait_calls.swap(0, Ordering::Relaxed),
        wait_us_total: wait_total.swap(0, Ordering::Relaxed),
        wait_us_max: wait_max.swap(0, Ordering::Relaxed),
        #[cfg(feature = "sched_telemetry")]
        wait_hist: wait_hist_snapshot,
    }
}

pub(crate) fn record_sched_lock_hold<R: crate::BootRuntime>(
    calls: &AtomicU64,
    total: &AtomicU64,
    max: &AtomicU64,
    hold_hist: &[AtomicU64; SCHED_HIST_BUCKETS],
    start_ticks: u64,
) {
    let elapsed_us = ticks_to_us::<R>(crate::runtime::<R>().mono_ticks().wrapping_sub(start_ticks));
    calls.fetch_add(1, Ordering::Relaxed);
    total.fetch_add(elapsed_us, Ordering::Relaxed);
    update_max_u64(max, elapsed_us);
    // Histogram bucket increment is gated on sched_telemetry to bound overhead.
    #[cfg(feature = "sched_telemetry")]
    hold_hist[hist_bucket(elapsed_us)].fetch_add(1, Ordering::Relaxed);
    // Suppress unused-variable warning when the feature is disabled.
    #[cfg(not(feature = "sched_telemetry"))]
    let _ = hold_hist;
}

pub(crate) fn record_sched_lock_wait<R: crate::BootRuntime>(
    calls: &AtomicU64,
    total: &AtomicU64,
    max: &AtomicU64,
    wait_hist: &[AtomicU64; SCHED_HIST_BUCKETS],
    wait_start_ticks: u64,
) {
    let elapsed_us =
        ticks_to_us::<R>(crate::runtime::<R>().mono_ticks().wrapping_sub(wait_start_ticks));
    calls.fetch_add(1, Ordering::Relaxed);
    total.fetch_add(elapsed_us, Ordering::Relaxed);
    update_max_u64(max, elapsed_us);
    #[cfg(feature = "sched_telemetry")]
    wait_hist[hist_bucket(elapsed_us)].fetch_add(1, Ordering::Relaxed);
    #[cfg(not(feature = "sched_telemetry"))]
    let _ = wait_hist;
}

pub fn sched_lock_metrics_snapshot_and_reset() -> SchedLockSiteMetrics {
    // Collect per-CPU run-queue snapshots (non-destructive read for last; swap max)
    let mut runq_len_last = [0u64; types::MAX_CPUS];
    let mut runq_len_max = [0u64; types::MAX_CPUS];
    let mut idle_ticks_per_cpu = [0u64; types::MAX_CPUS];
    for i in 0..types::MAX_CPUS {
        runq_len_last[i] = PROF_RUNQ_LEN_LAST[i].load(Ordering::Relaxed);
        runq_len_max[i] = PROF_RUNQ_LEN_MAX[i].swap(0, Ordering::Relaxed);
        idle_ticks_per_cpu[i] = PROF_IDLE_TICKS_PER_CPU[i].swap(0, Ordering::Relaxed);
    }
    let runq_depth_variance_sample_count =
        PROF_RUNQ_DEPTH_VARIANCE_SAMPLE_COUNT.swap(0, Ordering::Relaxed);
    let runq_depth_variance_total = PROF_RUNQ_DEPTH_VARIANCE_TOTAL.swap(0, Ordering::Relaxed);
    let runq_depth_variance_avg = if runq_depth_variance_sample_count == 0 {
        0
    } else {
        runq_depth_variance_total / runq_depth_variance_sample_count
    };
    let mut wake_to_run_hist = [0u64; state::WAKE_LATENCY_HIST_BUCKETS];
    for (idx, slot) in wake_to_run_hist.iter_mut().enumerate() {
        *slot = PROF_WAKE_TO_RUN_HIST[idx].swap(0, Ordering::Relaxed);
    }
    SchedLockSiteMetrics {
        block_current: snapshot_sched_lock_metric(
            &PROF_SCHED_LOCK_BLOCK_CURRENT_CALLS,
            &PROF_SCHED_LOCK_BLOCK_CURRENT_US_TOTAL,
            &PROF_SCHED_LOCK_BLOCK_CURRENT_US_MAX,
            &PROF_SCHED_LOCK_BLOCK_CURRENT_HOLD_HIST,
            &PROF_SCHED_WAIT_BLOCK_CURRENT_CALLS,
            &PROF_SCHED_WAIT_BLOCK_CURRENT_US_TOTAL,
            &PROF_SCHED_WAIT_BLOCK_CURRENT_US_MAX,
            &PROF_SCHED_WAIT_BLOCK_CURRENT_HIST,
        ),
        wake_task: snapshot_sched_lock_metric(
            &PROF_SCHED_LOCK_WAKE_TASK_CALLS,
            &PROF_SCHED_LOCK_WAKE_TASK_US_TOTAL,
            &PROF_SCHED_LOCK_WAKE_TASK_US_MAX,
            &PROF_SCHED_LOCK_WAKE_TASK_HOLD_HIST,
            &PROF_SCHED_WAIT_WAKE_TASK_CALLS,
            &PROF_SCHED_WAIT_WAKE_TASK_US_TOTAL,
            &PROF_SCHED_WAIT_WAKE_TASK_US_MAX,
            &PROF_SCHED_WAIT_WAKE_TASK_HIST,
        ),
        yield_now: snapshot_sched_lock_metric(
            &PROF_SCHED_LOCK_YIELD_NOW_CALLS,
            &PROF_SCHED_LOCK_YIELD_NOW_US_TOTAL,
            &PROF_SCHED_LOCK_YIELD_NOW_US_MAX,
            &PROF_SCHED_LOCK_YIELD_NOW_HOLD_HIST,
            &PROF_SCHED_WAIT_YIELD_NOW_CALLS,
            &PROF_SCHED_WAIT_YIELD_NOW_US_TOTAL,
            &PROF_SCHED_WAIT_YIELD_NOW_US_MAX,
            &PROF_SCHED_WAIT_YIELD_NOW_HIST,
        ),
        sleep_ticks: snapshot_sched_lock_metric(
            &PROF_SCHED_LOCK_SLEEP_TICKS_CALLS,
            &PROF_SCHED_LOCK_SLEEP_TICKS_US_TOTAL,
            &PROF_SCHED_LOCK_SLEEP_TICKS_US_MAX,
            &PROF_SCHED_LOCK_SLEEP_TICKS_HOLD_HIST,
            &PROF_SCHED_WAIT_SLEEP_TICKS_CALLS,
            &PROF_SCHED_WAIT_SLEEP_TICKS_US_TOTAL,
            &PROF_SCHED_WAIT_SLEEP_TICKS_US_MAX,
            &PROF_SCHED_WAIT_SLEEP_TICKS_HIST,
        ),
        wake_sleepers: snapshot_sched_lock_metric(
            &PROF_SCHED_LOCK_WAKE_SLEEPERS_CALLS,
            &PROF_SCHED_LOCK_WAKE_SLEEPERS_US_TOTAL,
            &PROF_SCHED_LOCK_WAKE_SLEEPERS_US_MAX,
            &PROF_SCHED_LOCK_WAKE_SLEEPERS_HOLD_HIST,
            // wake_sleepers is called while the lock is already held;
            // no separate wait-time tracking needed for it.
            &PROF_SCHED_WAIT_WAKE_SLEEPERS_CALLS,
            &PROF_SCHED_WAIT_WAKE_SLEEPERS_US_TOTAL,
            &PROF_SCHED_WAIT_WAKE_SLEEPERS_US_MAX,
            &PROF_SCHED_WAIT_WAKE_SLEEPERS_HIST,
        ),
        resched_coalesced: PROF_RESCHED_COALESCED.swap(0, Ordering::Relaxed),
        ipi_suppressed: PROF_IPI_SUPPRESSED.swap(0, Ordering::Relaxed),
        ipi_sent_wake_task: DIAG_IPI_SENT_WAKE_TASK.swap(0, Ordering::Relaxed),
        ipi_sent_wake_sleepers: DIAG_IPI_SENT_WAKE_SLEEPERS.swap(0, Ordering::Relaxed),
        ipi_sent_spawn: DIAG_IPI_SENT_SPAWN.swap(0, Ordering::Relaxed),
        ipi_sent_prepare_schedule: DIAG_IPI_SENT_PREPARE_SCHEDULE.swap(0, Ordering::Relaxed),
        runq_len_last,
        runq_len_max,
        idle_ticks_per_cpu,
        runq_depth_variance_last: PROF_RUNQ_DEPTH_VARIANCE_LAST.load(Ordering::Relaxed),
        runq_depth_variance_max: PROF_RUNQ_DEPTH_VARIANCE_MAX.swap(0, Ordering::Relaxed),
        runq_depth_variance_avg,
        wake_to_run_count: PROF_WAKE_TO_RUN_COUNT.swap(0, Ordering::Relaxed),
        wake_to_run_ticks_total: PROF_WAKE_TO_RUN_TICKS_TOTAL.swap(0, Ordering::Relaxed),
        wake_to_run_ticks_max: PROF_WAKE_TO_RUN_TICKS_MAX.swap(0, Ordering::Relaxed),
        wake_to_run_hist,
        imbalance_total_us: PROF_IMBALANCE_TOTAL_US.swap(0, Ordering::Relaxed),
        imbalance_episodes: PROF_IMBALANCE_EPISODES.swap(0, Ordering::Relaxed),
        imbalance_longest_us: PROF_IMBALANCE_LONGEST_US.swap(0, Ordering::Relaxed),
        task_status_polls: PROF_TASK_STATUS_POLLS.swap(0, Ordering::Relaxed),
        runnable_transitions: PROF_RUNNABLE_TRANSITIONS.swap(0, Ordering::Relaxed),
        wake_task_fastpath_already_pending: PROF_WAKE_TASK_FASTPATH_ALREADY_PENDING
            .swap(0, Ordering::Relaxed),
        lock_order_violations: PROF_LOCK_ORDER_VIOLATIONS.swap(0, Ordering::Relaxed),
    }
}
