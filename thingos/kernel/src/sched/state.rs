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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Affinity {
    Any,
    Pinned(usize),
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
}
