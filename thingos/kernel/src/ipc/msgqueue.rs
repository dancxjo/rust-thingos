//! `KernelMessageQueue`: shared bounded FIFO queue substrate for IPC primitives.
//!
//! # Purpose
//!
//! This module provides the [`KernelMessageQueue<M>`] type — a bounded,
//! closable FIFO queue with an integrated [`WaitQueue`] — as the **shared
//! internal substrate** that both the `Inbox` (ownership-first) and the
//! `Port` structured-message path (connection-first) converge on.
//!
//! It is the "Option C: Shared core with two views" prototype described in
//! `docs/ipc/convergence_strategy.md`:
//!
//! ```text
//! KernelMessageQueue<M>
//!  ├── InboxView  — ownership-first, single consumer, typed delivery
//!  └── PortMsgView — connection-first, endpoint-capability framing
//! ```
//!
//! # Prototype status
//!
//! This module validates the feasibility of the layered convergence model.
//! Full migration of `Inbox` and `Port` to share this core follows Phase B/C
//! of the convergence roadmap.
//!
//! # Design
//!
//! - **Generic** over message type `M` so both `MessageEnvelope` (inbox) and
//!   `KernelMessage` (port structured-message path) can be stored.
//! - **Bounded** capacity; [`KernelMessageQueue::enqueue`] returns
//!   [`MqSendError::Full`] when at capacity instead of silently dropping.
//! - **Closable**; after close, enqueues return [`MqSendError::Closed`] and
//!   receivers drain remaining messages before receiving [`MqRecvError::Closed`].
//! - **Wake-on-send**; a successful [`KernelMessageQueue::enqueue`] wakes one
//!   task registered on [`KernelMessageQueue::waiters`].
//! - **Readiness queries** (`is_readable`, `is_writable`) enable poll
//!   integration via a VFS node wrapper without baking scheduler policy into
//!   the queue primitive itself.

use alloc::collections::VecDeque;
use alloc::sync::Arc;
use spin::Mutex;

use crate::sched::WaitQueue;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Error returned when [`KernelMessageQueue::enqueue`] fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MqSendError {
    /// The queue has reached its bounded capacity.
    Full {
        /// Maximum number of messages the queue can hold.
        capacity: usize,
    },
    /// The queue has been closed; no further messages can be enqueued.
    Closed,
}

/// Error returned when [`KernelMessageQueue::try_dequeue`] fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MqRecvError {
    /// The queue has been closed and all previously queued messages have
    /// already been consumed.
    Closed,
}

// ---------------------------------------------------------------------------
// Internal state
// ---------------------------------------------------------------------------

struct Inner<M> {
    queue: VecDeque<M>,
    capacity: usize,
    closed: bool,
}

impl<M> Inner<M> {
    fn new(capacity: usize) -> Self {
        let capacity = capacity.max(1);
        Self {
            queue: VecDeque::with_capacity(capacity.min(64)),
            capacity,
            closed: false,
        }
    }
}

// ---------------------------------------------------------------------------
// KernelMessageQueue
// ---------------------------------------------------------------------------

/// Shared bounded FIFO message queue with integrated wait-queue support.
///
/// This is the common substrate that both the `Inbox` (ownership-first) and
/// `Port` structured-message (connection-first) IPC paths converge on.
///
/// # Type parameter
///
/// `M` is the message type stored in the queue.  The kernel uses:
/// - `crate::inbox::MessageEnvelope` for the `Inbox` path.
/// - `crate::ipc::port::KernelMessage` for the `Port` structured-message path.
///
/// # Wakeup contract
///
/// A successful [`enqueue`](Self::enqueue) calls
/// [`WaitQueue::wake_one`] on the public [`waiters`](Self::waiters) field.
/// Callers that need to block on a dequeue should register the current task
/// on `waiters` before attempting [`try_dequeue`](Self::try_dequeue) and park
/// after seeing `Ok(None)`.  This matches the `Inbox` and `Port` blocking
/// patterns and intentionally avoids baking scheduler policy into the queue.
pub struct KernelMessageQueue<M> {
    inner: Mutex<Inner<M>>,
    /// Wait queue for tasks blocked waiting for a message.
    ///
    /// Exposed publicly so that scheduling/poll layers can register and
    /// dequeue waiters without the queue needing to know about scheduler
    /// policy.  [`enqueue`](Self::enqueue) calls
    /// [`WaitQueue::wake_one`] on success.
    pub waiters: WaitQueue,
}

