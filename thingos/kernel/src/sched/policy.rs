//! Scheduler policy interface.
//!
//! This module separates scheduling *policy* (decision logic) from scheduling
//! *mechanism* (enqueue, dequeue, context-switch, mailbox delivery).
//!
//! ## Split summary
//!
//! **Mechanism** (lives in `Scheduler` and `SchedState`):
//! - Enqueue / dequeue tasks in run queues.
//! - Perform context switches.
//! - Deliver remote-wake mailbox entries.
//!
//! **Policy** (lives in this module via `SchedPolicy`):
//! - [`SchedPolicy::pick_next_task`] — choose the next task to run on a CPU.
//! - [`SchedPolicy::choose_cpu`] — choose which CPU a woken task should run on.
//! - [`SchedPolicy::should_preempt_on_wake`] — decide whether an incoming task
//!   should preempt the currently running task.
//!
//! The default implementation is [`DefaultPolicy`], which encodes the existing
//! CFS-inspired priority + aging behaviour.  Custom policies can be provided
//! for testing or for evolving the scheduler without touching mechanism code.

use crate::sched::state::SchedState;

/// The scheduling policy interface.
///
/// Implementations answer the three core scheduling decisions without
/// performing any mechanical operations (no enqueue, dequeue, or switch).
/// All inputs are read-only views of scheduler state; side-effects belong
/// in the mechanism layer.
pub trait SchedPolicy {
    /// Choose the next task to run on `cpu`.
    ///
    /// Scans the per-CPU run queues as of `now_tick` and returns
    /// `(priority_queue_index, candidate_index_within_queue)` for the best
    /// runnable candidate, or `None` when no candidate is found.
    ///
    /// The caller (mechanism layer) is responsible for dequeuing the returned
    /// candidate and performing the actual context switch.
    fn pick_next_task(
        &self,
        state: &SchedState,
        cpu: usize,
        now_tick: u64,
    ) -> Option<(usize, usize)>;

    /// Choose the target CPU for a task with `Affinity::Any` that is about to
    /// be made runnable.
    ///
    /// Returns the CPU index that should receive the woken task.  The
    /// `preferred_cpu` hint is the CPU on which the task last ran (its
    /// locality preference).  The `local_cpu` is the CPU making the
    /// scheduling decision.
    fn choose_cpu(
        &self,
        state: &SchedState,
        preferred_cpu: usize,
        local_cpu: usize,
    ) -> usize;

    /// Decide whether the currently running task on `cpu` should be
    /// preempted by an incoming task of `incoming_priority`.
    ///
    /// Called when a remote-wake mailbox entry is drained and a new task is
    /// enqueued on `cpu`.  Returns `true` if `need_resched` should be set
    /// for that CPU.
    fn should_preempt_on_wake(
        &self,
        state: &SchedState,
        cpu: usize,
        incoming_priority: usize,
    ) -> bool;
}

// ---------------------------------------------------------------------------
// Pick-candidate helpers (pure, no side-effects)
// ---------------------------------------------------------------------------

/// Return `true` when `(eff, vruntime, queue_idx)` is a better pick than the
/// current best `(best_eff, best_vruntime, best_q)`.
///
/// Selection order:
/// 1. Higher effective priority always wins.
/// 2. Tie on priority: lower virtual runtime wins (CFS-style).
/// 3. Tie on both: lower base-priority queue wins (larger age debt).
#[inline]
pub(super) fn is_better_pick_candidate(
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
    vruntime == best_vruntime && best_q.is_some_and(|bq| queue_idx < bq)
}

// ---------------------------------------------------------------------------
// DefaultPolicy
// ---------------------------------------------------------------------------

/// Default scheduling policy.
///
/// Implements a CFS-inspired priority scheduler with:
/// - Per-priority run queues (Idle=0, Low=1, Normal=2, High=3, Realtime=4).
/// - Anti-starvation aging: waiting tasks gain effective-priority boosts every
///   [`crate::sched::types::AGING_THRESHOLD_TICKS`] ticks.
/// - Virtual-runtime tie-breaking: equal-priority tasks are ordered by
///   accumulated CPU time.
/// - Any-affinity wake placement: routes to the preferred CPU unless it is
///   overloaded, in which case the least-loaded online CPU is chosen.
/// - Preemption on wake: a waking task preempts the current task when its
///   effective priority is at least as high as the current task's, or when
///   the CPU is running its idle task.
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultPolicy;

