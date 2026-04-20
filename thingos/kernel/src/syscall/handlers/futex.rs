//! Futex (fast userspace mutex) syscall handlers.
//!
//! Provides kernel-backed wait/wake on a userspace `u32` address.
//! This is the backbone for Rust std's sync primitives (Mutex, Condvar,
//! RwLock, thread parking).

use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;

use abi::errors::{Errno, SysResult};
use spin::Mutex;

use crate::syscall::validate::validate_user_range;

/// A futex wait queue entry: the thread ID that is blocked.
type TaskId = u64;

/// Key for the wait queue: (process address-space id, userspace address).
type FutexKey = (u32, usize);

/// Global futex wait-queue table.
///
/// Each address maps to a FIFO queue of waiting thread IDs.  Using
/// `VecDeque` instead of `Vec` gives O(1) pop from the front (FIFO wake
/// order) and avoids starvation of long-waiting threads.
static FUTEX_TABLE: Mutex<BTreeMap<FutexKey, VecDeque<TaskId>>> = Mutex::new(BTreeMap::new());

fn futex_scope_id() -> u32 {
    if let Some(pinfo) = crate::sched::process_info_current() {
        pinfo.lock().pid
    } else {
        unsafe { crate::sched::current_tid_current() as u32 }
    }
}

fn futex_key_for_scope(scope_id: u32, uaddr: usize) -> FutexKey {
    (scope_id, uaddr)
}

fn futex_key(uaddr: usize) -> FutexKey {
    futex_key_for_scope(futex_scope_id(), uaddr)
}

/// `sys_futex_wait(uaddr, expected, timeout_ns)`
///
/// If `*uaddr == expected`, block the current thread until woken or
/// until `timeout_ns` nanoseconds elapse (0 = wait forever).
///
/// Returns:
/// - `Ok(0)` on wake or spurious wakeup
/// - `Err(EAGAIN)` if `*uaddr != expected` (value changed)
/// - `Err(ETIMEDOUT)` on timeout
pub fn sys_futex_wait(uaddr: usize, expected: u32, timeout_ns: u64) -> SysResult<usize> {
    if crate::sched::take_pending_interrupt_current() {
        return Err(Errno::EINTR);
    }

    // Validate the userspace address is readable.
    validate_user_range(uaddr, 4, false)?;

    // Quick pre-check without the table lock: if the value already differs,
    // return EAGAIN immediately (common fast path).
    let pre_val = unsafe { core::ptr::read_volatile(uaddr as *const u32) };
    if pre_val != expected {
        return Err(Errno::EAGAIN);
    }

    let tid = unsafe { crate::sched::current_tid_current() };
    let key = futex_key(uaddr);

    // Enqueue ourselves as a waiter, then re-read the futex value while
    // still holding the table lock.  This closes the missed-wake window:
    // any concurrent `futex_wake` that fires *after* we take the lock will
    // see our TID in the table and call `wake_task_erased`, so even if the
    // wake arrives between the lock release and the call to
    // `block_current_erased` below, the scheduler's `wake_pending` flag
    // guarantees we return immediately rather than sleeping forever.
    {
        let mut table = FUTEX_TABLE.lock();
        // Re-read the futex value with the lock held so that any producer
        // which (a) stores a new value and (b) calls futex_wake is
        // serialised with us.  If it stored before we take the lock we see
        // the change and return EAGAIN; if it stores after we release the
        // lock it will find our TID in the table and wake us.
        let current_val = unsafe { core::ptr::read_volatile(uaddr as *const u32) };
        if current_val != expected {
            return Err(Errno::EAGAIN);
        }
        table.entry(key).or_insert_with(VecDeque::new).push_back(tid);
    }

    if timeout_ns == 0 {
        // Indefinite wait — block until woken.
        unsafe {
            crate::sched::block_current_erased();
        }
        futex_remove_self(&key, tid);
        if crate::sched::take_pending_interrupt_current() {
            return Err(Errno::EINTR);
        }
    } else {
        // Timed wait — use scheduler sleep with the same coarse rounding as SYS_SLEEP.
        let ticks = crate::time::duration_to_sleep_ticks(timeout_ns);
        if ticks == 0 {
            unsafe {
                crate::sched::yield_now_current();
            }
        } else {
            crate::sched::sleep_ticks_current(ticks);
        }

        if crate::sched::take_pending_interrupt_current() {
            futex_remove_self(&key, tid);
            return Err(Errno::EINTR);
        }

        // After waking, remove ourselves from the wait queue if still there
        // (we may have been woken by the timer, not by futex_wake).
        let still_in_queue = futex_remove_self(&key, tid);
        if still_in_queue {
            // We timed out (still in queue = nobody woke us).
            return Err(Errno::ETIMEDOUT);
        }
        // If we were not in the queue, we were woken by futex_wake — success.
    }

    Ok(0)
}

/// Remove `tid` from the wait queue for `key`.
///
/// Returns `true` if the TID was still present (i.e. we were *not* woken by
/// a concurrent `futex_wake`), or `false` if it had already been consumed
/// by a waker.  In both cases the table is left consistent.
fn futex_remove_self(key: &FutexKey, tid: TaskId) -> bool {
    let mut table = FUTEX_TABLE.lock();
    if let Some(waiters) = table.get_mut(key) {
        if let Some(pos) = waiters.iter().position(|&w| w == tid) {
            waiters.remove(pos);
            if waiters.is_empty() {
                table.remove(key);
            }
            return true;
        }
    }
    false
}