impl<M> KernelMessageQueue<M> {
    /// Create a new open queue with the given bounded capacity.
    ///
    /// A `capacity` of 0 is silently raised to 1.
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Mutex::new(Inner::new(capacity)),
            waiters: WaitQueue::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Core queue operations
    // -----------------------------------------------------------------------

    /// Enqueue one message.
    ///
    /// On success returns `Ok(())` and wakes one waiter (if any).
    ///
    /// # Errors
    ///
    /// - [`MqSendError::Full`]   — queue is at capacity.
    /// - [`MqSendError::Closed`] — queue has been closed.
    pub fn enqueue(&self, msg: M) -> Result<(), MqSendError> {
        {
            let mut inner = self.inner.lock();
            if inner.closed {
                return Err(MqSendError::Closed);
            }
            if inner.queue.len() >= inner.capacity {
                return Err(MqSendError::Full {
                    capacity: inner.capacity,
                });
            }
            inner.queue.push_back(msg);
        }
        // Wake one waiter outside the lock to avoid priority inversion.
        self.waiters.wake_one();
        Ok(())
    }

    /// Attempt to dequeue the next message without blocking.
    ///
    /// Returns:
    /// - `Ok(Some(msg))` — a message was available and has been removed.
    /// - `Ok(None)`      — the queue is empty but still open.
    /// - `Err(MqRecvError::Closed)` — the queue is closed **and** empty.
    ///
    /// A closed queue that still holds messages returns `Ok(Some(…))` until
    /// all messages are consumed; only then does it return
    /// `Err(MqRecvError::Closed)`.
    pub fn try_dequeue(&self) -> Result<Option<M>, MqRecvError> {
        let mut inner = self.inner.lock();
        if let Some(msg) = inner.queue.pop_front() {
            return Ok(Some(msg));
        }
        if inner.closed {
            return Err(MqRecvError::Closed);
        }
        Ok(None)
    }

    // -----------------------------------------------------------------------
    // Lifecycle
    // -----------------------------------------------------------------------

    /// Close the queue.
    ///
    /// After this call:
    /// - All subsequent [`enqueue`](Self::enqueue) calls return
    ///   [`MqSendError::Closed`].
    /// - [`try_dequeue`](Self::try_dequeue) continues to drain any messages
    ///   still queued at close time; once empty it returns
    ///   [`Err(MqRecvError::Closed)`].
    /// - All tasks registered on [`waiters`](Self::waiters) are woken.
    ///
    /// Closing an already-closed queue is a no-op.
    pub fn close(&self) {
        {
            let mut inner = self.inner.lock();
            if inner.closed {
                return;
            }
            inner.closed = true;
        }
        self.waiters.wake_all();
    }

    // -----------------------------------------------------------------------
    // Readiness / diagnostics
    // -----------------------------------------------------------------------

    /// Returns `true` if the queue has at least one message ready to dequeue,
    /// **or** the queue is closed (EOF-readable).
    ///
    /// Maps to `POLLIN` readiness in the VFS poll model.
    pub fn is_readable(&self) -> bool {
        let inner = self.inner.lock();
        !inner.queue.is_empty() || inner.closed
    }

    /// Returns `true` if the queue has capacity for at least one more message
    /// and has not been closed.
    ///
    /// Maps to `POLLOUT` readiness in the VFS poll model.
    pub fn is_writable(&self) -> bool {
        let inner = self.inner.lock();
        !inner.closed && inner.queue.len() < inner.capacity
    }

    /// Returns `true` if the queue has been closed.
    ///
    /// Maps to `POLLHUP` readiness in the VFS poll model.
    pub fn is_closed(&self) -> bool {
        self.inner.lock().closed
    }

    /// Current number of messages queued.
    pub fn len(&self) -> usize {
        self.inner.lock().queue.len()
    }

    /// Returns `true` if the queue contains no messages.
    pub fn is_empty(&self) -> bool {
        self.inner.lock().queue.is_empty()
    }

    /// Returns `true` if the queue has reached its bounded capacity.
    pub fn is_full(&self) -> bool {
        let inner = self.inner.lock();
        inner.queue.len() >= inner.capacity
    }

    /// The maximum number of messages the queue can hold.
    pub fn capacity(&self) -> usize {
        self.inner.lock().capacity
    }
}

// SAFETY: KernelMessageQueue is safe to share between threads.
// The `Mutex<Inner<M>>` serialises all queue mutations; `WaitQueue` is
// independently thread-safe.  `M` must be `Send` for the queue to be `Send`.
unsafe impl<M: Send> Send for KernelMessageQueue<M> {}
unsafe impl<M: Send> Sync for KernelMessageQueue<M> {}

