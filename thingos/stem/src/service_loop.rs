//! Inbox-backed service loop — the standard userspace shape for ThingOS services.
//!
//! `ServiceLoop` is the small, opinionated wrapper that makes the
//! [inbox-backed actor rule](../../../../docs/ipc/service_loop.md) cheap to
//! follow:
//!
//! > A ThingOS service is an inbox-backed actor.  It may additionally wait
//! > on FDs, IRQs, child exits, or provider ports, but its **control plane**
//! > is its inbox.
//!
//! Internally it is a [`WaitSet`] with the calling task's inbox FD
//! pre-registered as the first source, plus a scratch payload buffer sized
//! at construction.  When the inbox and one or more secondary sources fire
//! on the same wake, the inbox event is returned **first** — control-plane
//! priority is the whole point.
//!
//! # Non-goals
//!
//! - **Not** an async runtime.  No futures, no executors, no tasks.
//! - **Does not** own bulk data plumbing (memfd / rings / DMA).
//! - **Does not** hide ports.  Services that still need provider-port RPC
//!   keep their `PortHandle` and register its FD-bridged readable end as a
//!   secondary [`WaitToken`].
//! - **Does not** add new syscalls.  Pure userspace composition over
//!   [`crate::syscall::message::msg_inbox_open_self`],
//!   [`crate::syscall::message::msg_recv`], and [`WaitSet`].
//!
//! # Example
//!
//! ```no_run
//! use core::ops::ControlFlow;
//! use stem::Duration;
//! use stem::service_loop::{ServiceEvent, ServiceLoop};
//!
//! let mut svc = ServiceLoop::new(4096).unwrap();
//!
//! // Register secondary readiness sources alongside the inbox.
//! let pipe_token = svc.add_fd_readable(3).unwrap();
//!
//! svc.run(|event| match event {
//!     ServiceEvent::Message { kind, payload, .. } => {
//!         // Dispatch typed control message.
//!         let _ = (kind, payload);
//!         ControlFlow::Continue(())
//!     }
//!     ServiceEvent::Ready { token, .. } if token == pipe_token => {
//!         // Drain pipe data.
//!         ControlFlow::Continue(())
//!     }
//!     ServiceEvent::Ready { .. } => ControlFlow::Continue(()),
//!     ServiceEvent::InboxClosed => ControlFlow::Break(()),
//!     ServiceEvent::Timeout => ControlFlow::Continue(()),
//! }, Some(Duration::from_secs(5))).unwrap();
//! ```

use alloc::vec;
use alloc::vec::Vec;
use core::ops::ControlFlow;
use core::sync::atomic::{AtomicU64, Ordering};

use abi::wire::KindId;

use crate::errors::Errno;
use crate::syscall::message::{msg_inbox_open_self, msg_recv};
use crate::syscall::vfs::vfs_close;
use crate::time::Duration;
use crate::wait_set::{WaitEvent, WaitSet, WaitToken};

// ─── LoopMetrics ─────────────────────────────────────────────────────────────

/// Sentinel stored in [`LoopMetrics::current_event_kind`] when a
/// [`ServiceEvent::Ready`] is being dispatched (not a typed message).
pub const EVENT_KIND_READY: u64 = u64::MAX - 1;
/// Sentinel stored in [`LoopMetrics::current_event_kind`] when
/// [`ServiceEvent::InboxClosed`] is being dispatched.
pub const EVENT_KIND_INBOX_CLOSED: u64 = u64::MAX - 2;
/// Sentinel stored in [`LoopMetrics::current_event_kind`] when
/// [`ServiceEvent::Timeout`] is being dispatched.
pub const EVENT_KIND_TIMEOUT: u64 = u64::MAX - 3;
/// Sentinel stored in [`LoopMetrics::current_event_kind`] when the loop is
/// idle (blocked in `waitset.wait()`) or has not yet dispatched any event.
pub const EVENT_KIND_IDLE: u64 = 0;

