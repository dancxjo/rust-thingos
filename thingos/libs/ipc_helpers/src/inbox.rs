//! Inbox message-passing helpers.
//!
//! This module exposes ergonomic wrappers around the low-level
//! [`stem::syscall::message`] functions so that services and drivers can
//! adopt the inbox/message IPC pattern without writing boilerplate.
//!
//! # When to use inbox vs channel
//!
//! | Criterion                        | Inbox (this module)           | Channel (`channel` module)        |
//! |----------------------------------|-------------------------------|-----------------------------------|
//! | Recipient identity               | Process by PID                | Explicit handle exchange          |
//! | Connection setup required        | None (process-addressed)      | `port_create` + publish handle |
//! | Delivery semantics               | At-most-once, typed           | Byte-stream or structured message |
//! | Backpressure model               | Inbox-full `EAGAIN` to sender | Ring-full `EAGAIN` to sender      |
//! | Fan-out (one-to-many)            | `msg_broadcast` to pgid       | Manual fan-out to multiple handles|
//! | Good fit for                     | Lifecycle events, job exit,   | Request/reply RPC, streaming data |
//! |                                  | system-wide typed events      |                                   |
//!
//! # Quick start: receiving messages
//!
//! ```ignore
//! use ipc_helpers::inbox::{InboxReceiver, KindId};
//!
//! let mut rx = InboxReceiver::new(256);
//! loop {
//!     let msg = rx.recv_blocking();
//!     if msg.kind == KindId::SHUTDOWN {
//!         break;
//!     }
//!     // process msg.payload …
//! }
//! ```
//!
//! # Quick start: sending messages
//!
//! ```ignore
//! use ipc_helpers::inbox::{send_typed, KindId};
//!
//! // Send a typed message to process 42.
//! send_typed(42, KindId::SHUTDOWN, b"").unwrap();
//! ```

use abi::errors::Errno;
use stem::syscall::message::{msg_broadcast, msg_recv, msg_recv_blocking as stem_recv_blocking,
                              msg_send};

// Re-export types callers need so they only import from this module.
pub use stem::syscall::message::{BroadcastResult, KindId, ReceivedMessage};

// ── InboxReceiver ─────────────────────────────────────────────────────────────

/// A stateful helper for receiving typed messages from the calling process's inbox.
///
/// Owns a payload scratch buffer so that the caller does not need to manage
/// buffer lifetimes manually.  All receive operations go through this struct.
///
/// # Example
///
/// ```ignore
/// use ipc_helpers::inbox::InboxReceiver;
///
/// let mut rx = InboxReceiver::new(4096);
/// loop {
///     let msg = rx.recv_blocking();
///     // dispatch on msg.kind …
/// }
/// ```
pub struct InboxReceiver {
    max_payload: usize,
}

impl InboxReceiver {
    /// Create a new receiver with the given maximum payload buffer size.
    ///
    /// Messages with payloads larger than `max_payload` will be truncated on
    /// receive; use a size appropriate for your protocol.
    pub fn new(max_payload: usize) -> Self {
        Self { max_payload }
    }

    /// Non-blocking dequeue.
    ///
    /// Returns `Ok(msg)` when a message was available, or `Err(Errno::EAGAIN)`
    /// when the inbox is empty.
    pub fn try_recv(&self) -> Result<ReceivedMessage, Errno> {
        let mut kind = KindId([0u8; 16]);
        let mut payload_buf = alloc::vec![0u8; self.max_payload];
        let actual_len = msg_recv(&mut kind, &mut payload_buf)?;
        let copy_len = actual_len.min(self.max_payload);
        payload_buf.truncate(copy_len);
        Ok(ReceivedMessage { kind, payload: payload_buf })
    }

    /// Block until a message is available, then dequeue and return it.
    ///
    /// Yields the current task between retries so this is scheduler-friendly.
    pub fn recv_blocking(&self) -> ReceivedMessage {
        stem_recv_blocking(self.max_payload)
    }
}

// ── Convenience send functions ────────────────────────────────────────────────

/// Send one typed message directly to process `pid`.
///
/// Thin wrapper around [`stem::syscall::message::msg_send`] for use from
/// service and driver code that imports `ipc_helpers`.
///
/// # Errors
///
/// Returns `Errno::EAGAIN` when the recipient inbox is full.  Callers should
/// apply backpressure or use [`send_typed_blocking`] to retry automatically.
pub fn send_typed(pid: u32, kind: KindId, payload: &[u8]) -> Result<(), Errno> {
    msg_send(pid, kind, payload)
}

/// Send one typed message to process `pid`, retrying on full inbox.
///
/// Yields between retries.  Suitable when backpressure must not cause the
/// caller to drop the message.
pub fn send_typed_blocking(pid: u32, kind: KindId, payload: &[u8]) -> Result<(), Errno> {
    loop {
        match msg_send(pid, kind, payload) {
            Ok(()) => return Ok(()),
            Err(Errno::EAGAIN) => stem::syscall::yield_now(),
            Err(e) => return Err(e),
        }
    }
}

/// Broadcast one typed message to all members of process group `pgid`.
///
/// Thin wrapper around [`stem::syscall::message::msg_broadcast`] that unpacks
/// the compact status word into a [`BroadcastResult`].
pub fn broadcast_typed(
    pgid: u32,
    kind: KindId,
    payload: &[u8],
) -> Result<BroadcastResult, Errno> {
    msg_broadcast(pgid, kind, payload)
}
