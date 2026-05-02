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
//!
//! ## Per-CPU Ownership Model
//!
//! Each logical CPU has an exclusive [`state::CpuScheduler`] that owns:
//! - `current` — the task currently executing on this CPU.
//! - `idle_task` — the CPU-local idle task.
//! - `runq` — the local priority-indexed run queue (see [`state::RunQueue`]).
//! - `need_resched` — the reschedule-pending flag for this CPU.
//! - `stats` — per-CPU scheduling counters.
//!
//! **Invariant**: A CPU owns its [`state::CpuScheduler`] state. Other CPUs
//! **must not** directly mutate another CPU's local run queue. Cross-CPU
//! scheduling effects go through explicit delivery mechanisms (remote-wake
//! mailboxes, IPIs). See [`REMOTE_WAKE_MAILBOXES`] and later issues for the
//! mailbox path.
//!
//! Normal scheduling on CPU *N* reads and writes through
//! `sched.state.per_cpu[N]` (a [`state::CpuScheduler`] value). The global
//! [`Scheduler`] coordinates cross-CPU policy — task placement, load
//! balancing, diagnostics — without owning CPU-local execution state directly.
//!
//! ## Per-CPU Preemption (No Global Preemption Lock)
//!
//! Preemption decisions are made **independently per CPU** without a global
//! preemption lock:
//!
//! * The per-CPU `need_resched` flag (mirrored to the lockless
//!   [`GLOBAL_NEED_RESCHED`] array) is set by:
//!   - the timer tick handler when the current task's timeslice expires,
//!   - the remote-wake mailbox drain when a higher-priority task is woken,
//!   - cross-CPU wakeup/IPI delivery paths.
//! * Safe-point callers (e.g. `task::resched_if_needed`,
//!   `task::preempt_enable`) first check [`need_resched_pending`] atomically
//!   — **without** acquiring the scheduler lock — and only acquire the lock
//!   when a reschedule is actually needed.
//! * Each CPU therefore makes its own preemption decision based solely on
//!   its own per-CPU atomic flag, scaling independently with CPU count.
//!
//! Lock-order policy:
//! - `SCHEDULER` must never take `task::registry::REGISTRY` or
//!   `device_registry::REGISTRY`.
//! - Scheduler-owned paths defer registry/device work and apply it after
//!   releasing `SCHEDULER`.

pub(crate) mod blocking;
pub mod bridge;
pub mod hooks;
pub use hooks::protect_user_range_current;
pub mod lifecycle;
pub use lifecycle::*;
pub mod policy;
pub mod profiling;
pub use profiling::*;
pub(crate) use profiling::{is_task_on_other_cpu, set_cpu_current_task};
mod sleep;
mod spawn;
mod stack;
pub mod state;
pub(crate) mod types;
mod vm;
pub(crate) mod wait_queue;

// Re-export all public items
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, AtomicIsize, AtomicU8, AtomicU64, AtomicUsize, Ordering};

pub use blocking::{
    block_current, block_current_erased, init_blocking_hooks, try_wake_task_from_irq,
    try_wake_task_from_irq_erased, wake_task, wake_task_erased,
};
pub use hooks::{
    CpuSchedDiag, ProcessSnapshot, SchedDiag, add_user_mapping_current, alloc_user_stack_current,
    available_parallelism_current, check_user_mapping_current, collect_sched_diag_current,
    current_priority_current, current_task_name_current, current_task_resource_id,
    current_tid_current, current_user_fs_base_current, dump_stats_current, exit_current,
    get_signal_mask_current, get_thread_pending_current, get_user_mapping_at_current,
    handle_user_stack_fault_current, interrupt_task_current, kill_by_tid_current,
    list_process_ids_by_pgid_current, list_processes_current, poll_task_exit_current,
    process_info_current, process_info_for_pid_current, process_info_for_tid_current,
    register_task_exit_waiter_current, register_timeout_wake_current, remove_user_mappings_current,
    set_current_task_name_current, set_current_user_fs_base_current, set_priority_current,
    set_signal_mask_current, set_thread_pending_current, sleep_ticks_current,
    spawn_process_current, spawn_process_ex_current, spawn_process_from_path_current,
    spawn_user_thread_current, take_pending_interrupt_current, task_exec_current,
    task_status_current, task_wait_current, unregister_task_exit_waiter_current,
    unregister_timeout_wake_current, waitpid_current, yield_now_current,
};
pub use policy::{DefaultPolicy, SchedPolicy};
pub use sleep::{sleep_ms, sleep_ticks, sleep_until, yield_now};
pub use spawn::{
    SpawnExResult, StdioSpec, boot_spawn_process, spawn, spawn_user_task_full, spawn_user_thread,
    spawn_user_thread_ex, spawn_with_priority, user_thread_trampoline,
};
use spin::Mutex;
pub use stack::{alloc_user_stack, handle_stack_fault, map_user_page, map_user_page_perms};
pub use state::{
    CpuSchedStats, CpuScheduler, MigrationState, RunQueue, WakeMailbox, WakeMailboxEntry,
};
pub use types::{
    DEFAULT_TIMESLICE, ScheduleReason, Scheduler, StackFaultResult, SwitchDecision, SwitchParams,
};
pub use wait_queue::WaitQueue;

use crate::task::{Affinity, StartupArg, Task, TaskId, TaskPriority, TaskState};
use crate::{BootRuntime, BootTasking};

#[cfg(any(feature = "sched_debug", debug_assertions))]
static SWITCH_LOG_COUNT: AtomicUsize = AtomicUsize::new(0);

pub static SCHEDULER: Mutex<Option<usize>> = Mutex::new(None);
pub static SCHEDULER_LOCK_OWNER: AtomicIsize = AtomicIsize::new(-1);
pub static SCHEDULER_LOCK_ACQUIRED_AT: AtomicU64 = AtomicU64::new(0);

pub struct SchedLockTrackingGuard<R: BootRuntime> {
    owner_cpu: isize,
    _phantom: PhantomData<R>,
}

#[inline]
fn clear_sched_lock_tracking_owner(owner_cpu: isize) {
    if SCHEDULER_LOCK_OWNER
        .compare_exchange(owner_cpu, -1, Ordering::AcqRel, Ordering::Acquire)
        .is_ok()
    {
        SCHEDULER_LOCK_ACQUIRED_AT.store(0, Ordering::Release);
    }
}

#[inline]
pub fn set_sched_lock_tracking<R: BootRuntime>(cpu_idx: usize) {
    SCHEDULER_LOCK_OWNER.store(cpu_idx as isize, Ordering::Release);
    // Use TICK_COUNT instead of mono_ticks to avoid false-positive deadlocks caused by TSC
    // desynchronization across CPUs (especially between BSP and APs under KVM).
    // Store TICK_COUNT + 1 so that a tick count of 0 does not disable the watchdog.
    SCHEDULER_LOCK_ACQUIRED_AT.store(TICK_COUNT.load(Ordering::Relaxed) + 1, Ordering::Release);
}

#[inline]
pub fn sched_lock_tracking_guard<R: BootRuntime>(cpu_idx: usize) -> SchedLockTrackingGuard<R> {
    set_sched_lock_tracking::<R>(cpu_idx);
    SchedLockTrackingGuard { owner_cpu: cpu_idx as isize, _phantom: PhantomData }
}

#[inline]
pub fn clear_sched_lock_tracking<R: BootRuntime>() {
    let cpu_owner = crate::runtime::<R>().current_cpu_index() as isize;
    clear_sched_lock_tracking_owner(cpu_owner);
}

impl<R: BootRuntime> Drop for SchedLockTrackingGuard<R> {
    fn drop(&mut self) {
        clear_sched_lock_tracking_owner(self.owner_cpu);
    }
}

#[inline]
fn debug_assert_scheduler_not_held_by_this_cpu<R: BootRuntime>(context: &str) {
    let owner = SCHEDULER_LOCK_OWNER.load(Ordering::Acquire);
    let cpu = crate::runtime::<R>().current_cpu_index() as isize;
    // Keep lightweight telemetry in all builds; debug builds also assert.
    if owner == cpu {
        PROF_LOCK_ORDER_VIOLATIONS.fetch_add(1, Ordering::Relaxed);
    }
    #[cfg(debug_assertions)]
    {
        debug_assert_ne!(
            owner, cpu,
            "scheduler lock-order violation: {} attempted while SCHEDULER is held on CPU {}",
            context, cpu
        );
    }
}

/// Assert that a run-queue enqueue targets the **calling CPU** only.
///
/// Per-CPU run queues are **owned** by their CPU.  Only the owning CPU must
/// enqueue tasks directly into its own run queue; all other CPUs must route
/// through the per-CPU [`WakeMailbox`][crate::sched::state::WakeMailbox] (via
/// [`enqueue_remote_wake_mailbox`]).
///
/// This function increments [`PROF_CROSS_CPU_RUNQ_DIRECT_ENQUEUE`] in all
/// build configurations whenever a violation is detected, and additionally
/// fires a [`debug_assert`] in debug builds.
#[inline]
fn debug_assert_runq_cpu_is_local<R: BootRuntime>(target_cpu: usize) {
    let current = crate::runtime::<R>().current_cpu_index();
    if target_cpu != current {
        PROF_CROSS_CPU_RUNQ_DIRECT_ENQUEUE.fetch_add(1, Ordering::Relaxed);
        #[cfg(all(debug_assertions, not(test)))]
        debug_assert_eq!(
            target_cpu, current,
            "SCHED ownership violation: CPU {} attempted direct enqueue into CPU {}'s run queue; \
             use enqueue_remote_wake_mailbox() for cross-CPU operations",
            current, target_cpu
        );
    }
}

#[inline]
pub(crate) fn scheduler_lock_held_by_this_cpu<R: BootRuntime>() -> bool {
    let owner = SCHEDULER_LOCK_OWNER.load(Ordering::Acquire);
    owner == crate::runtime::<R>().current_cpu_index() as isize
}

/// Number of histogram buckets used for hold/wait time distributions.
/// Boundaries (µs): <1, 1–10, 10–100, 100–1000, ≥1000
pub const SCHED_HIST_BUCKETS: usize = 5;
const PREPARE_SCHEDULE_PICK_BUDGET: usize = 16;
const PREPARE_SCHEDULE_FAIR_SCAN_DEPTH_PER_PRIORITY: usize = 8;
const PREPARE_SCHEDULE_MISROUTE_REPAIR_BUDGET: usize = 8;
const PREPARE_SCHEDULE_MISROUTE_BACKLOG_CAP: usize = 128;
// Keep steal scans bounded to limit idle-path latency while still peeking past
// a small pinned/unstealable head segment.
const STEAL_SCAN_DEPTH_PER_PRIORITY: usize = 8;
/// Minimum run-queue depth on a victim CPU before it becomes a steal target.
///
/// A threshold of 2 means we only steal when there is genuine imbalance: the
/// victim already has one task running plus at least one waiting.
const STEAL_MIN_VICTIM_DEPTH: usize = 2;
/// CPU-index radius that defines "nearby" CPUs for steal ordering.
///
/// CPUs whose index falls within `[local - STEAL_NEARBY_RADIUS, local +
/// STEAL_NEARBY_RADIUS]` (inclusive, clamped to valid range) are tried first
/// during `idle_steal`.  This is a topology heuristic: nearby indices are
/// often on the same package or share last-level cache, so stealing from them
/// tends to have lower cache-miss overhead than stealing from distant CPUs.
const STEAL_NEARBY_RADIUS: usize = 4;
#[inline]
pub(crate) fn cross_cpu_runq_migration_enabled() -> bool {
    false
}
// Allow local wake routing for Any-affinity tasks when the previous CPU is
// meaningfully busier, while still preserving cache locality under similar load.
const ANY_WAKE_LOCAL_DEPTH_BIAS: usize = 1;
const TERMINATE_CURRENT_SWITCH_RETRY_BUDGET: usize = 32;
const RUNQ_GLOBAL_TELEMETRY_SAMPLE_STRIDE: u64 = 64;
const RESCHED_IPI_NEVER_SENT: u64 = u64::MAX;
const RESCHED_IPI_MIN_TICK_DELTA: u64 = 2;

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
fn us_latency_hist_bucket(us: u64) -> usize {
    match us {
        0..=4 => 0,
        5..=19 => 1,
        20..=99 => 2,
        100..=499 => 3,
        _ => 4,
    }
}

#[inline]
fn idle_episode_hist_bucket(us: u64) -> usize {
    match us {
        0..=4 => 0,
        5..=49 => 1,
        50..=499 => 2,
        _ => 3,
    }
}

#[inline]
/// Return weighted vruntime debt accrued for one scheduler tick.
///
/// Lower numeric deltas for higher priorities approximate weighted fair service:
/// higher-priority tasks accumulate debt more slowly, while lower-priority tasks
/// pay debt faster. The progression is intentionally coarse and power-of-two so
/// it is cheap on the hot tick path while still differentiating priorities.
fn vruntime_tick_delta(priority: TaskPriority) -> u64 {
    match priority {
        TaskPriority::Realtime => 1,
        TaskPriority::High => 2,
        TaskPriority::Normal => 4,
        TaskPriority::Low => 8,
        TaskPriority::Idle => 16,
    }
}

/// Legacy pick-candidate comparison helper.
///
/// This function has been superseded by
/// [`crate::sched::policy::is_better_pick_candidate`], which is called from
/// [`crate::sched::policy::DefaultPolicy::pick_next_task`].  It is retained
/// here to avoid breaking any code that may reference it directly.
#[allow(dead_code)]
#[inline]
fn better_fair_pick_candidate(
    eff: usize,
    vruntime: u64,
    queue_idx: usize,
    best_eff: usize,
    best_vruntime: u64,
    best_q: Option<usize>,
) -> bool {
    if eff > best_eff {
        return true;
    }
    if eff != best_eff {
        return false;
    }
    if vruntime < best_vruntime {
        return true;
    }
    vruntime == best_vruntime && best_q.is_some_and(|best_queue_idx| queue_idx < best_queue_idx)
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
static REMOTE_WAKE_MAILBOXES: [state::WakeMailbox; types::MAX_CPUS] =
    [const { state::WakeMailbox::new() }; types::MAX_CPUS];

static REMOTE_WAKE_MAILBOX_ENQUEUE_EPOCH: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};
static REMOTE_WAKE_MAILBOX_LAST_IPI_EPOCH: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};
pub static DIAG_REMOTE_WAKE_MAILBOX_NO_IPI: AtomicU64 = AtomicU64::new(0);
const REMOTE_WAKE_MAILBOX_AGE_HIST_BUCKETS: usize = 5;
pub static PROF_REMOTE_WAKE_MAILBOX_AGE_HIST: [AtomicU64; REMOTE_WAKE_MAILBOX_AGE_HIST_BUCKETS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; REMOTE_WAKE_MAILBOX_AGE_HIST_BUCKETS]
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
static ANY_WAKE_OVERLOAD_POLICY: AtomicU8 = AtomicU8::new(AnyWakeOverloadPolicy::Steal as u8);
static ANY_WAKE_OVERLOAD_GAP: AtomicUsize = AtomicUsize::new(2);
// The streak counter storage is AtomicU8, but clamp APIs operate on usize.
const ANY_WAKE_OVERLOAD_STREAK_MAX: usize = u8::MAX as usize;
static ANY_WAKE_OVERLOAD_STREAK_REQUIRED: AtomicUsize = AtomicUsize::new(1);
static ANY_WAKE_OVERLOAD_STREAK: [AtomicU8; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU8 = AtomicU8::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};
#[cfg(test)]
static TEST_LEAST_LOADED_ONLINE_CPU_CALLS: AtomicU64 = AtomicU64::new(0);

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
    state.per_cpu.get(cpu).map(|pc| pc.runq.total_len()).unwrap_or(0)
}

