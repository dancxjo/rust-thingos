use alloc::collections::{BTreeMap, BTreeSet, VecDeque};
use alloc::vec::Vec;

use core::sync::atomic::{AtomicBool, Ordering};
use spin::{Mutex, Once};

/// Unique identifier for a kernel thread (scheduler task).
pub type ThreadId = u64;
/// Backward-compatible alias — prefer `ThreadId` in new code.
pub type TaskId = ThreadId;

/// Scheduling priority levels; higher variants preempt lower ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreadPriority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Realtime = 4,
}
/// Backward-compatible alias — prefer `ThreadPriority` in new code.
pub type TaskPriority = ThreadPriority;

/// Scheduler class / latency domain for a runnable thread.
///
/// This separates high-level scheduling intent from the legacy single-priority
/// lattice so class-specific policies can evolve independently.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadSchedClass {
    NormalTimeslice,
    Realtime,
    /// Reserved class for explicit kernel interrupt-thread / bottom-half work.
    ///
    /// This class is intentionally not selected by `default_sched_class()`;
    /// callers must opt in explicitly when class-aware admission is added.
    InterruptBottomHalf,
    BackgroundMaintenance,
}
/// Backward-compatible alias — prefer `ThreadSchedClass` in new code.
pub type TaskSchedClass = ThreadSchedClass;

impl ThreadPriority {
    /// Current default class mapping for the existing priority lattice.
    ///
    /// This preserves current behavior while providing an explicit class seam
    /// for future class-specific policy and queueing rules.
    pub const fn default_sched_class(self) -> ThreadSchedClass {
        match self {
            ThreadPriority::Idle | ThreadPriority::Low => ThreadSchedClass::BackgroundMaintenance,
            ThreadPriority::Normal | ThreadPriority::High => ThreadSchedClass::NormalTimeslice,
            ThreadPriority::Realtime => ThreadSchedClass::Realtime,
        }
    }
}

/// Bitmask of CPU indices representing the set of CPUs a task is allowed to use.
///
/// Internally stores up to 64 CPU indices as a bitmask (`u64`).  Bit `n` set
/// means CPU index `n` is in the set.  CPU indices ≥ 64 are never in any set.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CpuSet(pub u64);

impl CpuSet {
    /// A set that contains **all** CPU indices (bits 0..63 set).
    pub const fn all() -> Self {
        CpuSet(u64::MAX)
    }

    /// A set containing **only** `cpu`.  Returns an empty set if `cpu >= 64`.
    pub const fn only(cpu: CpuId) -> Self {
        if cpu < 64 {
            CpuSet(1u64 << cpu)
        } else {
            CpuSet(0)
        }
    }

    /// Returns `true` if `cpu` is in this set.
    #[inline]
    pub fn contains(self, cpu: CpuId) -> bool {
        cpu < 64 && (self.0 >> cpu) & 1 != 0
    }

    /// Returns the lowest-indexed CPU in the set, or `None` if empty.
    #[inline]
    pub fn first(self) -> Option<CpuId> {
        if self.0 == 0 { None } else { Some(self.0.trailing_zeros() as CpuId) }
    }

    /// Returns `true` if the set contains at least one CPU.
    #[inline]
    pub fn any(self) -> bool {
        self.0 != 0
    }

    /// Number of CPUs in the set.
    #[inline]
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }

    /// Pick the best CPU from the set, bounded by `cpu_count`.
    ///
    /// Priority order:
    /// 1. `preferred` — if it is in the set and `< cpu_count`.
    /// 2. `last_cpu` — if it is in the set and `< cpu_count`.
    /// 3. Lowest-indexed CPU in the set that is `< cpu_count`.
    ///
    /// Returns `None` if no allowed CPU is within `cpu_count`.
    pub fn pick(self, preferred: Option<CpuId>, last_cpu: Option<CpuId>, cpu_count: usize) -> Option<CpuId> {
        if let Some(p) = preferred {
            if p < cpu_count && self.contains(p) {
                return Some(p);
            }
        }
        if let Some(l) = last_cpu {
            if l < cpu_count && self.contains(l) {
                return Some(l);
            }
        }
        let mut mask = self.0;
        while mask != 0 {
            let bit = mask.trailing_zeros() as CpuId;
            if bit < cpu_count {
                return Some(bit);
            }
            mask &= !(1u64 << bit);
        }
        None
    }
}

impl core::fmt::Debug for CpuSet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CpuSet({:#018x})", self.0)
    }
}

/// Rich CPU affinity model for a kernel thread.
///
/// Unlike the simple [`Affinity`] enum, `CpuAffinity` supports:
/// - A **bitmask** of allowed CPUs via [`CpuSet`], enabling flexible multi-CPU
///   constraints (not just "any CPU" or "one pinned CPU").
/// - An optional **preferred** CPU hint for guiding placement without hard pinning.
/// - An optional **last_cpu** recording the last CPU this thread was placed on
///   within the allowed set, for cache-warm placement decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuAffinity {
    /// Bitmask of CPUs on which this thread is allowed to run.
    pub allowed: CpuSet,
    /// Preferred CPU hint: the scheduler tries this CPU first when placing the task.
    /// `None` means no preference among the allowed CPUs.
    pub preferred: Option<CpuId>,
    /// The last CPU (within the allowed set) on which this thread was placed.
    /// Updated by the scheduler on each placement decision.
    pub last_cpu: Option<CpuId>,
}

impl CpuAffinity {
    /// Affinity that allows any CPU (equivalent to `Affinity::Any`).
    pub const fn any() -> Self {
        CpuAffinity { allowed: CpuSet::all(), preferred: None, last_cpu: None }
    }

    /// Affinity pinned to a single CPU (equivalent to `Affinity::Pinned(cpu)`).
    pub const fn pinned(cpu: CpuId) -> Self {
        CpuAffinity { allowed: CpuSet::only(cpu), preferred: Some(cpu), last_cpu: None }
    }

    /// Returns `true` if this task is allowed to run on `cpu` (within `cpu_count` online CPUs).
    #[inline]
    pub fn allows(&self, cpu: CpuId, cpu_count: usize) -> bool {
        cpu < cpu_count && self.allowed.contains(cpu)
    }

    /// Pick the best target CPU given the current online CPU count.
    /// Returns `None` if no allowed CPU is within `[0, cpu_count)`.
    #[inline]
    pub fn pick_cpu(&self, cpu_count: usize) -> Option<CpuId> {
        self.allowed.pick(self.preferred, self.last_cpu, cpu_count)
    }

    /// Effective degree of parallelism for a task with this affinity,
    /// capped at `cpu_count` online CPUs.  Always at least 1.
    pub fn effective_parallelism(&self, cpu_count: usize) -> usize {
        if cpu_count == 0 {
            return 1;
        }
        let mut count = 0usize;
        let mut mask = self.allowed.0;
        while mask != 0 {
            let bit = mask.trailing_zeros() as usize;
            if bit < cpu_count {
                count += 1;
            }
            mask &= !(1u64 << bit);
        }
        count.max(1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Affinity {
    /// Task may run on any online CPU.
    Any,
    /// Task is pinned to a single CPU index.
    Pinned(usize),
    /// Task is governed by a rich [`CpuAffinity`] policy.
    ///
    /// Use this variant when a task must be restricted to a subset of CPUs,
    /// or when a placement preference or last-run hint should guide the
    /// scheduler's pick without hard-pinning to a single CPU.
    Restricted(CpuAffinity),
}

impl Affinity {
    /// Returns `true` if the task is allowed to run on the given `cpu`
    /// (bounded by `cpu_count` online CPUs).
    #[inline]
    pub fn allows_cpu(&self, cpu: CpuId, cpu_count: usize) -> bool {
        match self {
            Affinity::Any => cpu < cpu_count,
            Affinity::Pinned(p) => *p == cpu && cpu < cpu_count,
            Affinity::Restricted(a) => a.allows(cpu, cpu_count),
        }
    }
}

/// Lifecycle state of a kernel thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Runnable,
    Running,
    Blocked,
    Dead,
}
/// Backward-compatible alias — prefer `ThreadState` in new code.
pub type TaskState = ThreadState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitReason {
    BlockCurrent,
}

/// CPU index type alias used by migration state.
pub type CpuId = usize;

/// Explicit migration state for a kernel thread.
///
/// Tracks the lifecycle of a task moving between CPUs so that migration is
/// always observable and never silently bypasses safety checks.  The valid
/// transitions are:
///
/// ```text
/// Local ──────────────────────► Requested { target }
///   ▲                                    │           │
///   │  (arrived at destination)          │ (steal /  │ (cancelled /
///   │                                    │  balance) │  target offline)
///   │                                    ▼           │
///   └──────────────────────────── InTransit          │
///                                                    ▼
///                                                  Local
///
/// Any ─────────────────────────► Pinned   (affinity locked)
/// Pinned ──────────────────────► Local    (affinity cleared)
/// ```
///
/// Tasks in the `Running` lifecycle state **must not** be migrated; call
/// [`MigrationState::is_migratable`] before attempting any move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationState {
    /// Task is resident on its current CPU with no pending migration.
    Local,
    /// Migration to `target` has been requested (e.g. by wake placement or
    /// the load balancer) but the task has not yet been dequeued for transit.
    Requested { target: CpuId },
    /// Task has been dequeued from the source CPU and is being placed on the
    /// destination CPU.  No further migration may be initiated until the task
    /// arrives and transitions back to `Local`.
    InTransit,
    /// Task is pinned by its affinity and may not be migrated.
    Pinned,
}

impl Default for MigrationState {
    fn default() -> Self {
        MigrationState::Local
    }
}

impl MigrationState {
    /// Return `true` if the task is currently eligible for migration.
    ///
    /// A task in `Running` lifecycle state must never be migrated, so callers
    /// should gate any migration attempt with this check combined with a
    /// `TaskState::Running` guard.  Additionally a `Pinned` or already
    /// `InTransit` task must not be re-migrated.
    #[inline]
    pub fn is_migratable(self) -> bool {
        matches!(self, MigrationState::Local | MigrationState::Requested { .. })
    }

