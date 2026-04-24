//! Core scheduler types and data structures.

use alloc::collections::BTreeMap;
use core::marker::PhantomData;

use crate::BootRuntime;
use crate::task::{TaskId, TaskState};

/// Default time slice in ticks (~100ms at 100Hz timer)
pub const DEFAULT_TIMESLICE: u32 = 10;

/// Number of sleeper wakeups budgeted per scheduler tick.
pub const WAKE_SLEEPERS_BUDGET_PER_TICK: usize = 64;

/// Maximum accumulated wake budget carry between ticks.
pub const WAKE_SLEEPERS_BUDGET_CARRY_CAP: usize = WAKE_SLEEPERS_BUDGET_PER_TICK * 8;

/// Maximum number of CPUs supported
pub const MAX_CPUS: usize = 32;
const _ASSERT_MAX_CPUS_FITS_IN_U64: [(); 1] = [(); (MAX_CPUS <= u64::BITS as usize) as usize];

/// Anti-starvation: ticks to wait before boosting priority by one level
/// At 100Hz, 500 ticks = ~5 seconds
///
/// This ensures low-priority tasks don't starve even when high-priority tasks
/// are continuously runnable. After waiting for AGING_THRESHOLD_TICKS, a task's
/// priority is temporarily boosted by one level until it gets scheduled.
pub const AGING_THRESHOLD_TICKS: u64 = 500;

/// Anti-starvation: maximum priority boost levels (prevents excessive boosting)
///
/// Limits how many priority levels a task can be boosted. For example, with
/// MAX_PRIORITY_BOOST = 2, a Low priority task can be boosted to at most High
/// priority (Low -> Normal -> High), but never to Realtime.
pub const MAX_PRIORITY_BOOST: usize = 2;

/// Consecutive spin-yield threshold before requeue placement is penalized.
///
/// A task that repeatedly yields without blocking can consume scheduler
/// attention without useful work. Once it has crossed this threshold and is
/// observed yielding again from a prior yield requeue, the scheduler demotes
/// its immediate requeue band by [`SPIN_YIELD_PENALTY_BANDS`].
pub const SPIN_YIELD_PENALTY_THRESHOLD: u64 = 8;

/// Number of priority bands to demote pathological spin-yielders on requeue.
pub const SPIN_YIELD_PENALTY_BANDS: usize = 1;

// PerCpu is now in state.rs

/// Class-specific scheduling policy.
///
/// Current scheduler behavior still uses a single per-priority run queue, but
/// this policy contract defines distinct latency domains so class-specific
/// semantics can be adopted without overloading priority values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchedulerClassPolicy {
    pub class: crate::task::TaskSchedClass,
    pub min_priority: crate::task::TaskPriority,
    pub max_priority: crate::task::TaskPriority,
    pub default_timeslice_ticks: u32,
    pub aging_enabled: bool,
    pub preempt_lower_on_wake: bool,
}

/// Return the canonical scheduler policy for a latency class.
///
/// The returned policy encodes the current class contract: allowed priority
/// band, default timeslice, whether aging is enabled, and whether wakeups in
/// this class should preempt lower-priority work.
pub const fn policy_for_sched_class(class: crate::task::TaskSchedClass) -> SchedulerClassPolicy {
    match class {
        crate::task::TaskSchedClass::NormalTimeslice => SchedulerClassPolicy {
            class,
            min_priority: crate::task::TaskPriority::Normal,
            max_priority: crate::task::TaskPriority::High,
            default_timeslice_ticks: DEFAULT_TIMESLICE,
            aging_enabled: true,
            preempt_lower_on_wake: true,
        },
        crate::task::TaskSchedClass::Realtime => SchedulerClassPolicy {
            class,
            min_priority: crate::task::TaskPriority::Realtime,
            max_priority: crate::task::TaskPriority::Realtime,
            default_timeslice_ticks: DEFAULT_TIMESLICE,
            aging_enabled: false,
            preempt_lower_on_wake: true,
        },
        crate::task::TaskSchedClass::InterruptBottomHalf => SchedulerClassPolicy {
            class,
            min_priority: crate::task::TaskPriority::High,
            max_priority: crate::task::TaskPriority::Realtime,
            default_timeslice_ticks: 1,
            aging_enabled: false,
            preempt_lower_on_wake: true,
        },
        crate::task::TaskSchedClass::BackgroundMaintenance => SchedulerClassPolicy {
            class,
            min_priority: crate::task::TaskPriority::Idle,
            max_priority: crate::task::TaskPriority::Low,
            default_timeslice_ticks: DEFAULT_TIMESLICE,
            aging_enabled: true,
            preempt_lower_on_wake: false,
        },
    }
}