/// Lightweight per-loop instrumentation for watchdog and diagnostics.
///
/// All fields are monotonic nanosecond timestamps (zero = "never observed") or
/// a small sentinel integer, updated with `Relaxed` atomic stores so there is
/// no measurable overhead in the dispatch hot path.
///
/// Exposed via [`ServiceLoop::metrics`] and written to
/// `/proc/self/serviceloop/stats` by [`ServiceLoop::publish_metrics`].
///
/// # Detecting stuck daemons
///
/// A daemon is stuck in dispatch when:
/// ```text
/// current_time - last_dispatch_start_ns > THRESHOLD
/// AND last_dispatch_end_ns < last_dispatch_start_ns
/// ```
/// A daemon has stopped waking when:
/// ```text
/// current_time - last_exit_wait_ns > THRESHOLD
/// AND last_enter_wait_ns >= last_exit_wait_ns
/// ```
pub struct LoopMetrics {
    /// Monotonic nanoseconds when the loop last called `waitset.wait()`
    /// (i.e., blocked waiting for the next event).  Zero if never observed.
    pub last_enter_wait_ns: AtomicU64,
    /// Monotonic nanoseconds when `waitset.wait()` last returned
    /// (i.e., an event was delivered to the loop).  Zero if never observed.
    pub last_exit_wait_ns: AtomicU64,
    /// Monotonic nanoseconds when the loop last began dispatching an event
    /// to its caller (just before `next_event` returns).  Zero if never.
    pub last_dispatch_start_ns: AtomicU64,
    /// Monotonic nanoseconds when the loop last finished dispatching — i.e.,
    /// when the caller returned control to `next_event` for the next cycle.
    /// Zero if never observed (no second call to `next_event` yet).
    pub last_dispatch_end_ns: AtomicU64,
    /// Identifies the most recently dispatched event.
    ///
    /// For [`ServiceEvent::Message`] events this is the high 8 bytes of the
    /// [`KindId`] (a stable fast-path identifier).
    /// For other events use the `EVENT_KIND_*` sentinels defined in this
    /// module.
    pub current_event_kind: AtomicU64,
}

impl Default for LoopMetrics {
    fn default() -> Self {
        Self {
            last_enter_wait_ns: AtomicU64::new(0),
            last_exit_wait_ns: AtomicU64::new(0),
            last_dispatch_start_ns: AtomicU64::new(0),
            last_dispatch_end_ns: AtomicU64::new(0),
            current_event_kind: AtomicU64::new(EVENT_KIND_IDLE),
        }
    }
}

impl LoopMetrics {
    /// Snapshot all fields with `Relaxed` loads.
    ///
    /// The snapshot is not atomically consistent across fields, but is
    /// sufficient for watchdog heuristics.
    pub fn snapshot(&self) -> LoopMetricsSnapshot {
        LoopMetricsSnapshot {
            last_enter_wait_ns: self.last_enter_wait_ns.load(Ordering::Relaxed),
            last_exit_wait_ns: self.last_exit_wait_ns.load(Ordering::Relaxed),
            last_dispatch_start_ns: self.last_dispatch_start_ns.load(Ordering::Relaxed),
            last_dispatch_end_ns: self.last_dispatch_end_ns.load(Ordering::Relaxed),
            current_event_kind: self.current_event_kind.load(Ordering::Relaxed),
        }
    }
}

/// A non-atomic point-in-time snapshot of [`LoopMetrics`].
///
/// Produced by [`LoopMetrics::snapshot`] and suitable for serialisation,
/// logging, or watchdog comparisons.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoopMetricsSnapshot {
    pub last_enter_wait_ns: u64,
    pub last_exit_wait_ns: u64,
    pub last_dispatch_start_ns: u64,
    pub last_dispatch_end_ns: u64,
    pub current_event_kind: u64,
}

impl LoopMetricsSnapshot {
    /// Serialise the snapshot to the provided `buf` as a newline-terminated
    /// text record compatible with `/proc/self/serviceloop/stats`.
    ///
    /// Returns the number of bytes written, or 0 if `buf` is too small.
    pub fn write_text(&self, buf: &mut [u8]) -> usize {
        use core::fmt::Write as _;
        let mut w = crate::utils::SliceWriter::new(buf);
        let _ = core::write!(
            w,
            "last_enter_wait_ns: {}\nlast_exit_wait_ns: {}\nlast_dispatch_start_ns: {}\nlast_dispatch_end_ns: {}\ncurrent_event_kind: {}\n",
            self.last_enter_wait_ns,
            self.last_exit_wait_ns,
            self.last_dispatch_start_ns,
            self.last_dispatch_end_ns,
            self.current_event_kind,
        );
        w.written()
    }
}

// ─── ServiceEvent ────────────────────────────────────────────────────────────

/// A single dispatched event yielded by [`ServiceLoop::next_event`].
///
/// Returned by value; the message payload borrows the loop's internal scratch
/// buffer and is therefore valid only until the next call to `next_event` or
/// `drain_inbox`.
#[derive(Debug)]
pub enum ServiceEvent<'a> {
    /// A typed control message was dequeued from the calling task's inbox.
    ///
    /// `payload` borrows the loop's internal scratch buffer.  Copy out before
    /// the next loop iteration if the data needs to outlive it.
    Message {
        /// 16-byte schema identifier for the payload.
        kind: KindId,
        /// Message payload bytes (borrowed from the loop's scratch buffer).
        payload: &'a [u8],
    },
    /// A registered secondary readiness source fired (FD, IRQ, task exit,
    /// port, watch).  Use the `token` to dispatch.
    Ready {
        /// The token assigned when the source was added.
        token: WaitToken,
        /// The full underlying [`WaitEvent`] for callers that need
        /// readability / writability / hangup / IRQ / exit detail.
        event: WaitEvent,
    },
    /// The inbox was closed (process is terminating or the inbox FD was
    /// revoked).  Services should treat this as a clean shutdown signal
    /// rather than spinning on EOF.
    InboxClosed,
    /// The supplied timeout elapsed before any source fired.
    ///
    /// Only returned when [`ServiceLoop::next_event`] is called with a
    /// finite `timeout`.
    Timeout,
}

