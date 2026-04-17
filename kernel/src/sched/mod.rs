//! Preemptive priority-based scheduler
//!
//! This module is split into focused submodules:
//! - `types`: Core data structures and enums
//! - `blocking`: Task blocking and wake primitives
//! - `hooks`: Type-erased hook system for callers without generic params
//! - `spawn`: Task and thread spawning
//! - `stack`: User stack allocation and fault handling
//! - `sleep`: Timing and yield functions
//! - `events`: Lock-free scheduler event types

pub(crate) mod blocking;
pub mod bridge;
pub mod hooks;
pub use hooks::protect_user_range_current;
mod sleep;
mod spawn;
mod stack;
pub mod state;
pub(crate) mod types;
mod vm;
pub(crate) mod wait_queue;

// Re-export all public items
use core::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, AtomicUsize, Ordering};

pub use blocking::{
    block_current, block_current_erased, init_blocking_hooks, wake_task, wake_task_erased,
};
pub use hooks::{
    ProcessSnapshot, add_user_mapping_current, alloc_user_stack_current,
    available_parallelism_current, check_user_mapping_current, current_priority_current,
    current_task_name_current, current_task_resource_id, current_tid_current,
    current_user_fs_base_current, dump_stats_current,
    exit_current, get_signal_mask_current, get_thread_pending_current, get_user_mapping_at_current,
    handle_user_stack_fault_current, interrupt_task_current, kill_by_tid_current,
    list_processes_current, poll_task_exit_current, process_info_current,
    process_info_for_pid_current, process_info_for_tid_current, register_task_exit_waiter_current,
    register_timeout_wake_current, remove_user_mappings_current, set_current_task_name_current,
    set_current_user_fs_base_current, set_priority_current, set_signal_mask_current,
    set_thread_pending_current, sleep_ticks_current, spawn_process_current,
    spawn_process_ex_current, spawn_process_from_path_current, spawn_user_thread_current,
    take_pending_interrupt_current, task_exec_current, task_status_current, task_wait_current,
    unregister_task_exit_waiter_current, unregister_timeout_wake_current, waitpid_current,
    yield_now_current,
};
pub use sleep::{sleep_ms, sleep_ticks, sleep_until, yield_now};
pub use spawn::{
    SpawnExResult, StdioSpec, boot_spawn_process, spawn, spawn_user_task_full, spawn_user_thread,
    spawn_user_thread_ex, spawn_with_priority, user_thread_trampoline,
};
use spin::Mutex;
pub use stack::{alloc_user_stack, handle_stack_fault, map_user_page, map_user_page_perms};
pub use types::{DEFAULT_TIMESLICE, ScheduleReason, Scheduler, StackFaultResult, SwitchParams};
pub use wait_queue::WaitQueue;

use crate::task::{Affinity, StartupArg, Task, TaskId, TaskPriority, TaskState};
use crate::{BootRuntime, BootTasking};

#[cfg(any(feature = "sched_debug", debug_assertions))]
static SWITCH_LOG_COUNT: AtomicUsize = AtomicUsize::new(0);

pub static SCHEDULER: Mutex<Option<usize>> = Mutex::new(None);

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
static LAST_DEBUG_SUMMARY_MONO: AtomicU64 = AtomicU64::new(0);
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

/// Count of reschedule requests that were coalesced (flag was already set).
pub static PROF_RESCHED_COALESCED: AtomicU64 = AtomicU64::new(0);

/// Count of remote IPI sends that were suppressed because the per-CPU pending
/// flag was already set (i.e. a previous IPI is already in flight or pending).
pub static PROF_IPI_SUPPRESSED: AtomicU64 = AtomicU64::new(0);

/// Count of `task_status` / poll calls (task-state poll count by caller).
pub static PROF_TASK_STATUS_POLLS: AtomicU64 = AtomicU64::new(0);

/// Count of task transitions into the Runnable state (runnable transitions).
pub static PROF_RUNNABLE_TRANSITIONS: AtomicU64 = AtomicU64::new(0);

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

/// Number of histogram buckets used for hold/wait time distributions.
/// Boundaries (µs): <1, 1–10, 10–100, 100–1000, ≥1000
pub const SCHED_HIST_BUCKETS: usize = 5;
const PREPARE_SCHEDULE_PICK_BUDGET: usize = 16;
const PREPARE_SCHEDULE_MISROUTE_REPAIR_BUDGET: usize = 8;
const PREPARE_SCHEDULE_MISROUTE_BACKLOG_CAP: usize = 128;

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
    /// Number of `task_status` polls since the last snapshot.
    pub task_status_polls: u64,
    /// Number of Blocked → Runnable transitions since the last snapshot.
    pub runnable_transitions: u64,
    /// Wake calls that avoided the SCHEDULER lock because a wake was already
    /// pending for the target task.
    pub wake_task_fastpath_already_pending: u64,
}

// ---------------------------------------------------------------------------
// Per-callsite hold-time atomics
// ---------------------------------------------------------------------------
static PROF_SCHED_LOCK_BLOCK_CURRENT_CALLS: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_BLOCK_CURRENT_US_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_BLOCK_CURRENT_US_MAX: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_WAKE_TASK_CALLS: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_WAKE_TASK_US_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_WAKE_TASK_US_MAX: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_YIELD_NOW_CALLS: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_YIELD_NOW_US_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_YIELD_NOW_US_MAX: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_SLEEP_TICKS_CALLS: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_SLEEP_TICKS_US_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_SLEEP_TICKS_US_MAX: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_WAKE_SLEEPERS_CALLS: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_WAKE_SLEEPERS_US_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_LOCK_WAKE_SLEEPERS_US_MAX: AtomicU64 = AtomicU64::new(0);

// ---------------------------------------------------------------------------
// Per-callsite hold-time histogram atomics.
// Statics are always defined (negligible memory); bucket increments are gated
// on sched_telemetry inside record_sched_lock_hold / record_sched_lock_wait.
// ---------------------------------------------------------------------------
#[allow(clippy::declare_interior_mutable_const)]
const HIST_ZERO: AtomicU64 = AtomicU64::new(0);

static PROF_SCHED_LOCK_BLOCK_CURRENT_HOLD_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];
static PROF_SCHED_LOCK_WAKE_TASK_HOLD_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];
static PROF_SCHED_LOCK_YIELD_NOW_HOLD_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];
static PROF_SCHED_LOCK_SLEEP_TICKS_HOLD_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];
static PROF_SCHED_LOCK_WAKE_SLEEPERS_HOLD_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];

// ---------------------------------------------------------------------------
// Per-callsite acquisition wait-time atomics
// ---------------------------------------------------------------------------
static PROF_SCHED_WAIT_BLOCK_CURRENT_CALLS: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_WAIT_BLOCK_CURRENT_US_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_WAIT_BLOCK_CURRENT_US_MAX: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_WAIT_WAKE_TASK_CALLS: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_WAIT_WAKE_TASK_US_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_WAIT_WAKE_TASK_US_MAX: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_WAIT_YIELD_NOW_CALLS: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_WAIT_YIELD_NOW_US_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_WAIT_YIELD_NOW_US_MAX: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_WAIT_SLEEP_TICKS_CALLS: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_WAIT_SLEEP_TICKS_US_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_WAIT_SLEEP_TICKS_US_MAX: AtomicU64 = AtomicU64::new(0);

// ---------------------------------------------------------------------------
// Per-callsite wait-time histogram atomics (same always-defined strategy)
// ---------------------------------------------------------------------------
static PROF_SCHED_WAIT_BLOCK_CURRENT_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];
static PROF_SCHED_WAIT_WAKE_TASK_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];
static PROF_SCHED_WAIT_YIELD_NOW_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];
static PROF_SCHED_WAIT_SLEEP_TICKS_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];

// Dedicated zero-valued wait statics for wake_sleepers (called while lock is
// already held, so there is no acquisition wait to track).
static PROF_SCHED_WAIT_WAKE_SLEEPERS_CALLS: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_WAIT_WAKE_SLEEPERS_US_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_WAIT_WAKE_SLEEPERS_US_MAX: AtomicU64 = AtomicU64::new(0);
static PROF_SCHED_WAIT_WAKE_SLEEPERS_HIST: [AtomicU64; SCHED_HIST_BUCKETS] =
    [HIST_ZERO; SCHED_HIST_BUCKETS];

/// Lock-skip self-healing: when try_resched_if_needed() fails to acquire
/// the scheduler lock, set this flag so the next safe-point yields.
///
/// This is per-CPU to prevent one CPU from accidentally consuming another's
/// reschedule request.
static GLOBAL_NEED_RESCHED: [AtomicBool; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
    [ATOMIC_FALSE; types::MAX_CPUS]
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum AnyWakeOverloadPolicy {
    Off = 0,
    Redirect = 1,
    /// Preserve locality preference only when not overloaded; otherwise treat
    /// the wakeup as stealable by the least-loaded online CPU.
    ///
    /// Current implementation routes to the same target as `Redirect`; the
    /// distinct variant keeps a policy surface for follow-up steal mechanics.
    Steal = 2,
}

static ANY_WAKE_POLICY_INIT_DONE: AtomicBool = AtomicBool::new(false);
static ANY_WAKE_OVERLOAD_POLICY: AtomicU8 = AtomicU8::new(AnyWakeOverloadPolicy::Off as u8);
static ANY_WAKE_OVERLOAD_GAP: AtomicUsize = AtomicUsize::new(4);
// The streak counter storage is AtomicU8, but clamp APIs operate on usize.
const ANY_WAKE_OVERLOAD_STREAK_MAX: usize = u8::MAX as usize;
static ANY_WAKE_OVERLOAD_STREAK_REQUIRED: AtomicUsize = AtomicUsize::new(3);
static ANY_WAKE_OVERLOAD_STREAK: [AtomicU8; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU8 = AtomicU8::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};

fn any_wake_overload_policy_from_u8(v: u8) -> AnyWakeOverloadPolicy {
    match v {
        0 => AnyWakeOverloadPolicy::Off,
        1 => AnyWakeOverloadPolicy::Redirect,
        2 => AnyWakeOverloadPolicy::Steal,
        _ => AnyWakeOverloadPolicy::Off,
    }
}

fn parse_any_wake_overload_policy(value: &str) -> AnyWakeOverloadPolicy {
    match value {
        "redirect" => AnyWakeOverloadPolicy::Redirect,
        "steal" => AnyWakeOverloadPolicy::Steal,
        _ => AnyWakeOverloadPolicy::Off,
    }
}

fn init_any_wake_policy_from_env_once() {
    if ANY_WAKE_POLICY_INIT_DONE
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_ok()
    {
        // Build-time tunables via `option_env!` (captured at compile time).
        if let Some(v) = option_env!("THINGOS_SCHED_ANY_WAKE_POLICY") {
            ANY_WAKE_OVERLOAD_POLICY
                .store(parse_any_wake_overload_policy(v) as u8, Ordering::Release);
        }
        if let Some(v) = option_env!("THINGOS_SCHED_ANY_WAKE_OVERLOAD_GAP") {
            if let Ok(gap) = v.parse::<usize>() {
                ANY_WAKE_OVERLOAD_GAP.store(gap.max(1), Ordering::Release);
            }
        }
        if let Some(v) = option_env!("THINGOS_SCHED_ANY_WAKE_OVERLOAD_STREAK") {
            if let Ok(streak) = v.parse::<usize>() {
                ANY_WAKE_OVERLOAD_STREAK_REQUIRED
                    .store(streak.clamp(1, ANY_WAKE_OVERLOAD_STREAK_MAX), Ordering::Release);
            }
        }
    }
}

fn runq_depth_for_cpu(state: &crate::sched::state::SchedState, cpu: usize) -> usize {
    state.per_cpu.get(cpu).map(|pc| pc.runq.iter().map(|q| q.len()).sum::<usize>()).unwrap_or(0)
}

fn least_loaded_online_cpu(state: &crate::sched::state::SchedState) -> Option<(usize, usize)> {
    state
        .online_cpus
        .iter()
        .copied()
        .filter(|&cpu| cpu < state.per_cpu.len())
        .map(|cpu| (cpu, runq_depth_for_cpu(state, cpu)))
        .min_by_key(|&(_, depth)| depth)
}

/// Per-CPU start tick for the current try-lock miss warning window.
static TRYLOCK_MISS_WINDOW_START: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};

/// Per-CPU count of try-lock misses within the current warning window.
static TRYLOCK_MISS_WINDOW_COUNT: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};

/// Per-CPU tick timestamp when the try-lock miss threshold warning was last emitted.
static TRYLOCK_MISS_LAST_WARN_TICK: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};

/// If trylock misses exceed this count in a 2-second window, emit a warning.
/// Under SMP the scheduler lock can be briefly held by another CPU during its
/// own scheduling cycle; 50 fired too readily at startup with 6 vCPUs. 100
/// gives better signal-to-noise without hiding genuine long-hold-time issues.
pub const TRYLOCK_MISS_WARN_THRESHOLD: u64 = 100;

/// Minimum interval between threshold warning emissions per CPU.
pub const TRYLOCK_MISS_WARN_COOLDOWN_SECS: u64 = 30;

#[inline]
fn ticks_to_us<R: BootRuntime>(ticks: u64) -> u64 {
    let rt = crate::runtime::<R>();
    let freq = rt.mono_freq_hz().max(1);
    ticks.saturating_mul(1_000_000) / freq
}

#[inline]
fn update_max_u64(slot: &AtomicU64, val: u64) {
    let mut prev = slot.load(Ordering::Relaxed);
    while val > prev {
        match slot.compare_exchange_weak(prev, val, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(actual) => prev = actual,
        }
    }
}

