//! Generic WaitQueue for task synchronization
//!
//! Provides FIFO waking to avoid thundering herd issues and ensure fairness.

use alloc::collections::{BTreeSet, VecDeque};
use alloc::vec::Vec;
use spin::Mutex;

pub struct WaitQueue {
    inner: Mutex<WaitQueueInner>,
}

struct WaitQueueInner {
    /// FIFO registration order for wake fairness.
    waiters: VecDeque<u64>,
    /// Membership index used for O(log n) insert/remove and dedupe checks.
    in_queue: BTreeSet<u64>,
}

impl WaitQueue {
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(WaitQueueInner {
                waiters: VecDeque::new(),
                in_queue: BTreeSet::new(),
            }),
        }
    }

    /// Returns true if there are no waiters.
    pub fn is_empty(&self) -> bool {
        self.inner.lock().in_queue.is_empty()
    }

    /// Add a task to the wait queue
    pub fn push_back(&self, tid: u64) {
        let mut inner = self.inner.lock();
        if inner.in_queue.insert(tid) {
            crate::kdebug!("WaitQueue::push_back: adding task {}", tid);
            inner.waiters.push_back(tid);
        } else {
            crate::kdebug!("WaitQueue::push_back: task {} already in queue", tid);
        }
    }

    /// Wake the first task in the queue
    pub fn wake_one(&self) {
        let tid = {
            let mut inner = self.inner.lock();
            let mut chosen = None;
            while let Some(candidate) = inner.waiters.pop_front() {
                if inner.in_queue.remove(&candidate) {
                    chosen = Some(candidate);
                    break;
                }
            }
            chosen
        };

        if let Some(tid) = tid {
            crate::kdebug!("WaitQueue::wake_one: waking task {}", tid);
            unsafe {
                crate::sched::wake_task_erased(tid);
            }
        }
    }

    /// Wake all tasks in the queue
    pub fn wake_all(&self) {
        let waiters = {
            let mut inner = self.inner.lock();
            let mut to_wake = Vec::new();
            while let Some(candidate) = inner.waiters.pop_front() {
                if inner.in_queue.remove(&candidate) {
                    to_wake.push(candidate);
                }
            }
            to_wake
        };

        for tid in waiters {
            unsafe {
                crate::kdebug!("WaitQueue::wake_all: waking task {}", tid);
                crate::sched::wake_task_erased(tid);
            }
        }
    }

    /// Remove a task from the wait queue (e.g. on timeout or interrupt)
    pub fn remove(&self, tid: u64) {
        self.inner.lock().in_queue.remove(&tid);
    }

    /// Drain the queue without waking. Caller decides when waking is safe.
    pub fn drain(&self) -> Vec<u64> {
        let mut inner = self.inner.lock();
        let mut drained = Vec::new();
        while let Some(candidate) = inner.waiters.pop_front() {
            if inner.in_queue.remove(&candidate) {
                drained.push(candidate);
            }
        }
        drained
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sched::blocking::WAKE_TASK_HOOK;
    use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

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
        (0..len)
            .map(|idx| WOKEN_IDS[idx].load(Ordering::SeqCst))
            .collect()
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