// ─── LoopState ───────────────────────────────────────────────────────────────

/// Coarse state of a [`ServiceLoop`], mirroring the kernel's `ServiceLoopState`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum LoopState {
    #[default]
    Idle = 0,
    Waiting = 1,
    Dispatching = 2,
    Shutdown = 3,
}

impl LoopState {
    fn tag(self) -> u32 {
        self as u32
    }
}

// ─── ServiceLoop ─────────────────────────────────────────────────────────────

/// Inbox-backed service loop.
///
/// See the [module-level documentation](self) for the design contract.
pub struct ServiceLoop {
    inbox_fd: u32,
    inbox_token: WaitToken,
    waitset: WaitSet,
    scratch: Vec<u8>,
    /// Set when the inbox surfaces a hangup / EOF; latched so that subsequent
    /// `next_event` calls keep returning [`ServiceEvent::InboxClosed`] rather
    /// than busy-spinning.
    inbox_closed: bool,
    /// Per-loop instrumentation for watchdog and diagnostics.
    metrics: LoopMetrics,
    // ── Diagnostic state (reported to the kernel via SYS_SERVICE_LOOP_REPORT) ─
    /// Human-readable name for this loop instance (e.g. the service name).
    name: Vec<u8>,
    /// Current loop state.
    state: LoopState,
    /// Label of the last event dispatched.
    last_event: Vec<u8>,
    /// Monotonic nanosecond timestamp of the last dispatch.
    last_dispatch_ns: u64,
    /// Total wakeup count (incremented every time a non-timeout event fires).
    wakeups: u64,
    /// Timeout wakeup count.
    timeouts: u64,
    /// Dispatch error count.
    errors: u64,
}

impl ServiceLoop {
    /// Construct a new `ServiceLoop` for the calling task.
    ///
    /// `max_payload` sizes the internal scratch buffer used to receive
    /// inbox messages.  Payloads larger than `max_payload` are truncated by
    /// the kernel; services that need to handle larger messages should
    /// either size up or use a dedicated bulk-data channel (memfd / ring).
    ///
    /// # Errors
    ///
    /// - Any [`Errno`] returned by [`msg_inbox_open_self`] (e.g. `ENOENT` if
    ///   procfs is not mounted, `ENFILE` on FD exhaustion).
    /// - `Errno::ENOSPC` from the underlying [`WaitSet`] is impossible here
    ///   because the inbox is the first registration.
    pub fn new(max_payload: usize) -> Result<Self, Errno> {
        let inbox_fd = msg_inbox_open_self()?;
        let mut waitset = WaitSet::new();
        let inbox_token = match waitset.add_fd_readable(inbox_fd) {
            Ok(tok) => tok,
            Err(e) => {
                // Best-effort cleanup so we don't leak the inbox FD on the
                // (essentially impossible) registration failure path.
                let _ = vfs_close(inbox_fd);
                return Err(e);
            }
        };
        Ok(Self {
            inbox_fd,
            inbox_token,
            waitset,
            scratch: vec![0u8; max_payload],
            inbox_closed: false,
            metrics: LoopMetrics::default(),
            name: Vec::new(),
            state: LoopState::Idle,
            last_event: Vec::new(),
            last_dispatch_ns: 0,
            wakeups: 0,
            timeouts: 0,
            errors: 0,
        })
    }

    /// Set a human-readable name for this loop instance.
    ///
    /// The name (up to 64 bytes) is reported to the kernel and appears in
    /// `/proc/<pid>/serviceloop/name`.  Calling this is optional; without a
    /// name the file contains `"-"`.
    ///
    /// This method only updates the stored name; the new name is included in
    /// the next `report_state` call that occurs during normal loop operation.
    /// It does not itself trigger a state transition or a syscall.
    pub fn set_name(&mut self, name: &str) {
        let bytes = name.as_bytes();
        let len = bytes.len().min(64);
        self.name = bytes[..len].to_vec();
    }

    /// The token assigned to the inbox readiness source.
    ///
    /// Exposed so callers that need to inspect the underlying [`WaitSet`]
    /// (via [`Self::waitset`]) can identify the inbox source.
    #[inline]
    pub fn inbox_token(&self) -> WaitToken {
        self.inbox_token
    }

    /// The underlying inbox FD (e.g. for `vfs_read` of one message at a
    /// time, or to pass to other syscalls).
    #[inline]
    pub fn inbox_fd(&self) -> u32 {
        self.inbox_fd
    }

