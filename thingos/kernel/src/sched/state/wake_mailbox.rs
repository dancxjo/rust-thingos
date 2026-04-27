use alloc::collections::VecDeque;
use core::sync::atomic::{AtomicBool, Ordering};

use spin::Mutex;

use super::ThreadId;

/// Entry pushed into a [`WakeMailbox`] by a remote CPU.
///
/// Carries all scheduling metadata needed to enqueue the woken task on the
/// destination CPU without additional registry lookups.
#[derive(Clone, Copy, Debug)]
pub struct WakeMailboxEntry {
    /// Identifier of the task (thread) being woken.
    ///
    /// `ThreadId` and `TaskId` are the same underlying type (`u64`); this
    /// field uses `ThreadId` to match the scheduler-layer naming convention.
    pub tid: ThreadId,
    /// Scheduling priority level (index into the run-queue priority array).
    pub priority: usize,
    /// Scheduler tick at which this wakeup was enqueued; used for aging.
    pub enqueued_at_tick: u64,
    /// Monotonic timestamp of the wakeup; used for age-histogram telemetry.
    pub wake_mono: u64,
}

/// Per-CPU cross-CPU wakeup mailbox.
///
/// Other CPUs must not directly enqueue tasks into a CPU's local run queue.
/// Instead they call [`WakeMailbox::push`] to deposit a [`WakeMailboxEntry`];
/// the owning CPU drains the mailbox at safe scheduling points
/// (start of `schedule()`, timer tick, exit from idle) and inserts the tasks
/// into its own run queue.
///
/// # Synchronization
///
/// `WakeMailbox` uses a [`spin::Mutex`] around the internal [`VecDeque`] to
/// protect concurrent push operations from multiple remote CPUs.  The
/// `pending` flag is an [`AtomicBool`] that allows the drain fast-path to skip
/// the lock entirely when no entries are present.
///
/// # Usage
///
/// ```text
/// // remote CPU:
/// cpu_n.wake_mailbox.push(WakeMailboxEntry { tid, priority, ... });
///
/// // owning CPU, at a safe scheduling point:
/// for entry in cpu_n.wake_mailbox.drain() {
///     cpu_n.runq.enqueue(entry.priority, entry.tid);
/// }
/// ```
pub struct WakeMailbox {
    inner: Mutex<VecDeque<WakeMailboxEntry>>,
    pending: AtomicBool,
}

impl WakeMailbox {
    /// Create a new, empty mailbox.
    ///
    /// This is a `const fn` so it can be used in static initializers.
    pub const fn new() -> Self {
        WakeMailbox { inner: Mutex::new(VecDeque::new()), pending: AtomicBool::new(false) }
    }

    /// Push a [`WakeMailboxEntry`] from a remote CPU.
    ///
    /// Safe to call from any CPU without holding the scheduler lock.  The
    /// internal [`spin::Mutex`] protects concurrent pushes from multiple
    /// remote CPUs.
    pub fn push(&self, entry: WakeMailboxEntry) {
        self.inner.lock().push_back(entry);
        self.pending.store(true, Ordering::Release);
    }

    /// Drain all pending entries, returning them as a [`VecDeque`].
    ///
    /// Returns an empty collection immediately when no entries are present
    /// (the `pending` fast-path avoids the lock).  Intended to be called
    /// **only** by the CPU that owns this mailbox at a safe scheduling point.
    pub fn drain(&self) -> VecDeque<WakeMailboxEntry> {
        if !self.pending.swap(false, Ordering::AcqRel) {
            return VecDeque::new();
        }
        core::mem::take(&mut *self.inner.lock())
    }

    /// Return `true` if there is at least one undelivered entry.
    ///
    /// This is a non-consuming hint; the owning CPU can use it to quickly
    /// check for pending cross-CPU wakeups before committing to a full drain.
    #[inline]
    pub fn is_pending(&self) -> bool {
        self.pending.load(Ordering::Acquire)
    }
}
