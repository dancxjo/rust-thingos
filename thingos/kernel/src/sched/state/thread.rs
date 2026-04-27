use super::{Affinity, MigrationState, ThreadId};
use crate::task::{TaskPriority, TaskState};

/// Scheduler-side metadata for a single kernel thread.
///
/// This struct carries scheduler-internal queue tracking state plus a
/// **hot-field cache** of frequently read values from `Thread<R>` in the
/// global registry.  The cache avoids taking the REGISTRY lock inside
/// the SCHEDULER hot path (nested locking was the primary source of lock
/// convoy behaviour under SMP).
///
/// # Canonical fields (no equivalent in `Thread<R>`)
/// - `tid` — stable key for `SchedState::threads` lookups.
/// - `runq_location` — tracks which `(cpu, priority)` run-queue slot currently
///   holds this thread; there is no corresponding field in `Thread<R>`.
///
/// # Cached hot fields (mirrors of `Thread<R>` fields)
/// These must be kept in sync with the registry whenever the corresponding
/// field changes.  They are updated by the scheduler paths that mutate the
/// underlying registry entry; no other code should modify them directly.
/// - `state` — lifecycle state; updated by `wake_task_locked`, `wake_sleepers`,
///   `block_current`, `sleep_ticks`, `prepare_schedule`, and `mark_task_exited`.
/// - `priority` — current scheduling priority; updated by `set_priority`.
/// - `affinity` — CPU affinity; updated at spawn time and by `cpu_online`.
/// - `last_cpu` — last CPU this thread ran on; updated by `prepare_schedule`.
/// - `timeslice_remaining` — ticks remaining before preemption; decremented
///   each timer tick in `schedule_point` without re-entering REGISTRY.
/// - `enqueued_at_tick` — tick when this thread was last enqueued; used by
///   periodic aging maintenance in `schedule_point` to materialize promotions
///   without re-entering REGISTRY.
pub struct ThreadSchedFields {
    pub tid: ThreadId,
    pub runq_location: Option<(usize, usize)>,
    /// Cached copy of `Thread<R>::state`.
    pub state: TaskState,
    /// Cached copy of `Thread<R>::priority`.
    pub priority: TaskPriority,
    /// Cached copy of `Thread<R>::affinity`.
    pub affinity: Affinity,
    /// Cached copy of `Thread<R>::last_cpu`.
    pub last_cpu: Option<usize>,
    /// Last CPU that enqueued this task via wakeup/unblock path.
    pub wake_cpu: Option<usize>,
    /// Last CPU that actually ran this task.
    pub run_cpu: Option<usize>,
    /// Cached copy of `Thread<R>::timeslice_remaining`.
    pub timeslice_remaining: u32,
    /// Cached copy of `Thread<R>::enqueued_at_tick`.
    /// Used by periodic scheduler aging maintenance to materialize promotion
    /// into runnable priority buckets.
    pub enqueued_at_tick: u64,
    /// Cached copy of `Thread<R>::wake_pending`.
    ///
    /// Set by `wake_task_locked` when the target task is not already blocked
    /// (preemptive wake signal).  Checked and cleared by `block_current` so
    /// that the hot `wake_pending` check no longer re-enters the REGISTRY lock
    /// while the SCHEDULER lock is held.
    pub wake_pending: bool,
    /// Count of voluntary yields this task has performed via `prepare_yield`.
    ///
    /// Incremented by the scheduler each time `CooperativeYield` causes this
    /// task to be pushed back onto the run queue.  Useful for diagnosing
    /// spin-yield anti-patterns (e.g. a task that never truly blocks will show
    /// a rapidly-growing counter here).
    pub voluntary_yields: u64,
    /// Explicit migration lifecycle state.
    ///
    /// Tracks whether this task has a pending migration request, is currently
    /// in transit between CPUs, or is pinned.  Updated by the steal, balance,
    /// and placement paths.  Use [`MigrationState::is_migratable`] and
    /// [`MigrationState::try_transition`] to enforce safe state transitions
    /// before initiating any cross-CPU movement.
    pub migration_state: MigrationState,
}
/// Backward-compatible alias — prefer `ThreadSchedFields` in new code.
pub type TaskSchedFields = ThreadSchedFields;
