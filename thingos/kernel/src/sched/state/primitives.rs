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
    pub const COUNT: usize = 5;

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
        if cpu < 64 { CpuSet(1u64 << cpu) } else { CpuSet(0) }
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
    pub fn pick(
        self,
        preferred: Option<CpuId>,
        last_cpu: Option<CpuId>,
        cpu_count: usize,
    ) -> Option<CpuId> {
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
        if allowed { Ok(next) } else { Err(self) }
    }
}

pub const WAKE_LATENCY_HIST_BUCKETS: usize = 5;
pub const IDLE_EPISODE_HIST_BUCKETS: usize = 4;