// ---------------------------------------------------------------------------
// Convenience type aliases
// ---------------------------------------------------------------------------

/// A `KernelMessageQueue` pre-typed for the `Inbox` message path.
///
/// Uses `crate::inbox::MessageEnvelope` as the message type so that an
/// `InboxView` can be built directly on this queue.
pub type InboxQueue = KernelMessageQueue<crate::inbox::MessageEnvelope>;

/// A shared-reference wrapper used when multiple producers or consumers
/// need to hold a reference to the same queue.
pub type SharedMq<M> = Arc<KernelMessageQueue<M>>;

// ---------------------------------------------------------------------------
// InboxView
// ---------------------------------------------------------------------------

/// An ownership-first view over a [`KernelMessageQueue`].
///
/// `InboxView` exposes the inbox semantic API (`send` / `try_recv` /
/// `close`) over an `Arc<KernelMessageQueue<MessageEnvelope>>`.  It
/// demonstrates how the `Inbox` public API can be layered on the shared
/// queue core without duplicating queue mechanics.
///
/// # Relationship to `Inbox`
///
/// `Inbox` currently has its own internal queue implementation.  In a full
/// Phase B migration `Inbox` would hold an `Arc<InboxQueue>` internally.
/// `InboxView` is the prototype that validates that this layering works
/// correctly before committing to the migration.
pub struct InboxView {
    queue: Arc<InboxQueue>,
}

impl InboxView {
    /// Create an `InboxView` wrapping the given shared queue.
    pub fn new(queue: Arc<InboxQueue>) -> Self {
        Self { queue }
    }

    /// Enqueue a [`crate::inbox::MessageEnvelope`] (inbox-semantic `send`).
    ///
    /// Returns `Ok(())` on success; `Err` on full or closed queue.
    pub fn send(
        &self,
        envelope: crate::inbox::MessageEnvelope,
    ) -> Result<(), crate::inbox::SendError> {
        self.queue.enqueue(envelope).map_err(|e| match e {
            MqSendError::Full { capacity } => crate::inbox::SendError::Full { capacity },
            MqSendError::Closed => crate::inbox::SendError::Closed,
        })
    }

    /// Non-blocking dequeue (inbox-semantic `try_recv`).
    pub fn try_recv(
        &self,
    ) -> Result<Option<crate::inbox::MessageEnvelope>, crate::inbox::RecvError> {
        self.queue.try_dequeue().map_err(|_| crate::inbox::RecvError::Closed)
    }

    /// Close the inbox view (marks the queue closed, wakes all waiters).
    pub fn close(&self) {
        self.queue.close();
    }

    /// Access the wait queue to register a blocking receiver.
    pub fn waiters(&self) -> &WaitQueue {
        &self.queue.waiters
    }

    /// Readiness: `true` when at least one message is available or queue is
    /// closed.
    pub fn is_readable(&self) -> bool {
        self.queue.is_readable()
    }

    /// Readiness: `true` when queue has capacity and is not closed.
    pub fn is_writable(&self) -> bool {
        self.queue.is_writable()
    }
}

// ---------------------------------------------------------------------------
// PortMsgView
// ---------------------------------------------------------------------------

/// A connection-first view over a [`KernelMessageQueue`] for structured
/// messages.
///
/// `PortMsgView` exposes the port/channel structured-message semantic API
/// (`send_msg` / `try_recv_msg`) over an
/// `Arc<KernelMessageQueue<KernelMessage>>`.  It demonstrates how the
/// `Port` structured-message path can be layered on the shared queue core.
///
/// Note: The `Port` byte-stream ring-buffer path is **not** part of this
/// shared core; stream semantics remain `Port`-specific (see
/// `docs/ipc/convergence_strategy.md`, Option B assessment).
pub struct PortMsgView {
    queue: Arc<KernelMessageQueue<crate::ipc::port::KernelMessage>>,
}

impl PortMsgView {
    /// Create a `PortMsgView` wrapping the given shared queue.
    pub fn new(queue: Arc<KernelMessageQueue<crate::ipc::port::KernelMessage>>) -> Self {
        Self { queue }
    }

    /// Enqueue a structured message (port-semantic `send_msg`).
    pub fn send_msg(
        &self,
        data: alloc::vec::Vec<u8>,
        caps: alloc::vec::Vec<alloc::sync::Arc<dyn crate::vfs::VfsNode>>,
    ) -> Result<(), MqSendError> {
        self.queue
            .enqueue(crate::ipc::port::KernelMessage { data, caps })
    }

