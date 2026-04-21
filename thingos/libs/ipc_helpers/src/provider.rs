//! VFS provider server loop.
//!
//! [`ProviderLoop`] abstracts the port framing for a userland VFS provider.
//! It reads requests from the provider port, dispatches them to your
//! handler, and sends responses back to the kernel.
//!
//! # Usage
//!
//! ```ignore
//! use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
//! use abi::vfs_rpc::VfsRpcOp;
//! use abi::errors::Errno;
//!
//! fn run(vfs_read: u32) {
//!     let mut lp = ProviderLoop::new(vfs_read);
//!     loop {
//!         let req = match lp.next_request() {
//!             Ok(r) => r,
//!             Err(_) => break,  // port closed — exit cleanly
//!         };
//!         let resp = dispatch(&req);
//!         lp.send_response(req.resp_port, resp).ok();
//!     }
//! }
//!
//! fn dispatch(req: &ipc_helpers::provider::ProviderRequest) -> ProviderResponse {
//!     match req.op {
//!         VfsRpcOp::Lookup => ProviderResponse::ok_u64(1),
//!         VfsRpcOp::Read   => ProviderResponse::ok_bytes(b"Hello, world!\n"),
//!         VfsRpcOp::Stat   => ProviderResponse::ok_stat(0o100644, 14, 1),
//!         VfsRpcOp::Close  => ProviderResponse::ok_empty(),
//!         _                => ProviderResponse::err(Errno::ENOSYS),
//!     }
//! }
//! ```

use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp::{
    AttrGet, AttrList, AttrRemove, AttrSet, Close, DeviceCall, Lookup, Poll, Read, Readdir, Rename,
    Stat, SubscribeReady, UnsubscribeReady, Write,
};
use abi::vfs_rpc::{VFS_RPC_MAX_REQ, VFS_RPC_MAX_RESP, VfsRpcOp, VfsRpcReqHeader};
use stem::syscall::port::{port_recv, port_send_all, port_try_recv};

/// A decoded VFS RPC request from the kernel.
pub struct ProviderRequest {
    /// The response port the kernel is waiting on.  Pass this to
    /// [`ProviderLoop::send_response`].
    pub resp_port: u32,
    /// The operation code.
    pub op: VfsRpcOp,
    /// The op-specific payload bytes (after the 7-byte header).
    pub payload: alloc::vec::Vec<u8>,
}

/// A response to be sent back to the kernel.
#[derive(Debug, Clone)]
pub struct ProviderResponse {
    /// 0 = OK, non-zero = errno.
    pub status: u8,
    /// Payload bytes (only meaningful when `status == 0`).
    pub payload: alloc::vec::Vec<u8>,
}

impl ProviderResponse {
    /// Successful response with no payload (e.g. `Close`).
    pub fn ok_empty() -> Self {
        Self { status: 0, payload: alloc::vec![] }
    }

    /// Successful response with a raw byte payload.
    pub fn ok_bytes(data: &[u8]) -> Self {
        Self { status: 0, payload: alloc::vec::Vec::from(data) }
    }

    /// Successful `Lookup` response carrying a `u64` handle.
    pub fn ok_u64(value: u64) -> Self {
        let mut payload = alloc::vec![0u8; 8];
        payload[..8].copy_from_slice(&value.to_le_bytes());
        Self { status: 0, payload }
    }

    /// Successful `Stat` response carrying `mode`, `size`, `ino`.
    pub fn ok_stat(mode: u32, size: u64, ino: u64) -> Self {
        let mut payload = alloc::vec![0u8; 4 + 8 + 8];
        payload[0..4].copy_from_slice(&mode.to_le_bytes());
        payload[4..12].copy_from_slice(&size.to_le_bytes());
        payload[12..20].copy_from_slice(&ino.to_le_bytes());
        Self { status: 0, payload }
    }

    /// Successful `Read`/`Readdir` response.
    ///
    /// Prepends the 4-byte `bytes_read` count as required by the wire format.
    pub fn ok_read(data: &[u8]) -> Self {
        let mut payload = alloc::vec![0u8; 4 + data.len()];
        let len = data.len() as u32;
        payload[0..4].copy_from_slice(&len.to_le_bytes());
        payload[4..].copy_from_slice(data);
        Self { status: 0, payload }
    }

    /// Successful `Write` response with the number of bytes written.
    pub fn ok_written(n: u32) -> Self {
        let mut payload = alloc::vec![0u8; 4];
        payload[0..4].copy_from_slice(&n.to_le_bytes());
        Self { status: 0, payload }
    }