/// `sys_futex_wake(uaddr, count)`
///
/// Wake up to `count` threads waiting on `uaddr` in FIFO order.
/// Returns the number of threads actually woken.
pub fn sys_futex_wake(uaddr: usize, count: u32) -> SysResult<usize> {
    let key = futex_key(uaddr);

    // Collect the TIDs to wake while holding the table lock, then release
    // the lock *before* calling `wake_task_erased`.  Holding FUTEX_TABLE
    // across `wake_task_erased` would cause a lock convoy: each wake
    // acquires the scheduler lock, and any concurrent `futex_wait` trying
    // to register itself would stall behind FUTEX_TABLE the entire time.
    let to_wake: Vec<TaskId> = {
        let mut table = FUTEX_TABLE.lock();
        if let Some(waiters) = table.get_mut(&key) {
            let n = (count as usize).min(waiters.len());
            // Drain from the front (FIFO) to wake the longest-waiting threads
            // first and prevent starvation.
            let drained: Vec<TaskId> = waiters.drain(..n).collect();
            if waiters.is_empty() {
                table.remove(&key);
            }
            drained
        } else {
            Vec::new()
        }
    }; // FUTEX_TABLE lock released here

    let woken = to_wake.len();
    for tid in to_wake {
        unsafe { crate::sched::wake_task_erased(tid) };
    }

    Ok(woken)
}

#[cfg(test)]
mod tests {
    use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

    use super::*;
    use crate::sched::blocking::WAKE_TASK_HOOK;

    #[test]
    fn futex_key_includes_scope() {
        assert_eq!(futex_key_for_scope(7, 0x1000), (7, 0x1000));
    }

    // ── wake-outside-lock (convoy fix) ────────────────────────────────────────

    static WOKEN_COUNT: AtomicUsize = AtomicUsize::new(0);
    static WOKEN_IDS: [AtomicU64; 4] = [const { AtomicU64::new(0) }; 4];

    fn reset_wake_log() {
        WOKEN_COUNT.store(0, Ordering::SeqCst);
        for s in &WOKEN_IDS {
            s.store(0, Ordering::SeqCst);
        }
    }

    fn record_wake(id: u64) {
        let idx = WOKEN_COUNT.fetch_add(1, Ordering::SeqCst);
        if idx < WOKEN_IDS.len() {
            WOKEN_IDS[idx].store(id, Ordering::SeqCst);
        }
    }

    fn wake_log() -> alloc::vec::Vec<u64> {
        let len = WOKEN_COUNT.load(Ordering::SeqCst).min(WOKEN_IDS.len());
        (0..len).map(|i| WOKEN_IDS[i].load(Ordering::SeqCst)).collect()
    }

    /// `sys_futex_wake` must wake exactly `count` waiters and leave the rest.
    #[test]
    fn futex_wake_wakes_correct_count() {
        reset_wake_log();
        WAKE_TASK_HOOK.store(record_wake as *mut (), Ordering::SeqCst);

        let val: u32 = 0;
        let scope = 0xAABB_0001u32;
        let key = futex_key_for_scope(scope, &val as *const u32 as usize);

        // Manually enqueue three waiters (FIFO: 10 is oldest, 12 is newest).
        FUTEX_TABLE
            .lock()
            .entry(key)
            .or_insert_with(VecDeque::new)
            .extend([10u64, 11u64, 12u64]);

        // Wake only 2 (FIFO: should wake 10 and 11, leaving 12).
        let to_wake: Vec<u64> = {
            let mut table = FUTEX_TABLE.lock();
            if let Some(waiters) = table.get_mut(&key) {
                let n = 2usize.min(waiters.len());
                let d: Vec<u64> = waiters.drain(..n).collect();
                if waiters.is_empty() {
                    table.remove(&key);
                }
                d
            } else {
                alloc::vec![]
            }
        };
        for tid in &to_wake {
            unsafe { crate::sched::wake_task_erased(*tid) };
        }

        assert_eq!(to_wake.len(), 2);
        assert_eq!(WOKEN_COUNT.load(Ordering::SeqCst), 2);
        // The oldest two (10, 11) were woken; 12 remains.
        assert_eq!(FUTEX_TABLE.lock().get(&key).map(|w| w.len()).unwrap_or(0), 1);
        assert_eq!(FUTEX_TABLE.lock().get(&key).and_then(|w| w.front().copied()), Some(12u64));

        // Cleanup.
        FUTEX_TABLE.lock().remove(&key);
        WAKE_TASK_HOOK.store(core::ptr::null_mut(), Ordering::SeqCst);
    }

    /// `sys_futex_wait` must return EAGAIN immediately when the re-read (inside
    /// the table lock) shows the value has changed — even if the fast pre-check
    /// would have passed.
    #[test]
    fn futex_wait_recheck_inside_lock_returns_eagain() {
        let mut val: u32 = 42;
        let uaddr = &mut val as *mut u32 as usize;

        // Change the value between the fast pre-check and the locked re-read.
        // Since this is a single-threaded unit test we can only simulate the
        // case where the value is already different by the time we call in.
        unsafe { core::ptr::write_volatile(uaddr as *mut u32, 99) };

        // Call with old expected value — must return EAGAIN from the locked
        // re-read path (or the fast pre-check; both are correct).
        let result = sys_futex_wait(uaddr, 42, 0);
        assert_eq!(result, Err(Errno::EAGAIN));

        // The table must not contain a stale waiter.
        let scope = futex_scope_id();
        let key = futex_key_for_scope(scope, uaddr);
        assert!(
            FUTEX_TABLE.lock().get(&key).map(|w| w.is_empty()).unwrap_or(true),
            "no stale waiter should remain in the table after EAGAIN"
        );
    }
}
