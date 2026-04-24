//! `/net/` VFS provider for netd (issue #541)
//!
//! Implements the full `/net/` filesystem tree as a userland VFS provider.
//! Applications interact with the network stack using ordinary file operations.
//!
//! ## Tree layout
//! ```text
//! /net/
//! ├── interfaces/
//! │   └── eth0/
//! │       ├── status    ← read:  multiline interface state
//! │       ├── addr      ← read/write: CIDR "192.168.1.50/24\n"
//! │       ├── flags     ← write: "up" / "down"
//! │       ├── mtu       ← read/write: decimal MTU
//! │       ├── stats     ← read:  rx/tx counts
//! │       └── events    ← pollable link-state stream
//! ├── routes            ← read:  table; write: "add default via 1.2.3.4 dev eth0"
//! ├── tcp/
//! │   ├── new           ← read:  allocates socket, returns id
//! │   └── <id>/
//! │       ├── ctl       ← write: "connect HOST_OR_IP PORT" / "listen PORT [BACKLOG]" / "shutdown read|write|both" / "ttl N" / "linger off|SECS" / "only_v6 0|1" / "close"
//! │       ├── data      ← read/write: TCP byte stream
//! │       ├── accept    ← read:  "<conn_id> <ip> <port>\n" (listener sockets only)
//! │       ├── status    ← read:  state text
//! │       └── events    ← pollable connection events
//! ├── udp/
//! │   ├── new           ← read:  allocates socket, returns id
//! │   └── <id>/
//! │       ├── ctl       ← write: "bind PORT" / "connect IP PORT" / "broadcast 0|1" / "ttl N" / "multicast_loop_v4 0|1" / "multicast_ttl_v4 N" / "multicast_loop_v6 0|1" / "join_multicast_v4 G IFACE" / "leave_multicast_v4 G IFACE" / "join_multicast_v6 G IFIDX" / "leave_multicast_v6 G IFIDX" / "close"
//! │       ├── data      ← write: [4: dest_ipv4][2: dest_port_le][4: len_le][payload]
//! │       │               read:  [4: src_ipv4][2: src_port_le][4: len_le][payload]
//! │       └── status    ← read:  state text, including `broadcast: true|false`
//! └── dns/
//!     └── lookup        ← write: hostname; read: dotted-decimal IPv4 address
//! ```
//!
//! ## IPC substrate — migration note
//!
//! netd intentionally keeps the inline VFS RPC decoder in `drain_rpcs` /
//! `handle_one` for now.  The current `op_*` methods write responses directly
//! to the reply port and are called from a poll loop that also drives synchronous
//! DNS completion, so a full `ProviderLoop` conversion would be a larger refactor.
//!
//! Architecture expectation:
//! - keep request draining non-blocking (`port_try_recv`) so networking work and
//!   VFS servicing continue to interleave;
//! - keep response framing compatible with `VfsRpcReqHeader` (covered by unit
//!   tests in this module);
//! - prefer `ipc_helpers::provider::ProviderLoop` for new providers (see
//!   `drivers/virtio_netd/src/vfs_provider.rs`).
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use abi::errors::Errno;
use abi::vfs_rpc::{VFS_RPC_MAX_REQ, VfsRpcOp, VfsRpcReqHeader};
use smoltcp::iface::{Interface, SocketSet};
use smoltcp::wire::{IpAddress, IpCidr, Ipv4Address};
use stem::syscall::port::{PortHandle, port_create, port_send_all, port_try_recv};
use stem::syscall::vfs::vfs_mount;
use stem::{info, warn};

/// A TCP connect request that is waiting for DNS resolution.
#[derive(Clone)]
pub struct DeferredConnect {
    /// The API handle of the TCP socket awaiting connection.
    pub api_handle: u32,
    /// The original hostname (for DNS lookup).
    pub hostname: alloc::string::String,
    /// The destination port.
    pub port: u16,
    /// The response port to send the write result to.
    pub resp_port: PortHandle,
    /// The request ID to echo in the response.
    pub req_id: u16,
    /// Original write length (for the WriteResult::Ok response).
    pub text_len: usize,
}

use crate::socket_api::SocketApi;

// ── errno shorthands ─────────────────────────────────────────────────────────

pub const E_OK: u8 = 0;
const E_NOENT: u8 = 2;
const E_IO: u8 = 5;
const E_INVAL: u8 = 22;
const E_ROFS: u8 = 30;
const E_NOTSUP: u8 = 38;

// ── file-type bits ────────────────────────────────────────────────────────────

const S_IFDIR: u32 = 0o040_000;
const S_IFREG: u32 = 0o100_000;

// ── poll interest bits ───────────────────────────────────────────────────────

const POLLIN: u32 = 0x0001;
#[allow(dead_code)]
const POLLOUT: u32 = 0x0004;

// ── static handle constants ──────────────────────────────────────────────────

const HANDLE_ROOT: u64 = 1;
const HANDLE_INTERFACES_DIR: u64 = 2;
const HANDLE_ETH0_DIR: u64 = 3;
const HANDLE_ETH0_STATUS: u64 = 4;
const HANDLE_ETH0_ADDR: u64 = 5;
const HANDLE_ETH0_FLAGS: u64 = 6;
const HANDLE_ETH0_MTU: u64 = 7;
const HANDLE_ETH0_STATS: u64 = 8;
const HANDLE_ETH0_EVENTS: u64 = 9;
const HANDLE_ROUTES: u64 = 10;
const HANDLE_TCP_DIR: u64 = 11;
const HANDLE_TCP_NEW: u64 = 12;
const HANDLE_UDP_DIR: u64 = 13;
const HANDLE_UDP_NEW: u64 = 14;
const HANDLE_DNS_DIR: u64 = 15;
const HANDLE_DNS_LOOKUP: u64 = 16;
const HANDLE_DNS_SERVER: u64 = 17;
const HANDLE_ICMP_DIR: u64 = 18;
const HANDLE_ICMP_NEW: u64 = 19;

/// Dynamic handle base for TCP socket sub-files.
/// Handle = TCP_DYN_BASE | ((api_handle as u64) << 8) | subfile_id
pub const TCP_DYN_BASE: u64 = 0x0001_0000;
pub const UDP_DYN_BASE: u64 = 0x0100_0000;
pub const ICMP_DYN_BASE: u64 = 0x0200_0000;

// subfile IDs
const SF_DIR: u8 = 0;
const SF_CTL: u8 = 1;
const SF_DATA: u8 = 2;
const SF_STATUS: u8 = 3;
const SF_EVENTS: u8 = 4;
/// Listener-only: returns accepted connection id on read.
const SF_ACCEPT: u8 = 5;

// ── IP config ────────────────────────────────────────────────────────────────

/// Snapshot of the current IPv4 configuration published by DHCP or static
/// assignment.  Stored inside `NetVfsProvider` and updated when the interface
/// configuration changes.
#[derive(Clone, Copy)]
pub struct IpConfig {
    pub ip: Ipv4Address,
    pub prefix_len: u8,
    pub gateway: Ipv4Address,
    pub dns_server: Ipv4Address,
}

// ── main provider struct ──────────────────────────────────────────────────────