#[inline]
fn snapshot_sched_lock_metric(
    calls: &AtomicU64,
    total: &AtomicU64,
    max: &AtomicU64,
    hold_hist: &[AtomicU64; SCHED_HIST_BUCKETS],
    wait_calls: &AtomicU64,
    wait_total: &AtomicU64,
    wait_max: &AtomicU64,
    wait_hist: &[AtomicU64; SCHED_HIST_BUCKETS],
) -> SchedLockMetrics {
    #[cfg(feature = "sched_telemetry")]
    let hold_hist_snapshot = {
        let mut b = [0u64; SCHED_HIST_BUCKETS];
        for (i, a) in hold_hist.iter().enumerate() {
            b[i] = a.swap(0, Ordering::Relaxed);
        }
        b
    };
    #[cfg(feature = "sched_telemetry")]
    let wait_hist_snapshot = {
        let mut b = [0u64; SCHED_HIST_BUCKETS];
        for (i, a) in wait_hist.iter().enumerate() {
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

pub(crate) fn record_sched_lock_hold<R: BootRuntime>(
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

/// Record a lock-acquisition wait (time from before calling `.lock()` to after
/// the lock is held).  Separate from `record_sched_lock_hold` so callers that
/// only enter through a single trylock path can skip this.
pub(crate) fn record_sched_lock_wait<R: BootRuntime>(
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

/// Increment the coalesced-reschedule counter if `GLOBAL_NEED_RESCHED[cpu]`
/// was already set, then unconditionally set it.  Returns `true` if the flag
/// was already set (i.e. the request was coalesced).
#[inline]
pub(crate) fn set_global_need_resched(cpu: usize) -> bool {
    let was_set = GLOBAL_NEED_RESCHED[cpu].swap(true, Ordering::Release);
    if was_set {
        PROF_RESCHED_COALESCED.fetch_add(1, Ordering::Relaxed);
    }
    was_set
}

/// Sample the run-queue depth for `cpu` and update the last/max statics.
/// This is a no-op when the `sched_telemetry` feature is disabled so that
/// the per-schedule-point iteration incurs zero overhead in normal builds.
#[inline]
pub(crate) fn sample_runq_len(sched: &mut types::Scheduler<impl BootRuntime>, cpu: usize) {
    if let Some(pc) = sched.state.per_cpu.get_mut(cpu) {
        let len64 = pc.runq.iter().map(|q| q.len()).sum::<usize>() as u64;
        PROF_RUNQ_SAMPLE_COUNT[cpu].fetch_add(1, Ordering::Relaxed);
        PROF_RUNQ_SAMPLE_TOTAL[cpu].fetch_add(len64, Ordering::Relaxed);
        pc.stats.runq_sample_count = pc.stats.runq_sample_count.saturating_add(1);
        pc.stats.runq_sample_total = pc.stats.runq_sample_total.saturating_add(len64);
        #[cfg(feature = "sched_telemetry")]
        {
            PROF_RUNQ_LEN_LAST[cpu].store(len64, Ordering::Relaxed);
            update_max_u64(&PROF_RUNQ_LEN_MAX[cpu], len64);
        }
    }
    #[cfg(not(feature = "sched_telemetry"))]
    let _ = (sched, cpu);
}

pub fn sched_lock_metrics_snapshot_and_reset() -> SchedLockSiteMetrics {
    // Collect per-CPU run-queue snapshots (non-destructive read for last; swap max)
    let mut runq_len_last = [0u64; types::MAX_CPUS];
    let mut runq_len_max = [0u64; types::MAX_CPUS];
    for i in 0..types::MAX_CPUS {
        runq_len_last[i] = PROF_RUNQ_LEN_LAST[i].load(Ordering::Relaxed);
        runq_len_max[i] = PROF_RUNQ_LEN_MAX[i].swap(0, Ordering::Relaxed);
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
        task_status_polls: PROF_TASK_STATUS_POLLS.swap(0, Ordering::Relaxed),
        runnable_transitions: PROF_RUNNABLE_TRANSITIONS.swap(0, Ordering::Relaxed),
        wake_task_fastpath_already_pending: PROF_WAKE_TASK_FASTPATH_ALREADY_PENDING
            .swap(0, Ordering::Relaxed),
    }
}

/// Called from timer ISR - records tick and triggers reschedule if needed
/// Uses try_resched_if_needed to avoid deadlock when SCHEDULER is held by main code
pub fn on_tick<R: BootRuntime>() {
    let cpu_idx = crate::runtime::<R>().current_cpu_index();
    if let Some(lock) = SCHEDULER.try_lock() {
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
            if let Some(pc) = sched.state.per_cpu.get_mut(cpu_idx) {
                pc.stats.timer_interrupts = pc.stats.timer_interrupts.saturating_add(1);
            }
        }
    }
    let ticks = if cpu_idx == 0 {
        TICK_COUNT.fetch_add(1, Ordering::Relaxed) + 1
    } else {
        TICK_COUNT.load(Ordering::Relaxed)
    };

    DIAG_IPI_HANDLER.fetch_add(1, Ordering::Relaxed);

    try_resched_if_needed::<R>();
    emit_debug_summary::<R>(cpu_idx);
}

fn emit_debug_summary<R: BootRuntime>(caller_cpu: usize) {
    if caller_cpu != 0 {
        return;
    }
    let rt = crate::runtime::<R>();
    let now = rt.mono_ticks();
    let interval = rt.mono_freq_hz().max(1);
    let last = LAST_DEBUG_SUMMARY_MONO.load(Ordering::Relaxed);
    if last != 0 && now.saturating_sub(last) < interval {
        return;
    }
    if LAST_DEBUG_SUMMARY_MONO
        .compare_exchange(last, now, Ordering::Relaxed, Ordering::Relaxed)
        .is_err()
    {
        return;
    }

    let Some(lock) = SCHEDULER.try_lock() else {
        return;
    };
    let Some(ptr) = *lock else {
        return;
    };
    // SAFETY: `ptr` is written from `init::<R>` and remains valid for kernel
    // lifetime; this function only reads scheduler state under SCHEDULER lock.
    let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
    crate::kdebug!("SCHED-DBG: cpus_online={}", sched.state.online_cpu_count);
    for &i in &sched.state.online_cpus {
        let pc = &sched.state.per_cpu[i];
        let runq: usize = pc.runq.iter().map(|q| q.len()).sum();
        let runq_avg = if pc.stats.runq_sample_count == 0 {
            0
        } else {
            pc.stats.runq_sample_total / pc.stats.runq_sample_count
        };
        crate::kdebug!(
            "SCHED-DBG: cpu={} curr={:?} runq={} runq_avg={} runq_samples={} ctxsw={} idle2busy={} tick={} ipi={} enq={} deq={} wake={} lock_miss={} lock_pending={} lock_blocked={}",
            i,
            pc.current,
            runq,
            runq_avg,
            pc.stats.runq_sample_count,
            pc.stats.context_switches,
            pc.stats.idle_to_nonidle,
            pc.stats.timer_interrupts,
            pc.stats.resched_ipi_received,
            pc.stats.runnable_enqueues,
            pc.stats.runnable_dequeues,
            pc.stats.wakeups,
            pc.stats.lock_trylock_misses,
            pc.stats.lock_trylock_misses_with_pending_resched,
            pc.stats.lock_blocked_dispatch
        );
    }
}

/// Called from IPI handler - triggers reschedule without advancing time
pub fn on_resched_ipi<R: BootRuntime>() {
    let cpu_idx = crate::runtime::<R>().current_cpu_index();
    if let Some(lock) = SCHEDULER.try_lock() {
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
            if let Some(pc) = sched.state.per_cpu.get_mut(cpu_idx) {
                pc.stats.resched_ipi_received = pc.stats.resched_ipi_received.saturating_add(1);
            }
        }
    }
    DIAG_IPI_HANDLER.fetch_add(1, Ordering::Relaxed);
    crate::kdebug!(
        "SCHED: Resched IPI received on CPU {}",
        crate::runtime::<R>().current_cpu_index()
    );
    try_resched_if_needed::<R>();
}

/// Interrupt-safe version of resched_if_needed - uses try_lock to avoid deadlock
/// If SCHEDULER lock is contended, simply skip rescheduling this tick
fn try_resched_if_needed<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let irq = rt.irq_disable();
    let cpu_idx = rt.current_cpu_index();

    // Use try_lock to avoid deadlock if SCHEDULER is held by main code
    if let Some(lock) = SCHEDULER.try_lock() {
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
            let current = sched.state.per_cpu.get(cpu_idx).and_then(|pc| pc.current);
            let idle = sched.state.per_cpu.get(cpu_idx).and_then(|pc| pc.idle_task);
            let runq_total = sched
                .state
                .per_cpu
                .get(cpu_idx)
                .map(|pc| pc.runq.iter().map(|q| q.len()).sum::<usize>())
                .unwrap_or(0);
            // Capture the current task's priority from the hot-field cache to
            // avoid a nested REGISTRY lock.
            let current_prio = current
                .and_then(|tid| sched.state.get_thread(tid))
                .map(|sf| sf.priority as usize)
                .unwrap_or(0);
            // Capture whether a reschedule was explicitly requested *before*
            // schedule_point() clears these flags.
            let resched_requested =
                sched.state.per_cpu.get(cpu_idx).map_or(false, |pc| pc.need_resched)
                    || GLOBAL_NEED_RESCHED[cpu_idx].load(Ordering::Acquire);
            let switch = sched.schedule_point(ScheduleReason::PreemptTick);
            // Drain IPIs deferred by wake_sleepers while the SCHEDULER lock is
            // still held, so we can send them after releasing the lock.
            let deferred_ipis = core::mem::take(&mut sched.pending_wake_ipis);
            // Drain IPIs deferred by prepare_schedule misroute handling while
            // the lock is still held, so we can send them after unlock.
            let deferred_prepare_ipis = core::mem::take(&mut sched.pending_prepare_schedule_ipis);
            let deferred_registry_syncs = core::mem::take(&mut sched.pending_registry_syncs);
            if let Some(switch) = switch {
                // Must drop lock before context switch!
                drop(lock);

                // Send deferred wake-sleeper IPIs now that the lock is released.
                for cpu in deferred_ipis {
                    DIAG_IPI_SENT.fetch_add(1, Ordering::Relaxed);
                    DIAG_IPI_SENT_WAKE_SLEEPERS.fetch_add(1, Ordering::Relaxed);
                    rt.send_ipi(cpu, 0x30);
                }
                send_deferred_prepare_schedule_ipis::<R>(deferred_prepare_ipis);
                apply_deferred_registry_syncs::<R>(deferred_registry_syncs);

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
            } else {
                if resched_requested {
                    // A resched was explicitly requested but no context switch happened.
                    // This is only a genuine anomaly when there are tasks at STRICTLY
                    // higher priority than the current task that should have preempted it.
                    // When the scheduler correctly re-selects the current task (it is the
                    // highest-priority runnable task), runq[(current_prio+1)..] will be
                    // empty and we stay silent.  Emitting a warning for that normal case
                    // produced misleading "handled resched but made no switch" floods
                    // under SMP when a Normal-priority task is the only high-priority
                    // runnable task while several lower-priority tasks wait in the queue.
                    //
                    // Additionally, suppress the warning when preemption was disabled at
                    // the time schedule_point ran.  In that case schedule_point sets
                    // per_cpu.need_resched = true so the reschedule is correctly deferred
                    // to the next safe preemption point; the queued higher-priority task
                    // will run as soon as the critical section exits.
                    let deferred_by_preempt =
                        sched.state.per_cpu.get(cpu_idx).map_or(false, |pc| pc.need_resched);
                    if !deferred_by_preempt {
                        let has_strictly_higher = sched
                            .state
                            .per_cpu
                            .get(cpu_idx)
                            .map(|pc| {
                                let start = (current_prio + 1).min(pc.runq.len());
                                pc.runq[start..].iter().any(|q| !q.is_empty())
                            })
                            .unwrap_or(false);
                        if has_strictly_higher {
                            crate::kdebug!(
                                "SCHED: CPU {} handled resched but made no switch: current={:?} idle={:?} runq_total={}",
                                cpu_idx,
                                current,
                                idle,
                                runq_total
                            );
                        }
                    }
                }
                // Release the lock before sending any deferred IPIs.
                drop(lock);
                // Send deferred wake-sleeper IPIs after the SCHEDULER lock is released.
                for cpu in deferred_ipis {
                    DIAG_IPI_SENT.fetch_add(1, Ordering::Relaxed);
                    DIAG_IPI_SENT_WAKE_SLEEPERS.fetch_add(1, Ordering::Relaxed);
                    rt.send_ipi(cpu, 0x30);
                }
                send_deferred_prepare_schedule_ipis::<R>(deferred_prepare_ipis);
                apply_deferred_registry_syncs::<R>(deferred_registry_syncs);
            }
        }
    } else {
        PROF_RESCHED_TRYLOCK_MISS.fetch_add(1, Ordering::Relaxed);
        PROF_TRYLOCK_MISS_PER_CPU[cpu_idx].fetch_add(1, Ordering::Relaxed);
        if GLOBAL_NEED_RESCHED[cpu_idx].load(Ordering::Acquire) {
            PROF_TRYLOCK_MISS_PENDING_PER_CPU[cpu_idx].fetch_add(1, Ordering::Relaxed);
        }
        if let Some(lock) = SCHEDULER.try_lock() {
            if let Some(ptr) = *lock {
                let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
                if let Some(pc) = sched.state.per_cpu.get_mut(cpu_idx) {
                    pc.stats.lock_trylock_misses = pc.stats.lock_trylock_misses.saturating_add(1);
                    if GLOBAL_NEED_RESCHED[cpu_idx].load(Ordering::Acquire) {
                        pc.stats.lock_trylock_misses_with_pending_resched =
                            pc.stats.lock_trylock_misses_with_pending_resched.saturating_add(1);
                        pc.stats.lock_blocked_dispatch =
                            pc.stats.lock_blocked_dispatch.saturating_add(1);
                    }
                }
            }
        }
        // Warn only when misses cross threshold in a 2-second per-CPU window.
        let now = rt.mono_ticks();
        let window_ticks = rt.mono_freq_hz().max(1).saturating_mul(2);
        let window_start = &TRYLOCK_MISS_WINDOW_START[cpu_idx];
        let window_count = &TRYLOCK_MISS_WINDOW_COUNT[cpu_idx];

        let start = window_start.load(Ordering::Relaxed);
        if start == 0 || now.saturating_sub(start) > window_ticks {
            window_start.store(now, Ordering::Relaxed);
            window_count.store(1, Ordering::Relaxed);
        } else {
            let misses = window_count.fetch_add(1, Ordering::Relaxed) + 1;
            if misses == TRYLOCK_MISS_WARN_THRESHOLD {
                let cooldown_ticks =
                    rt.mono_freq_hz().max(1).saturating_mul(TRYLOCK_MISS_WARN_COOLDOWN_SECS);
                let last_warn = TRYLOCK_MISS_LAST_WARN_TICK[cpu_idx].load(Ordering::Relaxed);
                if last_warn == 0 || now.saturating_sub(last_warn) >= cooldown_ticks {
                    TRYLOCK_MISS_LAST_WARN_TICK[cpu_idx].store(now, Ordering::Relaxed);
                    crate::kdebug!(
                        "SCHED: CPU {} resched try_lock misses reached {} in 2s (suppressing until window reset)",
                        cpu_idx,
                        TRYLOCK_MISS_WARN_THRESHOLD
                    );
                }
            }
        }
        // Self-healing: tell the next safe point to reschedule
        set_global_need_resched(cpu_idx);
    }
    // If try_lock failed, skip rescheduling this tick - not a problem, next tick will try again

    rt.irq_restore(irq);
}

pub(crate) fn current_cpu_index<R: BootRuntime>() -> usize {
    let rt = crate::runtime::<R>();
    rt.current_cpu_index()
}

pub(crate) fn send_deferred_prepare_schedule_ipis<R: BootRuntime>(
    deferred_ipis: alloc::vec::Vec<usize>,
) {
    if deferred_ipis.is_empty() {
        return;
    }
    let rt = crate::runtime::<R>();
    for cpu in deferred_ipis {
        DIAG_IPI_SENT.fetch_add(1, Ordering::Relaxed);
        DIAG_IPI_SENT_PREPARE_SCHEDULE.fetch_add(1, Ordering::Relaxed);
        rt.send_ipi(cpu, 0x30);
    }
}

pub(crate) fn apply_deferred_registry_syncs<R: BootRuntime>(
    deferred_updates: alloc::vec::Vec<types::DeferredRegistrySync>,
) {
    for update in deferred_updates {
        if let Some(mut task) = crate::task::registry::get_task_mut::<R>(update.tid) {
            if let Some(state) = update.new_state {
                task.state = state;
            }
            if let Some(enqueued_at_tick) = update.new_enqueued_at_tick {
                task.enqueued_at_tick = enqueued_at_tick;
            }
            if let Some(last_cpu) = update.new_last_cpu {
                task.last_cpu = Some(last_cpu);
            }
        }
    }
}

pub(crate) fn select_any_affinity_wake_cpu<R: BootRuntime>(
    sched: &types::Scheduler<R>,
    preferred_cpu: usize,
) -> usize {
    init_any_wake_policy_from_env_once();

    let local_cpu = current_cpu_index::<R>();
    let preferred = if preferred_cpu < sched.state.per_cpu.len()
        && sched.state.online_cpus.contains(&preferred_cpu)
    {
        preferred_cpu
    } else if local_cpu < sched.state.per_cpu.len() {
        local_cpu
    } else {
        0
    };

    let policy = any_wake_overload_policy_from_u8(ANY_WAKE_OVERLOAD_POLICY.load(Ordering::Acquire));
    if policy == AnyWakeOverloadPolicy::Off {
        return preferred;
    }

    let preferred_depth = runq_depth_for_cpu(&sched.state, preferred);
    let overload_gap = ANY_WAKE_OVERLOAD_GAP.load(Ordering::Acquire);
    let overloaded = preferred_depth >= overload_gap;
    let Some(streak_cell) = ANY_WAKE_OVERLOAD_STREAK.get(preferred) else {
        return preferred;
    };
    if !overloaded {
        streak_cell.store(0, Ordering::Release);
        return preferred;
    }

    let streak_required = ANY_WAKE_OVERLOAD_STREAK_REQUIRED.load(Ordering::Acquire) as u8;
    let prior_streak = streak_cell.load(Ordering::Acquire);
    let next_streak = prior_streak.saturating_add(1);
    streak_cell.store(next_streak, Ordering::Release);
    if next_streak < streak_required {
        return preferred;
    }

    let Some((least_cpu, least_depth)) = least_loaded_online_cpu(&sched.state) else {
        return preferred;
    };
    // Run-queue depth is a bounded queue-length sum; use saturating subtraction
    // so "depth delta >= gap" cannot wrap.
    let overloaded_vs_least = preferred_depth.saturating_sub(least_depth) >= overload_gap;
    if overloaded_vs_least && least_cpu != preferred {
        streak_cell.store(0, Ordering::Release);
        least_cpu
    } else {
        preferred
    }
}

#[cfg(test)]
fn reset_any_wake_policy_for_tests() {
    ANY_WAKE_POLICY_INIT_DONE.store(true, Ordering::Release);
    ANY_WAKE_OVERLOAD_POLICY.store(AnyWakeOverloadPolicy::Off as u8, Ordering::Release);
    ANY_WAKE_OVERLOAD_GAP.store(4, Ordering::Release);
    ANY_WAKE_OVERLOAD_STREAK_REQUIRED.store(3, Ordering::Release);
    for streak in &ANY_WAKE_OVERLOAD_STREAK {
        streak.store(0, Ordering::Release);
    }
}

#[cfg(test)]
fn set_any_wake_policy_for_tests(policy: &str, overload_gap: usize) {
    set_any_wake_policy_for_tests_with_streak(policy, overload_gap, 1);
}

#[cfg(test)]
fn set_any_wake_policy_for_tests_with_streak(policy: &str, overload_gap: usize, overload_streak: usize) {
    ANY_WAKE_POLICY_INIT_DONE.store(true, Ordering::Release);
    ANY_WAKE_OVERLOAD_POLICY.store(parse_any_wake_overload_policy(policy) as u8, Ordering::Release);
    ANY_WAKE_OVERLOAD_GAP.store(overload_gap.max(1), Ordering::Release);
    ANY_WAKE_OVERLOAD_STREAK_REQUIRED.store(
        overload_streak.clamp(1, ANY_WAKE_OVERLOAD_STREAK_MAX),
        Ordering::Release,
    );
    for streak in &ANY_WAKE_OVERLOAD_STREAK {
        streak.store(0, Ordering::Release);
    }
}

pub fn init<R: BootRuntime>() {
    crate::kdebug!("  Acquiring scheduler lock...");
    let mut lock = SCHEDULER.lock();
    crate::kdebug!("  Lock acquired, checking if initialized...");
    if lock.is_none() {
        crate::kdebug!("  Allocating scheduler...");
        let sched = alloc::boxed::Box::new(types::Scheduler::<R>::new());
        crate::kdebug!("  Leaking scheduler...");
        let s = alloc::boxed::Box::leak(sched);
        crate::kdebug!("  Initializing boot task...");
        init_boot_task::<R>(s);
        crate::kdebug!("  Storing scheduler pointer...");
        *lock = Some(s as *mut types::Scheduler<R> as usize);
        unsafe {
            hooks::YIELD_HOOK = Some(sleep::yield_now::<R>);
            hooks::EXIT_HOOK = Some(exit::<R>);
            hooks::SPAWN_USER_HOOK = Some(spawn::spawn_user_thread_ex::<R>);
            hooks::SPAWN_PROCESS_HOOK = Some(spawn::boot_spawn_process::<R>);
            hooks::CURRENT_TID_HOOK = Some(current_tid::<R>);
            hooks::INTERRUPT_TASK_HOOK = Some(interrupt_task::<R>);
            hooks::TAKE_PENDING_INTERRUPT_HOOK = Some(take_pending_interrupt::<R>);
            hooks::TASK_STATUS_HOOK = Some(task_status::<R>);
            hooks::TASK_WAIT_HOOK = Some(wait_task::<R>);
            hooks::SET_PRIORITY_HOOK = Some(set_priority::<R>);
            hooks::CURRENT_PRIORITY_HOOK = Some(current_priority::<R>);
            hooks::AVAILABLE_PARALLELISM_HOOK = Some(available_parallelism::<R>);
            hooks::ALLOC_USER_STACK_HOOK = Some(stack::alloc_user_stack::<R>);
            hooks::RUN_SCHEDULER_HOOK = Some(crate::task::run_scheduler::<R>);
            hooks::KILL_BY_TID_HOOK = Some(kill_by_tid::<R>);
            hooks::DUMP_STATS_HOOK = Some(crate::task::dump_stats::<R>);
            crate::memory::set_map_user_page_hook(stack::map_user_page::<R>);
            crate::memory::set_map_user_page_perms_hook(stack::map_user_page_perms::<R>);
            crate::memory::set_unmap_user_page_hook(stack::unmap_user_page::<R>);
            crate::memory::set_protect_user_page_hook(stack::protect_user_page::<R>);
            hooks::STACK_FAULT_HOOK = Some(stack::handle_stack_fault::<R>);
            hooks::SLEEP_TICKS_HOOK = Some(sleep::sleep_ticks::<R>);
            hooks::ADD_USER_MAPPING_HOOK = Some(vm::add_user_mapping::<R>);
            hooks::REMOVE_USER_MAPPINGS_HOOK = Some(vm::remove_user_mappings::<R>);
            hooks::CHECK_USER_MAPPING_HOOK = Some(vm::check_user_mapping::<R>);
            hooks::GET_USER_MAPPING_AT_HOOK = Some(vm::get_user_mapping_at::<R>);
            hooks::PROTECT_USER_RANGE_HOOK = Some(vm::protect_user_range::<R>);
            hooks::PROCESS_INFO_HOOK = Some(process_info::<R>);
            hooks::PROCESS_INFO_FOR_TID_HOOK = Some(process_info_for_tid::<R>);
            hooks::PROCESS_INFO_FOR_PID_HOOK = Some(process_info_for_pid::<R>);
            hooks::SPAWN_PROCESS_EX_HOOK = Some(spawn::boot_spawn_process_ex::<R>);
            hooks::SPAWN_PROCESS_FROM_PATH_HOOK = Some(spawn::spawn_process_from_path::<R>);
            hooks::CURRENT_RESOURCE_HOOK = Some(current_task_resource_id_impl::<R>);
            hooks::POLL_TASK_EXIT_HOOK = Some(poll_task_exit::<R>);
            hooks::REGISTER_TASK_EXIT_WAITER_HOOK = Some(register_task_exit_waiter_public::<R>);
            hooks::UNREGISTER_TASK_EXIT_WAITER_HOOK = Some(unregister_task_exit_waiter::<R>);
            hooks::REGISTER_TIMEOUT_WAKE_HOOK = Some(register_timeout_wake::<R>);
            hooks::UNREGISTER_TIMEOUT_WAKE_HOOK = Some(unregister_timeout_wake::<R>);
            hooks::LIST_PROCESSES_HOOK = Some(list_processes::<R>);
            hooks::CURRENT_TASK_NAME_HOOK = Some(current_task_name_impl::<R>);
            hooks::TASK_EXEC_HOOK = Some(crate::task::exec::task_exec_current::<R>);
            hooks::SET_CURRENT_USER_FS_BASE_HOOK = Some(set_current_user_fs_base::<R>);
            hooks::CURRENT_USER_FS_BASE_HOOK = Some(current_user_fs_base::<R>);
            hooks::SET_CURRENT_TASK_NAME_HOOK = Some(set_current_task_name::<R>);
            hooks::WAITPID_HOOK = Some(waitpid::<R>);
            hooks::GET_SIGNAL_MASK_HOOK = Some(get_signal_mask::<R>);
            hooks::SET_SIGNAL_MASK_HOOK = Some(set_signal_mask::<R>);
            hooks::GET_THREAD_PENDING_HOOK = Some(get_thread_pending::<R>);
            hooks::SET_THREAD_PENDING_HOOK = Some(set_thread_pending::<R>);
            crate::memory::set_translate_user_page_hook(vm::translate_user_page::<R>);
        }
        blocking::init_blocking_hooks::<R>();
        let cpu_total = if let Some(ptr) = *lock {
            let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
            sched.total_cpu_count
        } else {
            1
        };
        crate::contract!("Scheduler initialized");
    }
}

fn init_boot_task<R: BootRuntime>(sched: &mut types::Scheduler<R>) {
    let rt = crate::runtime::<R>();
    let cpu_total = rt.cpu_total_count();

    // Initialize PerCpu state for all CPUs (initially empty/offline)
    for _ in 0..cpu_total {
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
    }

    sched.total_cpu_count = cpu_total;
    sched.state.set_boot_cpu_online();

    // Enter early-boot mode: defer remote placement and suppress IPI traffic
    // until end_bringup() is called after all service spawning is complete.
    sched.bringup_in_progress = true;

    crate::kdebug!("  Creating boot task...");

    let layout = alloc::alloc::Layout::from_size_align(16384, 8).unwrap();
    let stack_base = unsafe { alloc::alloc::alloc(layout) };
    if stack_base.is_null() {
        panic!("Failed to allocate stack for boot task");
    }
    let stack_top = (stack_base as u64) + 16384;

    let task: Task<R> = Task {
        id: 0,
        state: TaskState::Running,
        priority: TaskPriority::Normal,
        kstack_base: stack_base,
        kstack_size: 16384,
        kstack_top: stack_top,
        ctx: Default::default(),
        aspace: rt.tasking().active_address_space(),
        simd: crate::simd::SimdState::new(rt),
        exit_code: None,
        exit_waiters: crate::sched::WaitQueue::new(),
        is_user: false,
        wake_pending: false,
        pending_interrupt: false,
        stack_info: None,
        mappings: alloc::sync::Arc::new(spin::Mutex::new(
            crate::memory::mappings::MappingList::new(),
        )),
        timeslice_remaining: types::DEFAULT_TIMESLICE,
        affinity: crate::task::Affinity::Any,
        last_cpu: Some(0),
        name: {
            let mut n = [0u8; 32];
            n[0] = b'b';
            n[1] = b'o';
            n[2] = b'o';
            n[3] = b't';
            n
        },
        name_len: 4,
        process_info: None,
        enqueued_at_tick: TICK_COUNT.load(Ordering::Relaxed),
        base_priority: TaskPriority::Normal,
        user_fs_base: 0,
        detached: false,
        signals: crate::signal::ThreadSignals::new(),
    };
    let sched_fields = bridge::TaskSchedCache::from_thread(&task)
        .with_wake_cpu(Some(0))
        .with_run_cpu(Some(0))
        .into_sched_fields(task.id);
    sched.state.insert_task(sched_fields);
    crate::task::registry::get_registry::<R>().insert(alloc::boxed::Box::new(task));

    // Boot task runs on CPU 0
    sched.state.per_cpu[0].current = Some(0);

    // Link boot task to CPU 0

    crate::kdebug!("  Creating idle tasks...");

    // Create idle task for CPU 0 initially
    {
        let i = 0;
        let idle_id = sched.spawn(
            idle_task::<R>,
            StartupArg::Raw(i),
            TaskPriority::Idle,
            crate::task::Affinity::Pinned(i),
        );

        // Remove from run queues - idle tasks are special
        for q in sched.state.per_cpu.iter_mut().flat_map(|pc| pc.runq.iter_mut()) {
            if let Some(pos) = q.iter().position(|&id| id == idle_id) {
                q.remove(pos);
            }
        }

        // Set as this CPU's idle task
        sched.state.per_cpu[i].idle_task = Some(idle_id);

        // Pin idle task to its CPU
        if let Some(mut t) = crate::task::registry::get_task_mut::<R>(idle_id) {
            t.affinity = crate::task::Affinity::Pinned(i);
        }
    }

    crate::kdebug!("  Boot task initialized");
}

impl<R: BootRuntime> types::Scheduler<R> {
    pub fn schedule_point(
        &mut self,
        reason: ScheduleReason,
    ) -> Option<
        SwitchParams<
            <R::Tasking as BootTasking>::Context,
            <R::Tasking as BootTasking>::AddressSpace,
        >,
    > {
        let cpu_idx = current_cpu_index::<R>();
        let global_requested = GLOBAL_NEED_RESCHED[cpu_idx].swap(false, Ordering::Acquire);

        if self.preempt_disable_depth > 0 {
            if global_requested {
                self.state.per_cpu[cpu_idx].need_resched = true;
            }
            return None;
        }

        match reason {
            ScheduleReason::PreemptTick => {
                // Wake any sleeping tasks whose time has expired.
                self.wake_sleepers();

                // Check preemption watchdog
                self.check_preempt_watchdog();

                let mut should_yield = global_requested || self.state.per_cpu[cpu_idx].need_resched;
                self.state.per_cpu[cpu_idx].need_resched = false;

                // Tick bookkeeping: decrement timeslice via the hot-field cache,
                // avoiding a nested REGISTRY lock on every timer tick.
                if let Some(current_id) = self.state.per_cpu[cpu_idx].current {
                    if let Some(sf) = self.state.get_thread_mut(current_id) {
                        if sf.timeslice_remaining > 0 {
                            sf.timeslice_remaining -= 1;
                        }
                        if sf.timeslice_remaining == 0 {
                            // Reset for next run
                            sf.timeslice_remaining = types::DEFAULT_TIMESLICE;
                            should_yield = true;
                        }
                    }
                }

                if should_yield {
                    // Force reschedule (safe now because REGISTRY lock is dropped)
                    return self.prepare_yield();
                }
                return None; // Not expired yet
            }
            ScheduleReason::SafePoint
            | ScheduleReason::ReschedIfNeeded
            | ScheduleReason::SleepWait => {
                // No tick bookkeeping, no timeslice decrement.
                // Simply yield if a reschedule was requested.
                if self.state.per_cpu[cpu_idx].need_resched
                    || global_requested
                    || reason == ScheduleReason::SleepWait
                {
                    self.state.per_cpu[cpu_idx].need_resched = false;
                    return self.prepare_yield();
                }
                return None;
            }
            _ => {
                // For other reasons (Unblock, etc), always attempt yield
                return self.prepare_yield();
            }
        }
    }

    /// Check if preemption has been disabled too long
    fn check_preempt_watchdog(&mut self) {
        if self.preempt_disable_depth > 0 && !self.watchdog_warned {
            let now = TICK_COUNT.load(Ordering::Relaxed);
            if now.saturating_sub(self.preempt_disable_since) > 500 {
                // );
                self.watchdog_warned = true;
            }
        }
    }

    /// Wake any sleeping tasks whose sleep time has expired
    fn wake_sleepers(&mut self) {
        let now = TICK_COUNT.load(Ordering::Relaxed);
        let lock_start = crate::runtime::<R>().mono_ticks();
        let mut wake_budget = self
            .wake_sleepers_budget_carry
            .saturating_add(types::WAKE_SLEEPERS_BUDGET_PER_TICK)
            .min(types::WAKE_SLEEPERS_BUDGET_CARRY_CAP);

        // Collect pending IPIs and send them *after* this function returns (i.e.
        // after the caller drops the SCHEDULER lock) to reduce IPI-while-locked
        // contention on SMP.  We accumulate at most one IPI per remote CPU; in
        // practice the number of CPUs is small so a fixed-size stack buffer is used.
        let mut pending_ipis: alloc::vec::Vec<usize> = alloc::vec::Vec::new();

        // Collect all (tid, priority, target_cpu) from the hot-field cache first.
        // We then do a single REGISTRY lock acquisition for the batch of REGISTRY
        // writes rather than one acquisition per task, reducing the number of
        // nested SCHEDULER → REGISTRY lock cycles from N to 1.
        let mut to_wake: alloc::vec::Vec<(u64, usize, usize)> = alloc::vec::Vec::new();

        while wake_budget > 0 {
            let Some((&wake_tick, _)) = self.state.sleep_queue.first_key_value() else {
                break;
            };
            if wake_tick <= now {
                let (_, mut tids) = self.state.sleep_queue.pop_first().unwrap();
                let to_take = core::cmp::min(wake_budget, tids.len());

                for tid in tids.drain(..to_take) {
                    // This task left the sleep queue (woken or dropped if task
                    // record vanished), so clear direct membership now.
                    self.state.sleep_membership.remove(&tid);
                    // Read scheduling fields from the hot-field cache only.
                    // REGISTRY is not accessed in this inner loop.
                    if let Some(sf) = self.state.get_thread(tid) {
                        let priority = sf.priority as usize;
                        let target_cpu = match sf.affinity {
                            crate::task::Affinity::Pinned(cpu) => cpu,
                            crate::task::Affinity::Any => {
                                let preferred =
                                    sf.last_cpu.unwrap_or_else(|| current_cpu_index::<R>());
                                select_any_affinity_wake_cpu::<R>(self, preferred)
                            }
                        };
                        to_wake.push((tid, priority, target_cpu));
                    }
                    // If not in hot-field cache, skip (task was already removed).
                }

                wake_budget = wake_budget.saturating_sub(to_take);
                if !tids.is_empty() {
                    self.state.sleep_queue.insert(wake_tick, tids);
                    // Remaining tids stayed in this bucket after budget limiting;
                    // refresh their direct membership indices in one pass.
                    self.state.refresh_sleep_bucket_membership(wake_tick);
                    break;
                }
            } else {
                break;
            }
        }

        self.wake_sleepers_budget_carry = wake_budget;

        // Single REGISTRY lock acquisition for all waking tasks.
        // This replaces the previous per-task `get_task_mut` calls, reducing
        // SCHEDULER → REGISTRY nested-lock cycles from N to 1.
        let confirmed: alloc::vec::Vec<(u64, usize, usize)> = if !to_wake.is_empty() {
            let mut reg = crate::task::registry::get_registry::<R>();
            to_wake
                .iter()
                .filter_map(|&(tid, priority, target_cpu)| {
                    if let Some(task) = reg.get_mut(tid) {
                        task.state = TaskState::Runnable;
                        task.enqueued_at_tick = now;
                        Some((tid, priority, target_cpu))
                    } else {
                        None // task removed from REGISTRY; skip enqueue
                    }
                })
                .collect()
            // REGISTRY lock released here (reg dropped).
        } else {
            alloc::vec::Vec::new()
        };

        // Now that the REGISTRY lock is released, update the hot-field cache and
        // enqueue confirmed tasks into the run queues.
        for (tid, priority, target_cpu) in confirmed {
            // Update the scheduler-side cache to match the REGISTRY write above.
            if let Some(sf) = self.state.get_thread_mut(tid) {
                sf.state = TaskState::Runnable;
                sf.enqueued_at_tick = now;
                sf.wake_cpu = Some(target_cpu);
            }

            let actual_cpu = if target_cpu < self.state.per_cpu.len() { target_cpu } else { 0 };
            self.state.enqueue_task(actual_cpu, priority, tid);
            if let Some(pc) = self.state.per_cpu.get_mut(actual_cpu) {
                pc.stats.wakeups = pc.stats.wakeups.saturating_add(1);
            }

            // Use cached priority for the current task to avoid a REGISTRY lock.
            let current_prio = self
                .state
                .per_cpu
                .get(actual_cpu)
                .and_then(|pc| pc.current)
                .and_then(|cid| self.state.get_thread(cid))
                .map(|sf| sf.priority as usize)
                .unwrap_or(0);
            if priority > current_prio {
                if actual_cpu == current_cpu_index::<R>() {
                    self.state.per_cpu[current_cpu_index::<R>()].need_resched = true;
                }
            }

            if actual_cpu != current_cpu_index::<R>() {
                // Suppress duplicate IPI if the pending flag was already
                // set by a previous wakeup.  The in-flight IPI will pick
                // up this task when it is processed.
                let already_pending = set_global_need_resched(actual_cpu);
                if !already_pending {
                    crate::kdebug!(
                        "SCHED: Nudging CPU {} for task {} (prio {})",
                        actual_cpu,
                        tid,
                        priority
                    );
                    // De-dup: only queue once per CPU during this pass.
                    if !pending_ipis.contains(&actual_cpu) {
                        pending_ipis.push(actual_cpu);
                    }
                } else {
                    PROF_IPI_SUPPRESSED.fetch_add(1, Ordering::Relaxed);
                }
            }
        }

        record_sched_lock_hold::<R>(
            &PROF_SCHED_LOCK_WAKE_SLEEPERS_CALLS,
            &PROF_SCHED_LOCK_WAKE_SLEEPERS_US_TOTAL,
            &PROF_SCHED_LOCK_WAKE_SLEEPERS_US_MAX,
            &PROF_SCHED_LOCK_WAKE_SLEEPERS_HOLD_HIST,
            lock_start,
        );

        // Defer IPI sends until after the SCHEDULER lock is released.
        // Lock-owning call sites drain `self.pending_wake_ipis` and send them
        // after dropping the lock, so `send_ipi` is never called while
        // SCHEDULER is held.
        for cpu in pending_ipis {
            self.pending_wake_ipis.push(cpu);
        }
    }

    pub fn preempt_disable(&mut self) {
        if self.preempt_disable_depth == 0 {
            // Track when we started disabling preemption
            self.preempt_disable_since = TICK_COUNT.load(Ordering::Relaxed);
            self.watchdog_warned = false;
        }
        self.preempt_disable_depth += 1;
        if self.preempt_disable_depth == 1 {
            // Only trace on transition to disabled? Or depth change?
            // User task says "Record (..., preempt_disable_depth)".
            // Let's trace all for now, or just 0->1.
            // 0->1 is most important for start of disable region.
            crate::trace::irq_ring::push(abi::trace::TraceEvent::PreemptDisable {
                depth: self.preempt_disable_depth as u32,
                timestamp: crate::trace::now(),
            });
        }
    }

    fn flush_metrics_if_needed(&mut self) {
        let rt = crate::runtime::<R>();
        let now = rt.mono_ticks();
        let limit = rt.mono_freq_hz() * 2;

        if self.metrics.last_flush == 0 {
            self.metrics.last_flush = now;
            return;
        }

        if now - self.metrics.last_flush > limit {
            #[cfg(feature = "diagnostic-apps")]
            crate::log_event!(
               crate::logging::LogLevel::Info,
               "sched.activity",
               "Scheduler Activity Rollup",
               {
                   yields: self.metrics.yields,
                   pops: self.metrics.pops,
                   pushes: self.metrics.pushes,
                   idle_picks: self.metrics.idle_picks,
                   runq_len: self.runq.iter().map(|q| q.len()).sum::<usize>() as u64
               },
               about=[]
            );

            self.metrics.yields = 0;
            self.metrics.pops = 0;
            self.metrics.pushes = 0;
            self.metrics.idle_picks = 0;
            self.metrics.last_flush = now;
        }
    }

    pub fn preempt_enable(
        &mut self,
    ) -> Option<
        SwitchParams<
            <R::Tasking as BootTasking>::Context,
            <R::Tasking as BootTasking>::AddressSpace,
        >,
    > {
        if self.preempt_disable_depth > 0 {
            self.preempt_disable_depth -= 1;
        }

        if self.preempt_disable_depth == 0
            && self.state.per_cpu[current_cpu_index::<R>()].need_resched
        {
            self.state.per_cpu[current_cpu_index::<R>()].need_resched = false;
            return self.schedule_point(ScheduleReason::SafePoint);
        }
        None
    }

    pub fn prepare_yield(
        &mut self,
    ) -> Option<
        SwitchParams<
            <R::Tasking as BootTasking>::Context,
            <R::Tasking as BootTasking>::AddressSpace,
        >,
    > {
        let cpu_idx = current_cpu_index::<R>();
        let current_id = self.state.per_cpu.get(cpu_idx)?.current?;

        self.metrics.yields += 1;

        // Don't push idle task, dead tasks, or already-blocked tasks back to runq.
        // Use the scheduler-side hot-field cache to avoid a nested REGISTRY lock.
        if Some(current_id) != self.state.per_cpu[cpu_idx].idle_task {
            if let Some(t) = self.state.get_thread(current_id) {
                if t.state != TaskState::Dead && t.state != TaskState::Blocked {
                    let priority = t.priority;
                    // Push to LOCAL runq (we are yielding on this CPU)
                    self.state.enqueue_task(cpu_idx, priority as usize, current_id);
                    self.metrics.pushes += 1;
                }
            }
        }

        self.prepare_schedule()
    }

    /// Drain up to `max_to_flush` deferred misrouted tasks, requeue each task on
    /// its target CPU, and schedule a deduplicated remote reschedule nudge.
    ///
    /// This keeps misroute cleanup incremental so `prepare_schedule` can keep a
    /// short picker fast path even when a large misroute backlog exists.
    #[inline]
    fn flush_pending_misrouted_requeues_bounded(&mut self, max_to_flush: usize) {
        for _ in 0..max_to_flush {
            let Some((prio, target_cpu, id)) = self.pending_misrouted_requeues.pop() else {
                break;
            };
            self.state.enqueue_task(target_cpu, prio, id);
            self.queue_prepare_schedule_ipi_dedup(target_cpu);
        }
    }

    /// Queue a misrouted task for bounded repair when backlog allows.
    ///
    /// If the deferred backlog cap is reached, fall back to synchronous repair
    /// of this one task so tasks are never dropped and the deferred queue
    /// remains memory-bounded.
    #[inline]
    fn defer_or_repair_misroute(&mut self, prio: usize, target_cpu: usize, id: TaskId) {
        if self.pending_misrouted_requeues.len() < PREPARE_SCHEDULE_MISROUTE_BACKLOG_CAP {
            self.pending_misrouted_requeues.push((prio, target_cpu, id));
            return;
        }
        // Backlog safety valve: avoid unbounded memory growth if misroute intake
        // outpaces the bounded per-call repair budget.
        self.state.enqueue_task(target_cpu, prio, id);
        self.queue_prepare_schedule_ipi_dedup(target_cpu);
    }

    #[inline]
    fn queue_prepare_schedule_ipi_dedup(&mut self, target_cpu: usize) {
        let already_pending = set_global_need_resched(target_cpu);
        if !already_pending {
            if !self.pending_prepare_schedule_ipis.contains(&target_cpu) {
                self.pending_prepare_schedule_ipis.push(target_cpu);
            }
        } else {
            PROF_IPI_SUPPRESSED.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub(crate) fn prepare_schedule(
        &mut self,
    ) -> Option<
        SwitchParams<
            <R::Tasking as BootTasking>::Context,
            <R::Tasking as BootTasking>::AddressSpace,
        >,
    > {
        self.flush_metrics_if_needed();

        let rt = crate::runtime::<R>();
        let cpu_idx = current_cpu_index::<R>();
        let real_cpu_id = rt.current_cpu_id().0 as usize;
        if cpu_idx != real_cpu_id {
            crate::kprintln!(
                "FATAL GS CORRUPTION: Core {} thinks it is index {} via GS!",
                real_cpu_id,
                cpu_idx
            );
        }
        if cpu_idx >= self.state.per_cpu.len() {
            return None;
        }
        let per_cpu_len = self.state.per_cpu.len();

        // Sample run-queue depth for this CPU before we start dequeuing.
        sample_runq_len(self, cpu_idx);

        // Snapshot tick count once per scheduling decision so aging is both
        // consistent across this pick and free of repeated tick loads.
        let now = TICK_COUNT.load(Ordering::Relaxed);

        let mut next_id = None;
        let mut pick_attempts = 0usize;
        // Priority scan — skip dead and misrouted tasks, evaluating aging on-pick
        while pick_attempts < PREPARE_SCHEDULE_PICK_BUDGET {
            let mut best_q = None;
            let mut best_eff = 0;

            for p in (1..5).rev() {
                if let Some(&id) = self.state.per_cpu[cpu_idx].runq[p].front() {
                    let mut eff = p; // Start with base priority (queue index)
                    if p < 4 {
                        // aging only applies up to High
                        // Use the hot-field cache to avoid a nested REGISTRY lock.
                        if let Some(sf) = self.state.get_thread(id) {
                            let wait_ticks = now.saturating_sub(sf.enqueued_at_tick);
                            let boost = (wait_ticks / types::AGING_THRESHOLD_TICKS) as usize;
                            let boost = boost.min(types::MAX_PRIORITY_BOOST);
                            eff = (p + boost).min(4);
                        }
                    }
                    // Use >= so that an aged lower-priority task wins the tie when its
                    // boosted effective priority equals a higher-priority task's. We scan
                    // from p=4 down to p=1; a later (lower-p) match with equal eff
                    // replaces the earlier one, meaning the task that *needed* aging to
                    // compete gets to run first, preventing indefinite starvation.
                    if eff >= best_eff {
                        best_eff = eff;
                        best_q = Some(p);
                    }
                }
            }

            if let Some(p) = best_q {
                let id = self.state.dequeue_task_front(cpu_idx, p).unwrap();
                pick_attempts += 1;
                self.metrics.pops += 1;

                // Use the hot-field cache for dead/affinity checks to avoid a
                // nested REGISTRY lock on every task dequeue.
                match self.state.get_thread(id) {
                    None => continue, // stale runq entry — skip
                    Some(sf) if sf.state == TaskState::Dead => continue,
                    Some(sf) => {
                        if let crate::task::Affinity::Pinned(target) = sf.affinity {
                            if target != cpu_idx && target < per_cpu_len {
                                self.defer_or_repair_misroute(sf.priority as usize, target, id);
                                continue;
                            }
                        }
                    }
                }
                next_id = Some(id);
                break;
            } else {
                break;
            }
        }

        let next_id = match next_id {
            Some(id) => Some(id),
            None => {
                // Check Idle queue — skip dead and misrouted tasks
                let mut found_idle_q = None;
                while pick_attempts < PREPARE_SCHEDULE_PICK_BUDGET {
                    let Some(id) = self.state.dequeue_task_front(cpu_idx, 0) else {
                        break;
                    };
                    pick_attempts += 1;
                    self.metrics.pops += 1;
                    // Use the hot-field cache for dead/affinity checks.
                    match self.state.get_thread(id) {
                        None => continue, // stale runq entry — skip
                        Some(sf) if sf.state == TaskState::Dead => continue,
                        Some(sf) => {
                            if let crate::task::Affinity::Pinned(target) = sf.affinity {
                                if target != cpu_idx && target < per_cpu_len {
                                    self.defer_or_repair_misroute(sf.priority as usize, target, id);
                                    continue;
                                }
                            }
                        }
                    }
                    found_idle_q = Some(id);
                    break;
                }
                if let Some(id) = found_idle_q {
                    Some(id)
                } else if let Some(idle) = self.state.per_cpu[cpu_idx].idle_task {
                    self.metrics.idle_picks += 1;
                    Some(idle)
                } else {
                    None
                }
            }
        };

        // Keep cleanup bounded so the picker path remains short under heavy
        // misroute pressure.
        self.flush_pending_misrouted_requeues_bounded(PREPARE_SCHEDULE_MISROUTE_REPAIR_BUDGET);

        let next_id = next_id?;

        let current_id = self.state.per_cpu[cpu_idx]
            .current
            .expect("prepare_schedule called without current task");

        if next_id == current_id {
            let idx = self.state.get_task_index(current_id).unwrap_or_else(|| {
                crate::kerror!(
                    "SchedTasks: {:?}",
                    self.state.threads.iter().map(|f| f.tid).collect::<alloc::vec::Vec<_>>()
                );
                panic!("failed to find current_id {} in get_task_index", current_id)
            });
            // Keep scheduler cache fields in sync. No REGISTRY write is needed
            // in the same-task (no-switch) case: this task remains running and
            // no lifecycle transition occurred.
            self.state.threads[idx].state = TaskState::Running;
            self.state.threads[idx].run_cpu = Some(cpu_idx);
            return None;
        }

        let current_was_idle = Some(current_id) == self.state.per_cpu[cpu_idx].idle_task;
        let next_is_nonidle = Some(next_id) != self.state.per_cpu[cpu_idx].idle_task;
        if current_was_idle && next_is_nonidle {
            self.state.per_cpu[cpu_idx].stats.idle_to_nonidle =
                self.state.per_cpu[cpu_idx].stats.idle_to_nonidle.saturating_add(1);
        }
        self.state.per_cpu[cpu_idx].stats.context_switches =
            self.state.per_cpu[cpu_idx].stats.context_switches.saturating_add(1);

        self.state.per_cpu[cpu_idx].current = Some(next_id);

        let old_idx = self.state.get_task_index(current_id).unwrap_or_else(|| {
            crate::kerror!(
                "SchedTasks: {:?}",
                self.state.threads.iter().map(|f| f.tid).collect::<alloc::vec::Vec<_>>()
            );
            panic!("failed to find current_id {} in get_task_index", current_id)
        });
        let new_idx = self.state.get_task_index(next_id).unwrap_or_else(|| {
            crate::kerror!(
                "SchedTasks: {:?}",
                self.state.threads.iter().map(|f| f.tid).collect::<alloc::vec::Vec<_>>()
            );
            panic!("failed to find next_id {} in get_task_index", next_id)
        });

        // Drive transition decisions from the scheduler hot-cache and defer
        // REGISTRY synchronization for these state fields until after the
        // SCHEDULER lock is released.
        let old_was_running = self.state.threads[old_idx].state == TaskState::Running;
        let mut old_registry_sync = types::DeferredRegistrySync {
            tid: current_id,
            new_state: None,
            new_enqueued_at_tick: None,
            new_last_cpu: Some(cpu_idx),
        };
        if old_was_running {
            self.state.threads[old_idx].state = TaskState::Runnable;
            self.state.threads[old_idx].enqueued_at_tick = now;
            old_registry_sync.new_state = Some(TaskState::Runnable);
            old_registry_sync.new_enqueued_at_tick = Some(now);
        }
        self.state.threads[old_idx].last_cpu = Some(cpu_idx);
        self.pending_registry_syncs.push(old_registry_sync);
        self.state.threads[new_idx].state = TaskState::Running;
        self.state.threads[new_idx].last_cpu = Some(cpu_idx);
        self.state.threads[new_idx].run_cpu = Some(cpu_idx);
        self.pending_registry_syncs
            .push(types::DeferredRegistrySync {
                tid: next_id,
                new_state: Some(TaskState::Running),
                new_enqueued_at_tick: None,
                new_last_cpu: Some(cpu_idx),
            });
        // Only timeslice_remaining is intentionally managed exclusively via the
        // hot-field cache:
        // schedule_point decrements it without touching REGISTRY. The REGISTRY
        // copy may therefore be stale between context switches; this is
        // intentional and acceptable because no correctness-critical path reads
        // it from REGISTRY (dump_stats shows it for diagnostics only), unlike
        // lifecycle/placement fields which are now deferred-synced above.

        let mut reg = crate::task::registry::get_registry::<R>();
        let (old_task, new_task) = if old_idx < new_idx {
            let (left, right) = reg.threads.split_at_mut(new_idx);
            (&mut left[old_idx], &mut right[0])
        } else {
            let (left, right) = reg.threads.split_at_mut(old_idx);
            (&mut right[0], &mut left[new_idx])
        };

        // Update the lock-free mapping cache for this CPU so check_user_mapping is fast
        crate::sched::vm::CURRENT_MAPPINGS[cpu_idx].store(
            alloc::sync::Arc::as_ptr(&new_task.mappings) as *mut _,
            core::sync::atomic::Ordering::Release,
        );

        old_task.simd.save(crate::runtime::<R>());
        new_task.simd.restore(crate::runtime::<R>());

        crate::trace::irq_ring::push(abi::trace::TraceEvent::ContextSwitch {
            from: old_task.id,
            to: new_task.id,
            timestamp: crate::trace::now(),
        });

        Some(SwitchParams {
            from_ctx: &mut old_task.ctx as *mut _,
            to_ctx: &new_task.ctx as *const _,
            to_aspace: new_task.aspace,
            from_tid: old_task.id,
            to_tid: new_task.id,
            from_aspace: old_task.aspace,
            from_user: old_task.is_user,
            to_user: new_task.is_user,
            from_user_fs_base: &mut old_task.user_fs_base as *mut u64,
            to_user_fs_base: new_task.user_fs_base,
        })
    }

    pub fn terminate_current(
        &mut self,
        code: i32,
    ) -> (
        SwitchParams<
            <R::Tasking as BootTasking>::Context,
            <R::Tasking as BootTasking>::AddressSpace,
        >,
        alloc::vec::Vec<u64>,
    ) {
        let cpu_idx = current_cpu_index::<R>();
        let current_id = self
            .state
            .per_cpu
            .get(cpu_idx)
            .and_then(|pc| pc.current)
            .expect("terminate_current called with no current task");

        let waiters = mark_task_exited::<R>(self, current_id, code);
        purge_task_from_scheduler_queues::<R>(self, current_id);

        // Release any claimed devices
        let released = crate::device_registry::REGISTRY.lock().release_all_for_task(current_id);
        if released > 0 {
            crate::kinfo!("DEVICE: released {} claims for task {}", released, current_id);
        }

        loop {
            if let Some(switch) = self.prepare_schedule() {
                return (switch, waiters);
            }
        }
    }

    pub fn set_priority(&mut self, id: TaskId, priority: TaskPriority) {
        if let Some(idx) = self.state.get_task_index(id) {
            let old_priority = crate::task::registry::get_registry::<R>().threads[idx].priority;
            crate::task::registry::get_registry::<R>().threads[idx].priority = priority;
            crate::task::registry::get_registry::<R>().threads[idx].base_priority = priority; // Update base priority for anti-starvation

            // Keep the scheduler-side priority cache in sync.
            self.state.threads[idx].priority = priority;

            // If it's runnable and in a runq, move it to the new runq
            if self.state.threads[idx].state == TaskState::Runnable {
                let loc = self.state.get_task(id).and_then(|t| t.runq_location);
                if let Some((cpu, _)) = loc {
                    self.state.remove_task_from_runq(id);
                    self.state.enqueue_task(cpu, priority as usize, id);
                }
            }
        }
    }

    /// Mark a secondary CPU as online and initialize its idle task.
    pub fn cpu_online(&mut self, cpu_index: usize) {
        crate::kdebug!("SMP: CPU {} online (triggered by scheduler spawn)", cpu_index);
        self.bringup_in_progress = false;

        // Create idle task for this new CPU
        let i = cpu_index;
        let idle_id = self.spawn(
            idle_task::<R>,
            StartupArg::Raw(i),
            TaskPriority::Idle,
            crate::task::Affinity::Pinned(i),
        );

        // Remove from run queues - idle tasks are special
        self.state.remove_task_from_runq(idle_id);

        // Set as this CPU's idle task
        self.state.per_cpu[i].idle_task = Some(idle_id);

        // Pin idle task to its CPU and keep the cache in sync.
        if let Some(mut t) = crate::task::registry::get_task_mut::<R>(idle_id) {
            t.affinity = crate::task::Affinity::Pinned(i);
        }
        if let Some(sf) = self.state.get_task_mut(idle_id) {
            sf.affinity = crate::task::Affinity::Pinned(i);
        }
    }
}

/// Transition the scheduler out of early-boot mode.
///
/// During early boot (`bringup_in_progress == true`) the scheduler places all
/// `Affinity::Any` tasks on the local (boot) CPU and suppresses remote-wakeup
/// IPIs so that service bring-up incurs minimal cross-CPU coordination.
///
/// Call this function once all initial services have been spawned and the
/// system is ready to enter steady-state scheduling.  After this point the
/// normal round-robin CPU selection and IPI delivery resume.
pub fn end_bringup<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        if sched.bringup_in_progress {
            crate::kinfo!(
                "SCHED: early-boot bringup complete; resuming steady-state SMP scheduling"
            );
            sched.bringup_in_progress = false;
        }
    }
    rt.irq_restore(_irq);
}

pub fn set_priority<R: BootRuntime>(id: TaskId, priority: TaskPriority) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        sched.set_priority(id, priority);
    }
    rt.irq_restore(_irq);
}

