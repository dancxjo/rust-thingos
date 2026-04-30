//! VFS node wrapper for an [`Inbox`].
//!
//! Wraps an `Arc<Inbox>` as a [`VfsNode`] so that inbox-backed delivery
//! queues can participate in `SYS_FS_POLL`-based readiness monitoring
//! alongside ports and pipes.
//!
//! # Motivation
//!
//! The `Inbox` primitive already has a [`WaitQueue`](crate::sched::WaitQueue)
//! for blocking receives, but it lacks a first-class VFS/FD surface.  This
//! node provides that surface without modifying the `Inbox` core.
//!
//! This implements Phase C, step 1 of the convergence roadmap described in
//! `docs/ipc/convergence_strategy.md`:
//! > Add inbox VFS wrapper node (poll + waiter hooks).
//!
//! # Poll semantics
//!
//! | Flag       | Condition |
//! |------------|-----------|
//! | `POLLIN`   | Inbox has at least one message, **or** inbox is closed (EOF). |
//! | `POLLHUP`  | Inbox is closed (ownership-model peer-gone equivalent). |
//! | `POLLOUT`  | Inbox is not full **and** not closed (sender-side readiness). |
//!
//! # Read / write semantics
//!
//! - [`read`](InboxNode::read): dequeues one message; copies its payload
//!   bytes into the caller's buffer.  Returns `EAGAIN` when the queue is
//!   empty, `EPIPE` when closed and drained.
//! - [`write`](InboxNode::write): constructs an anonymous [`MessageEnvelope`]
//!   from the raw bytes and enqueues it.  Returns `EAGAIN` on full, `EPIPE`
//!   on closed.  The kind-id is set to all-zeros (opaque raw delivery); use
//!   `Inbox::send` directly for typed delivery with a proper `KindId`.

use alloc::sync::Arc;
use alloc::vec::Vec;

use abi::errors::{Errno, SysResult};

use super::{VfsNode, VfsStat};
use crate::inbox::{Inbox, MessageEnvelope, SendError};
use crate::message::{KindId, Message};

/// VFS node backed by an [`Inbox`].
///
/// Enables inbox-backed queues to participate in `SYS_FS_POLL` readiness
/// monitoring alongside ports and pipes, fulfilling the unified readiness
/// model described in `docs/ipc/inbox_vs_port_semantics.md` §5.
pub struct InboxNode {
    inbox: Arc<Inbox>,
}

impl InboxNode {
    /// Wrap an existing `Arc<Inbox>` as a pollable VFS node.
    pub fn new(inbox: Arc<Inbox>) -> Self {
        Self { inbox }
    }

    /// Access the underlying [`Inbox`].
    pub fn inbox(&self) -> &Arc<Inbox> {
        &self.inbox
    }
}

impl VfsNode for InboxNode {
    /// Dequeue one message and copy its payload into `buf`.
    ///
    /// Returns the number of bytes copied (may be less than the message
    /// payload length if `buf` is smaller).  Returns `EAGAIN` when the queue
    /// is empty; `EPIPE` when the queue is closed and drained.
    fn read(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        match self.inbox.try_recv() {
            Ok(Some(env)) => {
                let data = &env.message.payload;
                let n = data.len().min(buf.len());
                buf[..n].copy_from_slice(&data[..n]);
                Ok(n)
            }
            Ok(None) => Err(Errno::EAGAIN),
            Err(_) => Err(Errno::EPIPE),
        }
    }

    /// Enqueue an anonymous raw-bytes message.
    ///
    /// Wraps `buf` in a [`MessageEnvelope`] with a zero [`KindId`] and
    /// enqueues it.  Callers that need typed delivery should use
    /// `Inbox::send` directly.
    ///
    /// Returns `EAGAIN` when the queue is full; `EPIPE` when closed.
    fn write(&self, _offset: u64, buf: &[u8]) -> SysResult<usize> {
        let msg = Message::new(KindId([0u8; 16]), buf.to_vec());
        let env = MessageEnvelope::anonymous(msg);
        match self.inbox.send(env) {
            Ok(()) => Ok(buf.len()),
            Err(SendError::Full { .. }) => Err(Errno::EAGAIN),
            Err(SendError::Closed) => Err(Errno::EPIPE),
        }
    }