    /// Number of registered sources, including the inbox.
    #[inline]
    pub fn len(&self) -> usize {
        self.waitset.len()
    }

    /// `true` when only the inbox is registered.
    #[inline]
    pub fn is_inbox_only(&self) -> bool {
        self.waitset.len() == 1
    }

    /// Borrow the underlying [`WaitSet`] for read-only inspection.
    ///
    /// Useful for tests and diagnostics.  Mutation must go through the
    /// `add_*` / `remove` methods so the inbox token stays consistent.
    #[inline]
    pub fn waitset(&self) -> &WaitSet {
        &self.waitset
    }

    // ── secondary registration (thin pass-throughs to WaitSet) ───────────

    /// Register a VFS file descriptor for read readiness.
    pub fn add_fd_readable(&mut self, fd: u32) -> Result<WaitToken, Errno> {
        self.waitset.add_fd_readable(fd)
    }

    /// Register a VFS file descriptor for write readiness.
    pub fn add_fd_writable(&mut self, fd: u32) -> Result<WaitToken, Errno> {
        self.waitset.add_fd_writable(fd)
    }

    /// Watch a VFS thing for change events.
    ///
    /// Equivalent to [`add_fd_readable`][Self::add_fd_readable] for VFS
    /// watch FDs; kept as a separate name so call sites read clearly.
    #[allow(deprecated)]
    pub fn add_vfs_watch(&mut self, fd: u32) -> Result<WaitToken, Errno> {
        self.waitset.add_vfs_watch(fd)
    }

    /// Register a child task ID for exit notification.
    pub fn add_task_exit(&mut self, tid: u64) -> Result<WaitToken, Errno> {
        self.waitset.add_task_exit(tid)
    }

    /// Register an IRQ subscription handle (from `device_irq_subscribe`).
    pub fn add_irq(&mut self, irq_handle: u64) -> Result<WaitToken, Errno> {
        self.waitset.add_irq(irq_handle)
    }

    /// Remove a previously registered secondary source.
    ///
    /// Returns `true` if the token was found and removed.  Attempting to
    /// remove the inbox token returns `false` and leaves the loop intact —
    /// the inbox is structural and cannot be detached.
    pub fn remove(&mut self, token: WaitToken) -> bool {
        if token == self.inbox_token {
            return false;
        }
        self.waitset.remove(token)
    }

    // ── instrumentation ──────────────────────────────────────────────────

    /// Borrow the loop's per-instance instrumentation counters.
    ///
    /// All fields are `AtomicU64` and updated with `Relaxed` stores in the
    /// dispatch hot path.  Use [`LoopMetrics::snapshot`] to take a
    /// non-atomic point-in-time copy for logging or watchdog comparisons.
    #[inline]
    pub fn metrics(&self) -> &LoopMetrics {
        &self.metrics
    }

    /// Serialise the current metrics snapshot to `buf` and return the byte
    /// count written, or `Err(Errno::ENOBUFS)` when `buf` is too small to
    /// hold even a minimal record.
    ///
    /// The output is a plain-text key-value block compatible with the
    /// `/proc/self/serviceloop/stats` write format expected by the kernel.
    pub fn format_metrics(&self, buf: &mut [u8]) -> Result<usize, Errno> {
        let n = self.metrics.snapshot().write_text(buf);
        if n == 0 { Err(Errno::ENOBUFS) } else { Ok(n) }
    }

    /// Write the current metrics snapshot to `/proc/self/serviceloop/stats`.
    ///
    /// This is an explicit, opt-in call that persists the loop's most recent
    /// instrumentation state into the kernel-side procfs registry so that
    /// external observers (e.g. Sprout's watchdog) can read it without
    /// running inside the daemon's task.
    ///
    /// The write is a single VFS syscall; avoid calling it in the tight inner
    /// loop.  A reasonable cadence is once per second or on a significant
    /// state change.
    ///
    /// Errors are silently ignored to keep callers free of defensive
    /// boilerplate — the metrics are advisory.
    pub fn publish_metrics(&self) {
        let mut buf = [0u8; 256];
        if let Ok(n) = self.format_metrics(&mut buf) {
            let _ = write_proc_serviceloop_stats(&buf[..n]);
        }
    }

    // ── core dispatch ────────────────────────────────────────────────────