pub fn task_status<R: BootRuntime>(id: TaskId) -> Option<(TaskState, Option<i32>)> {
    // Task state and exit code live in the registry, not the scheduler.
    // Holding SCHEDULER here was unnecessary and caused timer-ISR try_lock
    // misses on all other CPUs (the supervisor polls every task every cycle).
    PROF_TASK_STATUS_POLLS.fetch_add(1, Ordering::Relaxed);
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let res = crate::task::registry::get_task::<R>(id).map(|t| (t.state, t.exit_code));
    rt.irq_restore(_irq);
    res
}

pub fn current_priority<R: BootRuntime>() -> TaskPriority {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let res = if let Some(lock) = SCHEDULER.try_lock() {
        if let Some(ptr) = *lock {
            let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
            let cpu = current_cpu_index::<R>();
            sched
                .state
                .per_cpu
                .get(cpu)
                .and_then(|pc| pc.current)
                .and_then(|tid| crate::task::registry::get_task::<R>(tid))
                .map(|t| bridge::SchedulableRuntime::from_thread(&*t).priority)
                .unwrap_or(TaskPriority::Normal)
        } else {
            TaskPriority::Normal
        }
    } else {
        TaskPriority::Normal
    };
    rt.irq_restore(_irq);
    res
}

pub fn current_tid<R: BootRuntime>() -> u64 {
    crate::runtime::<R>().current_tid()
}