    /// Enqueue one typed inbox message with optional attached capabilities.
    ///
    /// The userspace `sendmsg` ABI only carries one byte slice, so inbox
    /// senders prefix the payload with a 16-byte `KindId`.  Short messages are
    /// accepted as anonymous raw payloads with a zero kind for compatibility
    /// with the byte-oriented `write` path.
    fn sock_sendmsg(&self, data: &[u8], fds: Vec<Arc<dyn VfsNode>>) -> SysResult<()> {
        let (kind, payload) = if data.len() >= 16 {
            let mut kind = [0u8; 16];
            kind.copy_from_slice(&data[..16]);
            (KindId(kind), data[16..].to_vec())
        } else {
            (KindId([0u8; 16]), data.to_vec())
        };
        let msg = Message::new(kind, payload);
        let env = MessageEnvelope::anonymous_with_capabilities(msg, fds);
        match self.inbox.send(env) {
            Ok(()) => Ok(()),
            Err(SendError::Full { .. }) => Err(Errno::EAGAIN),
            Err(SendError::Closed) => Err(Errno::EPIPE),
        }
    }

    /// Dequeue one typed inbox message and return `[KindId][payload]` plus
    /// attached capabilities.
    fn sock_recvmsg(&self) -> SysResult<Option<(Vec<u8>, Vec<Arc<dyn VfsNode>>)>> {
        match self.inbox.try_recv() {
            Ok(Some(env)) => {
                let mut data = Vec::with_capacity(16 + env.message.payload.len());
                data.extend_from_slice(&env.message.kind.0);
                data.extend_from_slice(&env.message.payload);
                Ok(Some((data, env.capabilities)))
            }
            Ok(None) => Ok(None),
            Err(_) => Err(Errno::EPIPE),
        }
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFIFO | 0o600,
            size: self.inbox.len() as u64,
            ino: 0,
            ..Default::default()
        })
    }

    /// Poll this inbox node for readiness.
    ///
    /// - `POLLIN`  — inbox non-empty or closed (EOF readable).
    /// - `POLLHUP` — inbox is closed.
    /// - `POLLOUT` — inbox has capacity and is not closed.
    fn poll(&self) -> u16 {
        use abi::syscall::poll_flags::{POLLHUP, POLLIN, POLLOUT};
        let mut revents = 0u16;
        if !self.inbox.is_empty() || self.inbox.is_closed() {
            revents |= POLLIN;
        }
        if self.inbox.is_closed() {
            revents |= POLLHUP;
        }
        if !self.inbox.is_full() && !self.inbox.is_closed() {
            revents |= POLLOUT;
        }
        revents
    }

    /// Close the underlying inbox (marks it closed, wakes all waiters).
    fn close(&self) {
        self.inbox.close();
    }

    /// Register a task to be woken when the inbox becomes readable.
    fn add_waiter(&self, tid: u64) {
        self.inbox.waiters.push_back(tid);
    }

    /// Deregister a previously registered waiter.
    fn remove_waiter(&self, tid: u64) {
        self.inbox.waiters.remove(tid);
    }
}

#[cfg(test)]
mod tests {
    use alloc::sync::Arc;

    use abi::syscall::poll_flags;

    use super::*;

    fn make_inbox(capacity: usize) -> Arc<Inbox> {
        Arc::new(Inbox::new(capacity))
    }

    fn make_node(capacity: usize) -> InboxNode {
        InboxNode::new(make_inbox(capacity))
    }

    // ── Poll semantics ─────────────────────────────────────────────────────

    #[test]
    fn empty_open_inbox_not_pollin_but_is_pollout() {
        let node = make_node(8);
        let flags = node.poll();
        assert_eq!(flags & poll_flags::POLLIN, 0, "empty open inbox not POLLIN");
        assert_ne!(flags & poll_flags::POLLOUT, 0, "empty open inbox POLLOUT");
        assert_eq!(flags & poll_flags::POLLHUP, 0, "open inbox not POLLHUP");
    }

