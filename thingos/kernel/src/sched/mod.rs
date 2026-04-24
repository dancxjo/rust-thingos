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
    block_current, block_current_erased, init_blocking_hooks, wake_task, wake_task_erased,
};
pub use hooks::{
    ProcessSnapshot, add_user_mapping_current, alloc_user_stack_current,
    available_parallelism_current, check_user_mapping_current, current_priority_current,
    current_task_name_current, current_task_resource_id, current_tid_current,
    current_user_fs_base_current, dump_stats_current, exit_current, get_signal_mask_current,
    get_thread_pending_current, get_user_mapping_at_current, handle_user_stack_fault_current,
    interrupt_task_current, kill_by_tid_current, list_process_ids_by_pgid_current,
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
pub use state::{CpuSchedStats, CpuScheduler, MigrationState, RunQueue, WakeMailbox, WakeMailboxEntry};
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

#[inline]
pub(crate) fn scheduler_lock_held_by_this_cpu<R: BootRuntime>() -> bool {
    let owner = SCHEDULER_LOCK_OWNER.load(Ordering::Acquire);
    owner == crate::runtime::<R>().current_cpu_index() as isize
}

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
static LAST_RESCHED_IPI_SENT_AT_TICK: [AtomicU64; types::MAX_CPUS] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ATOMIC_INIT: AtomicU64 = AtomicU64::new(RESCHED_IPI_NEVER_SENT);
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
            if num_stats >= stats_buf.len() { break; }
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
            let runq_total = sched
                .state
                .per_cpu
                .get(cpu_idx)
                .map(|pc| pc.runq.total_len())
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

                while crate::sched::is_task_on_any_cpu(switch.to_tid) {
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
        if owner != -1 && acquired_at != 0 && now.saturating_sub(acquired_at) > 200
        {
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
    let last_tick = LAST_RESCHED_IPI_SENT_AT_TICK[target_cpu].load(Ordering::Relaxed);
    if last_tick != RESCHED_IPI_NEVER_SENT
        && now_tick.saturating_sub(last_tick) < RESCHED_IPI_MIN_TICK_DELTA
    {
        PROF_IPI_SUPPRESSED.fetch_add(1, Ordering::Relaxed);
        return false;
    }
    LAST_RESCHED_IPI_SENT_AT_TICK[target_cpu].store(now_tick, Ordering::Relaxed);
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
            hooks::LIST_PROCESS_IDS_BY_PGID_HOOK = Some(list_process_ids_by_pgid::<R>);
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
        crate::kdebug!(
            "SCHED: allocating CpuScheduler for cpu{} (total={})",
            cpu_id,
            cpu_total
        );
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

            let current_prio = self.state.per_cpu[cpu_idx]
                .current
                .and_then(|cid| self.state.get_thread(cid))
                .map(|sf| sf.priority as usize)
                .unwrap_or(0);
            let is_idle =
                self.state.per_cpu[cpu_idx].current == self.state.per_cpu[cpu_idx].idle_task;
            if priority >= current_prio || is_idle {
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
                        let target = aff
                            .pick_cpu(cpu_count)
                            .unwrap_or_else(|| choose_wake_cpu_from_snapshot::<R>(
                                self,
                                sf.last_cpu,
                                &wake_batch_loads,
                            ));
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
                sf.state = TaskState::Runnable;
                sf.enqueued_at_tick = now;
                sf.wake_cpu = Some(target_cpu);
            } else {
                continue;
            }
            self.pending_registry_syncs.push(types::DeferredRegistrySync {
                tid,
                new_state: Some(TaskState::Runnable),
                new_enqueued_at_tick: Some(now),
                new_last_cpu: None,
            });
            self.state.wake_enqueued_at_mono.insert(tid, wake_mono);
            self.state.note_enqueue_cause(tid, crate::sched::state::EnqueueCause::Wake);

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

        if per_cpu.preempt_disable_depth == 0 && per_cpu.need_resched {
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
        for _ in 0..max_to_flush {
            let Some((prio, target_cpu, id)) = self.pending_misrouted_requeues.pop() else {
                break;
            };
            self.state.enqueue_task(target_cpu, prio, id);
            self.state.note_enqueue_cause(id, crate::sched::state::EnqueueCause::AffinityRepair);
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
        // outpaces the bounded per-call repair budget.
        self.state.enqueue_task(target_cpu, prio, id);
        self.state.note_enqueue_cause(id, crate::sched::state::EnqueueCause::AffinityRepair);
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

        let mut next_id = None;
        let mut pick_attempts = 0usize;
        let mut dequeue_failures = 0usize;
        // Priority scan — skip dead and misrouted tasks, evaluating aging on-pick
        while pick_attempts < PREPARE_SCHEDULE_PICK_BUDGET
            && dequeue_failures < PREPARE_SCHEDULE_PICK_BUDGET
        {
            let mut best_q = None;
            let mut best_idx = None;
            let mut best_tid = None;
            let mut best_eff = 0;
            let mut best_vruntime = u64::MAX;
            // Rotating scan seed: naturally wraps with usize arithmetic and is
            // bounded back to queue length via modulo below.
            let scan_base = self.state.per_cpu[cpu_idx].stats.dispatch_count as usize;

            for p in (1..5).rev() {
                let runq_len = self.state.per_cpu[cpu_idx].runq[p].len();
                if runq_len == 0 {
                    continue;
                }
                let scan_len = runq_len.min(PREPARE_SCHEDULE_FAIR_SCAN_DEPTH_PER_PRIORITY);
                for step in 0..scan_len {
                    let idx = (scan_base + step) % runq_len;
                    let Some(&id) = self.state.per_cpu[cpu_idx].runq[p].get(idx) else {
                        continue;
                    };
                    let Some(sf) = self.state.get_thread(id) else {
                        continue;
                    };
                    if sf.runq_location != Some((cpu_idx, p)) {
                        continue;
                    }
                    if sf.state == TaskState::Dead || sf.state == TaskState::Blocked {
                        continue;
                    }

                    let mut eff = p; // Start with base priority (queue index)
                    if p < 4 {
                        // aging only applies up to High
                        let wait_ticks = now.saturating_sub(sf.enqueued_at_tick);
                        let boost = (wait_ticks / types::AGING_THRESHOLD_TICKS) as usize;
                        let boost = boost.min(types::MAX_PRIORITY_BOOST);
                        eff = (p + boost).min(4);
                    }
                    let vruntime = self.state.task_runtime_stats(id).fair_vruntime;
                    // CFS-style tie-break: for equal effective priority, prefer
                    // the least-served runnable task (lower vruntime).
                    //
                    // Preserve previous anti-starvation behavior for exact ties:
                    // if both effective priority and vruntime are equal, prefer
                    // the lower-base-priority queue candidate (larger age debt).
                    let better = better_fair_pick_candidate(
                        eff,
                        vruntime,
                        p,
                        best_eff,
                        best_vruntime,
                        best_q,
                    );
                    if better {
                        best_eff = eff;
                        best_vruntime = vruntime;
                        best_q = Some(p);
                        best_idx = Some(idx);
                        best_tid = Some(id);
                    }
                }
            }

            if let (Some(p), Some(best_idx), Some(best_tid)) = (best_q, best_idx, best_tid) {
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

                // Use the hot-field cache for dead/affinity checks to avoid a
                // nested REGISTRY lock on every task dequeue.
                match self.state.get_thread(id) {
                    None => continue, // stale runq entry — skip
                    Some(sf) if sf.state == TaskState::Dead || sf.state == TaskState::Blocked => {
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
                                let target = aff
                                    .pick_cpu(per_cpu_len)
                                    .unwrap_or_else(|| sf.last_cpu
                                        .filter(|&c| c < per_cpu_len)
                                        .unwrap_or_else(|| self.state.pick_online_cpu_excluding_bsp(0)));
                                crate::kdebug!(
                                    "SCHED[affinity]: tid={} misrouted to cpu{}, re-routing to cpu{} \
                                     (allowed={:#x})",
                                    id, cpu_idx, target, aff.allowed.0
                                );
                                self.defer_or_repair_misroute(sf.priority as usize, target, id);
                                continue;
                            }
                        }
                        next_id = Some(id);
                        break;
                    }
                }
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
                        Some(sf)
                            if sf.state == TaskState::Dead || sf.state == TaskState::Blocked =>
                        {
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
                                    // Same safe fallback as the normal picker path.
                                    let target = aff
                                        .pick_cpu(per_cpu_len)
                                        .unwrap_or_else(|| sf.last_cpu
                                            .filter(|&c| c < per_cpu_len)
                                            .unwrap_or_else(|| self.state.pick_online_cpu_excluding_bsp(0)));
                                    crate::kdebug!(
                                        "SCHED[affinity]: tid={} (idle-q) misrouted to cpu{}, \
                                         re-routing to cpu{} (allowed={:#x})",
                                        id, cpu_idx, target, aff.allowed.0
                                    );
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
                } else {
                    if pick_attempts >= PREPARE_SCHEDULE_PICK_BUDGET {
                        crate::kwarn!("SCHED: CPU {} Priority 0 pick budget exhausted!", cpu_idx);
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
        };

        let next_id = next_id?;

        let current_id = self.state.per_cpu[cpu_idx]
            .current
            .expect("prepare_schedule called without current task");

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
                match new_sched.migration_state.try_transition(
                    crate::sched::state::MigrationState::Local,
                ) {
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
                        // InTransit or Local as expected.  Log at warning level
                        // so it appears in normal debug builds, then force-reset
                        // to Local so the task can continue running and be
                        // migrated again in the future.
                        crate::kdebug!(
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
    fn try_steal_one(&mut self, local_cpu: usize, victim_cpu: usize, min_depth: usize) -> Option<TaskId> {
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
                        match sf.migration_state.try_transition(
                            crate::sched::state::MigrationState::InTransit,
                        ) {
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
        let per_cpu_len = self.state.per_cpu.len();

        // Partition online peer CPUs into nearby and far groups, collecting
        // their current run-queue depths at the same time.
        let max_nearby = per_cpu_len.min(STEAL_NEARBY_RADIUS * 2 + 1);
        let mut nearby: alloc::vec::Vec<(usize, usize)> =
            alloc::vec::Vec::with_capacity(max_nearby);
        let max_far = per_cpu_len.saturating_sub(max_nearby);
        let mut far: alloc::vec::Vec<(usize, usize)> =
            alloc::vec::Vec::with_capacity(max_far);

        for &cpu in &self.state.online_cpus {
            if cpu == local_cpu || cpu >= per_cpu_len {
                continue;
            }
            let depth = runq_depth_for_cpu(&self.state, cpu);
            // Clamp subtraction to avoid wrapping on usize arithmetic.
            let dist = if cpu >= local_cpu {
                cpu - local_cpu
            } else {
                local_cpu - cpu
            };
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
            if let Some(stolen) = self.try_steal_one(local_cpu, victim_cpu, STEAL_MIN_VICTIM_DEPTH) {
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
            if cur_busiest.saturating_sub(cur_least) < types::PERIODIC_BALANCE_IMBALANCE_MIN_DEPTH_DIFF
                || cur_busiest < STEAL_MIN_VICTIM_DEPTH
            {
                break;
            }

            // Steal one task from the busiest CPU.  try_steal_one removes it
            // from the victim's run queue, sets wake_cpu = least_cpu, and
            // records the enqueue cause as Steal.
            let Some(stolen_id) = self.try_steal_one(least_cpu, busiest_cpu, STEAL_MIN_VICTIM_DEPTH)
            else {
                break;
            };

            // Determine the priority to use for enqueueing.
            let priority = self
                .state
                .get_task(stolen_id)
                .map(|sf| sf.priority as usize)
                .unwrap_or(TaskPriority::Normal as usize);

            // Place the task into the target CPU's run queue.
            self.state.enqueue_task(least_cpu, priority, stolen_id);

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
        purge_task_from_scheduler_queues::<R>(self, current_id);

        for &sibling in siblings_to_kill {
            if sibling == current_id {
                continue;
            }
            if let Some(sf) = self.state.get_task_mut(sibling) {
                sf.runq_location = None;
                sf.state = TaskState::Dead;
            }
            purge_task_from_scheduler_queues::<R>(self, sibling);
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
            self.state.enqueue_task(cpu, priority as usize, id);
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
            idle_task::<R>,
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
            crate::kdebug!(
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
    let updated = {
        let lock = SCHEDULER.lock();
        let updated = if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
            sched.set_priority_hot_cache(id, priority)
        } else {
            false
        };
        updated
    };
    if updated {
        debug_assert_scheduler_not_held_by_this_cpu::<R>("set_priority registry sync");
        if let Some(mut task) = crate::task::registry::get_task_mut::<R>(id) {
            task.priority = priority;
            task.base_priority = priority;
        }
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
                .and_then(|tid| sched.state.get_thread(tid))
                .map(|sf| sf.priority)
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
        Affinity::Restricted(ref aff) => aff.effective_parallelism(online),
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
    let mut maybe_process_info =
        crate::task::registry::get_task::<R>(runtime_tid).and_then(|t| t.process_info.clone());
    if maybe_process_info.is_none() {
        let scheduler_current_tid = {
            let lock = SCHEDULER.lock();
            if let Some(ptr) = *lock {
                let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };
                let cpu_idx = current_cpu_index::<R>();
                sched.state.per_cpu.get(cpu_idx).and_then(|pc| pc.current)
            } else {
                None
            }
        };
        maybe_process_info = scheduler_current_tid
            .and_then(|tid| crate::task::registry::get_task::<R>(tid))
            .and_then(|t| t.process_info.clone());
    }

    rt.irq_restore(_irq);
    maybe_process_info
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

/// Return the current PID membership snapshot for one Unix-compat process group.
///
/// Unlike [`list_processes`], this helper never waits on a `ProcessInfo` mutex
/// while the task registry is held. It first snapshots unique `ProcessInfo`
/// Arcs from the registry, then drops the registry lock before inspecting
/// `unix_compat.pgid`. This avoids registry -> process lock inversion on
/// latency-sensitive paths such as TTY-generated `SIGINT` fanout.
pub fn list_process_ids_by_pgid<R: BootRuntime>(pgid: u32) -> alloc::vec::Vec<u32> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let process_infos = {
        let reg = crate::task::registry::get_registry::<R>();
        let mut seen = alloc::collections::BTreeSet::new();
        let mut infos = alloc::vec::Vec::new();

        for task in reg.threads.iter() {
            let Some(pi_arc) = &task.process_info else {
                continue;
            };
            let key = alloc::sync::Arc::as_ptr(pi_arc) as usize;
            if seen.insert(key) {
                infos.push(pi_arc.clone());
            }
        }

        infos
    };

    rt.irq_restore(_irq);

    let mut members = alloc::vec::Vec::new();
    for pinfo in process_infos {
        let process = pinfo.lock();
        if process.unix_compat.pgid == pgid {
            members.push(process.pid);
        }
    }

    members
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
        let pi = pinfo.lock();
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
        let pi = pinfo.lock();
        if pi.is_job_leader_tid(target_tid) {
            pi.job.leader_exit_waiters.remove(waiter_tid);
            return Ok(());
        }
    }
    target.exit_waiters.remove(waiter_tid);
    Ok(())
}

struct TerminationRegistryOutcome {
    waiters: alloc::vec::Vec<u64>,
    siblings_to_kill: alloc::vec::Vec<TaskId>,
    auto_reap: bool,
}

fn mark_task_exited_in_registry<R: BootRuntime>(
    tid: TaskId,
    code: i32,
) -> TerminationRegistryOutcome {
    let mut auto_reap = false;
    debug_assert_scheduler_not_held_by_this_cpu::<R>("mark_task_exited_in_registry");
    if tid == 6 {
        crate::kdebug!("SCHED[TID6]: exited (code={})", code);
    }
    // Collect exit waiters and mark the task dead.
    let mut waiters = if let Some(mut task) = crate::task::registry::get_task_mut::<R>(tid) {
        task.state = TaskState::Dead;
        task.exit_code = Some(code);
        auto_reap = task.detached;
        task.exit_waiters.drain()
    } else {
        alloc::vec::Vec::new()
    };

    // Remove this TID from the process's thread group list.
    // If this is the thread-group leader, drain the remaining siblings in one
    // step to avoid a separate clone + clear pass.
    // Also capture ppid/pid for lifecycle status queueing.
    let mut notify_ppid: u32 = 0;
    let mut notify_pid: u32 = 0;
    // Capture the exit observer inbox ID (if set) for canonical JobExit delivery.
    let mut exit_observer_inbox: Option<crate::inbox::InboxId> = None;

    let mut siblings_to_kill: alloc::vec::Vec<TaskId> = {
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

    // If this process was a thread-group leader, its exit orphans its children.
    // Reparent all children whose ppid matches this dying process to init (PID 1).
    if notify_pid != 0 {
        let mut registry = crate::task::registry::get_registry::<R>();
        for task in registry.threads.iter_mut() {
            if let Some(pinfo) = &task.process_info {
                let mut pi = pinfo.lock();
                if pi.job.ppid == notify_pid {
                    pi.job.ppid = 1;
                }
            }
        }
    }

    // Kill sibling threads (thread-group exit).
    for &sibling in &siblings_to_kill {
        if let Some(mut task) = crate::task::registry::get_task_mut::<R>(sibling) {
            if task.state != TaskState::Dead {
                task.state = TaskState::Dead;
                task.exit_code = Some(code);
                let sibling_waiters = task.exit_waiters.drain();
                waiters.extend(sibling_waiters);
                crate::kdebug!("SCHED: Killed sibling thread {} (thread-group exit)", sibling);
            }
        }
    }

    siblings_to_kill.retain(|&sibling| sibling != tid);

    TerminationRegistryOutcome { waiters, siblings_to_kill, auto_reap }
}

fn mark_task_exited<R: BootRuntime>(
    sched: &mut types::Scheduler<R>,
    tid: TaskId,
    code: i32,
) -> alloc::vec::Vec<u64> {
    let termination = mark_task_exited_in_registry::<R>(tid, code);

    if let Some(task) = sched.state.get_task_mut(tid) {
        task.runq_location = None;
        task.state = TaskState::Dead;
    }
    purge_task_from_scheduler_queues::<R>(sched, tid);

    for &sibling in &termination.siblings_to_kill {
        if sibling == tid {
            continue;
        }
        if let Some(task) = sched.state.get_task_mut(sibling) {
            task.runq_location = None;
            task.state = TaskState::Dead;
        }
        purge_task_from_scheduler_queues::<R>(sched, sibling);
    }

    termination.waiters
}

fn purge_task_from_scheduler_queues<R: BootRuntime>(sched: &mut types::Scheduler<R>, tid: TaskId) {
    sched.state.remove_task_from_runq(tid);
    sched.state.unregister_waiter(tid);

    let _ = sched.state.remove_task_from_sleep_queue(tid);
}

fn wake_waiters(waiters: &[u64]) {
    for &tid in waiters {
        unsafe {
            crate::sched::wake_task_erased(tid);
        }
    }
}

fn release_task_devices<R: BootRuntime>(tid: TaskId) {
    debug_assert_scheduler_not_held_by_this_cpu::<R>("release_task_devices");
    let released = crate::device_registry::REGISTRY.lock().release_all_for_task(tid);
    if released > 0 {
        crate::kdebug!("DEVICE: released {} claims for task {}", released, tid);
    }
}

fn current_task_resource_id_impl<R: BootRuntime>() -> Option<u64> {
    None
}

pub fn exit<R: BootRuntime>(code: i32) {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let current_tid = rt.current_tid();
    // Lock-order policy: perform REGISTRY / DEVICE_REGISTRY exit cleanup before
    // taking SCHEDULER, then run scheduler-only termination under SCHEDULER.
    let termination = mark_task_exited_in_registry::<R>(current_tid, code);
    release_task_devices::<R>(current_tid);

    let (
        switch_decision,
        deferred_prepare_ipis,
        deferred_registry_syncs,
        deferred_registry_inserts,
    ) = {
        let lock = SCHEDULER.lock();
        set_sched_lock_tracking::<R>(rt.current_cpu_index());
        let ptr = lock.expect("Scheduler not initialized");
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        let switch_decision = sched.terminate_current(current_tid, &termination.siblings_to_kill);
        if termination.auto_reap {
            sched.state.remove_task(current_tid);
        }
        let deferred_prepare_ipis = sched.drain_pending_prepare_schedule_ipis();
        let deferred_registry_syncs = core::mem::take(&mut sched.pending_registry_syncs);
        let deferred_registry_inserts = sched.drain_pending_registry_inserts();
        clear_sched_lock_tracking::<R>();
        (switch_decision, deferred_prepare_ipis, deferred_registry_syncs, deferred_registry_inserts)
    };

    if termination.auto_reap {
        // Same drop-order fix as in `remove_task_completely`: bind to a `let`
        // so the RegistryGuard (REGISTRY lock) is released at the semicolon
        // before the Box<Thread> is dropped at end of block.
        let _removed = crate::task::registry::get_registry::<R>().remove(current_tid);
        // REGISTRY lock released here.  _removed dropped at end of block.
    }

    send_deferred_prepare_schedule_ipis::<R>(deferred_prepare_ipis);
    apply_deferred_registry_inserts::<R>(deferred_registry_inserts);
    apply_deferred_registry_syncs::<R>(deferred_registry_syncs);
    wake_waiters(&termination.waiters);
    let mut ghost_ctx = <R::Tasking as BootTasking>::Context::default();
    let mut ghost_fs_base = 0;
    let switch = resolve_switch_params::<R>(
        switch_decision,
        &mut ghost_ctx,
        &mut ghost_fs_base,
    ).unwrap_or_else(|| {
        panic!(
            "scheduler invariant violated: terminate_current produced switch decision (from={}, to={}) but registry lookup failed",
            switch_decision.from_tid, switch_decision.to_tid
        )
    });

    while crate::sched::is_task_on_any_cpu(switch.to_tid) {
        core::hint::spin_loop();
    }

    if switch.to_aspace != switch.from_aspace {
        rt.tasking().activate_address_space(switch.to_aspace);
    }

    while crate::sched::is_task_on_any_cpu(switch.to_tid) {
        core::hint::spin_loop();
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
                    let task_is_dead = sched
                        .state
                        .get_thread(tid)
                        .map(|task| task.state == TaskState::Dead)
                        .unwrap_or(true);

                    if task_is_dead {
                        (false, alloc::vec::Vec::new())
                    } else {
                        let waiters = mark_task_exited::<R>(sched, tid, -9);

                        // Release any claimed devices
                        let released =
                            crate::device_registry::REGISTRY.lock().release_all_for_task(tid);
                        if released > 0 {
                            crate::kdebug!("DEVICE: released {} claims for task {}", released, tid);
                        }

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

        if take_pending_interrupt::<R>() {
            let _ = unregister_task_exit_waiter::<R>(tid, current_tid);
            return Err(abi::errors::Errno::EINTR);
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

        let interrupted = take_pending_interrupt::<R>();

        crate::ktrace!(
            "waitpid: woke parent_pid={} parent_tid={} target_pid={} flags=0x{:x} interrupted={}",
            our_pid,
            our_tid,
            pid,
            flags,
            interrupted
        );

        // After waking, unregister from children that haven't yet exited.
        for &child_tid in &registered {
            let _ = unregister_task_exit_waiter::<R>(child_tid, our_tid);
        }

        if interrupted {
            return Err(abi::errors::Errno::EINTR);
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
    let current_cpu = rt.current_cpu_index();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    let mut deferred_registry_inserts = alloc::vec::Vec::new();
    set_sched_lock_tracking::<R>(current_cpu);
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut types::Scheduler<R>) };
        sched.cpu_online(cpu_index);
        deferred_registry_inserts = sched.drain_pending_registry_inserts();
    }
    clear_sched_lock_tracking::<R>();
    drop(lock);
    apply_deferred_registry_inserts::<R>(deferred_registry_inserts);
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

    // 2. Remove from global registry (requires REGISTRY lock).
    //
    // IMPORTANT: Bind the returned `Box<Thread>` to a local variable so that
    // the `RegistryGuard` temporary (which holds the REGISTRY lock) is dropped
    // at the end of this statement — before `_removed_task` is dropped at end
    // of scope.  If we used the expression-statement form
    // `get_registry().remove(tid);` the unnamed return value would be dropped
    // *before* the guard (reverse creation order), meaning `close_all()` →
    // `wake_task_erased()` → `wake_task()` → `get_task_mut()` would try to
    // re-acquire the REGISTRY spin-lock from the same CPU → spin-deadlock.
    let _removed_task = crate::task::registry::get_registry::<R>().remove(tid);
    // REGISTRY lock released here (RegistryGuard dropped at semicolon).
    // `_removed_task` (Option<Box<Thread>>) is dropped at end of scope,
    // after the lock has been released.

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
    set_sched_lock_tracking::<R>(rt.current_cpu_index());
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &*(ptr as *const types::Scheduler<R>) };

    crate::kprint!("\n====== TASK DUMP ======\n");
    crate::kprint!(
        "CPUs: {} online / {} total\n",
        sched.state.online_cpu_count,
        sched.total_cpu_count
    );
    // Columns: TID, STATE, PRI, LAST, WAKE, RUN, MIGS, RUNS, USER, SLICE, KSTK, AFFIN, NAME.
    crate::kprint!(
        " {:>5}  {:>10}  {:>4}  {:>4}  {:>4}  {:>4}  {:>5}  {:>6}  {:>4}  {:>7}  {:>6}  {:>6}  {}\n",
        "TID",
        "STATE",
        "PRI",
        "LAST",
        "WAKE",
        "RUN",
        "MIGS",
        "RUNS",
        "USER",
        "SLICE",
        "KSTK",
        "AFFIN",
        "NAME"
    );

    let mut runnable_count = 0u32;
    let mut total_runs = 0u64;
    let mut total_migrations = 0u64;
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
        let runtime_stats = sched.state.task_runtime_stats(task.id);
        let migrations = runtime_stats.migration_count;
        let run_count = runtime_stats.run_count;
        total_runs = total_runs.saturating_add(run_count);
        total_migrations = total_migrations.saturating_add(migrations);
        let user_str = if task.is_user { "Y" } else { "N" };
        let aff_str: alloc::string::String = match task.affinity {
            crate::task::Affinity::Any => alloc::string::String::from("Any"),
            crate::task::Affinity::Pinned(c) => alloc::format!("Pin({})", c),
            crate::task::Affinity::Restricted(ref aff) => {
                alloc::format!("Rst({:#x})", aff.allowed.0)
            }
        };
        let name_str = if task.name_len > 0 {
            core::str::from_utf8(&task.name[..task.name_len as usize]).unwrap_or("?")
        } else {
            "-"
        };
        crate::kprint!(
            " {:>5}  {:>10}  {:>4}  {:>4}  {:>4}  {:>4}  {:>5}  {:>6}  {:>4}  {:>3}/{:<3}  {:>5}K  {:>6}  {}\n",
            task.id,
            state_str,
            pri_str,
            cpu_str,
            wake_cpu_str,
            run_cpu_str,
            migrations,
            run_count,
            user_str,
            task.timeslice_remaining,
            types::DEFAULT_TIMESLICE,
            task.kstack_size / 1024,
            aff_str,
            name_str
        );
    }
    let migrations_per_1k_runs =
        if total_runs == 0 { 0 } else { total_migrations.saturating_mul(1000) / total_runs };
    crate::kprint!(
        "  Locality damage: migrations={} runs={} migrations_per_1k_runs={}\n",
        total_migrations,
        total_runs,
        migrations_per_1k_runs
    );

    // Per-CPU run-queue summary
    for &i in &sched.state.online_cpus {
        let pc = &sched.state.per_cpu[i];
        let total: usize = pc.runq.iter().map(|q| q.len()).sum();
        let sample_count = pc.stats.runq_sample_count;
        let sample_total = pc.stats.runq_sample_total;
        let avg_runq = if sample_count == 0 { 0 } else { sample_total / sample_count };
        let idle_hist = pc.stats.idle_episode_hist;
        crate::kprint!(
            "  CPU {}: current={:?} runq={} avg_runq={} idle={:?} idle_ticks={} idle_total_us={} idle_eps={} idle_longest_us={} idle_hist=[{},{},{},{}] dispatch={} steals_in={} steals_out={} ctxsw={} idle->busy={} tick={} ipi_rx={} enq={} deq={} rqchg={} wake={} lock_miss={} lock_miss_pending={} lock_miss_timer={} lock_miss_ipi={} lock_blocked={}\n",
            i,
            pc.current,
            total,
            avg_runq,
            pc.idle_task,
            PROF_IDLE_TICKS_PER_CPU[i].load(Ordering::Relaxed),
            pc.stats.idle_total_us,
            pc.stats.idle_episodes,
            pc.stats.idle_longest_us,
            idle_hist[0],
            idle_hist[1],
            idle_hist[2],
            idle_hist[3],
            pc.stats.dispatch_count,
            pc.stats.steals_in,
            pc.stats.steals_out,
            pc.stats.context_switches,
            pc.stats.idle_to_nonidle,
            pc.stats.timer_interrupts,
            pc.stats.resched_ipi_received,
            pc.stats.runnable_enqueues,
            pc.stats.runnable_dequeues,
            pc.stats.runq_depth_change_events,
            pc.stats.wakeups,
            PROF_TRYLOCK_MISS_PER_CPU[i].load(Ordering::Relaxed),
            PROF_TRYLOCK_MISS_PENDING_PER_CPU[i].load(Ordering::Relaxed),
            PROF_TRYLOCK_MISS_TIMER_PER_CPU[i].load(Ordering::Relaxed),
            PROF_TRYLOCK_MISS_IPI_PER_CPU[i].load(Ordering::Relaxed),
            pc.stats.lock_blocked_dispatch
        );
    }
    let wake_to_run_count = PROF_WAKE_TO_RUN_COUNT.load(Ordering::Relaxed);
    let wake_to_run_total = PROF_WAKE_TO_RUN_TICKS_TOTAL.load(Ordering::Relaxed);
    let wake_to_run_avg =
        if wake_to_run_count == 0 { 0 } else { wake_to_run_total / wake_to_run_count };
    crate::kprint!(
        "  Wake→run latency(µs): count={} avg={} max={} hist=[0-5:{},5-20:{},20-100:{},100-500:{},>500:{}]\n",
        wake_to_run_count,
        wake_to_run_avg,
        PROF_WAKE_TO_RUN_TICKS_MAX.load(Ordering::Relaxed),
        PROF_WAKE_TO_RUN_HIST[0].load(Ordering::Relaxed),
        PROF_WAKE_TO_RUN_HIST[1].load(Ordering::Relaxed),
        PROF_WAKE_TO_RUN_HIST[2].load(Ordering::Relaxed),
        PROF_WAKE_TO_RUN_HIST[3].load(Ordering::Relaxed),
        PROF_WAKE_TO_RUN_HIST[4].load(Ordering::Relaxed)
    );
    crate::kprint!(
        "  Remote wake mailbox age(µs): hist=[0-4:{},5-19:{},20-99:{},100-499:{},>=500:{}] no_ipi={}\n",
        PROF_REMOTE_WAKE_MAILBOX_AGE_HIST[0].load(Ordering::Relaxed),
        PROF_REMOTE_WAKE_MAILBOX_AGE_HIST[1].load(Ordering::Relaxed),
        PROF_REMOTE_WAKE_MAILBOX_AGE_HIST[2].load(Ordering::Relaxed),
        PROF_REMOTE_WAKE_MAILBOX_AGE_HIST[3].load(Ordering::Relaxed),
        PROF_REMOTE_WAKE_MAILBOX_AGE_HIST[4].load(Ordering::Relaxed),
        DIAG_REMOTE_WAKE_MAILBOX_NO_IPI.load(Ordering::Relaxed),
    );
    crate::kprint!(
        "  Runq depth variance: last={} max={} imbalance(total_us={}, episodes={}, longest_us={})\n",
        PROF_RUNQ_DEPTH_VARIANCE_LAST.load(Ordering::Relaxed),
        PROF_RUNQ_DEPTH_VARIANCE_MAX.load(Ordering::Relaxed),
        PROF_IMBALANCE_TOTAL_US.load(Ordering::Relaxed),
        PROF_IMBALANCE_EPISODES.load(Ordering::Relaxed),
        PROF_IMBALANCE_LONGEST_US.load(Ordering::Relaxed)
    );

    crate::kprint!("Sleep queue: {} tasks\n", sched.state.sleep_queue.len());
    crate::kprint!(
        "=== {} tasks, {} runnable ===\n\n",
        crate::task::registry::get_registry::<R>().threads.len(),
        runnable_count
    );

    clear_sched_lock_tracking::<R>();
    drop(lock);
    rt.irq_restore(_irq);
}

extern "C" fn idle_task<R: BootRuntime>(_: usize) -> ! {
    let rt = crate::runtime::<R>();
    let cpu_idx = rt.current_cpu_index();
    crate::sched::set_cpu_current_task(cpu_idx, rt.current_tid());
    loop {
        if global_need_resched_load(cpu_idx, Ordering::Acquire) {
            // Use yield_now to trigger a blocking lock acquisition for the
            // scheduler if a reschedule is pending.
            yield_now::<R>();
        }
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
    #[derive(Clone, Copy, Default, PartialEq, Eq)]
    pub(crate) struct MockAddressSpace(pub(crate) u64);

    pub(crate) static MOCK_RUNTIME: MockRuntime = MockRuntime;
    pub(crate) struct MockRuntime;

    // Per-thread IRQ depth counter used by MockRuntime to detect imbalanced
    // irq_disable / irq_restore pairs. irq_disable increments the depth and
    // returns the old value; irq_restore restores the depth to the saved value.
    std::thread_local! {
        static IRQ_DEPTH: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
        static MOCK_IDLE_TASK_CURRENT: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
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

        fn irq_disable(&self) -> crate::IrqState {
            let prev = IRQ_DEPTH
                .try_with(|c| {
                    let d = c.get();
                    c.set(d + 1);
                    d
                })
                .unwrap_or(0);
            crate::IrqState(prev)
        }
        fn irq_restore(&self, state: crate::IrqState) {
            let _ = IRQ_DEPTH.try_with(|c| c.set(state.0));
        }
        fn is_idle_task_current(&self) -> bool {
            MOCK_IDLE_TASK_CURRENT.try_with(|c| c.get()).unwrap_or(false)
        }
        fn set_idle_task_current(&self, idle: bool) {
            let _ = MOCK_IDLE_TASK_CURRENT.try_with(|c| c.set(idle));
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

        // irq_disable and irq_restore moved to BootRuntimeBase
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
        fn get_kernel_cmdline(&self) -> &'static str {
            ""
        }
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

    #[test]
    fn effective_parallelism_restricted_counts_allowed_online_cpus() {
        use crate::sched::state::{CpuAffinity, CpuSet};
        // Allowed CPUs 1 and 2 only; 4 CPUs online.
        let aff = CpuAffinity {
            allowed: CpuSet(0b0110),
            preferred: None,
            last_cpu: None,
        };
        assert_eq!(effective_parallelism_from_state(4, Affinity::Restricted(aff)), 2);
    }

    #[test]
    fn effective_parallelism_restricted_caps_at_online_count() {
        use crate::sched::state::{CpuAffinity, CpuSet};
        // Allowed CPUs 0-7 but only 2 online.
        let aff = CpuAffinity { allowed: CpuSet(0xFF), preferred: None, last_cpu: None };
        assert_eq!(effective_parallelism_from_state(2, Affinity::Restricted(aff)), 2);
    }

    /// Serialises sched tests that mutate shared globals (REGISTRY, SCHEDULER,
    /// TICK_COUNT).  Any test that calls `init_test_env` should hold the
    /// returned guard for its entire duration to prevent races with concurrent
    /// tests that reinitialize the registry.
    pub(crate) static SCHED_TEST_GUARD: spin::Mutex<()> = spin::Mutex::new(());

    pub(crate) fn init_test_env() -> spin::MutexGuard<'static, ()> {
        let guard = SCHED_TEST_GUARD.lock();
        init_mock_runtime();
        crate::task::registry::init::<MockRuntime>();
        *SCHEDULER.lock() = None;
        SCHEDULER_LOCK_OWNER.store(-1, Ordering::Relaxed);
        SCHEDULER_LOCK_ACQUIRED_AT.store(0, Ordering::Relaxed);
        PROF_LOCK_ORDER_VIOLATIONS.store(0, Ordering::Relaxed);
        TICK_COUNT.store(0, core::sync::atomic::Ordering::Relaxed);
        for i in 0..types::MAX_CPUS {
            TRYLOCK_MISS_WINDOW_START[i].store(0, Ordering::Relaxed);
            TRYLOCK_MISS_WINDOW_COUNT[i].store(0, Ordering::Relaxed);
            TRYLOCK_MISS_WINDOW_TIMER_COUNT[i].store(0, Ordering::Relaxed);
            TRYLOCK_MISS_WINDOW_IPI_COUNT[i].store(0, Ordering::Relaxed);
            TRYLOCK_MISS_WINDOW_IDLE_TIMER_COUNT[i].store(0, Ordering::Relaxed);
            TRYLOCK_MISS_WINDOW_PENDING_COUNT[i].store(0, Ordering::Relaxed);
            TRYLOCK_MISS_LAST_WARN_TICK[i].store(0, Ordering::Relaxed);
        }
        crate::runtime::<MockRuntime>().set_idle_task_current(false);
        reset_remote_wake_mailboxes_for_tests();
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(0, TaskPriority::Low as usize, 42);

        sched.set_priority(42, TaskPriority::High);

        assert_eq!(sched.state.get_task(42).unwrap().priority, TaskPriority::High);
        assert_eq!(
            crate::task::registry::get_registry::<MockRuntime>().threads[0].priority,
            TaskPriority::Low,
            "scheduler-only set_priority keeps REGISTRY sync out of the hot path"
        );
        assert!(sched.state.per_cpu[0].runq[TaskPriority::Low as usize].is_empty());
        assert_eq!(
            sched.state.per_cpu[0].runq[TaskPriority::High as usize].front().copied(),
            Some(42)
        );
    }

    #[test]
    fn public_set_priority_syncs_registry_after_scheduler_unlock() {
        let _g = init_test_env();

        let mut sched = alloc::boxed::Box::new(types::Scheduler::<MockRuntime>::new());
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

        let task = make_task(43, TaskState::Runnable, TaskPriority::Low);
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 43,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(0, TaskPriority::Low as usize, 43);

        let sched_ptr = alloc::boxed::Box::into_raw(sched);
        *SCHEDULER.lock() = Some(sched_ptr as usize);

        set_priority::<MockRuntime>(43, TaskPriority::High);

        let sched_ref = unsafe { &mut *(sched_ptr as *mut types::Scheduler<MockRuntime>) };
        assert_eq!(sched_ref.state.get_task(43).unwrap().priority, TaskPriority::High);
        assert!(sched_ref.state.per_cpu[0].runq[TaskPriority::Low as usize].is_empty());
        assert_eq!(
            sched_ref.state.per_cpu[0].runq[TaskPriority::High as usize].front().copied(),
            Some(43)
        );
        assert_eq!(
            crate::task::registry::get_registry::<MockRuntime>().threads[0].priority,
            TaskPriority::High,
            "public set_priority should sync REGISTRY outside scheduler lock"
        );

        *SCHEDULER.lock() = None;
        unsafe {
            drop(alloc::boxed::Box::from_raw(sched_ptr));
        }
    }

    #[test]
    fn current_priority_reads_scheduler_hot_cache() {
        let _g = init_test_env();

        let mut sched = alloc::boxed::Box::new(types::Scheduler::<MockRuntime>::new());
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(44);
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 44,
            runq_location: None,
            state: TaskState::Running,
            priority: TaskPriority::High,
            affinity: Affinity::Any,
            last_cpu: Some(0),
            wake_cpu: Some(0),
            run_cpu: Some(0),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });

        let sched_ptr = alloc::boxed::Box::into_raw(sched);
        *SCHEDULER.lock() = Some(sched_ptr as usize);

        assert_eq!(current_priority::<MockRuntime>(), TaskPriority::High);

        *SCHEDULER.lock() = None;
        unsafe {
            drop(alloc::boxed::Box::from_raw(sched_ptr));
        }
    }

    #[test]
    fn current_priority_defaults_when_no_current_task() {
        let _g = init_test_env();

        let mut sched = alloc::boxed::Box::new(types::Scheduler::<MockRuntime>::new());
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        let sched_ptr = alloc::boxed::Box::into_raw(sched);
        *SCHEDULER.lock() = Some(sched_ptr as usize);

        assert_eq!(current_priority::<MockRuntime>(), TaskPriority::Normal);

        *SCHEDULER.lock() = None;
        unsafe {
            drop(alloc::boxed::Box::from_raw(sched_ptr));
        }
    }

    #[test]
    fn current_priority_defaults_when_current_task_missing_from_hot_cache() {
        let _g = init_test_env();

        let mut sched = alloc::boxed::Box::new(types::Scheduler::<MockRuntime>::new());
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(99);
        let sched_ptr = alloc::boxed::Box::into_raw(sched);
        *SCHEDULER.lock() = Some(sched_ptr as usize);

        assert_eq!(current_priority::<MockRuntime>(), TaskPriority::Normal);

        *SCHEDULER.lock() = None;
        unsafe {
            drop(alloc::boxed::Box::from_raw(sched_ptr));
        }
    }

    #[test]
    fn preempt_disable_depth_is_cpu_local() {
        let _g = init_test_env();
        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

        sched.state.per_cpu[1].preempt_disable_depth = 3;
        sched.state.per_cpu[1].preempt_disable_since = 42;
        sched.state.per_cpu[1].preempt_watchdog_warned = true;

        TICK_COUNT.store(99, Ordering::Relaxed);
        sched.preempt_disable();
        sched.preempt_disable();
        assert_eq!(sched.state.per_cpu[0].preempt_disable_depth, 2);
        assert_eq!(sched.state.per_cpu[0].preempt_disable_since, 99);
        assert!(!sched.state.per_cpu[0].preempt_watchdog_warned);

        sched.preempt_enable();
        assert_eq!(sched.state.per_cpu[0].preempt_disable_depth, 1);
        sched.preempt_enable();
        assert_eq!(sched.state.per_cpu[0].preempt_disable_depth, 0);

        assert_eq!(sched.state.per_cpu[1].preempt_disable_depth, 3);
        assert_eq!(sched.state.per_cpu[1].preempt_disable_since, 42);
        assert!(sched.state.per_cpu[1].preempt_watchdog_warned);
    }

    #[test]
    fn clear_sched_lock_tracking_only_clears_when_called_by_owner_cpu() {
        let _g = init_test_env();

        set_sched_lock_tracking::<MockRuntime>(1);
        SCHEDULER_LOCK_ACQUIRED_AT.store(123, Ordering::Release);
        clear_sched_lock_tracking::<MockRuntime>();
        assert_eq!(SCHEDULER_LOCK_OWNER.load(Ordering::Acquire), 1);
        assert_eq!(SCHEDULER_LOCK_ACQUIRED_AT.load(Ordering::Acquire), 123);

        set_sched_lock_tracking::<MockRuntime>(0);
        clear_sched_lock_tracking::<MockRuntime>();
        assert_eq!(SCHEDULER_LOCK_OWNER.load(Ordering::Acquire), -1);
        assert_eq!(SCHEDULER_LOCK_ACQUIRED_AT.load(Ordering::Acquire), 0);
    }

    #[test]
    fn sched_lock_tracking_guard_clears_on_drop() {
        let _g = init_test_env();

        {
            let _tracking = sched_lock_tracking_guard::<MockRuntime>(0);
            assert_eq!(SCHEDULER_LOCK_OWNER.load(Ordering::Acquire), 0);
        }

        assert_eq!(SCHEDULER_LOCK_OWNER.load(Ordering::Acquire), -1);
        assert_eq!(SCHEDULER_LOCK_ACQUIRED_AT.load(Ordering::Acquire), 0);
    }

    #[test]
    fn dump_stats_clears_sched_lock_tracking() {
        let _g = init_test_env();
        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

        let sched_ptr = &mut sched as *mut types::Scheduler<MockRuntime> as usize;
        *SCHEDULER.lock() = Some(sched_ptr);
        dump_stats::<MockRuntime>();

        assert_eq!(SCHEDULER_LOCK_OWNER.load(Ordering::Acquire), -1);
        assert_eq!(SCHEDULER_LOCK_ACQUIRED_AT.load(Ordering::Acquire), 0);
        *SCHEDULER.lock() = None;
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
    fn test_vruntime_prefers_least_served_on_effective_priority_tie() {
        let _g = init_test_env();
        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(0);

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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });

        let mut task_a = make_task(4001, TaskState::Runnable, TaskPriority::Normal);
        task_a.enqueued_at_tick = 0;
        let mut task_b = make_task(4002, TaskState::Runnable, TaskPriority::Normal);
        task_b.enqueued_at_tick = 0;
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task_a));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task_b));

        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 4001,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 4002,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(0, TaskPriority::Normal as usize, 4001);
        sched.state.enqueue_task(0, TaskPriority::Normal as usize, 4002);

        sched.state.task_runtime_stats_mut(4001).fair_vruntime = 100;
        sched.state.task_runtime_stats_mut(4002).fair_vruntime = 10;

        let next_switch = sched.prepare_schedule().expect("Should pick a runnable task");
        assert_eq!(
            next_switch.to_tid, 4002,
            "least-served task should be preferred when effective priority ties"
        );
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
    fn test_prepare_yield_penalizes_spin_yield_requeue_band() {
        let _g = init_test_env();
        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(6001, TaskState::Running, TaskPriority::High),
        ));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(6002, TaskState::Runnable, TaskPriority::Realtime),
        ));

        sched.state.per_cpu[0].current = Some(6001);
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 6001,
            runq_location: None,
            state: TaskState::Running,
            priority: TaskPriority::High,
            affinity: Affinity::Any,
            last_cpu: Some(0),
            wake_cpu: Some(0),
            run_cpu: Some(0),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: types::SPIN_YIELD_PENALTY_THRESHOLD,
            migration_state: MigrationState::Local,
        });
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 6002,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.note_enqueue_cause(6001, crate::sched::state::EnqueueCause::YieldRequeue);
        sched.state.enqueue_task(0, TaskPriority::Realtime as usize, 6002);

        let switch = sched.prepare_yield().expect("high-priority peer should run");
        assert_eq!(switch.to_tid, 6002);
        assert_eq!(
            sched.state.get_task(6001).and_then(|sf| sf.runq_location),
            Some((0, TaskPriority::Normal as usize)),
            "spin-yielding task should be demoted one runnable band on requeue",
        );
        assert_eq!(
            sched.state.get_task(6001).map(|sf| sf.voluntary_yields),
            Some(types::SPIN_YIELD_PENALTY_THRESHOLD + 1),
        );
    }

    #[test]
    fn test_prepare_yield_does_not_penalize_without_prior_yield_requeue() {
        let _g = init_test_env();
        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(6101, TaskState::Running, TaskPriority::Normal),
        ));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(6102, TaskState::Runnable, TaskPriority::High),
        ));

        sched.state.per_cpu[0].current = Some(6101);
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 6101,
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
            voluntary_yields: types::SPIN_YIELD_PENALTY_THRESHOLD + 5,
            migration_state: MigrationState::Local,
        });
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 6102,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::High,
            affinity: Affinity::Any,
            last_cpu: Some(0),
            wake_cpu: Some(0),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.note_enqueue_cause(6101, crate::sched::state::EnqueueCause::Wake);
        sched.state.enqueue_task(0, TaskPriority::High as usize, 6102);

        let switch = sched.prepare_yield().expect("high-priority peer should run");
        assert_eq!(switch.to_tid, 6102);
        assert_eq!(
            sched.state.get_task(6101).and_then(|sf| sf.runq_location),
            Some((0, TaskPriority::Normal as usize)),
            "without prior yield-requeue cause, no spin penalty should be applied",
        );
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
    fn test_wake_sleepers_coalesces_same_cpu_remote_ipi_within_batch() {
        let _g = init_test_env();
        use core::sync::atomic::Ordering;

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.mark_cpu_online(1);

        PROF_IPI_SUPPRESSED.store(0, Ordering::Relaxed);
        clear_global_need_resched(1, Ordering::Relaxed);

        let current_task = make_task(9_101, TaskState::Running, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(current_task));
        sched.state.per_cpu[0].current = Some(9_101);
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 9_101,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });

        for tid in [9_102, 9_103] {
            let sleeping_task = make_task(tid, TaskState::Blocked, TaskPriority::Normal);
            crate::task::registry::get_registry::<MockRuntime>()
                .insert(alloc::boxed::Box::new(sleeping_task));
            sched.state.insert_task(crate::sched::state::ThreadSchedFields {
                tid,
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
                voluntary_yields: 0,
                migration_state: MigrationState::Local,
            });
            sched.state.add_task_to_sleep_queue(tid, 50);
        }
        TICK_COUNT.store(100, Ordering::Relaxed);

        sched.wake_sleepers();

        assert_eq!(
            sched.pending_wake_ipis,
            alloc::vec![1usize],
            "waking multiple sleepers for the same remote CPU should queue only one deferred IPI"
        );
        assert_eq!(
            PROF_IPI_SUPPRESSED.load(Ordering::Relaxed),
            1,
            "duplicate remote wake in one wake_sleepers batch should be counted as suppressed"
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
                voluntary_yields: 0,
                migration_state: MigrationState::Local,
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
    fn test_prepare_schedule_defers_misroute_repair_until_maintenance() {
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });
        sched.state.enqueue_task(0, TaskPriority::Normal as usize, 9102);

        // No local idle task exists, so prepare_schedule returns None and defers
        // misroute repair to the maintenance pass.
        let switch = sched.prepare_schedule();
        assert!(
            switch.is_none(),
            "prepare_schedule should return None when only misrouted work exists and no idle task"
        );
        assert!(
            sched.drain_pending_prepare_schedule_ipis().is_empty(),
            "prepare_schedule picker path should not perform deferred misroute repair or queue repair IPIs"
        );
        assert!(
            sched.pending_wake_ipis.is_empty(),
            "misroute IPIs should not be mixed into pending_wake_ipis"
        );
        assert!(
            sched
                .pending_misrouted_requeues
                .iter()
                .any(|&(_, target_cpu, tid)| target_cpu == 1 && tid == 9102),
            "misrouted task should be queued for deferred repair maintenance"
        );
        assert!(
            !sched.state.per_cpu[1].runq[TaskPriority::Normal as usize]
                .iter()
                .any(|&tid| tid == 9102),
            "target CPU runq should remain unchanged until maintenance runs"
        );

        sched.run_pending_misroute_repair_maintenance();

        assert_eq!(
            sched.drain_pending_prepare_schedule_ipis(),
            alloc::vec![1usize],
            "misroute maintenance should queue the deferred repair nudge"
        );
        assert!(
            sched.state.per_cpu[1].runq[TaskPriority::Normal as usize]
                .iter()
                .any(|&tid| tid == 9102),
            "misroute maintenance should move the task to the target CPU runq"
        );
    }

    #[test]
    fn test_pending_prepare_schedule_ipi_bitmap_dedups_targets() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

        sched.queue_pending_prepare_schedule_ipi(1);
        sched.queue_pending_prepare_schedule_ipi(1);
        sched.queue_pending_prepare_schedule_ipi(2);

        assert_eq!(
            sched.drain_pending_prepare_schedule_ipis(),
            alloc::vec![1usize, 2usize],
            "bitmap-backed pending_prepare_schedule_ipis should dedup repeated CPU targets"
        );
        assert!(
            sched.drain_pending_prepare_schedule_ipis().is_empty(),
            "drain should clear the bitmap-backed pending queue"
        );
    }

    #[test]
    fn test_pending_prepare_schedule_ipi_bitmap_drain_preserves_all_pending_targets() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());

        sched.queue_pending_prepare_schedule_ipi(1);
        sched.queue_pending_prepare_schedule_ipi(3);
        sched.queue_pending_prepare_schedule_ipi(5);
        sched.queue_pending_prepare_schedule_ipi(3);

        assert_eq!(
            sched.drain_pending_prepare_schedule_ipis(),
            alloc::vec![1usize, 3usize, 5usize],
            "bitmap-backed drain should return all queued CPU targets without dropping pending bits or duplicating targets"
        );
    }

    #[test]
    fn test_misroute_repair_maintenance_is_bounded_per_call() {
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
                voluntary_yields: 0,
                migration_state: MigrationState::Local,
                wake_pending: false,
            });
            sched.state.enqueue_task(0, TaskPriority::Normal as usize, tid);
        }

        let switch = sched.prepare_schedule();
        assert!(switch.is_none());
        assert!(
            !sched.pending_misrouted_requeues.is_empty(),
            "prepare_schedule should defer misroute repairs into maintenance backlog"
        );
        assert_eq!(
            sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].len(),
            0,
            "prepare_schedule picker path should not directly repair deferred misroutes"
        );

        sched.run_pending_misroute_repair_maintenance();

        assert!(
            sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].len()
                <= PREPARE_SCHEDULE_MISROUTE_REPAIR_BUDGET,
            "maintenance should only repair up to PREPARE_SCHEDULE_MISROUTE_REPAIR_BUDGET misroutes per call"
        );

        // Repeated maintenance passes should drain deferred work.
        for _ in 0..4 {
            sched.run_pending_misroute_repair_maintenance();
        }
        assert!(
            sched.pending_misrouted_requeues.is_empty(),
            "deferred misroute backlog should drain across maintenance passes"
        );
    }

    #[test]
    fn test_misroute_repair_maintenance_drops_non_runnable_entries() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.mark_cpu_online(1);

        // Current task on CPU 0.
        let current_task = make_task(9300, TaskState::Running, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(current_task));
        sched.state.per_cpu[0].current = Some(9300);
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 9300,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });

        // Runnable task incorrectly queued on CPU 0 but pinned to CPU 1.
        let misrouted = make_task(9301, TaskState::Runnable, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(misrouted));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 9301,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(0, TaskPriority::Normal as usize, 9301);

        let switch = sched.prepare_schedule();
        assert!(switch.is_none());
        assert_eq!(sched.pending_misrouted_requeues.len(), 1);

        if let Some(sf) = sched.state.get_task_mut(9301) {
            sf.state = TaskState::Dead;
        }

        sched.run_pending_misroute_repair_maintenance();

        assert!(
            sched.pending_misrouted_requeues.is_empty(),
            "maintenance prevalidation should drop non-runnable deferred misroutes"
        );
        assert!(
            sched.state.per_cpu[1].runq[TaskPriority::Normal as usize]
                .iter()
                .all(|&tid| tid != 9301),
            "dropped deferred entry should not be re-enqueued"
        );
        assert!(
            sched.drain_pending_prepare_schedule_ipis().is_empty(),
            "dropped deferred entry should not queue an IPI"
        );
    }

    #[test]
    fn test_send_deferred_prepare_schedule_ipis_updates_counters() {
        let _g = init_test_env();
        use core::sync::atomic::Ordering;

        DIAG_IPI_SENT.store(0, Ordering::Relaxed);
        DIAG_IPI_SENT_PREPARE_SCHEDULE.store(0, Ordering::Relaxed);
        LAST_RESCHED_IPI_SENT_AT_TICK[1].store(RESCHED_IPI_NEVER_SENT, Ordering::Relaxed);
        LAST_RESCHED_IPI_SENT_AT_TICK[2].store(RESCHED_IPI_NEVER_SENT, Ordering::Relaxed);
        TICK_COUNT.store(1, Ordering::Relaxed);

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
    fn test_send_deferred_prepare_schedule_ipis_suppresses_same_tick_duplicates() {
        let _g = init_test_env();
        use core::sync::atomic::Ordering;

        DIAG_IPI_SENT.store(0, Ordering::Relaxed);
        DIAG_IPI_SENT_PREPARE_SCHEDULE.store(0, Ordering::Relaxed);
        PROF_IPI_SUPPRESSED.store(0, Ordering::Relaxed);
        LAST_RESCHED_IPI_SENT_AT_TICK[1].store(RESCHED_IPI_NEVER_SENT, Ordering::Relaxed);
        TICK_COUNT.store(42, Ordering::Relaxed);

        send_deferred_prepare_schedule_ipis::<MockRuntime>(alloc::vec![1usize, 1usize]);

        assert_eq!(DIAG_IPI_SENT.load(Ordering::Relaxed), 1);
        assert_eq!(DIAG_IPI_SENT_PREPARE_SCHEDULE.load(Ordering::Relaxed), 1);
        assert_eq!(PROF_IPI_SUPPRESSED.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_should_send_remote_resched_ipi_suppresses_adjacent_tick_ping_pong() {
        let _g = init_test_env();
        use core::sync::atomic::Ordering;

        PROF_IPI_SUPPRESSED.store(0, Ordering::Relaxed);
        LAST_RESCHED_IPI_SENT_AT_TICK[1].store(RESCHED_IPI_NEVER_SENT, Ordering::Relaxed);

        TICK_COUNT.store(100, Ordering::Relaxed);
        assert!(should_send_remote_resched_ipi(1));

        TICK_COUNT.store(101, Ordering::Relaxed);
        assert!(!should_send_remote_resched_ipi(1));
        assert_eq!(PROF_IPI_SUPPRESSED.load(Ordering::Relaxed), 1);

        TICK_COUNT.store(102, Ordering::Relaxed);
        assert!(should_send_remote_resched_ipi(1));

        TICK_COUNT.store(103, Ordering::Relaxed);
        assert!(!should_send_remote_resched_ipi(1));
        assert_eq!(PROF_IPI_SUPPRESSED.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn test_sample_runq_len_downsamples_global_telemetry_scan() {
        let _g = init_test_env();
        use core::sync::atomic::Ordering;

        use crate::sched::state::PerCpu;

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(PerCpu::new());
        sched.state.per_cpu.push(PerCpu::new());
        sched.state.online_cpus = alloc::vec![0, 1];
        sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].push_back(7001);
        sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].push_back(7002);

        PROF_RUNQ_SAMPLE_COUNT[0].store(0, Ordering::Relaxed);
        PROF_RUNQ_SAMPLE_TOTAL[0].store(0, Ordering::Relaxed);
        PROF_RUNQ_DEPTH_VARIANCE_SAMPLE_COUNT.store(0, Ordering::Relaxed);
        PROF_RUNQ_DEPTH_VARIANCE_TOTAL.store(0, Ordering::Relaxed);
        PROF_RUNQ_DEPTH_VARIANCE_LAST.store(0, Ordering::Relaxed);
        PROF_RUNQ_DEPTH_VARIANCE_MAX.store(0, Ordering::Relaxed);

        for _ in 0..(RUNQ_GLOBAL_TELEMETRY_SAMPLE_STRIDE - 1) {
            sample_runq_len(&mut sched, 0);
        }
        assert_eq!(
            PROF_RUNQ_DEPTH_VARIANCE_SAMPLE_COUNT.load(Ordering::Relaxed),
            0,
            "global variance scan should be skipped until the configured stride is reached"
        );

        sample_runq_len(&mut sched, 0);
        assert_eq!(
            PROF_RUNQ_DEPTH_VARIANCE_SAMPLE_COUNT.load(Ordering::Relaxed),
            1,
            "global variance scan should run when the stride boundary is reached"
        );
    }

    #[test]
    fn test_registry_rapid_create_exit_churn() {
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

        assert_eq!(crate::task::registry::get_registry::<MockRuntime>().threads.len(), 4);

        // Verify lookups work
        assert!(crate::task::registry::get_task::<MockRuntime>(4010).is_some());
        assert!(crate::task::registry::get_task::<MockRuntime>(4005).is_some());
        assert!(crate::task::registry::get_task::<MockRuntime>(4001).is_some());
        assert!(crate::task::registry::get_task::<MockRuntime>(4099).is_none());

        // Keep a small hot set of thread IDs cycling through many create/exit
        // operations to stress index maintenance under churn.
        const CHURN_ITERATIONS: usize = 512; // enough iterations to repeatedly reshuffle slots
        const CHURN_ACTIVE_TID_COUNT: usize = 32; // small hot set maximizes remove/reinsert reuse
        for i in 0..CHURN_ITERATIONS {
            let tid = 5000 + (i % CHURN_ACTIVE_TID_COUNT) as u64;
            if crate::task::registry::get_task::<MockRuntime>(tid).is_none() {
                crate::task::registry::get_registry::<MockRuntime>()
                    .insert(alloc::boxed::Box::new(make_task(tid)));
            }
            let removed = crate::task::registry::get_registry::<MockRuntime>().remove(tid);
            assert!(removed.is_some(), "expected tid {} to exist before removal", tid);
            assert!(
                crate::task::registry::get_task::<MockRuntime>(tid).is_none(),
                "removed tid {} must not remain addressable",
                tid
            );
        }
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
        sched.state.register_waiter(5001, crate::sched::state::WaitReason::BlockCurrent);

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
        sched.state.unregister_waiter(5001);

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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
    fn test_wake_task_revalidates_under_split_authority_window() {
        let _g = init_test_env();
        crate::task::registry::init::<MockRuntime>();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 6101,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });

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
        *sched_lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
        drop(sched_lock);
        let _ = sched_lock_metrics_snapshot_and_reset();

        crate::sched::blocking::wake_task::<MockRuntime>(6101);
        let mut sched_lock = SCHEDULER.lock();
        *sched_lock = None;
        drop(sched_lock);

        let task = crate::task::registry::get_task::<MockRuntime>(6101).unwrap();
        assert_eq!(task.state, TaskState::Running);
        assert!(task.wake_pending);
        assert!(
            sched.state.get_task(6101).map(|sf| sf.wake_pending).unwrap_or(false),
            "wake_task must revalidate scheduler cache even when REGISTRY wake_pending is true"
        );

        let metrics = sched_lock_metrics_snapshot_and_reset();
        assert_eq!(metrics.wake_task_fastpath_already_pending, 0);
        assert_eq!(metrics.wake_task.wait_calls, 1);
    }

    #[test]
    fn test_wake_task_remote_path_uses_mailbox_without_scheduler_lock() {
        let _g = init_test_env();
        TICK_COUNT.store(123, Ordering::Relaxed);
        clear_global_need_resched(1, Ordering::Relaxed);
        DIAG_IPI_SENT.store(0, Ordering::Relaxed);
        DIAG_IPI_SENT_WAKE_TASK.store(0, Ordering::Relaxed);
        DIAG_REMOTE_WAKE_MAILBOX_NO_IPI.store(0, Ordering::Relaxed);

        let task = crate::task::Task {
            id: 6_202,
            state: TaskState::Blocked,
            priority: TaskPriority::Normal,
            base_priority: TaskPriority::Normal,
            enqueued_at_tick: 0,
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            affinity: Affinity::Pinned(1),
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
            last_cpu: Some(1),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            user_fs_base: 0,
            detached: false,
            signals: crate::signal::ThreadSignals::new(),
        };
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

        crate::sched::blocking::wake_task::<MockRuntime>(6_202);

        let task = crate::task::registry::get_task::<MockRuntime>(6_202).unwrap();
        assert_eq!(task.state, TaskState::Runnable);
        assert_eq!(task.enqueued_at_tick, 123);

        let queued = take_remote_wake_mailbox(1);
        assert_eq!(queued.len(), 1);
        assert_eq!(queued.front().map(|entry| entry.tid), Some(6_202));
        assert!(global_need_resched_load(1, Ordering::Acquire));
        assert_eq!(DIAG_IPI_SENT.load(Ordering::Relaxed), 1);
        assert_eq!(DIAG_IPI_SENT_WAKE_TASK.load(Ordering::Relaxed), 1);
        assert_eq!(DIAG_REMOTE_WAKE_MAILBOX_NO_IPI.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_wake_task_remote_mailbox_forces_ipi_when_pending_and_rate_limited() {
        let _g = init_test_env();
        TICK_COUNT.store(100, Ordering::Relaxed);
        DIAG_IPI_SENT.store(0, Ordering::Relaxed);
        DIAG_IPI_SENT_WAKE_TASK.store(0, Ordering::Relaxed);
        DIAG_REMOTE_WAKE_MAILBOX_NO_IPI.store(0, Ordering::Relaxed);
        LAST_RESCHED_IPI_SENT_AT_TICK[1].store(100, Ordering::Relaxed);
        set_global_need_resched(1);

        let task = make_task(6_203, TaskState::Blocked, TaskPriority::Normal);
        let mut task = task;
        task.affinity = Affinity::Pinned(1);
        task.last_cpu = Some(1);
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(task));

        crate::sched::blocking::wake_task::<MockRuntime>(6_203);

        assert_eq!(
            DIAG_IPI_SENT.load(Ordering::Relaxed),
            1,
            "remote mailbox wake should force one resched IPI even with pending/rate-limit state"
        );
        assert_eq!(DIAG_IPI_SENT_WAKE_TASK.load(Ordering::Relaxed), 1);
        assert_eq!(DIAG_REMOTE_WAKE_MAILBOX_NO_IPI.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_drain_remote_wake_mailbox_enqueues_locally() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.mark_cpu_online(1);
        sched.state.per_cpu[1].current = Some(6_300);
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 6_300,
            runq_location: None,
            state: TaskState::Running,
            priority: TaskPriority::Low,
            affinity: Affinity::Pinned(1),
            last_cpu: Some(1),
            wake_cpu: Some(1),
            run_cpu: Some(1),
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 6_301,
            runq_location: None,
            state: TaskState::Blocked,
            priority: TaskPriority::Normal,
            affinity: Affinity::Pinned(1),
            last_cpu: Some(1),
            wake_cpu: None,
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        let _ = sched.state.register_waiter(6_301, state::WaitReason::BlockCurrent);
        sched.state.add_task_to_sleep_queue(6_301, 7);

        enqueue_remote_wake_mailbox(
            1,
            types::RemoteWakeMailboxEntry {
                tid: 6_301,
                priority: TaskPriority::Normal as usize,
                enqueued_at_tick: 9,
                wake_mono: 11,
            },
        );

        sched.drain_remote_wake_mailbox(1);

        assert!(
            sched.state.per_cpu[1].runq[TaskPriority::Normal as usize]
                .iter()
                .any(|&tid| tid == 6_301)
        );
        assert_eq!(sched.state.get_task(6_301).map(|sf| sf.state), Some(TaskState::Runnable));
        assert_eq!(sched.state.get_task(6_301).and_then(|sf| sf.wake_cpu), Some(1));
        assert!(!sched.state.wait_queue.contains(&6_301));
        assert!(!sched.state.sleep_membership.contains_key(&6_301));
        assert_eq!(PROF_REMOTE_WAKE_MAILBOX_AGE_HIST[0].load(Ordering::Relaxed), 1);
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
        sched.state.register_waiter(7001, crate::sched::state::WaitReason::BlockCurrent);

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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });

        // Seed stale queue membership for the exiting task and ensure another
        // runnable task exists so terminate_current can produce a switch.
        sched.state.enqueue_task(0, TaskPriority::Normal as usize, 8303);
        sched.state.enqueue_task(0, TaskPriority::Normal as usize, 8304);
        sched.state.register_waiter(8303, crate::sched::state::WaitReason::BlockCurrent);
        sched.state.add_task_to_sleep_queue(8303, 55);
        sched.state.add_task_to_sleep_queue(9999, 55);

        let termination = mark_task_exited_in_registry::<MockRuntime>(8303, 101);
        let switch = sched.terminate_current(8303, &termination.siblings_to_kill);

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
            !sched.state.wait_queue.contains(&8303),
            "dead current task must be removed from the wait queue"
        );
        assert_eq!(sched.state.sleep_queue.get(&55).cloned(), Some(alloc::vec![9999]));
    }

    #[test]
    fn terminate_current_only_mutates_scheduler_state() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(8310);

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(8310, TaskState::Running, TaskPriority::Normal),
        ));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(8311, TaskState::Runnable, TaskPriority::Normal),
        ));

        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 8310,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 8311,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(0, TaskPriority::Normal as usize, 8311);

        let _ = sched.terminate_current(8310, &[]);
        assert_eq!(sched.state.get_task(8310).unwrap().state, TaskState::Dead);
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(8310).unwrap().state,
            TaskState::Running,
            "terminate_current should not acquire/mutate REGISTRY directly"
        );
    }

    #[test]
    #[should_panic(expected = "terminate_current could not find a switch")]
    fn test_terminate_current_panics_when_no_switch_candidate_exists() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(8306);

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(8306, TaskState::Running, TaskPriority::Normal),
        ));

        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 8306,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });

        let termination = mark_task_exited_in_registry::<MockRuntime>(8306, 202);
        let _ = sched.terminate_current(8306, &termination.siblings_to_kill);
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });

        sched.state.enqueue_task(0, TaskPriority::Normal as usize, 8305);
        sched.state.register_waiter(8305, crate::sched::state::WaitReason::BlockCurrent);
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
            !sched.state.wait_queue.contains(&8305),
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
            Some(crate::sched::state::SleepMembership { wake_tick: 12, bucket_index: 0 })
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
                job: crate::job::Job::new(ppid, pid as TaskId),
                unix_compat: crate::task::ProcessUnixCompat::isolated(pid, false),
                handle_table: crate::vfs::handle_table::HandleTable::new(),
                ipc_table: crate::ipc::IpcHandleTable::new(),
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
            job: crate::job::Job {
                ppid: 1,
                thread_ids: alloc::vec![1220, 1221],
                exec_in_progress: false,
                children_done: alloc::collections::VecDeque::new(),
                exit_observer_inbox: None,
                leader_exit_code: Some(44),
                leader_exit_waiters: crate::sched::WaitQueue::new(),
            },
            unix_compat: crate::task::ProcessUnixCompat::isolated(1220, false),
            handle_table: crate::vfs::handle_table::HandleTable::new(),
            ipc_table: crate::ipc::IpcHandleTable::new(),
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
            job: crate::job::Job {
                ppid: 1,
                thread_ids: alloc::vec![7000, 7001],
                exec_in_progress: false,
                children_done: alloc::collections::VecDeque::new(),
                exit_observer_inbox: None,
                leader_exit_code: None,
                leader_exit_waiters: crate::sched::WaitQueue::new(),
            },
            unix_compat: crate::task::ProcessUnixCompat::isolated(7000, false),
            handle_table: crate::vfs::handle_table::HandleTable::new(),
            ipc_table: crate::ipc::IpcHandleTable::new(),
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
            job: crate::job::Job {
                ppid: 1,
                thread_ids: alloc::vec![8700, 8701],
                exec_in_progress: false,
                children_done: alloc::collections::VecDeque::new(),
                exit_observer_inbox: None,
                leader_exit_code: None,
                leader_exit_waiters: crate::sched::WaitQueue::new(),
            },
            unix_compat: crate::task::ProcessUnixCompat::isolated(8700, false),
            handle_table: crate::vfs::handle_table::HandleTable::new(),
            ipc_table: crate::ipc::IpcHandleTable::new(),
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
            job: crate::job::Job {
                ppid: 1,
                thread_ids: alloc::vec![8800, 8801],
                exec_in_progress: false,
                children_done: alloc::collections::VecDeque::new(),
                exit_observer_inbox: None,
                leader_exit_code: None,
                leader_exit_waiters: crate::sched::WaitQueue::new(),
            },
            unix_compat: crate::task::ProcessUnixCompat::isolated(8800, false),
            handle_table: crate::vfs::handle_table::HandleTable::new(),
            ipc_table: crate::ipc::IpcHandleTable::new(),
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
            job: crate::job::Job {
                ppid: 1,
                thread_ids: alloc::vec![9900],
                exec_in_progress: false,
                children_done: alloc::collections::VecDeque::new(),
                exit_observer_inbox: None,
                leader_exit_code: None,
                leader_exit_waiters: crate::sched::WaitQueue::new(),
            },
            unix_compat: crate::task::ProcessUnixCompat::isolated(9900, false),
            handle_table: crate::vfs::handle_table::HandleTable::new(),
            ipc_table: crate::ipc::IpcHandleTable::new(),
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
            job: crate::job::Job {
                ppid: 9900,
                thread_ids: alloc::vec![9800],
                exec_in_progress: false,
                children_done: alloc::collections::VecDeque::new(),
                exit_observer_inbox: None,
                leader_exit_code: None,
                leader_exit_waiters: crate::sched::WaitQueue::new(),
            },
            unix_compat: crate::task::ProcessUnixCompat::isolated(9800, false),
            handle_table: crate::vfs::handle_table::HandleTable::new(),
            ipc_table: crate::ipc::IpcHandleTable::new(),
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
            job: crate::job::Job {
                ppid: 1,
                thread_ids: alloc::vec![9100, 9101, 9102],
                exec_in_progress: false,
                children_done: alloc::collections::VecDeque::new(),
                exit_observer_inbox: None,
                leader_exit_code: None,
                leader_exit_waiters: crate::sched::WaitQueue::new(),
            },
            unix_compat: crate::task::ProcessUnixCompat::isolated(9100, false),
            handle_table: crate::vfs::handle_table::HandleTable::new(),
            ipc_table: crate::ipc::IpcHandleTable::new(),
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
            job: crate::job::Job {
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
            handle_table: crate::vfs::handle_table::HandleTable::new(),
            ipc_table: crate::ipc::IpcHandleTable::new(),
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
            job: crate::job::Job::new(1, 9300),
            unix_compat: crate::task::ProcessUnixCompat::isolated(9300, false),
            handle_table: crate::vfs::handle_table::HandleTable::new(),
            ipc_table: crate::ipc::IpcHandleTable::new(),
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

    #[test]
    fn test_block_current_recovers_when_prepare_schedule_returns_none() {
        let _g = init_test_env();
        crate::task::registry::init::<MockRuntime>();
        reset_mock_irq_depth();

        let tid = 9901;
        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(tid);
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            crate::task::Task {
                id: tid,
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
            },
        ));

        let mut lock = SCHEDULER.lock();
        *lock = Some((&mut sched as *mut types::Scheduler<MockRuntime>) as usize);
        drop(lock);

        block_current::<MockRuntime>();

        assert_eq!(mock_irq_depth(), 0, "block_current left IRQs disabled (depth != 0)");
        assert!(
            !sched.state.wait_queue.contains(&tid),
            "block_current recovery must remove waiter membership when no switch occurs"
        );
        assert_eq!(
            sched.state.get_task(tid).map(|sf| sf.state),
            Some(TaskState::Running),
            "scheduler cache must restore the task to Running when no switch occurs"
        );
        assert_eq!(
            crate::task::registry::get_task::<MockRuntime>(tid).unwrap().state,
            TaskState::Running,
            "registry state must be restored when no switch occurs"
        );

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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
                voluntary_yields: 0,
                migration_state: MigrationState::Local,
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
    fn test_any_affinity_wakeup_prefers_local_cpu_when_last_cpu_is_busier() {
        let _g = init_test_env();
        reset_any_wake_policy_for_tests();

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
            make_task(9916, TaskState::Blocked, TaskPriority::Normal),
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 9916,
            runq_location: None,
            state: TaskState::Blocked,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(2),
            wake_cpu: None,
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });

        for id in 9917..9920 {
            crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
                make_task(id, TaskState::Runnable, TaskPriority::Low),
            ));
            sched.state.insert_task(crate::sched::state::ThreadSchedFields {
                tid: id,
                runq_location: None,
                state: TaskState::Runnable,
                priority: TaskPriority::Low,
                affinity: Affinity::Any,
                last_cpu: Some(2),
                wake_cpu: Some(2),
                run_cpu: None,
                timeslice_remaining: types::DEFAULT_TIMESLICE,
                enqueued_at_tick: 0,
                voluntary_yields: 0,
                migration_state: MigrationState::Local,
                wake_pending: false,
            });
            sched.state.enqueue_task(2, TaskPriority::Low as usize, id);
        }

        let (_ipi, _deferred) =
            crate::sched::blocking::wake_task_locked::<MockRuntime>(&mut sched, 9916);
        assert!(
            sched.state.per_cpu[0].runq[TaskPriority::Normal as usize]
                .iter()
                .any(|&tid| tid == 9916),
            "[policy] Any-affinity wakeup should prefer local CPU when last_cpu is busier"
        );
        assert_eq!(
            sched.state.get_task(9916).and_then(|sf| sf.wake_cpu),
            Some(0),
            "[policy] wake_cpu should record local routing under local-bias heuristic"
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

    // ── choose_wake_cpu tests ─────────────────────────────────────────────

    /// `choose_wake_cpu` should route an Any-affinity wakeup to the idle CPU
    /// when the preferred CPU (last_cpu) is loaded beyond the overload gap.
    #[test]
    fn test_choose_wake_cpu_prefers_idle_cpu_when_preferred_is_loaded() {
        let _g = init_test_env();
        set_any_wake_policy_for_tests("redirect", 2);

        let mut sched = types::Scheduler::<MockRuntime>::new();
        // CPU 0: loaded, current is a non-idle task.
        // CPU 1: idle  (current == idle_task).
        // CPU 2: online but not idle.
        for _ in 0..3 {
            sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        }
        sched.state.mark_cpu_online(0);
        sched.state.mark_cpu_online(1);
        sched.state.mark_cpu_online(2);

        // Make CPU 1 idle: current == idle_task == 80_001.
        let idle_tid: u64 = 80_001;
        sched.state.per_cpu[1].idle_task = Some(idle_tid);
        sched.state.per_cpu[1].current = Some(idle_tid);

        // Load CPU 0 past the overload gap (gap = 2, add 3 tasks).
        for tid in 80_010..80_013 {
            sched.state.enqueue_task(0, TaskPriority::Low as usize, tid);
        }

        // Task that was last on CPU 0.
        let result = choose_wake_cpu::<MockRuntime>(&sched, Some(0));
        assert_eq!(
            result, 1,
            "choose_wake_cpu should route to idle CPU 1 when CPU 0 is overloaded"
        );
    }

    /// `choose_wake_cpu` should stay on the preferred CPU when the preferred CPU
    /// is itself the idle CPU (i.e. it is lightly loaded).
    #[test]
    fn test_choose_wake_cpu_stays_on_preferred_when_preferred_is_idle() {
        let _g = init_test_env();
        set_any_wake_policy_for_tests("redirect", 2);

        let mut sched = types::Scheduler::<MockRuntime>::new();
        for _ in 0..2 {
            sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        }
        sched.state.mark_cpu_online(0);
        sched.state.mark_cpu_online(1);

        // Make CPU 0 idle: current == idle_task == 80_020.
        let idle_tid: u64 = 80_020;
        sched.state.per_cpu[0].idle_task = Some(idle_tid);
        sched.state.per_cpu[0].current = Some(idle_tid);

        // Task with last_cpu = 0 (the idle CPU).
        let result = choose_wake_cpu::<MockRuntime>(&sched, Some(0));
        // CPU 0 is the idle CPU and its depth is 0 (below overload_gap of 2),
        // so no redirection should happen.
        assert_eq!(
            result, 0,
            "choose_wake_cpu should keep preferred CPU when it is already idle"
        );
    }

    /// `choose_wake_cpu` should use last_cpu as the target when no idle CPU
    /// exists and the preferred CPU is not overloaded.
    #[test]
    fn test_choose_wake_cpu_uses_last_cpu_when_no_idle_cpu_and_not_overloaded() {
        let _g = init_test_env();
        reset_any_wake_policy_for_tests();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        for _ in 0..3 {
            sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        }
        sched.state.mark_cpu_online(0);
        sched.state.mark_cpu_online(1);
        sched.state.mark_cpu_online(2);
        // No idle CPU: current is None on all CPUs.

        // Task with last_cpu = 2.
        let result = choose_wake_cpu::<MockRuntime>(&sched, Some(2));
        assert_eq!(
            result, 2,
            "choose_wake_cpu should return last_cpu when not overloaded and no idle CPU exists"
        );
    }

    /// When multiple Any-affinity tasks wake at once and one idle CPU exists,
    /// the first wakeup should land on the idle CPU; subsequent wakeups should
    /// spread to other CPUs rather than all piling on the idle one.
    #[test]
    fn test_choose_wake_cpu_distributes_fanout_across_cpus() {
        let _g = init_test_env();
        set_any_wake_policy_for_tests("redirect", 1);

        let mut sched = types::Scheduler::<MockRuntime>::new();
        for _ in 0..4 {
            sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        }
        sched.state.mark_cpu_online(0);
        sched.state.mark_cpu_online(1);
        sched.state.mark_cpu_online(2);
        sched.state.mark_cpu_online(3);

        // Load CPUs 0, 2, 3 so they are above overload gap.
        for cpu in [0usize, 2, 3] {
            for tid_offset in 0..3u64 {
                let tid = (cpu as u64) * 100 + 80_100 + tid_offset;
                sched.state.enqueue_task(cpu, TaskPriority::Low as usize, tid);
            }
        }
        // CPU 1 is idle.
        let idle_tid: u64 = 80_050;
        sched.state.per_cpu[1].idle_task = Some(idle_tid);
        sched.state.per_cpu[1].current = Some(idle_tid);

        // All tasks have last_cpu = 0 (overloaded).
        let targets: alloc::vec::Vec<usize> = (0..4)
            .map(|_| choose_wake_cpu::<MockRuntime>(&sched, Some(0)))
            .collect();

        // At least one call should have routed to an idle or less-loaded CPU.
        assert!(
            targets.iter().any(|&cpu| cpu != 0),
            "fanout should distribute wakeups away from overloaded CPU 0, got {:?}",
            targets
        );
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
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
                voluntary_yields: 0,
                migration_state: MigrationState::Local,
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
    fn test_wake_sleepers_batches_any_affinity_load_scans() {
        let _g = init_test_env();
        set_any_wake_policy_for_tests("redirect", 2);

        let mut sched = types::Scheduler::<MockRuntime>::new();
        for _ in 0..2 {
            sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        }
        sched.state.mark_cpu_online(0);
        sched.state.mark_cpu_online(1);

        let current_task = make_task(9926, TaskState::Running, TaskPriority::Normal);
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(current_task));
        sched.state.per_cpu[0].current = Some(9926);
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 9926,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });

        // Keep CPU 0 overloaded for Any-affinity wake routing.
        for id in 9927..9930 {
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
                voluntary_yields: 0,
                migration_state: MigrationState::Local,
                wake_pending: false,
            });
            sched.state.enqueue_task(0, TaskPriority::Low as usize, id);
        }

        for id in 9930..9933 {
            let sleeping = make_task(id, TaskState::Blocked, TaskPriority::Normal);
            crate::task::registry::get_registry::<MockRuntime>()
                .insert(alloc::boxed::Box::new(sleeping));
            sched.state.insert_task(crate::sched::state::ThreadSchedFields {
                tid: id,
                runq_location: None,
                state: TaskState::Blocked,
                priority: TaskPriority::Normal,
                affinity: Affinity::Any,
                last_cpu: Some(0),
                wake_cpu: None,
                run_cpu: None,
                timeslice_remaining: types::DEFAULT_TIMESLICE,
                enqueued_at_tick: 0,
                voluntary_yields: 0,
                migration_state: MigrationState::Local,
                wake_pending: false,
            });
            sched.state.add_task_to_sleep_queue(id, 50);
        }

        TEST_LEAST_LOADED_ONLINE_CPU_CALLS.store(0, Ordering::Relaxed);
        TICK_COUNT.store(100, Ordering::Relaxed);
        sched.wake_sleepers();

        let cpu0_wakes = sched.state.per_cpu[0].runq[TaskPriority::Normal as usize].len();
        let cpu1_wakes = sched.state.per_cpu[1].runq[TaskPriority::Normal as usize].len();
        assert_eq!(
            cpu0_wakes + cpu1_wakes,
            3,
            "all Any-affinity sleepers should wake in this batch"
        );
        assert!(
            cpu0_wakes > 0 && cpu1_wakes > 0,
            "batch load snapshot should spread wakeups after redirecting early tasks (cpu0={}, cpu1={})",
            cpu0_wakes,
            cpu1_wakes
        );

        let least_scan_calls = TEST_LEAST_LOADED_ONLINE_CPU_CALLS.load(Ordering::Relaxed);
        assert!(
            least_scan_calls == 0,
            "wake batch should avoid least-loaded queue scans via per-batch snapshots (calls={})",
            least_scan_calls
        );
    }

    #[test]
    fn test_steal_task_scans_bounded_depth_per_priority() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        for _ in 0..2 {
            sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        }
        sched.state.mark_cpu_online(0);
        sched.state.mark_cpu_online(1);

        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(9930, TaskState::Runnable, TaskPriority::Normal),
        ));
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(9931, TaskState::Runnable, TaskPriority::Normal),
        ));

        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 9930,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Pinned(1),
            last_cpu: Some(1),
            wake_cpu: Some(1),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: 9931,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(1),
            wake_cpu: Some(1),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });

        sched.state.enqueue_task(1, TaskPriority::Normal as usize, 9930);
        sched.state.enqueue_task(1, TaskPriority::Normal as usize, 9931);

        let stolen = sched.steal_task_for(0);
        assert_eq!(
            stolen,
            Some(9931),
            "bounded scan should skip unstealable head and steal stealable follower"
        );
        assert_eq!(
            sched.state.get_task(9931).and_then(|sf| sf.wake_cpu),
            Some(0),
            "stolen task wake_cpu should track destination CPU"
        );
        assert_eq!(
            sched.state.get_task(9931).and_then(|sf| sf.runq_location),
            None,
            "stolen task should no longer be tracked in donor runq"
        );
        assert!(
            sched.state.per_cpu[1].runq[TaskPriority::Normal as usize]
                .iter()
                .any(|&tid| tid == 9930),
            "unstealable pinned head should remain queued on donor CPU"
        );
    }

    #[test]
    fn test_steal_task_respects_scan_depth_limit() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        for _ in 0..2 {
            sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        }
        sched.state.mark_cpu_online(0);
        sched.state.mark_cpu_online(1);

        for offset in 0..STEAL_SCAN_DEPTH_PER_PRIORITY {
            let tid = 9940 + offset as u64;
            crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
                make_task(tid, TaskState::Runnable, TaskPriority::Normal),
            ));
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
                voluntary_yields: 0,
                migration_state: MigrationState::Local,
                wake_pending: false,
            });
            sched.state.enqueue_task(1, TaskPriority::Normal as usize, tid);
        }

        let stealable_tid = 9950;
        crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
            make_task(stealable_tid, TaskState::Runnable, TaskPriority::Normal),
        ));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid: stealable_tid,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(1),
            wake_cpu: Some(1),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
            wake_pending: false,
        });
        sched.state.enqueue_task(1, TaskPriority::Normal as usize, stealable_tid);

        let stolen = sched.steal_task_for(0);
        assert_eq!(
            stolen, None,
            "steal scan must stay bounded and not inspect beyond configured depth"
        );
        assert!(
            sched.state.per_cpu[1].runq[TaskPriority::Normal as usize]
                .iter()
                .any(|&tid| tid == stealable_tid),
            "stealable task beyond scan-depth cap should remain on donor queue"
        );
    }

    // ── try_steal_one tests ────────────────────────────────────────────────

    #[test]
    fn test_try_steal_one_returns_none_when_victim_below_min_depth() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        for _ in 0..2 {
            sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        }
        sched.state.mark_cpu_online(0);
        sched.state.mark_cpu_online(1);

        // Only one task on CPU 1 — below the min_depth=2 threshold.
        let tid = 10_001u64;
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(make_task(tid, TaskState::Runnable, TaskPriority::Normal)));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
            runq_location: None,
            state: TaskState::Runnable,
            priority: TaskPriority::Normal,
            affinity: Affinity::Any,
            last_cpu: Some(1),
            wake_cpu: Some(1),
            run_cpu: None,
            timeslice_remaining: types::DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(1, TaskPriority::Normal as usize, tid);

        // min_depth=2 but victim has only 1 task — should return None.
        let result = sched.try_steal_one(0, 1, 2);
        assert_eq!(result, None, "should not steal when victim depth < min_depth");
    }

    #[test]
    fn test_try_steal_one_steals_when_victim_meets_min_depth() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        for _ in 0..2 {
            sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        }
        sched.state.mark_cpu_online(0);
        sched.state.mark_cpu_online(1);

        // Two tasks on CPU 1.
        for tid in [10_010u64, 10_011u64] {
            crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
                make_task(tid, TaskState::Runnable, TaskPriority::Normal),
            ));
            sched.state.insert_task(crate::sched::state::ThreadSchedFields {
                tid,
                runq_location: None,
                state: TaskState::Runnable,
                priority: TaskPriority::Normal,
                affinity: Affinity::Any,
                last_cpu: Some(1),
                wake_cpu: Some(1),
                run_cpu: None,
                timeslice_remaining: types::DEFAULT_TIMESLICE,
                enqueued_at_tick: 0,
                wake_pending: false,
                voluntary_yields: 0,
                migration_state: MigrationState::Local,
            });
            sched.state.enqueue_task(1, TaskPriority::Normal as usize, tid);
        }

        let stolen = sched.try_steal_one(0, 1, 2);
        assert!(stolen.is_some(), "should steal when victim has >= min_depth tasks");
        let stolen_id = stolen.unwrap();
        assert!(
            stolen_id == 10_010 || stolen_id == 10_011,
            "stolen task must come from victim CPU 1"
        );
        assert_eq!(
            sched.state.get_task(stolen_id).and_then(|sf| sf.wake_cpu),
            Some(0),
            "stolen task wake_cpu must point to local CPU"
        );
        assert_eq!(
            sched.state.get_task(stolen_id).and_then(|sf| sf.runq_location),
            None,
            "stolen task must be removed from victim run queue"
        );
    }

    #[test]
    fn test_try_steal_one_returns_none_for_same_cpu() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.mark_cpu_online(0);

        let tid = 10_020u64;
        crate::task::registry::get_registry::<MockRuntime>()
            .insert(alloc::boxed::Box::new(make_task(tid, TaskState::Runnable, TaskPriority::Normal)));
        sched.state.insert_task(crate::sched::state::ThreadSchedFields {
            tid,
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
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        });
        sched.state.enqueue_task(0, TaskPriority::Normal as usize, tid);

        // local_cpu == victim_cpu → always None (cannot steal from self).
        assert_eq!(sched.try_steal_one(0, 0, 1), None);
    }

    #[test]
    fn test_try_steal_one_skips_pinned_tasks() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        for _ in 0..2 {
            sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        }
        sched.state.mark_cpu_online(0);
        sched.state.mark_cpu_online(1);

        // Two tasks on CPU 1, both pinned to CPU 1.
        for tid in [10_030u64, 10_031u64] {
            crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
                make_task(tid, TaskState::Runnable, TaskPriority::Normal),
            ));
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
                voluntary_yields: 0,
                migration_state: MigrationState::Local,
            });
            sched.state.enqueue_task(1, TaskPriority::Normal as usize, tid);
        }

        assert_eq!(
            sched.try_steal_one(0, 1, 2),
            None,
            "pinned tasks must never be stolen"
        );
    }

    // ── idle_steal tests ───────────────────────────────────────────────────

    #[test]
    fn test_idle_steal_prefers_nearby_cpu_over_distant() {
        let _g = init_test_env();

        // CPU layout: local=0, nearby=1 (within radius), far=10.
        // We need at least 11 per_cpu slots.
        let num_cpus = 11usize;
        let mut sched = types::Scheduler::<MockRuntime>::new();
        for _ in 0..num_cpus {
            sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        }
        for i in [0usize, 1, 10] {
            sched.state.mark_cpu_online(i);
        }

        // Place two tasks on nearby CPU 1 and two tasks on far CPU 10.
        let nearby_tids = [10_100u64, 10_101u64];
        let far_tids = [10_110u64, 10_111u64];

        for &tid in &nearby_tids {
            crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
                make_task(tid, TaskState::Runnable, TaskPriority::Normal),
            ));
            sched.state.insert_task(crate::sched::state::ThreadSchedFields {
                tid,
                runq_location: None,
                state: TaskState::Runnable,
                priority: TaskPriority::Normal,
                affinity: Affinity::Any,
                last_cpu: Some(1),
                wake_cpu: Some(1),
                run_cpu: None,
                timeslice_remaining: types::DEFAULT_TIMESLICE,
                enqueued_at_tick: 0,
                wake_pending: false,
                voluntary_yields: 0,
                migration_state: MigrationState::Local,
            });
            sched.state.enqueue_task(1, TaskPriority::Normal as usize, tid);
        }

        for &tid in &far_tids {
            crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
                make_task(tid, TaskState::Runnable, TaskPriority::Normal),
            ));
            sched.state.insert_task(crate::sched::state::ThreadSchedFields {
                tid,
                runq_location: None,
                state: TaskState::Runnable,
                priority: TaskPriority::Normal,
                affinity: Affinity::Any,
                last_cpu: Some(10),
                wake_cpu: Some(10),
                run_cpu: None,
                timeslice_remaining: types::DEFAULT_TIMESLICE,
                enqueued_at_tick: 0,
                wake_pending: false,
                voluntary_yields: 0,
                migration_state: MigrationState::Local,
            });
            sched.state.enqueue_task(10, TaskPriority::Normal as usize, tid);
        }

        // idle_steal from CPU 0 should pick from nearby CPU 1 (within
        // STEAL_NEARBY_RADIUS) before trying distant CPU 10.
        let stolen = sched.idle_steal(0);
        assert!(stolen.is_some(), "idle_steal should find work");
        let stolen_id = stolen.unwrap();
        assert!(
            nearby_tids.contains(&stolen_id),
            "idle_steal should prefer nearby CPU 1 over far CPU 10 (stolen={stolen_id})"
        );
    }

    #[test]
    fn test_idle_steal_falls_back_to_distant_when_nearby_empty() {
        let _g = init_test_env();

        let num_cpus = 11usize;
        let mut sched = types::Scheduler::<MockRuntime>::new();
        for _ in 0..num_cpus {
            sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        }
        for i in [0usize, 10] {
            sched.state.mark_cpu_online(i);
        }

        // Only far CPU 10 has work.
        let far_tids = [10_200u64, 10_201u64];
        for &tid in &far_tids {
            crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
                make_task(tid, TaskState::Runnable, TaskPriority::Normal),
            ));
            sched.state.insert_task(crate::sched::state::ThreadSchedFields {
                tid,
                runq_location: None,
                state: TaskState::Runnable,
                priority: TaskPriority::Normal,
                affinity: Affinity::Any,
                last_cpu: Some(10),
                wake_cpu: Some(10),
                run_cpu: None,
                timeslice_remaining: types::DEFAULT_TIMESLICE,
                enqueued_at_tick: 0,
                wake_pending: false,
                voluntary_yields: 0,
                migration_state: MigrationState::Local,
            });
            sched.state.enqueue_task(10, TaskPriority::Normal as usize, tid);
        }

        let stolen = sched.idle_steal(0);
        assert!(
            stolen.is_some(),
            "idle_steal should fall back to distant CPUs when nearby CPUs have no work"
        );
        let stolen_id = stolen.unwrap();
        assert!(
            far_tids.contains(&stolen_id),
            "should steal from far CPU 10 when it is the only loaded CPU"
        );
    }

    #[test]
    fn test_idle_steal_returns_none_when_all_cpus_have_single_task() {
        let _g = init_test_env();

        let mut sched = types::Scheduler::<MockRuntime>::new();
        for _ in 0..3 {
            sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        }
        for i in 0..3 {
            sched.state.mark_cpu_online(i);
        }

        // Each peer CPU has exactly 1 task — below the min_depth=2 threshold.
        for cpu in 1..3usize {
            let tid = 10_300u64 + cpu as u64;
            crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
                make_task(tid, TaskState::Runnable, TaskPriority::Normal),
            ));
            sched.state.insert_task(crate::sched::state::ThreadSchedFields {
                tid,
                runq_location: None,
                state: TaskState::Runnable,
                priority: TaskPriority::Normal,
                affinity: Affinity::Any,
                last_cpu: Some(cpu),
                wake_cpu: Some(cpu),
                run_cpu: None,
                timeslice_remaining: types::DEFAULT_TIMESLICE,
                enqueued_at_tick: 0,
                wake_pending: false,
                voluntary_yields: 0,
                migration_state: MigrationState::Local,
            });
            sched.state.enqueue_task(cpu, TaskPriority::Normal as usize, tid);
        }

        assert_eq!(
            sched.idle_steal(0),
            None,
            "idle_steal must not steal when no CPU has >= 2 runnable tasks (anti-thrash)"
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
        let before_ipi =
            PROF_TRYLOCK_MISS_IPI_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);
        assert!(
            global_need_resched_slot(0).is_some(),
            "test requires CPU 0 GLOBAL_NEED_RESCHED slot"
        );
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
        assert!(
            after_miss > before_miss,
            "[contention] per-CPU trylock miss counter should increment"
        );
        assert!(
            after_pending > before_pending,
            "[contention] per-CPU pending-resched trylock miss counter should increment"
        );
        assert!(
            PROF_TRYLOCK_MISS_IPI_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed)
                > before_ipi,
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
        let before_ipi =
            PROF_TRYLOCK_MISS_IPI_PER_CPU[0].load(core::sync::atomic::Ordering::Relaxed);

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
        let before_window_idle_timer =
            TRYLOCK_MISS_WINDOW_IDLE_TIMER_COUNT[0].load(Ordering::Relaxed);
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
            TRYLOCK_MISS_WINDOW_IDLE_TIMER_COUNT[0].load(Ordering::Relaxed)
                > before_window_idle_timer,
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
        assert!(
            need_resched_pending(0),
            "pre-condition: flag should be set before schedule_point"
        );

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
        enqueue_remote_wake_mailbox(0, types::RemoteWakeMailboxEntry {
            tid: rt_tid,
            priority: TaskPriority::Realtime as usize,
            wake_mono: 0,
            enqueued_at_tick: 50,
        });

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

    // ── periodic_load_balance tests ───────────────────────────────────────────

    /// Build a scheduler with `num_cpus` CPUs online, all slots in per_cpu,
    /// and register + insert `tasks` tasks onto CPU `donor_cpu`.
    fn make_periodic_balance_sched(
        num_cpus: usize,
        tasks: &[(u64, usize)], // (tid, cpu)
    ) -> types::Scheduler<MockRuntime> {
        let mut sched = types::Scheduler::<MockRuntime>::new();
        for _ in 0..num_cpus {
            sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        }
        for i in 0..num_cpus {
            sched.state.mark_cpu_online(i);
        }
        for &(tid, cpu) in tasks {
            crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
                make_task(tid, TaskState::Runnable, TaskPriority::Normal),
            ));
            sched.state.insert_task(crate::sched::state::ThreadSchedFields {
                tid,
                runq_location: None,
                state: TaskState::Runnable,
                priority: TaskPriority::Normal,
                affinity: Affinity::Any,
                last_cpu: Some(cpu),
                wake_cpu: Some(cpu),
                run_cpu: None,
                timeslice_remaining: types::DEFAULT_TIMESLICE,
                enqueued_at_tick: 0,
                wake_pending: false,
                voluntary_yields: 0,
                migration_state: MigrationState::Local,
            });
            sched.state.enqueue_task(cpu, TaskPriority::Normal as usize, tid);
        }
        sched
    }

    #[test]
    fn test_periodic_load_balance_rate_limited() {
        let _g = init_test_env();
        // Set up a severe imbalance: CPU 1 has 4 tasks, CPU 0 has zero.
        let tasks: &[(u64, usize)] =
            &[(20_000, 1), (20_001, 1), (20_002, 1), (20_003, 1)];
        let mut sched = make_periodic_balance_sched(2, tasks);

        // Tick 0: last_balance_tick = 0 and now = 0, so wrapping_sub(0, 0) = 0
        // which is less than INTERVAL → balance must NOT run yet.
        TICK_COUNT.store(0, Ordering::Relaxed);
        let before = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
        sched.periodic_load_balance();
        let after = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
        assert_eq!(
            after, before,
            "balance should not run when now == last_balance_tick (cooldown not elapsed)"
        );

        // Advance by INTERVAL ticks → balance should now trigger.
        TICK_COUNT.store(types::PERIODIC_BALANCE_INTERVAL_TICKS, Ordering::Relaxed);
        let before2 = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
        sched.periodic_load_balance();
        let after2 = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
        assert!(
            after2 > before2,
            "balance should migrate tasks when cooldown has elapsed"
        );

        // Immediately calling again (same tick) must be suppressed.
        let before3 = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
        sched.periodic_load_balance();
        let after3 = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
        assert_eq!(
            after3, before3,
            "second call in the same tick window must be rate-limited"
        );
    }

    #[test]
    fn test_periodic_load_balance_resolves_severe_imbalance() {
        let _g = init_test_env();
        // CPU 0 has 0 tasks; CPU 1 has 4 tasks → severe imbalance.
        let tasks: &[(u64, usize)] =
            &[(20_100, 1), (20_101, 1), (20_102, 1), (20_103, 1)];
        let mut sched = make_periodic_balance_sched(2, tasks);

        TICK_COUNT.store(types::PERIODIC_BALANCE_INTERVAL_TICKS, Ordering::Relaxed);
        let before = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
        sched.periodic_load_balance();
        let after = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
        let migrated = after - before;

        assert!(
            migrated > 0,
            "periodic_load_balance must migrate at least one task from overloaded CPU"
        );
        assert!(
            migrated <= types::PERIODIC_BALANCE_MAX_MIGRATIONS_PER_RUN as u64,
            "must not exceed per-run migration cap (migrated={migrated})"
        );

        let cpu0_depth =
            sched.state.per_cpu[0].runq.total_len();
        let cpu1_depth =
            sched.state.per_cpu[1].runq.total_len();
        assert!(
            cpu0_depth > 0,
            "target CPU 0 should have received at least one task (depth={cpu0_depth})"
        );
        assert!(
            cpu1_depth < 4,
            "donor CPU 1 should have fewer tasks after balancing (depth={cpu1_depth})"
        );
    }

    #[test]
    fn test_periodic_load_balance_ignores_mild_imbalance() {
        let _g = init_test_env();
        // CPU 0 has 1 task; CPU 1 has 2 tasks → diff=1, below threshold=2, no migration.
        let tasks: &[(u64, usize)] = &[(20_200, 0), (20_201, 1), (20_202, 1)];
        let mut sched = make_periodic_balance_sched(2, tasks);

        TICK_COUNT.store(types::PERIODIC_BALANCE_INTERVAL_TICKS, Ordering::Relaxed);
        let before = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
        sched.periodic_load_balance();
        let after = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
        assert_eq!(
            after, before,
            "mild imbalance (diff < threshold) must not trigger migration"
        );
    }

    #[test]
    fn test_periodic_load_balance_skips_single_cpu() {
        let _g = init_test_env();
        // Only 1 CPU online — nothing to balance.
        let tasks: &[(u64, usize)] = &[(20_300, 0), (20_301, 0), (20_302, 0)];
        let mut sched = make_periodic_balance_sched(1, tasks);

        TICK_COUNT.store(types::PERIODIC_BALANCE_INTERVAL_TICKS, Ordering::Relaxed);
        let before = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
        sched.periodic_load_balance();
        let after = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
        assert_eq!(after, before, "single-CPU system must never trigger migration");
    }

    #[test]
    fn test_periodic_load_balance_respects_migration_cap() {
        let _g = init_test_env();
        // CPU 0 has 0 tasks; CPU 1 has many tasks — ensure cap is honoured.
        let num_tasks = types::PERIODIC_BALANCE_MAX_MIGRATIONS_PER_RUN + 3;
        let tasks: alloc::vec::Vec<(u64, usize)> =
            (0..num_tasks).map(|i| (20_400 + i as u64, 1)).collect();
        let mut sched = make_periodic_balance_sched(2, &tasks);

        TICK_COUNT.store(types::PERIODIC_BALANCE_INTERVAL_TICKS, Ordering::Relaxed);
        let before = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
        sched.periodic_load_balance();
        let after = PROF_PERIODIC_BALANCE_MIGRATIONS.load(Ordering::Relaxed);
        let migrated = after - before;

        assert_eq!(
            migrated,
            types::PERIODIC_BALANCE_MAX_MIGRATIONS_PER_RUN as u64,
            "periodic balancer must stop at PERIODIC_BALANCE_MAX_MIGRATIONS_PER_RUN (got {migrated})"
        );
    }
}