    /// Block until at least one registered source becomes ready, then
    /// dispatch one event.
    ///
    /// Dispatch order when multiple sources fire on the same wake:
    /// 1. Inbox closure (latched once seen — see [`ServiceEvent::InboxClosed`]).
    /// 2. Inbox readability → drain exactly one message and return
    ///    [`ServiceEvent::Message`].
    /// 3. The first secondary [`ServiceEvent::Ready`] in the kernel-returned
    ///    order.
    ///
    /// `timeout = None` blocks indefinitely.  A finite timeout that elapses
    /// before any source fires returns [`ServiceEvent::Timeout`].
    pub fn next_event(&mut self, timeout: Option<Duration>) -> Result<ServiceEvent<'_>, Errno> {
        // Mark the end of the previous dispatch cycle (or initialise to 0 on
        // the very first call).  This timestamp captures how long the caller
        // spent in their handler.
        self.metrics.last_dispatch_end_ns.store(crate::time::monotonic_ns(), Ordering::Relaxed);
        self.metrics.current_event_kind.store(EVENT_KIND_IDLE, Ordering::Relaxed);

        // Latched shutdown: keep returning InboxClosed so callers can break
        // their loop on a single match arm without spinning on EOF.
        if self.inbox_closed {
            self.metrics
                .last_dispatch_start_ns
                .store(crate::time::monotonic_ns(), Ordering::Relaxed);
            self.metrics.current_event_kind.store(EVENT_KIND_INBOX_CLOSED, Ordering::Relaxed);
            return Ok(ServiceEvent::InboxClosed);
        }

        self.metrics.last_enter_wait_ns.store(crate::time::monotonic_ns(), Ordering::Relaxed);
        self.report_state(LoopState::Waiting, b"");

        let events = match timeout {
            Some(timeout) => {
                let timeout_ns = timeout.as_nanos();
                let deadline = crate::time::monotonic_ns().saturating_add(timeout_ns);
                loop {
                    let events = self.waitset.wait(Some(Duration::from_nanos(0)))?;
                    if !events.is_empty() || crate::time::monotonic_ns() >= deadline {
                        break events;
                    }
                    let remaining = deadline.saturating_sub(crate::time::monotonic_ns());
                    crate::time::sleep_ns(remaining.min(10_000_000));
                }
            }
            None => self.waitset.wait(None::<Duration>)?,
        };
        self.metrics.last_exit_wait_ns.store(crate::time::monotonic_ns(), Ordering::Relaxed);

        if events.is_empty() {
            self.metrics
                .last_dispatch_start_ns
                .store(crate::time::monotonic_ns(), Ordering::Relaxed);
            self.metrics.current_event_kind.store(EVENT_KIND_TIMEOUT, Ordering::Relaxed);
            self.timeouts = self.timeouts.saturating_add(1);
            self.report_state(LoopState::Idle, b"timeout");
            return Ok(ServiceEvent::Timeout);
        }

        // Pass 1: inbox-first dispatch.  Hangup wins over readability so a
        // closing inbox surfaces as a clean shutdown event even when the
        // kernel reports both flags at once.
        let mut inbox_readable: Option<WaitEvent> = None;
        let mut inbox_hangup = false;
        for ev in events.iter() {
            if ev.token() == self.inbox_token {
                if ev.is_hangup() || ev.is_error() {
                    inbox_hangup = true;
                }
                if ev.is_readable() {
                    inbox_readable = Some(*ev);
                }
            }
        }

        if inbox_hangup {
            self.inbox_closed = true;
            self.metrics
                .last_dispatch_start_ns
                .store(crate::time::monotonic_ns(), Ordering::Relaxed);
            self.metrics.current_event_kind.store(EVENT_KIND_INBOX_CLOSED, Ordering::Relaxed);
            self.report_state(LoopState::Shutdown, b"inbox_closed");
            return Ok(ServiceEvent::InboxClosed);
        }

        if inbox_readable.is_some() {
            self.wakeups = self.wakeups.saturating_add(1);
            return self.recv_one_inbox_message();
        }

        // Pass 2: first secondary ready event in kernel-returned order.
        for ev in events.into_iter() {
            if ev.token() != self.inbox_token {
                self.metrics
                    .last_dispatch_start_ns
                    .store(crate::time::monotonic_ns(), Ordering::Relaxed);
                self.metrics.current_event_kind.store(EVENT_KIND_READY, Ordering::Relaxed);
                self.wakeups = self.wakeups.saturating_add(1);
                self.report_state(LoopState::Dispatching, b"ready");
                return Ok(ServiceEvent::Ready { token: ev.token(), event: ev });
            }
        }

