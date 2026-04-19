//! Port send/recv wrappers.
//!
//! These helpers sit on top of the raw `stem::syscall::port` functions and
//! add framing, polling, and ergonomic error handling.

use abi::errors::Errno;
use stem::syscall::port::{
    port_close, port_recv, port_send_all, port_try_recv, PortHandle,
};

/// Send `data` over `handle`, retrying on `EAGAIN` until the ring has space.
/// Yields the current task between retries to avoid a busy-wait.
///
/// Returns `Err(Errno::EPIPE)` when the peer is gone.
pub fn send_all_blocking(handle: PortHandle, data: &[u8]) -> Result<(), Errno> {
    loop {
        match port_send_all(handle, data) {
            Ok(_) => return Ok(()),
            Err(Errno::EAGAIN) => stem::syscall::yield_now(),
            Err(e) => return Err(e),
        }
    }
}

/// Attempt a non-blocking receive.  Returns `Ok(None)` when no data is ready
/// instead of `Err(Errno::EAGAIN)`.
pub fn try_recv_opt(
    handle: PortHandle,
    buf: &mut [u8],
) -> Result<Option<usize>, Errno> {
    match port_try_recv(handle, buf) {
        Ok(n) => Ok(Some(n)),
        Err(Errno::EAGAIN) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Block on `handle` until a complete message arrives, then return the byte
/// count.  The underlying `port_recv` already blocks, so this is a thin
/// ergonomic wrapper.
pub fn recv_blocking(handle: PortHandle, buf: &mut [u8]) -> Result<usize, Errno> {
    port_recv(handle, buf)
}

// ── RAII port wrapper ──────────────────────────────────────────────────────

/// A thin RAII wrapper that closes a port handle on drop.
pub struct OwnedPort(pub PortHandle);

impl OwnedPort {
    pub fn new(handle: PortHandle) -> Self {
        Self(handle)
    }

    pub fn handle(&self) -> PortHandle {
        self.0
    }

    pub fn send(&self, data: &[u8]) -> Result<(), Errno> {
        send_all_blocking(self.0, data)
    }

    pub fn recv<'a>(&self, buf: &'a mut [u8]) -> Result<usize, Errno> {
        recv_blocking(self.0, buf)
    }

    /// Release the handle without closing it (e.g. to transfer ownership).
    pub fn into_inner(self) -> PortHandle {
        let h = self.0;
        core::mem::forget(self);
        h
    }
}

impl Drop for OwnedPort {
    fn drop(&mut self) {
        port_close(self.0).ok();
    }
}
