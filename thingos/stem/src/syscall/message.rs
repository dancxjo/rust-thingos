//! Typed message delivery and receipt syscall wrappers.
//!
//! Provides ergonomic access to the three inbox/message syscalls:
//!
//! | Function         | Syscall            | Description                              |
//! |------------------|--------------------|------------------------------------------|
//! | [`msg_send`]     | `SYS_MSG_SEND`     | Deliver one message to a process by PID  |
//! | [`msg_recv`]     | `SYS_MSG_RECV`     | Dequeue one message from own inbox       |
//! | [`msg_broadcast`]| `SYS_MSG_BROADCAST`| Broadcast to all members of a pgid       |
//!
//! # Wire format
//!
//! All three calls share the same 16-byte KindId prefix layout at the ABI
//! boundary.  The KindId identifies the semantic type of the payload so that
//! receivers can dispatch without inspecting payload bytes.
//!
//! # Blocking behaviour
//!
//! `msg_recv` returns `Err(Errno::EAGAIN)` when the inbox is empty.  To block
//! until a message arrives use [`msg_recv_blocking`], which yields the current
//! task between retries.  For poll-based multiplexing, convert the inbox to an
//! FD (see `docs/ipc/convergence_strategy.md` Phase C) and use `SYS_FS_POLL`.

// ── KindId re-export ─────────────────────────────────────────────────────────
/// A 16-byte identifier for the semantic type (schema kind) of a message payload.
///
/// Re-exported from `abi` so callers do not need to import `abi` separately.
pub use abi::KindId;
use abi::errors::Errno;
use abi::syscall::vfs_flags::O_RDONLY;
use abi::syscall::{SYS_MSG_BROADCAST, SYS_MSG_RECV, SYS_MSG_SEND};

use crate::syscall::arch::raw_syscall6;

// ── Broadcast result ─────────────────────────────────────────────────────────

/// Aggregate result returned by [`msg_broadcast`].
///
/// Packed from the compact status word returned by `SYS_MSG_BROADCAST`:
/// ```text
/// bits 31:16 — failures (saturated to 0xFFFF)
/// bits 15:0  — successes (saturated to 0xFFFF)
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BroadcastResult {
    /// Number of processes that successfully received the message.
    pub succeeded: usize,
    /// Number of per-recipient delivery failures.
    pub failed: usize,
}

impl BroadcastResult {
    fn from_packed(word: usize) -> Self {
        Self { succeeded: word & 0xFFFF, failed: (word >> 16) & 0xFFFF }
    }
}

// ── Received message ─────────────────────────────────────────────────────────

/// A typed message dequeued from the calling process's inbox by [`msg_recv`].
#[derive(Clone, Debug)]
pub struct ReceivedMessage {
    /// The 16-byte KindId identifying the schema of the payload.
    pub kind: KindId,
    /// The message payload bytes.
    pub payload: alloc::vec::Vec<u8>,
}

// ── Syscall wrappers ─────────────────────────────────────────────────────────

/// Send one typed message directly to process `pid`.
///
/// The `kind` field identifies the schema of `payload`.  The kernel delivers
/// the message to the target process's inbox atomically.
///
/// # Errors
///
/// - `Errno::ESRCH`  — recipient process not found.
/// - `Errno::EAGAIN` — recipient inbox is full; retry or apply backpressure.
/// - `Errno::EINVAL` — null or misaligned pointer argument.
/// - `Errno::EFAULT` — user pointer is not accessible.
pub fn msg_send(pid: u32, kind: KindId, payload: &[u8]) -> Result<(), Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_MSG_SEND,
            pid as usize,
            kind.0.as_ptr() as usize,
            payload.as_ptr() as usize,
            payload.len(),
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(|_| ())
}

/// Dequeue one typed message from the calling process's inbox.
///
/// Writes the message KindId and payload bytes into the provided buffers.
/// Returns the actual payload length; if the actual length exceeds
/// `payload_buf.len()` the payload was truncated in the copy (the kind
/// and actual length are still valid).
///
/// Returns `Err(Errno::EAGAIN)` when the inbox is empty (non-blocking).
/// Use [`msg_recv_blocking`] to spin-wait, or use `SYS_FS_POLL` on the
/// inbox FD for event-driven receives.
pub fn msg_recv(kind_buf: &mut KindId, payload_buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_MSG_RECV,
            kind_buf.0.as_mut_ptr() as usize,
            payload_buf.as_mut_ptr() as usize,
            payload_buf.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

/// Block until a message is available in the calling process's inbox, then
/// dequeue and return it.
///
/// Allocates a heap buffer of `max_payload` bytes for the payload.  If the
/// actual payload is larger it will be truncated to `max_payload`; callers
/// that need the full message should retry with a larger buffer.
///
/// # Panics
///
/// Never panics.
pub fn msg_recv_blocking(max_payload: usize) -> ReceivedMessage {
    let mut kind = KindId([0u8; 16]);
    let mut payload_buf = alloc::vec![0u8; max_payload];

    let actual_len = loop {
        match msg_recv(&mut kind, &mut payload_buf) {
            Ok(n) => break n,
            Err(Errno::EAGAIN) => crate::syscall::yield_now(),
            Err(_) => crate::syscall::yield_now(),
        }
    };

    let copy_len = actual_len.min(max_payload);
    payload_buf.truncate(copy_len);

    ReceivedMessage { kind, payload: payload_buf }
}

/// Broadcast one typed message to all members of process group `pgid`.
///
/// Membership is snapshotted once at call time; fanout continues past
/// per-recipient failures.  The returned [`BroadcastResult`] summarises how
/// many deliveries succeeded and how many failed.
///
/// # Errors
///
/// - `Errno::EINVAL` — `pgid == 0` or invalid pointer arguments.
/// - `Errno::EFAULT` — user pointer is not accessible.
pub fn msg_broadcast(pgid: u32, kind: KindId, payload: &[u8]) -> Result<BroadcastResult, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_MSG_BROADCAST,
            pgid as usize,
            kind.0.as_ptr() as usize,
            payload.as_ptr() as usize,
            payload.len(),
            0,
            0,
        )
    };
    abi::errors::errno(ret).map(BroadcastResult::from_packed)
}

/// Open the calling process inbox as a pollable/readable VFS FD.
///
/// This uses the procfs path-open model (`/proc/self/inbox`) so callers can
/// combine inbox readiness with regular files and channel FDs in `SYS_FS_POLL`.
pub fn msg_inbox_open_self() -> Result<u32, Errno> {
    crate::syscall::vfs::vfs_open("/proc/self/inbox", O_RDONLY)
}