        // Reachable only if the kernel returned events that were all the
        // inbox token with neither readability nor hangup set — treat as a
        // spurious wake and ask the caller to retry.
        self.metrics.last_dispatch_start_ns.store(crate::time::monotonic_ns(), Ordering::Relaxed);
        self.metrics.current_event_kind.store(EVENT_KIND_TIMEOUT, Ordering::Relaxed);
        self.report_state(LoopState::Idle, b"spurious");
        Ok(ServiceEvent::Timeout)
    }

    /// Drain *all* currently queued inbox messages, invoking `f` for each.
    ///
    /// This is the "I want to batch" escape hatch documented in
    /// `docs/ipc/service_loop.md`.  By default, [`Self::next_event`] returns
    /// exactly one message per wake to keep the loop predictable; callers
    /// that need higher throughput can call `drain_inbox` after dispatching
    /// the first message.
    ///
    /// Returns the number of messages drained.  Stops on the first transient
    /// `EAGAIN` (no more messages) or on the first `ControlFlow::Break`
    /// returned by `f`.
    pub fn drain_inbox<F>(&mut self, mut f: F) -> Result<usize, Errno>
    where
        F: FnMut(KindId, &[u8]) -> ControlFlow<()>,
    {
        let mut count = 0usize;
        loop {
            let mut kind = KindId([0u8; 16]);
            match msg_recv(&mut kind, &mut self.scratch) {
                Ok(n) => {
                    let copy_len = n.min(self.scratch.len());
                    if let ControlFlow::Break(()) = f(kind, &self.scratch[..copy_len]) {
                        return Ok(count + 1);
                    }
                    count += 1;
                }
                Err(Errno::EAGAIN) => return Ok(count),
                Err(e) => return Err(e),
            }
        }
    }

    /// Convenience driver: loop on `next_event`, dispatching to `handler`
    /// until it returns `ControlFlow::Break(())` (or until `next_event`
    /// fails).
    ///
    /// The `timeout` argument is forwarded verbatim to each `next_event`
    /// call; pass `None` for a strict event-driven loop.
    pub fn run<F>(&mut self, mut handler: F, timeout: Option<Duration>) -> Result<(), Errno>
    where
        F: FnMut(ServiceEvent<'_>) -> ControlFlow<()>,
    {
        loop {
            let event = self.next_event(timeout)?;
            if let ControlFlow::Break(()) = handler(event) {
                return Ok(());
            }
        }
    }

    /// Convenience driver that adds a one-shot shutdown hook.
    ///
    /// Loops on `next_event`, dispatching to `handler` for normal events.
    /// When [`ServiceEvent::InboxClosed`] is received, `on_shutdown` is
    /// called exactly once and then this method returns `Ok(())`.
    /// `on_shutdown` is also called if `handler` returns
    /// `ControlFlow::Break`.
    ///
    /// This is the preferred entry point for daemons that want the
    /// canonical shutdown contract:
    ///
    /// ```no_run
    /// use core::ops::ControlFlow;
    /// use stem::service_loop::{ServiceEvent, ServiceLoop};
    ///
    /// let mut svc = ServiceLoop::new(4096).unwrap();
    /// svc.run_until_shutdown(
    ///     |ev| match ev {
    ///         ServiceEvent::Message { .. } => ControlFlow::Continue(()),
    ///         _ => ControlFlow::Continue(()),
    ///     },
    ///     || {
    ///         // cleanup: unmount paths, close handles, flush logs …
    ///     },
    ///     None,
    /// ).unwrap();
    /// ```
    pub fn run_until_shutdown<F, S>(
        &mut self,
        mut handler: F,
        on_shutdown: S,
        timeout: Option<Duration>,
    ) -> Result<(), Errno>
    where
        F: FnMut(ServiceEvent<'_>) -> ControlFlow<()>,
        S: FnOnce(),
    {
        loop {
            match self.next_event(timeout)? {
                ServiceEvent::InboxClosed => {
                    on_shutdown();
                    return Ok(());
                }
                event => {
                    if let ControlFlow::Break(()) = handler(event) {
                        on_shutdown();
                        return Ok(());
                    }
                }
            }
        }
    }

    // ── internals ────────────────────────────────────────────────────────

    fn recv_one_inbox_message(&mut self) -> Result<ServiceEvent<'_>, Errno> {
        let mut kind = KindId([0u8; 16]);
        match msg_recv(&mut kind, &mut self.scratch) {
            Ok(n) => {
                // Record dispatch start and current event kind.
                self.metrics
                    .last_dispatch_start_ns
                    .store(crate::time::monotonic_ns(), Ordering::Relaxed);
                // Use the high 8 bytes of the KindId as a compact identifier.
                let kind_hi =
                    u64::from_le_bytes(kind.0[0..8].try_into().expect("KindId is 16 bytes"));
                self.metrics.current_event_kind.store(kind_hi, Ordering::Relaxed);
                let copy_len = n.min(self.scratch.len());
                self.report_state(LoopState::Dispatching, b"message");
                Ok(ServiceEvent::Message { kind, payload: &self.scratch[..copy_len] })
            }
            Err(Errno::EAGAIN) => {
                // Spurious readability — kernel woke us but no message was
                // available (e.g. a racing receiver drained it).  Surface
                // as Timeout so the caller's loop spins without crashing,
                // matching the contract that EAGAIN is not a service-level
                // error.
                self.metrics
                    .last_dispatch_start_ns
                    .store(crate::time::monotonic_ns(), Ordering::Relaxed);
                self.metrics.current_event_kind.store(EVENT_KIND_TIMEOUT, Ordering::Relaxed);
                self.report_state(LoopState::Idle, b"spurious");
                Ok(ServiceEvent::Timeout)
            }
            Err(e) => {
                self.errors = self.errors.saturating_add(1);
                Err(e)
            }
        }
    }

    /// Report the current diagnostic state to the kernel via
    /// `SYS_SERVICE_LOOP_REPORT`.
    ///
    /// This is a best-effort call; errors are silently ignored so that
    /// missing procfs (e.g. in unit tests) does not break service logic.
    fn report_state(&mut self, new_state: LoopState, event_label: &[u8]) {
        self.state = new_state;
        if !event_label.is_empty() {
            let len = event_label.len().min(64);
            self.last_event.clear();
            self.last_event.extend_from_slice(&event_label[..len]);
        }
        self.last_dispatch_ns = crate::time::monotonic_ns();
        let _ = crate::syscall::service_loop_report(
            self.state.tag(),
            &self.name,
            &self.last_event,
            self.last_dispatch_ns,
        );
    }
}