    /// Attempt a state transition, returning the new state on success or `Err`
    /// with the current state on an illegal transition.
    ///
    /// Legal transitions:
    /// - `Local` → `Requested { target }`
    /// - `Local` → `Pinned`
    /// - `Requested { .. }` → `InTransit`
    /// - `Requested { .. }` → `Local`  (cancellation)
    /// - `InTransit` → `Local`          (arrival)
    /// - `Pinned` → `Local`             (affinity cleared)
    pub fn try_transition(self, next: MigrationState) -> Result<MigrationState, MigrationState> {
        let allowed = match (self, next) {
            (MigrationState::Local, MigrationState::Requested { .. }) => true,
            (MigrationState::Local, MigrationState::Pinned) => true,
            (MigrationState::Requested { .. }, MigrationState::InTransit) => true,
            (MigrationState::Requested { .. }, MigrationState::Local) => true,
            (MigrationState::InTransit, MigrationState::Local) => true,
            (MigrationState::Pinned, MigrationState::Local) => true,
            _ => false,
        };
        if allowed {
            Ok(next)
        } else {
            Err(self)
        }
    }
}

pub const WAKE_LATENCY_HIST_BUCKETS: usize = 5;
pub const IDLE_EPISODE_HIST_BUCKETS: usize = 4;
const RUNQ_STALE_PURGE_BUDGET: usize = 32;
const RUNQ_COMPACT_TRIGGER_MIN_LEN: usize = RUNQ_STALE_PURGE_BUDGET * 4;
const SLEEP_WHEEL_SLOTS: usize = 256;

/// Number of scheduling priority levels (Idle=0 … Realtime=4).
const PRIORITY_LEVELS: usize = 5;

/// Per-CPU priority-indexed run queue.
///
/// Index `p` holds runnable [`ThreadId`]s at priority level `p` (where
/// 0 = Idle and 4 = Realtime).  Each `RunQueue` belongs to exactly one
/// [`CpuScheduler`]; only the owning CPU's scheduler should normally
/// enqueue or dequeue from it.
///
/// # Ownership invariant
///
/// A CPU-local run queue should normally be mutated only by its owning
/// CPU scheduler.  Cross-CPU delivery is prepared for in future work
/// (issue #597).
///
/// # Index access
///
/// `RunQueue` implements [`Deref`] and [`DerefMut`] targeting the
/// underlying `[VecDeque<ThreadId>; PRIORITY_LEVELS]` so that existing
/// read-only or test-only indexed access (`runq[prio]`) continues to
/// compile.  Production enqueue/dequeue paths should use the typed
/// methods (`enqueue`, `pop_front_raw`, `remove_at`, `retain_at`) so
/// that `nonempty_runnable_mask` is kept in sync automatically.
pub struct RunQueue {
    queues: [VecDeque<ThreadId>; PRIORITY_LEVELS],
    /// Bitset of non-empty runnable queues for priorities 1..=4.
    /// Bit `p` is set when `queues[p]` currently has at least one entry.
    nonempty_runnable_mask: u8,
}

impl RunQueue {
    /// Create a new, empty run queue with pre-allocated capacity.
    pub fn new() -> Self {
        RunQueue {
            queues: [
                VecDeque::with_capacity(128),
                VecDeque::with_capacity(128),
                VecDeque::with_capacity(128),
                VecDeque::with_capacity(128),
                VecDeque::with_capacity(128),
            ],
            nonempty_runnable_mask: 0,
        }
    }

    /// Enqueue `tid` at the given `prio` level.
    ///
    /// Updates `nonempty_runnable_mask` when `prio > 0` (non-idle).
    pub fn enqueue(&mut self, prio: usize, tid: ThreadId) {
        self.queues[prio].push_back(tid);
        self.mark_nonempty(prio);
    }

    /// Pop and return the front entry at `prio`, updating the mask.
    ///
    /// This is a raw pop with no lazy-invalidation logic; callers that
    /// need stale-entry skipping should use the `SchedState` dequeue
    /// helpers which layer that logic on top.
    pub fn pop_front_raw(&mut self, prio: usize) -> Option<ThreadId> {
        let tid = self.queues[prio].pop_front()?;
        self.sync_mask_if_empty(prio);
        Some(tid)
    }

    /// Remove and return the entry at `idx` within `prio`, updating the mask.
    pub fn remove_at(&mut self, prio: usize, idx: usize) -> Option<ThreadId> {
        let tid = self.queues[prio].remove(idx)?;
        self.sync_mask_if_empty(prio);
        Some(tid)
    }

    /// Retain entries in `prio` matching `f`, then sync the mask.
    pub fn retain_at<F>(&mut self, prio: usize, mut f: F)
    where
        F: FnMut(&ThreadId) -> bool,
    {
        self.queues[prio].retain(|tid| f(tid));
        self.sync_mask_if_empty(prio);
    }

    /// Pick the highest-priority non-idle runnable entry and return it,
    /// removing it from the queue.
    ///
    /// Returns `None` if all non-idle queues are empty.  No
    /// lazy-invalidation is performed here; the caller is responsible for
    /// discarding stale results if needed.
    pub fn pick_next(&mut self) -> Option<(usize, ThreadId)> {
        for prio in (1..PRIORITY_LEVELS).rev() {
            if let Some(tid) = self.queues[prio].pop_front() {
                self.sync_mask_if_empty(prio);
                return Some((prio, tid));
            }
        }
        None
    }

    /// Total number of runnable threads across **non-idle** priority levels
    /// (priorities 1..=4).  This is the value that should be reported as
    /// the per-CPU runnable task count in debug/stats output.
    pub fn runnable_count(&self) -> usize {
        self.queues[1..].iter().map(|q| q.len()).sum()
    }

    /// Total number of entries across **all** priority levels, including
    /// the idle queue (priority 0).  Used for load-balance depth metrics.
    pub fn total_len(&self) -> usize {
        self.queues.iter().map(|q| q.len()).sum()
    }

    /// Returns `true` when all non-idle queues are empty.
    pub fn is_empty(&self) -> bool {
        self.nonempty_runnable_mask == 0
    }

    /// Returns `true` if any priority level **strictly above** `min_prio`
    /// contains at least one entry.  Used to detect preemption candidates.
    pub fn has_higher_priority_work(&self, min_prio: usize) -> bool {
        let start = (min_prio + 1).min(PRIORITY_LEVELS);
        self.queues[start..].iter().any(|q| !q.is_empty())
    }

    /// Bitmask of non-empty runnable queues (priorities 1..=4).
    #[inline]
    pub fn nonempty_runnable_mask(&self) -> u8 {
        self.nonempty_runnable_mask
    }

    // ── Internal mask helpers ──────────────────────────────────────────────

    #[inline]
    fn mark_nonempty(&mut self, prio: usize) {
        if prio > 0 {
            self.nonempty_runnable_mask |= 1u8 << prio;
        }
    }

    #[inline]
    fn sync_mask_if_empty(&mut self, prio: usize) {
        if prio > 0 && self.queues[prio].is_empty() {
            self.nonempty_runnable_mask &= !(1u8 << prio);
        }
    }
}

impl Default for RunQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl core::ops::Deref for RunQueue {
    type Target = [VecDeque<ThreadId>; PRIORITY_LEVELS];

    fn deref(&self) -> &Self::Target {
        &self.queues
    }
}

impl core::ops::DerefMut for RunQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queues
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnqueueCause {
    Unknown,
    Spawn,
    Wake,
    Steal,
    AffinityRepair,
    YieldRequeue,
}

#[derive(Debug, Clone, Copy)]
pub struct TaskRuntimeStats {
    pub run_count: u64,
    /// CFS-style fairness debt approximation used by scheduler pick heuristics.
    ///
    /// Lower values indicate less accumulated CPU service; picker logic can
    /// prefer tasks with lower debt when effective priority ties.
    pub fair_vruntime: u64,
    pub migration_count: u64,
    pub migration_wake: u64,
    pub migration_steal: u64,
    pub migration_affinity: u64,
    pub migration_yield_requeue: u64,
    pub migration_other: u64,
    pub runs_since_last_migration: u64,
    pub runs_between_migrations_total: u64,
    pub min_runs_between_migrations: u64,
    pub max_runs_between_migrations: u64,
    pub wake_to_run_count: u64,
    pub wake_to_run_ticks_total: u64,
    pub wake_to_run_ticks_max: u64,
    pub wake_to_run_hist: [u64; WAKE_LATENCY_HIST_BUCKETS],
}

impl Default for TaskRuntimeStats {
    fn default() -> Self {
        TaskRuntimeStats {
            run_count: 0,
            fair_vruntime: 0,
            migration_count: 0,
            migration_wake: 0,
            migration_steal: 0,
            migration_affinity: 0,
            migration_yield_requeue: 0,
            migration_other: 0,
            runs_since_last_migration: 0,
            runs_between_migrations_total: 0,
            min_runs_between_migrations: u64::MAX,
            max_runs_between_migrations: 0,
            wake_to_run_count: 0,
            wake_to_run_ticks_total: 0,
            wake_to_run_ticks_max: 0,
            wake_to_run_hist: [0; WAKE_LATENCY_HIST_BUCKETS],
        }
    }
}

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

/// Timer-wheel entry for a sleeping thread.
#[derive(Debug, Clone, Copy)]
pub struct SleepEntry {
    pub tid: ThreadId,
    pub wake_tick: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SleepMembership {
    pub wake_tick: u64,
    pub bucket_index: usize,
}

/// Entry pushed into a [`WakeMailbox`] by a remote CPU.
///
/// Carries all scheduling metadata needed to enqueue the woken task on the
/// destination CPU without additional registry lookups.
#[derive(Clone, Copy, Debug)]
pub struct WakeMailboxEntry {
    /// Identifier of the task (thread) being woken.
    ///
    /// `ThreadId` and `TaskId` are the same underlying type (`u64`); this
    /// field uses `ThreadId` to match the scheduler-layer naming convention.
    pub tid: ThreadId,
    /// Scheduling priority level (index into the run-queue priority array).
    pub priority: usize,
    /// Scheduler tick at which this wakeup was enqueued; used for aging.
    pub enqueued_at_tick: u64,
    /// Monotonic timestamp of the wakeup; used for age-histogram telemetry.
    pub wake_mono: u64,
}

/// Per-CPU cross-CPU wakeup mailbox.
///
/// Other CPUs must not directly enqueue tasks into a CPU's local run queue.
/// Instead they call [`WakeMailbox::push`] to deposit a [`WakeMailboxEntry`];
/// the owning CPU drains the mailbox at safe scheduling points
/// (start of `schedule()`, timer tick, exit from idle) and inserts the tasks
/// into its own run queue.
///
/// # Synchronization
///
/// `WakeMailbox` uses a [`spin::Mutex`] around the internal [`VecDeque`] to
/// protect concurrent push operations from multiple remote CPUs.  The
/// `pending` flag is an [`AtomicBool`] that allows the drain fast-path to skip
/// the lock entirely when no entries are present.
///
/// # Usage
///
/// ```text
/// // remote CPU:
/// cpu_n.wake_mailbox.push(WakeMailboxEntry { tid, priority, ... });
///
/// // owning CPU, at a safe scheduling point:
/// for entry in cpu_n.wake_mailbox.drain() {
///     cpu_n.runq.enqueue(entry.priority, entry.tid);
/// }
/// ```
pub struct WakeMailbox {
    inner: Mutex<VecDeque<WakeMailboxEntry>>,
    pending: AtomicBool,
}

impl WakeMailbox {
    /// Create a new, empty mailbox.
    ///
    /// This is a `const fn` so it can be used in static initializers.
    pub const fn new() -> Self {
        WakeMailbox {
            inner: Mutex::new(VecDeque::new()),
            pending: AtomicBool::new(false),
        }
    }