    #[test]
    fn inbox_with_message_is_pollin() {
        let node = make_node(8);
        node.write(0, b"hello").expect("write");
        let flags = node.poll();
        assert_ne!(flags & poll_flags::POLLIN, 0, "non-empty inbox POLLIN");
    }

    #[test]
    fn closed_inbox_is_pollin_and_pollhup() {
        let node = make_node(8);
        node.close();
        let flags = node.poll();
        assert_ne!(flags & poll_flags::POLLIN, 0, "closed inbox POLLIN (EOF)");
        assert_ne!(flags & poll_flags::POLLHUP, 0, "closed inbox POLLHUP");
        assert_eq!(flags & poll_flags::POLLOUT, 0, "closed inbox not POLLOUT");
    }

    #[test]
    fn full_inbox_not_pollout() {
        let node = make_node(2);
        node.write(0, b"a").expect("write 1");
        node.write(0, b"b").expect("write 2");
        let flags = node.poll();
        assert_eq!(flags & poll_flags::POLLOUT, 0, "full inbox not POLLOUT");
    }

    // ── Read / write semantics ─────────────────────────────────────────────

    #[test]
    fn write_then_read_round_trips_payload() {
        let node = make_node(8);
        node.write(0, b"world").expect("write");
        let mut buf = [0u8; 16];
        let n = node.read(0, &mut buf).expect("read");
        assert_eq!(&buf[..n], b"world");
    }

    #[test]
    fn sendmsg_then_recvmsg_round_trips_kind_payload_and_caps() {
        let node = make_node(8);
        let cap: Arc<dyn VfsNode> = Arc::new(make_node(1));
        let kind = [7u8; 16];
        let mut data = Vec::new();
        data.extend_from_slice(&kind);
        data.extend_from_slice(b"payload");

        node.sock_sendmsg(&data, vec![cap.clone()]).expect("sendmsg");

        let (out_data, caps) = node.sock_recvmsg().expect("recvmsg").expect("queued");
        assert_eq!(&out_data[..16], &kind);
        assert_eq!(&out_data[16..], b"payload");
        assert_eq!(caps.len(), 1);
        assert!(Arc::ptr_eq(&caps[0], &cap));
    }

    #[test]
    fn read_on_empty_open_inbox_returns_eagain() {
        let node = make_node(8);
        let mut buf = [0u8; 8];
        let err = node.read(0, &mut buf).unwrap_err();
        assert_eq!(err, Errno::EAGAIN);
    }

    #[test]
    fn read_on_closed_drained_inbox_returns_epipe() {
        let node = make_node(8);
        node.close();
        let mut buf = [0u8; 8];
        let err = node.read(0, &mut buf).unwrap_err();
        assert_eq!(err, Errno::EPIPE);
    }

    #[test]
    fn write_on_full_inbox_returns_eagain() {
        let node = make_node(1);
        node.write(0, b"a").expect("first write");
        let err = node.write(0, b"b").unwrap_err();
        assert_eq!(err, Errno::EAGAIN);
    }

    #[test]
    fn write_on_closed_inbox_returns_epipe() {
        let node = make_node(8);
        node.close();
        let err = node.write(0, b"x").unwrap_err();
        assert_eq!(err, Errno::EPIPE);
    }

    // ── Waiter registration ────────────────────────────────────────────────

    #[test]
    fn add_and_remove_waiter_does_not_panic() {
        let node = make_node(8);
        node.add_waiter(42);
        node.remove_waiter(42);
    }

    // ── Stat ───────────────────────────────────────────────────────────────

    #[test]
    fn stat_reports_inbox_depth_as_size() {
        let node = make_node(8);
        node.write(0, b"msg").expect("write");
        let stat = node.stat().expect("stat");
        assert_eq!(stat.size, 1, "stat.size == message count");
        assert_ne!(stat.mode & VfsStat::S_IFIFO, 0, "FIFO mode bit set");
    }
}