    /// Non-blocking dequeue of a structured message (port-semantic `try_recv_msg`).
    pub fn try_recv_msg(&self) -> Option<crate::ipc::port::KernelMessage> {
        self.queue.try_dequeue().ok().flatten()
    }

    /// Access the wait queue to register a blocking receiver.
    pub fn waiters(&self) -> &WaitQueue {
        &self.queue.waiters
    }

    /// Readiness: `true` when at least one message is available.
    pub fn is_readable(&self) -> bool {
        self.queue.is_readable()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inbox::{MessageEnvelope, SendError};
    use crate::message::{KindId, Message};
    use crate::sched::blocking::WAKE_TASK_HOOK;
    use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

    // ── Wake-recording helpers ─────────────────────────────────────────────

    static WOKEN_IDS: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];
    static WOKEN_LEN: AtomicUsize = AtomicUsize::new(0);
    static WAKE_GUARD: spin::Mutex<()> = spin::Mutex::new(());

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
            .map(|i| WOKEN_IDS[i].load(Ordering::SeqCst))
            .collect()
    }

    fn make_envelope(tag: u8) -> MessageEnvelope {
        MessageEnvelope::anonymous(Message::new(KindId([tag; 16]), alloc::vec![tag]))
    }

    // ── KernelMessageQueue basic semantics ─────────────────────────────────

    #[test]
    fn enqueue_and_try_dequeue_round_trip() {
        let q: KernelMessageQueue<u32> = KernelMessageQueue::new(8);
        assert!(q.enqueue(42u32).is_ok());
        assert_eq!(q.try_dequeue().unwrap(), Some(42u32));
    }

    #[test]
    fn try_dequeue_empty_open_returns_none() {
        let q: KernelMessageQueue<u32> = KernelMessageQueue::new(8);
        assert_eq!(q.try_dequeue().unwrap(), None);
    }

    #[test]
    fn enqueue_beyond_capacity_returns_full() {
        let q: KernelMessageQueue<u8> = KernelMessageQueue::new(2);
        assert!(q.enqueue(1).is_ok());
        assert!(q.enqueue(2).is_ok());
        assert_eq!(q.enqueue(3), Err(MqSendError::Full { capacity: 2 }));
    }

    #[test]
    fn fifo_ordering_preserved() {
        let q: KernelMessageQueue<u8> = KernelMessageQueue::new(8);
        for i in 0u8..5 {
            q.enqueue(i).unwrap();
        }
        for i in 0u8..5 {
            assert_eq!(q.try_dequeue().unwrap(), Some(i));
        }
    }

    #[test]
    fn close_rejects_subsequent_enqueues() {
        let q: KernelMessageQueue<u8> = KernelMessageQueue::new(8);
        q.close();
        assert_eq!(q.enqueue(1), Err(MqSendError::Closed));
    }

    #[test]
    fn close_drains_existing_messages_before_closed_error() {
        let q: KernelMessageQueue<u8> = KernelMessageQueue::new(8);
        q.enqueue(10).unwrap();
        q.enqueue(20).unwrap();
        q.close();
        assert_eq!(q.try_dequeue().unwrap(), Some(10));
        assert_eq!(q.try_dequeue().unwrap(), Some(20));
        assert_eq!(q.try_dequeue(), Err(MqRecvError::Closed));
    }

    #[test]
    fn readiness_queries_reflect_state() {
        let q: KernelMessageQueue<u8> = KernelMessageQueue::new(2);
        assert!(!q.is_readable(), "empty open queue not readable");
        assert!(q.is_writable(), "empty open queue writable");
        assert!(!q.is_closed());

        q.enqueue(1).unwrap();
        assert!(q.is_readable(), "non-empty queue readable");

        q.enqueue(2).unwrap();
        assert!(!q.is_writable(), "full queue not writable");

        q.close();
        assert!(q.is_closed());
        assert!(q.is_readable(), "closed queue still readable (POLLIN for EOF)");
        assert!(!q.is_writable(), "closed queue not writable");
    }

    // ── Wake integration ───────────────────────────────────────────────────

    #[test]
    fn enqueue_wakes_one_registered_waiter() {
        let _guard = WAKE_GUARD.lock();
        reset_wakes();
        WAKE_TASK_HOOK.store(record_wake as *mut (), Ordering::SeqCst);

        let q: KernelMessageQueue<u8> = KernelMessageQueue::new(8);
        q.waiters.push_back(99);
        q.enqueue(1).unwrap();

        assert_eq!(wake_log(), alloc::vec![99], "waiter should be woken on enqueue");

        WAKE_TASK_HOOK.store(core::ptr::null_mut(), Ordering::SeqCst);
    }

    #[test]
    fn close_wakes_all_registered_waiters() {
        let _guard = WAKE_GUARD.lock();
        reset_wakes();
        WAKE_TASK_HOOK.store(record_wake as *mut (), Ordering::SeqCst);

        let q: KernelMessageQueue<u8> = KernelMessageQueue::new(8);
        q.waiters.push_back(10);
        q.waiters.push_back(11);
        q.close();

        let log = wake_log();
        assert!(log.contains(&10), "waiter 10 woken on close");
        assert!(log.contains(&11), "waiter 11 woken on close");

        WAKE_TASK_HOOK.store(core::ptr::null_mut(), Ordering::SeqCst);
    }

    // ── InboxView ──────────────────────────────────────────────────────────

    #[test]
    fn inbox_view_send_and_try_recv_round_trip() {
        let q = Arc::new(KernelMessageQueue::new(8));
        let view = InboxView::new(Arc::clone(&q));
        let env = make_envelope(0xAB);
        assert!(view.send(env.clone()).is_ok());
        assert_eq!(view.try_recv().unwrap(), Some(env));
    }

    #[test]
    fn inbox_view_full_error_maps_correctly() {
        let q = Arc::new(KernelMessageQueue::new(1));
        let view = InboxView::new(Arc::clone(&q));
        view.send(make_envelope(1)).unwrap();
        assert_eq!(view.send(make_envelope(2)), Err(SendError::Full { capacity: 1 }));
    }

    #[test]
    fn inbox_view_closed_error_maps_correctly() {
        let q = Arc::new(KernelMessageQueue::new(8));
        let view = InboxView::new(Arc::clone(&q));
        view.close();
        assert_eq!(view.send(make_envelope(1)), Err(SendError::Closed));
        assert_eq!(view.try_recv(), Err(crate::inbox::RecvError::Closed));
    }

    // ── PortMsgView ────────────────────────────────────────────────────────

    #[test]
    fn port_msg_view_send_and_recv_round_trip() {
        use crate::ipc::port::KernelMessage;

        let q = Arc::new(KernelMessageQueue::<KernelMessage>::new(8));
        let view = PortMsgView::new(Arc::clone(&q));
        view.send_msg(alloc::vec![1, 2, 3], alloc::vec![]).unwrap();
        let msg = view.try_recv_msg().expect("message should be present");
        assert_eq!(msg.data, alloc::vec![1, 2, 3]);
        assert!(msg.caps.is_empty());
    }

    #[test]
    fn port_msg_view_empty_returns_none() {
        use crate::ipc::port::KernelMessage;

        let q = Arc::new(KernelMessageQueue::<KernelMessage>::new(8));
        let view = PortMsgView::new(q);
        assert!(view.try_recv_msg().is_none());
    }

    // ── Cross-communication: Inbox and Port views over a shared queue ──────
    //
    // This test validates the "cross-communication" requirement from Phase 5.
    // An InboxView enqueues a message; a second consumer (modelled here as a
    // direct queue access to simulate a PortMsgView-equivalent receiver)
    // dequeues it.  In a full implementation the two views would hold
    // `Arc<KernelMessageQueue<CommonMsg>>` references to the same object.

    #[test]
    fn shared_queue_inbox_enqueue_visible_to_port_receiver() {
        // Both views share the same underlying queue Arc.
        let q: Arc<InboxQueue> = Arc::new(KernelMessageQueue::new(8));
        let inbox_view = InboxView::new(Arc::clone(&q));
        // "Port-side" receiver accesses the queue directly (in production
        // this would be a PortMsgView or equivalent).
        let port_receiver = Arc::clone(&q);

        let env = make_envelope(0xFF);
        inbox_view.send(env.clone()).expect("inbox send should succeed");

        let received = port_receiver
            .try_dequeue()
            .expect("no error")
            .expect("message should be present");
        assert_eq!(received, env, "port receiver sees same message inbox enqueued");
    }

    #[test]
    fn shared_queue_close_observed_by_both_views() {
        let q: Arc<InboxQueue> = Arc::new(KernelMessageQueue::new(8));
        let inbox_view = InboxView::new(Arc::clone(&q));
        let shared_ref = Arc::clone(&q);

        // Close via inbox view.
        inbox_view.close();

        // Observed via the raw queue reference (representing port-side).
        assert!(shared_ref.is_closed(), "close propagates to shared queue");
        assert_eq!(
            inbox_view.send(make_envelope(1)),
            Err(SendError::Closed),
            "inbox send rejected after close"
        );
    }
}
