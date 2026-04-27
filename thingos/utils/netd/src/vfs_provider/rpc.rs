use alloc::vec::Vec;

use abi::errors::Errno;
use abi::vfs_rpc::{VFS_RPC_MAX_REQ, VfsRpcOp, VfsRpcReqHeader};
use smoltcp::iface::{Interface, SocketSet};
use smoltcp::wire::Ipv4Address;
use stem::syscall::port::{PortHandle, port_create, port_try_recv};
use stem::syscall::vfs::vfs_mount;
use stem::warn;

use super::wire::{send_err, send_resp, send_write_ok};
use super::{
    E_IO, E_NOTSUP, E_OK, ICMP_DYN_BASE, IpConfig, NetVfsProvider, TCP_DYN_BASE, UDP_DYN_BASE,
};
use crate::socket_api::SocketApi;

impl NetVfsProvider {
    /// Create the `/net/` provider (does NOT mount yet).
    ///
    /// Returns `None` if port creation fails.
    pub fn new(mac: [u8; 6], mtu: usize, link_up: bool) -> Option<Self> {
        let (req_write, req_read) = match port_create(VFS_RPC_MAX_REQ * 8) {
            Ok(p) => p,
            Err(e) => {
                warn!("NetVfsProvider: failed to create RPC port: {:?}", e);
                return None;
            }
        };

        debug!("NetVfsProvider: created ports (write={} read={})", req_write, req_read);

        Some(Self {
            req_read,
            req_write,
            mac,
            mtu,
            link_up,
            ip_config: None,
            rx_bytes: 0,
            tx_bytes: 0,
            rx_packets: 0,
            tx_packets: 0,
            req_buf: alloc::vec![0u8; VFS_RPC_MAX_REQ],
            pending: Vec::new(),
            dns_pending: None,
            dns_result: None,
            deferred_connects: Vec::new(),
        })
    }

    /// Mount the provider at the given path.
    /// Call this after creating the provider but before entering the RPC loop.
    /// Returns true on success, false on failure.
    pub fn mount(&self, mount_point: &str) -> bool {
        match vfs_mount(self.req_write, mount_point) {
            Ok(()) => {
                debug!(
                    "NetVfsProvider: mounted at {} (port w={} r={})",
                    mount_point, self.req_write, self.req_read
                );
                true
            }
            Err(e) => {
                warn!("NetVfsProvider: vfs_mount failed: {:?}", e);
                false
            }
        }
    }

    /// The port handle the RPC loop reads from (pass to `port_wait` / `port_len`).
    pub fn req_read_port(&self) -> PortHandle {
        self.req_read
    }

    /// Update the IP configuration after DHCP completes.
    pub fn set_ip_config(
        &mut self,
        ip: Ipv4Address,
        prefix_len: u8,
        gateway: Ipv4Address,
        dns_server: Ipv4Address,
    ) {
        self.ip_config = Some(IpConfig { ip, prefix_len, gateway, dns_server });
    }