fn effective_parallelism_from_state(online_cpu_count: usize, affinity: Affinity) -> usize {
    let online = online_cpu_count.max(1);
    match affinity {
        Affinity::Pinned(_) => 1,
        Affinity::Any => online,
    }
}

pub fn available_parallelism<R: BootRuntime>() -> usize {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let result = if let Some(lock) = SCHEDULER.try_lock() {
        if let Some(ptr) = *lock {
            let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
            let online = sched.state.online_cpu_count;
            let affinity = crate::task::registry::get_task::<R>(rt.current_tid())
                .map(|task| bridge::SchedulableRuntime::from_thread(&*task).affinity)
                .unwrap_or(Affinity::Any);
            effective_parallelism_from_state(online, affinity)
        } else {
            1
        }
    } else {
        1
    };

    rt.irq_restore(_irq);
    result.max(1)
}

fn current_task_name_impl<R: BootRuntime>() -> [u8; 32] {
    let tid = current_tid::<R>();
    if let Some(task) = crate::task::registry::get_task::<R>(tid) {
        task.name
    } else {
        let mut n = [0u8; 32];
        n[0..7].copy_from_slice(b"unknown");
        n
    }
}

/// Update the current task's stored `user_fs_base` field.
///
/// Called by the TLS-set syscall handler after writing the hardware register,
/// so that the value is saved correctly on the next context switch without
/// needing an extra MSR read.
fn set_current_user_fs_base<R: BootRuntime>(base: u64) {
    let tid = crate::runtime::<R>().current_tid();
    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(tid) {
        task.user_fs_base = base;
    }
}

/// Return the current task's stored `user_fs_base` field.
fn current_user_fs_base<R: BootRuntime>() -> u64 {
    let tid = crate::runtime::<R>().current_tid();
    crate::task::registry::get_task::<R>(tid).map(|task| task.user_fs_base).unwrap_or(0)
}

/// Update the calling thread's human-readable name.
///
/// Called by `SYS_TASK_SET_NAME`.  Defensively clamps to 31 bytes even
/// though the syscall handler already enforces this limit, so that the
/// function stays safe if called from other internal paths in the future.
fn set_current_task_name<R: BootRuntime>(ptr: *const u8, len: usize) {
    let tid = crate::runtime::<R>().current_tid();
    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(tid) {
        let len = len.min(31);
        // SAFETY: `ptr` points to a kernel buffer that was copied from user
        // space by the syscall handler before this hook is called.
        let src = unsafe { core::slice::from_raw_parts(ptr, len) };
        task.name[..len].copy_from_slice(src);
        task.name_len = len as u8;
    }
}

fn interrupt_task<R: BootRuntime>(tid: TaskId) -> Result<(), abi::errors::Errno> {
    let should_wake = if let Some(mut task) = crate::task::registry::get_task_mut::<R>(tid) {
        if task.state == TaskState::Dead {
            return Err(abi::errors::Errno::ESRCH);
        }
        task.pending_interrupt = true;
        task.state == TaskState::Blocked
    } else {
        return Err(abi::errors::Errno::ESRCH);
    };

    if should_wake {
        wake_task::<R>(tid);
    }

    Ok(())
}

fn take_pending_interrupt<R: BootRuntime>() -> bool {
    let tid = crate::runtime::<R>().current_tid();
    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(tid) {
        let was_pending = task.pending_interrupt;
        task.pending_interrupt = false;
        was_pending
    } else {
        false
    }
}

/// Get the current task's ProcessInfo Arc, if any.
pub fn process_info<R: BootRuntime>()
-> Option<alloc::sync::Arc<spin::Mutex<crate::task::ProcessInfo>>> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    // Prefer the runtime's current TID. During syscall/trap handling this stays
    // authoritative even if the scheduler's per-CPU `current` view is transiently stale.
    let runtime_tid = rt.current_tid();
    let result = crate::task::registry::get_task::<R>(runtime_tid)
        .and_then(|t| t.process_info.clone())
        .or_else(|| {
            let lock = SCHEDULER.lock();
            if let Some(ptr) = *lock {
                let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
                let cpu_idx = current_cpu_index::<R>();
                sched
                    .state
                    .per_cpu
                    .get(cpu_idx)
                    .and_then(|pc| pc.current)
                    .and_then(|tid| crate::task::registry::get_task::<R>(tid))
                    .and_then(|t| t.process_info.clone())
            } else {
                None
            }
        });

    rt.irq_restore(_irq);
    result
}

pub fn process_info_for_tid<R: BootRuntime>(
    tid: u64,
) -> Option<alloc::sync::Arc<spin::Mutex<crate::task::ProcessInfo>>> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let result = crate::task::registry::get_task::<R>(tid).and_then(|t| t.process_info.clone());
    rt.irq_restore(_irq);
    result
}

pub fn process_info_for_pid<R: BootRuntime>(
    pid: u32,
) -> Option<alloc::sync::Arc<spin::Mutex<crate::task::ProcessInfo>>> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let reg = crate::task::registry::get_registry::<R>();

    let mut candidate: Option<alloc::sync::Arc<spin::Mutex<crate::task::ProcessInfo>>> = None;
    for task in reg.threads.iter() {
        let Some(pi_arc) = &task.process_info else {
            continue;
        };
        if pi_arc.lock().pid != pid {
            continue;
        }

        if task.id == pid as u64 {
            candidate = Some(pi_arc.clone());
            break;
        }

        if candidate.is_none() {
            candidate = Some(pi_arc.clone());
        }
    }

    drop(reg);
    rt.irq_restore(_irq);
    candidate
}

/// Return a snapshot of all live processes (those with a ProcessInfo).
///
/// Called from the `LIST_PROCESSES_HOOK` slot so that procfs can render
/// `/proc/<pid>/…` files without knowing the concrete `R` type.
pub fn list_processes<R: BootRuntime>() -> alloc::vec::Vec<hooks::ProcessSnapshot> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let mut out = alloc::vec::Vec::new();
    let foreground_pgid = crate::vfs::devfs::console_foreground_pgid();
    {
        let reg = crate::task::registry::get_registry::<R>();

        // Phase 9: build a TID→state map so each process snapshot can carry the
        // full set of thread states from `ProcessLifecycle.thread_ids`.  This
        // allows `kernel::job::bridge::job_state_from_snapshot` to give accurate
        // `JobState` for multi-threaded processes rather than only seeing the
        // thread-group leader's state.
        let mut tid_state: alloc::collections::BTreeMap<TaskId, TaskState> =
            alloc::collections::BTreeMap::new();
        for task in reg.threads.iter() {
            tid_state.insert(task.id, task.state);
        }

        for task in reg.threads.iter() {
            if let Some(pi_arc) = &task.process_info {
                let pi = pi_arc.lock();
                let name_bytes = &task.name[..task.name_len as usize];
                let name = alloc::string::String::from_utf8_lossy(name_bytes).into_owned();
                out.push(pi.compatibility_snapshot_for_task(
                    task.id,
                    name,
                    task.state,
                    task.exit_code,
                    foreground_pgid,
                    &tid_state,
                ));
            }
        }
    }
    rt.irq_restore(_irq);
    out
}

fn register_task_exit_waiter<R: BootRuntime>(
    target_tid: TaskId,
    waiter_tid: TaskId,
) -> Result<Option<i32>, abi::errors::Errno> {
    let target =
        crate::task::registry::get_task::<R>(target_tid).ok_or(abi::errors::Errno::ECHILD)?;

    // Joining a detached thread is not permitted.
    if target.detached {
        return Err(abi::errors::Errno::EINVAL);
    }

    if let Some(pinfo) = target.process_info.as_ref() {
        let mut pi = pinfo.lock();
        if pi.is_job_leader_tid(target_tid) {
            if let Some(code) = pi.job.leader_exit_code {
                return Ok(Some(code));
            }
            pi.job.leader_exit_waiters.push_back(waiter_tid);
            return Ok(None);
        }
    }

    if target.state == TaskState::Dead {
        return Ok(Some(target.exit_code.unwrap_or(0)));
    }

    target.exit_waiters.push_back(waiter_tid);
    Ok(None)
}

pub fn poll_task_exit<R: BootRuntime>(
    target_tid: TaskId,
) -> Result<Option<i32>, abi::errors::Errno> {
    let target =
        crate::task::registry::get_task::<R>(target_tid).ok_or(abi::errors::Errno::ECHILD)?;

    if let Some(pinfo) = target.process_info.as_ref() {
        let pi = pinfo.lock();
        if pi.is_job_leader_tid(target_tid) {
            if let Some(code) = pi.job.leader_exit_code {
                return Ok(Some(code));
            }
            return Ok(None);
        }
    }

    if target.state == TaskState::Dead { Ok(Some(target.exit_code.unwrap_or(0))) } else { Ok(None) }
}

pub fn register_task_exit_waiter_public<R: BootRuntime>(
    target_tid: TaskId,
    waiter_tid: TaskId,
) -> Result<Option<i32>, abi::errors::Errno> {
    register_task_exit_waiter::<R>(target_tid, waiter_tid)
}

pub fn unregister_task_exit_waiter<R: BootRuntime>(
    target_tid: TaskId,
    waiter_tid: TaskId,
) -> Result<(), abi::errors::Errno> {
    let target =
        crate::task::registry::get_task::<R>(target_tid).ok_or(abi::errors::Errno::ECHILD)?;
    if let Some(pinfo) = target.process_info.as_ref() {
        let mut pi = pinfo.lock();
        if pi.is_job_leader_tid(target_tid) {
            pi.job.leader_exit_waiters.remove(waiter_tid);
            return Ok(());
        }
    }
    target.exit_waiters.remove(waiter_tid);
    Ok(())
}

fn mark_task_exited<R: BootRuntime>(
    sched: &mut types::Scheduler<R>,
    tid: TaskId,
    code: i32,
) -> alloc::vec::Vec<u64> {
    // Collect exit waiters and mark the task dead.
    let mut waiters = if let Some(mut task) = crate::task::registry::get_task_mut::<R>(tid) {
        task.state = TaskState::Dead;
        task.exit_code = Some(code);
        task.exit_waiters.drain()
    } else {
        alloc::vec::Vec::new()
    };

    if let Some(task) = sched.state.get_task_mut(tid) {
        task.runq_location = None;
        task.state = TaskState::Dead;
    }

    // Remove this TID from the process's thread group list.
    // If this is the thread-group leader, drain the remaining siblings in one
    // step to avoid a separate clone + clear pass.
    // Also capture ppid/pid for lifecycle status queueing.
    let mut notify_ppid: u32 = 0;
    let mut notify_pid: u32 = 0;
    // Capture the exit observer inbox ID (if set) for canonical JobExit delivery.
    let mut exit_observer_inbox: Option<crate::inbox::InboxId> = None;

    let siblings_to_kill: alloc::vec::Vec<TaskId> = {
        let pinfo_opt =
            crate::task::registry::get_task::<R>(tid).and_then(|t| t.process_info.clone());
        if let Some(pinfo) = pinfo_opt {
            let mut pi = pinfo.lock();
            pi.remove_thread_from_job(tid);

            // If the exiting thread is the thread-group leader (its TID == pid),
            // drain all remaining siblings and schedule them for termination.
            if pi.is_job_leader_tid(tid) {
                notify_ppid = pi.runtime_parent_pid();
                notify_pid = pi.runtime_pid();
                exit_observer_inbox = pi.job_exit_observer_inbox();
                if pi.job.leader_exit_code.is_none() {
                    waiters.extend(pi.job.complete_leader_exit(code));
                }
                pi.take_job_thread_ids()
            } else {
                alloc::vec::Vec::new()
            }
        } else {
            alloc::vec::Vec::new()
        }
    };

    // Project leader-exit lifecycle semantics through the canonical Job bridge
    // (Unix wait status queue + optional JobExit observer notification).
    let parent_waiters =
        crate::job::bridge::publish_leader_exit(notify_ppid, notify_pid, code, exit_observer_inbox);
    waiters.extend(parent_waiters);

    // Kill sibling threads (thread-group exit).
    for &sibling in &siblings_to_kill {
        if let Some(mut task) = crate::task::registry::get_task_mut::<R>(sibling) {
            if task.state != TaskState::Dead {
                task.state = TaskState::Dead;
                task.exit_code = Some(code);
                let sibling_waiters = task.exit_waiters.drain();
                waiters.extend(sibling_waiters);
                drop(task);

                if let Some(sf) = sched.state.get_task_mut(sibling) {
                    sf.runq_location = None;
                    sf.state = TaskState::Dead;
                }
                sched.state.remove_task_from_runq(sibling);
                crate::kdebug!("SCHED: Killed sibling thread {} (thread-group exit)", sibling);
            }
        }
    }

    waiters
}

fn purge_task_from_scheduler_queues<R: BootRuntime>(sched: &mut types::Scheduler<R>, tid: TaskId) {
    sched.state.remove_task_from_runq(tid);

    if let Some(pos) = sched.state.wait_queue.iter().position(|&wid| wid == tid) {
        sched.state.wait_queue.remove(pos);
    }

    let _ = sched.state.remove_task_from_sleep_queue(tid);
}

fn wake_waiters(waiters: &[u64]) {
    for &tid in waiters {
        unsafe {
            crate::sched::wake_task_erased(tid);
        }
    }
}

fn current_task_resource_id_impl<R: BootRuntime>() -> Option<u64> {
    None
}

pub fn exit<R: BootRuntime>(code: i32) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let (switch, waiters, deferred_prepare_ipis, deferred_registry_syncs) = {
        let lock = SCHEDULER.lock();
        let ptr = lock.expect("Scheduler not initialized");
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        let (switch, waiters) = sched.terminate_current(code);
        let deferred_prepare_ipis = core::mem::take(&mut sched.pending_prepare_schedule_ipis);
        let deferred_registry_syncs = core::mem::take(&mut sched.pending_registry_syncs);
        (switch, waiters, deferred_prepare_ipis, deferred_registry_syncs)
    };

    send_deferred_prepare_schedule_ipis::<R>(deferred_prepare_ipis);
    apply_deferred_registry_syncs::<R>(deferred_registry_syncs);
    wake_waiters(&waiters);

    unsafe {
        rt.tasking().activate_address_space(switch.to_aspace);
    }

    unsafe {
        rt.tasking().switch_with_tls(
            &mut *(switch.from_ctx as *mut _),
            &*switch.to_ctx,
            switch.to_tid,
            switch.from_user_fs_base,
            switch.to_user_fs_base,
        );
    }

    unreachable!("Thread continued after terminating!");
}

/// Kill an arbitrary task by TID. Returns true if the task was found and killed.
/// The task is marked Dead with exit code -9 and removed from all run queues.
pub fn kill_by_tid<R: BootRuntime>(tid: u64) -> bool {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let (killed, waiters) = {
        let lock = SCHEDULER.lock();
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };

            // Don't allow killing the current task via this path
            let cpu_idx = current_cpu_index::<R>();
            if let Some(current_id) = sched.state.per_cpu.get(cpu_idx).and_then(|pc| pc.current) {
                if current_id == tid {
                    (false, alloc::vec::Vec::new())
                } else {
                    let task_killed = crate::task::registry::get_task::<R>(tid)
                        .map(|task| task.state != TaskState::Dead)
                        .unwrap_or(false);

                    if !task_killed {
                        (false, alloc::vec::Vec::new())
                    } else {
                        let waiters = mark_task_exited::<R>(sched, tid, -9);

                        purge_task_from_scheduler_queues::<R>(sched, tid);

                        // Release any claimed devices
                        let released =
                            crate::device_registry::REGISTRY.lock().release_all_for_task(tid);
                        if released > 0 {
                            crate::kinfo!("DEVICE: released {} claims for task {}", released, tid);
                        }

                        crate::kinfo!("SCHED: Killed task {} (SIGKILL)", tid);
                        (true, waiters)
                    }
                }
            } else {
                (false, alloc::vec::Vec::new())
            }
        } else {
            (false, alloc::vec::Vec::new())
        }
    };

    wake_waiters(&waiters);
    rt.irq_restore(_irq);
    killed
}