    /// Push a [`WakeMailboxEntry`] from a remote CPU.
    ///
    /// Safe to call from any CPU without holding the scheduler lock.  The
    /// internal [`spin::Mutex`] protects concurrent pushes from multiple
    /// remote CPUs.
    pub fn push(&self, entry: WakeMailboxEntry) {
        self.inner.lock().push_back(entry);
        self.pending.store(true, Ordering::Release);
    }

    /// Drain all pending entries, returning them as a [`VecDeque`].
    ///
    /// Returns an empty collection immediately when no entries are present
    /// (the `pending` fast-path avoids the lock).  Intended to be called
    /// **only** by the CPU that owns this mailbox at a safe scheduling point.
    pub fn drain(&self) -> VecDeque<WakeMailboxEntry> {
        if !self.pending.swap(false, Ordering::AcqRel) {
            return VecDeque::new();
        }
        core::mem::take(&mut *self.inner.lock())
    }

    /// Return `true` if there is at least one undelivered entry.
    ///
    /// This is a non-consuming hint; the owning CPU can use it to quickly
    /// check for pending cross-CPU wakeups before committing to a full drain.
    #[inline]
    pub fn is_pending(&self) -> bool {
        self.pending.load(Ordering::Acquire)
    }
}

/// Per-CPU scheduler state.
///
/// # Ownership invariant
///
/// Each logical CPU **owns** its `CpuScheduler` exclusively:
/// - `current` — the task currently executing on this CPU.
/// - `idle_task` — the CPU-local idle task.
/// - `runq` — the priority-indexed local run queue (see [`RunQueue`]).
/// - `need_resched` — the reschedule-pending flag.
/// - `stats` — per-CPU scheduling counters.
///
/// Other CPUs **must not** directly mutate another CPU's `CpuScheduler`.
/// Cross-CPU scheduling effects must go through explicit delivery mechanisms
/// (remote-wake mailboxes, IPIs).  The global [`SchedState`] coordinates
/// cross-CPU policy (task placement, load balancing, diagnostics) without
/// owning CPU-local execution state directly.
pub struct CpuScheduler {
    /// Logical index of the CPU that owns this scheduler state.
    pub cpu_id: usize,
    /// CPU-local priority-indexed run queue.
    ///
    /// Owned exclusively by this CPU scheduler.  The `nonempty_runnable_mask`
    /// is maintained inside `RunQueue`; access it via `runq.nonempty_runnable_mask()`.
    pub runq: RunQueue,
    pub idle_task: Option<ThreadId>,
    pub current: Option<ThreadId>,
    pub last_switch: u64,
    pub preempt_disable_depth: usize,
    pub preempt_disable_since: u64,
    pub preempt_watchdog_warned: bool,
    pub need_resched: bool,
    pub idle_enter_mono_ticks: Option<u64>,
    pub stats: PerCpuSchedStats,
    /// Per-CPU cross-CPU wakeup mailbox.
    ///
    /// Remote CPUs deliver tasks here instead of directly mutating this CPU's
    /// `runq`.  The owning CPU drains this mailbox at safe scheduling points
    /// (start of `schedule_point`, timer tick, idle exit) and locally enqueues
    /// any delivered tasks.  See [`WakeMailbox`] for the push/drain contract.
    pub wake_mailbox: WakeMailbox,
}

/// Backward-compatible alias — prefer [`CpuScheduler`] in new code.
pub type PerCpu = CpuScheduler;

/// Backward-compatible stats alias — prefer `CpuSchedStats` in new code for
/// consistency with the `CpuScheduler` naming convention.
pub type CpuSchedStats = PerCpuSchedStats;

impl CpuScheduler {
    /// Create a new scheduler for CPU 0.
    ///
    /// Prefer [`new_for_cpu`][Self::new_for_cpu] when the CPU index is known.
    pub fn new() -> Self {
        Self::new_for_cpu(0)
    }

    /// Create a new, empty per-CPU scheduler state for the given CPU.
    ///
    /// All run queues start empty and `current`/`idle_task` are `None`.
    pub fn new_for_cpu(cpu_id: usize) -> Self {
        CpuScheduler {
            cpu_id,
            runq: RunQueue::new(),
            idle_task: None,
            current: None,
            last_switch: 0,
            preempt_disable_depth: 0,
            preempt_disable_since: 0,
            preempt_watchdog_warned: false,
            need_resched: false,
            idle_enter_mono_ticks: None,
            stats: PerCpuSchedStats::default(),
            wake_mailbox: WakeMailbox::new(),
        }
    }

    /// Emit a debug log line describing the current per-CPU scheduler state.
    ///
    /// Identifies the owning CPU plus the currently running task, the idle
    /// task, the number of runnable tasks across all non-idle priority levels,
    /// and whether a reschedule is pending.  Call this from scheduling decision
    /// points to correlate log output with the CPU-local scheduler involved.
    pub fn log_state(&self) {
        let runnable = self.runq.runnable_count();
        crate::kdebug!(
            "SCHED[cpu{}]: current={:?} idle={:?} runnable={} need_resched={}",
            self.cpu_id,
            self.current,
            self.idle_task,
            runnable,
            self.need_resched,
        );
    }
}


#[derive(Debug, Clone, Copy, Default)]
pub struct PerCpuSchedStats {
    pub context_switches: u64,
    pub dispatch_count: u64,
    pub idle_to_nonidle: u64,
    pub steals_in: u64,
    pub steals_out: u64,
    pub timer_interrupts: u64,
    pub resched_ipi_received: u64,
    pub runnable_enqueues: u64,
    pub runnable_dequeues: u64,
    pub runq_depth_change_events: u64,
    pub wakeups: u64,
    pub lock_trylock_misses: u64,
    pub lock_trylock_misses_with_pending_resched: u64,
    pub lock_blocked_dispatch: u64,
    pub runq_sample_count: u64,
    pub runq_sample_total: u64,
    pub idle_total_us: u64,
    pub idle_episodes: u64,
    pub idle_longest_us: u64,
    pub idle_episode_hist: [u64; IDLE_EPISODE_HIST_BUCKETS],
    /// Number of entries pushed into this CPU's wake mailbox by remote CPUs.
    pub mailbox_pushes: u64,
    /// Number of drain operations performed on this CPU's wake mailbox.
    pub mailbox_drains: u64,
    /// Total number of tasks delivered from the wake mailbox into the local run queue.
    pub mailbox_tasks_drained: u64,
}

pub struct SchedState {
    /// TID-keyed scheduler cache.
    ///
    /// A map avoids the O(n) element shifts that came from keeping this in a
    /// sorted `Vec` during spawn/exit churn. We keep ordered iteration by TID
    /// via `BTreeMap` without positional coupling to registry storage.
    pub threads: BTreeMap<ThreadId, ThreadSchedFields>,
    pub thread_slot_by_tid: BTreeMap<ThreadId, usize>,
    pub free_thread_slots: Vec<usize>,
    pub next_thread_slot: usize,
    pub per_cpu: Vec<PerCpu>,
    /// Fixed-slot timer wheel for sleeping tasks.
    ///
    /// Slot `i` contains `SleepEntry` values whose `wake_tick % SLEEP_WHEEL_SLOTS == i`.
    /// Different deadlines can share a slot; each entry stores its full `wake_tick`.
    pub sleep_queue: Vec<Vec<SleepEntry>>,
    /// Next scheduler tick to scan in the sleep timer wheel.
    ///
    /// This advances monotonically during wake processing and may be rewound when
    /// a newly-added sleeper has an earlier wake tick.
    pub sleep_scan_tick: u64,
    pub sleep_membership: BTreeMap<ThreadId, SleepMembership>,
    pub wait_queue: BTreeSet<ThreadId>,
    pub wait_reasons: BTreeMap<ThreadId, WaitReason>,
    /// Per-task runtime/migration/latency statistics.
    pub task_runtime_stats: BTreeMap<ThreadId, TaskRuntimeStats>,
    /// Monotonic timestamp when a task was most recently made runnable by a wake path.
    pub wake_enqueued_at_mono: BTreeMap<ThreadId, u64>,
    /// Last enqueue cause tag for each task.
    pub last_enqueue_cause: BTreeMap<ThreadId, EnqueueCause>,
    pub online_cpu_count: usize,
    pub online_cpus: Vec<usize>,
}

/// CPU-local run-queue locks.
///
/// These locks protect per-CPU queue mutations (`enqueue/dequeue/compact`) so
/// hot queue traffic no longer relies solely on the global scheduler lock.
/// Shared structures (`threads`, sleep queue, wait queue, CPU topology) remain
/// protected by the scheduler-global lock in higher-level paths.
static PER_CPU_RUNQ_LOCKS: Once<Vec<Mutex<()>>> = Once::new();

#[inline]
fn per_cpu_runq_locks() -> &'static Vec<Mutex<()>> {
    PER_CPU_RUNQ_LOCKS.call_once(|| {
        let mut locks = Vec::with_capacity(crate::sched::types::MAX_CPUS);
        for _ in 0..crate::sched::types::MAX_CPUS {
            locks.push(Mutex::new(()));
        }
        locks
    })
}

#[inline]
fn lock_per_cpu_runq(cpu: usize) -> spin::mutex::MutexGuard<'static, ()> {
    per_cpu_runq_locks()
        .get(cpu)
        .unwrap_or_else(|| {
            panic!(
                "per-cpu runq lock index {} out of bounds [0, {})",
                cpu,
                crate::sched::types::MAX_CPUS
            )
        })
        .lock()
}

