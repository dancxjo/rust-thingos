use alloc::collections::VecDeque;

use super::ThreadId;

pub(crate) const RUNQ_STALE_PURGE_BUDGET: usize = 32;
pub(crate) const RUNQ_COMPACT_TRIGGER_MIN_LEN: usize = RUNQ_STALE_PURGE_BUDGET * 4;

/// Number of scheduling priority levels (Idle=0 … Realtime=4).
pub const PRIORITY_LEVELS: usize = 5;

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