fn least_loaded_online_cpu(state: &crate::sched::state::SchedState) -> Option<(usize, usize)> {
    #[cfg(test)]
    TEST_LEAST_LOADED_ONLINE_CPU_CALLS.fetch_add(1, Ordering::Relaxed);
    state
        .online_cpus
        .iter()
        .copied()
        .filter(|&cpu| cpu < state.per_cpu.len())
        .map(|cpu| (cpu, runq_depth_for_cpu(state, cpu)))
        .min_by_key(|&(_, depth)| depth)
}

struct WakeBatchLoadSnapshot {
    per_cpu_depths: alloc::vec::Vec<usize>,
}

impl WakeBatchLoadSnapshot {
    fn new(state: &crate::sched::state::SchedState) -> Self {
        let mut per_cpu_depths = alloc::vec::Vec::with_capacity(state.per_cpu.len());
        for cpu in 0..state.per_cpu.len() {
            per_cpu_depths.push(runq_depth_for_cpu(state, cpu));
        }
        Self { per_cpu_depths }
    }

    #[inline]
    fn depth_for_cpu(&self, cpu: usize) -> usize {
        self.per_cpu_depths.get(cpu).copied().unwrap_or(0)
    }

    fn least_loaded_online_cpu(
        &self,
        state: &crate::sched::state::SchedState,
    ) -> Option<(usize, usize)> {
        state
            .online_cpus
            .iter()
            .copied()
            .filter(|&cpu| cpu < self.per_cpu_depths.len())
            .map(|cpu| (cpu, self.depth_for_cpu(cpu)))
            .min_by_key(|&(_, depth)| depth)
    }

    #[inline]
    fn note_enqueue(&mut self, cpu: usize) {
        if let Some(depth) = self.per_cpu_depths.get_mut(cpu) {
            *depth = depth.saturating_add(1);
        }
    }
}

/// Returns `true` if `cpu` is currently running its idle task.
///
/// A CPU is considered idle when both its `current` and `idle_task` slots are
/// initialized *and* they refer to the same task ID.  Uninitialized CPUs
/// (either field is `None`) are treated as non-idle to avoid spurious routing
/// during early boot — the `unwrap_or(false)` ensures that a CPU with no
/// per-CPU entry (e.g. an out-of-range index) is likewise treated as non-idle
/// rather than panicking.
fn is_cpu_idle(state: &crate::sched::state::SchedState, cpu: usize) -> bool {
    state
        .per_cpu
        .get(cpu)
        .map(|pc| matches!((pc.current, pc.idle_task), (Some(cur), Some(idle)) if cur == idle))
        .unwrap_or(false)
}

/// Find any idle online CPU.
///
/// Returns the index of the first online CPU whose `current` task is its
/// `idle_task`, or `None` if no online CPU is currently idle.  The scan
/// order follows `state.online_cpus` (sorted ascending by CPU index).
///
/// The `cpu < state.per_cpu.len()` bounds check guards against transient
/// states where `online_cpus` contains an index that was registered before
/// the corresponding `per_cpu` slot was pushed (e.g. during early SMP
/// bring-up).  In steady state the two collections are always in sync, so
/// the check is purely defensive and never eliminates a valid candidate.
fn find_idle_online_cpu(state: &crate::sched::state::SchedState) -> Option<usize> {
    state
        .online_cpus
        .iter()
        .copied()
        .find(|&cpu| cpu < state.per_cpu.len() && is_cpu_idle(state, cpu))
}

/// Select the target CPU for an `Affinity::Any` task wakeup.
///
/// This is the canonical entry point for wake CPU selection.  The policy
/// applies in order:
///
/// 1. **Locality** — prefer `last_cpu` unless the local CPU is meaningfully
///    less loaded (see `select_preferred_any_affinity_wake_cpu`).
/// 2. **Idle CPU** — if the preferred CPU has at least `overload_gap` tasks
///    queued *and* an idle online CPU is available, route to the idle CPU
///    rather than adding to an already-busy queue.
/// 3. **Least-loaded** — fall back to the overload-aware redirect in
///    `select_any_affinity_wake_cpu`, which steers to the least-loaded
///    online CPU when the preferred one is overloaded.
pub(crate) fn choose_wake_cpu<R: BootRuntime>(
    sched: &types::Scheduler<R>,
    last_cpu: Option<usize>,
) -> usize {
    if !cross_cpu_runq_migration_enabled() {
        let per_cpu_len = sched.state.per_cpu.len();
        if per_cpu_len == 0 {
            return 0;
        }
        let current_cpu = current_cpu_index::<R>().min(per_cpu_len - 1);
        return last_cpu.filter(|&cpu| cpu < per_cpu_len).unwrap_or(current_cpu);
    }

    let preferred = select_preferred_any_affinity_wake_cpu::<R>(sched, last_cpu);

    let overload_gap = ANY_WAKE_OVERLOAD_GAP.load(Ordering::Acquire);
    let preferred_depth = runq_depth_for_cpu(&sched.state, preferred);
    if preferred_depth >= overload_gap {
        if let Some(idle_cpu) = find_idle_online_cpu(&sched.state) {
            if idle_cpu != preferred {
                return idle_cpu;
            }
        }
    }

    select_any_affinity_wake_cpu::<R>(sched, preferred)
}

/// Snapshot-aware variant of [`choose_wake_cpu`] for batch wakeup paths.
///
/// Uses a pre-captured [`WakeBatchLoadSnapshot`] for run-queue depth queries
/// instead of reading live per-CPU state on every call.  Idle CPU detection
/// still reads live per-CPU state because idle status is not captured in the
/// snapshot (it changes infrequently relative to queue depths).
///
/// **Consistency note**: because idle detection bypasses the snapshot, a
/// CPU that transitions from idle to running between the snapshot capture
/// and the idle check may still be selected as the idle target.  This is
/// benign — the woken task will simply find a now-running CPU and compete
/// normally, which is no worse than any other Any-affinity placement.
fn choose_wake_cpu_from_snapshot<R: BootRuntime>(
    sched: &types::Scheduler<R>,
    last_cpu: Option<usize>,
    load_snapshot: &WakeBatchLoadSnapshot,
) -> usize {
    if !cross_cpu_runq_migration_enabled() {
        let per_cpu_len = sched.state.per_cpu.len();
        if per_cpu_len == 0 {
            return 0;
        }
        let current_cpu = current_cpu_index::<R>().min(per_cpu_len - 1);
        return last_cpu.filter(|&cpu| cpu < per_cpu_len).unwrap_or(current_cpu);
    }

    let preferred =
        select_preferred_any_affinity_wake_cpu_from_snapshot::<R>(sched, last_cpu, load_snapshot);

    let overload_gap = ANY_WAKE_OVERLOAD_GAP.load(Ordering::Acquire);
    let preferred_depth = load_snapshot.depth_for_cpu(preferred);
    if preferred_depth >= overload_gap {
        if let Some(idle_cpu) = find_idle_online_cpu(&sched.state) {
            if idle_cpu != preferred {
                return idle_cpu;
            }
        }
    }

    select_any_affinity_wake_cpu_from_snapshot::<R>(sched, preferred, load_snapshot)
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

/// Per-CPU count of timer-triggered try-lock misses within the current warning window.
static TRYLOCK_MISS_WINDOW_TIMER_COUNT: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};

/// Per-CPU count of IPI-triggered try-lock misses within the current warning window.
static TRYLOCK_MISS_WINDOW_IPI_COUNT: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};

/// Per-CPU count of try-lock misses that happened on timer ticks while this CPU
/// was already running its idle task and had no pending reschedule request.
static TRYLOCK_MISS_WINDOW_IDLE_TIMER_COUNT: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
    [ATOMIC_ZERO; types::MAX_CPUS]
};

/// Per-CPU count of try-lock misses observed while a reschedule request was
/// already pending for this CPU.
static TRYLOCK_MISS_WINDOW_PENDING_COUNT: [AtomicU64; types::MAX_CPUS] = {
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
pub const TRYLOCK_MISS_WARN_THRESHOLD: u64 = 1000;

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
    _hold_hist: &[AtomicU64; SCHED_HIST_BUCKETS],
    wait_calls: &AtomicU64,
    wait_total: &AtomicU64,
    wait_max: &AtomicU64,
    _wait_hist: &[AtomicU64; SCHED_HIST_BUCKETS],
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
fn global_need_resched_slot(cpu: usize) -> Option<&'static AtomicBool> {
    GLOBAL_NEED_RESCHED.get(cpu)
}

#[inline]
/// Run a bool-returning operation against `GLOBAL_NEED_RESCHED[cpu]` with
/// bounds checks and a conservative fallback on invalid indices.
///
/// Returning `true` on invalid indices intentionally errs toward preserving
/// pending-reschedule demand instead of under-reporting it.
fn checked_global_need_resched_bool(
    cpu: usize,
    op: &'static str,
    f: impl FnOnce(&AtomicBool) -> bool,
) -> bool {
    if let Some(slot) = global_need_resched_slot(cpu) {
        f(slot)
    } else {
        crate::kerror!(
            "Sched: GLOBAL_NEED_RESCHED {} ignored for invalid CPU {} (MAX={})",
            op,
            cpu,
            types::MAX_CPUS
        );
        // Conservative fallback: assume reschedule is already/still needed.
        true
    }
}

#[inline]
fn global_need_resched_load(cpu: usize, ordering: Ordering) -> bool {
    checked_global_need_resched_bool(cpu, "load", |slot| slot.load(ordering))
}

#[inline]
fn global_need_resched_swap(cpu: usize, value: bool, ordering: Ordering) -> bool {
    let op = if value { "swap(true)" } else { "swap(false)" };
    checked_global_need_resched_bool(cpu, op, |slot| slot.swap(value, ordering))
}

#[inline]
fn clear_global_need_resched(cpu: usize, ordering: Ordering) {
    if let Some(slot) = global_need_resched_slot(cpu) {
        slot.store(false, ordering);
    } else {
        crate::kerror!(
            "Sched: GLOBAL_NEED_RESCHED store(false) ignored for invalid CPU {} (MAX={})",
            cpu,
            types::MAX_CPUS
        );
    }
}

#[inline]
pub(crate) fn set_global_need_resched(cpu: usize) -> bool {
    let was_set = global_need_resched_swap(cpu, true, Ordering::Release);
    if was_set {
        PROF_RESCHED_COALESCED.fetch_add(1, Ordering::Relaxed);
    }
    was_set
}

/// Returns `true` if the local CPU has a pending reschedule request.
///
/// This is a **lock-free** fast-path check that reads the per-CPU atomic
/// `need_resched` flag without acquiring the global scheduler lock.  Callers
/// (e.g. `resched_if_needed`, `preempt_enable`) use this to skip the
/// expensive lock acquisition when no reschedule is pending.
///
/// The flag is set by:
/// * the timer tick handler (timeslice expiry, lock-skip self-healing),
/// * remote-wake mailbox drain (higher-priority task woken on this CPU),
/// * `wake_sleepers` / misrouted-requeue paths, and
/// * `set_global_need_resched` from any cross-CPU delivery path.
#[inline]
pub fn need_resched_pending(cpu: usize) -> bool {
    global_need_resched_load(cpu, Ordering::Acquire)
}

