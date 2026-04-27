use super::{IDLE_EPISODE_HIST_BUCKETS, RunQueue, ThreadId, WakeMailbox};

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
