//! Kernel-side advisory file locking (flock semantics).
//!
//! This module implements a simple in-kernel advisory lock table.  Locks are
//! tracked per **inode number**, with per-process holder sets, mirroring POSIX
//! `flock(2)` semantics where each process holds at most one advisory lock per
//! open file.
//!
//! Supported operations (the `how` argument to [`flock`]):
//!
//! * `LOCK_SH` — acquire a shared (read) lock
//! * `LOCK_EX` — acquire an exclusive (write) lock
//! * `LOCK_NB` — non-blocking flag (can be OR-ed with `LOCK_SH`/`LOCK_EX`)
//! * `LOCK_UN` — release the lock
//!
//! # Blocking semantics
//!
//! When a blocking request (`LOCK_SH` or `LOCK_EX` without `LOCK_NB`) cannot
//! be satisfied immediately, the calling thread is put to sleep (via the
//! scheduler's blocking primitives) and woken once the conflicting lock is
//! released.  The acquisition is then retried automatically.
//!
//! When `LOCK_NB` is set, `EAGAIN` is returned immediately instead.
//!
//! * Lock upgrade (shared → exclusive while no other holder) is detected and
//!   allowed when the calling process is the sole holder.
//!
//! # Design notes
//!
//! The previous design used two separate global tables: a `(ino, pid) →
//! LockType` table and a separate `ino → Vec<TID>` wait-queue table.  This
//! required O(n) full-table scans for conflict checks and O(n) waiter-list
//! operations, and wakeups were performed while holding the global table lock
//! (lock-convoy risk).
//!
//! The new design uses a **single** `ino → InodeLock` table where each entry
//! carries:
//! * A `BTreeSet<u32>` of shared-lock holders (O(log n) insert/remove/check).
//! * An `Option<u32>` exclusive-lock holder.
//! * An `Arc<WaitQueue>` for blocked waiters — shared with the waiter so that
//!   `release` can wake via the Arc *after* releasing the table lock, eliminating
//!   the lock-convoy path.

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::sync::Arc;

use abi::errors::{Errno, SysResult};
use abi::syscall::flock_flags::{LOCK_EX, LOCK_NB, LOCK_SH, LOCK_UN};
use spin::Mutex;

use crate::sched::wait_queue::WaitQueue;

/// Per-inode advisory lock state.
struct InodeLock {
    /// PIDs holding shared locks (at most one lock per PID).
    shared_holders: BTreeSet<u32>,
    /// PID holding the exclusive lock, or `None`.
    exclusive_holder: Option<u32>,
    /// Wait queue for tasks blocked on this inode's lock.
    ///
    /// Stored as `Arc` so that `release` can clone the Arc, remove the entry
    /// from the table (if unlocked), release the table lock, and then call
    /// `wake_all` — all without holding the table lock during wakeups.
    waitq: Arc<WaitQueue>,
}

/// Global advisory lock table: inode → per-inode lock state.
static FLOCK_TABLE: Mutex<BTreeMap<u64, InodeLock>> = Mutex::new(BTreeMap::new());