#[inline]
pub(crate) fn enqueue_remote_wake_mailbox(
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
fn take_remote_wake_mailbox(
    cpu: usize,
) -> alloc::collections::VecDeque<types::RemoteWakeMailboxEntry> {
    if cpu >= types::MAX_CPUS {
        return alloc::collections::VecDeque::new();
    }
    REMOTE_WAKE_MAILBOXES[cpu].drain()
}

#[cfg(test)]
fn reset_remote_wake_mailboxes_for_tests() {
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

/// Sample the run-queue depth for `cpu` and update the last/max statics.
/// This is a no-op when the `sched_telemetry` feature is disabled so that
/// the per-schedule-point iteration incurs zero overhead in normal builds.
#[inline]
pub(crate) fn sample_runq_len<R: BootRuntime>(sched: &mut types::Scheduler<R>, cpu: usize) {
    let mut sample_seq = 0u64;
    if let Some(pc) = sched.state.per_cpu.get_mut(cpu) {
        let len64 = pc.runq.total_len() as u64;
        sample_seq = PROF_RUNQ_SAMPLE_COUNT[cpu].fetch_add(1, Ordering::Relaxed) + 1;
        PROF_RUNQ_SAMPLE_TOTAL[cpu].fetch_add(len64, Ordering::Relaxed);
        pc.stats.runq_sample_count = pc.stats.runq_sample_count.saturating_add(1);
        pc.stats.runq_sample_total = pc.stats.runq_sample_total.saturating_add(len64);
        #[cfg(feature = "sched_telemetry")]
        {
            PROF_RUNQ_LEN_LAST[cpu].store(len64, Ordering::Relaxed);
            update_max_u64(&PROF_RUNQ_LEN_MAX[cpu], len64);
        }
    }
    if sample_seq == 0 || sample_seq % RUNQ_GLOBAL_TELEMETRY_SAMPLE_STRIDE != 0 {
        return;
    }
    let online = &sched.state.online_cpus;
    if !online.is_empty() {
        let mut sum = 0u64;
        let mut count = 0u64;
        let mut any_idle_cpu = false;
        let mut any_loaded_cpu = false;
        for &idx in online {
            if let Some(pc) = sched.state.per_cpu.get(idx) {
                let depth = pc.runq.total_len() as u64;
                sum = sum.saturating_add(depth);
                count = count.saturating_add(1);
                if pc.current == pc.idle_task {
                    any_idle_cpu = true;
                }
                if depth > 1 {
                    any_loaded_cpu = true;
                }
            }
        }
        if count > 0 {
            let mean = sum / count;
            let mut variance_sum = 0u64;
            for &idx in online {
                if let Some(pc) = sched.state.per_cpu.get(idx) {
                    let len = pc.runq.total_len() as u64;
                    let diff = len.abs_diff(mean);
                    variance_sum = variance_sum.saturating_add(diff.saturating_mul(diff));
                }
            }
            let variance = variance_sum / count;
            PROF_RUNQ_DEPTH_VARIANCE_LAST.store(variance, Ordering::Relaxed);
            update_max_u64(&PROF_RUNQ_DEPTH_VARIANCE_MAX, variance);
            PROF_RUNQ_DEPTH_VARIANCE_SAMPLE_COUNT.fetch_add(1, Ordering::Relaxed);
            PROF_RUNQ_DEPTH_VARIANCE_TOTAL.fetch_add(variance, Ordering::Relaxed);
        }
        let imbalance_active = any_idle_cpu && any_loaded_cpu;
        let now_mono = crate::runtime::<R>().mono_ticks();
        match (imbalance_active, sched.imbalance_active_since_mono) {
            (true, None) => {
                sched.imbalance_active_since_mono = Some(now_mono);
                sched.imbalance_episodes = sched.imbalance_episodes.saturating_add(1);
                PROF_IMBALANCE_EPISODES.fetch_add(1, Ordering::Relaxed);
            }
            (false, Some(start)) => {
                let elapsed_us = ticks_to_us::<R>(now_mono.wrapping_sub(start));
                sched.imbalance_total_us = sched.imbalance_total_us.saturating_add(elapsed_us);
                sched.imbalance_longest_us = sched.imbalance_longest_us.max(elapsed_us);
                PROF_IMBALANCE_TOTAL_US.fetch_add(elapsed_us, Ordering::Relaxed);
                update_max_u64(&PROF_IMBALANCE_LONGEST_US, elapsed_us);
                sched.imbalance_active_since_mono = None;
            }
            _ => {}
        }
    }
    #[cfg(not(feature = "sched_telemetry"))]
    let _ = (sched, cpu);
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

/// Called from timer ISR - records tick and triggers reschedule if needed
/// Uses try_resched_if_needed to avoid deadlock when SCHEDULER is held by main code
pub fn on_tick<R: BootRuntime>() {
    let cpu_idx = crate::runtime::<R>().current_cpu_index();
    let _ticks = if cpu_idx == 0 {
        TICK_COUNT.fetch_add(1, Ordering::Relaxed) + 1
    } else {
        TICK_COUNT.load(Ordering::Relaxed)
    };

    if cpu_idx == 0 {
        crate::vfs::devfs::ConsoleNode::poll_input();
        crate::time::maybe_log_system_clock_tick(crate::time::monotonic_now_ns());
    }

    DIAG_IPI_HANDLER.fetch_add(1, Ordering::Relaxed);

    try_resched_if_needed::<R>(DispatchTrigger::TimerTick);
    emit_debug_summary::<R>(cpu_idx);
}

fn emit_debug_summary<R: BootRuntime>(caller_cpu: usize) {
    if caller_cpu != 0 {
        return;
    }
    let rt = crate::runtime::<R>();
    let now = rt.mono_ticks();
    let interval = rt.mono_freq_hz().max(1);
    let last = profiling::LAST_DEBUG_SUMMARY_MONO.load(Ordering::Relaxed);
    if last != 0 && now.saturating_sub(last) < interval {
        return;
    }
    if profiling::LAST_DEBUG_SUMMARY_MONO
        .compare_exchange(last, now, Ordering::Relaxed, Ordering::Relaxed)
        .is_err()
    {
        return;
    }

    #[derive(Copy, Clone)]
    struct CpuStats {
        i: usize,
        curr: Option<crate::sched::TaskId>,
        runq: usize,
        runq_avg: u64,
        runq_samples: u64,
        ctxsw: u64,
        idle2busy: u64,
        tick: u64,
        ipi: u64,
        enq: u64,
        deq: u64,
        wake: u64,
        lock_miss: u64,
        lock_pending: u64,
        lock_blocked: u64,
    }
    let mut cpus_online = 0;
    let mut stats_buf = [core::mem::MaybeUninit::<CpuStats>::uninit(); 32];
    let mut num_stats = 0;

    {
        let Some(lock) = SCHEDULER.try_lock() else {
            return;
        };
        let Some(ptr) = *lock else {
            return;
        };
        // SAFETY: `ptr` is written from `init::<R>` and remains valid for kernel
        // lifetime; this function only reads scheduler state under SCHEDULER lock.
        let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
        cpus_online = sched.state.online_cpu_count;
        for &i in &sched.state.online_cpus {
            if num_stats >= stats_buf.len() {
                break;
            }
            let pc = &sched.state.per_cpu[i];
            let runq: usize = pc.runq.total_len();
            let runq_avg = if pc.stats.runq_sample_count == 0 {
                0
            } else {
                pc.stats.runq_sample_total / pc.stats.runq_sample_count
            };
            stats_buf[num_stats].write(CpuStats {
                i,
                curr: pc.current,
                runq,
                runq_avg,
                runq_samples: pc.stats.runq_sample_count,
                ctxsw: pc.stats.context_switches,
                idle2busy: pc.stats.idle_to_nonidle,
                tick: pc.stats.timer_interrupts,
                ipi: pc.stats.resched_ipi_received,
                enq: pc.stats.runnable_enqueues,
                deq: pc.stats.runnable_dequeues,
                wake: pc.stats.wakeups,
                lock_miss: pc.stats.lock_trylock_misses,
                lock_pending: pc.stats.lock_trylock_misses_with_pending_resched,
                lock_blocked: pc.stats.lock_blocked_dispatch,
            });
            num_stats += 1;
        }
    } // drop lock

    crate::ktrace!("SCHED-DBG: cpus_online={}", cpus_online);
    for idx in 0..num_stats {
        let s = unsafe { stats_buf[idx].assume_init_ref() };
        crate::ktrace!(
            "SCHED-DBG: cpu={} curr={:?} runq={} runq_avg={} runq_samples={} ctxsw={} idle2busy={} tick={} ipi={} enq={} deq={} wake={} lock_miss={} lock_pending={} lock_blocked={}",
            s.i,
            s.curr,
            s.runq,
            s.runq_avg,
            s.runq_samples,
            s.ctxsw,
            s.idle2busy,
            s.tick,
            s.ipi,
            s.enq,
            s.deq,
            s.wake,
            s.lock_miss,
            s.lock_pending,
            s.lock_blocked
        );
    }
}

/// Called from IPI handler - triggers reschedule without advancing time
pub fn on_resched_ipi<R: BootRuntime>() {
    DIAG_IPI_HANDLER.fetch_add(1, Ordering::Relaxed);
    try_resched_if_needed::<R>(DispatchTrigger::ReschedIpi);
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum DispatchTrigger {
    TimerTick,
    ReschedIpi,
}

impl DispatchTrigger {
    #[inline]
    fn schedule_reason(self) -> ScheduleReason {
        match self {
            DispatchTrigger::TimerTick => ScheduleReason::PreemptTick,
            DispatchTrigger::ReschedIpi => ScheduleReason::ReschedIfNeeded,
        }
    }

    /// Stable trigger label used in scheduler contention diagnostics.
    #[inline]
    fn as_str(self) -> &'static str {
        match self {
            DispatchTrigger::TimerTick => "timer_tick",
            DispatchTrigger::ReschedIpi => "resched_ipi",
        }
    }
}

/// Interrupt-safe version of resched_if_needed - uses try_lock to avoid deadlock
/// If SCHEDULER lock is contended, simply skip rescheduling this tick
fn try_resched_if_needed<R: BootRuntime>(trigger: DispatchTrigger) {
    let rt = crate::runtime::<R>();
    let irq = rt.irq_disable();
    let cpu_idx = rt.current_cpu_index();
    let mut lock = None;
    let mut attempts = 0;
    while attempts < 1 {
        if let Some(l) = SCHEDULER.try_lock() {
            lock = Some(l);
            break;
        }
        attempts += 1;
        core::hint::spin_loop();
    }

    if lock.is_none() {
        let owner = SCHEDULER_LOCK_OWNER.load(Ordering::Acquire);
        let acquired_at = SCHEDULER_LOCK_ACQUIRED_AT.load(Ordering::Acquire);
        // TICK_COUNT increments at 100Hz. 200 ticks = 2 seconds.
        let now = TICK_COUNT.load(Ordering::Relaxed) + 1;
        let held_duration = if acquired_at > 0 { now.saturating_sub(acquired_at) } else { 0 };

        // WATCHDOG: Detect if the lock has been held for an implausibly long time.
        // If it's held > 2 seconds, we likely have a deadlock or a lock leak.
        if owner != -1 && acquired_at != 0 && held_duration > 200 {
            panic!(
                "SCHEDULER LOCK WATCHDOG: Lock held by CPU {} for {} ticks ({} ms) - potential DEADLOCK",
                owner,
                held_duration,
                held_duration * 10
            );
        }
    }

    if let Some(lock) = lock {
        set_sched_lock_tracking::<R>(cpu_idx);
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
            if let Some(pc) = sched.state.per_cpu.get_mut(cpu_idx) {
                match trigger {
                    DispatchTrigger::TimerTick => {
                        pc.stats.timer_interrupts = pc.stats.timer_interrupts.saturating_add(1);
                        if cpu_idx < types::MAX_CPUS && pc.current == pc.idle_task {
                            PROF_IDLE_TICKS_PER_CPU[cpu_idx].fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    DispatchTrigger::ReschedIpi => {
                        pc.stats.resched_ipi_received =
                            pc.stats.resched_ipi_received.saturating_add(1);
                    }
                }
            }
            let current = sched.state.per_cpu.get(cpu_idx).and_then(|pc| pc.current);
            let idle = sched.state.per_cpu.get(cpu_idx).and_then(|pc| pc.idle_task);
            let runq_total =
                sched.state.per_cpu.get(cpu_idx).map(|pc| pc.runq.total_len()).unwrap_or(0);
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
                    || global_need_resched_load(cpu_idx, Ordering::Acquire);
            let switch = sched.schedule_point(trigger.schedule_reason());
            // Drain IPIs deferred by wake_sleepers while the SCHEDULER lock is
            // still held, so we can send them after releasing the lock.
            let deferred_ipis = core::mem::take(&mut sched.pending_wake_ipis);
            // Drain IPIs deferred by prepare_schedule misroute handling while
            // the lock is still held, so we can send them after unlock.
            let deferred_prepare_ipis = sched.drain_pending_prepare_schedule_ipis();
            let deferred_registry_syncs = core::mem::take(&mut sched.pending_registry_syncs);
            if let Some(switch_decision) = switch {
                // Must drop lock before context switch!
                clear_sched_lock_tracking::<R>();
                drop(lock);

                // Send deferred wake-sleeper IPIs now that the lock is released.
                for cpu in deferred_ipis {
                    DIAG_IPI_SENT.fetch_add(1, Ordering::Relaxed);
                    DIAG_IPI_SENT_WAKE_SLEEPERS.fetch_add(1, Ordering::Relaxed);
                    rt.send_ipi(cpu, 0x30);
                }
                send_deferred_prepare_schedule_ipis::<R>(deferred_prepare_ipis);
                apply_deferred_registry_syncs::<R>(deferred_registry_syncs);
                let mut ghost_ctx = <R::Tasking as BootTasking>::Context::default();
                let mut ghost_fs_base = 0;
                let Some(switch) =
                    resolve_switch_params::<R>(switch_decision, &mut ghost_ctx, &mut ghost_fs_base)
                else {
                    rt.irq_restore(irq);
                    return;
                };

                // Update CPU_CURRENT_TASK *before* the switch so other CPUs
                // never see a stale entry for the outgoing task.  This
                // eliminates the cross-CPU deadlock where two CPUs spin on
                // is_task_on_any_cpu with IRQs disabled, each waiting for
                // the other's post-switch update.
                crate::sched::set_cpu_current_task(cpu_idx, switch.to_tid);

                // Bounded safety check: the target should no longer appear
                // as current on any *other* CPU.  A brief spin covers the
                // window where a remote CPU is between its own pre-switch
                // update and switch_with_tls.
                let mut _spins = 0u32;
                while crate::sched::is_task_on_other_cpu(switch.to_tid, cpu_idx) {
                    _spins += 1;
                    if _spins > 10_000 {
                        break;
                    }
                    core::hint::spin_loop();
                }

                if switch.to_aspace != switch.from_aspace {
                    rt.tasking().activate_address_space(switch.to_aspace);
                }

                unsafe {
                    rt.tasking().switch_with_tls(
                        &mut *switch.from_ctx,
                        &*switch.to_ctx,
                        switch.to_tid,
                        switch.from_user_fs_base,
                        switch.to_user_fs_base,
                    );
                }

                // Post-switch: the resumed task updates tracking for
                // whichever CPU it is now running on (may differ from
                // the CPU it was suspended on).
                crate::sched::set_cpu_current_task(
                    crate::runtime::<R>().current_cpu_index(),
                    crate::runtime::<R>().current_tid(),
                );
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
                            .map(|pc| pc.runq.has_higher_priority_work(current_prio))
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
                clear_sched_lock_tracking::<R>();
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
        } else {
            clear_sched_lock_tracking::<R>();
            drop(lock);
        }
    } else {
        PROF_RESCHED_TRYLOCK_MISS.fetch_add(1, Ordering::Relaxed);
        PROF_TRYLOCK_MISS_PER_CPU[cpu_idx].fetch_add(1, Ordering::Relaxed);
        match trigger {
            DispatchTrigger::TimerTick => {
                PROF_TRYLOCK_MISS_TIMER_PER_CPU[cpu_idx].fetch_add(1, Ordering::Relaxed);
            }
            DispatchTrigger::ReschedIpi => {
                PROF_TRYLOCK_MISS_IPI_PER_CPU[cpu_idx].fetch_add(1, Ordering::Relaxed);
            }
        }

        // WATCHDOG: Detect if the lock has been held for an implausibly long time.
        // If it's held > 2 seconds, we likely have a deadlock or a lock leak.
        let now = TICK_COUNT.load(Ordering::Relaxed) + 1;
        let owner = SCHEDULER_LOCK_OWNER.load(Ordering::Acquire);
        let acquired_at = SCHEDULER_LOCK_ACQUIRED_AT.load(Ordering::Acquire);

        // TICK_COUNT increments at 100Hz. 200 ticks = 2 seconds.
        if owner != -1 && acquired_at != 0 && now.saturating_sub(acquired_at) > 200 {
            panic!(
                "SCHEDULER LOCK WATCHDOG: Lock held by CPU {} for {} ticks ({} ms) - potential DEADLOCK",
                owner,
                now.saturating_sub(acquired_at),
                now.saturating_sub(acquired_at) * 10
            );
        }

        let pending_resched = global_need_resched_load(cpu_idx, Ordering::Acquire);
        if pending_resched {
            PROF_TRYLOCK_MISS_PENDING_PER_CPU[cpu_idx].fetch_add(1, Ordering::Relaxed);
        }
        let idle_timer_miss =
            trigger == DispatchTrigger::TimerTick && rt.is_idle_task_current() && !pending_resched;
        if idle_timer_miss {
            PROF_TRYLOCK_MISS_IDLE_TIMER_PER_CPU[cpu_idx].fetch_add(1, Ordering::Relaxed);
        }
        // Warn only when misses cross threshold in a 2-second per-CPU window.
        let now = rt.mono_ticks();
        let window_ticks = rt.mono_freq_hz().max(1).saturating_mul(2);
        let window_start = &TRYLOCK_MISS_WINDOW_START[cpu_idx];
        let window_count = &TRYLOCK_MISS_WINDOW_COUNT[cpu_idx];
        let window_timer_count = &TRYLOCK_MISS_WINDOW_TIMER_COUNT[cpu_idx];
        let window_ipi_count = &TRYLOCK_MISS_WINDOW_IPI_COUNT[cpu_idx];
        let window_idle_timer_count = &TRYLOCK_MISS_WINDOW_IDLE_TIMER_COUNT[cpu_idx];
        let window_pending_count = &TRYLOCK_MISS_WINDOW_PENDING_COUNT[cpu_idx];
        let is_actionable_miss = !idle_timer_miss;

        let start = window_start.load(Ordering::Relaxed);
        if start == 0 || now.saturating_sub(start) > window_ticks {
            window_start.store(now, Ordering::Relaxed);
            window_count.store(if is_actionable_miss { 1 } else { 0 }, Ordering::Relaxed);
            match trigger {
                DispatchTrigger::TimerTick => {
                    window_timer_count.store(1, Ordering::Relaxed);
                    window_ipi_count.store(0, Ordering::Relaxed);
                }
                DispatchTrigger::ReschedIpi => {
                    window_timer_count.store(0, Ordering::Relaxed);
                    window_ipi_count.store(1, Ordering::Relaxed);
                }
            }
            window_idle_timer_count.store(if idle_timer_miss { 1 } else { 0 }, Ordering::Relaxed);
            window_pending_count.store(if pending_resched { 1 } else { 0 }, Ordering::Relaxed);
        } else {
            match trigger {
                DispatchTrigger::TimerTick => {
                    window_timer_count.fetch_add(1, Ordering::Relaxed);
                }
                DispatchTrigger::ReschedIpi => {
                    window_ipi_count.fetch_add(1, Ordering::Relaxed);
                }
            }
            if idle_timer_miss {
                window_idle_timer_count.fetch_add(1, Ordering::Relaxed);
            } else {
                let misses = window_count.fetch_add(1, Ordering::Relaxed) + 1;
                if misses == TRYLOCK_MISS_WARN_THRESHOLD {
                    let cooldown_ticks =
                        rt.mono_freq_hz().max(1).saturating_mul(TRYLOCK_MISS_WARN_COOLDOWN_SECS);
                    let last_warn = TRYLOCK_MISS_LAST_WARN_TICK[cpu_idx].load(Ordering::Relaxed);
                    if last_warn == 0 || now.saturating_sub(last_warn) >= cooldown_ticks {
                        TRYLOCK_MISS_LAST_WARN_TICK[cpu_idx].store(now, Ordering::Relaxed);
                        let window_timer = window_timer_count.load(Ordering::Relaxed);
                        let window_ipi = window_ipi_count.load(Ordering::Relaxed);
                        let window_idle_timer = window_idle_timer_count.load(Ordering::Relaxed);
                        let window_pending = window_pending_count.load(Ordering::Relaxed);
                        crate::kdebug!(
                            "SCHED: CPU {} resched try_lock actionable misses (excluding idle timer-only misses) reached {} in 2s (timer={} ipi={} pending={} idle_timer={} last_trigger={} suppressing until window reset)",
                            cpu_idx,
                            TRYLOCK_MISS_WARN_THRESHOLD,
                            window_timer,
                            window_ipi,
                            window_pending,
                            window_idle_timer,
                            trigger.as_str(),
                        );
                    }
                }
            }
            if pending_resched {
                window_pending_count.fetch_add(1, Ordering::Relaxed);
            }
        }
        // Self-healing: tell the next safe point to reschedule, except for
        // timer-tick lock misses while this CPU is already idle and has no
        // pending reschedule signal.
        if !idle_timer_miss {
            set_global_need_resched(cpu_idx);
        }
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
        if !should_send_remote_resched_ipi(cpu) {
            continue;
        }
        DIAG_IPI_SENT.fetch_add(1, Ordering::Relaxed);
        DIAG_IPI_SENT_PREPARE_SCHEDULE.fetch_add(1, Ordering::Relaxed);
        rt.send_ipi(cpu, 0x30);
    }
}

#[inline]
pub(crate) fn should_send_remote_resched_ipi(target_cpu: usize) -> bool {
    if target_cpu >= types::MAX_CPUS {
        return true;
    }
    let now_tick = TICK_COUNT.load(Ordering::Relaxed);
    let last_tick = profiling::LAST_RESCHED_IPI_SENT_AT_TICK[target_cpu].load(Ordering::Relaxed);
    if last_tick != RESCHED_IPI_NEVER_SENT
        && now_tick.saturating_sub(last_tick) < RESCHED_IPI_MIN_TICK_DELTA
    {
        PROF_IPI_SUPPRESSED.fetch_add(1, Ordering::Relaxed);
        return false;
    }
    profiling::LAST_RESCHED_IPI_SENT_AT_TICK[target_cpu].store(now_tick, Ordering::Relaxed);
    true
}

pub(crate) fn apply_deferred_registry_syncs<R: BootRuntime>(
    deferred_updates: alloc::vec::Vec<types::DeferredRegistrySync>,
) {
    debug_assert_scheduler_not_held_by_this_cpu::<R>("apply_deferred_registry_syncs");
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

pub(crate) fn apply_deferred_registry_inserts<R: BootRuntime>(
    deferred_inserts: alloc::vec::Vec<alloc::boxed::Box<crate::task::Task<R>>>,
) {
    if deferred_inserts.is_empty() {
        return;
    }
    debug_assert_scheduler_not_held_by_this_cpu::<R>("apply_deferred_registry_inserts");
    crate::kdebug!("REGISTRY: Applying {} deferred inserts", deferred_inserts.len());
    let mut registry = crate::task::registry::get_registry::<R>();
    for task in deferred_inserts {
        crate::kdebug!("REGISTRY: Inserting TID={}", task.id);
        registry.insert(task);
    }
    crate::kdebug!("REGISTRY: Inserts applied");
}

pub(crate) fn resolve_switch_params<R: BootRuntime>(
    decision: SwitchDecision,
    ghost_ctx: &mut <R::Tasking as BootTasking>::Context,
    ghost_fs_base: &mut u64,
) -> Option<
    SwitchParams<<R::Tasking as BootTasking>::Context, <R::Tasking as BootTasking>::AddressSpace>,
> {
    debug_assert_scheduler_not_held_by_this_cpu::<R>("resolve_switch_params");
    let mut registry = crate::task::registry::get_registry::<R>();
    let from_idx = registry.get_index(decision.from_tid);
    let to_idx = registry.get_index(decision.to_tid)?;

    if let Some(from_idx) = from_idx {
        let (from_task, to_task) = if from_idx < to_idx {
            let (left, right) = registry.threads.split_at_mut(to_idx);
            (&mut left[from_idx], &mut right[0])
        } else if from_idx > to_idx {
            let (left, right) = registry.threads.split_at_mut(from_idx);
            (&mut right[0], &mut left[to_idx])
        } else {
            // from_tid == to_tid (should have been caught in prepare_schedule)
            return None;
        };

        // CURRENT_MAPPINGS is currently typed as a mutable raw pointer for
        // historical compatibility, but the stored Arc target is treated as
        // read-only by mapping-check fast paths unless they take the mapping lock.
        crate::sched::vm::CURRENT_MAPPINGS[decision.cpu_idx]
            .store(alloc::sync::Arc::as_ptr(&to_task.mappings) as *mut _, Ordering::Release);

        from_task.simd.save(crate::runtime::<R>());
        to_task.simd.restore(crate::runtime::<R>());

        crate::trace::irq_ring::push(abi::trace::TraceEvent::ContextSwitch {
            from: from_task.id,
            to: to_task.id,
            timestamp: crate::trace::now(),
        });

        Some(SwitchParams {
            from_ctx: &mut from_task.ctx,
            to_ctx: &to_task.ctx,
            to_aspace: to_task.aspace,
            from_aspace: from_task.aspace,
            from_tid: from_task.id,
            to_tid: to_task.id,
            from_user: from_task.is_user,
            to_user: to_task.is_user,
            from_user_fs_base: &mut from_task.user_fs_base,
            to_user_fs_base: to_task.user_fs_base,
        })
    } else {
        // The outgoing task was already reaped (likely by another CPU).
        // Use the provided ghost storage to avoid saving into a dropped Thread struct.
        crate::kdebug!(
            "SCHED: from_tid {} reaped during switch on CPU {}, using ghost storage (to={})",
            decision.from_tid,
            decision.cpu_idx,
            decision.to_tid
        );
        let to_task = &mut registry.threads[to_idx];

        crate::sched::vm::CURRENT_MAPPINGS[decision.cpu_idx]
            .store(alloc::sync::Arc::as_ptr(&to_task.mappings) as *mut _, Ordering::Release);
        to_task.simd.restore(crate::runtime::<R>());

        Some(SwitchParams {
            from_ctx: ghost_ctx as *mut _,
            to_ctx: &to_task.ctx,
            to_aspace: to_task.aspace,
            from_aspace: to_task.aspace, // Dummy same as target
            from_tid: decision.from_tid,
            to_tid: decision.to_tid,
            from_user: false, // Reaped task is now essentially a kernel context switch away
            to_user: to_task.is_user,
            from_user_fs_base: ghost_fs_base as *mut _,
            to_user_fs_base: to_task.user_fs_base,
        })
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

fn select_any_affinity_wake_cpu_from_snapshot<R: BootRuntime>(
    sched: &types::Scheduler<R>,
    preferred_cpu: usize,
    load_snapshot: &WakeBatchLoadSnapshot,
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

    let preferred_depth = load_snapshot.depth_for_cpu(preferred);
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

    let Some((least_cpu, least_depth)) = load_snapshot.least_loaded_online_cpu(&sched.state) else {
        return preferred;
    };
    let overloaded_vs_least = preferred_depth.saturating_sub(least_depth) >= overload_gap;
    if overloaded_vs_least && least_cpu != preferred {
        streak_cell.store(0, Ordering::Release);
        least_cpu
    } else {
        preferred
    }
}

pub(crate) fn select_preferred_any_affinity_wake_cpu<R: BootRuntime>(
    sched: &types::Scheduler<R>,
    last_cpu: Option<usize>,
) -> usize {
    let local_cpu = current_cpu_index::<R>();
    let local_online =
        local_cpu < sched.state.per_cpu.len() && sched.state.online_cpus.contains(&local_cpu);
    let fallback = if local_online {
        local_cpu
    } else if let Some(cpu) =
        sched.state.online_cpus.iter().copied().find(|&cpu| cpu < sched.state.per_cpu.len())
    {
        cpu
    } else if local_cpu < sched.state.per_cpu.len() {
        // Defensive fallback for transient test/bootstrap states where online
        // bookkeeping lags but per-CPU storage is already initialized.
        local_cpu
    } else {
        0
    };

    let Some(last_cpu) = last_cpu
        .filter(|&cpu| cpu < sched.state.per_cpu.len() && sched.state.online_cpus.contains(&cpu))
    else {
        return fallback;
    };

    if !local_online || last_cpu == local_cpu {
        return last_cpu;
    }

    // Compare total runnable depth across all priority queues on each CPU.
    let local_depth = runq_depth_for_cpu(&sched.state, local_cpu);
    let last_depth = runq_depth_for_cpu(&sched.state, last_cpu);
    // A bias of 1 preserves locality by keeping `last_cpu` unless local CPU has
    // at least 2 fewer queued tasks.
    // saturating_add is defensive for pathological queue lengths.
    if local_depth.saturating_add(ANY_WAKE_LOCAL_DEPTH_BIAS) < last_depth {
        local_cpu
    } else {
        last_cpu
    }
}

fn select_preferred_any_affinity_wake_cpu_from_snapshot<R: BootRuntime>(
    sched: &types::Scheduler<R>,
    last_cpu: Option<usize>,
    load_snapshot: &WakeBatchLoadSnapshot,
) -> usize {
    let local_cpu = current_cpu_index::<R>();
    let local_online =
        local_cpu < sched.state.per_cpu.len() && sched.state.online_cpus.contains(&local_cpu);
    let fallback = if local_online {
        local_cpu
    } else if let Some(cpu) =
        sched.state.online_cpus.iter().copied().find(|&cpu| cpu < sched.state.per_cpu.len())
    {
        cpu
    } else if local_cpu < sched.state.per_cpu.len() {
        local_cpu
    } else {
        0
    };

    let Some(last_cpu) = last_cpu
        .filter(|&cpu| cpu < sched.state.per_cpu.len() && sched.state.online_cpus.contains(&cpu))
    else {
        return fallback;
    };

    if !local_online || last_cpu == local_cpu {
        return last_cpu;
    }

    let local_depth = load_snapshot.depth_for_cpu(local_cpu);
    let last_depth = load_snapshot.depth_for_cpu(last_cpu);
    if local_depth.saturating_add(ANY_WAKE_LOCAL_DEPTH_BIAS) < last_depth {
        local_cpu
    } else {
        last_cpu
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
fn set_any_wake_policy_for_tests_with_streak(
    policy: &str,
    overload_gap: usize,
    overload_streak: usize,
) {
    ANY_WAKE_POLICY_INIT_DONE.store(true, Ordering::Release);
    ANY_WAKE_OVERLOAD_POLICY.store(parse_any_wake_overload_policy(policy) as u8, Ordering::Release);
    ANY_WAKE_OVERLOAD_GAP.store(overload_gap.max(1), Ordering::Release);
    ANY_WAKE_OVERLOAD_STREAK_REQUIRED
        .store(overload_streak.clamp(1, ANY_WAKE_OVERLOAD_STREAK_MAX), Ordering::Release);
    for streak in &ANY_WAKE_OVERLOAD_STREAK {
        streak.store(0, Ordering::Release);
    }
}

pub fn init<R: BootRuntime>() {
    crate::ktrace!("  Acquiring scheduler lock...");
    let mut lock = SCHEDULER.lock();
    set_sched_lock_tracking::<R>(0); // Init runs on boot CPU (0)
    crate::ktrace!("  Lock acquired, checking if initialized...");
    if lock.is_none() {
        crate::ktrace!("  Allocating scheduler...");
        let sched = alloc::boxed::Box::new(types::Scheduler::<R>::new());
        crate::ktrace!("  Leaking scheduler...");
        let s = alloc::boxed::Box::leak(sched);
        crate::ktrace!("  Initializing boot task...");
        init_boot_task::<R>(s);
        crate::ktrace!("  Storing scheduler pointer...");
        *lock = Some(s as *mut types::Scheduler<R> as usize);
        unsafe {
            hooks::YIELD_HOOK = Some(sleep::yield_now::<R>);
            hooks::EXIT_HOOK = Some(exit::<R>);
            hooks::SPAWN_USER_HOOK = Some(spawn::spawn_user_thread_ex::<R>);
            hooks::SPAWN_PROCESS_HOOK = Some(spawn::boot_spawn_process::<R>);
            hooks::CURRENT_TID_HOOK = Some(current_tid::<R>);
            hooks::INTERRUPT_TASK_HOOK = Some(lifecycle::interrupt_task::<R>);
            hooks::TAKE_PENDING_INTERRUPT_HOOK = Some(lifecycle::take_pending_interrupt::<R>);
            hooks::TASK_STATUS_HOOK = Some(task_status::<R>);
            hooks::TASK_WAIT_HOOK = Some(wait_task::<R>);
            hooks::SET_PRIORITY_HOOK = Some(set_priority::<R>);
            hooks::CURRENT_PRIORITY_HOOK = Some(current_priority::<R>);
            hooks::AVAILABLE_PARALLELISM_HOOK = Some(available_parallelism::<R>);
            hooks::ALLOC_USER_STACK_HOOK = Some(stack::alloc_user_stack::<R>);
            hooks::RUN_SCHEDULER_HOOK = Some(crate::task::run_scheduler::<R>);
            hooks::KILL_BY_TID_HOOK = Some(kill_by_tid::<R>);
            hooks::DUMP_STATS_HOOK = Some(crate::task::dump_stats::<R>);
            hooks::COLLECT_SCHED_DIAG_HOOK = Some(collect_sched_diag::<R>);
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
            hooks::CURRENT_RESOURCE_HOOK = Some(lifecycle::current_task_resource_id_impl::<R>);
            hooks::POLL_TASK_EXIT_HOOK = Some(poll_task_exit::<R>);
            hooks::REGISTER_TASK_EXIT_WAITER_HOOK = Some(register_task_exit_waiter_public::<R>);
            hooks::UNREGISTER_TASK_EXIT_WAITER_HOOK = Some(unregister_task_exit_waiter::<R>);
            hooks::REGISTER_TIMEOUT_WAKE_HOOK = Some(register_timeout_wake::<R>);
            hooks::UNREGISTER_TIMEOUT_WAKE_HOOK = Some(unregister_timeout_wake::<R>);
            hooks::LIST_PROCESSES_HOOK = Some(list_processes::<R>);
            hooks::LIST_PROCESS_IDS_BY_PGID_HOOK = Some(list_process_ids_by_pgid::<R>);
            hooks::CURRENT_TASK_NAME_HOOK = Some(lifecycle::current_task_name_impl::<R>);
            hooks::TASK_EXEC_HOOK = Some(crate::task::exec::task_exec_current::<R>);
            hooks::SET_CURRENT_USER_FS_BASE_HOOK = Some(lifecycle::set_current_user_fs_base::<R>);
            hooks::CURRENT_USER_FS_BASE_HOOK = Some(lifecycle::current_user_fs_base::<R>);
            hooks::SET_CURRENT_TASK_NAME_HOOK = Some(lifecycle::set_current_task_name::<R>);
            hooks::WAITPID_HOOK = Some(waitpid::<R>);
            hooks::GET_SIGNAL_MASK_HOOK = Some(lifecycle::get_signal_mask::<R>);
            hooks::SET_SIGNAL_MASK_HOOK = Some(lifecycle::set_signal_mask::<R>);
            hooks::GET_THREAD_PENDING_HOOK = Some(lifecycle::get_thread_pending::<R>);
            hooks::SET_THREAD_PENDING_HOOK = Some(lifecycle::set_thread_pending::<R>);
            crate::memory::set_translate_user_page_hook(vm::translate_user_page::<R>);
        }
        blocking::init_blocking_hooks::<R>();
        // Release fence: ensures every hook pointer written above is visible
        // to any CPU that subsequently observes the SCHEDULER lock release or
        // any other acquire barrier.  Required because the hook statics are
        // `static mut` read without a lock on the fast path.
        core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
        let _cpu_total = if let Some(ptr) = *lock {
            let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
            sched.total_cpu_count
        } else {
            1
        };
        crate::kinfo!("Scheduler initialized");
    }
    clear_sched_lock_tracking::<R>();
}

fn init_boot_task<R: BootRuntime>(sched: &mut types::Scheduler<R>) {
    let rt = crate::runtime::<R>();
    let cpu_total = rt.cpu_total_count();

    // Initialize one CpuScheduler per CPU (initially empty/offline).
    // Each CPU scheduler is given its logical CPU index so debug output and
    // invariant checks can identify the owning CPU without consulting external
    // state.
    for cpu_id in 0..cpu_total {
        let cpu_sched = crate::sched::state::CpuScheduler::new_for_cpu(cpu_id);
        crate::kdebug!("SCHED: allocating CpuScheduler for cpu{} (total={})", cpu_id, cpu_total);
        sched.state.per_cpu.push(cpu_sched);
    }
    crate::kinfo!("SCHED: {} per-CPU scheduler(s) allocated", cpu_total);
    crate::kinfo!(
        "SCHED: per-CPU preemption initialized ({} independent preemption domains, no global preemption lock)",
        cpu_total
    );

    sched.total_cpu_count = cpu_total;
    sched.state.set_boot_cpu_online();

    // Enter early-boot mode: defer remote placement and suppress IPI traffic
    // until end_bringup() is called after all service spawning is complete.
    sched.bringup_in_progress = true;

    crate::ktrace!("  Creating boot task...");

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
    crate::sched::set_cpu_current_task(0, 0);

    // Link boot task to CPU 0

    crate::ktrace!("  Creating idle tasks...");

    // Create idle task for CPU 0 initially
    {
        let i = 0;
        let idle_id = sched.spawn(
            lifecycle::idle_task::<R>,
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

    crate::ktrace!("  Boot task initialized");

    // Emit initial per-CPU scheduler state at debug verbosity.
    for pc in sched.state.per_cpu.iter() {
        pc.log_state();
    }
}

impl<R: BootRuntime> types::Scheduler<R> {
    pub fn schedule_point(&mut self, reason: ScheduleReason) -> Option<SwitchDecision> {
        let cpu_idx = current_cpu_index::<R>();
        let global_requested = global_need_resched_swap(cpu_idx, false, Ordering::Acquire);
        self.drain_remote_wake_mailbox(cpu_idx);

        if self.state.per_cpu[cpu_idx].preempt_disable_depth > 0 {
            if global_requested {
                self.state.per_cpu[cpu_idx].need_resched = true;
                // Mirror to the per-CPU atomic flag so the lockless fast-path
                // in resched_if_needed / preempt_enable can observe it without
                // re-acquiring the scheduler lock.
                set_global_need_resched(cpu_idx);
            }
            return None;
        }

        match reason {
            ScheduleReason::PreemptTick => {
                // Wake any sleeping tasks whose time has expired.
                self.wake_sleepers();

                // Check preemption watchdog
                self.check_preempt_watchdog();

                // Slow-path periodic load balancer: proactively migrate tasks
                // from overloaded CPUs to underloaded ones every
                // PERIODIC_BALANCE_INTERVAL_TICKS ticks.
                self.periodic_load_balance();

                let mut should_yield = global_requested || self.state.per_cpu[cpu_idx].need_resched;
                self.state.per_cpu[cpu_idx].need_resched = false;

                // Tick bookkeeping: decrement timeslice via the hot-field cache,
                // avoiding a nested REGISTRY lock on every timer tick.
                if let Some(current_id) = self.state.per_cpu[cpu_idx].current {
                    let current_priority = if let Some(sf) = self.state.get_thread_mut(current_id) {
                        let priority = sf.priority;
                        if sf.timeslice_remaining > 0 {
                            sf.timeslice_remaining -= 1;
                        }
                        if sf.timeslice_remaining == 0 {
                            // Reset for next run
                            sf.timeslice_remaining = types::DEFAULT_TIMESLICE;
                            should_yield = true;
                        }
                        Some(priority)
                    } else {
                        None
                    };
                    if let Some(priority) = current_priority {
                        if priority != TaskPriority::Idle {
                            let delta = vruntime_tick_delta(priority);
                            let stats = self.state.task_runtime_stats_mut(current_id);
                            stats.fair_vruntime = stats.fair_vruntime.saturating_add(delta);
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

    fn drain_remote_wake_mailbox(&mut self, cpu_idx: usize) {
        // This function must only be called by the CPU that owns cpu_idx.
        // Verify this invariant in non-test debug builds.
        debug_assert_runq_cpu_is_local::<R>(cpu_idx);

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
                    if sf.migration_state == MigrationState::Local {
                        let _ = sf
                            .migration_state
                            .try_transition(MigrationState::Requested { target: cpu_idx });
                    }
                    if let MigrationState::Requested { .. } = sf.migration_state {
                        let _ = sf.migration_state.try_transition(MigrationState::InTransit);
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
            self.state.note_enqueue_cause(tid, crate::sched::state::EnqueueCause::Wake);
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
                set_global_need_resched(cpu_idx);
            }
        }
    }

    /// Check if preemption has been disabled too long
    fn check_preempt_watchdog(&mut self) {
        let cpu_idx = current_cpu_index::<R>();
        let per_cpu = &mut self.state.per_cpu[cpu_idx];
        if per_cpu.preempt_disable_depth > 0 && !per_cpu.preempt_watchdog_warned {
            let now = TICK_COUNT.load(Ordering::Relaxed);
            if now.saturating_sub(per_cpu.preempt_disable_since) > 500 {
                // );
                per_cpu.preempt_watchdog_warned = true;
            }
        }
    }

    /// Wake any sleeping tasks whose sleep time has expired
    fn wake_sleepers(&mut self) {
        let now = TICK_COUNT.load(Ordering::Relaxed);
        let current_cpu = current_cpu_index::<R>();
        let lock_start = crate::runtime::<R>().mono_ticks();
        let mut wake_budget = self
            .wake_sleepers_budget_carry
            .saturating_add(types::WAKE_SLEEPERS_BUDGET_PER_TICK)
            .min(types::WAKE_SLEEPERS_BUDGET_CARRY_CAP);

        // Collect pending IPIs and send them *after* this function returns (i.e.
        // after the caller drops the SCHEDULER lock) to reduce IPI-while-locked
        // contention on SMP. Bitmap dedup keeps enqueue O(1) per wake.
        let mut pending_ipi_bitmap = 0u64;

        // Collect all (tid, priority, target_cpu) from the hot-field cache first.
        // We then do a single REGISTRY lock acquisition for the batch of REGISTRY
        // writes rather than one acquisition per task, reducing the number of
        // nested SCHEDULER → REGISTRY lock cycles from N to 1.
        let mut to_wake: alloc::vec::Vec<(u64, usize, usize)> = alloc::vec::Vec::new();
        // Snapshot run-queue depths once for this wake batch so Any-affinity
        // placement can reuse the same balancing view without re-scanning all
        // per-CPU queues for each task.
        let mut wake_batch_loads = WakeBatchLoadSnapshot::new(&self.state);

        // Use the timer-wheel helper which handles bucket scanning,
        // membership bookkeeping, and budget limiting internally.
        let due_tids = self.state.take_due_sleepers(now, wake_budget);
        let taken = due_tids.len();

        for tid in due_tids {
            // Read scheduling fields from the hot-field cache only.
            // REGISTRY is not accessed in this inner loop.
            if let Some(sf) = self.state.get_thread(tid) {
                let priority = sf.priority as usize;
                let target_cpu = match sf.affinity {
                    crate::task::Affinity::Pinned(cpu) => {
                        wake_batch_loads.note_enqueue(cpu);
                        cpu
                    }
                    crate::task::Affinity::Any => {
                        let target = choose_wake_cpu_from_snapshot::<R>(
                            self,
                            sf.last_cpu,
                            &wake_batch_loads,
                        );
                        wake_batch_loads.note_enqueue(target);
                        target
                    }
                    crate::task::Affinity::Restricted(ref aff) => {
                        let cpu_count = self.state.per_cpu.len().max(1);
                        let target = if !cross_cpu_runq_migration_enabled() {
                            let current_cpu = current_cpu.min(cpu_count - 1);
                            if let Some(last_cpu) = sf
                                .last_cpu
                                .filter(|&cpu| cpu < cpu_count && aff.allows(cpu, cpu_count))
                            {
                                last_cpu
                            } else if aff.allows(current_cpu, cpu_count) {
                                current_cpu
                            } else {
                                aff.pick_cpu(cpu_count).unwrap_or(current_cpu)
                            }
                        } else {
                            aff.pick_cpu(cpu_count).unwrap_or_else(|| {
                                choose_wake_cpu_from_snapshot::<R>(
                                    self,
                                    sf.last_cpu,
                                    &wake_batch_loads,
                                )
                            })
                        };
                        wake_batch_loads.note_enqueue(target);
                        target
                    }
                };
                to_wake.push((tid, priority, target_cpu));
            }
            // If not in hot-field cache, skip (task was already removed).
        }

        wake_budget = wake_budget.saturating_sub(taken);

        self.wake_sleepers_budget_carry = wake_budget;

        // Keep wake processing entirely within the scheduler-side hot cache and
        // defer canonical REGISTRY writes until the outer lock-owning call site
        // drops SCHEDULER. Taking REGISTRY here recreates the exact nested lock
        // ordering that can wedge CPU 0 under wake-heavy workloads.
        let wake_mono = crate::runtime::<R>().mono_ticks();
        for (tid, priority, target_cpu) in to_wake {
            if let Some(sf) = self.state.get_thread_mut(tid) {
                if sf.state == TaskState::Dead {
                    continue;
                }
                if sf.state != TaskState::Blocked {
                    // The timeout can fire after a waiter arms its deadline but
                    // before it reaches block_current(). Preserve the wake as a
                    // pending handoff so block_current returns immediately
                    // instead of sleeping forever on an already-expired timeout.
                    sf.wake_pending = true;
                    continue;
                }
                if sf.last_cpu.is_some_and(|c| c != target_cpu) {
                    if sf.migration_state == MigrationState::Local {
                        let _ = sf
                            .migration_state
                            .try_transition(MigrationState::Requested { target: target_cpu });
                    }
                    if let MigrationState::Requested { .. } = sf.migration_state {
                        let _ = sf.migration_state.try_transition(MigrationState::InTransit);
                    }
                }
                sf.state = TaskState::Runnable;
                sf.enqueued_at_tick = now;
                sf.wake_cpu = Some(target_cpu);
                sf.wake_pending = false;
            } else {
                continue;
            }
            self.state.unregister_waiter(tid);
            self.pending_registry_syncs.push(types::DeferredRegistrySync {
                tid,
                new_state: Some(TaskState::Runnable),
                new_enqueued_at_tick: Some(now),
                new_last_cpu: None,
            });
            self.state.wake_enqueued_at_mono.insert(tid, wake_mono);
            self.state.note_enqueue_cause(tid, crate::sched::state::EnqueueCause::Wake);

            let actual_cpu = if target_cpu < self.state.per_cpu.len() { target_cpu } else { 0 };
            if actual_cpu == current_cpu {
                // Local CPU: enqueue directly into the local run queue.
                self.state.enqueue_task(actual_cpu, priority, tid);
            } else {
                // Remote CPU: push through the wake mailbox so the owning CPU
                // enqueues the task itself, preserving per-CPU ownership.
                enqueue_remote_wake_mailbox(
                    actual_cpu,
                    types::RemoteWakeMailboxEntry {
                        tid,
                        priority,
                        enqueued_at_tick: now,
                        wake_mono,
                    },
                );
            }
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
            let current_is_idle =
                self.state.per_cpu.get(actual_cpu).is_some_and(|pc| pc.current == pc.idle_task);
            if priority >= current_prio || current_is_idle {
                if actual_cpu == current_cpu {
                    self.state.per_cpu[current_cpu].need_resched = true;
                    // Mirror to the per-CPU atomic flag so the lockless
                    // fast-path in resched_if_needed / preempt_enable sees it.
                    set_global_need_resched(current_cpu);
                }
            }

            if actual_cpu != current_cpu {
                if actual_cpu < types::MAX_CPUS && (pending_ipi_bitmap & (1u64 << actual_cpu)) != 0
                {
                    PROF_IPI_SUPPRESSED.fetch_add(1, Ordering::Relaxed);
                    continue;
                }
                // Suppress duplicate IPI if the pending flag was already
                // set by a previous wakeup.  The in-flight IPI will pick
                // up this task when it is processed.
                let already_pending = set_global_need_resched(actual_cpu);
                if !already_pending {
                    if actual_cpu < types::MAX_CPUS {
                        pending_ipi_bitmap |= 1u64 << actual_cpu;
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
        for cpu in 0..self.state.per_cpu.len().min(types::MAX_CPUS) {
            if (pending_ipi_bitmap & (1u64 << cpu)) != 0 {
                self.pending_wake_ipis.push(cpu);
            }
        }
    }

    pub fn preempt_disable(&mut self) {
        let cpu_idx = current_cpu_index::<R>();
        let per_cpu = &mut self.state.per_cpu[cpu_idx];
        if per_cpu.preempt_disable_depth == 0 {
            // Track when we started disabling preemption
            per_cpu.preempt_disable_since = TICK_COUNT.load(Ordering::Relaxed);
            per_cpu.preempt_watchdog_warned = false;
        }
        per_cpu.preempt_disable_depth += 1;
        if per_cpu.preempt_disable_depth == 1 {
            // Only trace on transition to disabled? Or depth change?
            // User task says "Record (..., preempt_disable_depth)".
            // Let's trace all for now, or just 0->1.
            // 0->1 is most important for start of disable region.
            crate::trace::irq_ring::push(abi::trace::TraceEvent::PreemptDisable {
                depth: per_cpu.preempt_disable_depth as u32,
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

    pub fn preempt_enable(&mut self) -> Option<SwitchDecision> {
        let cpu_idx = current_cpu_index::<R>();
        let per_cpu = &mut self.state.per_cpu[cpu_idx];
        if per_cpu.preempt_disable_depth > 0 {
            per_cpu.preempt_disable_depth -= 1;
        }

        if per_cpu.preempt_disable_depth == 0
            && (per_cpu.need_resched || need_resched_pending(cpu_idx))
        {
            per_cpu.need_resched = false;
            return self.schedule_point(ScheduleReason::SafePoint);
        }
        None
    }

    pub fn prepare_yield(&mut self) -> Option<SwitchDecision> {
        let cpu_idx = current_cpu_index::<R>();
        let current_id = self.state.per_cpu.get(cpu_idx)?.current?;

        self.metrics.yields += 1;

        // Don't push idle task, dead tasks, or already-blocked tasks back to runq.
        // Use the scheduler-side hot-field cache to avoid a nested REGISTRY lock.
        if Some(current_id) != self.state.per_cpu[cpu_idx].idle_task {
            let previous_enqueue_was_yield = matches!(
                self.state.last_enqueue_cause(current_id),
                crate::sched::state::EnqueueCause::YieldRequeue
            );
            let mut requeue_prio = None;
            if let Some(sf_mut) = self.state.get_thread_mut(current_id) {
                if sf_mut.state != TaskState::Dead && sf_mut.state != TaskState::Blocked {
                    let spin_penalty_eligible =
                        sf_mut.voluntary_yields >= types::SPIN_YIELD_PENALTY_THRESHOLD;
                    let mut prio = sf_mut.priority as usize;
                    sf_mut.voluntary_yields = sf_mut.voluntary_yields.saturating_add(1);
                    if previous_enqueue_was_yield && spin_penalty_eligible {
                        prio = prio
                            .saturating_sub(types::SPIN_YIELD_PENALTY_BANDS)
                            .max(TaskPriority::Low as usize);
                    }
                    requeue_prio = Some(prio);
                }
            }
            if let Some(requeue_prio) = requeue_prio {
                // Push to LOCAL runq (we are yielding on this CPU)
                self.state.enqueue_task(cpu_idx, requeue_prio, current_id);
                self.state.note_enqueue_cause(
                    current_id,
                    crate::sched::state::EnqueueCause::YieldRequeue,
                );
                self.metrics.pushes += 1;
            }
        }

        let switch = self.prepare_schedule();
        self.run_pending_misroute_repair_maintenance();
        switch
    }

    /// Drain up to `max_to_flush` deferred misrouted tasks, requeue each task on
    /// its target CPU, and schedule a deduplicated remote reschedule nudge.
    ///
    /// This keeps misroute cleanup incremental so `prepare_schedule` can keep a
    /// short picker fast path even when a large misroute backlog exists.
    #[inline]
    fn flush_pending_misrouted_requeues_bounded(&mut self, max_to_flush: usize) {
        let current_cpu = current_cpu_index::<R>();
        let now_tick = TICK_COUNT.load(Ordering::Relaxed);
        for _ in 0..max_to_flush {
            let Some((prio, target_cpu, id)) = self.pending_misrouted_requeues.pop() else {
                break;
            };
            if target_cpu == current_cpu {
                // Owning CPU: enqueue directly.
                self.state.enqueue_task(target_cpu, prio, id);
                self.state
                    .note_enqueue_cause(id, crate::sched::state::EnqueueCause::AffinityRepair);
            } else {
                // Remote CPU: route through mailbox to preserve per-CPU
                // ownership.  The drain on the target CPU will enqueue the
                // task; the IPI below ensures the drain runs promptly.
                let wake_mono = crate::runtime::<R>().mono_ticks();
                enqueue_remote_wake_mailbox(
                    target_cpu,
                    types::RemoteWakeMailboxEntry {
                        tid: id,
                        priority: prio,
                        enqueued_at_tick: now_tick,
                        wake_mono,
                    },
                );
            }
            self.queue_prepare_schedule_ipi_dedup(target_cpu);
        }
    }

    /// Re-resolve deferred misroute entries against latest scheduler fields.
    ///
    /// Returns the effective `(priority, target_cpu)` to enqueue to, or `None`
    /// when the entry should be dropped (task removed/non-runnable/already queued).
    #[inline]
    fn resolve_pending_misroute_requeue(
        &self,
        queued_prio: usize,
        queued_target_cpu: usize,
        id: TaskId,
    ) -> Option<(usize, usize)> {
        let sf = self.state.get_thread(id)?;
        if sf.state != TaskState::Runnable || sf.runq_location.is_some() {
            return None;
        }
        let per_cpu_len = self.state.per_cpu.len();
        if per_cpu_len == 0 {
            return None;
        }
        let prio = sf.priority as usize;
        match sf.affinity {
            crate::task::Affinity::Pinned(cpu) if cpu < per_cpu_len => Some((prio, cpu)),
            crate::task::Affinity::Pinned(_) => {
                Some((queued_prio, queued_target_cpu.min(per_cpu_len - 1)))
            }
            crate::task::Affinity::Any => {
                let fallback = queued_target_cpu.min(per_cpu_len - 1);
                Some((prio, sf.last_cpu.filter(|&cpu| cpu < per_cpu_len).unwrap_or(fallback)))
            }
            crate::task::Affinity::Restricted(ref aff) => {
                let target = aff
                    .pick_cpu(per_cpu_len)
                    .unwrap_or_else(|| queued_target_cpu.min(per_cpu_len - 1));
                Some((prio, target))
            }
        }
    }

    /// Opportunistically drop stale deferred misroutes and refresh queued route
    /// metadata before draining bounded repair work.
    #[inline]
    fn prevalidate_pending_misrouted_requeues(&mut self) {
        let mut i = 0usize;
        while i < self.pending_misrouted_requeues.len() {
            let (queued_prio, queued_target_cpu, id) = self.pending_misrouted_requeues[i];
            if let Some((prio, target_cpu)) =
                self.resolve_pending_misroute_requeue(queued_prio, queued_target_cpu, id)
            {
                if (prio, target_cpu) != (queued_prio, queued_target_cpu) {
                    self.pending_misrouted_requeues[i] = (prio, target_cpu, id);
                }
                i += 1;
            } else {
                self.pending_misrouted_requeues.swap_remove(i);
            }
        }
    }

    /// Run deferred misroute cleanup outside the picker path.
    #[inline]
    fn run_pending_misroute_repair_maintenance(&mut self) {
        self.prevalidate_pending_misrouted_requeues();
        self.flush_pending_misrouted_requeues_bounded(PREPARE_SCHEDULE_MISROUTE_REPAIR_BUDGET);
    }

    /// Queue a misrouted task for bounded repair when backlog allows.
    ///
    /// If the deferred backlog cap is reached, fall back to synchronous repair
    /// of this one task so tasks are never dropped and the deferred queue
    /// remains memory-bounded.
    #[inline]
    fn defer_or_repair_misroute(&mut self, prio: usize, target_cpu: usize, id: TaskId) {
        let Some((prio, target_cpu)) = self.resolve_pending_misroute_requeue(prio, target_cpu, id)
        else {
            return;
        };
        if self.pending_misrouted_requeues.len() < PREPARE_SCHEDULE_MISROUTE_BACKLOG_CAP {
            self.pending_misrouted_requeues.push((prio, target_cpu, id));
            return;
        }
        // Backlog safety valve: avoid unbounded memory growth if misroute intake
        // outpaces the bounded per-call repair budget.  Route through the
        // mailbox for remote CPUs to preserve per-CPU ownership.
        let current_cpu = current_cpu_index::<R>();
        if target_cpu == current_cpu {
            self.state.enqueue_task(target_cpu, prio, id);
            self.state.note_enqueue_cause(id, crate::sched::state::EnqueueCause::AffinityRepair);
        } else {
            let now_tick = TICK_COUNT.load(Ordering::Relaxed);
            let wake_mono = crate::runtime::<R>().mono_ticks();
            enqueue_remote_wake_mailbox(
                target_cpu,
                types::RemoteWakeMailboxEntry {
                    tid: id,
                    priority: prio,
                    enqueued_at_tick: now_tick,
                    wake_mono,
                },
            );
        }
        self.queue_prepare_schedule_ipi_dedup(target_cpu);
    }

    #[inline]
    fn queue_prepare_schedule_ipi_dedup(&mut self, target_cpu: usize) {
        let already_pending = set_global_need_resched(target_cpu);
        if !already_pending {
            self.queue_pending_prepare_schedule_ipi(target_cpu);
        } else {
            PROF_IPI_SUPPRESSED.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub(crate) fn prepare_schedule(&mut self) -> Option<SwitchDecision> {
        self.flush_metrics_if_needed();

        let rt = crate::runtime::<R>();
        let cpu_idx = current_cpu_index::<R>();
        let real_cpu_id = rt.current_cpu_id().0 as usize;
        if cpu_idx != real_cpu_id {
            crate::kerror!(
                "FATAL GS CORRUPTION: Core {} thinks it is index {} via GS!",
                real_cpu_id,
                cpu_idx
            );
        }
        if cpu_idx >= self.state.per_cpu.len() || cpu_idx >= types::MAX_CPUS {
            crate::kerror!(
                "Sched: CPU index {} out of bounds (per_cpu={}, MAX={})",
                cpu_idx,
                self.state.per_cpu.len(),
                types::MAX_CPUS
            );
            return None;
        }
        let per_cpu_len = self.state.per_cpu.len();

        // Clear the need-resched flag immediately. Any entry into the picker
        // constitutes evaluation of current runnable state; even if we decide
        // to stay on the same task, the "need" has been satisfied for now.
        // This prevents the busy-yield loop in the idle task and safe-points.
        clear_global_need_resched(cpu_idx, Ordering::Release);
        self.state.per_cpu[cpu_idx].need_resched = false;

        // Sample run-queue depth for this CPU before we start dequeuing.
        sample_runq_len(self, cpu_idx);

        // Snapshot tick count once per scheduling decision so aging is both
        // consistent across this pick and free of repeated tick loads.
        let now = TICK_COUNT.load(Ordering::Relaxed);

        let current_id = self.state.per_cpu[cpu_idx]
            .current
            .expect("prepare_schedule called without current task");

        let mut next_id = None;
        let mut deferred_current_requeue: Option<(usize, TaskId)> = None;
        let mut pick_attempts = 0usize;
        let mut dequeue_failures = 0usize;
        // Priority scan — skip dead and misrouted tasks, evaluating aging on pick.
        // Policy decides which (queue, index) to try; mechanism dequeues and validates.
        while pick_attempts < PREPARE_SCHEDULE_PICK_BUDGET
            && dequeue_failures < PREPARE_SCHEDULE_PICK_BUDGET
        {
            // Policy: choose the best candidate from the run queues.
            let Some((p, best_idx)) = self.policy.pick_next_task(&self.state, cpu_idx, now) else {
                break;
            };

            // Mechanism: peek to capture the TID at the selected position.
            let Some(&best_tid) = self.state.per_cpu[cpu_idx].runq[p].get(best_idx) else {
                dequeue_failures = dequeue_failures.saturating_add(1);
                continue;
            };

            // Mechanism: confirm the entry is still at the expected index and dequeue it.
            let still_same = self.state.per_cpu[cpu_idx].runq[p]
                .get(best_idx)
                .copied()
                .is_some_and(|tid| tid == best_tid);
            if !still_same {
                dequeue_failures = dequeue_failures.saturating_add(1);
                continue;
            }
            let Some(id) = self.state.dequeue_task_at(cpu_idx, p, best_idx) else {
                // Dequeue returned None despite the peek succeeding; the entry
                // must have been concurrently removed (e.g., by a misroute
                // repair). Skip and retry the priority scan.
                dequeue_failures = dequeue_failures.saturating_add(1);
                continue;
            };
            if id != best_tid {
                self.state.enqueue_task(cpu_idx, p, id);
                dequeue_failures = dequeue_failures.saturating_add(1);
                continue;
            };
            pick_attempts += 1;
            self.metrics.pops += 1;

            // Mechanism: validate the dequeued task (dead/blocked/affinity checks).
            // Uses the hot-field cache to avoid a nested REGISTRY lock on every dequeue.
            match self.state.get_thread(id) {
                None => continue, // stale runq entry — skip
                Some(sf)
                    if sf.state == TaskState::Dead
                        || sf.state == TaskState::Blocked
                        || (sf.state == TaskState::Running && id != current_id) =>
                {
                    // Skip non-runnable tasks.
                    continue;
                }
                Some(sf) => {
                    if let crate::task::Affinity::Pinned(target) = sf.affinity {
                        if target != cpu_idx && target < per_cpu_len {
                            self.defer_or_repair_misroute(sf.priority as usize, target, id);
                            continue;
                        }
                    }
                    if let crate::task::Affinity::Restricted(ref aff) = sf.affinity {
                        if !aff.allows(cpu_idx, per_cpu_len) {
                            // Fall back to the queued CPU (clamped) when no
                            // allowed CPU is currently online, rather than
                            // hard-coding CPU 0 which may itself be offline.
                            let target = aff.pick_cpu(per_cpu_len).unwrap_or_else(|| {
                                sf.last_cpu
                                    .filter(|&c| c < per_cpu_len)
                                    .unwrap_or_else(|| self.state.pick_online_cpu_excluding_bsp(0))
                            });
                            crate::kdebug!(
                                "SCHED[affinity]: tid={} misrouted to cpu{}, re-routing to cpu{} \
                                 (allowed={:#x})",
                                id,
                                cpu_idx,
                                target,
                                aff.allowed.0
                            );
                            self.defer_or_repair_misroute(sf.priority as usize, target, id);
                            continue;
                        }
                    }
                    if id == current_id && Some(current_id) != self.state.per_cpu[cpu_idx].idle_task
                    {
                        deferred_current_requeue.get_or_insert((p, id));
                        continue;
                    }
                    next_id = Some(id);
                    break;
                }
            }
        }

        let next_id = match next_id {
            Some(id) => Some(id),
            None => {
                if deferred_current_requeue.is_some() {
                    Some(current_id)
                } else {
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
                            Some(sf)
                                if sf.state == TaskState::Dead
                                    || sf.state == TaskState::Blocked
                                    || (sf.state == TaskState::Running && id != current_id) =>
                            {
                                continue;
                            }
                            Some(sf) => {
                                if let crate::task::Affinity::Pinned(target) = sf.affinity {
                                    if target != cpu_idx && target < per_cpu_len {
                                        self.defer_or_repair_misroute(
                                            sf.priority as usize,
                                            target,
                                            id,
                                        );
                                        continue;
                                    }
                                }
                                if let crate::task::Affinity::Restricted(ref aff) = sf.affinity {
                                    if !aff.allows(cpu_idx, per_cpu_len) {
                                        // Same safe fallback as the normal picker path.
                                        let target =
                                            aff.pick_cpu(per_cpu_len).unwrap_or_else(|| {
                                                sf.last_cpu
                                                    .filter(|&c| c < per_cpu_len)
                                                    .unwrap_or_else(|| {
                                                        self.state.pick_online_cpu_excluding_bsp(0)
                                                    })
                                            });
                                        crate::kdebug!(
                                            "SCHED[affinity]: tid={} (idle-q) misrouted to cpu{}, \
                                         re-routing to cpu{} (allowed={:#x})",
                                            id,
                                            cpu_idx,
                                            target,
                                            aff.allowed.0
                                        );
                                        self.defer_or_repair_misroute(
                                            sf.priority as usize,
                                            target,
                                            id,
                                        );
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
                    } else {
                        if pick_attempts >= PREPARE_SCHEDULE_PICK_BUDGET {
                            crate::kwarn!(
                                "SCHED: CPU {} Priority 0 pick budget exhausted!",
                                cpu_idx
                            );
                        }
                        // Attempt to steal a task from a peer CPU before falling
                        // back to the idle task.  Prefer nearby CPUs first to
                        // exploit shared caches and reduce inter-socket traffic.
                        // See `idle_steal` for the full selection algorithm.
                        let stolen = self.idle_steal(cpu_idx);
                        if stolen.is_some() {
                            stolen
                        } else if let Some(idle) = self.state.per_cpu[cpu_idx].idle_task {
                            self.metrics.idle_picks += 1;
                            Some(idle)
                        } else {
                            None
                        }
                    }
                }
            }
        };

        let next_id = next_id?;

        if next_id != current_id {
            if let Some((prio, id)) = deferred_current_requeue.take() {
                let should_requeue = self.state.get_thread(id).is_some_and(|sf| {
                    sf.state != TaskState::Dead
                        && sf.state != TaskState::Blocked
                        && sf.runq_location.is_none()
                });
                if should_requeue {
                    self.state.enqueue_task(cpu_idx, prio, id);
                    self.state
                        .note_enqueue_cause(id, crate::sched::state::EnqueueCause::YieldRequeue);
                }
            }
        }

        if next_id == current_id {
            let Some(current_sched) = self.state.get_task_mut(current_id) else {
                // The current task was reaped by another CPU while still running here.
                // We cannot continue running it; fallback to the idle task if available.
                crate::kwarn!(
                    "SCHED: current_id {} reaped while running on CPU {}; falling back to idle",
                    current_id,
                    cpu_idx
                );
                if let Some(idle) = self.state.per_cpu[cpu_idx].idle_task {
                    // Update current so next tick doesn't repeat this check
                    self.state.per_cpu[cpu_idx].current = Some(idle);
                    rt.set_idle_task_current(true);
                }
                return None;
            };
            // Keep scheduler cache fields in sync. No REGISTRY write is needed
            // in the same-task (no-switch) case: this task remains running and
            // no lifecycle transition occurred.
            current_sched.state = TaskState::Running;
            current_sched.run_cpu = Some(cpu_idx);

            let next_is_idle = Some(next_id) == self.state.per_cpu[cpu_idx].idle_task;
            rt.set_idle_task_current(next_is_idle);

            return None;
        }

        let current_was_idle = Some(current_id) == self.state.per_cpu[cpu_idx].idle_task;
        let next_is_nonidle = Some(next_id) != self.state.per_cpu[cpu_idx].idle_task;
        let now_mono = rt.mono_ticks();
        if !current_was_idle && !next_is_nonidle {
            if self.state.per_cpu[cpu_idx].idle_enter_mono_ticks.is_none() {
                self.state.per_cpu[cpu_idx].idle_enter_mono_ticks = Some(now_mono);
                self.state.per_cpu[cpu_idx].stats.idle_episodes =
                    self.state.per_cpu[cpu_idx].stats.idle_episodes.saturating_add(1);
            }
        }
        if current_was_idle && next_is_nonidle {
            self.state.per_cpu[cpu_idx].stats.idle_to_nonidle =
                self.state.per_cpu[cpu_idx].stats.idle_to_nonidle.saturating_add(1);
            if let Some(idle_enter) = self.state.per_cpu[cpu_idx].idle_enter_mono_ticks.take() {
                let idle_us = ticks_to_us::<R>(now_mono.wrapping_sub(idle_enter));
                let stats = &mut self.state.per_cpu[cpu_idx].stats;
                stats.idle_total_us = stats.idle_total_us.saturating_add(idle_us);
                stats.idle_longest_us = stats.idle_longest_us.max(idle_us);
                let bucket = idle_episode_hist_bucket(idle_us);
                stats.idle_episode_hist[bucket] = stats.idle_episode_hist[bucket].saturating_add(1);
            }
        }
        self.state.per_cpu[cpu_idx].stats.context_switches =
            self.state.per_cpu[cpu_idx].stats.context_switches.saturating_add(1);
        self.state.per_cpu[cpu_idx].stats.dispatch_count =
            self.state.per_cpu[cpu_idx].stats.dispatch_count.saturating_add(1);

        self.state.per_cpu[cpu_idx].current = Some(next_id);

        // Drive transition decisions from the scheduler hot-cache and defer
        // REGISTRY synchronization for these state fields until after the
        // SCHEDULER lock is released.
        let old_was_running = self
            .state
            .get_task(current_id)
            .map(|task| task.state == TaskState::Running)
            .unwrap_or(false);
        let mut old_registry_sync = types::DeferredRegistrySync {
            tid: current_id,
            new_state: None,
            new_enqueued_at_tick: None,
            new_last_cpu: Some(cpu_idx),
        };
        if let Some(old_sched) = self.state.get_task_mut(current_id) {
            if old_was_running {
                old_sched.state = TaskState::Runnable;
                old_sched.enqueued_at_tick = now;
                old_registry_sync.new_state = Some(TaskState::Runnable);
                old_registry_sync.new_enqueued_at_tick = Some(now);
            }
            old_sched.last_cpu = Some(cpu_idx);
            self.pending_registry_syncs.push(old_registry_sync);
        } else {
            // Task already removed from scheduler state (reaped).
            // resolve_switch_params will handle saving its registers into ghost storage.
        }

        let mut migrated = false;
        {
            let Some(new_sched) = self.state.get_task_mut(next_id) else {
                crate::kerror!("SchedTasks: {:?}", self.state.thread_ids());
                panic!("failed to find next_id {} in scheduler state", next_id);
            };
            if let Some(prev_cpu) = new_sched.last_cpu {
                if prev_cpu != cpu_idx {
                    migrated = true;
                }
            }
            // Finalise migration: arriving task transitions back to Local.
            if migrated {
                match new_sched
                    .migration_state
                    .try_transition(crate::sched::state::MigrationState::Local)
                {
                    Ok(new_state) => {
                        crate::kdebug!(
                            "MIGRATE[tid={}]: {:?} → Local (arrived on cpu{})",
                            next_id,
                            new_sched.migration_state,
                            cpu_idx,
                        );
                        new_sched.migration_state = new_state;
                    }
                    Err(bad_state) => {
                        // An unexpected migration state on arrival is a logic
                        // error: the task arrived on a new CPU but was not in
                        // InTransit or Local as expected.  Log at debug level
                        // only to avoid flooding the console during high-load
                        // migration storms, then force-reset to Local so the
                        // task can continue running.
                        crate::ktrace!(
                            "MIGRATE[tid={}]: BUG: unexpected migration state {:?} on arrival \
                             at cpu{} — forcing Local to allow forward progress",
                            next_id,
                            bad_state,
                            cpu_idx,
                        );
                        // Force-reset to Local so the task can be migrated again.
                        new_sched.migration_state = crate::sched::state::MigrationState::Local;
                    }
                }
            }
            new_sched.state = TaskState::Running;
            new_sched.last_cpu = Some(cpu_idx);
            new_sched.run_cpu = Some(cpu_idx);
        }
        if migrated {
            let enqueue_cause = self.state.last_enqueue_cause(next_id);
            let stats = self.state.task_runtime_stats_mut(next_id);
            stats.migration_count = stats.migration_count.saturating_add(1);
            match enqueue_cause {
                crate::sched::state::EnqueueCause::Wake => {
                    stats.migration_wake = stats.migration_wake.saturating_add(1)
                }
                crate::sched::state::EnqueueCause::Steal => {
                    stats.migration_steal = stats.migration_steal.saturating_add(1)
                }
                crate::sched::state::EnqueueCause::AffinityRepair => {
                    stats.migration_affinity = stats.migration_affinity.saturating_add(1)
                }
                crate::sched::state::EnqueueCause::YieldRequeue => {
                    stats.migration_yield_requeue = stats.migration_yield_requeue.saturating_add(1)
                }
                _ => stats.migration_other = stats.migration_other.saturating_add(1),
            }
            let runs_before = stats.runs_since_last_migration;
            stats.runs_between_migrations_total =
                stats.runs_between_migrations_total.saturating_add(runs_before);
            stats.min_runs_between_migrations = stats.min_runs_between_migrations.min(runs_before);
            stats.max_runs_between_migrations = stats.max_runs_between_migrations.max(runs_before);
            stats.runs_since_last_migration = 0;
        }
        {
            let stats = self.state.task_runtime_stats_mut(next_id);
            stats.run_count = stats.run_count.saturating_add(1);
            stats.runs_since_last_migration = stats.runs_since_last_migration.saturating_add(1);
        }
        if let Some(wake_mono) = self.state.wake_enqueued_at_mono.remove(&next_id) {
            let wake_to_run_us = ticks_to_us::<R>(now_mono.wrapping_sub(wake_mono));
            PROF_WAKE_TO_RUN_COUNT.fetch_add(1, Ordering::Relaxed);
            PROF_WAKE_TO_RUN_TICKS_TOTAL.fetch_add(wake_to_run_us, Ordering::Relaxed);
            update_max_u64(&PROF_WAKE_TO_RUN_TICKS_MAX, wake_to_run_us);
            let bucket = us_latency_hist_bucket(wake_to_run_us);
            PROF_WAKE_TO_RUN_HIST[bucket].fetch_add(1, Ordering::Relaxed);
            let stats = self.state.task_runtime_stats_mut(next_id);
            stats.wake_to_run_count = stats.wake_to_run_count.saturating_add(1);
            stats.wake_to_run_ticks_total =
                stats.wake_to_run_ticks_total.saturating_add(wake_to_run_us);
            stats.wake_to_run_ticks_max = stats.wake_to_run_ticks_max.max(wake_to_run_us);
            stats.wake_to_run_hist[bucket] = stats.wake_to_run_hist[bucket].saturating_add(1);
        }
        self.pending_registry_syncs.push(types::DeferredRegistrySync {
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
        let next_is_idle = Some(next_id) == self.state.per_cpu[cpu_idx].idle_task;
        rt.set_idle_task_current(next_is_idle);

        Some(SwitchDecision { cpu_idx, from_tid: current_id, to_tid: next_id })
    }

    /// Try to steal exactly one migratable task from `victim_cpu` into
    /// `local_cpu`.
    ///
    /// The victim must have at least `min_depth` runnable tasks so that we do
    /// not drain a peer that only has one task remaining (which would be
    /// consumed by the peer's own next scheduling cycle anyway).
    ///
    /// Scans priority queues from highest to lowest, peeking at most
    /// `STEAL_SCAN_DEPTH_PER_PRIORITY` entries per level to keep the path
    /// bounded.  `Affinity::Any` tasks are always eligible; `Affinity::Restricted`
    /// tasks are eligible if `local_cpu` is in their allowed CPU set; pinned
    /// tasks are skipped.
    ///
    /// Returns the stolen `TaskId` (removed from the victim's run queue and
    /// ready to be dispatched on `local_cpu`), or `None` if no suitable task
    /// was found.
    fn try_steal_one(
        &mut self,
        local_cpu: usize,
        victim_cpu: usize,
        min_depth: usize,
    ) -> Option<TaskId> {
        if victim_cpu == local_cpu {
            return None;
        }
        let victim_depth = runq_depth_for_cpu(&self.state, victim_cpu);
        if victim_depth < min_depth {
            return None;
        }
        // Steal the highest-priority non-pinned task.
        // Queues 1-4 correspond to TaskPriority::Idle+1 through Realtime (see
        // types::RUNQ_COUNT = 5 with queue 0 reserved for idle-priority tasks).
        for p in (1..5).rev() {
            // Bound the lookahead so idle-path steal attempts stay predictable.
            let scan_limit =
                self.state.per_cpu[victim_cpu].runq[p].len().min(STEAL_SCAN_DEPTH_PER_PRIORITY);
            let mut candidate_index = None;
            for idx in 0..scan_limit {
                // Re-read by index each step; if this slot no longer exists
                // (e.g. queue compaction from prior lazy-invalidated removals),
                // stop this priority scan attempt.
                let Some(tid) = self.state.per_cpu[victim_cpu].runq[p].get(idx).copied() else {
                    break;
                };
                let stealable = match self.state.get_thread(tid) {
                    Some(sf) => {
                        // Require canonical queue-placement metadata to match so
                        // we do not steal stale lazy-invalidated entries (see
                        // `SchedState::dequeue_thread_front` comment).
                        sf.state != TaskState::Dead
                            // Never migrate a currently running task.
                            && sf.state != TaskState::Running
                            // A voluntarily-yielded task is enqueued before
                            // the low-level context switch has saved and left
                            // its kernel stack.  Keep that entry local so a
                            // second CPU cannot run the same task during that
                            // switch-out window.
                            && self.state.last_enqueue_cause(tid)
                                != crate::sched::state::EnqueueCause::YieldRequeue
                            // Validate canonical placement before steal.
                            && sf.runq_location == Some((victim_cpu, p))
                            // Only steal tasks that are allowed to run on the local CPU.
                            && match sf.affinity {
                                crate::task::Affinity::Any => true,
                                crate::task::Affinity::Pinned(_) => false,
                                crate::task::Affinity::Restricted(ref aff) => {
                                    aff.allows(local_cpu, self.state.per_cpu.len())
                                }
                            }
                            // Only steal tasks whose migration state allows it.
                            && sf.migration_state.is_migratable()
                    }
                    _ => false,
                };
                if stealable {
                    candidate_index = Some(idx);
                    break;
                }
            }
            if let Some(idx) = candidate_index {
                if let Some(stolen_id) = self.state.dequeue_task_at(victim_cpu, p, idx) {
                    if let Some(pc) = self.state.per_cpu.get_mut(victim_cpu) {
                        pc.stats.steals_out = pc.stats.steals_out.saturating_add(1);
                    }
                    if let Some(pc) = self.state.per_cpu.get_mut(local_cpu) {
                        pc.stats.steals_in = pc.stats.steals_in.saturating_add(1);
                    }
                    if let Some(sf) = self.state.get_task_mut(stolen_id) {
                        sf.wake_cpu = Some(local_cpu);
                        // Transition migration state: Local/Requested → InTransit.
                        match sf
                            .migration_state
                            .try_transition(crate::sched::state::MigrationState::InTransit)
                        {
                            Ok(new_state) => {
                                crate::kdebug!(
                                    "MIGRATE[tid={}]: {:?} → InTransit (steal cpu{} → cpu{})",
                                    stolen_id,
                                    sf.migration_state,
                                    victim_cpu,
                                    local_cpu,
                                );
                                sf.migration_state = new_state;
                            }
                            Err(bad_state) => {
                                crate::kdebug!(
                                    "MIGRATE[tid={}]: illegal steal transition from {:?} (cpu{} → cpu{})",
                                    stolen_id,
                                    bad_state,
                                    victim_cpu,
                                    local_cpu,
                                );
                            }
                        }
                    }
                    self.state
                        .note_enqueue_cause(stolen_id, crate::sched::state::EnqueueCause::Steal);
                    self.metrics.steals += 1;
                    return Some(stolen_id);
                }
            }
        }
        None
    }

    /// Attempt to steal work from a peer CPU when the local run queue is empty.
    ///
    /// This is the idle-path entry point for work stealing.  It builds an
    /// ordered candidate list that **prefers nearby CPUs** (indices within
    /// `STEAL_NEARBY_RADIUS` of `local_cpu`) over distant ones, to exploit
    /// shared caches and reduce inter-socket traffic.  Within each proximity
    /// group the candidates are sorted by descending run-queue depth so we
    /// target the most imbalanced peer first.
    ///
    /// The search stops at the first successful steal so that every idle CPU
    /// gets exactly one task per call, avoiding a thundering-herd scenario
    /// where many idle CPUs simultaneously drain a single loaded peer.
    ///
    /// # Anti-thrashing
    ///
    /// A minimum depth threshold (`STEAL_MIN_VICTIM_DEPTH`) ensures that a
    /// victim CPU must have at least two runnable tasks before we steal from
    /// it.  This prevents repeatedly passing a single task back and forth
    /// between CPUs when the system is nearly idle.
    fn idle_steal(&mut self, local_cpu: usize) -> Option<TaskId> {
        if !cross_cpu_runq_migration_enabled() {
            let _ = local_cpu;
            return None;
        }

        let per_cpu_len = self.state.per_cpu.len();

        // Partition online peer CPUs into nearby and far groups, collecting
        // their current run-queue depths at the same time.
        let max_nearby = per_cpu_len.min(STEAL_NEARBY_RADIUS * 2 + 1);
        let mut nearby: alloc::vec::Vec<(usize, usize)> =
            alloc::vec::Vec::with_capacity(max_nearby);
        let max_far = per_cpu_len.saturating_sub(max_nearby);
        let mut far: alloc::vec::Vec<(usize, usize)> = alloc::vec::Vec::with_capacity(max_far);

        for &cpu in &self.state.online_cpus {
            if cpu == local_cpu || cpu >= per_cpu_len {
                continue;
            }
            let depth = runq_depth_for_cpu(&self.state, cpu);
            // Clamp subtraction to avoid wrapping on usize arithmetic.
            let dist = if cpu >= local_cpu { cpu - local_cpu } else { local_cpu - cpu };
            if dist <= STEAL_NEARBY_RADIUS {
                nearby.push((cpu, depth));
            } else {
                far.push((cpu, depth));
            }
        }

        // Sort each group so we try the most-loaded victim first.
        nearby.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        far.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        // Try nearby CPUs first, then fall back to distant ones.
        for (victim_cpu, _) in nearby.iter().chain(far.iter()).copied() {
            if let Some(stolen) = self.try_steal_one(local_cpu, victim_cpu, STEAL_MIN_VICTIM_DEPTH)
            {
                return Some(stolen);
            }
        }
        None
    }

    /// Convenience wrapper kept for backward compatibility with callers that
    /// used the old name.  Delegates to [`idle_steal`][Self::idle_steal].
    #[inline]
    fn steal_task_for(&mut self, local_cpu: usize) -> Option<TaskId> {
        self.idle_steal(local_cpu)
    }

    /// Slow-path periodic load balancer for severe multi-CPU imbalance.
    ///
    /// This is called from the [`ScheduleReason::PreemptTick`] path and runs at
    /// most once every [`types::PERIODIC_BALANCE_INTERVAL_TICKS`] ticks.  When
    /// the busiest online CPU has at least
    /// [`types::PERIODIC_BALANCE_IMBALANCE_MIN_DEPTH_DIFF`] more runnable tasks
    /// than the least-loaded CPU, it migrates up to
    /// [`types::PERIODIC_BALANCE_MAX_MIGRATIONS_PER_RUN`] migratable tasks
    /// (`Affinity::Any` or `Affinity::Restricted` tasks whose allowed set includes
    /// the target CPU) from the busiest CPU into the least-loaded CPU's run queue.
    ///
    /// # Design notes
    ///
    /// * **Rate limiting** — the `last_balance_tick` timestamp prevents the
    ///   balancer from running more than once per interval, avoiding oscillation.
    /// * **Bounded migrations** — at most `PERIODIC_BALANCE_MAX_MIGRATIONS_PER_RUN`
    ///   tasks are moved per pass so the tick path stays bounded.
    /// * **No oscillation** — the minimum depth-difference threshold means a
    ///   single-task imbalance (which often self-corrects in one scheduling
    ///   cycle) is ignored.
    /// * **Complementary to idle-steal** — `idle_steal` is reactive (runs when a
    ///   CPU becomes idle); this balancer is proactive (runs periodically even
    ///   when all CPUs have at least some work) to address sustained severe
    ///   imbalances that do not trigger idle-steal.
    fn periodic_load_balance(&mut self) {
        if !cross_cpu_runq_migration_enabled() {
            return;
        }

        let now = TICK_COUNT.load(Ordering::Relaxed);

        // Rate limit: skip if we balanced recently.
        if now.wrapping_sub(self.last_balance_tick) < types::PERIODIC_BALANCE_INTERVAL_TICKS {
            return;
        }
        self.last_balance_tick = now;

        // Require at least two online CPUs for inter-CPU migration.
        if self.state.online_cpus.len() < 2 {
            return;
        }

        // Find the busiest and least-loaded online CPUs in a single pass.
        let mut busiest_cpu = 0usize;
        let mut busiest_depth = 0usize;
        let mut least_cpu = 0usize;
        let mut least_depth = usize::MAX;
        let mut found_any = false;

        for &cpu in &self.state.online_cpus {
            let depth = runq_depth_for_cpu(&self.state, cpu);
            if !found_any || depth > busiest_depth {
                busiest_depth = depth;
                busiest_cpu = cpu;
            }
            if !found_any || depth < least_depth {
                least_depth = depth;
                least_cpu = cpu;
            }
            found_any = true;
        }

        if !found_any || busiest_cpu == least_cpu {
            return;
        }

        // Only rebalance when the imbalance is severe enough to warrant
        // migration and the victim has enough tasks to donate.
        let depth_diff = busiest_depth.saturating_sub(least_depth);
        if depth_diff < types::PERIODIC_BALANCE_IMBALANCE_MIN_DEPTH_DIFF
            || busiest_depth < STEAL_MIN_VICTIM_DEPTH
        {
            return;
        }

        // Migrate up to PERIODIC_BALANCE_MAX_MIGRATIONS_PER_RUN tasks.
        let mut migrated = 0usize;
        while migrated < types::PERIODIC_BALANCE_MAX_MIGRATIONS_PER_RUN {
            // Re-check imbalance before each individual migration to avoid
            // over-migrating when the situation improves mid-pass.
            let cur_busiest = runq_depth_for_cpu(&self.state, busiest_cpu);
            let cur_least = runq_depth_for_cpu(&self.state, least_cpu);
            if cur_busiest.saturating_sub(cur_least)
                < types::PERIODIC_BALANCE_IMBALANCE_MIN_DEPTH_DIFF
                || cur_busiest < STEAL_MIN_VICTIM_DEPTH
            {
                break;
            }

            // Steal one task from the busiest CPU.  try_steal_one removes it
            // from the victim's run queue, sets wake_cpu = least_cpu, and
            // records the enqueue cause as Steal.
            let Some(stolen_id) =
                self.try_steal_one(least_cpu, busiest_cpu, STEAL_MIN_VICTIM_DEPTH)
            else {
                break;
            };

            // Determine the priority to use for enqueueing.
            let priority = self
                .state
                .get_task(stolen_id)
                .map(|sf| sf.priority as usize)
                .unwrap_or(TaskPriority::Normal as usize);

            // Place the task into the target CPU's run queue.  Route through
            // the wake mailbox when least_cpu is a remote CPU so that only the
            // owning CPU directly mutates its own run queue.
            let current_cpu = current_cpu_index::<R>();
            if least_cpu == current_cpu {
                self.state.enqueue_task(least_cpu, priority, stolen_id);
            } else {
                let now_tick = TICK_COUNT.load(Ordering::Relaxed);
                let wake_mono = crate::runtime::<R>().mono_ticks();
                enqueue_remote_wake_mailbox(
                    least_cpu,
                    types::RemoteWakeMailboxEntry {
                        tid: stolen_id,
                        priority,
                        enqueued_at_tick: now_tick,
                        wake_mono,
                    },
                );
            }

            migrated += 1;
            PROF_PERIODIC_BALANCE_MIGRATIONS.fetch_add(1, Ordering::Relaxed);
        }

        if migrated > 0 {
            // Nudge the target CPU so it picks up the newly enqueued work
            // promptly.  The IPI is sent after releasing the SCHEDULER lock
            // (via the deferred bitmap mechanism) to avoid lock-order issues.
            self.queue_pending_prepare_schedule_ipi(least_cpu);
        }
    }

    pub fn terminate_current(
        &mut self,
        terminating_tid: TaskId,
        siblings_to_kill: &[TaskId],
    ) -> SwitchDecision {
        let cpu_idx = current_cpu_index::<R>();
        let current_id = self
            .state
            .per_cpu
            .get(cpu_idx)
            .and_then(|pc| pc.current)
            .expect("terminate_current called with no current task");
        if terminating_tid != current_id {
            panic!(
                "scheduler invariant violated: terminate_current tid mismatch (cpu={}, scheduler_current={}, terminating_tid={})",
                cpu_idx, current_id, terminating_tid
            );
        }

        if let Some(task) = self.state.get_task_mut(current_id) {
            task.runq_location = None;
            task.state = TaskState::Dead;
        }
        lifecycle::purge_task_from_scheduler_queues::<R>(self, current_id);

        for &sibling in siblings_to_kill {
            if sibling == current_id {
                continue;
            }
            if let Some(sf) = self.state.get_task_mut(sibling) {
                sf.runq_location = None;
                sf.state = TaskState::Dead;
            }
            lifecycle::purge_task_from_scheduler_queues::<R>(self, sibling);
        }

        for _ in 0..TERMINATE_CURRENT_SWITCH_RETRY_BUDGET {
            if let Some(switch) = self.prepare_schedule() {
                self.run_pending_misroute_repair_maintenance();
                return switch;
            }
            self.run_pending_misroute_repair_maintenance();
            core::hint::spin_loop();
        }

        let local_runq_depth = self
            .state
            .per_cpu
            .get(cpu_idx)
            .map(|pc| pc.runq.iter().map(|q| q.len()).sum::<usize>())
            .unwrap_or(0);
        let idle_tid = self.state.per_cpu.get(cpu_idx).and_then(|pc| pc.idle_task);
        crate::kerror!(
            "SCHED: terminate_current exhausted retries (cpu={}, current_tid={}, idle_tid={:?}, local_runq_depth={}, pending_misroutes={}, online_cpus={:?})",
            cpu_idx,
            current_id,
            idle_tid,
            local_runq_depth,
            self.pending_misrouted_requeues.len(),
            self.state.online_cpus
        );
        crate::kerror!("SCHED: known thread IDs in scheduler state: {:?}", self.state.thread_ids());
        panic!(
            "scheduler invariant violated: terminate_current could not find a switch after {} attempts (cpu={}, current_tid={})",
            TERMINATE_CURRENT_SWITCH_RETRY_BUDGET, cpu_idx, current_id
        );
    }

    /// Update only scheduler hot-cache/run-queue priority state.
    ///
    /// This intentionally avoids touching REGISTRY so callers can hold
    /// SCHEDULER without creating a nested `SCHEDULER -> REGISTRY` lock edge.
    /// Returns `true` when the task existed in scheduler state.
    fn set_priority_hot_cache(&mut self, id: TaskId, priority: TaskPriority) -> bool {
        let mut requeue_cpu = None;
        let Some(task) = self.state.get_task_mut(id) else {
            return false;
        };
        task.priority = priority;
        if task.state == TaskState::Runnable {
            requeue_cpu = task.runq_location.map(|(cpu, _)| cpu);
        }

        if let Some(cpu) = requeue_cpu {
            self.state.remove_task_from_runq(id);
            // Route through mailbox for remote CPUs so only the owning CPU
            // mutates its own run queue directly.
            let current_cpu = current_cpu_index::<R>();
            if cpu == current_cpu {
                self.state.enqueue_task(cpu, priority as usize, id);
                // The priority changed; ask the local CPU to reschedule so it
                // can pick up the re-enqueued task at the new priority.
                self.state.per_cpu[cpu].need_resched = true;
            } else {
                let now_tick = TICK_COUNT.load(Ordering::Relaxed);
                let wake_mono = crate::runtime::<R>().mono_ticks();
                enqueue_remote_wake_mailbox(
                    cpu,
                    types::RemoteWakeMailboxEntry {
                        tid: id,
                        priority: priority as usize,
                        enqueued_at_tick: now_tick,
                        wake_mono,
                    },
                );
                self.queue_prepare_schedule_ipi_dedup(cpu);
            }
        }
        true
    }

    pub fn set_priority(&mut self, id: TaskId, priority: TaskPriority) {
        let _ = self.set_priority_hot_cache(id, priority);
    }

    /// Mark a secondary CPU as online and initialize its idle task.
    pub fn cpu_online(&mut self, cpu_index: usize) {
        crate::kdebug!("SMP: CPU {} online (triggered by scheduler spawn)", cpu_index);
        while self.state.per_cpu.len() <= cpu_index {
            self.state.per_cpu.push(crate::sched::state::PerCpu::new());
        }
        self.state.mark_cpu_online(cpu_index);
        self.total_cpu_count = self.total_cpu_count.max(cpu_index.saturating_add(1));

        // Create idle task for this new CPU
        let i = cpu_index;
        let idle_id = self.spawn(
            lifecycle::idle_task::<R>,
            StartupArg::Raw(i),
            TaskPriority::Idle,
            crate::task::Affinity::Pinned(i),
        );

        // Remove from run queues - idle tasks are special
        self.state.remove_task_from_runq(idle_id);

        // Set as this CPU's idle task
        self.state.per_cpu[i].idle_task = Some(idle_id);

        // Keep scheduler cache affinity pinned for idle tasks. The canonical
        // registry affinity is already initialized from the spawn call.
        if let Some(sf) = self.state.get_task_mut(idle_id) {
            sf.affinity = crate::task::Affinity::Pinned(i);
        }
    }
}

#[cfg(test)]
mod tests;