/// Wait semantics are non-consuming today: any task that can name a TID may
/// observe its terminal status, and dead task records stay resident for later polls/waits.
pub fn wait_task<R: BootRuntime>(tid: TaskId) -> Result<i32, abi::errors::Errno> {
    let current_tid = current_tid::<R>();

    if tid == current_tid {
        return Err(abi::errors::Errno::EINVAL);
    }

    loop {
        if let Some(code) = register_task_exit_waiter::<R>(tid, current_tid)? {
            return Ok(code);
        }

        unsafe {
            block_current_erased();
        }
    }
}

/// Collect TIDs of all tasks that are children of `our_pid`.
///
/// If `target_pid > 0`, only the specific child with that PID is returned.
/// Otherwise, all direct children are returned.
fn collect_child_tids<R: BootRuntime>(our_pid: u32, target_pid: i64) -> alloc::vec::Vec<TaskId> {
    let reg = crate::task::registry::get_registry::<R>();
    reg.threads
        .iter()
        .filter_map(|task| {
            task.process_info.as_ref().and_then(|pi| {
                let pi = pi.lock();
                if pi.runtime_parent_pid() != our_pid {
                    return None;
                }
                if target_pid > 0 && pi.runtime_pid() != target_pid as u32 {
                    return None;
                }
                Some(task.id)
            })
        })
        .collect()
}

fn queued_status_matches(flags: u32, status: i32) -> bool {
    use abi::signal::{wifcontinued, wifstopped};
    use abi::types::waitpid_flags;

    if wifstopped(status) {
        return (flags & waitpid_flags::WUNTRACED) != 0;
    }
    if wifcontinued(status) {
        return (flags & waitpid_flags::WCONTINUED) != 0;
    }
    true
}

fn reap_child_pid_if_dead<R: BootRuntime>(child_pid: u32, status: i32) {
    if abi::signal::wifstopped(status) || abi::signal::wifcontinued(status) {
        return;
    }

    let reg = crate::task::registry::get_registry::<R>();
    let child_tid = reg.threads.iter().find_map(|task| {
        task.process_info
            .as_ref()
            .filter(|pi| pi.lock().runtime_pid() == child_pid)
            .map(|_| task.id)
    });
    drop(reg);

    if let Some(child_tid) = child_tid {
        remove_task_completely::<R>(child_tid);
    }
}

fn take_queued_child_status<R: BootRuntime>(
    _our_pid: u32,
    target_pid: i64,
    flags: u32,
) -> Option<(u64, i32)> {
    let our_process = crate::sched::process_info_current()?;
    let mut process = our_process.lock();
    let mut match_index: Option<usize> = None;

    for (idx, (child_pid, status)) in process.job.children_done.iter().enumerate() {
        if target_pid > 0 && *child_pid != target_pid as u32 {
            continue;
        }
        if queued_status_matches(flags, *status) {
            match_index = Some(idx);
            break;
        }
    }

    let (child_pid, status) = process.job.children_done.remove(match_index?)?;
    drop(process);
    reap_child_pid_if_dead::<R>(child_pid, status);
    Some((child_pid as u64, status))
}

/// Internal implementation: performs the wait with an explicit `our_pid`.
///
/// Factored out so tests can exercise the core logic without registering a
/// task at the current-TID slot (which is always 0 in the `MockRuntime`).
fn waitpid_for_pid<R: BootRuntime>(
    our_pid: u32,
    pid: i64,
    flags: u32,
) -> Result<(u64, i32), abi::errors::Errno> {
    use abi::types::waitpid_flags;
    let wnohang = (flags & waitpid_flags::WNOHANG) != 0;

    let our_tid = current_tid::<R>();

    loop {
        if let Some(result) = take_queued_child_status::<R>(our_pid, pid, flags) {
            crate::ktrace!(
                "waitpid: queued status delivered parent_pid={} parent_tid={} target_pid={} child_pid={} status=0x{:x} flags=0x{:x}",
                our_pid,
                our_tid,
                pid,
                result.0,
                result.1 as u32,
                flags
            );
            return Ok(result);
        }

        // Collect matching children (releases registry guard before returning).
        let children = collect_child_tids::<R>(our_pid, pid);

        if children.is_empty() {
            return Err(abi::errors::Errno::ECHILD);
        }

        // Fast path: look for a dead child without registering.
        for &child_tid in &children {
            // Collect exit info without holding the registry guard across the reap call.
            let dead_info = crate::task::registry::get_task::<R>(child_tid).and_then(|task| {
                // `task` (ThreadRef / registry guard) is dropped when this closure returns.
                if task.state == TaskState::Dead {
                    let (child_pid, code) = task
                        .process_info
                        .as_ref()
                        .map(|pinfo| {
                            let pi_guard = pinfo.lock();
                            let code =
                                pi_guard.effective_exit_code_for_tid(child_tid, task.exit_code);
                            (pi_guard.runtime_pid() as u64, code.unwrap_or(0))
                        })
                        .unwrap_or((child_tid, task.exit_code.unwrap_or(0)));
                    Some((child_pid, code))
                } else {
                    None
                }
            });

            if let Some((child_pid, code)) = dead_info {
                // Reap: remove the dead child's record from both the registry and
                // the scheduler state so they stay in sync.
                remove_task_completely::<R>(child_tid);
                return Ok((child_pid, code));
            }
        }

        if wnohang {
            // POSIX: return 0 as the child PID to indicate "no child exited yet".
            return Ok((0, 0));
        }

        // Register as an exit waiter for every live child.  If any child has
        // already died between the check above and the register call,
        // `register_task_exit_waiter` returns `Some(code)` immediately.
        let mut registered: alloc::vec::Vec<TaskId> = alloc::vec::Vec::new();
        let mut early_result: Option<(u64, i32)> = None;
        let mut early_reap_tid: Option<TaskId> = None;

        for &child_tid in &children {
            match register_task_exit_waiter::<R>(child_tid, our_tid) {
                Ok(Some(code)) => {
                    // Child died between our fast-path check and now.
                    let child_pid = crate::task::registry::get_task::<R>(child_tid)
                        .and_then(|t| {
                            t.process_info.as_ref().map(|pi| pi.lock().runtime_pid() as u64)
                        })
                        .unwrap_or(child_tid);
                    early_result = Some((child_pid, code));
                    early_reap_tid = Some(child_tid);
                    break;
                }
                Ok(None) => registered.push(child_tid),
                Err(_) => {} // Child vanished — skip it.
            }
        }

        if let Some(result) = early_result {
            // Clean up any waiters we already registered before finding the dead child.
            for &child_tid in &registered {
                let _ = unregister_task_exit_waiter::<R>(child_tid, our_tid);
            }
            // Reap the dead child.
            if let Some(reap_tid) = early_reap_tid {
                remove_task_completely::<R>(reap_tid);
            }
            return Ok(result);
        }

        if registered.is_empty() {
            // All children died in the window between collection and registration.
            crate::ktrace!(
                "waitpid: no registrations parent_pid={} parent_tid={} target_pid={} flags=0x{:x}; retrying",
                our_pid,
                our_tid,
                pid,
                flags
            );
            continue;
        }

        crate::ktrace!(
            "waitpid: blocking parent_pid={} parent_tid={} target_pid={} flags=0x{:x} registered_children={}",
            our_pid,
            our_tid,
            pid,
            flags,
            registered.len()
        );

        // Block until any registered child exits.
        unsafe {
            block_current_erased();
        }

        crate::ktrace!(
            "waitpid: woke parent_pid={} parent_tid={} target_pid={} flags=0x{:x}",
            our_pid,
            our_tid,
            pid,
            flags
        );

        // After waking, unregister from children that haven't yet exited.
        for &child_tid in &registered {
            let _ = unregister_task_exit_waiter::<R>(child_tid, our_tid);
        }

        // Loop back to find the next queued child state transition.
    }
}

/// Wait for a child process to exit, returning `(child_pid, exit_code)`.
///
/// - `pid > 0`: wait for the specific child with that PID.
/// - `pid == -1` or `pid == 0`: wait for any child of the calling process.
/// - `flags & WNOHANG`: return `Ok((0, 0))` immediately if no child has exited.
///
/// Returns `Err(ECHILD)` when no matching children exist at all.
pub fn waitpid<R: BootRuntime>(pid: i64, flags: u32) -> Result<(u64, i32), abi::errors::Errno> {
    let our_tid = current_tid::<R>();

    // Retrieve the calling process's PID from its ProcessInfo.
    let our_pid = {
        let task =
            crate::task::registry::get_task::<R>(our_tid).ok_or(abi::errors::Errno::EINVAL)?;
        task.process_info
            .as_ref()
            .map(|pi| pi.lock().runtime_pid())
            .ok_or(abi::errors::Errno::EINVAL)?
    };

    waitpid_for_pid::<R>(our_pid, pid, flags)
}

// ── Signal mask hooks ─────────────────────────────────────────────────────────

fn get_signal_mask<R: BootRuntime>() -> abi::signal::SigSet {
    let tid = current_tid::<R>();
    crate::task::registry::get_task::<R>(tid)
        .map(|t| t.signals.effective_mask())
        .unwrap_or(abi::signal::SigSet::EMPTY)
}

fn set_signal_mask<R: BootRuntime>(mask: abi::signal::SigSet) {
    let tid = current_tid::<R>();
    if let Some(mut t) = crate::task::registry::get_task_mut::<R>(tid) {
        t.signals.mask = abi::signal::SigSet(mask.0 & !crate::signal::UNCATCHABLE.0);
    }
}

fn get_thread_pending<R: BootRuntime>() -> abi::signal::SigSet {
    let tid = current_tid::<R>();
    crate::task::registry::get_task::<R>(tid)
        .map(|t| t.signals.pending)
        .unwrap_or(abi::signal::SigSet::EMPTY)
}

fn set_thread_pending<R: BootRuntime>(pending: abi::signal::SigSet) {
    let tid = current_tid::<R>();
    if let Some(mut t) = crate::task::registry::get_task_mut::<R>(tid) {
        t.signals.pending = pending;
    }
}

pub fn register_timeout_wake<R: BootRuntime>(tid: TaskId, wake_tick: u64) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        sched.state.add_task_to_sleep_queue(tid, wake_tick);
    }
    rt.irq_restore(_irq);
}

pub fn unregister_timeout_wake<R: BootRuntime>(tid: TaskId) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        let _ = sched.state.remove_task_from_sleep_queue(tid);
    }
    rt.irq_restore(_irq);
}

pub fn cpu_online<R: BootRuntime>(cpu_index: usize) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        sched.cpu_online(cpu_index);
    }
    rt.irq_restore(_irq);
}

/// Remove a task completely from both the global registry and the scheduler state.
///
/// This is the canonical way to "reap" a task.  It ensures that the thread
/// list in the registry stays in sync with the scheduler's sorted `threads`
/// vector, preventing index drift that would otherwise lead to panics in the
/// context switcher.
pub fn remove_task_completely<R: BootRuntime>(tid: TaskId) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    // 1. Remove from scheduler state (requires SCHEDULER lock)
    {
        let lock = SCHEDULER.lock();
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
            purge_task_from_scheduler_queues::<R>(sched, tid);
            sched.state.remove_task(tid);
        }
    }

    // 2. Remove from global registry (requires REGISTRY lock)
    crate::task::registry::get_registry::<R>().remove(tid);

    rt.irq_restore(_irq);
}

fn format_optional_cpu(cpu: Option<usize>) -> alloc::string::String {
    cpu.map_or_else(|| alloc::string::String::from("-"), |c| alloc::format!("{}", c))
}

/// Build `(last_cpu, wake_cpu, run_cpu)` diagnostic strings for dump output.
///
/// `last_cpu` prefers scheduler hot-trace state when present and falls back to
/// the task-local `last_cpu` mirror. `wake_cpu` and `run_cpu` come from
/// scheduler trace fields.
fn task_cpu_trace_strings(
    task_last_cpu: Option<usize>,
    sched_fields: Option<&crate::sched::state::ThreadSchedFields>,
) -> (alloc::string::String, alloc::string::String, alloc::string::String) {
    let last_cpu = sched_fields.and_then(|sf| sf.last_cpu).or(task_last_cpu);
    let wake_cpu = sched_fields.and_then(|sf| sf.wake_cpu);
    let run_cpu = sched_fields.and_then(|sf| sf.run_cpu);
    (format_optional_cpu(last_cpu), format_optional_cpu(wake_cpu), format_optional_cpu(run_cpu))
}

pub fn dump_stats<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };

    crate::kprint!("\n====== TASK DUMP ======\n");
    crate::kprint!(
        "CPUs: {} online / {} total\n",
        sched.state.online_cpu_count,
        sched.total_cpu_count
    );
    // Columns: TID, STATE, PRI, LAST, WAKE, RUN, USER, SLICE, KSTK, AFFIN, NAME.
    crate::kprint!(
        " {:>5}  {:>10}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>7}  {:>6}  {:>6}  {}\n",
        "TID",
        "STATE",
        "PRI",
        "LAST",
        "WAKE",
        "RUN",
        "USER",
        "SLICE",
        "KSTK",
        "AFFIN",
        "NAME"
    );

    let mut runnable_count = 0u32;
    for task in crate::task::registry::get_registry::<R>().threads.iter() {
        let state_str = match task.state {
            TaskState::Runnable => {
                runnable_count += 1;
                "Runnable"
            }
            TaskState::Running => {
                runnable_count += 1;
                "Running"
            }
            TaskState::Blocked => "Blocked",
            TaskState::Dead => "Dead",
        };
        let pri_str = match task.priority {
            crate::task::TaskPriority::Idle => "Idle",
            crate::task::TaskPriority::Low => "Low",
            crate::task::TaskPriority::Normal => "Norm",
            crate::task::TaskPriority::High => "High",
            crate::task::TaskPriority::Realtime => "RT",
        };
        let (cpu_str, wake_cpu_str, run_cpu_str) =
            task_cpu_trace_strings(task.last_cpu, sched.state.get_task(task.id));
        let user_str = if task.is_user { "Y" } else { "N" };
        let aff_str: alloc::string::String = match task.affinity {
            crate::task::Affinity::Any => alloc::string::String::from("Any"),
            crate::task::Affinity::Pinned(c) => alloc::format!("Pin({})", c),
        };
        let name_str = if task.name_len > 0 {
            core::str::from_utf8(&task.name[..task.name_len as usize]).unwrap_or("?")
        } else {
            "-"
        };
        crate::kprint!(
            " {:>5}  {:>10}  {:>4}  {:>4}  {:>4}  {:>4}  {:>4}  {:>3}/{:<3}  {:>5}K  {:>6}  {}\n",
            task.id,
            state_str,
            pri_str,
            cpu_str,
            wake_cpu_str,
            run_cpu_str,
            user_str,
            task.timeslice_remaining,
            types::DEFAULT_TIMESLICE,
            task.kstack_size / 1024,
            aff_str,
            name_str
        );
    }

    // Per-CPU run-queue summary
    for &i in &sched.state.online_cpus {
        let pc = &sched.state.per_cpu[i];
        let total: usize = pc.runq.iter().map(|q| q.len()).sum();
        let sample_count = pc.stats.runq_sample_count;
        let sample_total = pc.stats.runq_sample_total;
        let avg_runq = if sample_count == 0 { 0 } else { sample_total / sample_count };
        crate::kprint!(
            "  CPU {}: current={:?} runq={} avg_runq={} idle={:?} ctxsw={} idle->busy={} tick={} ipi_rx={} enq={} deq={} wake={} lock_miss={} lock_miss_pending={} lock_blocked={}\n",
            i,
            pc.current,
            total,
            avg_runq,
            pc.idle_task,
            pc.stats.context_switches,
            pc.stats.idle_to_nonidle,
            pc.stats.timer_interrupts,
            pc.stats.resched_ipi_received,
            pc.stats.runnable_enqueues,
            pc.stats.runnable_dequeues,
            pc.stats.wakeups,
            PROF_TRYLOCK_MISS_PER_CPU[i].load(Ordering::Relaxed),
            PROF_TRYLOCK_MISS_PENDING_PER_CPU[i].load(Ordering::Relaxed),
            pc.stats.lock_blocked_dispatch
        );
    }

    crate::kprint!("Sleep queue: {} tasks\n", sched.state.sleep_queue.len());
    crate::kprint!(
        "=== {} tasks, {} runnable ===\n\n",
        crate::task::registry::get_registry::<R>().threads.len(),
        runnable_count
    );

    rt.irq_restore(_irq);
}

extern "C" fn idle_task<R: BootRuntime>(_: usize) -> ! {
    let rt = crate::runtime::<R>();
    loop {
        rt.wait_for_interrupt();
    }
}

pub static CPU_ONLINE: AtomicUsize = AtomicUsize::new(0);