    fn try_parse_one(&mut self) -> Result<Option<(PortHandle, VfsRpcOp, u16, Vec<u8>)>, Errno> {
        let hdr_size = core::mem::size_of::<VfsRpcReqHeader>();
        if self.pending.len() < hdr_size {
            return Ok(None);
        }

        let hdr: VfsRpcReqHeader =
            unsafe { core::ptr::read_unaligned(self.pending.as_ptr() as *const VfsRpcReqHeader) };
        let Some(op) = VfsRpcOp::from_u8(hdr.op) else {
            return Err(Errno::EINVAL);
        };

        let payload_len = match op {
            VfsRpcOp::Lookup => {
                if self.pending.len() < hdr_size + 4 {
                    return Ok(None);
                }
                let p = &self.pending[hdr_size..];
                let path_len = u32::from_le_bytes([p[0], p[1], p[2], p[3]]) as usize;
                4 + path_len
            }
            VfsRpcOp::Read | VfsRpcOp::Readdir => 20,
            VfsRpcOp::Write => {
                if self.pending.len() < hdr_size + 20 {
                    return Ok(None);
                }
                let p = &self.pending[hdr_size..];
                let data_len = u32::from_le_bytes([p[16], p[17], p[18], p[19]]) as usize;
                20 + data_len
            }
            VfsRpcOp::Stat | VfsRpcOp::Close | VfsRpcOp::UnsubscribeReady => 8,
            VfsRpcOp::Poll | VfsRpcOp::SubscribeReady => 12,
            VfsRpcOp::DeviceCall => return Err(Errno::ENOTSUP),
            VfsRpcOp::Rename => return Err(Errno::ENOTSUP),
            VfsRpcOp::AttrGet
            | VfsRpcOp::AttrSet
            | VfsRpcOp::AttrRemove
            | VfsRpcOp::AttrList
            | VfsRpcOp::Readlink
            | VfsRpcOp::ReadIntoFd => return Err(Errno::ENOTSUP),
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
        Ok(Some((hdr.resp_port as PortHandle, op, hdr.req_id, payload)))
    }

    pub fn try_next_request(
        &mut self,
    ) -> Result<Option<(PortHandle, VfsRpcOp, u16, Vec<u8>)>, Errno> {
        loop {
            match self.try_parse_one() {
                Ok(Some(req)) => return Ok(Some(req)),
                Ok(None) => break,
                Err(Errno::EINVAL) => {
                    if self.pending.is_empty() {
                        break;
                    }
                    self.pending.drain(..1);
                }
                Err(e) => return Err(e),
            }
        }

        match port_try_recv(self.req_read, &mut self.req_buf) {
            Ok(0) => Ok(None),
            Ok(n) => {
                self.pending.extend_from_slice(&self.req_buf[..n]);
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

    /// Process all pending VFS RPC messages (non-blocking drain).
    ///
    /// `socket_api` and `socket_set` are borrowed so that socket operations
    /// (open/connect/send/recv) can be dispatched inline without locking.
    ///
    /// DNS resolution is **not** performed here.  When a hostname is written
    /// to `/net/dns/lookup`, it is stored in `dns_pending` and the main loop
    /// drives the async query via `take_dns_pending` / `set_dns_result`.
    pub fn drain_rpcs<D: smoltcp::phy::Device>(
        &mut self,
        iface: &mut Interface,
        device: &mut D,
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) -> bool {
        let mut did_work = false;
        let mut count = 0;
        loop {
            if count >= 16 {
                did_work = true; // Ensure we loop again immediately but give main loop a chance
                break;
            }
            match self.try_next_request() {
                Ok(Some((resp_port, op, req_id, payload))) => {
                    trace!("NETD: handling RPC op={:?} payload_len={}", op, payload.len());
                    self.handle_decoded(
                        iface, device, socket_set, socket_api, resp_port, op, req_id, &payload,
                    );
                    did_work = true;
                    count += 1;
                }
                Ok(None) => break,
                Err(Errno::ENOTSUP) => {
                    did_work = true;
                    count += 1;
                }
                Err(_) => break,
            }
        }

        did_work
    }

    /// Process exactly **one** pending VFS RPC request and return whether
    /// any work was performed.  Unlike [`drain_rpcs`], this method processes
    /// at most one request per call so the caller can release any shared
    /// network-state lock between invocations, giving the network poll
    /// thread a chance to run `iface.poll` in between RPCs.
    ///
    /// Returns `true` if a request was decoded and dispatched.
    pub fn drain_one_rpc<D: smoltcp::phy::Device>(
        &mut self,
        iface: &mut Interface,
        device: &mut D,
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) -> bool {
        match self.try_next_request() {
            Ok(Some((resp_port, op, req_id, payload))) => {
                trace!("NETD RPC: op={:?} payload_len={}", op, payload.len());
                self.handle_decoded(
                    iface, device, socket_set, socket_api, resp_port, op, req_id, &payload,
                );
                true
            }
            Ok(None) => false,
            Err(Errno::ENOTSUP) => {
                // Unsupported op frame consumed; treat as work done so the
                // caller loops again to drain any remaining requests.
                true
            }
            Err(_) => false,
        }
    }

    // ── Async DNS interface for the main loop ────────────────────────────────

    /// Take the pending DNS lookup hostname, if any.  Returns `Some((hostname,
    /// dns_server))` when a new query should be started.
    pub fn take_dns_pending(&mut self) -> Option<(alloc::string::String, Ipv4Address)> {
        // Only start a new query if there is no result yet (avoids re-querying
        // when the result hasn't been consumed by a read).
        if self.dns_result.is_some() {
            return None;
        }
        let hostname = self.dns_pending.take()?;
        let dns_ip = match self.effective_dns_server() {
            Some(ip) => ip,
            None => {
                self.dns_result = Some("error".into());
                return None;
            }
        };
        Some((hostname, dns_ip))
    }

    /// Set the result of an async DNS lookup.  The next read from
    /// `/net/dns/lookup` will return this value.
    pub fn set_dns_result(&mut self, result: alloc::string::String) {
        self.dns_result = Some(result);
    }

    /// Take any pending hostname that needs DNS resolution for a TCP connect.
    /// Returns `Some((hostname, dns_server))` together with the deferred
    /// connect metadata when a deferred connect needs resolution.
    pub fn take_deferred_connect_pending(
        &mut self,
    ) -> Option<(alloc::string::String, Ipv4Address)> {
        if self.deferred_connects.is_empty() {
            return None;
        }
        // Return the hostname of the first deferred connect
        let hostname = self.deferred_connects[0].hostname.clone();
        let dns_ip = self.effective_dns_server()?;
        Some((hostname, dns_ip))
    }

    /// Complete a deferred TCP connect with the resolved IP address.
    pub fn complete_deferred_connect(
        &mut self,
        resolved_ip: Option<Ipv4Address>,
        iface: &mut Interface,
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) {
        if self.deferred_connects.is_empty() {
            return;
        }
        let dc = self.deferred_connects.remove(0);
        if let Some(ip) = resolved_ip {
            let r =
                socket_api.handle_connect_existing(iface, socket_set, dc.api_handle, ip, dc.port);
            if r {
                send_write_ok(dc.resp_port, dc.req_id, dc.text_len as u32);
            } else {
                send_err(dc.resp_port, dc.req_id, E_IO);
            }
        } else {
            send_err(dc.resp_port, dc.req_id, E_IO);
        }
    }

    // ── RPC dispatch ─────────────────────────────────────────────────────────

    pub fn handle_decoded<D: smoltcp::phy::Device>(
        &mut self,
        iface: &mut Interface,
        device: &mut D,
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
        resp_port: PortHandle,
        op: VfsRpcOp,
        req_id: u16,
        payload: &[u8],
    ) {
        match op {
            VfsRpcOp::Lookup => self.op_lookup(resp_port, req_id, payload),
            VfsRpcOp::Read => self.op_read(resp_port, req_id, payload, socket_set, socket_api),
            VfsRpcOp::Write => {
                self.op_write(resp_port, req_id, payload, iface, device, socket_set, socket_api)
            }
            VfsRpcOp::Readdir => self.op_readdir(resp_port, req_id, payload, socket_api),
            VfsRpcOp::Stat => self.op_stat(resp_port, req_id, payload, socket_api),
            VfsRpcOp::Close => self.op_close(resp_port, req_id, payload, socket_set, socket_api),
            VfsRpcOp::Poll => self.op_poll(resp_port, req_id, payload, socket_set, socket_api),
            VfsRpcOp::DeviceCall => send_err(resp_port, req_id, E_NOTSUP as u8),
            VfsRpcOp::Rename => send_err(resp_port, req_id, E_NOTSUP as u8),
            VfsRpcOp::SubscribeReady => send_resp(resp_port, req_id, &[E_OK as u8]),
            VfsRpcOp::UnsubscribeReady => send_resp(resp_port, req_id, &[E_OK as u8]),
            VfsRpcOp::AttrGet
            | VfsRpcOp::AttrSet
            | VfsRpcOp::AttrRemove
            | VfsRpcOp::AttrList
            | VfsRpcOp::Readlink
            | VfsRpcOp::ReadIntoFd => send_err(resp_port, req_id, E_NOTSUP as u8),
        }
    }

    pub fn push_notifications(&mut self, socket_set: &mut SocketSet, socket_api: &mut SocketApi) {
        socket_api.push_notifications(socket_set, self.req_write, TCP_DYN_BASE);
        socket_api.push_notifications(socket_set, self.req_write, UDP_DYN_BASE);
        socket_api.push_notifications(socket_set, self.req_write, ICMP_DYN_BASE);
    }
}