    /// Successful `Poll` response carrying the ready event mask.
    pub fn ok_poll(revents: u32) -> Self {
        let mut payload = alloc::vec![0u8; 4];
        payload[0..4].copy_from_slice(&revents.to_le_bytes());
        Self { status: 0, payload }
    }

    /// Successful `DeviceCall` response payload.
    ///
    /// Wire layout: `[ret_val: u32][out_len: u32][out_bytes...]`.
    pub fn ok_device_call(ret_val: u32, out_data: &[u8]) -> Self {
        let mut payload = alloc::vec![0u8; 8 + out_data.len()];
        payload[0..4].copy_from_slice(&ret_val.to_le_bytes());
        payload[4..8].copy_from_slice(&(out_data.len() as u32).to_le_bytes());
        payload[8..].copy_from_slice(out_data);
        Self { status: 0, payload }
    }

    /// Successful `AttrGet` response payload.
    ///
    /// Wire layout: `[val_type: u8][value_bytes...]`.
    pub fn ok_attr_get(val_type: u8, value: &[u8]) -> Self {
        let mut payload = alloc::vec![0u8; 1 + value.len()];
        payload[0] = val_type;
        payload[1..].copy_from_slice(value);
        Self { status: 0, payload }
    }

    /// Error response carrying an errno.
    pub fn err(e: Errno) -> Self {
        Self { status: e as u8, payload: alloc::vec![] }
    }
}

/// The VFS provider server loop.
///
/// Owns the read end of the provider port.  Call [`next_request`] in a
/// loop to receive decoded requests, then call [`send_response`] to reply.
pub struct ProviderLoop {
    read_handle: u32,
    recv_buf: alloc::vec::Vec<u8>,
    pending: alloc::vec::Vec<u8>,
}

impl ProviderLoop {
    /// Create a new loop bound to `vfs_read` — the read end of the provider
    /// port (created with `port_create` and passed to the supervisor).
    pub fn new(vfs_read: u32) -> Self {
        Self {
            read_handle: vfs_read,
            recv_buf: alloc::vec![0u8; VFS_RPC_MAX_REQ],
            pending: alloc::vec::Vec::new(),
        }
    }

    fn try_parse_one(&mut self) -> Result<Option<ProviderRequest>, Errno> {
        let hdr_size = core::mem::size_of::<VfsRpcReqHeader>();
        if self.pending.len() < hdr_size {
            return Ok(None);
        }

        // SAFETY: pending has at least hdr_size bytes; header is repr(C, packed).
        let hdr: VfsRpcReqHeader =
            unsafe { core::ptr::read_unaligned(self.pending.as_ptr() as *const VfsRpcReqHeader) };
        let op = match VfsRpcOp::from_u8(hdr.op) {
            Some(op) => op,
            None => {
                // Since this is userspace and we might not have a logger ready,
                // we'll just return EINVAL and hope the kernel log helps.
                return Err(Errno::EINVAL);
            }
        };
        // For debugging AttrList issue
        if hdr.op == 15 {
             // We recognized it, but let's be sure.
        }

        let payload_len = match op {
            Lookup => {
                if self.pending.len() < hdr_size + 4 {
                    return Ok(None);
                }
                let p = &self.pending[hdr_size..];
                let path_len = u32::from_le_bytes([p[0], p[1], p[2], p[3]]) as usize;
                4 + path_len
            }
            Read | Readdir => 20,
            Write => {
                if self.pending.len() < hdr_size + 20 {
                    return Ok(None);
                }
                let p = &self.pending[hdr_size..];
                let data_len = u32::from_le_bytes([p[16], p[17], p[18], p[19]]) as usize;
                20 + data_len
            }
            Stat | Close | UnsubscribeReady => 8,
            Poll | SubscribeReady => 12,
            DeviceCall => {
                let dc_size = core::mem::size_of::<abi::device::DeviceCall>();
                if self.pending.len() < hdr_size + 8 + dc_size {
                    return Ok(None);
                }
                let p = &self.pending[hdr_size..];
                // payload: [handle: u64][DeviceCall][in_data...]
                // DeviceCall.in_len is at byte offset 16 in the struct.
                let in_len_off = 8 + 16;
                let in_len = u32::from_le_bytes([
                    p[in_len_off],
                    p[in_len_off + 1],
                    p[in_len_off + 2],
                    p[in_len_off + 3],
                ]) as usize;
                8 + dc_size + in_len
            }
            Rename => {
                if self.pending.len() < hdr_size + 4 {
                    return Ok(None);
                }
                let p = &self.pending[hdr_size..];
                let old_len = u32::from_le_bytes([p[0], p[1], p[2], p[3]]) as usize;
                if self.pending.len() < hdr_size + 4 + old_len + 4 {
                    return Ok(None);
                }
                let q = &self.pending[hdr_size + 4 + old_len..];
                let new_len = u32::from_le_bytes([q[0], q[1], q[2], q[3]]) as usize;
                4 + old_len + 4 + new_len
            }
            AttrGet | AttrRemove => {
                if self.pending.len() < hdr_size + 8 + 2 {
                    return Ok(None);
                }
                let p = &self.pending[hdr_size..];
                let name_len = u16::from_le_bytes([p[8], p[9]]) as usize;
                8 + 2 + name_len
            }
            AttrSet => {
                if self.pending.len() < hdr_size + 8 + 8 {
                    return Ok(None);
                }
                let p = &self.pending[hdr_size..];
                let name_len = u16::from_le_bytes([p[8], p[9]]) as usize;
                let val_len = u32::from_le_bytes([p[12], p[13], p[14], p[15]]) as usize;
                8 + 8 + name_len + val_len
            }
            AttrList => 8,
        };

        if payload_len > (VFS_RPC_MAX_REQ - hdr_size) {
            return Err(Errno::EINVAL);
        }

        let frame_len = hdr_size + payload_len;
        if self.pending.len() < frame_len {
            return Ok(None);
        }

        let payload = self.pending[hdr_size..frame_len].to_vec();
        self.pending.drain(..frame_len);

        Ok(Some(ProviderRequest { resp_port: hdr.resp_port, op, payload }))
    }