/// Entry point for secondary CPUs.
///
/// # Safety
/// Must only be called from `kernel_secondary_entry`.
pub unsafe fn enter_secondary(cpu_index: usize) -> ! {
    // Mark as online
    CPU_ONLINE.fetch_add(1, Ordering::Relaxed);
    crate::kdebug!("SMP: Secondary CPU {} online!", cpu_index);

    // Enter scheduler loop via the hook which bootstraps this CPU.
    // The run_scheduler hook will call bootstrap_cpu to set up this CPU's
    if let Some(hook) = unsafe { hooks::RUN_SCHEDULER_HOOK } {
        hook();
    } else {
        panic!("Scheduler hook not initialized!");
    }

    // Fallback if run_scheduler returns (it shouldn't)
    loop {
        crate::runtime_base().wait_for_interrupt();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::{Affinity, TaskPriority, TaskState};
    use crate::{
        BootRuntime, BootRuntimeBase, BootTasking, MapKind, MapPerms, UserEntry, UserTaskSpec,
    };

    // Mock types for testing - copy from spawn.rs tests
    #[derive(Default, Copy, Clone)]
    pub(crate) struct MockContext(pub(crate) usize);
    #[derive(Clone, Copy, Default)]
    pub(crate) struct MockAddressSpace(pub(crate) u64);

    pub(crate) static MOCK_RUNTIME: MockRuntime = MockRuntime;
    pub(crate) struct MockRuntime;

    // Per-thread IRQ depth counter used by MockRuntime to detect imbalanced
    // irq_disable / irq_restore pairs. irq_disable increments the depth and
    // returns the old value; irq_restore restores the depth to the saved value.
    std::thread_local! {
        static IRQ_DEPTH: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    }

    /// Returns the current mock IRQ depth for the calling test thread.
    /// A value of 0 means interrupts are conceptually enabled (balanced state).
    pub(crate) fn mock_irq_depth() -> usize {
        IRQ_DEPTH.with(|c| c.get())
    }

    /// Resets the mock IRQ depth to 0 (call at the start of each test that
    /// checks IRQ balance to ensure a clean baseline).
    pub(crate) fn reset_mock_irq_depth() {
        IRQ_DEPTH.with(|c| c.set(0));
    }

    impl BootRuntimeBase for MockRuntime {
        fn putchar(&self, _c: u8) {}
        fn mono_ticks(&self) -> u64 {
            0
        }
        fn mono_freq_hz(&self) -> u64 {
            1
        }
        fn init_secondary_cpu(&self, _cpu_index: usize) {}
        fn phys_to_virt_offset(&self) -> u64 {
            0
        }
    }
    impl BootRuntime for MockRuntime {
        type Tasking = MockRuntime;
        fn tasking(&self) -> &Self {
            self
        }
        fn halt(&self) -> ! {
            loop {}
        }
        fn irq_disable(&self) -> crate::IrqState {
            let prev = IRQ_DEPTH.with(|c| {
                let d = c.get();
                c.set(d + 1);
                d
            });
            crate::IrqState(prev)
        }
        fn irq_restore(&self, state: crate::IrqState) {
            IRQ_DEPTH.with(|c| c.set(state.0));
        }
        fn phys_memory_map(&self) -> &'static [crate::PhysRange] {
            &[]
        }
        fn modules(&self) -> &'static [crate::BootModuleDesc] {
            &[]
        }
        fn framebuffer(&self) -> Option<crate::FramebufferInfo> {
            None
        }
        fn simd_state_layout(&self) -> (usize, usize) {
            (0, 1)
        }
        unsafe fn simd_save(&self, _ptr: *mut u8) {}
        unsafe fn simd_restore(&self, _ptr: *const u8) {}
    }
    impl BootTasking for MockRuntime {
        type Runtime = MockRuntime;
        type Context = MockContext;
        type AddressSpace = MockAddressSpace;
        fn init(&self, _hhdm: u64) {}
        fn init_kernel_context(
            &self,
            _entry: extern "C" fn(usize) -> !,
            _st: u64,
            _arg: usize,
        ) -> Self::Context {
            MockContext(_arg)
        }
        fn init_user_context(
            &self,
            _spec: UserTaskSpec<Self::AddressSpace>,
            _kst: u64,
        ) -> Self::Context {
            MockContext(_spec.arg)
        }
        unsafe fn switch(&self, _f: &mut Self::Context, _t: &Self::Context, _tid: u64) {}
        unsafe fn enter_user(&self, _e: UserEntry) -> ! {
            loop {}
        }
        fn make_user_address_space(&self) -> Self::AddressSpace {
            MockAddressSpace(0)
        }
        fn active_address_space(&self) -> Self::AddressSpace {
            MockAddressSpace(0)
        }
        fn activate_address_space(&self, _as: Self::AddressSpace) {}
        fn map_page(
            &self,
            _as: Self::AddressSpace,
            _v: u64,
            _p: u64,
            _pr: MapPerms,
            _k: MapKind,
            _a: &dyn crate::FrameAllocatorHook,
        ) -> Result<(), ()> {
            Ok(())
        }
        fn unmap_page(&self, _as: Self::AddressSpace, _v: u64) -> Result<Option<u64>, ()> {
            Ok(None)
        }
        fn protect_page(
            &self,
            _as: Self::AddressSpace,
            _virt: u64,
            _perms: MapPerms,
        ) -> Result<(), ()> {
            Ok(())
        }
        fn translate(&self, _as: Self::AddressSpace, _v: u64) -> Option<u64> {
            None
        }
        fn tlb_flush_page(&self, _v: u64) {}
    }

    static INIT_TESTS: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
    fn init_mock_runtime() {
        if !INIT_TESTS.swap(true, core::sync::atomic::Ordering::SeqCst) {
            crate::init_runtime(&MOCK_RUNTIME);
        }
    }

    #[test]
    fn effective_parallelism_any_uses_online_cpu_count() {
        assert_eq!(effective_parallelism_from_state(1, Affinity::Any), 1);
        assert_eq!(effective_parallelism_from_state(4, Affinity::Any), 4);
    }

    #[test]
    fn effective_parallelism_pinned_is_single_cpu() {
        assert_eq!(effective_parallelism_from_state(1, Affinity::Pinned(0)), 1);
        assert_eq!(effective_parallelism_from_state(8, Affinity::Pinned(3)), 1);
    }

    #[test]
    fn effective_parallelism_never_returns_zero() {
        assert_eq!(effective_parallelism_from_state(0, Affinity::Any), 1);
    }

    /// Serialises sched tests that mutate shared globals (REGISTRY, SCHEDULER,
    /// TICK_COUNT).  Any test that calls `init_test_env` should hold the
    /// returned guard for its entire duration to prevent races with concurrent
    /// tests that reinitialise the registry.
    pub(crate) static SCHED_TEST_GUARD: spin::Mutex<()> = spin::Mutex::new(());

    pub(crate) fn init_test_env() -> spin::MutexGuard<'static, ()> {
        let guard = SCHED_TEST_GUARD.lock();
        init_mock_runtime();
        crate::task::registry::init::<MockRuntime>();
        *SCHEDULER.lock() = None;
        TICK_COUNT.store(0, core::sync::atomic::Ordering::Relaxed);
        reset_any_wake_policy_for_tests();
        guard
    }

    fn make_task(
        id: TaskId,
        state: TaskState,
        priority: TaskPriority,
    ) -> crate::task::Task<MockRuntime> {
        crate::task::Task {
            id,
            state,
            priority,
            base_priority: priority,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        }
    }

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
        });
        sched.state.enqueue_task(0, TaskPriority::Low as usize, 42);

        sched.set_priority(42, TaskPriority::High);

        assert_eq!(sched.state.get_task(42).unwrap().priority, TaskPriority::High);
        assert_eq!(
            crate::task::registry::get_registry::<MockRuntime>().threads[0].priority,
            TaskPriority::High
        );
        assert!(sched.state.per_cpu[0].runq[TaskPriority::Low as usize].is_empty());
        assert_eq!(
            sched.state.per_cpu[0].runq[TaskPriority::High as usize].front().copied(),
            Some(42)
        );
    }

    #[test]
    fn test_priority_aging_boost() {
        let _g = init_test_env();
        // Test that tasks waiting too long get priority boost when scheduling
        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(0); // Dummy current task

        // The dummy current task must be in BOTH the global registry and the
        // scheduler's local sched-state so that `do_context_switch` can index
        // into both consistently (both vectors are sorted by TID).
        let dummy_current = make_task(0, TaskState::Running, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(dummy_current));
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
        });
        let task_normal = crate::task::Task {
            id: 1001,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 600, // Recent (now - 600 = 400 < AGING_THRESHOLD, so no boost)
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        // Create a low-priority task enqueued a long time ago
        let task_low = crate::task::Task {
            id: 1002,
            state: TaskState::Runnable,
            priority: TaskPriority::Low,
            base_priority: TaskPriority::Low,
            enqueued_at_tick: 0, // Very old
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(task_normal));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(task_low));

        // Scheduler state entries are separate from the global registry and must
        // be inserted explicitly so `prepare_schedule` can locate them.
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 1001,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(0),
            wake_cpu: Some(0),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            // Must match `task_normal.enqueued_at_tick` above (600) so the
            // hot-field cache reflects the correct wait time for aging.
            enqueued_at_tick: 600,
            wake_pending: false,
        });
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 1002,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Low,
            affinity: Affinity::Any,
            last_cpu: Some(0),
            wake_cpu: Some(0),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            // Must match `task_low.enqueued_at_tick` above (0) so the
            // hot-field cache reflects the correct wait time for aging.
            enqueued_at_tick: 0,
            wake_pending: false,
        });
        sched.state.enqueue_task(0, TaskPriority::Normal as usize, 1001);
        sched.state.enqueue_task(0, TaskPriority::Low as usize, 1002);

        // Simulate time advancing enough to give the Low task a boost of +2 (eff = High=3),
        // while the Normal task, enqueued at tick 600, only waits 400 ticks → no boost (eff = Normal=2).
        let now = types::AGING_THRESHOLD_TICKS * 2;
        TICK_COUNT.store(now, core::sync::atomic::Ordering::Relaxed);

        // Request schedule. The Low task should be selected because its effective priority is higher
        // than Normal due to wait time.
        let next_switch = sched.prepare_schedule().expect("Should find a task");
        assert_eq!(
            next_switch.to_tid, 1002,
            "Low priority task with aging should preempt normal task"
        );

        // Verify it was popped from the Low queue, not moved to High queue
        assert!(sched.state.per_cpu[0].runq[TaskPriority::Low as usize].is_empty());
        assert!(!sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].is_empty());
    }

    #[test]
    fn test_reset_priority_aging_on_schedule() {
        let _g = init_test_env();
        // Test that enqueued_at_tick resets when task is preempted/yields
        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

        // Task 1 is running
        let mut task1 = crate::task::Task {
            id: 2001,
            state: TaskState::Running,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0, // Very old
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: 0, // timeslice expired
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        // Task 2 is runnable
        let task2 = crate::task::Task {
            id: 2002,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 500, // Newer
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task1));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task2));

        sched.state.per_cpu[0].current = Some(2001);
        // Scheduler state entries for both tasks.
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 2001,
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
        });
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 2002,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(0),
            wake_cpu: Some(0),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 500,
            wake_pending: false,
        });
        sched.state.enqueue_task(0, TaskPriority::Normal as usize, 2002);

        // Time moves forward
        TICK_COUNT.store(1000, core::sync::atomic::Ordering::Relaxed);

        // Trigger a timer tick to cause preemption
        let switch = sched.prepare_yield().expect("Should preempt to task2");
        assert_eq!(switch.to_tid, 2002);
        let t1_before_sync = crate::task::registry::get_task::<MockRuntime>(2001).unwrap();
        assert_eq!(
            t1_before_sync.enqueued_at_tick, 0,
            "prepare_yield should defer REGISTRY sync out of prepare_schedule hot path"
        );
        assert_eq!(
            t1_before_sync.state,
            TaskState::Running,
            "REGISTRY state should remain unchanged until deferred sync is applied"
        );
        apply_deferred_registry_syncs::<MockRuntime>(core::mem::take(
            &mut sched.pending_registry_syncs,
        ));

        // Verify task1 was placed back in runq and its enqueued_at_tick was updated to TICK_COUNT
        let t1 = crate::task::registry::get_task::<MockRuntime>(2001).unwrap();
        assert_eq!(t1.enqueued_at_tick, 1000);
        assert_eq!(t1.state, TaskState::Runnable);
    }

    #[test]
    fn test_wake_preempts_lower_priority() {
        let _g = init_test_env();
        crate::task::registry::init::<MockRuntime>();
        use core::sync::atomic::Ordering;

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

        // Task 1: Normal priority, currently running
        let normal_task = crate::task::Task {
            id: 3001,
            state: TaskState::Running,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        // Task 2: Realtime priority, sleeping (about to wake)
        let rt_task = crate::task::Task {
            id: 3002,
            state: TaskState::Runnable,
            priority: TaskPriority::Realtime,
            base_priority: TaskPriority::Realtime,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(normal_task));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(rt_task));
        sched.state.per_cpu[0].current = Some(3001); // Normal task is running
        // Scheduler state entries for both tasks.
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 3001,
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
        });
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 3002,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Realtime,
            affinity: Affinity::Any,
            last_cpu: Some(0),
            wake_cpu: Some(0),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
        });

        // Put RT task in sleep queue with wake_tick in the past
        TICK_COUNT.store(100, Ordering::Relaxed);
        sched.state.add_task_to_sleep_queue(3002, 50);

        // Before: need_resched should be false
        assert!(!sched.state.per_cpu[0].need_resched, "need_resched should start false");

        // Wake sleepers — should detect RT > Normal and set need_resched
        sched.wake_sleepers();

        // Verify need_resched was set
        assert!(
            sched.state.per_cpu[0].need_resched,
            "need_resched should be true after waking a higher-priority task"
        );

        // Verify RT task was enqueued to the Realtime runq
        assert!(
            sched.state.per_cpu[0].runq[TaskPriority::Realtime as usize]
                .iter()
                .any(|&id| id == 3002),
            "RT task should be in the Realtime run queue"
        );

        // Now simulate schedule: prepare_yield should pick the RT task
        sched.state.per_cpu[0].need_resched = false; // clear so prepare_yield runs clean
        let switch = sched.prepare_yield();
        assert!(switch.is_some(), "Should produce a context switch");
        let switch = switch.unwrap();
        assert_eq!(switch.to_tid, 3002, "Scheduler should switch to the RT task");
        assert_eq!(switch.from_tid, 3001, "Scheduler should switch away from the Normal task");
    }

    /// Verify that `wake_sleepers` defers cross-CPU IPIs to `pending_wake_ipis`
    /// instead of calling `send_ipi` while the SCHEDULER lock is held.
    ///
    /// The fix for issue #131 changed `wake_sleepers` to push target CPUs onto
    /// `self.pending_wake_ipis` rather than invoking `send_ipi` directly. This
    /// test checks that, after a cross-CPU sleeper is woken:
    ///   1. The IPI target is present in `pending_wake_ipis`.
    ///   2. `pending_wake_ipis` is empty once the caller drains it.
    #[test]
    fn test_wake_sleepers_defers_ipi_to_pending_wake_ipis() {
        let _g = init_test_env();
        use core::sync::atomic::Ordering;

        let mut sched = types::Scheduler::<MockRuntime>::new();
        // Add two CPUs: CPU 0 (running this test) and CPU 1 (the IPI target).
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        // Mark CPU 1 as online so it appears in the online_cpus list.
        sched.state.mark_cpu_online(1);

        // Task running on CPU 0 (the "current" task for MockRuntime).
        let current_task = make_task(9001, TaskState::Running, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(current_task));
        sched.state.per_cpu[0].current = Some(9001);
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 9001,
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
        });

        // Sleeping task pinned to CPU 1 (a different CPU from current_cpu_index == 0).
        let sleeping_task = make_task(9002, TaskState::Blocked, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(sleeping_task));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 9002,
            runq_location: None,
            state: TaskState::Blocked,
            priority: TaskPriority::Normal,
            affinity: Affinity::Pinned(1),
            last_cpu: Some(1),
            wake_cpu: Some(1),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
        });

        // Put the sleeping task in the sleep queue with a wake_tick in the past.
        TICK_COUNT.store(100, Ordering::Relaxed);
        sched.state.add_task_to_sleep_queue(9002, 50);

        // `pending_wake_ipis` should be empty before wake_sleepers runs.
        assert!(
            sched.pending_wake_ipis.is_empty(),
            "pending_wake_ipis should be empty before wake_sleepers"
        );

        sched.wake_sleepers();

        // After wake_sleepers, the IPI for CPU 1 must be in pending_wake_ipis,
        // not already sent (send_ipi is a no-op in MockRuntime).
        assert_eq!(
            sched.pending_wake_ipis,
            alloc::vec![1usize],
            "wake_sleepers should defer cross-CPU IPI to pending_wake_ipis (got {:?})",
            sched.pending_wake_ipis
        );

        // Simulate the caller draining pending_wake_ipis after releasing the lock.
        let drained = core::mem::take(&mut sched.pending_wake_ipis);
        assert_eq!(drained, alloc::vec![1usize]);
        assert!(
            sched.pending_wake_ipis.is_empty(),
            "pending_wake_ipis should be empty after drain"
        );
    }

    #[test]
    fn test_wake_sleepers_enforces_budget_and_carries_remainder() {
        let _g = init_test_env();
        use core::sync::atomic::Ordering;

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

        let total_sleepers = types::WAKE_SLEEPERS_BUDGET_PER_TICK + 5;
        for tid in 10_000..(10_000 + total_sleepers as u64) {
            let sleeping_task = make_task(tid, TaskState::Blocked, TaskPriority::Normal);
            crate::task::registry::get_registry::<MockRuntime>()
                .insert(alloc::boxed::Box::new(sleeping_task));
            sched.state.insert_task(crate::sched::state::ThreadSchedFields {
                tid,
                runq_location: None,
                state: TaskState::Blocked,
                priority: TaskPriority::Normal,
                affinity: Affinity::Pinned(0),
                last_cpu: Some(0),
                wake_cpu: Some(0),
                run_cpu: None,
                timeslice_remaining: types::DEFAULT_TIMESLICE,
                enqueued_at_tick: 0,
                wake_pending: false,
            });
            sched.state.add_task_to_sleep_queue(tid, 50);
        }
        TICK_COUNT.store(100, Ordering::Relaxed);

        sched.wake_sleepers();
        assert_eq!(
            sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].len(),
            types::WAKE_SLEEPERS_BUDGET_PER_TICK,
            "wake_sleepers should honor per-tick wake budget"
        );
        assert_eq!(
            sched.state.sleep_queue.get(&50).map(|v| v.len()),
            Some(5),
            "sleep queue should retain remaining sleepers after budget is exhausted"
        );
        assert_eq!(
            sched.wake_sleepers_budget_carry, 0,
            "budget carry should be empty after fully consuming available budget"
        );

        sched.wake_sleepers();
        assert_eq!(
            sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].len(),
            total_sleepers,
            "second wake pass should process remaining sleepers"
        );
        assert!(
            !sched.state.sleep_queue.contains_key(&50),
            "sleep queue entry should be removed once all sleepers wake"
        );
        assert_eq!(
            sched.wake_sleepers_budget_carry,
            types::WAKE_SLEEPERS_BUDGET_PER_TICK - 5,
            "unused budget should carry forward to later ticks"
        );
    }

    #[test]
    fn test_prepare_schedule_defers_misroute_ipi_to_pending_prepare_queue() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.mark_cpu_online(1);

        // Current task on CPU 0.
        let current_task = make_task(9101, TaskState::Running, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(current_task));
        sched.state.per_cpu[0].current = Some(9101);
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 9101,
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
        });

        // Runnable task incorrectly queued on CPU 0 but pinned to CPU 1.
        let misrouted = make_task(9102, TaskState::Runnable, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(misrouted));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 9102,
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
        });
        sched.state.enqueue_task(0, TaskPriority::Normal as usize, 9102);

        // No local idle task exists, so prepare_schedule returns None after
        // requeueing the misrouted task and requesting a remote nudge.
        let switch = sched.prepare_schedule();
        assert!(
            switch.is_none(),
            "prepare_schedule should return None when only misrouted work exists and no idle task"
        );
        assert_eq!(
            sched.pending_prepare_schedule_ipis,
            alloc::vec![1usize],
            "prepare_schedule should defer misroute nudge to pending_prepare_schedule_ipis"
        );
        assert!(
            sched.pending_wake_ipis.is_empty(),
            "misroute IPIs should not be mixed into pending_wake_ipis"
        );
        assert!(
            sched.state.per_cpu[1].runq[TaskPriority::Normal as usize]
                .iter()
                .any(|&tid| tid == 9102),
            "misrouted task should be moved to target CPU runq"
        );
    }

    #[test]
    fn test_prepare_schedule_bounds_misroute_repair_work_per_call() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.mark_cpu_online(1);

        // Current task on CPU 0.
        let current_task = make_task(9200, TaskState::Running, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(current_task));
        sched.state.per_cpu[0].current = Some(9200);
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 9200,
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
        });

        // Fill CPU 0's normal queue with misrouted tasks pinned to CPU 1.
        let misrouted_total = PREPARE_SCHEDULE_PICK_BUDGET + 4;
        for i in 0..misrouted_total {
            let tid = 9201 + i as u64;
            let task = make_task(tid, TaskState::Runnable, TaskPriority::Normal);
            crate::task::registry::get_registry::<MockRuntime>()
                .insert(alloc::boxed::Box::new(task));
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
            });
            sched.state.enqueue_task(0, TaskPriority::Normal as usize, tid);
        }

        let switch = sched.prepare_schedule();
        assert!(switch.is_none());
        assert!(
            !sched.pending_misrouted_requeues.is_empty(),
            "misroute repairs should be deferred when work exceeds per-call repair budget"
        );
        assert!(
            sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].len()
                <= PREPARE_SCHEDULE_MISROUTE_REPAIR_BUDGET,
            "prepare_schedule should only repair up to PREPARE_SCHEDULE_MISROUTE_REPAIR_BUDGET misroutes per call"
        );

        // Repeated calls should drain deferred work.
        for _ in 0..4 {
            let _ = sched.prepare_schedule();
        }
        assert!(
            sched.pending_misrouted_requeues.is_empty(),
            "deferred misroute backlog should drain across subsequent prepare_schedule calls"
        );
    }

    #[test]
    fn test_send_deferred_prepare_schedule_ipis_updates_counters() {
        let _g = init_test_env();
        use core::sync::atomic::Ordering;

        DIAG_IPI_SENT.store(0, Ordering::Relaxed);
        DIAG_IPI_SENT_PREPARE_SCHEDULE.store(0, Ordering::Relaxed);

        send_deferred_prepare_schedule_ipis::<MockRuntime>(alloc::vec![1usize, 2usize]);

        assert_eq!(
            DIAG_IPI_SENT.load(Ordering::Relaxed),
            2,
            "generic IPI counter should increase for each deferred prepare_schedule send"
        );
        assert_eq!(
            DIAG_IPI_SENT_PREPARE_SCHEDULE.load(Ordering::Relaxed),
            2,
            "prepare_schedule source counter should increase for each deferred send"
        );
    }

    #[test]
    fn test_sorted_insertion() {
        let _g = init_test_env();
        crate::task::registry::init::<MockRuntime>();
        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

        // Helper to create dummy task
        let make_task = |id: TaskId| crate::task::Task {
            id,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: None,
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        // Insert tasks out of order
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(make_task(4010)));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(make_task(4005)));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(make_task(4020)));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(make_task(4001)));

        // Verify sorted order internally
        assert_eq!(crate::task::registry::get_registry::<MockRuntime>().threads.len(), 4);
        assert_eq!(crate::task::registry::get_registry::<MockRuntime>().threads[0].id, 4001);
        assert_eq!(crate::task::registry::get_registry::<MockRuntime>().threads[1].id, 4005);
        assert_eq!(crate::task::registry::get_registry::<MockRuntime>().threads[2].id, 4010);
        assert_eq!(crate::task::registry::get_registry::<MockRuntime>().threads[3].id, 4020);

        // Verify lookups work
        assert!(crate::task::registry::get_task::<MockRuntime>(4010).is_some());
        assert!(crate::task::registry::get_task::<MockRuntime>(4005).is_some());
        assert!(crate::task::registry::get_task::<MockRuntime>(4001).is_some());
        assert!(crate::task::registry::get_task::<MockRuntime>(4099).is_none());
    }

    #[test]
    fn test_block_and_wake_state_transitions() {
        let _g = init_test_env();
        crate::task::registry::init::<MockRuntime>();
        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

        let waiting_task = crate::task::Task {
            id: 5001,
            state: TaskState::Running,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(waiting_task));
        sched.state.per_cpu[0].current = Some(5001);

        // Put task in Wait queue and switch it to Blocked (simulating block_current behavior)
        if let Some(mut task) = crate::task::registry::get_task_mut::<MockRuntime>(5001) {
            task.state = TaskState::Blocked;
        }
        sched.state.wait_queue.push_back(5001);

        // Verify task is stuck blocked
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(5001).unwrap().state,
            TaskState::Blocked
        );

        // Emulate `wake_task_erased` via wake_task in MockRuntime context
        crate::sched::blocking::WAKE_TASK_HOOK.store(
            crate::sched::blocking::wake_task::<MockRuntime> as *mut (),
            core::sync::atomic::Ordering::SeqCst,
        );

        // Make sure scheduler hook resolves safely (we will mock inject the scheduler here via static for the hook)
        // Since we are unit testing `wake_task`, we can't easily use the global `SCHEDULER`.
        // So we just directly call the core logic we care about: the wake sleeper unblock logic.

        // Remove from wait queue if present
        if let Some(pos) = sched.state.wait_queue.iter().position(|&wid| wid == 5001) {
            sched.state.wait_queue.remove(pos);
        }

        // Update state to Runnable and add to runq
        if let Some(mut task) = crate::task::registry::get_task_mut::<MockRuntime>(5001) {
            if task.state == TaskState::Blocked {
                task.state = TaskState::Runnable;
                sched.state.enqueue_task(0, task.priority as usize, 5001);
            }
        }

        let woken_task = crate::task::registry::get_task::<MockRuntime>(5001).unwrap();
        assert_eq!(
            woken_task.state,
            TaskState::Runnable,
            "Task must transition from Blocked to Runnable upon wake"
        );

        // Verify task was placed in runq
        assert!(
            sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].iter().any(|&id| id == 5001),
            "Woken task must be in the run queue"
        );
    }

    #[test]
    fn test_wake_task_removes_sleep_queue_entry() {
        let _g = init_test_env();
        crate::task::registry::init::<MockRuntime>();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(6000);

        let current_task = crate::task::Task {
            id: 6000,
            state: TaskState::Running,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        let sleeping_task = crate::task::Task {
            id: 6001,
            state: TaskState::Blocked,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(current_task));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(sleeping_task));

        // Both tasks must be in ThreadSchedFields so that wake_task_locked can
        // find and properly wake the sleeping task via the hot-field cache.
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 6000,
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
        });
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 6001,
            runq_location: None,
            state: TaskState::Blocked,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(0),
            wake_cpu: Some(0),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
        });

        sched.state.add_task_to_sleep_queue(6001, 10);

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
        drop(sched_lock);

        crate::sched::blocking::wake_task::<MockRuntime>(6001);

        let task = crate::task::registry::get_task::<MockRuntime>(6001).unwrap();
        assert_eq!(task.state, TaskState::Runnable);
        assert!(sched.state.sleep_queue.is_empty());
        assert!(
            sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].iter().any(|&id| id == 6001)
        );

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = None;
    }

    #[test]
    fn test_wake_task_fastpath_skips_scheduler_lock_when_wake_already_pending() {
        let _g = init_test_env();
        crate::task::registry::init::<MockRuntime>();

        let task = crate::task::Task {
            id: 6101,
            state: TaskState::Running,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: true,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = None;
        drop(sched_lock);
        let _ = sched_lock_metrics_snapshot_and_reset();

        crate::sched::blocking::wake_task::<MockRuntime>(6101);

        let task = crate::task::registry::get_task::<MockRuntime>(6101).unwrap();
        assert_eq!(task.state, TaskState::Running);
        assert!(task.wake_pending);

        let metrics = sched_lock_metrics_snapshot_and_reset();
        assert_eq!(
            metrics.wake_task_fastpath_already_pending, 1,
            "wake_task should take the already-pending fast path"
        );
        assert_eq!(
            metrics.wake_task.wait_calls, 0,
            "wake_task fast path should avoid scheduler lock wait tracking"
        );
    }

    #[test]
    fn test_wake_task_without_pending_wake_uses_scheduler_path() {
        let _g = init_test_env();
        crate::task::registry::init::<MockRuntime>();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 6102,
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
        });

        let task = crate::task::Task {
            id: 6102,
            state: TaskState::Running,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
        drop(sched_lock);
        let _ = sched_lock_metrics_snapshot_and_reset();

        crate::sched::blocking::wake_task::<MockRuntime>(6102);

        let task = crate::task::registry::get_task::<MockRuntime>(6102).unwrap();
        assert!(task.wake_pending);
        let metrics = sched_lock_metrics_snapshot_and_reset();
        assert_eq!(metrics.wake_task_fastpath_already_pending, 0);
        assert_eq!(metrics.wake_task.wait_calls, 1);

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = None;
    }

    #[test]
    fn test_kill_by_tid_removes_wait_queue_entry() {
        let _g = init_test_env();
        crate::task::registry::init::<MockRuntime>();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(7000);

        let current_task = crate::task::Task {
            id: 7000,
            state: TaskState::Running,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        let blocked_task = crate::task::Task {
            id: 7001,
            state: TaskState::Blocked,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(current_task));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(blocked_task));
        sched.state.wait_queue.push_back(7001);

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
        drop(sched_lock);

        assert!(kill_by_tid::<MockRuntime>(7001));
        assert!(sched.state.wait_queue.is_empty());
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(7001).unwrap().state,
            TaskState::Dead
        );

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = None;
    }

    #[test]
    fn test_wait_task_returns_immediately_for_dead_target() {
        let _g = init_test_env();

        let dead_task = crate::task::Task {
            id: 8001,
            state: TaskState::Dead,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: Some(23),
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(dead_task));

        assert_eq!(wait_task::<MockRuntime>(8001).unwrap(), 23);
    }

    #[test]
    fn test_wait_task_returns_echild_for_missing_target() {
        let _g = init_test_env();

        assert_eq!(wait_task::<MockRuntime>(8999).unwrap_err(), abi::errors::Errno::ECHILD);
    }

    #[test]
    fn test_register_task_exit_waiter_tracks_live_target() {
        let _g = init_test_env();

        let live_task = crate::task::Task {
            id: 8101,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(live_task));

        assert_eq!(register_task_exit_waiter::<MockRuntime>(8101, 8102).unwrap(), None);

        let waiters =
            crate::task::registry::get_task::<MockRuntime>(8101).unwrap().exit_waiters.drain();
        assert_eq!(waiters, alloc::vec![8102]);
    }

    #[test]
    fn test_kill_by_tid_wakes_registered_exit_waiter() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(8200);

        let current_task = crate::task::Task {
            id: 8200,
            state: TaskState::Running,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        let waiter_task = crate::task::Task {
            id: 8201,
            state: TaskState::Blocked,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        let target_task = crate::task::Task {
            id: 8202,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(current_task));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(waiter_task));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(target_task));

        let target_fields = crate::sched::state::TaskSchedFields {
            tid: 8202,
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
        };
        sched.state.insert_task(target_fields);

        assert_eq!(register_task_exit_waiter::<MockRuntime>(8202, 8201).unwrap(), None);

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
        drop(sched_lock);

        assert!(kill_by_tid::<MockRuntime>(8202));
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(8202).unwrap().state,
            TaskState::Dead
        );
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(8202).unwrap().exit_code,
            Some(-9)
        );
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(8201).unwrap().state,
            TaskState::Runnable
        );
        assert!(
            sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].iter().any(|&id| id == 8201)
        );

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = None;
    }

    #[test]
    fn test_poll_task_exit_reports_pending_dead_and_missing_targets() {
        let _g = init_test_env();

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(8301, TaskState::Runnable, TaskPriority::Normal),
        ));

        let mut dead = make_task(8302, TaskState::Dead, TaskPriority::Normal);
        dead.exit_code = Some(17);
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(dead));

        assert_eq!(poll_task_exit::<MockRuntime>(8301).unwrap(), None);
        assert_eq!(poll_task_exit::<MockRuntime>(8302).unwrap(), Some(17));
        assert_eq!(poll_task_exit::<MockRuntime>(8399).unwrap_err(), abi::errors::Errno::ECHILD);
    }

    #[test]
    fn test_terminate_current_purges_dead_task_from_scheduler_queues() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(8303);

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(8303, TaskState::Running, TaskPriority::Normal),
        ));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(8304, TaskState::Runnable, TaskPriority::Normal),
        ));

        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 8303,
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
        });
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 8304,
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
        });

        // Seed stale queue membership for the exiting task and ensure another
        // runnable task exists so terminate_current can produce a switch.
        sched.state.enqueue_task(0, TaskPriority::Normal as usize, 8303);
        sched.state.enqueue_task(0, TaskPriority::Normal as usize, 8304);
        sched.state.wait_queue.push_back(8303);
        sched.state.add_task_to_sleep_queue(8303, 55);
        sched.state.add_task_to_sleep_queue(9999, 55);

        let (switch, _waiters) = sched.terminate_current(101);

        assert_eq!(switch.to_tid, 8304, "scheduler should switch to the next runnable task");
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(8303).unwrap().state,
            TaskState::Dead
        );
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(8303).unwrap().exit_code,
            Some(101)
        );
        assert!(
            !sched.state.per_cpu[0].runq[TaskPriority::Normal as usize]
                .iter()
                .any(|&tid| tid == 8303),
            "dead current task must be removed from the run queue"
        );
        assert!(
            !sched.state.wait_queue.iter().any(|&tid| tid == 8303),
            "dead current task must be removed from the wait queue"
        );
        assert_eq!(sched.state.sleep_queue.get(&55).cloned(), Some(alloc::vec![9999]));
    }

    #[test]
    fn test_remove_task_completely_purges_scheduler_queues() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(0);

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(make_task(0, TaskState::Running, TaskPriority::Normal)));
        let mut dead = make_task(8305, TaskState::Dead, TaskPriority::Normal);
        dead.exit_code = Some(42);
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(dead));

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
        });
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 8305,
            runq_location: None,
            state: TaskState::Dead,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(0),
            wake_cpu: Some(0),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
        });

        sched.state.enqueue_task(0, TaskPriority::Normal as usize, 8305);
        sched.state.wait_queue.push_back(8305);
        sched.state.add_task_to_sleep_queue(8305, 77);

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
        drop(sched_lock);

        remove_task_completely::<MockRuntime>(8305);

        assert!(crate::task::registry::get_task::<MockRuntime>(8305).is_none());
        assert!(sched.state.get_task(8305).is_none());
        assert!(
            !sched.state.per_cpu[0].runq[TaskPriority::Normal as usize]
                .iter()
                .any(|&tid| tid == 8305),
            "reaped task must be removed from the run queue"
        );
        assert!(
            !sched.state.wait_queue.iter().any(|&tid| tid == 8305),
            "reaped task must be removed from the wait queue"
        );
        assert!(!sched.state.sleep_queue.contains_key(&77));

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = None;
    }

    #[test]
    fn test_unregister_task_exit_waiter_removes_only_requested_waiter() {
        let _g = init_test_env();

        let target = make_task(8401, TaskState::Runnable, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(target));

        register_task_exit_waiter::<MockRuntime>(8401, 8402).unwrap();
        register_task_exit_waiter::<MockRuntime>(8401, 8403).unwrap();
        unregister_task_exit_waiter::<MockRuntime>(8401, 8402).unwrap();

        let waiters =
            crate::task::registry::get_task::<MockRuntime>(8401).unwrap().exit_waiters.drain();
        assert_eq!(waiters, alloc::vec![8403]);
    }

    #[test]
    fn test_register_timeout_wake_deduplicates_task_ids() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(8500);

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
        drop(sched_lock);

        register_timeout_wake::<MockRuntime>(8501, 42);
        register_timeout_wake::<MockRuntime>(8501, 42);
        register_timeout_wake::<MockRuntime>(8502, 42);

        assert_eq!(sched.state.sleep_queue.get(&42).cloned().unwrap(), alloc::vec![8501, 8502]);

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = None;
    }

    #[test]
    fn test_unregister_timeout_wake_removes_task_and_updates_membership() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(8600);
        sched.state.add_task_to_sleep_queue(8601, 11);
        sched.state.add_task_to_sleep_queue(8602, 11);
        sched.state.add_task_to_sleep_queue(8602, 12);
        sched.state.add_task_to_sleep_queue(8603, 12);

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
        drop(sched_lock);

        unregister_timeout_wake::<MockRuntime>(8602);

        assert_eq!(sched.state.sleep_queue.get(&11).cloned().unwrap(), alloc::vec![8601]);
        assert_eq!(sched.state.sleep_queue.get(&12).cloned().unwrap(), alloc::vec![8603]);
        assert!(!sched.state.sleep_membership.contains_key(&8602));
        assert_eq!(
            sched.state.sleep_membership.get(&8603).copied(),
            Some(crate::sched::state::SleepMembership {
                wake_tick: 12,
                bucket_index: 0,
            })
        );

        unregister_timeout_wake::<MockRuntime>(8603);
        assert!(!sched.state.sleep_queue.contains_key(&12));

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = None;
    }

    #[test]
    fn test_interrupt_task_marks_and_consumes_pending_interrupt() {
        let _g = init_test_env();

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(0, TaskState::Runnable, TaskPriority::Normal),
        ));

        interrupt_task::<MockRuntime>(0).expect("interrupt task");
        assert!(crate::task::registry::get_task::<MockRuntime>(0).unwrap().pending_interrupt);
        assert!(take_pending_interrupt::<MockRuntime>());
        assert!(!take_pending_interrupt::<MockRuntime>());
    }

    #[test]
    fn test_interrupt_task_returns_esrch_for_missing_task() {
        let _g = init_test_env();
        assert_eq!(interrupt_task::<MockRuntime>(9999).unwrap_err(), abi::errors::Errno::ESRCH);
    }

    // ── waitpid tests ─────────────────────────────────────────────────────────

    /// Helper: build a task with a populated ProcessInfo (pid + ppid).
    fn make_process_task(
        id: TaskId,
        state: TaskState,
        pid: u32,
        ppid: u32,
        exit_code: Option<i32>,
    ) -> crate::task::Task<MockRuntime> {
        crate::task::Task {
            id,
            state,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: true,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: Some(alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
                pid,
                job: crate::task::ProcessLifecycle::new(ppid, pid as TaskId),
                unix_compat: crate::task::ProcessUnixCompat::isolated(pid, false),
                thing_table: crate::vfs::thing_table::ThingTable::new(),
                namespace: crate::vfs::NamespaceRef::global(),
                cwd: alloc::string::String::from("/"),
                root: alloc::string::String::from("/"),
                exec_path: alloc::string::String::new(),
                authority: crate::task::ProcessAuthority::root(),
                space: crate::task::ProcessAddressSpace::empty(),
            }))),
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        }
    }

    #[test]
    fn test_list_processes_hides_exit_code_for_live_job() {
        let _g = init_test_env();

        // Stale/non-authoritative exit_code on a live task should not surface
        // through ProcessSnapshot.
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_process_task(1200, TaskState::Running, 1200, 1, Some(77)),
        ));

        let snapshots = list_processes::<MockRuntime>();
        assert_eq!(snapshots.len(), 1);
        assert_eq!(snapshots[0].pid, 1200);
        assert_eq!(snapshots[0].state, TaskState::Running);
        assert_eq!(snapshots[0].exit_code, None);
    }

    #[test]
    fn test_list_processes_prefers_job_leader_exit_code_for_leader() {
        let _g = init_test_env();

        let mut leader = make_process_task(1210, TaskState::Dead, 1210, 1, Some(7));
        let pinfo = leader.process_info.as_ref().unwrap();
        pinfo.lock().job.leader_exit_code = Some(33);

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(leader));

        let snapshots = list_processes::<MockRuntime>();
        assert_eq!(snapshots.len(), 1);
        assert_eq!(snapshots[0].pid, 1210);
        assert_eq!(snapshots[0].tid, 1210);
        assert_eq!(snapshots[0].state, TaskState::Dead);
        assert_eq!(snapshots[0].exit_code, Some(33));
    }

    #[test]
    fn test_list_processes_uses_thread_exit_code_for_non_leader_tasks() {
        let _g = init_test_env();

        let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
            pid: 1220,
            job: crate::task::ProcessLifecycle {
                ppid: 1,
                thread_ids: alloc::vec![1220, 1221],
                exec_in_progress: false,
                children_done: alloc::collections::VecDeque::new(),
                exit_observer_inbox: None,
                leader_exit_code: Some(44),
                leader_exit_waiters: crate::sched::WaitQueue::new(),
            },
            unix_compat: crate::task::ProcessUnixCompat::isolated(1220, false),
            thing_table: crate::vfs::thing_table::ThingTable::new(),
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            root: alloc::string::String::from("/"),
            exec_path: alloc::string::String::new(),
            authority: crate::task::ProcessAuthority::root(),
            space: crate::task::ProcessAddressSpace::empty(),
        }));

        let mut sibling = make_task(1221, TaskState::Dead, TaskPriority::Normal);
        sibling.exit_code = Some(9);
        sibling.process_info = Some(alloc::sync::Arc::clone(&pinfo));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(sibling));

        let snapshots = list_processes::<MockRuntime>();
        assert_eq!(snapshots.len(), 1);
        assert_eq!(snapshots[0].tid, 1221);
        assert_eq!(snapshots[0].exit_code, Some(9));
    }

    #[test]
    fn test_list_processes_projects_place_and_authority_from_process_aggregator() {
        let _g = init_test_env();

        let mut leader = make_process_task(1230, TaskState::Running, 1230, 1, None);
        let pinfo = leader.process_info.as_ref().unwrap();
        {
            let mut pi = pinfo.lock();
            pi.cwd = alloc::string::String::from("/work");
            pi.root = alloc::string::String::from("/srv/chroot");
            pi.exec_path = alloc::string::String::from("/bin/demo");
            pi.authority.uid = 1000;
            pi.authority.gid = 1001;
            pi.authority.capability_mask = 0x24;
            pi.namespace = crate::vfs::NamespaceRef::isolated();
        }
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(leader));

        let snapshots = list_processes::<MockRuntime>();
        assert_eq!(snapshots.len(), 1);
        let snap = &snapshots[0];
        assert_eq!(snap.pid, 1230);
        assert_eq!(snap.cwd, "/work");
        assert_eq!(snap.root_path, "/srv/chroot");
        assert_eq!(snap.exec_path, "/bin/demo");
        assert_eq!(snap.uid, 1000);
        assert_eq!(snap.gid, 1001);
        assert_eq!(snap.capability_mask, 0x24);
        assert_ne!(snap.namespace_label, "global");
    }

    #[test]
    fn test_waitpid_returns_exit_code_for_dead_specific_child() {
        let _g = init_test_env();

        // Child task: pid=1001, ppid=1000, exit_code=42.
        // Tests call waitpid_for_pid directly so no parent task is needed.
        let child = make_process_task(9101, TaskState::Dead, 1001, 1000, Some(42));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

        let (child_pid, code) =
            waitpid_for_pid::<MockRuntime>(1000, 1001, 0).expect("waitpid specific");
        assert_eq!(child_pid, 1001, "returned child pid");
        assert_eq!(code, 42, "returned exit code");
    }

    #[test]
    fn test_waitpid_returns_exit_code_for_any_dead_child() {
        let _g = init_test_env();

        // Child: pid=2001, ppid=2000, exit_code=7
        let child = make_process_task(9201, TaskState::Dead, 2001, 2000, Some(7));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

        // pid == -1: wait for any child of process 2000
        let (child_pid, code) = waitpid_for_pid::<MockRuntime>(2000, -1, 0).expect("waitpid any");
        assert_eq!(child_pid, 2001);
        assert_eq!(code, 7);
    }

    #[test]
    fn test_waitpid_echild_when_no_children_exist() {
        let _g = init_test_env();

        // Registry is empty; no children for pid=3000
        let err = waitpid_for_pid::<MockRuntime>(3000, -1, 0).unwrap_err();
        assert_eq!(err, abi::errors::Errno::ECHILD);
    }

    #[test]
    fn test_waitpid_echild_when_specific_child_not_found() {
        let _g = init_test_env();

        // A child with a *different* ppid — should not be found for pid=4000
        let unrelated = make_process_task(9401, TaskState::Dead, 9999, 5000, Some(0));
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(unrelated));

        // Looking for child pid=9999 under parent pid=4000 → ECHILD
        let err = waitpid_for_pid::<MockRuntime>(4000, 9999, 0).unwrap_err();
        assert_eq!(err, abi::errors::Errno::ECHILD);
    }

    #[test]
    fn test_waitpid_wnohang_returns_zero_when_child_alive() {
        let _g = init_test_env();

        // Live child — not yet exited
        let child = make_process_task(9501, TaskState::Runnable, 5001, 5000, None);
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

        let (child_pid, code) =
            waitpid_for_pid::<MockRuntime>(5000, -1, abi::types::waitpid_flags::WNOHANG)
                .expect("wnohang");
        assert_eq!(child_pid, 0, "no child exited yet");
        assert_eq!(code, 0);
    }

    #[test]
    fn test_waitpid_multiple_children_returns_first_dead() {
        let _g = init_test_env();

        // Two children under parent pid=6000: first alive, second dead
        let child_alive = make_process_task(9601, TaskState::Runnable, 6001, 6000, None);
        let child_dead = make_process_task(9602, TaskState::Dead, 6002, 6000, Some(99));

        let mut reg = crate::task::registry::get_registry::<MockRuntime>();
        reg.insert(alloc::boxed::Box::new(child_alive));
        reg.insert(alloc::boxed::Box::new(child_dead));
        drop(reg);

        let (child_pid, code) = waitpid_for_pid::<MockRuntime>(6000, -1, 0).expect("waitpid multi");
        assert_eq!(child_pid, 6002);
        assert_eq!(code, 99);
    }

    /// After `waitpid` successfully returns a dead child's exit code the child's
    /// registry entry must be removed (reaped).  Without reaping, dead process
    /// records accumulate indefinitely ("zombie" leak).
    #[test]
    fn test_waitpid_reaps_dead_child() {
        let _g = init_test_env();

        let child = make_process_task(9701, TaskState::Dead, 7001, 7000, Some(55));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

        // The child is still in the registry before the wait.
        assert!(
            crate::task::registry::get_task::<MockRuntime>(9701).is_some(),
            "child must be in registry before waitpid"
        );

        let (child_pid, code) =
            waitpid_for_pid::<MockRuntime>(7000, 7001, 0).expect("waitpid reap");
        assert_eq!(child_pid, 7001);
        assert_eq!(code, 55);

        // After a successful wait the child record must have been reaped.
        assert!(
            crate::task::registry::get_task::<MockRuntime>(9701).is_none(),
            "child record must be removed from registry after reaping"
        );
    }

    /// Once a child has been reaped, a second `waitpid` for the same child must
    /// return `ECHILD` — the record no longer exists.
    #[test]
    fn test_waitpid_echild_after_reaping() {
        let _g = init_test_env();

        let child = make_process_task(9702, TaskState::Dead, 7002, 7003, Some(0));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

        // First wait reaps the child.
        waitpid_for_pid::<MockRuntime>(7003, 7002, 0).expect("first waitpid");

        // Second wait must fail because the record was removed.
        let err = waitpid_for_pid::<MockRuntime>(7003, 7002, 0).unwrap_err();
        assert_eq!(
            err,
            abi::errors::Errno::ECHILD,
            "second waitpid after reaping must return ECHILD"
        );
    }

    /// A dead child whose parent has NOT yet called `waitpid` must remain in
    /// the registry (zombie semantics: exit status preserved until collected).
    #[test]
    fn test_dead_child_stays_in_registry_until_reaped() {
        let _g = init_test_env();

        let child = make_process_task(9703, TaskState::Dead, 7010, 7011, Some(3));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

        // No waitpid called yet — record must still be present.
        assert!(
            crate::task::registry::get_task::<MockRuntime>(9703).is_some(),
            "dead child must remain in registry until reaped"
        );

        // Verify state and exit code are accessible while zombie.
        let task = crate::task::registry::get_task::<MockRuntime>(9703).unwrap();
        assert_eq!(task.state, TaskState::Dead);
        assert_eq!(task.exit_code, Some(3));
    }

    /// `waitpid` with WNOHANG must not reap any child when no child has exited.
    #[test]
    fn test_waitpid_wnohang_does_not_reap_live_child() {
        let _g = init_test_env();

        let child = make_process_task(9704, TaskState::Runnable, 7020, 7021, None);
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

        let (returned_pid, _) =
            waitpid_for_pid::<MockRuntime>(7021, -1, abi::types::waitpid_flags::WNOHANG)
                .expect("wnohang on live child");
        assert_eq!(returned_pid, 0, "wnohang returns 0 when no child exited");

        // Live child must still be in registry.
        assert!(
            crate::task::registry::get_task::<MockRuntime>(9704).is_some(),
            "live child must remain in registry after WNOHANG poll"
        );
    }

    /// Helper: create a task + ProcessInfo with `tgid` populated and a shared
    /// thread_ids list for multi-thread tests.
    fn make_thread_task(
        id: TaskId,
        state: TaskState,
        pid: u32,
        ppid: u32,
        shared_pinfo: alloc::sync::Arc<spin::Mutex<crate::task::ProcessInfo>>,
    ) -> crate::task::Task<MockRuntime> {
        crate::task::Task {
            id,
            state,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: true,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: Some(shared_pinfo),
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        }
    }

    /// Thread IDs are tracked in ProcessInfo.thread_ids when tasks share the
    /// same ProcessInfo Arc.
    #[test]
    fn test_thread_ids_tracked_in_process_info() {
        let _g = init_test_env();

        // Build a shared ProcessInfo for a 2-thread group.
        // pid = 7000 (thread-group leader), thread_ids = [7000, 7001].
        let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
            pid: 7000,
            job: crate::task::ProcessLifecycle {
                ppid: 1,
                thread_ids: alloc::vec![7000, 7001],
                exec_in_progress: false,
                children_done: alloc::collections::VecDeque::new(),
                exit_observer_inbox: None,
                leader_exit_code: None,
                leader_exit_waiters: crate::sched::WaitQueue::new(),
            },
            unix_compat: crate::task::ProcessUnixCompat::isolated(7000, false),
            thing_table: crate::vfs::thing_table::ThingTable::new(),
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            root: alloc::string::String::from("/"),
            exec_path: alloc::string::String::new(),
            authority: crate::task::ProcessAuthority::root(),
            space: crate::task::ProcessAddressSpace::empty(),
        }));

        {
            let pi = pinfo.lock();
            assert_eq!(pi.job.thread_ids.len(), 2);
            assert!(pi.job.thread_ids.contains(&7000));
            assert!(pi.job.thread_ids.contains(&7001));
            assert_eq!(pi.pid, 7000);
        }
    }

    /// When the thread-group leader exits, sibling threads are removed from the
    /// thread_ids list and their scheduler state is set to Dead.
    #[test]
    fn test_mark_task_exited_removes_tid_from_thread_ids() {
        let _g = init_test_env();

        // Shared ProcessInfo for a 2-thread group: leader 8700, sibling 8701.
        let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
            pid: 8700,
            job: crate::task::ProcessLifecycle {
                ppid: 1,
                thread_ids: alloc::vec![8700, 8701],
                exec_in_progress: false,
                children_done: alloc::collections::VecDeque::new(),
                exit_observer_inbox: None,
                leader_exit_code: None,
                leader_exit_waiters: crate::sched::WaitQueue::new(),
            },
            unix_compat: crate::task::ProcessUnixCompat::isolated(8700, false),
            thing_table: crate::vfs::thing_table::ThingTable::new(),
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            root: alloc::string::String::from("/"),
            exec_path: alloc::string::String::new(),
            authority: crate::task::ProcessAuthority::root(),
            space: crate::task::ProcessAddressSpace::empty(),
        }));

        // Register both tasks.
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_thread_task(8700, TaskState::Running, 8700, 1, pinfo.clone()),
        ));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_thread_task(8701, TaskState::Runnable, 8700, 1, pinfo.clone()),
        ));

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(8700);

        // Exit the sibling thread first — its TID should be removed from thread_ids.
        let _ = mark_task_exited::<MockRuntime>(&mut sched, 8701, 0);

        {
            let pi = pinfo.lock();
            // 8701 should have been removed.
            assert!(!pi.job.thread_ids.contains(&8701), "sibling TID still in thread_ids");
            // 8700 (leader) is still present — it hasn't exited yet.
            assert!(pi.job.thread_ids.contains(&8700), "leader TID wrongly removed");
        }

        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(8701).unwrap().state,
            TaskState::Dead,
            "sibling should be dead"
        );
    }

    /// When the thread-group leader (tid == pid) exits, remaining sibling
    /// threads are also killed (thread-group exit).
    #[test]
    fn test_thread_group_leader_exit_kills_siblings() {
        let _g = init_test_env();

        // Shared ProcessInfo for a 2-thread group: leader 8800, sibling 8801.
        let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
            pid: 8800,
            job: crate::task::ProcessLifecycle {
                ppid: 1,
                thread_ids: alloc::vec![8800, 8801],
                exec_in_progress: false,
                children_done: alloc::collections::VecDeque::new(),
                exit_observer_inbox: None,
                leader_exit_code: None,
                leader_exit_waiters: crate::sched::WaitQueue::new(),
            },
            unix_compat: crate::task::ProcessUnixCompat::isolated(8800, false),
            thing_table: crate::vfs::thing_table::ThingTable::new(),
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            root: alloc::string::String::from("/"),
            exec_path: alloc::string::String::new(),
            authority: crate::task::ProcessAuthority::root(),
            space: crate::task::ProcessAddressSpace::empty(),
        }));

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_thread_task(8800, TaskState::Running, 8800, 1, pinfo.clone()),
        ));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_thread_task(8801, TaskState::Runnable, 8800, 1, pinfo.clone()),
        ));

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(8800);

        // Exit the thread-group leader.
        let _ = mark_task_exited::<MockRuntime>(&mut sched, 8800, 42);

        // Leader must be dead.
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(8800).unwrap().state,
            TaskState::Dead,
            "leader should be dead"
        );

        // Sibling must also be dead (killed by thread-group exit).
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(8801).unwrap().state,
            TaskState::Dead,
            "sibling should be killed on leader exit"
        );

        // Both TIDs removed from thread_ids.
        assert!(
            pinfo.lock().job.thread_ids.is_empty(),
            "thread_ids should be empty after group exit"
        );
    }

    /// Leader exit is projected to the parent wait queue using encoded
    /// wait-status semantics, and parent threads are returned as wake targets.
    #[test]
    fn test_mark_task_exited_queues_parent_wait_status() {
        let _g = init_test_env();
        unsafe {
            crate::sched::hooks::PROCESS_INFO_FOR_PID_HOOK =
                Some(process_info_for_pid::<MockRuntime>);
        }

        // Parent process (pid 9900) with one thread waiting in waitpid path.
        let parent_pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
            pid: 9900,
            job: crate::task::ProcessLifecycle {
                ppid: 1,
                thread_ids: alloc::vec![9900],
                exec_in_progress: false,
                children_done: alloc::collections::VecDeque::new(),
                exit_observer_inbox: None,
                leader_exit_code: None,
                leader_exit_waiters: crate::sched::WaitQueue::new(),
            },
            unix_compat: crate::task::ProcessUnixCompat::isolated(9900, false),
            thing_table: crate::vfs::thing_table::ThingTable::new(),
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            root: alloc::string::String::from("/"),
            exec_path: alloc::string::String::new(),
            authority: crate::task::ProcessAuthority::root(),
            space: crate::task::ProcessAddressSpace::empty(),
        }));

        // Child process (pid 9800) whose leader exits with code 7.
        let child_pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
            pid: 9800,
            job: crate::task::ProcessLifecycle {
                ppid: 9900,
                thread_ids: alloc::vec![9800],
                exec_in_progress: false,
                children_done: alloc::collections::VecDeque::new(),
                exit_observer_inbox: None,
                leader_exit_code: None,
                leader_exit_waiters: crate::sched::WaitQueue::new(),
            },
            unix_compat: crate::task::ProcessUnixCompat::isolated(9800, false),
            thing_table: crate::vfs::thing_table::ThingTable::new(),
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            root: alloc::string::String::from("/"),
            exec_path: alloc::string::String::new(),
            authority: crate::task::ProcessAuthority::root(),
            space: crate::task::ProcessAddressSpace::empty(),
        }));

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_thread_task(9900, TaskState::Blocked, 9900, 1, parent_pinfo.clone()),
        ));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_thread_task(9800, TaskState::Running, 9800, 9900, child_pinfo),
        ));

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(9800);

        let waiters = mark_task_exited::<MockRuntime>(&mut sched, 9800, 7);
        assert_eq!(waiters, alloc::vec![9900], "parent thread should be returned for wakeup");

        let parent = parent_pinfo.lock();
        assert_eq!(parent.job.children_done.len(), 1);
        assert_eq!(
            parent.job.children_done.front().copied(),
            Some((9800, abi::signal::w_exit_status(7))),
            "leader exit should be queued as encoded wait status"
        );
    }

    /// exec_in_progress: killing siblings during exec collapse removes their
    /// TIDs from thread_ids, leaving only the exec-calling thread.
    #[test]
    fn test_exec_collapse_kills_siblings_and_updates_thread_ids() {
        let _g = init_test_env();

        // Shared ProcessInfo for a 3-thread group: leader 9100, siblings 9101, 9102.
        let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
            pid: 9100,
            job: crate::task::ProcessLifecycle {
                ppid: 1,
                thread_ids: alloc::vec![9100, 9101, 9102],
                exec_in_progress: false,
                children_done: alloc::collections::VecDeque::new(),
                exit_observer_inbox: None,
                leader_exit_code: None,
                leader_exit_waiters: crate::sched::WaitQueue::new(),
            },
            unix_compat: crate::task::ProcessUnixCompat::isolated(9100, false),
            thing_table: crate::vfs::thing_table::ThingTable::new(),
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            root: alloc::string::String::from("/"),
            exec_path: alloc::string::String::new(),
            authority: crate::task::ProcessAuthority::root(),
            space: crate::task::ProcessAddressSpace::empty(),
        }));

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_thread_task(9100, TaskState::Running, 9100, 1, pinfo.clone()),
        ));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_thread_task(9101, TaskState::Runnable, 9100, 1, pinfo.clone()),
        ));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_thread_task(9102, TaskState::Runnable, 9100, 1, pinfo.clone()),
        ));

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        // Mark 9100 as the "current" (exec-calling) thread.
        sched.state.per_cpu[0].current = Some(9100);

        // Step 1: simulate exec – set exec_in_progress.
        pinfo.lock().job.exec_in_progress = true;

        // Step 2: collect siblings.
        let caller_tid: TaskId = 9100;
        let siblings: alloc::vec::Vec<TaskId> =
            pinfo.lock().job.thread_ids.iter().copied().filter(|&t| t != caller_tid).collect();
        assert_eq!(siblings.len(), 2);

        // Step 3: kill siblings (simulates kill_by_tid path).
        for sibling in siblings {
            let _ = mark_task_exited::<MockRuntime>(&mut sched, sibling, -9);
        }

        // Both siblings must be dead.
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(9101).unwrap().state,
            TaskState::Dead,
            "sibling 9101 should be dead"
        );
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(9102).unwrap().state,
            TaskState::Dead,
            "sibling 9102 should be dead"
        );

        // thread_ids should contain only the caller.
        {
            let pi = pinfo.lock();
            assert_eq!(
                pi.job.thread_ids,
                alloc::vec![caller_tid],
                "only caller TID should remain after collapse"
            );
        }

        // Step 4: simulate commit – clear exec_in_progress.
        pinfo.lock().job.exec_in_progress = false;
        assert!(!pinfo.lock().job.exec_in_progress, "exec_in_progress cleared after commit");
    }

    /// exec collapse with 4 threads is deterministic: ALL siblings (9701–9703)
    /// are in Dead state before the exec-caller (9700) proceeds to commit.
    ///
    /// This tests the full scheduler + registry path that `task_exec_current`
    /// uses via `kill_by_tid` → `mark_task_exited`:
    ///   1. Set exec_in_progress.
    ///   2. Collect sibling TIDs (exclude caller).
    ///   3. Kill every sibling via mark_task_exited.
    ///   4. Assert every sibling is Dead and only caller TID remains.
    ///   5. Assert exec-caller is NOT Dead.
    ///   6. Commit: clear exec_in_progress.
    #[test]
    fn test_exec_collapse_determinism_four_threads() {
        let _g = init_test_env();

        let caller_tid: TaskId = 9700;
        let sibling_tids: [TaskId; 3] = [9701, 9702, 9703];

        let mut all_tids = alloc::vec![caller_tid];
        all_tids.extend_from_slice(&sibling_tids);

        let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
            pid: 9700,
            job: crate::task::ProcessLifecycle {
                ppid: 1,
                thread_ids: all_tids.clone(),
                exec_in_progress: false,
                children_done: alloc::collections::VecDeque::new(),
                exit_observer_inbox: None,
                leader_exit_code: None,
                leader_exit_waiters: crate::sched::WaitQueue::new(),
            },
            unix_compat: {
                let mut uc = crate::task::ProcessUnixCompat::isolated(9700, false);
                uc.set_spawn_context(alloc::vec![b"old".to_vec()], alloc::vec![]);
                uc
            },
            thing_table: crate::vfs::thing_table::ThingTable::new(),
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            root: alloc::string::String::from("/"),
            exec_path: alloc::string::String::from("/old/binary"),
            authority: crate::task::ProcessAuthority::root(),
            space: crate::task::ProcessAddressSpace::empty(),
        }));

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_thread_task(caller_tid, TaskState::Running, 9700, 1, pinfo.clone()),
        ));
        for &sid in &sibling_tids {
            crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
                make_thread_task(sid, TaskState::Runnable, 9700, 1, pinfo.clone()),
            ));
        }

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(caller_tid);

        // Phase 1: set exec_in_progress atomically.
        pinfo.lock().job.exec_in_progress = true;

        // Phase 2: collect sibling TIDs (excluding caller).
        let siblings: alloc::vec::Vec<TaskId> =
            pinfo.lock().job.thread_ids.iter().copied().filter(|&t| t != caller_tid).collect();
        assert_eq!(siblings.len(), 3, "expected 3 siblings");
        assert!(!siblings.contains(&caller_tid), "caller must not appear in sibling list");

        // Phase 3: kill every sibling (as task_exec_current calls kill_by_tid_current).
        for &sid in &siblings {
            let _ = mark_task_exited::<MockRuntime>(&mut sched, sid, -9);
        }

        // Phase 4: invariant — every sibling must be Dead before commit.
        for &sid in &sibling_tids {
            assert_eq!(
                crate::task::registry::get_task::<MockRuntime>(sid)
                    .expect("sibling must remain as zombie")
                    .state,
                TaskState::Dead,
                "sibling {} must be Dead after exec collapse",
                sid
            );
        }

        // thread_ids must contain only the exec-caller.
        assert_eq!(
            pinfo.lock().job.thread_ids,
            alloc::vec![caller_tid],
            "only exec-caller TID must remain in thread_ids after collapse"
        );

        // The exec-caller itself must NOT be Dead.
        assert_ne!(
            crate::task::registry::get_task::<MockRuntime>(caller_tid)
                .expect("exec-caller must still be in registry")
                .state,
            TaskState::Dead,
            "exec-caller must not be killed during collapse"
        );

        // Phase 5: commit — clear exec_in_progress.
        pinfo.lock().job.exec_in_progress = false;
        assert!(
            !pinfo.lock().job.exec_in_progress,
            "exec_in_progress must be cleared after commit"
        );
    }

    /// exec_in_progress blocks additional thread creation at the process level.
    #[test]
    fn test_exec_in_progress_rejects_new_threads() {
        let _g = init_test_env();

        let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
            pid: 9300,
            job: crate::task::ProcessLifecycle::new(1, 9300),
            unix_compat: crate::task::ProcessUnixCompat::isolated(9300, false),
            thing_table: crate::vfs::thing_table::ThingTable::new(),
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            root: alloc::string::String::from("/"),
            exec_path: alloc::string::String::new(),
            authority: crate::task::ProcessAuthority::root(),
            space: crate::task::ProcessAddressSpace::empty(),
        }));

        // Before exec: flag is clear — new threads would be accepted.
        assert!(!pinfo.lock().job.exec_in_progress);

        // Set exec_in_progress (as task_exec_current does at the start).
        pinfo.lock().job.exec_in_progress = true;

        // The sys_spawn_thread handler checks this flag and returns EAGAIN.
        // Here we verify the condition it tests.
        assert!(
            pinfo.lock().job.exec_in_progress,
            "exec_in_progress must be set to block SYS_SPAWN_THREAD"
        );

        // Rollback: clear the flag on pre-commit failure.
        pinfo.lock().job.exec_in_progress = false;
        assert!(!pinfo.lock().job.exec_in_progress, "flag cleared after rollback");
    }

    // ── TLS-base and detached-thread tests ───────────────────────────────────

    /// A task constructed with a non-zero `user_fs_base` retains that value.
    ///
    /// This is the kernel-side invariant for the TLS-base handoff: the spawn
    /// path stores `tls_base` in `Task.user_fs_base`, and the scheduler
    /// writes it to hardware (FS_BASE) on the first context switch.
    #[test]
    fn test_tls_base_stored_in_task_user_fs_base() {
        let _g = init_test_env();

        let tls_base: u64 = 0xDEAD_CAFE_0000_0000;

        let task = crate::task::Task {
            id: 9800,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: true,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: tls_base,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

        let stored = crate::task::registry::get_task::<MockRuntime>(9800)
            .expect("task must be in registry")
            .user_fs_base;
        assert_eq!(stored, tls_base, "user_fs_base must equal the requested tls_base");
    }

    /// Joining a detached thread must return `EINVAL`.
    #[test]
    fn test_detached_thread_cannot_be_joined() {
        let _g = init_test_env();

        let task = crate::task::Task {
            id: 9801,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: true,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: true, // detached — must not be joinable
            signals: crate::signal::ThreadSignals::new(),
        };

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

        assert_eq!(
            register_task_exit_waiter::<MockRuntime>(9801, 9802).unwrap_err(),
            abi::errors::Errno::EINVAL,
            "joining a detached thread must return EINVAL"
        );
    }

    /// A live joinable (non-detached) thread allows waiting via
    /// `register_task_exit_waiter`, returning `None` (not yet exited).
    #[test]
    fn test_joinable_thread_can_be_waited_on() {
        let _g = init_test_env();

        let task = crate::task::Task {
            id: 9803,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: true,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Any,
            kstack_base: core::ptr::null_mut(),
            kstack_size: 0,
            kstack_top: 0,
            ctx: Default::default(),
            aspace: MockAddressSpace(0),
            simd: crate::simd::SimdState::new(&MOCK_RUNTIME),
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            last_cpu: Some(0),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false, // joinable
            signals: crate::signal::ThreadSignals::new(),
        };

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

        // Should succeed and return None (thread still running).
        assert_eq!(
            register_task_exit_waiter::<MockRuntime>(9803, 9804).unwrap(),
            None,
            "joining a live joinable thread must return None"
        );
    }

    // ── IRQ balance regression tests ──────────────────────────────────────────

    /// `block_current` must restore IRQ state on the early-return path that
    /// triggers when there is no current task on the calling CPU (`current_id
    /// == None`).  Previously this path returned without calling `irq_restore`,
    /// leaving the CPU with interrupts permanently disabled.
    #[test]
    fn test_block_current_restores_irq_when_no_current_task() {
        let _g = init_test_env();
        reset_mock_irq_depth();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        // CPU 0 with no current task.
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = None;

        let mut lock = SCHEDULER.lock();
        *lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
        drop(lock);

        block_current::<MockRuntime>();

        // IRQ depth must be back to 0 — irq_disable was paired with irq_restore.
        assert_eq!(mock_irq_depth(), 0, "block_current left IRQs disabled (depth != 0)");

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = None;
    }

    /// `sleep_ticks` must restore IRQ state on the early-return path that
    /// triggers when there is no current task on the calling CPU (`current_id
    /// == None`).  Without the fix the `None` arm returned without calling
    /// `irq_restore`, leaving interrupts disabled.
    #[test]
    fn test_sleep_ticks_restores_irq_when_no_current_task() {
        let _g = init_test_env();
        reset_mock_irq_depth();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        // CPU 0 with no current task.
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = None;

        let mut lock = SCHEDULER.lock();
        *lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
        drop(lock);

        sleep_ticks::<MockRuntime>(5);

        // IRQ depth must be back to 0 — irq_disable was paired with irq_restore.
        assert_eq!(mock_irq_depth(), 0, "sleep_ticks left IRQs disabled (depth != 0)");

        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = None;
    }

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

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(make_task(0, TaskState::Running, TaskPriority::Normal)));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(9901, TaskState::Blocked, TaskPriority::Normal),
        ));

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
        });

        let (_ipi, _deferred) =
            crate::sched::blocking::wake_task_locked::<MockRuntime>(&mut sched, 9901);
        assert!(
            sched.state.per_cpu[2].runq[TaskPriority::Normal as usize]
                .iter()
                .any(|&tid| tid == 9901),
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

        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(make_task(0, TaskState::Running, TaskPriority::Normal)));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(9911, TaskState::Blocked, TaskPriority::Normal),
        ));

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
        });

        // Make CPU 0 overloaded relative to CPU 1.
        const OVERLOAD_TASK_START: u64 = 9912;
        const OVERLOAD_TASK_END: u64 = 9915;
        for id in OVERLOAD_TASK_START..OVERLOAD_TASK_END {
            crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
                make_task(id, TaskState::Runnable, TaskPriority::Low),
            ));
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
                wake_pending: false,
            });
            sched.state.enqueue_task(0, TaskPriority::Low as usize, id);
        }

        let (_ipi, _deferred) =
            crate::sched::blocking::wake_task_locked::<MockRuntime>(&mut sched, 9911);
        assert!(
            sched.state.per_cpu[1].runq[TaskPriority::Normal as usize]
                .iter()
                .any(|&tid| tid == 9911),
            "[policy] redirect should move Any-affinity wakeup to least-loaded CPU"
        );
        assert_eq!(
            sched.state.get_task(9911).and_then(|sf| sf.wake_cpu),
            Some(1),
            "[policy] wake_cpu should track redirected Any-affinity wakeup target"
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
        });

        let sleeping = make_task(9921, TaskState::Blocked, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(sleeping));
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
                wake_pending: false,
            });
            sched.state.enqueue_task(0, TaskPriority::Low as usize, id);
        }

        TICK_COUNT.store(100, Ordering::Relaxed);
        sched.state.add_task_to_sleep_queue(9921, 50);
        sched.wake_sleepers();

        assert!(
            sched.state.per_cpu[1].runq[TaskPriority::Normal as usize]
                .iter()
                .any(|&tid| tid == 9921),
            "[policy] wake_sleepers should redirect Any-affinity wakeup under overload"
        );
        assert_eq!(
            sched.state.get_task(9921).and_then(|sf| sf.wake_cpu),
            Some(1),
            "[policy] wake_sleepers should record redirected wake_cpu for Any-affinity task"
        );
    }

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
        GLOBAL_NEED_RESCHED[0].store(true, core::sync::atomic::Ordering::Release);

        // Must run while SCHEDULER lock is held so try_lock path fails.
        try_resched_if_needed::<MockRuntime>();

        let after_miss = PROF_TRYLOCK_MISS_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);
        let after_pending =
            PROF_TRYLOCK_MISS_PENDING_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);
        assert!(
            after_miss > before_miss,
            "[contention] per-CPU trylock miss counter should increment"
        );
        assert!(
            after_pending > before_pending,
            "[contention] per-CPU pending-resched trylock miss counter should increment"
        );

        drop(lock);
        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = None;
    }
}