/// Userland VFS provider that serves the `/net/` namespace.
pub struct NetVfsProvider {
    /// Read-end of the VFS RPC port (provider reads requests from here).
    req_read: PortHandle,
    /// Write-end of the VFS RPC port (used for vfs_notify).
    pub req_write: PortHandle,
    /// MAC address of the first interface (eth0).
    pub mac: [u8; 6],
    /// MTU of eth0.
    pub mtu: usize,
    /// Current link state.
    pub link_up: bool,
    /// Current IP configuration (populated after DHCP).
    pub ip_config: Option<IpConfig>,
    /// Monoton rx/tx byte counters for stats.
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    /// RPC request staging buffer.
    req_buf: Vec<u8>,
    /// Pending undecoded RPC bytes. Provider ports are byte streams and may
    /// split or coalesce requests across recv calls.
    pending: Vec<u8>,
    /// Pending DNS hostname to resolve (written by op_write to HANDLE_DNS_LOOKUP).
    dns_pending: Option<alloc::string::String>,
    /// Resolved DNS result (dotted-decimal IPv4 or error text).
    dns_result: Option<alloc::string::String>,
    /// TCP connect requests deferred while waiting for DNS resolution.
    pub deferred_connects: Vec<DeferredConnect>,
}

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
            | VfsRpcOp::Readlink => return Err(Errno::ENOTSUP),
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

    pub fn try_next_request(&mut self) -> Result<Option<(PortHandle, VfsRpcOp, u16, Vec<u8>)>, Errno> {
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
            | VfsRpcOp::Readlink => send_err(resp_port, req_id, E_NOTSUP as u8),
        }
    }

    pub fn push_notifications(&mut self, socket_set: &mut SocketSet, socket_api: &mut SocketApi) {
        socket_api.push_notifications(socket_set, self.req_write, TCP_DYN_BASE);
        socket_api.push_notifications(socket_set, self.req_write, UDP_DYN_BASE);
        socket_api.push_notifications(socket_set, self.req_write, ICMP_DYN_BASE);
    }

    fn write_result_from_send(&mut self, response: &[u8], framing_overhead: usize) -> WriteResult {
        if response.len() < 4 {
            return WriteResult::Error;
        }
        let sent = u16::from_le_bytes([response[2], response[3]]) as usize;
        if sent == 0 {
            return WriteResult::Error;
        }
        self.tx_bytes += sent as u64;
        self.tx_packets += 1;
        WriteResult::Ok(framing_overhead + sent)
    }

    // ── Lookup ────────────────────────────────────────────────────────────────

    pub fn op_lookup(&self, resp_port: PortHandle, req_id: u16, payload: &[u8]) {
        if payload.len() < 4 {
            send_err(resp_port, req_id, E_INVAL);
            return;
        }
        let path_len =
            u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]) as usize;
        if payload.len() < 4 + path_len {
            send_err(resp_port, req_id, E_INVAL);
            return;
        }
        let path = match core::str::from_utf8(&payload[4..4 + path_len]) {
            Ok(s) => s.trim_matches('/'),
            Err(_) => {
                send_err(resp_port, req_id, E_INVAL);
                return;
            }
        };

        trace!("NETD: lookup path='{}'", path);
        match self.resolve_path(path) {
            Some(handle) => {
                trace!("NETD: lookup path='{}' -> handle {}", path, handle);
                send_handle(resp_port, req_id, handle);
            }
            None => {
                warn!("NETD: lookup path='{}' failed", path);
                send_err(resp_port, req_id, E_NOENT);
            }
        }
    }

    // ── Read ─────────────────────────────────────────────────────────────────

    fn op_read(
        &mut self,
        resp_port: PortHandle,
        req_id: u16,
        payload: &[u8],
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) {
        if payload.len() < 20 {
            send_err(resp_port, req_id, E_INVAL);
            return;
        }
        let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
        let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap());
        let len = u32::from_le_bytes(payload[16..20].try_into().unwrap()) as usize;

        let data = self.read_handle(handle, offset, len, socket_set, socket_api);
        match data {
            ReadResult::Data(bytes) => send_data(resp_port, req_id, &bytes),
            ReadResult::EOF => send_data(resp_port, req_id, &[]),
            ReadResult::Again => send_err(resp_port, req_id, 11), // EAGAIN
            ReadResult::Error => send_err(resp_port, req_id, E_IO),
            ReadResult::NotSupported => send_err(resp_port, req_id, E_NOTSUP as u8),
        }
    }

    // ── Write ─────────────────────────────────────────────────────────────────

    fn op_write<D: smoltcp::phy::Device>(
        &mut self,
        resp_port: PortHandle,
        req_id: u16,
        payload: &[u8],
        iface: &mut Interface,
        device: &mut D,
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) {
        if payload.len() < 20 {
            send_err(resp_port, req_id, E_INVAL);
            return;
        }
        let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
        let _offset = u64::from_le_bytes(payload[8..16].try_into().unwrap());
        let data_len = u32::from_le_bytes(payload[16..20].try_into().unwrap()) as usize;
        if payload.len() < 20 + data_len {
            send_err(resp_port, req_id, E_INVAL);
            return;
        }
        let data = &payload[20..20 + data_len];

        let result = self
            .write_handle(handle, data, resp_port, req_id, iface, device, socket_set, socket_api);
        match result {
            WriteResult::Ok(n) => send_write_ok(resp_port, req_id, n as u32),
            WriteResult::Error => send_err(resp_port, req_id, E_IO),
            WriteResult::ReadOnly => send_err(resp_port, req_id, E_ROFS),
            WriteResult::NotSupported => send_err(resp_port, req_id, E_NOTSUP as u8),
            WriteResult::Deferred => {
                // If deferred, we need to update the last deferred connect to have the req_id
                if let Some(dc) = self.deferred_connects.last_mut() {
                    dc.req_id = req_id;
                }
            }
        }
    }

    // ── Readdir ───────────────────────────────────────────────────────────────

    fn op_readdir(
        &self,
        resp_port: PortHandle,
        req_id: u16,
        payload: &[u8],
        socket_api: &SocketApi,
    ) {
        if payload.len() < 20 {
            send_err(resp_port, req_id, E_INVAL);
            return;
        }
        let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
        let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap()) as usize;
        let max_bytes = u32::from_le_bytes(payload[16..20].try_into().unwrap()) as usize;

        let entries = self.list_dir(handle, socket_api);
        let entries_slice = entries.as_slice();

        let start = offset.min(entries_slice.len());
        let tail = &entries_slice[start..];

        let mut out: Vec<u8> = Vec::new();
        for (name, ino, file_type) in tail {
            let name_bytes = name.as_bytes();
            let name_len = name_bytes.len().min(255) as u8;
            let entry_size = 10 + name_len as usize;
            if out.len() + entry_size > max_bytes {
                break;
            }
            out.extend_from_slice(&ino.to_le_bytes());
            out.push(*file_type); // DT_DIR=4 or DT_REG=8
            out.push(name_len);
            out.extend_from_slice(&name_bytes[..name_len as usize]);
        }

        send_data(resp_port, req_id, &out);
    }

    // ── Stat ─────────────────────────────────────────────────────────────────

    fn op_stat(&self, resp_port: PortHandle, req_id: u16, payload: &[u8], socket_api: &SocketApi) {
        if payload.len() < 8 {
            send_err(resp_port, req_id, E_INVAL);
            return;
        }
        let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());

        let (mode, size) = self.stat_handle(handle, socket_api);
        if mode == 0 {
            send_err(resp_port, req_id, E_NOENT);
            return;
        }

        let mut resp = [0u8; 21]; // 1 + 4 + 8 + 8
        resp[0] = E_OK as u8;
        resp[1..5].copy_from_slice(&mode.to_le_bytes());
        resp[5..13].copy_from_slice(&(size as u64).to_le_bytes());
        resp[13..21].copy_from_slice(&handle.to_le_bytes());
        send_resp(resp_port, req_id, &resp);
    }

    // ── Close ─────────────────────────────────────────────────────────────────

    fn op_close(
        &self,
        resp_port: PortHandle,
        req_id: u16,
        payload: &[u8],
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) {
        if payload.len() < 8 {
            send_err(resp_port, req_id, E_INVAL);
            return;
        }
        let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());

        // If it's a TCP or UDP socket *directory* handle, close the underlying socket.
        if handle >= TCP_DYN_BASE && handle < UDP_DYN_BASE {
            let sf = (handle & 0xFF) as u8;
            let api_handle = ((handle - TCP_DYN_BASE) >> 8) as u32;
            if sf == SF_DIR {
                let _ = socket_api.handle_close(socket_set, api_handle);
            }
        } else if handle >= UDP_DYN_BASE && handle < ICMP_DYN_BASE {
            let sf = (handle & 0xFF) as u8;
            let api_handle = ((handle - UDP_DYN_BASE) >> 8) as u32;
            if sf == SF_DIR {
                let _ = socket_api.handle_close(socket_set, api_handle);
            }
        } else if handle >= ICMP_DYN_BASE {
            let sf = (handle & 0xFF) as u8;
            let api_handle = ((handle - ICMP_DYN_BASE) >> 8) as u32;
            if sf == SF_DIR {
                let _ = socket_api.handle_close(socket_set, api_handle);
            }
        }

        send_resp(resp_port, req_id, &[E_OK as u8]);
    }

    // ── Poll ─────────────────────────────────────────────────────────────────

    fn op_poll(
        &self,
        resp_port: PortHandle,
        req_id: u16,
        payload: &[u8],
        socket_set: &mut SocketSet,
        socket_api: &SocketApi,
    ) {
        if payload.len() < 12 {
            send_err(resp_port, req_id, E_INVAL);
            return;
        }
        let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
        let _events = u32::from_le_bytes(payload[8..12].try_into().unwrap());

        let revents = self.poll_handle(handle, socket_set, socket_api);

        trace!("NETD: op_poll handle={} revents=0x{:04x}", handle, revents);

        let mut resp = [0u8; 5];
        resp[0] = E_OK as u8;
        resp[1..5].copy_from_slice(&revents.to_le_bytes());
        send_resp(resp_port, req_id, &resp);
    }

    // ── Path resolution ───────────────────────────────────────────────────────

    /// Resolve a path (relative to mount point, leading/trailing slashes stripped)
    /// to a u64 handle.  Returns `None` for unknown paths.
    fn resolve_path(&self, path: &str) -> Option<u64> {
        match path {
            "" | "/" => Some(HANDLE_ROOT),
            "interfaces" => Some(HANDLE_INTERFACES_DIR),
            "interfaces/eth0" => Some(HANDLE_ETH0_DIR),
            "interfaces/eth0/status" => Some(HANDLE_ETH0_STATUS),
            "interfaces/eth0/addr" => Some(HANDLE_ETH0_ADDR),
            "interfaces/eth0/flags" => Some(HANDLE_ETH0_FLAGS),
            "interfaces/eth0/mtu" => Some(HANDLE_ETH0_MTU),
            "interfaces/eth0/stats" => Some(HANDLE_ETH0_STATS),
            "interfaces/eth0/events" => Some(HANDLE_ETH0_EVENTS),
            "routes" => Some(HANDLE_ROUTES),
            "tcp" => Some(HANDLE_TCP_DIR),
            "tcp/new" => Some(HANDLE_TCP_NEW),
            "udp" => Some(HANDLE_UDP_DIR),
            "udp/new" => Some(HANDLE_UDP_NEW),
            "icmp" => Some(HANDLE_ICMP_DIR),
            "icmp/new" => Some(HANDLE_ICMP_NEW),
            "dns" => Some(HANDLE_DNS_DIR),
            "dns/lookup" => Some(HANDLE_DNS_LOOKUP),
            "dns/server" => Some(HANDLE_DNS_SERVER),
            other => self.resolve_dynamic_path(other),
        }
    }

    fn resolve_dynamic_path(&self, path: &str) -> Option<u64> {
        // tcp/<id>[/<subfile>]
        if let Some(rest) = path.strip_prefix("tcp/") {
            let (id_str, sub) = match rest.find('/') {
                Some(pos) => (&rest[..pos], &rest[pos + 1..]),
                None => (rest, ""),
            };
            let id: u32 = id_str.parse().ok()?;
            let sf: u8 = match sub {
                "" => SF_DIR,
                "ctl" => SF_CTL,
                "data" => SF_DATA,
                "status" => SF_STATUS,
                "events" => SF_EVENTS,
                "accept" => SF_ACCEPT,
                _ => return None,
            };
            return Some(TCP_DYN_BASE | ((id as u64) << 8) | sf as u64);
        }

        // udp/<id>[/<subfile>]
        if let Some(rest) = path.strip_prefix("udp/") {
            let (id_str, sub) = match rest.find('/') {
                Some(pos) => (&rest[..pos], &rest[pos + 1..]),
                None => (rest, ""),
            };
            let id: u32 = id_str.parse().ok()?;
            let sf: u8 = match sub {
                "" => SF_DIR,
                "ctl" => SF_CTL,
                "data" => SF_DATA,
                "status" => SF_STATUS,
                _ => return None,
            };
            return Some(UDP_DYN_BASE | ((id as u64) << 8) | sf as u64);
        }

        // icmp/<id>[/<subfile>]
        if let Some(rest) = path.strip_prefix("icmp/") {
            let (id_str, sub) = match rest.find('/') {
                Some(pos) => (&rest[..pos], &rest[pos + 1..]),
                None => (rest, ""),
            };
            let id: u32 = id_str.parse().ok()?;
            let sf: u8 = match sub {
                "" => SF_DIR,
                "ctl" => SF_CTL,
                "data" => SF_DATA,
                "status" => SF_STATUS,
                _ => return None,
            };
            return Some(ICMP_DYN_BASE | ((id as u64) << 8) | sf as u64);
        }

        None
    }

    // ── Handle reads ─────────────────────────────────────────────────────────

    fn read_handle(
        &mut self,
        handle: u64,
        offset: u64,
        len: usize,
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) -> ReadResult {
        match handle {
            HANDLE_ETH0_STATUS => ReadResult::text_offset(&self.eth0_status(), offset),
            HANDLE_ETH0_ADDR => ReadResult::text_offset(&self.eth0_addr_text(), offset),
            HANDLE_ETH0_MTU => ReadResult::text_offset(&alloc::format!("{}\n", self.mtu), offset),
            HANDLE_ETH0_STATS => ReadResult::text_offset(&self.eth0_stats(), offset),
            HANDLE_ETH0_EVENTS => {
                // Events are single-shot; subsequent reads return EOF until next event.
                if offset == 0 {
                    let s = if self.link_up { "link-up\n" } else { "link-down\n" };
                    ReadResult::Data(s.as_bytes().to_vec())
                } else {
                    ReadResult::EOF
                }
            }
            HANDLE_ROUTES => ReadResult::text_offset(&self.routes_text(socket_api), offset),
            HANDLE_DNS_SERVER => ReadResult::text_offset(&self.dns_server_text(), offset),
            // tcp/new: allocate a new TCP socket, return its id as text
            HANDLE_TCP_NEW => {
                if offset > 0 {
                    return ReadResult::EOF;
                }
                let Some(buf_idx) = socket_api.alloc_buffer() else {
                    warn!("NetVfsProvider: out of socket buffers for tcp/new");
                    return ReadResult::Error;
                };
                match socket_api.alloc_tcp_socket_raw(socket_set, buf_idx) {
                    Some(id) => ReadResult::Data(alloc::format!("{}\n", id).into_bytes()),
                    None => ReadResult::Error,
                }
            }
            // udp/new: allocate a new UDP socket
            HANDLE_UDP_NEW => {
                if offset > 0 {
                    return ReadResult::EOF;
                }
                let Some(buf_idx) = socket_api.alloc_buffer() else {
                    warn!("NetVfsProvider: out of socket buffers for udp/new");
                    return ReadResult::Error;
                };
                let api_handle = socket_api.alloc_udp_socket_raw(socket_set, buf_idx);
                match api_handle {
                    Some(id) => {
                        let text = alloc::format!("{}\n", id);
                        ReadResult::Data(text.into_bytes())
                    }
                    None => ReadResult::Error,
                }
            }
            HANDLE_ICMP_NEW => {
                if offset > 0 {
                    return ReadResult::EOF;
                }
                let Some(buf_idx) = socket_api.alloc_buffer() else {
                    warn!("NetVfsProvider: out of socket buffers for icmp/new");
                    return ReadResult::Error;
                };
                match socket_api.alloc_icmp_socket_raw(socket_set, buf_idx) {
                    Some(id) => ReadResult::Data(alloc::format!("{}\n", id).into_bytes()),
                    None => ReadResult::Error,
                }
            }
            // Dynamic TCP data
            h if h >= TCP_DYN_BASE && h < UDP_DYN_BASE => {
                let sf = (h & 0xFF) as u8;
                let api_handle = ((h - TCP_DYN_BASE) >> 8) as u32;
                self.read_tcp(api_handle, sf, offset, len, socket_set, socket_api)
            }
            // Dynamic UDP data
            h if h >= UDP_DYN_BASE && h < ICMP_DYN_BASE => {
                let sf = (h & 0xFF) as u8;
                let api_handle = ((h - UDP_DYN_BASE) >> 8) as u32;
                self.read_udp(api_handle, sf, offset, socket_set, socket_api)
            }
            h if h >= ICMP_DYN_BASE => {
                let sf = (h & 0xFF) as u8;
                let api_handle = ((h - ICMP_DYN_BASE) >> 8) as u32;
                self.read_icmp(api_handle, sf, offset, socket_set, socket_api)
            }
            // Directories are not readable as byte streams
            HANDLE_ROOT
            | HANDLE_INTERFACES_DIR
            | HANDLE_ETH0_DIR
            | HANDLE_TCP_DIR
            | HANDLE_UDP_DIR
            | HANDLE_ICMP_DIR
            | HANDLE_DNS_DIR => ReadResult::NotSupported,
            // dns/lookup: returns the resolved IP (EAGAIN if not yet resolved)
            HANDLE_DNS_LOOKUP => {
                if offset > 0 {
                    return ReadResult::EOF;
                }
                match &self.dns_result {
                    Some(result) if result != "error" => {
                        let text = result.clone() + "\n";
                        self.dns_result = None; // consume result
                        self.dns_pending = None;
                        ReadResult::Data(text.into_bytes())
                    }
                    Some(_) => {
                        self.dns_result = None;
                        self.dns_pending = None;
                        ReadResult::Error
                    }
                    None => ReadResult::Again, // EAGAIN: resolution in progress
                }
            }
            _ => ReadResult::Error,
        }
    }

    fn read_tcp(
        &mut self,
        api_handle: u32,
        sf: u8,
        offset: u64,
        len: usize,
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) -> ReadResult {
        match sf {
            SF_DIR => ReadResult::NotSupported,
            SF_STATUS => {
                ReadResult::text_offset(&socket_api.tcp_status_text(api_handle, socket_set), offset)
            }
            SF_DATA => {
                if len == 0 {
                    return ReadResult::Data(Vec::new());
                }
                let recv_len = len.min(u16::MAX as usize) as u16;
                let recv = socket_api.handle_recv(socket_set, api_handle, recv_len);
                // handle_recv returns [resp_type: u16][data...]
                if recv.len() < 2 {
                    return ReadResult::Error;
                }
                let resp_type = u16::from_le_bytes([recv[0], recv[1]]);
                match resp_type {
                    crate::socket_api::RESP_DATA => {
                        self.rx_bytes += (recv.len() - 2) as u64;
                        self.rx_packets += 1;
                        ReadResult::Data(recv[2..].to_vec())
                    }
                    crate::socket_api::RESP_EMPTY => ReadResult::Again,
                    crate::socket_api::RESP_CLOSED => ReadResult::EOF,
                    _ => ReadResult::Error,
                }
            }
            SF_EVENTS => {
                ReadResult::text_offset(&socket_api.tcp_events_text(api_handle, socket_set), offset)
            }
            SF_ACCEPT => {
                // Listener sockets: returns "<conn_id> <ip> <port>\n" when a
                // connection is ready, or EAGAIN when none is queued.
                let Some(buf_idx) = socket_api.alloc_buffer() else {
                    return ReadResult::Error;
                };
                let result = socket_api.handle_accept(socket_set, api_handle, 0, buf_idx);
                if result.len() < 2 {
                    socket_api.free_buffer(buf_idx);
                    return ReadResult::Error;
                }
                let resp_type = u16::from_le_bytes([result[0], result[1]]);
                match resp_type {
                    crate::socket_api::RESP_ACCEPT => {
                        // [2: RESP_ACCEPT][4: conn_handle][4: remote_ip][2: remote_port]
                        if result.len() < 12 {
                            return ReadResult::Error;
                        }
                        let conn_handle = u32::from_le_bytes(result[2..6].try_into().unwrap());
                        let ip = &result[6..10];
                        let port = u16::from_le_bytes(result[10..12].try_into().unwrap());
                        let text = alloc::format!(
                            "{} {}.{}.{}.{} {}\n",
                            conn_handle,
                            ip[0],
                            ip[1],
                            ip[2],
                            ip[3],
                            port
                        );
                        ReadResult::Data(text.into_bytes())
                    }
                    crate::socket_api::RESP_EMPTY => ReadResult::Again,
                    _ => ReadResult::Error,
                }
            }
            _ => ReadResult::Error,
        }
    }

    fn read_udp(
        &mut self,
        api_handle: u32,
        sf: u8,
        offset: u64,
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) -> ReadResult {
        match sf {
            SF_DIR => ReadResult::NotSupported,
            SF_STATUS => ReadResult::text_offset(&socket_api.udp_status_text(api_handle), offset),
            SF_DATA => {
                let recv = socket_api.handle_udp_recv_from(socket_set, api_handle);
                if recv.len() < 2 {
                    return ReadResult::Error;
                }
                let resp_type = u16::from_le_bytes([recv[0], recv[1]]);
                match resp_type {
                    crate::socket_api::RESP_DATA => {
                        // encode_udp_data format: [2: RESP_DATA][4: src_ip][2: src_port][payload]
                        // New wire format: [4: src_ip][2: src_port][4: payload_len][payload]
                        if recv.len() < 8 {
                            return ReadResult::Error;
                        }
                        let src_ip = &recv[2..6];
                        let src_port = u16::from_le_bytes([recv[6], recv[7]]);
                        let payload = &recv[8..];
                        self.rx_bytes += payload.len() as u64;
                        self.rx_packets += 1;
                        let mut out = alloc::vec![0u8; 4 + 2 + 4 + payload.len()];
                        out[..4].copy_from_slice(src_ip);
                        out[4..6].copy_from_slice(&src_port.to_le_bytes());
                        out[6..10].copy_from_slice(&(payload.len() as u32).to_le_bytes());
                        out[10..].copy_from_slice(payload);
                        ReadResult::Data(out)
                    }
                    crate::socket_api::RESP_EMPTY => ReadResult::Again,
                    _ => ReadResult::Error,
                }
            }
            _ => ReadResult::Error,
        }
    }

    fn read_icmp(
        &mut self,
        api_handle: u32,
        sf: u8,
        offset: u64,
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) -> ReadResult {
        match sf {
            SF_DIR => ReadResult::NotSupported,
            SF_STATUS => ReadResult::text_offset(
                &socket_api.icmp_status_text(api_handle, socket_set),
                offset,
            ),
            SF_DATA => {
                let recv = socket_api.handle_icmp_recv_from(socket_set, api_handle);
                if recv.len() < 2 {
                    return ReadResult::Error;
                }
                let resp_type = u16::from_le_bytes([recv[0], recv[1]]);
                match resp_type {
                    crate::socket_api::RESP_DATA => {
                        if recv.len() < 6 {
                            return ReadResult::Error;
                        }
                        let src_ip = &recv[2..6];
                        let payload = &recv[6..];
                        self.rx_bytes += payload.len() as u64;
                        self.rx_packets += 1;
                        let mut out = alloc::vec![0u8; 4 + 4 + payload.len()];
                        out[..4].copy_from_slice(src_ip);
                        out[4..8].copy_from_slice(&(payload.len() as u32).to_le_bytes());
                        out[8..].copy_from_slice(payload);
                        ReadResult::Data(out)
                    }
                    crate::socket_api::RESP_EMPTY => ReadResult::Again,
                    _ => ReadResult::Error,
                }
            }
            _ => ReadResult::Error,
        }
    }

    // ── Handle writes ─────────────────────────────────────────────────────────

    fn write_handle<D: smoltcp::phy::Device>(
        &mut self,
        handle: u64,
        data: &[u8],
        resp_port: PortHandle,
        req_id: u16,
        iface: &mut Interface,
        device: &mut D,
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) -> WriteResult {
        let text = match core::str::from_utf8(data) {
            Ok(s) => s.trim(),
            Err(_) => "",
        };

        match handle {
            HANDLE_ETH0_ADDR => self.write_eth0_addr(text, iface),
            HANDLE_ETH0_FLAGS => self.write_eth0_flags(text),
            HANDLE_ETH0_MTU => {
                if let Ok(n) = text.parse::<usize>() {
                    self.mtu = n;
                    WriteResult::Ok(data.len())
                } else {
                    WriteResult::Error
                }
            }
            HANDLE_ROUTES => self.write_routes(text, iface),
            // Read-only files
            HANDLE_ETH0_STATUS | HANDLE_ETH0_STATS | HANDLE_ETH0_EVENTS => WriteResult::ReadOnly,
            // dns/lookup: write hostname, clear any previous result
            HANDLE_DNS_LOOKUP => {
                let hostname = text.trim().to_string();
                if hostname.is_empty() {
                    return WriteResult::Error;
                }
                self.dns_pending = Some(hostname);
                self.dns_result = None;
                WriteResult::Ok(data.len())
            }
            // Dynamic TCP
            h if h >= TCP_DYN_BASE && h < UDP_DYN_BASE => {
                let sf = (h & 0xFF) as u8;
                let api_handle = ((h - TCP_DYN_BASE) >> 8) as u32;
                self.write_tcp(
                    api_handle, sf, data, text, resp_port, req_id, iface, device, socket_set,
                    socket_api,
                )
            }
            // Dynamic UDP
            h if h >= UDP_DYN_BASE && h < ICMP_DYN_BASE => {
                let sf = (h & 0xFF) as u8;
                let api_handle = ((h - UDP_DYN_BASE) >> 8) as u32;
                self.write_udp(
                    api_handle, sf, data, text, resp_port, req_id, socket_set, socket_api,
                )
            }
            h if h >= ICMP_DYN_BASE => {
                let sf = (h & 0xFF) as u8;
                let api_handle = ((h - ICMP_DYN_BASE) >> 8) as u32;
                self.write_icmp(
                    api_handle, sf, data, text, resp_port, req_id, socket_set, socket_api,
                )
            }
            _ => WriteResult::NotSupported,
        }
    }

    fn write_eth0_addr(&mut self, text: &str, iface: &mut Interface) -> WriteResult {
        // "192.168.1.50/24" or "192.168.1.50"
        let (ip_str, prefix_str) = match text.find('/') {
            Some(pos) => (&text[..pos], &text[pos + 1..]),
            None => (text, "24"),
        };
        let prefix_len: u8 = prefix_str.parse().unwrap_or(24);
        let ip = match parse_ipv4(ip_str) {
            Some(ip) => ip,
            None => return WriteResult::Error,
        };
        iface.update_ip_addrs(|addrs| {
            if let Some(slot) = addrs.iter_mut().next() {
                *slot = IpCidr::new(IpAddress::Ipv4(ip), prefix_len);
            }
        });
        if let Some(cfg) = &mut self.ip_config {
            cfg.ip = ip;
            cfg.prefix_len = prefix_len;
        } else {
            self.ip_config = Some(IpConfig {
                ip,
                prefix_len,
                gateway: Ipv4Address::new(0, 0, 0, 0),
                dns_server: Ipv4Address::new(0, 0, 0, 0),
            });
        }
        WriteResult::Ok(text.len())
    }

    fn write_eth0_flags(&mut self, text: &str) -> WriteResult {
        match text {
            "up" | "1" => {
                self.link_up = true;
                WriteResult::Ok(2)
            }
            "down" | "0" => {
                self.link_up = false;
                WriteResult::Ok(4)
            }
            _ => WriteResult::Error,
        }
    }

    fn write_routes(&self, text: &str, iface: &mut Interface) -> WriteResult {
        // "add default via 1.2.3.4 dev eth0"
        if let Some(rest) = text.strip_prefix("add default via ") {
            let gw_str = rest.split_whitespace().next().unwrap_or("");
            if let Some(gw) = parse_ipv4(gw_str) {
                iface.routes_mut().add_default_ipv4_route(gw).ok();
                return WriteResult::Ok(text.len());
            }
        }
        WriteResult::Error
    }

    fn write_tcp<D: smoltcp::phy::Device>(
        &mut self,
        api_handle: u32,
        sf: u8,
        raw: &[u8],
        text: &str,
        resp_port: PortHandle,
        req_id: u16,
        iface: &mut Interface,
        device: &mut D,
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) -> WriteResult {
        match sf {
            SF_CTL => {
                // "connect HOST_OR_IP PORT", "listen PORT [BACKLOG]", "shutdown read|write|both",
                // "ttl N", "linger off|SECS", "only_v6 0|1", or "close"
                if let Some(rest) = text.strip_prefix("connect ") {
                    let parts: Vec<&str> = rest.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(port) = parts[1].parse::<u16>() {
                            let host = parts[0];
                            if let Some(ip) = parse_ipv4(host) {
                                // Direct IP — connect immediately
                                let r = socket_api.handle_connect_existing(
                                    iface, socket_set, api_handle, ip, port,
                                );
                                return if r {
                                    WriteResult::Ok(text.len())
                                } else {
                                    WriteResult::Error
                                };
                            } else {
                                // Hostname — defer connect until DNS resolves.
                                // The response will be sent by
                                // complete_deferred_connect once the main loop
                                // finishes the async DNS query.
                                self.deferred_connects.push(DeferredConnect {
                                    api_handle,
                                    hostname: host.into(),
                                    port,
                                    resp_port,
                                    req_id,
                                    text_len: text.len(),
                                });
                                // Return a sentinel — the caller must NOT send
                                // a response for this write; it will be sent
                                // later by complete_deferred_connect.
                                return WriteResult::Deferred;
                            }
                        }
                    }
                } else if let Some(rest) = text.strip_prefix("listen ") {
                    // "listen PORT [BACKLOG]"
                    let mut parts = rest.split_whitespace();
                    if let Some(port_str) = parts.next() {
                        if let Ok(port) = port_str.parse::<u16>() {
                            let backlog: u16 =
                                parts.next().and_then(|s| s.parse().ok()).unwrap_or(4);
                            let r = socket_api
                                .handle_listen_existing(socket_set, api_handle, port, backlog);
                            return if r {
                                WriteResult::Ok(text.len())
                            } else {
                                WriteResult::Error
                            };
                        }
                    }
                } else if let Some(rest) = text.strip_prefix("shutdown ") {
                    let r = socket_api.handle_tcp_shutdown(socket_set, api_handle, rest.trim());
                    return if r { WriteResult::Ok(text.len()) } else { WriteResult::Error };
                } else if let Some(rest) = text.strip_prefix("ttl ") {
                    if let Ok(ttl) = rest.trim().parse::<u32>() {
                        let r = socket_api.handle_tcp_set_ttl(api_handle, ttl);
                        return if r { WriteResult::Ok(text.len()) } else { WriteResult::Error };
                    }
                } else if let Some(rest) = text.strip_prefix("linger ") {
                    let linger = if rest.trim() == "off" {
                        Some(None)
                    } else {
                        rest.trim().parse::<u64>().ok().map(Some)
                    };
                    if let Some(linger) = linger {
                        let r = socket_api.handle_tcp_set_linger(api_handle, linger);
                        return if r { WriteResult::Ok(text.len()) } else { WriteResult::Error };
                    }
                } else if let Some(rest) = text.strip_prefix("only_v6 ") {
                    let enabled = match rest.trim() {
                        "1" | "true" => true,
                        "0" | "false" => false,
                        _ => return WriteResult::Error,
                    };
                    let r = socket_api.handle_tcp_set_only_v6(api_handle, enabled);
                    return if r { WriteResult::Ok(text.len()) } else { WriteResult::Error };
                } else if text == "close" {
                    socket_api.handle_close(socket_set, api_handle);
                    return WriteResult::Ok(5);
                }
                WriteResult::Error
            }
            SF_DATA => {
                let result = socket_api.handle_send(socket_set, api_handle, raw);
                if result.len() >= 4 {
                    let sent = u16::from_le_bytes([result[2], result[3]]) as usize;
                    self.tx_bytes += sent as u64;
                    self.tx_packets += 1;
                    WriteResult::Ok(sent)
                } else {
                    WriteResult::Error
                }
            }
            _ => WriteResult::ReadOnly,
        }
    }

    fn write_udp(
        &mut self,
        api_handle: u32,
        sf: u8,
        raw: &[u8],
        text: &str,
        resp_port: PortHandle,
        req_id: u16,
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) -> WriteResult {
        match sf {
            SF_CTL => {
                // "bind PORT", "connect IP PORT", "broadcast 0|1", "ttl N",
                // "multicast_loop_v4 0|1", "multicast_ttl_v4 N",
                // "multicast_loop_v6 0|1", "join/leave_multicast_v4",
                // "join/leave_multicast_v6", or "close"
                if let Some(rest) = text.strip_prefix("bind ") {
                    if let Ok(port) = rest.trim().parse::<u16>() {
                        let r = socket_api.handle_udp_bind_port(socket_set, api_handle, port);
                        return if r { WriteResult::Ok(text.len()) } else { WriteResult::Error };
                    }
                } else if let Some(rest) = text.strip_prefix("connect ") {
                    let parts: Vec<&str> = rest.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let (Some(ip), Ok(port)) =
                            (parse_ipv4(parts[0]), parts[1].parse::<u16>())
                        {
                            let r = socket_api.handle_udp_connect(socket_set, api_handle, ip, port);
                            return if r {
                                WriteResult::Ok(text.len())
                            } else {
                                WriteResult::Error
                            };
                        }
                    }
                } else if let Some(rest) = text.strip_prefix("broadcast ") {
                    let enabled = match rest.trim() {
                        "1" | "true" => true,
                        "0" | "false" => false,
                        _ => return WriteResult::Error,
                    };
                    let r = socket_api.handle_udp_set_broadcast(api_handle, enabled);
                    return if r { WriteResult::Ok(text.len()) } else { WriteResult::Error };
                } else if let Some(rest) = text.strip_prefix("ttl ") {
                    if let Ok(ttl) = rest.trim().parse::<u32>() {
                        let r = socket_api.handle_udp_set_ttl(api_handle, ttl);
                        return if r { WriteResult::Ok(text.len()) } else { WriteResult::Error };
                    }
                } else if let Some(rest) = text.strip_prefix("multicast_loop_v4 ") {
                    let enabled = match rest.trim() {
                        "1" | "true" => true,
                        "0" | "false" => false,
                        _ => return WriteResult::Error,
                    };
                    let r = socket_api.handle_udp_set_multicast_loop_v4(api_handle, enabled);
                    return if r { WriteResult::Ok(text.len()) } else { WriteResult::Error };
                } else if let Some(rest) = text.strip_prefix("multicast_ttl_v4 ") {
                    if let Ok(ttl) = rest.trim().parse::<u32>() {
                        let r = socket_api.handle_udp_set_multicast_ttl_v4(api_handle, ttl);
                        return if r { WriteResult::Ok(text.len()) } else { WriteResult::Error };
                    }
                } else if let Some(rest) = text.strip_prefix("multicast_loop_v6 ") {
                    let enabled = match rest.trim() {
                        "1" | "true" => true,
                        "0" | "false" => false,
                        _ => return WriteResult::Error,
                    };
                    let r = socket_api.handle_udp_set_multicast_loop_v6(api_handle, enabled);
                    return if r { WriteResult::Ok(text.len()) } else { WriteResult::Error };
                } else if let Some(rest) = text.strip_prefix("join_multicast_v4 ") {
                    let parts: Vec<&str> = rest.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let (Some(group), Some(interface)) =
                            (parse_ipv4(parts[0]), parse_ipv4(parts[1]))
                        {
                            let r = socket_api
                                .handle_udp_join_multicast_v4(api_handle, group, interface);
                            return if r {
                                WriteResult::Ok(text.len())
                            } else {
                                WriteResult::Error
                            };
                        }
                    }
                } else if let Some(rest) = text.strip_prefix("leave_multicast_v4 ") {
                    let parts: Vec<&str> = rest.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let (Some(group), Some(interface)) =
                            (parse_ipv4(parts[0]), parse_ipv4(parts[1]))
                        {
                            let r = socket_api
                                .handle_udp_leave_multicast_v4(api_handle, group, interface);
                            return if r {
                                WriteResult::Ok(text.len())
                            } else {
                                WriteResult::Error
                            };
                        }
                    }
                } else if let Some(rest) = text.strip_prefix("join_multicast_v6 ") {
                    let parts: Vec<&str> = rest.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(interface) = parts[1].parse::<u32>() {
                            let r = socket_api
                                .handle_udp_join_multicast_v6(api_handle, parts[0], interface);
                            return if r {
                                WriteResult::Ok(text.len())
                            } else {
                                WriteResult::Error
                            };
                        }
                    }
                } else if let Some(rest) = text.strip_prefix("leave_multicast_v6 ") {
                    let parts: Vec<&str> = rest.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(interface) = parts[1].parse::<u32>() {
                            let r = socket_api
                                .handle_udp_leave_multicast_v6(api_handle, parts[0], interface);
                            return if r {
                                WriteResult::Ok(text.len())
                            } else {
                                WriteResult::Error
                            };
                        }
                    }
                } else if text == "close" {
                    socket_api.handle_close(socket_set, api_handle);
                    return WriteResult::Ok(5);
                }
                WriteResult::Error
            }
            SF_DATA => {
                // New wire format: [4: dest_ip][2: dest_port_le][4: payload_len_le][payload]
                if raw.len() < 10 {
                    return WriteResult::Error;
                }
                let dest_ip = Ipv4Address::from_bytes(&raw[..4]);
                let dest_port = u16::from_le_bytes([raw[4], raw[5]]);
                let payload_len = u32::from_le_bytes([raw[6], raw[7], raw[8], raw[9]]) as usize;
                if raw.len() < 10 + payload_len {
                    return WriteResult::Error;
                }
                let payload = &raw[10..10 + payload_len];
                let r = socket_api
                    .handle_udp_send_to(socket_set, api_handle, dest_ip, dest_port, payload);
                self.write_result_from_send(&r, 10)
            }
            _ => WriteResult::ReadOnly,
        }
    }

    fn write_icmp(
        &mut self,
        api_handle: u32,
        sf: u8,
        raw: &[u8],
        text: &str,
        resp_port: PortHandle,
        req_id: u16,
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) -> WriteResult {
        match sf {
            SF_CTL => {
                if let Some(rest) = text.strip_prefix("bind ") {
                    if let Ok(ident) = rest.trim().parse::<u16>() {
                        let r = socket_api.handle_icmp_bind(socket_set, api_handle, ident);
                        return if r { WriteResult::Ok(text.len()) } else { WriteResult::Error };
                    }
                } else if let Some(rest) = text.strip_prefix("ttl ") {
                    if let Ok(ttl) = rest.trim().parse::<u32>() {
                        let r = socket_api.handle_icmp_set_ttl(socket_set, api_handle, ttl);
                        return if r { WriteResult::Ok(text.len()) } else { WriteResult::Error };
                    }
                } else if text == "close" {
                    socket_api.handle_close(socket_set, api_handle);
                    return WriteResult::Ok(5);
                }
                WriteResult::Error
            }
            SF_DATA => {
                if raw.len() < 8 {
                    return WriteResult::Error;
                }
                let dest_ip = Ipv4Address::from_bytes(&raw[..4]);
                let payload_len = u32::from_le_bytes([raw[4], raw[5], raw[6], raw[7]]) as usize;
                if raw.len() < 8 + payload_len {
                    return WriteResult::Error;
                }
                let payload = &raw[8..8 + payload_len];
                let r = socket_api.handle_icmp_send_to(socket_set, api_handle, dest_ip, payload);
                self.write_result_from_send(&r, 8)
            }
            _ => WriteResult::ReadOnly,
        }
    }

    // ── Readdir / stat helpers ────────────────────────────────────────────────

    /// List directory entries for a given directory handle.
    /// Returns `(name, ino, file_type)` triples; file_type 4=DT_DIR, 8=DT_REG.
    fn list_dir(&self, handle: u64, socket_api: &SocketApi) -> Vec<(String, u64, u8)> {
        match handle {
            HANDLE_ROOT => vec![
                ("interfaces".into(), HANDLE_INTERFACES_DIR, 4),
                ("routes".into(), HANDLE_ROUTES, 8),
                ("tcp".into(), HANDLE_TCP_DIR, 4),
                ("udp".into(), HANDLE_UDP_DIR, 4),
                ("icmp".into(), HANDLE_ICMP_DIR, 4),
                ("dns".into(), HANDLE_DNS_DIR, 4),
            ],
            HANDLE_INTERFACES_DIR => vec![("eth0".into(), HANDLE_ETH0_DIR, 4)],
            HANDLE_ETH0_DIR => vec![
                ("status".into(), HANDLE_ETH0_STATUS, 8),
                ("addr".into(), HANDLE_ETH0_ADDR, 8),
                ("flags".into(), HANDLE_ETH0_FLAGS, 8),
                ("mtu".into(), HANDLE_ETH0_MTU, 8),
                ("stats".into(), HANDLE_ETH0_STATS, 8),
                ("events".into(), HANDLE_ETH0_EVENTS, 8),
            ],
            HANDLE_TCP_DIR => {
                let mut entries = vec![("new".into(), HANDLE_TCP_NEW, 8)];
                for id in socket_api.tcp_socket_ids() {
                    let dh = TCP_DYN_BASE | ((id as u64) << 8) | SF_DIR as u64;
                    entries.push((alloc::format!("{}", id), dh, 4));
                }
                entries
            }
            HANDLE_UDP_DIR => {
                let mut entries = vec![("new".into(), HANDLE_UDP_NEW, 8)];
                for id in socket_api.udp_socket_ids() {
                    let dh = UDP_DYN_BASE | ((id as u64) << 8) | SF_DIR as u64;
                    entries.push((alloc::format!("{}", id), dh, 4));
                }
                entries
            }
            HANDLE_ICMP_DIR => {
                let mut entries = vec![("new".into(), HANDLE_ICMP_NEW, 8)];
                for id in socket_api.icmp_socket_ids() {
                    let dh = ICMP_DYN_BASE | ((id as u64) << 8) | SF_DIR as u64;
                    entries.push((alloc::format!("{}", id), dh, 4));
                }
                entries
            }
            HANDLE_DNS_DIR => vec![
                ("lookup".into(), HANDLE_DNS_LOOKUP, 8),
                ("server".into(), HANDLE_DNS_SERVER, 8),
            ],
            // Dynamic TCP socket directory
            h if h >= TCP_DYN_BASE && h < UDP_DYN_BASE && (h & 0xFF) == SF_DIR as u64 => {
                let bid = TCP_DYN_BASE | (h & !0xFF);
                vec![
                    ("ctl".into(), bid | SF_CTL as u64, 8),
                    ("data".into(), bid | SF_DATA as u64, 8),
                    ("accept".into(), bid | SF_ACCEPT as u64, 8),
                    ("status".into(), bid | SF_STATUS as u64, 8),
                    ("events".into(), bid | SF_EVENTS as u64, 8),
                ]
            }
            // Dynamic UDP socket directory
            h if h >= UDP_DYN_BASE && h < ICMP_DYN_BASE && (h & 0xFF) == SF_DIR as u64 => {
                let bid = UDP_DYN_BASE | (h & !0xFF);
                vec![
                    ("ctl".into(), bid | SF_CTL as u64, 8),
                    ("data".into(), bid | SF_DATA as u64, 8),
                    ("status".into(), bid | SF_STATUS as u64, 8),
                ]
            }
            h if h >= ICMP_DYN_BASE && (h & 0xFF) == SF_DIR as u64 => {
                let bid = ICMP_DYN_BASE | (h & !0xFF);
                vec![
                    ("ctl".into(), bid | SF_CTL as u64, 8),
                    ("data".into(), bid | SF_DATA as u64, 8),
                    ("status".into(), bid | SF_STATUS as u64, 8),
                ]
            }
            _ => vec![],
        }
    }

    fn stat_handle(&self, handle: u64, socket_api: &SocketApi) -> (u32, usize) {
        match handle {
            HANDLE_ROOT
            | HANDLE_INTERFACES_DIR
            | HANDLE_ETH0_DIR
            | HANDLE_TCP_DIR
            | HANDLE_UDP_DIR
            | HANDLE_ICMP_DIR
            | HANDLE_DNS_DIR => (S_IFDIR | 0o555, 0),
            HANDLE_ETH0_STATUS => (S_IFREG | 0o444, self.eth0_status().len()),
            HANDLE_ETH0_ADDR => (S_IFREG | 0o644, self.eth0_addr_text().len()),
            HANDLE_ETH0_FLAGS => (S_IFREG | 0o222, 0),
            HANDLE_ETH0_MTU => (S_IFREG | 0o644, alloc::format!("{}\n", self.mtu).len()),
            HANDLE_ETH0_STATS => (S_IFREG | 0o444, self.eth0_stats().len()),
            HANDLE_ETH0_EVENTS => (S_IFREG | 0o444, 0),
            HANDLE_ROUTES => (S_IFREG | 0o644, 0),
            HANDLE_TCP_NEW | HANDLE_UDP_NEW | HANDLE_ICMP_NEW => (S_IFREG | 0o444, 0),
            HANDLE_DNS_LOOKUP => (S_IFREG | 0o644, 0),
            HANDLE_DNS_SERVER => (S_IFREG | 0o444, self.dns_server_text().len()),
            h if h >= TCP_DYN_BASE && h < UDP_DYN_BASE => {
                let sf = (h & 0xFF) as u8;
                let api_handle = ((h - TCP_DYN_BASE) >> 8) as u32;
                if socket_api.has_socket(api_handle) {
                    if sf == SF_DIR { (S_IFDIR | 0o555, 0) } else { (S_IFREG | 0o644, 0) }
                } else {
                    (0, 0) // not found
                }
            }
            h if h >= UDP_DYN_BASE && h < ICMP_DYN_BASE => {
                let sf = (h & 0xFF) as u8;
                let api_handle = ((h - UDP_DYN_BASE) >> 8) as u32;
                if socket_api.has_socket(api_handle) {
                    if sf == SF_DIR { (S_IFDIR | 0o555, 0) } else { (S_IFREG | 0o644, 0) }
                } else {
                    (0, 0) // not found
                }
            }
            h if h >= ICMP_DYN_BASE => {
                let sf = (h & 0xFF) as u8;
                let api_handle = ((h - ICMP_DYN_BASE) >> 8) as u32;
                if socket_api.has_socket(api_handle) {
                    if sf == SF_DIR { (S_IFDIR | 0o555, 0) } else { (S_IFREG | 0o644, 0) }
                } else {
                    (0, 0)
                }
            }
            _ => (0, 0),
        }
    }

    fn poll_handle(&self, handle: u64, socket_set: &mut SocketSet, socket_api: &SocketApi) -> u32 {
        match handle {
            HANDLE_ETH0_EVENTS => POLLIN,
            HANDLE_ETH0_STATUS | HANDLE_ETH0_ADDR | HANDLE_ETH0_MTU | HANDLE_ETH0_STATS
            | HANDLE_ROUTES | HANDLE_TCP_NEW | HANDLE_UDP_NEW | HANDLE_ICMP_NEW => POLLIN,
            h if h >= TCP_DYN_BASE && h < UDP_DYN_BASE => {
                let sf = (h & 0xFF) as u8;
                let api_handle = ((h - TCP_DYN_BASE) >> 8) as u32;
                socket_api.tcp_poll_ready(api_handle, sf, socket_set)
            }
            h if h >= UDP_DYN_BASE && h < ICMP_DYN_BASE => {
                let sf = (h & 0xFF) as u8;
                let api_handle = ((h - UDP_DYN_BASE) >> 8) as u32;
                socket_api.udp_poll_ready(api_handle, sf, socket_set)
            }
            h if h >= ICMP_DYN_BASE => {
                let sf = (h & 0xFF) as u8;
                let api_handle = ((h - ICMP_DYN_BASE) >> 8) as u32;
                socket_api.icmp_poll_ready(api_handle, sf, socket_set)
            }
            // dns/lookup is readable when a result is available
            HANDLE_DNS_LOOKUP => {
                if self.dns_result.is_some() {
                    POLLIN
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    // ── Text generators ───────────────────────────────────────────────────────

    fn eth0_status(&self) -> String {
        let state = if self.link_up { "up" } else { "down" };
        let link = if self.link_up { "up" } else { "down" };
        let mac = self.mac;
        let ip_line = match &self.ip_config {
            Some(c) => {
                let b = c.ip.as_bytes();
                alloc::format!("ipv4: {}.{}.{}.{}/{}\n", b[0], b[1], b[2], b[3], c.prefix_len)
            }
            None => "ipv4: unassigned\n".into(),
        };
        alloc::format!(
            "state: {}\nlink: {}\nmac: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}\nmtu: {}\n{}",
            state,
            link,
            mac[0],
            mac[1],
            mac[2],
            mac[3],
            mac[4],
            mac[5],
            self.mtu,
            ip_line
        )
    }

    fn eth0_addr_text(&self) -> String {
        match &self.ip_config {
            Some(c) => {
                let b = c.ip.as_bytes();
                alloc::format!("{}.{}.{}.{}/{}\n", b[0], b[1], b[2], b[3], c.prefix_len)
            }
            None => "0.0.0.0/0\n".into(),
        }
    }

    fn eth0_stats(&self) -> String {
        alloc::format!(
            "rx_bytes: {}\ntx_bytes: {}\nrx_packets: {}\ntx_packets: {}\n",
            self.rx_bytes,
            self.tx_bytes,
            self.rx_packets,
            self.tx_packets
        )
    }

    fn routes_text(&self, _socket_api: &SocketApi) -> String {
        match &self.ip_config {
            Some(c) => {
                let gw = c.gateway.as_bytes();
                let net_b = c.ip.as_bytes();
                // Derive network address by masking
                alloc::format!(
                    "default via {}.{}.{}.{} dev eth0\n{}.{}.{}.0/{} dev eth0\n",
                    gw[0],
                    gw[1],
                    gw[2],
                    gw[3],
                    net_b[0],
                    net_b[1],
                    net_b[2],
                    c.prefix_len
                )
            }
            None => "# no routes\n".into(),
        }
    }

    fn dns_server_text(&self) -> String {
        match &self.ip_config {
            Some(c) => {
                let d = c.dns_server.as_bytes();
                alloc::format!("{}.{}.{}.{}\n", d[0], d[1], d[2], d[3])
            }
            None => "0.0.0.0\n".into(),
        }
    }

    fn effective_dns_server(&self) -> Option<Ipv4Address> {
        self.ip_config.as_ref().map(|cfg| {
            if cfg.dns_server != Ipv4Address::UNSPECIFIED {
                cfg.dns_server
            } else if cfg.gateway == Ipv4Address::new(10, 0, 2, 2) {
                // QEMU user-mode networking exposes the NAT gateway at 10.0.2.2
                // but answers DNS on 10.0.2.3.
                Ipv4Address::new(10, 0, 2, 3)
            } else {
                cfg.gateway
            }
        })
    }
}

// ── Result types ─────────────────────────────────────────────────────────────

enum ReadResult {
    Data(Vec<u8>),
    EOF,
    Again,
    Error,
    NotSupported,
}

impl ReadResult {
    fn text_offset(text: &str, offset: u64) -> Self {
        let bytes = text.as_bytes();
        let off = offset as usize;
        if off >= bytes.len() { ReadResult::EOF } else { ReadResult::Data(bytes[off..].to_vec()) }
    }
}

#[cfg(test)]
mod tests {
    use abi::vfs_rpc::VfsRpcReqHeader;
    use smoltcp::wire::Ipv4Address;

    use super::{IpConfig, NetVfsProvider, parse_rpc_header};

    fn provider_with_config(dns_server: Ipv4Address, gateway: Ipv4Address) -> NetVfsProvider {
        NetVfsProvider {
            req_read: 0,
            mac: [0; 6],
            mtu: 1500,
            link_up: true,
            ip_config: Some(IpConfig {
                ip: Ipv4Address::new(10, 0, 2, 15),
                prefix_len: 24,
                gateway,
                dns_server,
            }),
            rx_bytes: 0,
            tx_bytes: 0,
            rx_packets: 0,
            tx_packets: 0,
            req_buf: alloc::vec![0u8; 32],
            dns_pending: None,
            dns_result: None,
        }
    }

    #[test]
    fn effective_dns_server_prefers_configured_dns() {
        let provider =
            provider_with_config(Ipv4Address::new(1, 1, 1, 1), Ipv4Address::new(10, 0, 2, 2));
        assert_eq!(provider.effective_dns_server(), Some(Ipv4Address::new(1, 1, 1, 1)));
    }

    #[test]
    fn effective_dns_server_maps_qemu_usernet_gateway_to_slirp_dns() {
        let provider =
            provider_with_config(Ipv4Address::UNSPECIFIED, Ipv4Address::new(10, 0, 2, 2));
        assert_eq!(provider.effective_dns_server(), Some(Ipv4Address::new(10, 0, 2, 3)));
    }

    #[test]
    fn effective_dns_server_falls_back_to_gateway_for_other_networks() {
        let provider =
            provider_with_config(Ipv4Address::UNSPECIFIED, Ipv4Address::new(192, 168, 1, 1));
        assert_eq!(provider.effective_dns_server(), Some(Ipv4Address::new(192, 168, 1, 1)));
    }

    #[test]
    fn parse_rpc_header_rejects_short_frames() {
        let short = [0u8; 4];
        assert!(parse_rpc_header(&short).is_none());
    }

    #[test]
    fn parse_rpc_header_returns_port_op_and_payload() {
        let mut req = alloc::vec![0u8; core::mem::size_of::<VfsRpcReqHeader>() + 3];
        req[0..4].copy_from_slice(&7u32.to_le_bytes());
        req[4] = 3;
        req[5..7].copy_from_slice(&0x1234u16.to_le_bytes());
        req[core::mem::size_of::<VfsRpcReqHeader>()..].copy_from_slice(&[0xAA, 0xBB, 0xCC]);

        let (resp_port, op, req_id, payload) =
            parse_rpc_header(&req).expect("expected valid header");
        assert_eq!(resp_port, 7);
        assert_eq!(op, 3);
        assert_eq!(req_id, 0x1234);
        assert_eq!(payload, &[0xAA, 0xBB, 0xCC]);
    }
}

enum WriteResult {
    Ok(usize),
    Error,
    ReadOnly,
    NotSupported,
    /// The operation has been deferred (e.g. awaiting async DNS resolution).
    /// The response will be sent later; `op_write` must not send one now.
    Deferred,
}

// ── Wire helpers ─────────────────────────────────────────────────────────────

pub fn send_resp(port: PortHandle, req_id: u16, data: &[u8]) {
    let mut resp = Vec::with_capacity(2 + data.len());
    resp.extend_from_slice(&req_id.to_le_bytes());
    resp.extend_from_slice(data);
    let _ = port_send_all(port, &resp);
}

fn send_err(port: PortHandle, req_id: u16, errno: u8) {
    let mut resp = [0u8; 3];
    resp[0..2].copy_from_slice(&req_id.to_le_bytes());
    resp[2] = errno;
    let _ = port_send_all(port, &resp);
}

fn send_handle(port: PortHandle, req_id: u16, handle: u64) {
    let mut resp = [0u8; 11];
    resp[0..2].copy_from_slice(&req_id.to_le_bytes());
    resp[2] = E_OK;
    resp[3..11].copy_from_slice(&handle.to_le_bytes());
    let _ = port_send_all(port, &resp);
}

fn send_data(port: PortHandle, req_id: u16, data: &[u8]) {
    let mut resp = Vec::with_capacity(7 + data.len());
    resp.extend_from_slice(&req_id.to_le_bytes());
    resp.push(E_OK);
    resp.extend_from_slice(&(data.len() as u32).to_le_bytes());
    resp.extend_from_slice(data);
    let _ = port_send_all(port, &resp);
}

fn send_write_ok(port: PortHandle, req_id: u16, bytes_written: u32) {
    let mut resp = [0u8; 7];
    resp[0..2].copy_from_slice(&req_id.to_le_bytes());
    resp[2] = E_OK;
    resp[3..7].copy_from_slice(&bytes_written.to_le_bytes());
    let _ = port_send_all(port, &resp);
}

// ── IPv4 parse helper ─────────────────────────────────────────────────────────

fn parse_ipv4(s: &str) -> Option<Ipv4Address> {
    let parts: Vec<&str> = s.trim().split('.').collect();
    if parts.len() != 4 {
        return None;
    }
    Some(Ipv4Address::new(
        parts[0].parse().ok()?,
        parts[1].parse().ok()?,
        parts[2].parse().ok()?,
        parts[3].parse().ok()?,
    ))
}

/// Decode the fixed-size VFS RPC request header and return `(resp_port, op, req_id, payload)`.
///
/// Returns `None` when `buf` is shorter than [`VfsRpcReqHeader`].
fn parse_rpc_header(buf: &[u8]) -> Option<(PortHandle, u8, u16, &[u8])> {
    let hdr_sz = core::mem::size_of::<VfsRpcReqHeader>();
    if buf.len() < hdr_sz {
        return None;
    }

    let resp_port = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as PortHandle;
    let op = buf[4];
    let req_id = u16::from_le_bytes([buf[5], buf[6]]);
    Some((resp_port, op, req_id, &buf[hdr_sz..]))
}