impl DefaultPolicy {
    /// Number of priority queue levels (Idle=0, Low=1, Normal=2, High=3, RT=4).
    const RUNQ_LEVELS: usize = 5;

    /// Maximum number of candidates to inspect per priority level per pick.
    const FAIR_SCAN_DEPTH: usize = 8;

    /// Minimum run-queue depth difference before redirecting a woken task to a
    /// less-loaded CPU.
    const OVERLOAD_GAP: usize = 2;

    pub fn new() -> Self {
        DefaultPolicy
    }
}

impl SchedPolicy for DefaultPolicy {
    fn pick_next_task(
        &self,
        state: &SchedState,
        cpu: usize,
        now_tick: u64,
    ) -> Option<(usize, usize)> {
        let per_cpu = state.per_cpu.get(cpu)?;
        let scan_base = per_cpu.stats.dispatch_count as usize;

        let mut best_q: Option<usize> = None;
        let mut best_idx: Option<usize> = None;
        let mut best_eff: usize = 0;
        let mut best_vruntime: u64 = u64::MAX;

        // Scan priority queues highest-first (Realtime=4 → Low=1).
        // Queue 0 (Idle) is handled separately by the caller's fallback path.
        for p in (1..Self::RUNQ_LEVELS).rev() {
            let runq_len = per_cpu.runq[p].len();
            if runq_len == 0 {
                continue;
            }
            let scan_len = runq_len.min(Self::FAIR_SCAN_DEPTH);
            for step in 0..scan_len {
                let idx = (scan_base + step) % runq_len;
                let Some(&tid) = per_cpu.runq[p].get(idx) else {
                    continue;
                };
                let Some(sf) = state.get_thread(tid) else {
                    continue;
                };
                // Skip tasks that are no longer on this CPU's queue.
                if sf.runq_location != Some((cpu, p)) {
                    continue;
                }
                use crate::task::TaskState;
                if sf.state == TaskState::Dead || sf.state == TaskState::Blocked {
                    continue;
                }

                // Compute aging boost (capped at MAX_PRIORITY_BOOST, not for Realtime).
                let mut eff = p;
                if p < 4 {
                    let wait = now_tick.saturating_sub(sf.enqueued_at_tick);
                    let boost =
                        (wait / crate::sched::types::AGING_THRESHOLD_TICKS) as usize;
                    let boost = boost.min(crate::sched::types::MAX_PRIORITY_BOOST);
                    eff = (p + boost).min(4);
                }
                let vruntime = state.task_runtime_stats(tid).fair_vruntime;

                if is_better_pick_candidate(eff, vruntime, p, best_eff, best_vruntime, best_q) {
                    best_eff = eff;
                    best_vruntime = vruntime;
                    best_q = Some(p);
                    best_idx = Some(idx);
                }
            }
        }

        match (best_q, best_idx) {
            (Some(q), Some(i)) => Some((q, i)),
            _ => None,
        }
    }

    fn choose_cpu(
        &self,
        state: &SchedState,
        preferred_cpu: usize,
        local_cpu: usize,
    ) -> usize {
        // Resolve preferred to a valid online CPU.
        let preferred = if preferred_cpu < state.per_cpu.len()
            && state.online_cpus.contains(&preferred_cpu)
        {
            preferred_cpu
        } else if local_cpu < state.per_cpu.len() {
            local_cpu
        } else {
            0
        };

        // If only one CPU is online (or preferred has spare capacity), stay local.
        if state.online_cpus.len() <= 1 {
            return preferred;
        }

        // Find the least-loaded online CPU.
        let preferred_depth =
            state.per_cpu.get(preferred).map(|pc| pc.runq.total_len()).unwrap_or(0);

        let Some((least_cpu, least_depth)) = state
            .online_cpus
            .iter()
            .copied()
            .filter(|&c| c < state.per_cpu.len())
            .map(|c| (c, state.per_cpu[c].runq.total_len()))
            .min_by_key(|&(_, d)| d)
        else {
            return preferred;
        };

        // Only redirect when the preferred CPU has at least OVERLOAD_GAP more tasks.
        if preferred_depth.saturating_sub(least_depth) >= Self::OVERLOAD_GAP
            && least_cpu != preferred
        {
            least_cpu
        } else {
            preferred
        }
    }