/// Apply a `flock(2)`-style lock operation.
///
/// * `ino` — inode number identifying the file
/// * `pid` — caller's process ID
/// * `how` — combination of [`abi::syscall::flock_flags`] values
///
/// # Errors
/// * [`Errno::EAGAIN`] (`EWOULDBLOCK`) — lock is held by another process and
///   `LOCK_NB` was specified.
/// * [`Errno::EINVAL`] — `how` does not contain a valid lock operation.
/// * [`Errno::EINTR`] — a signal was received while blocked.
pub fn flock(ino: u64, pid: u32, how: u32) -> SysResult<()> {
    if how & LOCK_UN != 0 {
        release(ino, pid);
        return Ok(());
    }

    let non_blocking = how & LOCK_NB != 0;

    loop {
        if crate::sched::take_pending_interrupt_current() {
            return Err(Errno::EINTR);
        }

        // Check for conflict and either acquire (success) or register as a
        // waiter (blocking path) — all under a single table lock acquisition.
        let wait_for = {
            let mut table = FLOCK_TABLE.lock();
            let entry = table.entry(ino).or_insert_with(|| InodeLock {
                shared_holders: BTreeSet::new(),
                exclusive_holder: None,
                waitq: Arc::new(WaitQueue::new()),
            });

            if how & LOCK_SH != 0 {
                // Conflict: another process holds an exclusive lock.
                let contended =
                    entry.exclusive_holder.is_some_and(|holder| holder != pid);
                if !contended {
                    // Acquire shared lock (may replace a prior EX lock held by
                    // same pid, i.e. downgrade).
                    if entry.exclusive_holder == Some(pid) {
                        entry.exclusive_holder = None;
                    }
                    entry.shared_holders.insert(pid);
                    return Ok(());
                }
            } else if how & LOCK_EX != 0 {
                // Conflict: any lock held by another process.
                let other_shared = entry.shared_holders.iter().any(|&p| p != pid);
                let other_exclusive =
                    entry.exclusive_holder.is_some_and(|holder| holder != pid);
                let contended = other_shared || other_exclusive;
                if !contended {
                    // Acquire exclusive lock (may replace a prior SH lock held
                    // by same pid, i.e. upgrade).
                    entry.shared_holders.remove(&pid);
                    entry.exclusive_holder = Some(pid);
                    return Ok(());
                }
            } else {
                return Err(Errno::EINVAL);
            }

            // Contended.
            if non_blocking {
                return Err(Errno::EAGAIN);
            }

            // Register as a waiter *inside* the table lock so that a
            // concurrent `release` that fires immediately after we drop the
            // table lock will find our TID in the wait queue and set
            // `wake_pending`, ensuring `block_current_erased` returns at once.
            let tid = unsafe { crate::sched::current_tid_current() };
            entry.waitq.push_back(tid as u64);
            entry.waitq.clone() // clone Arc so we can call remove() after blocking
        }; // table lock released

        unsafe { crate::sched::block_current_erased() };

        // Clean up wait-queue registration regardless of how we were woken.
        let tid = unsafe { crate::sched::current_tid_current() };
        wait_for.remove(tid as u64);

        if crate::sched::take_pending_interrupt_current() {
            return Err(Errno::EINTR);
        }
        // Retry the acquisition.
    }
}

/// Release any advisory lock held by `pid` on `ino` and wake blocked waiters.
///
/// This is called automatically by [`crate::syscall::handlers::vfs::sys_fs_close`]
/// when a file descriptor is closed, ensuring that stale locks are never left
/// in the table.
pub fn release(ino: u64, pid: u32) {
    // Remove the holder entries and, if the inode is now fully unlocked, prune
    // the table entry.  Clone the wait-queue Arc so we can call `wake_all`
    // *after* releasing the table lock, avoiding the scheduler-lock convoy.
    let waitq_opt = {
        let mut table = FLOCK_TABLE.lock();
        if let Some(entry) = table.get_mut(&ino) {
            entry.shared_holders.remove(&pid);
            if entry.exclusive_holder == Some(pid) {
                entry.exclusive_holder = None;
            }
            let waitq = entry.waitq.clone();
            // Remove the table entry when it is fully unlocked so the table
            // does not accumulate empty entries forever.  Waiters that were
            // registered (and are now being woken) hold their own Arc clone
            // and will simply retry and create a fresh entry if still needed.
            if entry.shared_holders.is_empty() && entry.exclusive_holder.is_none() {
                table.remove(&ino);
            }
            Some(waitq)
        } else {
            None
        }
    }; // table lock released

    // Wake every thread that was sleeping on this inode so they can retry.
    if let Some(waitq) = waitq_opt {
        waitq.wake_all(); // outside FLOCK_TABLE lock ✓
    }
}

#[cfg(test)]
mod tests {
    use abi::errors::Errno;
    use abi::syscall::flock_flags::{LOCK_EX, LOCK_NB, LOCK_SH, LOCK_UN};

    use super::*;

    /// Remove any existing state for the given (ino, pid) pair so tests do
    /// not interfere with each other.
    fn cleanup(ino: u64, pid: u32) {
        let mut table = FLOCK_TABLE.lock();
        if let Some(entry) = table.get_mut(&ino) {
            entry.shared_holders.remove(&pid);
            if entry.exclusive_holder == Some(pid) {
                entry.exclusive_holder = None;
            }
        }
        // Remove empty entry to avoid interfering with other tests.
        if let Some(entry) = table.get(&ino) {
            if entry.shared_holders.is_empty() && entry.exclusive_holder.is_none() {
                table.remove(&ino);
            }
        }
    }

    #[test]
    fn shared_lock_succeeds_when_no_lock_held() {
        let (ino, pid) = (0xF001, 1);
        cleanup(ino, pid);
        assert_eq!(flock(ino, pid, LOCK_SH), Ok(()));
        release(ino, pid);
    }

    #[test]
    fn multiple_processes_can_hold_shared_locks() {
        let ino = 0xF002;
        cleanup(ino, 10);
        cleanup(ino, 11);
        assert_eq!(flock(ino, 10, LOCK_SH), Ok(()));
        assert_eq!(flock(ino, 11, LOCK_SH), Ok(()));
        release(ino, 10);
        release(ino, 11);
    }

