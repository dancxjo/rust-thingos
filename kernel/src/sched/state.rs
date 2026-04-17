use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;

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

/// Scheduler-side metadata for a single kernel thread.
///
/// This struct carries scheduler-internal queue tracking state plus a
/// **hot-field cache** of frequently read values from `Thread<R>` in the
/// global registry.  The cache avoids taking the REGISTRY lock inside
/// the SCHEDULER hot path (nested locking was the primary source of lock
/// convoy behaviour under SMP).
///
/// # Canonical fields (no equivalent in `Thread<R>`)
/// - `tid` — needed to keep the scheduler's sorted `Vec` indexed in the same
///   order as `ThreadRegistry::threads` so that a single binary-search index
///   addresses both collections.
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
///   the priority-aging fairness logic in `prepare_schedule` without
///   re-entering REGISTRY.
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
    pub enqueued_at_tick: u64,
    /// Cached copy of `Thread<R>::wake_pending`.
    ///
    /// Set by `wake_task_locked` when the target task is not already blocked
    /// (preemptive wake signal).  Checked and cleared by `block_current` so
    /// that the hot `wake_pending` check no longer re-enters the REGISTRY lock
    /// while the SCHEDULER lock is held.
    pub wake_pending: bool,
}
/// Backward-compatible alias — prefer `ThreadSchedFields` in new code.
pub type TaskSchedFields = ThreadSchedFields;

#[derive(Debug, Clone, Copy)]
pub struct SleepEntry {
    pub tid: ThreadId,
    pub wake_tick: u64,
}

pub struct PerCpu {
    pub runq: [VecDeque<ThreadId>; 5],
    pub idle_task: Option<ThreadId>,
    pub current: Option<ThreadId>,
    pub last_switch: u64,
    pub need_resched: bool,
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
            idle_task: None,
            current: None,
            last_switch: 0,
            need_resched: false,
            stats: PerCpuSchedStats::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PerCpuSchedStats {
    pub context_switches: u64,
    pub idle_to_nonidle: u64,
    pub timer_interrupts: u64,
    pub resched_ipi_received: u64,
    pub runnable_enqueues: u64,
    pub runnable_dequeues: u64,
    pub wakeups: u64,
    pub lock_trylock_misses: u64,
    pub lock_trylock_misses_with_pending_resched: u64,
    pub lock_blocked_dispatch: u64,
    pub runq_sample_count: u64,
    pub runq_sample_total: u64,
}

pub struct SchedState {
    pub threads: Vec<ThreadSchedFields>,
    pub per_cpu: Vec<PerCpu>,
    pub sleep_queue: BTreeMap<u64, Vec<ThreadId>>,
    pub wait_queue: VecDeque<ThreadId>,
    pub online_cpu_count: usize,
    pub online_cpus: Vec<usize>,
}

impl SchedState {
    pub fn new() -> Self {
        SchedState {
            threads: Vec::with_capacity(1024),
            per_cpu: Vec::with_capacity(32),
            sleep_queue: BTreeMap::new(),
            wait_queue: VecDeque::with_capacity(1024),
            online_cpu_count: 1,
            online_cpus: Vec::with_capacity(32),
        }
    }

    pub fn set_boot_cpu_online(&mut self) {
        self.online_cpus.clear();
        self.online_cpus.push(0);
        self.online_cpu_count = 1;
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
        self.threads.binary_search_by_key(&tid, |t| t.tid).ok()
    }

    pub fn get_thread(&self, tid: ThreadId) -> Option<&ThreadSchedFields> {
        self.get_thread_index(tid).map(|idx| &self.threads[idx])
    }

    pub fn get_thread_mut(&mut self, tid: ThreadId) -> Option<&mut ThreadSchedFields> {
        self.get_thread_index(tid)
            .map(move |idx| &mut self.threads[idx])
    }

    pub fn insert_thread(&mut self, fields: ThreadSchedFields) {
        match self.threads.binary_search_by_key(&fields.tid, |t| t.tid) {
            Ok(_) => panic!("Thread ID {} already exists in sched", fields.tid),
            Err(idx) => self.threads.insert(idx, fields),
        }
    }

    pub fn enqueue_thread(&mut self, cpu: usize, prio: usize, tid: ThreadId) {
        if let Some(pc) = self.per_cpu.get_mut(cpu) {
            pc.runq[prio].push_back(tid);
            pc.stats.runnable_enqueues = pc.stats.runnable_enqueues.saturating_add(1);
        }
        if let Some(t) = self.get_thread_mut(tid) {
            t.runq_location = Some((cpu, prio));
        }
    }

    pub fn dequeue_thread_front(&mut self, cpu: usize, prio: usize) -> Option<ThreadId> {
        let SchedState {
            threads,
            per_cpu,
            ..
        } = self;

        if let Some(pc) = per_cpu.get_mut(cpu) {
            while let Some(tid) = pc.runq[prio].pop_front() {
                // Lazy-invalidation model: entries may stay in the VecDeque after
                // `remove_thread_from_runq` marks them not-enqueued.
                // Only return the entry if it still matches the task's canonical
                // runq placement metadata.
                let valid_location = threads
                    .binary_search_by_key(&tid, |t| t.tid)
                    .ok()
                    .and_then(|idx| threads[idx].runq_location)
                    == Some((cpu, prio));

                if !valid_location {
                    continue;
                }

                pc.stats.runnable_dequeues = pc.stats.runnable_dequeues.saturating_add(1);
                if let Ok(idx) = threads.binary_search_by_key(&tid, |t| t.tid) {
                    threads[idx].runq_location = None;
                }
                return Some(tid);
            }
        }
        None
    }

    pub fn remove_thread_from_runq(&mut self, tid: ThreadId) -> bool {
        if let Some(t) = self.get_thread_mut(tid) {
            if t.runq_location.is_some() {
                t.runq_location = None;
                return true;
            }
        }
        false
    }

    pub fn remove_thread(&mut self, tid: ThreadId) -> bool {
        if let Ok(idx) = self.threads.binary_search_by_key(&tid, |t| t.tid) {
            self.threads.remove(idx);
            true
        } else {
            false
        }
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
    pub fn remove_task_from_runq(&mut self, tid: ThreadId) -> bool {
        self.remove_thread_from_runq(tid)
    }
    #[inline]
    pub fn remove_task(&mut self, tid: ThreadId) -> bool {
        self.remove_thread(tid)
    }
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
        }
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
            state.per_cpu[0].runq[TaskPriority::Normal as usize]
                .iter()
                .any(|&tid| tid == 11),
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
            state.per_cpu[0].stats.runnable_dequeues,
            0,
            "stale entries should not count as runnable dequeues"
        );

        state.enqueue_thread(0, TaskPriority::Normal as usize, 13);
        assert_eq!(state.dequeue_thread_front(0, TaskPriority::Normal as usize), Some(13));
        assert_eq!(
            state.per_cpu[0].stats.runnable_dequeues,
            1,
            "valid dequeue should increment runnable_dequeues"
        );
    }
}