impl Drop for ServiceLoop {
    fn drop(&mut self) {
        // Best-effort: report shutdown state before closing.
        self.report_state(LoopState::Shutdown, b"drop");
        // Best-effort: close the inbox FD we opened in `new`.  We don't
        // surface errors — Drop is a destructor.
        let _ = vfs_close(self.inbox_fd);
    }
}

// ─── procfs publish helper ───────────────────────────────────────────────────

/// Write a metrics snapshot to `/proc/self/serviceloop/stats`.
///
/// The kernel will parse the text and store it in the per-pid service loop
/// stats registry, making it readable via `/proc/<pid>/serviceloop/`.
///
/// Returns `Ok(())` on success and silently swallows any VFS error so
/// callers (including [`ServiceLoop::publish_metrics`]) can treat publishing
/// as best-effort.
fn write_proc_serviceloop_stats(data: &[u8]) -> Result<(), Errno> {
    let fd = crate::syscall::vfs::vfs_open(
        "/proc/self/serviceloop/stats",
        crate::abi::syscall::vfs_flags::O_WRONLY,
    )?;
    let _ = crate::syscall::vfs::vfs_write(fd, data);
    let _ = crate::syscall::vfs::vfs_close(fd);
    Ok(())
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    //! Unit tests focus on the pieces that don't require a live kernel:
    //! `ServiceEvent` shape, dispatch ordering invariants, and the
    //! `inbox_closed` latch.  End-to-end coverage (inbox-first vs. pipe
    //! ready, IRQ wake, clean shutdown) lives in
    //! `docs/behavior/features/service-loop.feature`.
    use super::*;

    /// A `ServiceEvent::Message` borrows the scratch buffer exclusively for
    /// the lifetime of the borrow — verify the type compiles as documented.
    #[test]
    fn service_event_message_borrows_payload() {
        let kind = KindId([1u8; 16]);
        let buf = [0xAAu8; 4];
        let ev = ServiceEvent::Message { kind, payload: &buf };
        match ev {
            ServiceEvent::Message { kind: k, payload } => {
                assert_eq!(k.0, [1u8; 16]);
                assert_eq!(payload, &[0xAAu8; 4]);
            }
            _ => panic!("expected Message"),
        }
    }

    /// `ServiceEvent::InboxClosed` and `Timeout` carry no payload.
    #[test]
    fn service_event_terminal_variants_are_unit() {
        let _ = ServiceEvent::InboxClosed;
        let _ = ServiceEvent::Timeout;
    }

    // ── LoopMetrics tests ────────────────────────────────────────────────

    /// `LoopMetrics` starts with all fields zero / idle.
    #[test]
    fn loop_metrics_default_is_zero() {
        let m = LoopMetrics::default();
        assert_eq!(m.last_enter_wait_ns.load(Ordering::Relaxed), 0);
        assert_eq!(m.last_exit_wait_ns.load(Ordering::Relaxed), 0);
        assert_eq!(m.last_dispatch_start_ns.load(Ordering::Relaxed), 0);
        assert_eq!(m.last_dispatch_end_ns.load(Ordering::Relaxed), 0);
        assert_eq!(m.current_event_kind.load(Ordering::Relaxed), EVENT_KIND_IDLE);
    }

    /// `LoopMetrics::snapshot` returns all fields as a `LoopMetricsSnapshot`.
    #[test]
    fn loop_metrics_snapshot_round_trips() {
        let m = LoopMetrics::default();
        m.last_enter_wait_ns.store(100, Ordering::Relaxed);
        m.last_exit_wait_ns.store(200, Ordering::Relaxed);
        m.last_dispatch_start_ns.store(250, Ordering::Relaxed);
        m.last_dispatch_end_ns.store(300, Ordering::Relaxed);
        m.current_event_kind.store(EVENT_KIND_READY, Ordering::Relaxed);

        let s = m.snapshot();
        assert_eq!(s.last_enter_wait_ns, 100);
        assert_eq!(s.last_exit_wait_ns, 200);
        assert_eq!(s.last_dispatch_start_ns, 250);
        assert_eq!(s.last_dispatch_end_ns, 300);
        assert_eq!(s.current_event_kind, EVENT_KIND_READY);
    }

    /// `LoopMetricsSnapshot::write_text` produces a parseable key-value block.
    #[test]
    fn loop_metrics_snapshot_write_text_format() {
        let s = LoopMetricsSnapshot {
            last_enter_wait_ns: 1000,
            last_exit_wait_ns: 2000,
            last_dispatch_start_ns: 2500,
            last_dispatch_end_ns: 3000,
            current_event_kind: EVENT_KIND_IDLE,
        };
        let mut buf = [0u8; 256];
        let n = s.write_text(&mut buf);
        assert!(n > 0, "write_text should produce output");
        let text = core::str::from_utf8(&buf[..n]).expect("output must be valid UTF-8");
        assert!(text.contains("last_enter_wait_ns: 1000"), "missing last_enter_wait_ns");
        assert!(text.contains("last_exit_wait_ns: 2000"), "missing last_exit_wait_ns");
        assert!(text.contains("last_dispatch_start_ns: 2500"), "missing last_dispatch_start_ns");
        assert!(text.contains("last_dispatch_end_ns: 3000"), "missing last_dispatch_end_ns");
        assert!(text.contains("current_event_kind: 0"), "missing current_event_kind");
    }

    /// `write_text` returns 0 when the buffer is too small (empty).
    #[test]
    fn loop_metrics_snapshot_write_text_empty_buf() {
        let s = LoopMetricsSnapshot {
            last_enter_wait_ns: 1,
            last_exit_wait_ns: 2,
            last_dispatch_start_ns: 3,
            last_dispatch_end_ns: 4,
            current_event_kind: 5,
        };
        let mut buf = [];
        let n = s.write_text(&mut buf);
        // write_text writes 0 bytes for an empty buf (no panic).
        assert_eq!(n, 0);
    }

    /// Watchdog detection: a daemon with `last_dispatch_start_ns >
    /// last_dispatch_end_ns` is currently in dispatch.
    #[test]
    fn watchdog_dispatch_stuck_detection() {
        let m = LoopMetrics::default();
        m.last_dispatch_start_ns.store(5000, Ordering::Relaxed);
        // last_dispatch_end_ns stays at 0 (< start) — stuck in dispatch
        let s = m.snapshot();
        let in_dispatch =
            s.last_dispatch_start_ns > 0 && s.last_dispatch_end_ns < s.last_dispatch_start_ns;
        assert!(in_dispatch, "should be detected as stuck in dispatch");

        // After dispatch ends, end >= start
        m.last_dispatch_end_ns.store(6000, Ordering::Relaxed);
        let s2 = m.snapshot();
        let still_stuck =
            s2.last_dispatch_start_ns > 0 && s2.last_dispatch_end_ns < s2.last_dispatch_start_ns;
        assert!(!still_stuck, "should no longer be stuck after end >= start");
    }

    /// Watchdog detection: a daemon with no wakeups (`last_exit_wait_ns == 0`
    /// or very old) over a long interval should be flagged.
    #[test]
    fn watchdog_no_wakeup_detection() {
        let m = LoopMetrics::default();
        // last_exit_wait_ns = 0 means never woken
        let s = m.snapshot();
        let never_woken = s.last_exit_wait_ns == 0;
        assert!(never_woken, "default metrics should indicate no wakeups yet");
    }

    /// `EVENT_KIND_*` sentinels are distinct and non-overlapping with plausible
    /// KindId high-byte values.
    #[test]
    fn event_kind_sentinels_are_distinct() {
        assert_ne!(EVENT_KIND_IDLE, EVENT_KIND_READY);
        assert_ne!(EVENT_KIND_IDLE, EVENT_KIND_INBOX_CLOSED);
        assert_ne!(EVENT_KIND_IDLE, EVENT_KIND_TIMEOUT);
        assert_ne!(EVENT_KIND_READY, EVENT_KIND_INBOX_CLOSED);
        assert_ne!(EVENT_KIND_READY, EVENT_KIND_TIMEOUT);
        assert_ne!(EVENT_KIND_INBOX_CLOSED, EVENT_KIND_TIMEOUT);
        // Sentinels must not collide with common KindId high-bytes (all-zeros,
        // all-ones minus the three reserved values are reserved).
        assert_eq!(EVENT_KIND_IDLE, 0);
        assert!(EVENT_KIND_READY > u64::MAX / 2, "sentinels live in high range");
    }
}
