//! Generic WaitQueue for task synchronization
//!
//! Provides FIFO waking to avoid thundering herd issues and ensure fairness.
//!
//! # Thread safety
//!
//! [`WaitQueue`] is fully thread-safe.  All mutations are protected by an
//! internal `spin::Mutex` and wakeups (`wake_task_erased`) are issued **after**
//! that lock is released so a woken thread can re-acquire the lock immediately.
//! Duplicate registrations via [`WaitQueue::push_back`] are silently
//! deduplicated; racing callers cannot corrupt the queue.
//!
//! See `docs/kernel/threading-readiness.md` for a broader discussion of how
//! this primitive supports expanded userspace multithreading.

use alloc::collections::{BTreeSet, VecDeque};
use alloc::vec::Vec;

use spin::Mutex;

#[derive(Debug)]
pub struct WaitQueue {
    inner: Mutex<WaitQueueInner>,
}

#[derive(Debug)]
struct WaitQueueInner {
    /// FIFO registration order for wake fairness.
    waiters: VecDeque<u64>,
    /// Membership index used for O(log n) insert/remove and dedupe checks.
    in_queue: Option<BTreeSet<u64>>,
}

impl WaitQueueInner {
    fn in_queue_mut(&mut self) -> &mut BTreeSet<u64> {
        self.in_queue.get_or_insert_with(BTreeSet::new)
    }

    fn prune_if_empty(&mut self) {
        if self.in_queue.as_ref().is_some_and(BTreeSet::is_empty) {
            self.waiters.clear();
            self.in_queue = None;
        }
    }
}

impl WaitQueue {
    pub const fn new() -> Self {
        Self { inner: Mutex::new(WaitQueueInner { waiters: VecDeque::new(), in_queue: None }) }
    }

    /// Returns true if there are no waiters.
    pub fn is_empty(&self) -> bool {
        let inner = self.inner.lock();
        inner.in_queue.as_ref().map_or_else(|| inner.waiters.is_empty(), BTreeSet::is_empty)
    }

    /// Add a task to the wait queue
    pub fn push_back(&self, tid: u64) {
        let mut inner = self.inner.lock();
        let in_queue = inner.in_queue_mut();
        if in_queue.insert(tid) {
            inner.waiters.push_back(tid);
        }
    }

    /// Wake the first task in the queue
    pub fn wake_one(&self) {
        let tid = {
            let mut inner = self.inner.lock();
            if inner.in_queue.is_none() {
                return;
            }
            let mut chosen = None;
            while let Some(candidate) = inner.waiters.pop_front() {
                let removed =
                    inner.in_queue.as_mut().is_some_and(|in_queue| in_queue.remove(&candidate));
                if removed {
                    chosen = Some(candidate);
                    break;
                }
            }
            inner.prune_if_empty();
            chosen
        };

        if let Some(tid) = tid {
            unsafe {
                crate::sched::wake_task_erased(tid);
            }
        }
    }

    /// Wake all tasks in the queue
    pub fn wake_all(&self) {
        let waiters = {
            let mut inner = self.inner.lock();
            if inner.in_queue.is_none() {
                return;
            }
            let mut to_wake = Vec::new();
            while let Some(candidate) = inner.waiters.pop_front() {
                let removed =
                    inner.in_queue.as_mut().is_some_and(|in_queue| in_queue.remove(&candidate));
                if removed {
                    to_wake.push(candidate);
                }
            }
            inner.prune_if_empty();
            to_wake
        };

        for tid in waiters {
            unsafe {
                crate::sched::wake_task_erased(tid);
            }
        }
    }

    /// Remove a task from the wait queue (e.g. on timeout or interrupt)
    pub fn remove(&self, tid: u64) {
        let mut inner = self.inner.lock();
        if let Some(in_queue) = inner.in_queue.as_mut() {
            // Removal is index-only; stale FIFO entries are lazily skipped on wake/drain.
            in_queue.remove(&tid);
            inner.prune_if_empty();
        }
    }

    /// Drain the queue without waking. Caller decides when waking is safe.
    pub fn drain(&self) -> Vec<u64> {
        let mut inner = self.inner.lock();
        if inner.in_queue.is_none() {
            return Vec::new();
        }
        let mut drained = Vec::new();
        while let Some(candidate) = inner.waiters.pop_front() {
            let removed =
                inner.in_queue.as_mut().is_some_and(|in_queue| in_queue.remove(&candidate));
            if removed {
                drained.push(candidate);
            }
        }
        inner.prune_if_empty();
        drained
    }
}

#[cfg(test)]
mod tests {
    use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

    use super::*;
    use crate::sched::blocking::WAKE_TASK_HOOK;

    static WOKEN_IDS: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];
    static WOKEN_LEN: AtomicUsize = AtomicUsize::new(0);

    fn reset_wakes() {
        WOKEN_LEN.store(0, Ordering::SeqCst);
        for slot in &WOKEN_IDS {
            slot.store(0, Ordering::SeqCst);
        }
    }

    fn record_wake(id: u64) {
        let idx = WOKEN_LEN.fetch_add(1, Ordering::SeqCst);
        if idx < WOKEN_IDS.len() {
            WOKEN_IDS[idx].store(id, Ordering::SeqCst);
        }
    }

    fn wake_log() -> alloc::vec::Vec<u64> {
        let len = WOKEN_LEN.load(Ordering::SeqCst).min(WOKEN_IDS.len());
        (0..len).map(|idx| WOKEN_IDS[idx].load(Ordering::SeqCst)).collect()
    }

    #[test]
    fn push_back_deduplicates_waiters() {
        let q = WaitQueue::new();

        q.push_back(10);
        q.push_back(11);
        q.push_back(10);

        assert_eq!(q.drain(), alloc::vec![10, 11]);
    }

    #[test]
    fn wake_one_is_fifo_and_only_pops_one_waiter() {
        let q = WaitQueue::new();
        reset_wakes();
        WAKE_TASK_HOOK.store(record_wake as *mut (), Ordering::SeqCst);

        q.push_back(21);
        q.push_back(22);
        q.wake_one();

        assert_eq!(wake_log(), alloc::vec![21]);
        assert_eq!(q.drain(), alloc::vec![22]);
    }

    #[test]
    fn wake_all_drains_in_fifo_order() {
        let q = WaitQueue::new();
        reset_wakes();
        WAKE_TASK_HOOK.store(record_wake as *mut (), Ordering::SeqCst);

        q.push_back(31);
        q.push_back(32);
        q.push_back(33);
        q.wake_all();

        assert_eq!(wake_log(), alloc::vec![31, 32, 33]);
        assert!(q.drain().is_empty());
    }

    #[test]
    fn remove_erases_specific_waiter_without_disturbing_order() {
        let q = WaitQueue::new();

        q.push_back(41);
        q.push_back(42);
        q.push_back(43);
        q.remove(42);

        assert_eq!(q.drain(), alloc::vec![41, 43]);
    }

    #[test]
    fn wake_one_skips_removed_waiters() {
        let q = WaitQueue::new();
        reset_wakes();
        WAKE_TASK_HOOK.store(record_wake as *mut (), Ordering::SeqCst);

        q.push_back(51);
        q.push_back(52);
        q.push_back(53);
        q.remove(52);
        q.wake_one();
        q.wake_one();

        assert_eq!(wake_log(), alloc::vec![51, 53]);
        assert!(q.is_empty());
    }
}