#[cfg(test)]
fn try_lock_per_cpu_runq(cpu: usize) -> Option<spin::mutex::MutexGuard<'static, ()>> {
    per_cpu_runq_locks().get(cpu).and_then(|lock| lock.try_lock())
}

impl SchedState {
    /// Validate run-queue CPU index against both lock capacity and initialized
    /// scheduler CPU state length.
    ///
    /// `MAX_CPUS` is the lock array upper bound, while `per_cpu.len()` reflects
    /// CPUs currently initialized in this scheduler instance (tests often use a
    /// smaller vector).
    #[inline]
    fn validate_runq_cpu(&self, cpu: usize, context: &str) -> bool {
        let valid = cpu < crate::sched::types::MAX_CPUS && cpu < self.per_cpu.len();
        if !valid {
            crate::kwarn!(
                "SCHED: {} ignoring invalid cpu {} (max_cpus={}, initialized_per_cpu={})",
                context,
                cpu,
                crate::sched::types::MAX_CPUS,
                self.per_cpu.len()
            );
        }
        valid
    }

    pub fn new() -> Self {
        SchedState {
            threads: BTreeMap::new(),
            thread_slot_by_tid: BTreeMap::new(),
            free_thread_slots: Vec::with_capacity(1024),
            next_thread_slot: 0,
            per_cpu: Vec::with_capacity(32),
            sleep_queue: (0..SLEEP_WHEEL_SLOTS).map(|_| Vec::new()).collect(),
            sleep_scan_tick: 0,
            sleep_membership: BTreeMap::new(),
            wait_queue: BTreeSet::new(),
            wait_reasons: BTreeMap::new(),
            task_runtime_stats: BTreeMap::new(),
            wake_enqueued_at_mono: BTreeMap::new(),
            last_enqueue_cause: BTreeMap::new(),
            online_cpu_count: 1,
            online_cpus: Vec::with_capacity(32),
        }
    }

    pub fn set_boot_cpu_online(&mut self) {
        self.online_cpus.clear();
        self.online_cpus.push(0);
        self.online_cpu_count = 1;
    }

    pub fn register_waiter(&mut self, tid: ThreadId, reason: WaitReason) -> bool {
        let inserted = self.wait_queue.insert(tid);
        if inserted || self.wait_reasons.get(&tid).copied() != Some(reason) {
            self.wait_reasons.insert(tid, reason);
        }
        inserted
    }

    pub fn unregister_waiter(&mut self, tid: ThreadId) -> bool {
        self.wait_reasons.remove(&tid);
        self.wait_queue.remove(&tid)
    }

    pub fn mark_cpu_online(&mut self, cpu: usize) {
        if !self.online_cpus.contains(&cpu) {
            self.online_cpus.push(cpu);
            self.online_cpus.sort_unstable();
        }
        self.online_cpu_count = self.online_cpus.len();
    }

    pub fn pick_online_cpu_excluding_bsp(&self, rr_idx: usize) -> usize {
        if self.online_cpus.len() <= 1 {
            return 0;
        }
        let secondary = &self.online_cpus[1..];
        secondary[rr_idx % secondary.len()]
    }

    pub fn pick_online_cpu(&self, rr_idx: usize) -> usize {
        if self.online_cpus.is_empty() {
            0
        } else {
            self.online_cpus[rr_idx % self.online_cpus.len()]
        }
    }

    pub fn get_thread_index(&self, tid: ThreadId) -> Option<usize> {
        self.thread_slot_by_tid.get(&tid).copied()
    }

    pub fn get_thread(&self, tid: ThreadId) -> Option<&ThreadSchedFields> {
        self.threads.get(&tid)
    }

    pub fn thread_ids(&self) -> Vec<ThreadId> {
        self.threads.keys().copied().collect()
    }

    pub fn get_thread_mut(&mut self, tid: ThreadId) -> Option<&mut ThreadSchedFields> {
        self.threads.get_mut(&tid)
    }

    pub fn insert_thread(&mut self, fields: ThreadSchedFields) {
        let tid = fields.tid;
        if self.threads.contains_key(&tid) {
            panic!("Thread ID {} already exists in sched", tid);
        }
        let slot = self.free_thread_slots.pop().unwrap_or_else(|| {
            let slot = self.next_thread_slot;
            self.next_thread_slot =
                self.next_thread_slot.checked_add(1).expect("scheduler thread slot overflow");
            slot
        });
        self.threads.insert(tid, fields);
        self.thread_slot_by_tid.insert(tid, slot);
        self.task_runtime_stats.entry(tid).or_default();
        self.last_enqueue_cause.insert(tid, EnqueueCause::Spawn);
    }

    /// Enqueue `tid` at priority `prio` on CPU `cpu`'s run queue.
    ///
    /// # Ownership contract
    ///
    /// **This function must only be called by the CPU that owns the run queue
    /// (i.e. `cpu == current_cpu_index()`).** All other callers must instead
    /// push a [`WakeMailboxEntry`] via the per-CPU [`WakeMailbox`] so that the
    /// owning CPU performs the enqueue at its next scheduling point.
    ///
    /// Violating this rule takes `PER_CPU_RUNQ_LOCKS[cpu]` from a foreign CPU,
    /// which defeats the per-CPU ownership invariant even though the global
    /// `SCHEDULER` lock prevents data races today.  Higher-level paths enforce
    /// this via [`debug_assert_runq_cpu_is_local`].
    pub fn enqueue_thread(&mut self, cpu: usize, prio: usize, tid: ThreadId) {
        if !self.validate_runq_cpu(cpu, "enqueue_thread") {
            return;
        }
        let _cpu_lock = lock_per_cpu_runq(cpu);
        if let Some(pc) = self.per_cpu.get_mut(cpu) {
            pc.runq.enqueue(prio, tid);
            pc.stats.runnable_enqueues = pc.stats.runnable_enqueues.saturating_add(1);
            pc.stats.runq_depth_change_events = pc.stats.runq_depth_change_events.saturating_add(1);
        }
        if let Some(t) = self.get_thread_mut(tid) {
            t.runq_location = Some((cpu, prio));
        }
    }

    pub fn dequeue_thread_front(&mut self, cpu: usize, prio: usize) -> Option<ThreadId> {
        if !self.validate_runq_cpu(cpu, "dequeue_thread_front") {
            return None;
        }
        let _cpu_lock = lock_per_cpu_runq(cpu);
        let SchedState { threads, per_cpu, .. } = self;

        if let Some(pc) = per_cpu.get_mut(cpu) {
            for _cleanup_attempt in 0..RUNQ_STALE_PURGE_BUDGET {
                let Some(tid) = pc.runq.pop_front_raw(prio) else {
                    break;
                };
                // Lazy-invalidation model: entries may stay in the VecDeque after
                // `remove_thread_from_runq` marks them not-enqueued.
                // Only return the entry if it still matches the task's canonical
                // runq placement metadata.
                let valid_location =
                    threads.get(&tid).and_then(|thread| thread.runq_location) == Some((cpu, prio));

                if !valid_location {
                    continue;
                }

                pc.stats.runnable_dequeues = pc.stats.runnable_dequeues.saturating_add(1);
                pc.stats.runq_depth_change_events =
                    pc.stats.runq_depth_change_events.saturating_add(1);
                if let Some(thread) = threads.get_mut(&tid) {
                    thread.runq_location = None;
                }
                return Some(tid);
            }
        }
        None
    }

    /// Remove and return the queue entry at a specific run-queue index.
    ///
    /// This validates that the candidate thread's canonical `runq_location`
    /// still points at `(cpu, prio)` before removing it, matching the same
    /// lazy-invalidation safety model used by `dequeue_thread_front`.
    ///
    /// Prefer `dequeue_thread_front` for FIFO consumption; use this only for
    /// bounded lookahead paths (e.g. steal) that intentionally target a
    /// non-front candidate.
    pub fn dequeue_thread_at(&mut self, cpu: usize, prio: usize, idx: usize) -> Option<ThreadId> {
        if !self.validate_runq_cpu(cpu, "dequeue_thread_at") {
            return None;
        }
        let _cpu_lock = lock_per_cpu_runq(cpu);
        let SchedState { threads, per_cpu, .. } = self;

        let pc = per_cpu.get_mut(cpu)?;
        let tid = pc.runq[prio].get(idx).copied()?;
        let valid_location =
            threads.get(&tid).and_then(|thread| thread.runq_location) == Some((cpu, prio));
        if !valid_location {
            return None;
        }

        if pc.runq.remove_at(prio, idx).is_none() {
            return None;
        }
        pc.stats.runnable_dequeues = pc.stats.runnable_dequeues.saturating_add(1);
        pc.stats.runq_depth_change_events = pc.stats.runq_depth_change_events.saturating_add(1);
        if let Some(thread) = threads.get_mut(&tid) {
            thread.runq_location = None;
        }
        Some(tid)
    }

    pub fn remove_thread_from_runq(&mut self, tid: ThreadId) -> bool {
        let (cpu, prio) = {
            let Some(t) = self.get_thread_mut(tid) else {
                return false;
            };
            let Some((cpu, prio)) = t.runq_location.take() else {
                return false;
            };
            (cpu, prio)
        };
        if !self.validate_runq_cpu(cpu, "remove_thread_from_runq") {
            return false;
        }
        let _cpu_lock = lock_per_cpu_runq(cpu);
        self.opportunistic_compact_runq(cpu, prio);
        true
    }

    pub fn remove_thread(&mut self, tid: ThreadId) -> bool {
        // Best-effort cleanup: thread may not be sleeping.
        let _ = self.remove_task_from_sleep_queue(tid);
        self.wake_enqueued_at_mono.remove(&tid);
        self.task_runtime_stats.remove(&tid);
        self.last_enqueue_cause.remove(&tid);
        if self.threads.remove(&tid).is_none() {
            return false;
        }
        if let Some(slot) = self.thread_slot_by_tid.remove(&tid) {
            self.free_thread_slots.push(slot);
        }
        true
    }