    /// Try to receive the next request without blocking.
    ///
    /// Returns `Ok(None)` when no message is currently available (the port
    /// is empty).  Returns `Ok(Some(req))` when a request was decoded
    /// successfully.  Returns `Err` on a fatal port error (e.g. the kernel
    /// closed the request port — the provider should exit cleanly).
    ///
    /// Use this variant in event-loop drivers that must interleave VFS RPC
    /// handling with hardware polling (e.g. audio or network drivers).
    /// Blocking providers should use [`next_request`] instead.
    ///
    /// [`next_request`]: Self::next_request
    pub fn try_next_request(&mut self) -> Result<Option<ProviderRequest>, Errno> {
        loop {
            match self.try_parse_one() {
                Ok(Some(req)) => return Ok(Some(req)),
                Ok(None) => break,
                Err(Errno::EINVAL) => {
                    // Byte-stream ports can deliver fragmented/coalesced traffic.
                    // If framing gets out of sync, drop one byte and retry to
                    // recover alignment instead of returning EINVAL forever.
                    if self.pending.is_empty() {
                        break;
                    }
                    self.pending.drain(..1);
                }
                Err(e) => return Err(e),
            }
        }

        match port_try_recv(self.read_handle, &mut self.recv_buf) {
            Ok(0) => Ok(None),
            Ok(n) => {
                self.pending.extend_from_slice(&self.recv_buf[..n]);
                loop {
                    match self.try_parse_one() {
                        Ok(Some(req)) => return Ok(Some(req)),
                        Ok(None) => return Ok(None),
                        Err(Errno::EINVAL) => {
                            if self.pending.is_empty() {
                                return Ok(None);
                            }
                            self.pending.drain(..1);
                        }
                        Err(e) => return Err(e),
                    }
                }
            }
            Err(Errno::EAGAIN) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Block until the next request arrives and decode it.
    ///
    /// Returns `Err(Errno::EPIPE)` when the port is closed (provider
    /// should exit cleanly).
    pub fn next_request(&mut self) -> Result<ProviderRequest, Errno> {
        loop {
            match self.try_parse_one() {
                Ok(Some(req)) => return Ok(req),
                Ok(None) => {}
                Err(Errno::EINVAL) => {
                    if !self.pending.is_empty() {
                        self.pending.drain(..1);
                    }
                    continue;
                }
                Err(e) => return Err(e),
            }

            let n = port_recv(self.read_handle, &mut self.recv_buf)?;
            if n == 0 {
                return Err(Errno::EPIPE);
            }
            self.pending.extend_from_slice(&self.recv_buf[..n]);
        }
    }

    /// Send `response` back to the kernel on the given `resp_port`.
    pub fn send_response(&self, resp_port: u32, response: ProviderResponse) -> Result<(), Errno> {
        let total = 1 + response.payload.len();
        let mut buf = alloc::vec![0u8; total.min(VFS_RPC_MAX_RESP)];
        buf[0] = response.status;
        let payload_len = response.payload.len().min(buf.len() - 1);
        buf[1..1 + payload_len].copy_from_slice(&response.payload[..payload_len]);
        port_send_all(resp_port, &buf[..1 + payload_len]).map(|_| ())
    }
}