/// Convenience wrapper returning class policy for a task priority.
///
/// This uses `TaskPriority::default_sched_class()` so the current priority
/// lattice can opt into explicit class semantics without changing queue layout.
pub const fn policy_for_priority(priority: crate::task::TaskPriority) -> SchedulerClassPolicy {
    policy_for_sched_class(priority.default_sched_class())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackFaultResult {
    NotStack,
    Grew,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleReason {
    /// Timer ISR path only — runs tick bookkeeping (wake sleepers, watchdog,
    /// wait-time aging, timeslice decrement).
    PreemptTick,
    CooperativeYield,
    SleepWait,
    BlockedOnIo,
    /// Used by preempt_enable(). Yields if need_resched is set but does NOT
    /// run tick bookkeeping or decrement timeslices.
    SafePoint,
    /// Used by explicit resched_if_needed() checks at syscall-return or other
    /// safe points. Same behaviour as SafePoint, semantically distinct.
    ReschedIfNeeded,
}

pub struct SwitchParams<Ctx, AS> {
    pub from_ctx: *mut Ctx,
    pub to_ctx: *const Ctx,
    pub to_aspace: AS,
    pub from_aspace: AS,
    pub from_tid: TaskId,
    pub to_tid: TaskId,
    pub from_user: bool,
    pub to_user: bool,
    /// Pointer into the outgoing task's `user_fs_base` field (saved on switch-out).
    pub from_user_fs_base: *mut u64,
    /// The incoming task's saved TLS base (restored on switch-in).
    pub to_user_fs_base: u64,
}

/// Scheduler pick/commit decision produced under the SCHEDULER lock.
///
/// The expensive context materialization (`REGISTRY` lookup, SIMD save/restore,
/// mapping-cache update) is performed later, after dropping SCHEDULER.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwitchDecision {
    pub cpu_idx: usize,
    pub from_tid: TaskId,
    pub to_tid: TaskId,
}

/// Deferred REGISTRY synchronization for one task.
///
/// `prepare_schedule` records scheduling-side state transitions in the
/// scheduler hot cache first, then emits one or more of these updates so the
/// canonical REGISTRY record can be synchronized after the SCHEDULER lock is
/// released. Lock-owning scheduler call sites are responsible for draining and
/// applying these updates once they drop `SCHEDULER`. Replay coalesces by task
/// and applies batched updates in a single pass (see
/// `apply_deferred_registry_syncs` users in `sched`/`task` modules).
pub(crate) struct DeferredRegistrySync {
    /// Task being synchronized.
    pub tid: TaskId,
    /// New lifecycle state to write when a transition happened.
    pub new_state: Option<TaskState>,
    /// New enqueue tick when transitioning back to Runnable.
    pub new_enqueued_at_tick: Option<u64>,
    /// Last CPU the task was observed running or switching on.
    pub new_last_cpu: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct RemoteWakeMailboxEntry {
    pub tid: TaskId,
    pub priority: usize,
    pub enqueued_at_tick: u64,
    pub wake_mono: u64,
}

pub(crate) struct SchedulerMetrics {
    pub yields: u64,
    pub pops: u64,
    pub pushes: u64,
    pub idle_picks: u64,
    pub steals: u64,
    pub last_flush: u64,
}

pub struct Scheduler<R: BootRuntime> {
    pub(crate) state: crate::sched::state::SchedState,
    pub(crate) next_id: TaskId,
    pub(crate) total_cpu_count: usize,
    pub(crate) bringup_in_progress: bool,
    pub(crate) metrics: SchedulerMetrics,
    /// IPIs deferred by `wake_sleepers`. Populated under the SCHEDULER lock and
    /// drained after the lock is released so that `send_ipi` is never called
    /// while SCHEDULER is held.
    pub(crate) pending_wake_ipis: alloc::vec::Vec<usize>,
    /// Unused wake budget carried forward to future ticks.
    pub(crate) wake_sleepers_budget_carry: usize,
    /// Per-task requested sleep duration (ticks) captured at sleep registration.
    /// Used by Any-affinity wake placement to avoid bouncing brief sleepers.
    pub(crate) sleep_duration_ticks_by_tid: BTreeMap<TaskId, u64>,
    /// Bitmap of CPUs with deferred `prepare_schedule` misroute IPIs.
    /// Populated under the SCHEDULER lock and drained after the lock is
    /// released so remote nudges never run in the scheduler critical section.
    pub(crate) pending_prepare_schedule_ipis_bitmap: u64,
    /// Misrouted tasks deferred by `prepare_schedule`.
    /// Repaired in bounded maintenance batches outside the picker path so
    /// selection stays focused on choosing the next runnable task.
    pub(crate) pending_misrouted_requeues: alloc::vec::Vec<(usize, usize, TaskId)>,
    /// REGISTRY synchronization work deferred out of `prepare_schedule` so the
    /// hot selection path can update scheduler cache fields without immediate
    /// REGISTRY lock coupling. Callers must drain/apply after releasing
    /// SCHEDULER.
    pub(crate) pending_registry_syncs: alloc::vec::Vec<DeferredRegistrySync>,
    /// Tasks created while the scheduler lock is held that must be inserted
    /// into the canonical REGISTRY after `SCHEDULER` is released.
    pub(crate) pending_registry_inserts: alloc::vec::Vec<alloc::boxed::Box<crate::task::Task<R>>>,
    /// When true imbalance condition became active (any idle CPU while another CPU has depth >1).
    pub(crate) imbalance_active_since_mono: Option<u64>,
    /// Total time spent in imbalance condition (mono ticks converted to µs for reporting).
    pub(crate) imbalance_total_us: u64,
    /// Number of imbalance episodes observed.
    pub(crate) imbalance_episodes: u64,
    /// Longest single imbalance episode (µs).
    pub(crate) imbalance_longest_us: u64,
    _phantom: core::marker::PhantomData<R>,
}

impl SchedulerMetrics {
    pub fn new() -> Self {
        SchedulerMetrics { yields: 0, pops: 0, pushes: 0, idle_picks: 0, steals: 0, last_flush: 0 }
    }
}

impl<R: BootRuntime> Scheduler<R> {
    pub fn new() -> Self {
        Scheduler {
            state: crate::sched::state::SchedState::new(),
            next_id: 1,
            total_cpu_count: 1,
            bringup_in_progress: false,
            metrics: SchedulerMetrics::new(),
            pending_wake_ipis: alloc::vec::Vec::new(),
            wake_sleepers_budget_carry: 0,
            sleep_duration_ticks_by_tid: BTreeMap::new(),
            pending_prepare_schedule_ipis_bitmap: 0,
            pending_misrouted_requeues: alloc::vec::Vec::new(),
            pending_registry_syncs: alloc::vec::Vec::new(),
            pending_registry_inserts: alloc::vec::Vec::new(),
            imbalance_active_since_mono: None,
            imbalance_total_us: 0,
            imbalance_episodes: 0,
            imbalance_longest_us: 0,
            _phantom: PhantomData,
        }
    }

    pub fn current_id(&self) -> Option<TaskId> {
        // This is tricky without knowing which CPU we are asking about.
        // For backwards compat logging, valid use mainly inside scheduler or per-cpu hooks.
        // We really need current_cpu_index here.
        // But Scheduler::current_id passed no index.
        // We will return None or rely on caller to use per-cpu accessors.
        // Actually, let's remove this helper or make it panic/useless?
        // Or better: `Scheduler` methods should generally task `cpu_index`?
        None
    }

    pub fn current_id_on_cpu(&self, cpu: usize) -> Option<TaskId> {
        self.state.per_cpu.get(cpu).and_then(|pc| pc.current)
    }

    pub fn current_priority(&self) -> Option<crate::task::TaskPriority> {
        // Also needs cpu index.
        None
    }

    pub fn current_priority_on_cpu(&self, cpu: usize) -> Option<crate::task::TaskPriority> {
        let tid = self.current_id_on_cpu(cpu)?;
        crate::task::registry::get_task::<R>(tid).map(|t| t.priority)
    }

    pub fn drain_pending_registry_inserts(
        &mut self,
    ) -> alloc::vec::Vec<alloc::boxed::Box<crate::task::Task<R>>> {
        core::mem::take(&mut self.pending_registry_inserts)
    }

    /// Returns `true` if there is at least one task in a non-idle run queue
    /// (priority levels 1–4) for the given CPU. Used by `run_scheduler` to
    /// decide whether to halt or keep spinning.
    pub fn has_runnable_work(&self, cpu_idx: usize) -> bool {
        if let Some(pc) = self.state.per_cpu.get(cpu_idx) {
            pc.runq.nonempty_runnable_mask() != 0
        } else {
            false
        }
    }

    #[inline]
    pub(crate) fn queue_pending_prepare_schedule_ipi(&mut self, target_cpu: usize) {
        if target_cpu >= MAX_CPUS {
            return;
        }
        self.pending_prepare_schedule_ipis_bitmap |= 1u64 << target_cpu;
    }

    pub(crate) fn drain_pending_prepare_schedule_ipis(&mut self) -> alloc::vec::Vec<usize> {
        let bitmap = core::mem::replace(&mut self.pending_prepare_schedule_ipis_bitmap, 0);
        if bitmap == 0 {
            return alloc::vec::Vec::new();
        }

        let mut deferred = alloc::vec::Vec::with_capacity(bitmap.count_ones() as usize);
        let mut remaining = bitmap;
        while remaining != 0 {
            let cpu = remaining.trailing_zeros() as usize;
            deferred.push(cpu);
            remaining &= remaining - 1;
        }
        deferred
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_policy_uses_default_sched_class_mapping() {
        let policy = policy_for_priority(crate::task::TaskPriority::Normal);
        assert_eq!(policy.class, crate::task::TaskSchedClass::NormalTimeslice);
        assert!(policy.aging_enabled);
        assert!(policy.preempt_lower_on_wake);

        let policy = policy_for_priority(crate::task::TaskPriority::Realtime);
        assert_eq!(policy.class, crate::task::TaskSchedClass::Realtime);
        assert!(!policy.aging_enabled);
        assert!(policy.preempt_lower_on_wake);
    }

    #[test]
    fn background_and_interrupt_classes_have_distinct_policies() {
        let bg = policy_for_sched_class(crate::task::TaskSchedClass::BackgroundMaintenance);
        let irq = policy_for_sched_class(crate::task::TaskSchedClass::InterruptBottomHalf);

        assert_eq!(bg.min_priority, crate::task::TaskPriority::Idle);
        assert_eq!(bg.max_priority, crate::task::TaskPriority::Low);
        assert!(!bg.preempt_lower_on_wake);

        assert_eq!(irq.default_timeslice_ticks, 1);
        assert!(!irq.aging_enabled);
        assert!(irq.preempt_lower_on_wake);
    }
}