    #[inline]
    pub fn task_runtime_stats_mut(&mut self, tid: ThreadId) -> &mut TaskRuntimeStats {
        self.task_runtime_stats.entry(tid).or_default()
    }

    #[inline]
    pub fn task_runtime_stats(&self, tid: ThreadId) -> TaskRuntimeStats {
        self.task_runtime_stats.get(&tid).copied().unwrap_or_default()
    }

    #[inline]
    pub fn note_enqueue_cause(&mut self, tid: ThreadId, cause: EnqueueCause) {
        self.last_enqueue_cause.insert(tid, cause);
    }

    #[inline]
    pub fn last_enqueue_cause(&self, tid: ThreadId) -> EnqueueCause {
        self.last_enqueue_cause.get(&tid).copied().unwrap_or(EnqueueCause::Unknown)
    }

    pub fn add_task_to_sleep_queue(&mut self, tid: ThreadId, wake_tick: u64) {
        self.remove_task_from_sleep_queue(tid);

        let slot = sleep_wheel_slot(wake_tick);
        let idx = {
            let bucket = &mut self.sleep_queue[slot];
            bucket.push(SleepEntry { tid, wake_tick });
            bucket.len() - 1
        };
        self.sleep_membership.insert(tid, SleepMembership { wake_tick, bucket_index: idx });
        // If a newly inserted deadline is earlier than the next scan point,
        // rewind so wake processing does not skip this new sleeper.
        if wake_tick < self.sleep_scan_tick {
            self.sleep_scan_tick = wake_tick;
        }
    }

    pub fn remove_task_from_sleep_queue(&mut self, tid: ThreadId) -> bool {
        let Some(membership) = self.sleep_membership.get(&tid).copied() else {
            return false;
        };

        let mut removed = false;
        let mut moved: Option<(ThreadId, usize)> = None;
        let slot = sleep_wheel_slot(membership.wake_tick);

        if let Some(bucket) = self.sleep_queue.get_mut(slot) {
            let remove_idx = if bucket
                .get(membership.bucket_index)
                .is_some_and(|entry| entry.tid == tid && entry.wake_tick == membership.wake_tick)
            {
                Some(membership.bucket_index)
            } else {
                // Metadata can become stale when tests or transitional code
                // manipulate buckets directly; constrain fallback to this bucket
                // (never a global map scan).
                bucket.iter().position(|entry| entry.tid == tid)
            };

            if let Some(idx) = remove_idx {
                bucket.swap_remove(idx);
                if idx < bucket.len() {
                    moved = Some((bucket[idx].tid, idx));
                }
                removed = true;
            }
        }

        self.sleep_membership.remove(&tid);

        if let Some((moved_tid, moved_idx)) = moved {
            self.sleep_membership.insert(
                moved_tid,
                SleepMembership { wake_tick: membership.wake_tick, bucket_index: moved_idx },
            );
        }
        removed
    }

    /// Number of currently sleeping tasks tracked by direct membership index.
    ///
    /// This is `sleep_membership.len()` rather than a wheel-slot scan.
    #[inline]
    pub fn sleep_task_count(&self) -> usize {
        self.sleep_membership.len()
    }

    #[inline]
    pub fn sleep_queue_is_empty(&self) -> bool {
        self.sleep_membership.is_empty()
    }

    #[inline]
    pub fn sleep_bucket_contains(&self, wake_tick: u64) -> bool {
        let slot = sleep_wheel_slot(wake_tick);
        self.sleep_queue
            .get(slot)
            .is_some_and(|bucket| bucket.iter().any(|entry| entry.wake_tick == wake_tick))
    }

    pub fn sleep_bucket_snapshot(&self, wake_tick: u64) -> Option<Vec<ThreadId>> {
        let slot = sleep_wheel_slot(wake_tick);
        let bucket = self.sleep_queue.get(slot)?;
        let tids: Vec<ThreadId> = bucket
            .iter()
            .filter(|entry| entry.wake_tick == wake_tick)
            .map(|entry| entry.tid)
            .collect();
        if tids.is_empty() { None } else { Some(tids) }
    }

    pub fn take_due_sleepers(&mut self, now: u64, mut budget: usize) -> Vec<ThreadId> {
        let mut due = Vec::new();
        if budget == 0 {
            return due;
        }
        if self.sleep_membership.is_empty() {
            self.sleep_scan_tick = now;
            return due;
        }

        while budget > 0 && self.sleep_scan_tick <= now {
            let scan_tick = self.sleep_scan_tick;
            let slot = sleep_wheel_slot(scan_tick);
            let (sleep_queue, sleep_membership) =
                (&mut self.sleep_queue, &mut self.sleep_membership);
            let bucket = &mut sleep_queue[slot];
            let mut idx = 0;
            while idx < bucket.len() && budget > 0 {
                if bucket[idx].wake_tick <= now {
                    let entry = bucket.swap_remove(idx);
                    sleep_membership.remove(&entry.tid);
                    if idx < bucket.len() {
                        let moved = bucket[idx];
                        sleep_membership.insert(
                            moved.tid,
                            SleepMembership { wake_tick: moved.wake_tick, bucket_index: idx },
                        );
                    }
                    due.push(entry.tid);
                    budget -= 1;
                } else {
                    idx += 1;
                }
            }
            if budget == 0 {
                break;
            }
            if self.sleep_scan_tick == u64::MAX {
                break;
            }
            self.sleep_scan_tick += 1;
        }
        due
    }

    fn opportunistic_compact_runq(&mut self, cpu: usize, prio: usize) {
        let SchedState { threads, per_cpu, .. } = self;

        let Some(pc) = per_cpu.get_mut(cpu) else {
            return;
        };
        if pc.runq[prio].len() < RUNQ_COMPACT_TRIGGER_MIN_LEN {
            return;
        }

        let front_is_stale = pc.runq[prio].iter().take(RUNQ_STALE_PURGE_BUDGET).all(|entry_tid| {
            threads.get(entry_tid).and_then(|thread| thread.runq_location) != Some((cpu, prio))
        });
        if !front_is_stale {
            return;
        }

        pc.runq.retain_at(prio, |entry_tid| {
            threads.get(entry_tid).and_then(|thread| thread.runq_location) == Some((cpu, prio))
        });
    }

    // ── Backward-compatible forwarding methods ────────────────────────────────

    #[inline]
    pub fn get_task_index(&self, tid: ThreadId) -> Option<usize> {
        self.get_thread_index(tid)
    }
    #[inline]
    pub fn get_task(&self, tid: ThreadId) -> Option<&ThreadSchedFields> {
        self.get_thread(tid)
    }
    #[inline]
    pub fn get_task_mut(&mut self, tid: ThreadId) -> Option<&mut ThreadSchedFields> {
        self.get_thread_mut(tid)
    }
    #[inline]
    pub fn insert_task(&mut self, fields: ThreadSchedFields) {
        self.insert_thread(fields)
    }
    #[inline]
    pub fn enqueue_task(&mut self, cpu: usize, prio: usize, tid: ThreadId) {
        self.enqueue_thread(cpu, prio, tid)
    }
    #[inline]
    pub fn dequeue_task_front(&mut self, cpu: usize, prio: usize) -> Option<ThreadId> {
        self.dequeue_thread_front(cpu, prio)
    }
    #[inline]
    pub fn dequeue_task_at(&mut self, cpu: usize, prio: usize, idx: usize) -> Option<ThreadId> {
        self.dequeue_thread_at(cpu, prio, idx)
    }
    #[inline]
    pub fn remove_task_from_runq(&mut self, tid: ThreadId) -> bool {
        self.remove_thread_from_runq(tid)
    }
    #[inline]
    pub fn remove_task(&mut self, tid: ThreadId) -> bool {
        self.remove_thread(tid)
    }
}

