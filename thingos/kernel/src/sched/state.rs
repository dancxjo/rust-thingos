use alloc::collections::{BTreeMap, BTreeSet, VecDeque};
use alloc::vec::Vec;

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

pub struct PerCpu {
    pub runq: [VecDeque<ThreadId>; 5],
    /// Bitset of non-empty runnable queues for priorities 1..=4.
    /// Bit `p` is set when `runq[p]` currently has at least one entry.
    pub nonempty_runnable_mask: u8,
    pub idle_task: Option<ThreadId>,
    pub current: Option<ThreadId>,
    pub last_switch: u64,
    pub preempt_disable_depth: usize,
    pub preempt_disable_since: u64,
    pub preempt_watchdog_warned: bool,
    pub need_resched: bool,
    pub idle_enter_mono_ticks: Option<u64>,
    pub stats: PerCpuSchedStats,
}

impl PerCpu {
    pub fn new() -> Self {
        PerCpu {
            runq: [
                VecDeque::with_capacity(128),
                VecDeque::with_capacity(128),
                VecDeque::with_capacity(128),
                VecDeque::with_capacity(128),
                VecDeque::with_capacity(128),
            ],
            nonempty_runnable_mask: 0,
            idle_task: None,
            current: None,
            last_switch: 0,
            preempt_disable_depth: 0,
            preempt_disable_since: 0,
            preempt_watchdog_warned: false,
            need_resched: false,
            idle_enter_mono_ticks: None,
            stats: PerCpuSchedStats::default(),
        }
    }

    #[inline]
    fn mark_runnable_nonempty(&mut self, prio: usize) {
        if prio > 0 {
            self.nonempty_runnable_mask |= 1u8 << prio;
        }
    }

    #[inline]
    fn clear_runnable_if_empty(&mut self, prio: usize) {
        if prio > 0 && self.runq[prio].is_empty() {
            self.nonempty_runnable_mask &= !(1u8 << prio);
        }
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
            pc.runq[prio].push_back(tid);
            pc.mark_runnable_nonempty(prio);
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
                let Some(tid) = pc.runq[prio].pop_front() else {
                    pc.clear_runnable_if_empty(prio);
                    break;
                };
                pc.clear_runnable_if_empty(prio);
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

        let removed = pc.runq[prio].remove(idx);
        if removed.is_none() {
            return None;
        }
        pc.clear_runnable_if_empty(prio);
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
        let runq = &mut pc.runq[prio];
        if runq.len() < RUNQ_COMPACT_TRIGGER_MIN_LEN {
            return;
        }

        let front_is_stale = runq.iter().take(RUNQ_STALE_PURGE_BUDGET).all(|entry_tid| {
            threads.get(entry_tid).and_then(|thread| thread.runq_location) != Some((cpu, prio))
        });
        if !front_is_stale {
            return;
        }

        runq.retain(|entry_tid| {
            threads.get(entry_tid).and_then(|thread| thread.runq_location) == Some((cpu, prio))
        });
        pc.clear_runnable_if_empty(prio);
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
            state.per_cpu[0].nonempty_runnable_mask & (1u8 << TaskPriority::Normal as usize),
            0
        );

        assert_eq!(state.dequeue_thread_front(0, TaskPriority::Normal as usize), Some(51));
        assert_eq!(
            state.per_cpu[0].nonempty_runnable_mask & (1u8 << TaskPriority::Normal as usize),
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
            state.per_cpu[0].nonempty_runnable_mask, 0,
            "idle queue should not be marked as non-idle runnable work"
        );
    }
}