    fn should_preempt_on_wake(
        &self,
        state: &SchedState,
        cpu: usize,
        incoming_priority: usize,
    ) -> bool {
        let Some(pc) = state.per_cpu.get(cpu) else {
            return false;
        };
        // Always preempt when the CPU is running its idle task.
        let is_idle = matches!((pc.current, pc.idle_task), (Some(cur), Some(idle)) if cur == idle);
        if is_idle {
            return true;
        }
        // Preempt if the incoming task has at least as high a priority as the
        // currently running task.
        let current_prio = pc
            .current
            .and_then(|cid| state.get_thread(cid))
            .map(|sf| sf.priority as usize)
            .unwrap_or(0);
        incoming_priority >= current_prio
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sched::state::{CpuScheduler, SchedState, ThreadSchedFields};
    use crate::sched::types::DEFAULT_TIMESLICE;
    use crate::task::{Affinity, TaskPriority, TaskState};

    fn make_thread_fields(
        tid: u64,
        prio: usize,
        task_state: TaskState,
        cpu: usize,
    ) -> ThreadSchedFields {
        let priority = match prio {
            0 => TaskPriority::Idle,
            1 => TaskPriority::Low,
            2 => TaskPriority::Normal,
            3 => TaskPriority::High,
            _ => TaskPriority::Realtime,
        };
        ThreadSchedFields {
            tid,
            priority,
            state: task_state,
            timeslice_remaining: DEFAULT_TIMESLICE,
            enqueued_at_tick: 0,
            run_cpu: Some(cpu),
            wake_cpu: None,
            last_cpu: None,
            runq_location: None,
            affinity: Affinity::Any,
            voluntary_yields: 0,
            wake_pending: false,
            migration_state: crate::sched::state::MigrationState::Local,
        }
    }

    fn make_state_with_tasks(
        tasks: &[(u64, usize, TaskState)],
        cpu: usize,
    ) -> SchedState {
        let mut state = SchedState::new();
        while state.per_cpu.len() <= cpu {
            state.per_cpu.push(CpuScheduler::new_for_cpu(state.per_cpu.len()));
        }
        if !state.online_cpus.contains(&cpu) {
            state.online_cpus.push(cpu);
        }
        for &(tid, prio, task_state) in tasks {
            let sf = make_thread_fields(tid, prio, task_state, cpu);
            state.insert_thread(sf);
            if task_state == TaskState::Runnable {
                state.enqueue_thread(cpu, prio, tid);
            }
        }
        state
    }

    #[test]
    fn pick_next_prefers_higher_priority() {
        let state = make_state_with_tasks(
            &[
                (1, TaskPriority::Normal as usize, TaskState::Runnable),
                (2, TaskPriority::High as usize, TaskState::Runnable),
            ],
            0,
        );
        let policy = DefaultPolicy::new();
        let result = policy.pick_next_task(&state, 0, 0);
        assert!(result.is_some(), "should find a candidate");
        let (q, _) = result.unwrap();
        // High priority = queue index 3 in the [Idle=0,Low=1,Normal=2,High=3,RT=4] layout
        assert_eq!(q, TaskPriority::High as usize, "should prefer high-priority queue");
    }

    #[test]
    fn pick_next_returns_none_when_all_queues_empty() {
        let state = SchedState::new();
        let policy = DefaultPolicy::new();
        let result = policy.pick_next_task(&state, 0, 0);
        assert!(result.is_none());
    }

    #[test]
    fn pick_next_skips_dead_tasks() {
        let state = make_state_with_tasks(
            &[
                (1, TaskPriority::Normal as usize, TaskState::Dead),
                (2, TaskPriority::Low as usize, TaskState::Runnable),
            ],
            0,
        );
        let policy = DefaultPolicy::new();
        let result = policy.pick_next_task(&state, 0, 0);
        assert!(result.is_some());
        let (q, _) = result.unwrap();
        assert_eq!(q, TaskPriority::Low as usize, "should skip dead task in Normal queue");
    }

    #[test]
    fn should_preempt_on_wake_true_when_idle() {
        let mut state = SchedState::new();
        state.per_cpu.push(CpuScheduler::new_for_cpu(0));
        let idle_tid = 99u64;
        state.per_cpu[0].current = Some(idle_tid);
        state.per_cpu[0].idle_task = Some(idle_tid);

        let policy = DefaultPolicy::new();
        assert!(policy.should_preempt_on_wake(&state, 0, TaskPriority::Low as usize));
    }

    #[test]
    fn should_preempt_on_wake_true_when_equal_priority() {
        let mut state = SchedState::new();
        state.per_cpu.push(CpuScheduler::new_for_cpu(0));
        let current_tid = 1u64;
        let idle_tid = 99u64;
        state.per_cpu[0].current = Some(current_tid);
        state.per_cpu[0].idle_task = Some(idle_tid);
        state.insert_thread(make_thread_fields(
            current_tid,
            TaskPriority::Normal as usize,
            TaskState::Running,
            0,
        ));

        let policy = DefaultPolicy::new();
        assert!(policy.should_preempt_on_wake(&state, 0, TaskPriority::Normal as usize));
    }

    #[test]
    fn should_preempt_on_wake_false_when_lower_priority() {
        let mut state = SchedState::new();
        state.per_cpu.push(CpuScheduler::new_for_cpu(0));
        let current_tid = 1u64;
        let idle_tid = 99u64;
        state.per_cpu[0].current = Some(current_tid);
        state.per_cpu[0].idle_task = Some(idle_tid);
        state.insert_thread(make_thread_fields(
            current_tid,
            TaskPriority::High as usize,
            TaskState::Running,
            0,
        ));

        let policy = DefaultPolicy::new();
        assert!(!policy.should_preempt_on_wake(&state, 0, TaskPriority::Low as usize));
    }

    #[test]
    fn choose_cpu_returns_preferred_when_not_overloaded() {
        let mut state = SchedState::new();
        for i in 0..2 {
            state.per_cpu.push(CpuScheduler::new_for_cpu(i));
            state.online_cpus.push(i);
        }
        let policy = DefaultPolicy::new();
        // Both CPUs have depth 0 — not overloaded.
        assert_eq!(policy.choose_cpu(&state, 0, 0), 0);
    }

    #[test]
    fn choose_cpu_redirects_when_preferred_overloaded() {
        let mut state = SchedState::new();
        for i in 0..2 {
            state.per_cpu.push(CpuScheduler::new_for_cpu(i));
            state.online_cpus.push(i);
        }
        // Overload CPU 0 with 3 tasks.
        for tid in 1u64..=3u64 {
            let sf = make_thread_fields(tid, TaskPriority::Normal as usize, TaskState::Runnable, 0);
            state.insert_thread(sf);
            state.enqueue_thread(0, TaskPriority::Normal as usize, tid);
        }
        let policy = DefaultPolicy::new();
        // CPU 0 has depth 3, CPU 1 has depth 0: redirect to CPU 1.
        let chosen = policy.choose_cpu(&state, 0, 0);
        assert_eq!(chosen, 1, "should redirect to less-loaded CPU 1");
    }

    #[test]
    fn is_better_pick_candidate_prefers_higher_effective_priority() {
        assert!(is_better_pick_candidate(3, 100, 2, 2, 50, Some(3)));
        assert!(!is_better_pick_candidate(2, 100, 2, 3, 50, Some(3)));
    }

    #[test]
    fn is_better_pick_candidate_tiebreaks_by_vruntime() {
        // Same eff, lower vruntime wins.
        assert!(is_better_pick_candidate(2, 50, 2, 2, 100, Some(3)));
        assert!(!is_better_pick_candidate(2, 100, 2, 2, 50, Some(3)));
    }

    #[test]
    fn is_better_pick_candidate_tiebreaks_by_queue_index() {
        // Same eff + vruntime: lower queue index (higher age debt) wins.
        assert!(is_better_pick_candidate(2, 50, 1, 2, 50, Some(2)));
        assert!(!is_better_pick_candidate(2, 50, 2, 2, 50, Some(1)));
    }
}