#[inline]
fn sleep_wheel_slot(wake_tick: u64) -> usize {
    (wake_tick as usize) % SLEEP_WHEEL_SLOTS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sched_fields(tid: ThreadId, state: TaskState, priority: TaskPriority) -> ThreadSchedFields {
        ThreadSchedFields {
            tid,
            runq_location: None,
            state,
            priority,
            affinity: Affinity::Any,
            last_cpu: Some(0),
            wake_cpu: Some(0),
            run_cpu: None,
            timeslice_remaining: 0,
            enqueued_at_tick: 0,
            wake_pending: false,
            voluntary_yields: 0,
            migration_state: MigrationState::Local,
        }
    }

    #[test]
    fn default_sched_class_maps_priority_to_latency_domain() {
        assert_eq!(TaskPriority::Idle.default_sched_class(), TaskSchedClass::BackgroundMaintenance);
        assert_eq!(TaskPriority::Low.default_sched_class(), TaskSchedClass::BackgroundMaintenance);
        assert_eq!(TaskPriority::Normal.default_sched_class(), TaskSchedClass::NormalTimeslice);
        assert_eq!(TaskPriority::High.default_sched_class(), TaskSchedClass::NormalTimeslice);
        assert_eq!(TaskPriority::Realtime.default_sched_class(), TaskSchedClass::Realtime);
    }

    #[test]
    fn interrupt_bottom_half_is_not_assigned_by_default_priority_mapping() {
        let mapped = [
            TaskPriority::Idle.default_sched_class(),
            TaskPriority::Low.default_sched_class(),
            TaskPriority::Normal.default_sched_class(),
            TaskPriority::High.default_sched_class(),
            TaskPriority::Realtime.default_sched_class(),
        ];
        assert!(
            !mapped.contains(&TaskSchedClass::InterruptBottomHalf),
            "interrupt-bottom-half class should remain explicit rather than priority-derived"
        );
    }

    #[test]
    fn remove_thread_from_runq_uses_lazy_invalidation() {
        let mut state = SchedState::new();
        state.per_cpu.push(PerCpu::new());
        state.insert_thread(sched_fields(11, TaskState::Runnable, TaskPriority::Normal));
        state.enqueue_thread(0, TaskPriority::Normal as usize, 11);

        assert!(state.remove_thread_from_runq(11));
        assert_eq!(state.get_thread(11).and_then(|t| t.runq_location), None);
        assert!(
            state.per_cpu[0].runq[TaskPriority::Normal as usize].iter().any(|&tid| tid == 11),
            "lazy invalidation keeps stale entry in queue until dequeue"
        );
        assert_eq!(state.dequeue_thread_front(0, TaskPriority::Normal as usize), None);
        assert!(state.per_cpu[0].runq[TaskPriority::Normal as usize].is_empty());
    }

    #[test]
    fn stale_entry_is_skipped_after_requeueing_same_task() {
        let mut state = SchedState::new();
        state.per_cpu.push(PerCpu::new());
        state.insert_thread(sched_fields(12, TaskState::Runnable, TaskPriority::Normal));
        state.enqueue_thread(0, TaskPriority::Low as usize, 12);

        assert!(state.remove_thread_from_runq(12));
        state.enqueue_thread(0, TaskPriority::High as usize, 12);

        assert_eq!(
            state.dequeue_thread_front(0, TaskPriority::High as usize),
            Some(12),
            "valid re-enqueued entry should run"
        );
        assert_eq!(
            state.dequeue_thread_front(0, TaskPriority::Low as usize),
            None,
            "stale older queue entry must be skipped"
        );
    }

    #[test]
    fn stale_entries_do_not_increment_dequeue_stats() {
        let mut state = SchedState::new();
        state.per_cpu.push(PerCpu::new());
        state.insert_thread(sched_fields(13, TaskState::Runnable, TaskPriority::Normal));
        state.enqueue_thread(0, TaskPriority::Normal as usize, 13);
        assert!(state.remove_thread_from_runq(13));

        assert_eq!(state.per_cpu[0].stats.runnable_dequeues, 0);
        assert_eq!(state.dequeue_thread_front(0, TaskPriority::Normal as usize), None);
        assert_eq!(
            state.per_cpu[0].stats.runnable_dequeues, 0,
            "stale entries should not count as runnable dequeues"
        );

        state.enqueue_thread(0, TaskPriority::Normal as usize, 13);
        assert_eq!(state.dequeue_thread_front(0, TaskPriority::Normal as usize), Some(13));
        assert_eq!(
            state.per_cpu[0].stats.runnable_dequeues, 1,
            "valid dequeue should increment runnable_dequeues"
        );
    }

    #[test]
    fn dequeue_front_bounds_stale_cleanup_work_per_call() {
        let mut state = SchedState::new();
        state.per_cpu.push(PerCpu::new());

        // Exceed the single-call purge budget so one dequeue cannot fully clean
        // stale entries; leave two stale entries behind for the next call.
        let stale_count_exceeding_budget = RUNQ_STALE_PURGE_BUDGET + 2;
        for tid in 100..(100 + stale_count_exceeding_budget as u64) {
            state.insert_thread(sched_fields(tid, TaskState::Runnable, TaskPriority::Normal));
            state.enqueue_thread(0, TaskPriority::Normal as usize, tid);
            assert!(state.remove_thread_from_runq(tid));
        }

        let runnable_tid = 10_000;
        state.insert_thread(sched_fields(runnable_tid, TaskState::Runnable, TaskPriority::Normal));
        state.enqueue_thread(0, TaskPriority::Normal as usize, runnable_tid);

        assert_eq!(
            state.dequeue_thread_front(0, TaskPriority::Normal as usize),
            None,
            "first dequeue should stop after budgeted stale cleanup"
        );
        let expected_remaining_after_first_dequeue =
            stale_count_exceeding_budget - RUNQ_STALE_PURGE_BUDGET + 1;
        assert_eq!(
            state.per_cpu[0].runq[TaskPriority::Normal as usize].len(),
            expected_remaining_after_first_dequeue,
            "queue should still contain stale tail plus runnable task after bounded cleanup"
        );

        assert_eq!(
            state.dequeue_thread_front(0, TaskPriority::Normal as usize),
            Some(runnable_tid),
            "next dequeue should drain remaining stale entries then return runnable task"
        );
    }

    #[test]
    fn insert_remove_churn_preserves_tid_lookups() {
        let mut state = SchedState::new();
        for tid in 1..=128 {
            state.insert_thread(sched_fields(tid, TaskState::Runnable, TaskPriority::Normal));
        }
        let initial_slots: alloc::collections::BTreeMap<u64, usize> = (1..=128)
            .map(|tid| (tid, state.get_thread_index(tid).expect("slot for inserted tid")))
            .collect();

        for i in 0..2048 {
            let tid = 1 + (i % 128) as u64;
            assert!(state.remove_thread(tid));
            state.insert_thread(sched_fields(tid, TaskState::Runnable, TaskPriority::Normal));
        }

        assert_eq!(state.threads.len(), 128);
        for tid in 1..=128 {
            assert_eq!(state.get_thread(tid).map(|t| t.tid), Some(tid));
            assert_eq!(
                state.get_thread_index(tid),
                initial_slots.get(&tid).copied(),
                "slot index should remain stable across remove/insert churn for same tid"
            );
        }
    }

    #[test]
    fn sleep_membership_remove_updates_moved_index_and_cleans_empty_bucket() {
        let mut state = SchedState::new();
        state.add_task_to_sleep_queue(21, 100);
        state.add_task_to_sleep_queue(22, 100);

        assert!(state.remove_task_from_sleep_queue(21));
        assert_eq!(state.sleep_bucket_snapshot(100), Some(alloc::vec![22]));
        assert_eq!(
            state.sleep_membership.get(&22).copied(),
            Some(SleepMembership { wake_tick: 100, bucket_index: 0 })
        );

        assert!(state.remove_task_from_sleep_queue(22));
        assert!(!state.sleep_bucket_contains(100));
        assert!(!state.sleep_membership.contains_key(&22));
    }

    #[test]
    fn add_task_to_sleep_queue_moves_existing_membership_between_buckets() {
        let mut state = SchedState::new();
        state.add_task_to_sleep_queue(31, 11);
        state.add_task_to_sleep_queue(31, 12);

        assert!(
            state.sleep_bucket_snapshot(11).map_or(true, |v| v.is_empty()),
            "old bucket should be absent or empty after moving sleep membership"
        );
        assert_eq!(state.sleep_bucket_snapshot(12), Some(alloc::vec![31]));
        assert_eq!(
            state.sleep_membership.get(&31).copied(),
            Some(SleepMembership { wake_tick: 12, bucket_index: 0 })
        );
    }

    #[test]
    fn sleep_wheel_slot_collision_preserves_independent_deadlines() {
        let mut state = SchedState::new();
        state.add_task_to_sleep_queue(1001, 5);
        state.add_task_to_sleep_queue(1002, 5 + SLEEP_WHEEL_SLOTS as u64);

        assert_eq!(state.sleep_bucket_snapshot(5), Some(alloc::vec![1001]));
        assert_eq!(
            state.sleep_bucket_snapshot(5 + SLEEP_WHEEL_SLOTS as u64),
            Some(alloc::vec![1002])
        );

        let due_early = state.take_due_sleepers(5, 8);
        assert_eq!(due_early, alloc::vec![1001]);
        assert_eq!(
            state.sleep_bucket_snapshot(5 + SLEEP_WHEEL_SLOTS as u64),
            Some(alloc::vec![1002]),
            "future deadline sharing the same slot must remain queued"
        );

        let due_late = state.take_due_sleepers(5 + SLEEP_WHEEL_SLOTS as u64, 8);
        assert_eq!(due_late, alloc::vec![1002]);
        assert!(state.sleep_queue_is_empty());
    }

    #[test]
    fn register_waiter_deduplicates_tid_and_records_reason() {
        let mut state = SchedState::new();

        assert!(state.register_waiter(41, WaitReason::BlockCurrent));
        assert!(!state.register_waiter(41, WaitReason::BlockCurrent));
        assert!(state.wait_queue.contains(&41));
        assert_eq!(state.wait_queue.len(), 1);
        assert_eq!(state.wait_reasons.get(&41), Some(&WaitReason::BlockCurrent));
    }

    #[test]
    fn unregister_waiter_removes_wait_membership_and_reason() {
        let mut state = SchedState::new();
        state.register_waiter(42, WaitReason::BlockCurrent);

        assert!(state.unregister_waiter(42));
        assert!(!state.wait_queue.contains(&42));
        assert!(!state.wait_reasons.contains_key(&42));
        assert!(!state.unregister_waiter(42));
    }

    #[test]
    fn per_cpu_runq_locks_are_independent() {
        let cpu0_lock = try_lock_per_cpu_runq(0).expect("cpu0 runq lock should be acquirable");
        assert!(try_lock_per_cpu_runq(0).is_none(), "same CPU lock should not be re-entrant");
        assert!(try_lock_per_cpu_runq(1).is_some(), "different CPU lock should remain independent");
        drop(cpu0_lock);
        assert!(
            try_lock_per_cpu_runq(0).is_some(),
            "cpu0 runq lock should be acquirable again after release"
        );
    }

    #[test]
    fn runnable_mask_tracks_non_idle_enqueue_and_dequeue() {
        let mut state = SchedState::new();
        state.per_cpu.push(PerCpu::new());
        state.insert_thread(sched_fields(51, TaskState::Runnable, TaskPriority::Normal));

        state.enqueue_thread(0, TaskPriority::Normal as usize, 51);
        assert_ne!(
            state.per_cpu[0].runq.nonempty_runnable_mask() & (1u8 << TaskPriority::Normal as usize),
            0
        );

        assert_eq!(state.dequeue_thread_front(0, TaskPriority::Normal as usize), Some(51));
        assert_eq!(
            state.per_cpu[0].runq.nonempty_runnable_mask() & (1u8 << TaskPriority::Normal as usize),
            0
        );
    }

    #[test]
    fn runnable_mask_excludes_idle_queue() {
        let mut state = SchedState::new();
        state.per_cpu.push(PerCpu::new());
        state.insert_thread(sched_fields(61, TaskState::Runnable, TaskPriority::Idle));

        state.enqueue_thread(0, TaskPriority::Idle as usize, 61);
        assert_eq!(
            state.per_cpu[0].runq.nonempty_runnable_mask(), 0,
            "idle queue should not be marked as non-idle runnable work"
        );
    }

    // ── CpuScheduler-specific tests ──────────────────────────────────────────

    #[test]
    fn cpu_scheduler_new_for_cpu_stores_cpu_id() {
        let cs = CpuScheduler::new_for_cpu(3);
        assert_eq!(cs.cpu_id, 3, "cpu_id should match the argument passed to new_for_cpu");
    }

    #[test]
    fn cpu_scheduler_new_defaults_to_cpu_zero() {
        let cs = CpuScheduler::new();
        assert_eq!(cs.cpu_id, 0, "new() should default cpu_id to 0");
    }

    #[test]
    fn per_cpu_alias_resolves_to_cpu_scheduler() {
        // PerCpu is a type alias for CpuScheduler; constructing via the alias
        // should produce the same type and the cpu_id should default to 0.
        let pc = PerCpu::new();
        assert_eq!(pc.cpu_id, 0);
    }

    #[test]
    fn cpu_scheduler_starts_empty() {
        let cs = CpuScheduler::new_for_cpu(1);
        assert!(cs.current.is_none(), "new CpuScheduler should have no current task");
        assert!(cs.idle_task.is_none(), "new CpuScheduler should have no idle task");
        assert!(!cs.need_resched, "new CpuScheduler should not need rescheduling");
        assert_eq!(cs.runq.nonempty_runnable_mask(), 0, "new CpuScheduler should have empty run queues");
        assert_eq!(cs.runq.total_len(), 0, "all priority run queues should start empty");
    }

    #[test]
    fn cpu_scheduler_run_queue_is_per_cpu_type() {
        // Confirm that `runq` has the expected type (RunQueue = [VecDeque; 5]).
        // Accessing by index like a plain array should work.
        let mut cs = CpuScheduler::new_for_cpu(0);
        cs.runq[TaskPriority::Normal as usize].push_back(99);
        assert_eq!(cs.runq[TaskPriority::Normal as usize].len(), 1);
    }

    #[test]
    fn scoped_cpu_schedulers_have_independent_queues() {
        // Two CpuSchedulers should have fully independent run queues —
        // enqueuing to one must not affect the other.
        let mut cs0 = CpuScheduler::new_for_cpu(0);
        let mut cs1 = CpuScheduler::new_for_cpu(1);
        cs0.runq[TaskPriority::Normal as usize].push_back(1);
        assert_eq!(cs1.runq[TaskPriority::Normal as usize].len(), 0,
            "cpu1 run queue must not be affected by enqueue on cpu0");
        assert_eq!(cs0.cpu_id, 0);
        assert_eq!(cs1.cpu_id, 1);
    }

    // ── RunQueue struct tests ─────────────────────────────────────────────────

    #[test]
    fn run_queue_enqueue_updates_nonempty_mask() {
        let mut rq = RunQueue::new();
        assert_eq!(rq.nonempty_runnable_mask(), 0);
        rq.enqueue(TaskPriority::Normal as usize, 1);
        assert_ne!(
            rq.nonempty_runnable_mask() & (1u8 << TaskPriority::Normal as usize),
            0,
            "enqueue at Normal priority should set its bit in nonempty_runnable_mask"
        );
    }

    #[test]
    fn run_queue_enqueue_idle_does_not_set_mask() {
        let mut rq = RunQueue::new();
        rq.enqueue(TaskPriority::Idle as usize, 1);
        assert_eq!(
            rq.nonempty_runnable_mask(), 0,
            "idle-priority enqueue must not affect nonempty_runnable_mask"
        );
    }

    #[test]
    fn run_queue_pop_front_raw_clears_mask_on_drain() {
        let mut rq = RunQueue::new();
        rq.enqueue(TaskPriority::High as usize, 42);
        assert_eq!(rq.pop_front_raw(TaskPriority::High as usize), Some(42));
        assert_eq!(
            rq.nonempty_runnable_mask() & (1u8 << TaskPriority::High as usize),
            0,
            "mask bit should be cleared when queue becomes empty"
        );
    }

    #[test]
    fn run_queue_runnable_count_excludes_idle() {
        let mut rq = RunQueue::new();
        rq.enqueue(TaskPriority::Idle as usize, 1);
        rq.enqueue(TaskPriority::Normal as usize, 2);
        rq.enqueue(TaskPriority::High as usize, 3);
        assert_eq!(
            rq.runnable_count(), 2,
            "runnable_count should count only non-idle (prio >= 1) entries"
        );
    }

    #[test]
    fn run_queue_total_len_includes_idle() {
        let mut rq = RunQueue::new();
        rq.enqueue(TaskPriority::Idle as usize, 1);
        rq.enqueue(TaskPriority::Normal as usize, 2);
        assert_eq!(rq.total_len(), 2);
    }

    #[test]
    fn run_queue_is_empty_reflects_non_idle_queues() {
        let mut rq = RunQueue::new();
        assert!(rq.is_empty(), "new RunQueue should be empty");
        rq.enqueue(TaskPriority::Idle as usize, 1);
        assert!(rq.is_empty(), "idle-only entry should not make is_empty() false");
        rq.enqueue(TaskPriority::Normal as usize, 2);
        assert!(!rq.is_empty(), "non-idle entry should make is_empty() false");
    }

    #[test]
    fn run_queue_pick_next_returns_highest_priority() {
        let mut rq = RunQueue::new();
        rq.enqueue(TaskPriority::Low as usize, 10);
        rq.enqueue(TaskPriority::High as usize, 20);
        rq.enqueue(TaskPriority::Normal as usize, 30);
        let result = rq.pick_next();
        assert_eq!(
            result,
            Some((TaskPriority::High as usize, 20)),
            "pick_next should return the entry from the highest non-idle priority queue"
        );
        assert_eq!(rq.runnable_count(), 2, "pick_next should remove the returned entry");
    }

    #[test]
    fn run_queue_pick_next_returns_none_when_all_empty() {
        let mut rq = RunQueue::new();
        assert_eq!(rq.pick_next(), None, "pick_next on empty RunQueue should return None");
    }

    #[test]
    fn run_queue_pick_next_ignores_idle_queue() {
        let mut rq = RunQueue::new();
        rq.enqueue(TaskPriority::Idle as usize, 99);
        assert_eq!(
            rq.pick_next(), None,
            "pick_next should return None when only the idle queue is non-empty"
        );
    }

    #[test]
    fn run_queue_has_higher_priority_work() {
        let mut rq = RunQueue::new();
        rq.enqueue(TaskPriority::High as usize, 5);
        assert!(
            rq.has_higher_priority_work(TaskPriority::Normal as usize),
            "should detect High > Normal"
        );
        assert!(
            !rq.has_higher_priority_work(TaskPriority::High as usize),
            "should not find work strictly above High when only High is enqueued"
        );
    }

    #[test]
    fn run_queue_remove_at_updates_mask() {
        let mut rq = RunQueue::new();
        rq.enqueue(TaskPriority::Normal as usize, 7);
        rq.enqueue(TaskPriority::Normal as usize, 8);
        assert_eq!(rq.remove_at(TaskPriority::Normal as usize, 0), Some(7));
        assert_ne!(rq.nonempty_runnable_mask() & (1u8 << TaskPriority::Normal as usize), 0,
            "mask should remain set while queue still has entries");
        assert_eq!(rq.remove_at(TaskPriority::Normal as usize, 0), Some(8));
        assert_eq!(rq.nonempty_runnable_mask() & (1u8 << TaskPriority::Normal as usize), 0,
            "mask should be cleared when queue is fully drained via remove_at");
    }

    #[test]
    fn run_queue_retain_at_syncs_mask() {
        let mut rq = RunQueue::new();
        rq.enqueue(TaskPriority::Normal as usize, 1);
        rq.enqueue(TaskPriority::Normal as usize, 2);
        rq.retain_at(TaskPriority::Normal as usize, |tid| *tid == 2);
        assert_ne!(rq.nonempty_runnable_mask() & (1u8 << TaskPriority::Normal as usize), 0,
            "mask should remain set if retain leaves entries");
        rq.retain_at(TaskPriority::Normal as usize, |_| false);
        assert_eq!(rq.nonempty_runnable_mask() & (1u8 << TaskPriority::Normal as usize), 0,
            "mask should be cleared when retain removes all entries");
    }

    // ── WakeMailbox tests ─────────────────────────────────────────────────────

    #[test]
    fn wake_mailbox_starts_empty_and_not_pending() {
        let mb = WakeMailbox::new();
        assert!(!mb.is_pending(), "new mailbox should report no pending entries");
        assert!(mb.drain().is_empty(), "drain on empty mailbox should return empty");
    }

    #[test]
    fn wake_mailbox_push_sets_pending_flag() {
        let mb = WakeMailbox::new();
        mb.push(WakeMailboxEntry { tid: 1, priority: 2, enqueued_at_tick: 10, wake_mono: 100 });
        assert!(mb.is_pending(), "pending flag should be set after push");
    }

    #[test]
    fn wake_mailbox_drain_returns_pushed_entries_in_order() {
        let mb = WakeMailbox::new();
        mb.push(WakeMailboxEntry { tid: 10, priority: 1, enqueued_at_tick: 1, wake_mono: 1 });
        mb.push(WakeMailboxEntry { tid: 20, priority: 2, enqueued_at_tick: 2, wake_mono: 2 });
        mb.push(WakeMailboxEntry { tid: 30, priority: 3, enqueued_at_tick: 3, wake_mono: 3 });
        let drained = mb.drain();
        assert_eq!(drained.len(), 3, "drain should return all three entries");
        assert_eq!(drained[0].tid, 10, "first entry should be tid 10");
        assert_eq!(drained[1].tid, 20, "second entry should be tid 20");
        assert_eq!(drained[2].tid, 30, "third entry should be tid 30");
    }

    #[test]
    fn wake_mailbox_drain_clears_pending_and_leaves_empty() {
        let mb = WakeMailbox::new();
        mb.push(WakeMailboxEntry { tid: 5, priority: 2, enqueued_at_tick: 0, wake_mono: 0 });
        let _ = mb.drain();
        assert!(!mb.is_pending(), "pending flag should be cleared after drain");
        assert!(mb.drain().is_empty(), "second drain should return empty");
    }

    #[test]
    fn wake_mailbox_drain_without_pending_does_not_lock() {
        // When no push has been made, drain should return immediately without
        // touching the inner mutex.  A second drain should also be empty.
        let mb = WakeMailbox::new();
        let first = mb.drain();
        let second = mb.drain();
        assert!(first.is_empty());
        assert!(second.is_empty());
    }

    #[test]
    fn cpu_scheduler_has_wake_mailbox_field() {
        // Confirm that CpuScheduler exposes a wake_mailbox field that starts empty.
        let cs = CpuScheduler::new_for_cpu(2);
        assert!(!cs.wake_mailbox.is_pending(),
            "new CpuScheduler's wake_mailbox should start with no pending entries");
        assert!(cs.wake_mailbox.drain().is_empty(),
            "draining a fresh mailbox should yield no entries");
    }

    #[test]
    fn cpu_scheduler_wake_mailbox_push_and_drain() {
        let cs = CpuScheduler::new_for_cpu(4);
        cs.wake_mailbox.push(WakeMailboxEntry {
            tid: 99,
            priority: 2,
            enqueued_at_tick: 50,
            wake_mono: 1000,
        });
        assert!(cs.wake_mailbox.is_pending(), "mailbox should be pending after push");
        let entries = cs.wake_mailbox.drain();
        assert_eq!(entries.len(), 1, "drain should yield the pushed entry");
        assert_eq!(entries[0].tid, 99);
        assert_eq!(entries[0].priority, 2);
        assert!(!cs.wake_mailbox.is_pending(), "mailbox should be clear after drain");
    }

    #[test]
    fn per_cpu_sched_stats_has_mailbox_counters() {
        // Verify the three mailbox debug counters exist and default to zero.
        let stats = PerCpuSchedStats::default();
        assert_eq!(stats.mailbox_pushes, 0);
        assert_eq!(stats.mailbox_drains, 0);
        assert_eq!(stats.mailbox_tasks_drained, 0);
    }

    // -----------------------------------------------------------------------
    // MigrationState tests
    // -----------------------------------------------------------------------

    #[test]
    fn migration_state_default_is_local() {
        assert_eq!(MigrationState::default(), MigrationState::Local);
    }

    #[test]
    fn migration_state_local_is_migratable() {
        assert!(MigrationState::Local.is_migratable());
    }

    #[test]
    fn migration_state_requested_is_migratable() {
        assert!(MigrationState::Requested { target: 1 }.is_migratable());
    }

    #[test]
    fn migration_state_in_transit_is_not_migratable() {
        assert!(!MigrationState::InTransit.is_migratable());
    }

    #[test]
    fn migration_state_pinned_is_not_migratable() {
        assert!(!MigrationState::Pinned.is_migratable());
    }

    #[test]
    fn migration_state_legal_transitions() {
        // Local → Requested
        assert_eq!(
            MigrationState::Local.try_transition(MigrationState::Requested { target: 2 }),
            Ok(MigrationState::Requested { target: 2 }),
        );
        // Local → Pinned
        assert_eq!(
            MigrationState::Local.try_transition(MigrationState::Pinned),
            Ok(MigrationState::Pinned),
        );
        // Requested → InTransit
        assert_eq!(
            MigrationState::Requested { target: 2 }.try_transition(MigrationState::InTransit),
            Ok(MigrationState::InTransit),
        );
        // Requested → Local (cancellation)
        assert_eq!(
            MigrationState::Requested { target: 2 }.try_transition(MigrationState::Local),
            Ok(MigrationState::Local),
        );
        // InTransit → Local (arrival)
        assert_eq!(
            MigrationState::InTransit.try_transition(MigrationState::Local),
            Ok(MigrationState::Local),
        );
        // Pinned → Local (affinity cleared)
        assert_eq!(
            MigrationState::Pinned.try_transition(MigrationState::Local),
            Ok(MigrationState::Local),
        );
    }

    #[test]
    fn migration_state_illegal_transitions_return_err() {
        // InTransit → Requested is not allowed (no re-migration while in flight)
        assert!(
            MigrationState::InTransit
                .try_transition(MigrationState::Requested { target: 1 })
                .is_err()
        );
        // Pinned → InTransit is not allowed
        assert!(MigrationState::Pinned.try_transition(MigrationState::InTransit).is_err());
        // Local → InTransit is not a direct allowed transition
        assert!(MigrationState::Local.try_transition(MigrationState::InTransit).is_err());
        // InTransit → Pinned is not allowed
        assert!(MigrationState::InTransit.try_transition(MigrationState::Pinned).is_err());
    }

    #[test]
    fn thread_sched_fields_includes_migration_state() {
        let fields = sched_fields(1, TaskState::Runnable, TaskPriority::Normal);
        assert_eq!(fields.migration_state, MigrationState::Local);
    }

    // ── CpuSet tests ─────────────────────────────────────────────────────────

    #[test]
    fn cpu_set_all_contains_every_valid_cpu() {
        let set = CpuSet::all();
        assert!(set.contains(0));
        assert!(set.contains(1));
        assert!(set.contains(63));
        assert!(!set.contains(64), "index 64 is out of the 64-bit range");
    }

    #[test]
    fn cpu_set_only_contains_exactly_one_cpu() {
        let set = CpuSet::only(3);
        assert!(set.contains(3));
        assert!(!set.contains(0));
        assert!(!set.contains(2));
        assert!(!set.contains(4));
    }

    #[test]
    fn cpu_set_only_out_of_range_is_empty() {
        let set = CpuSet::only(64);
        assert!(!set.any(), "CpuSet::only(64) should produce an empty set");
    }

    #[test]
    fn cpu_set_first_returns_lowest_set_bit() {
        let set = CpuSet(0b1100); // bits 2 and 3 set
        assert_eq!(set.first(), Some(2));
    }

    #[test]
    fn cpu_set_first_on_empty_returns_none() {
        assert_eq!(CpuSet(0).first(), None);
    }

    #[test]
    fn cpu_set_count_matches_popcount() {
        assert_eq!(CpuSet(0b1011).count(), 3);
        assert_eq!(CpuSet(0).count(), 0);
        assert_eq!(CpuSet::only(7).count(), 1);
    }

    #[test]
    fn cpu_set_pick_prefers_preferred_when_allowed() {
        let set = CpuSet(0b1111); // CPUs 0..3 allowed
        assert_eq!(set.pick(Some(2), Some(1), 4), Some(2), "should pick preferred CPU 2");
    }

    #[test]
    fn cpu_set_pick_falls_back_to_last_cpu_when_preferred_absent() {
        let set = CpuSet(0b1111);
        assert_eq!(set.pick(None, Some(3), 4), Some(3), "should fall back to last_cpu");
    }

    #[test]
    fn cpu_set_pick_falls_back_to_first_allowed_cpu() {
        let set = CpuSet(0b1100); // CPUs 2 and 3
        assert_eq!(set.pick(None, None, 4), Some(2), "should return lowest allowed CPU");
    }

    #[test]
    fn cpu_set_pick_respects_cpu_count_bound() {
        let set = CpuSet::all();
        // Only 2 CPUs online, preferred=3 is out of range, last_cpu=5 is out of range
        assert_eq!(set.pick(Some(3), Some(5), 2), Some(0));
    }

    #[test]
    fn cpu_set_pick_returns_none_when_no_allowed_cpu_online() {
        let set = CpuSet::only(5); // CPU 5 allowed, but only 4 online
        assert_eq!(set.pick(None, None, 4), None);
    }

    // ── CpuAffinity tests ─────────────────────────────────────────────────────

    #[test]
    fn cpu_affinity_any_allows_all_cpus() {
        let aff = CpuAffinity::any();
        assert!(aff.allows(0, 4));
        assert!(aff.allows(3, 4));
        assert!(!aff.allows(4, 4), "out-of-bound CPU should not be allowed");
    }

    #[test]
    fn cpu_affinity_pinned_allows_only_specified_cpu() {
        let aff = CpuAffinity::pinned(2);
        assert!(!aff.allows(0, 4));
        assert!(!aff.allows(1, 4));
        assert!(aff.allows(2, 4));
        assert!(!aff.allows(3, 4));
    }

    #[test]
    fn cpu_affinity_pick_cpu_pinned_returns_pinned_cpu() {
        let aff = CpuAffinity::pinned(1);
        assert_eq!(aff.pick_cpu(4), Some(1));
    }

    #[test]
    fn cpu_affinity_pick_cpu_out_of_range_returns_none() {
        let aff = CpuAffinity::pinned(5); // only 4 CPUs online
        assert_eq!(aff.pick_cpu(4), None);
    }

    #[test]
    fn cpu_affinity_pick_cpu_prefers_preferred_hint() {
        let aff = CpuAffinity {
            allowed: CpuSet(0b1111), // CPUs 0-3
            preferred: Some(3),
            last_cpu: Some(0),
        };
        assert_eq!(aff.pick_cpu(4), Some(3), "preferred hint should win over last_cpu");
    }

    #[test]
    fn cpu_affinity_effective_parallelism_any_equals_cpu_count() {
        let aff = CpuAffinity::any();
        assert_eq!(aff.effective_parallelism(4), 4);
    }

    #[test]
    fn cpu_affinity_effective_parallelism_pinned_equals_one() {
        let aff = CpuAffinity::pinned(2);
        assert_eq!(aff.effective_parallelism(4), 1);
    }

    #[test]
    fn cpu_affinity_effective_parallelism_subset() {
        let aff = CpuAffinity {
            allowed: CpuSet(0b0110), // CPUs 1 and 2
            preferred: None,
            last_cpu: None,
        };
        assert_eq!(aff.effective_parallelism(4), 2);
    }

    #[test]
    fn cpu_affinity_effective_parallelism_zero_online_returns_one() {
        let aff = CpuAffinity::any();
        assert_eq!(aff.effective_parallelism(0), 1, "must return at least 1 even with 0 online CPUs");
    }

    // ── Affinity::allows_cpu tests ────────────────────────────────────────────

    #[test]
    fn affinity_any_allows_all_in_bounds_cpus() {
        assert!(Affinity::Any.allows_cpu(0, 4));
        assert!(Affinity::Any.allows_cpu(3, 4));
        assert!(!Affinity::Any.allows_cpu(4, 4));
    }

    #[test]
    fn affinity_pinned_allows_only_target_cpu() {
        assert!(Affinity::Pinned(2).allows_cpu(2, 4));
        assert!(!Affinity::Pinned(2).allows_cpu(1, 4));
        assert!(!Affinity::Pinned(2).allows_cpu(3, 4));
    }

    #[test]
    fn affinity_restricted_respects_allowed_set() {
        let aff = CpuAffinity { allowed: CpuSet(0b0110), preferred: None, last_cpu: None };
        assert!(!Affinity::Restricted(aff).allows_cpu(0, 4));
        assert!(Affinity::Restricted(aff).allows_cpu(1, 4));
        assert!(Affinity::Restricted(aff).allows_cpu(2, 4));
        assert!(!Affinity::Restricted(aff).allows_cpu(3, 4));
    }
}