    #[test]
    fn exclusive_lock_succeeds_when_no_lock_held() {
        let (ino, pid) = (0xF003, 2);
        cleanup(ino, pid);
        assert_eq!(flock(ino, pid, LOCK_EX), Ok(()));
        release(ino, pid);
    }

    #[test]
    fn exclusive_lock_fails_when_another_process_holds_shared() {
        let ino = 0xF004;
        cleanup(ino, 20);
        cleanup(ino, 21);
        flock(ino, 20, LOCK_SH).unwrap();
        assert_eq!(flock(ino, 21, LOCK_EX | LOCK_NB), Err(Errno::EAGAIN));
        release(ino, 20);
    }

    #[test]
    fn shared_lock_fails_when_another_process_holds_exclusive() {
        let ino = 0xF005;
        cleanup(ino, 30);
        cleanup(ino, 31);
        flock(ino, 30, LOCK_EX).unwrap();
        assert_eq!(flock(ino, 31, LOCK_SH | LOCK_NB), Err(Errno::EAGAIN));
        release(ino, 30);
    }

    #[test]
    fn unlock_when_no_lock_held_is_noop() {
        let (ino, pid) = (0xF006, 3);
        cleanup(ino, pid);
        assert_eq!(flock(ino, pid, LOCK_UN), Ok(()));
    }

    #[test]
    fn exclusive_lock_fails_when_another_process_holds_exclusive() {
        let ino = 0xF007;
        cleanup(ino, 40);
        cleanup(ino, 41);
        flock(ino, 40, LOCK_EX).unwrap();
        assert_eq!(flock(ino, 41, LOCK_EX | LOCK_NB), Err(Errno::EAGAIN));
        release(ino, 40);
    }

    #[test]
    fn release_removes_entry_from_table() {
        let (ino, pid) = (0xF008, 4);
        cleanup(ino, pid);
        flock(ino, pid, LOCK_EX).unwrap();
        release(ino, pid);
        // Entry must be gone (no holders, no waiters).
        assert!(
            FLOCK_TABLE
                .lock()
                .get(&ino)
                .map(|e| e.shared_holders.is_empty() && e.exclusive_holder.is_none())
                .unwrap_or(true),
            "entry should be removed or empty after release"
        );
    }

    #[test]
    fn invalid_how_returns_einval() {
        let (ino, pid) = (0xF009, 5);
        cleanup(ino, pid);
        assert_eq!(flock(ino, pid, 0), Err(Errno::EINVAL));
    }

    #[test]
    fn same_process_can_upgrade_shared_to_exclusive() {
        let (ino, pid) = (0xF00A, 6);
        cleanup(ino, pid);
        flock(ino, pid, LOCK_SH).unwrap();
        // Upgrade: the caller's own shared lock should not block exclusivity.
        assert_eq!(flock(ino, pid, LOCK_EX), Ok(()));
        release(ino, pid);
    }

    /// Verify that releasing a lock wakes any registered waiters.
    #[test]
    fn release_wakes_and_clears_wait_queue() {
        use crate::sched::blocking::WAKE_TASK_HOOK;
        use core::sync::atomic::{AtomicUsize, Ordering as AOrdering};

        static WOKEN: AtomicUsize = AtomicUsize::new(0);
        fn record_wake(_id: u64) {
            WOKEN.fetch_add(1, AOrdering::SeqCst);
        }

        WOKEN.store(0, AOrdering::SeqCst);
        WAKE_TASK_HOOK.store(record_wake as *mut (), AOrdering::SeqCst);

        let ino = 0xF00B;
        cleanup(ino, 50);

        // Manually insert a fake waiter TID to simulate a blocked thread.
        {
            let mut table = FLOCK_TABLE.lock();
            let entry = table.entry(ino).or_insert_with(|| InodeLock {
                shared_holders: BTreeSet::new(),
                exclusive_holder: None,
                waitq: Arc::new(WaitQueue::new()),
            });
            entry.waitq.push_back(9999);
        }

        // Acquire a lock so release has something to remove.
        flock(ino, 50, LOCK_EX).unwrap();

        // Release should drain the wait queue (wake_task_erased calls our hook).
        release(ino, 50);

        assert_eq!(WOKEN.load(AOrdering::SeqCst), 1, "waiter 9999 should be woken by release");
        // Table entry must be gone.
        assert!(
            FLOCK_TABLE.lock().get(&ino).is_none(),
            "table entry should be removed after release"
        );

        WAKE_TASK_HOOK.store(core::ptr::null_mut(), AOrdering::SeqCst);
    }
}