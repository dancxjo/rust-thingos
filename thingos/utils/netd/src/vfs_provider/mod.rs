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
use alloc::vec::Vec;

use abi::vfs_rpc::VfsRpcReqHeader;
use smoltcp::wire::Ipv4Address;
use stem::syscall::port::PortHandle;

mod fs;
mod read;
mod rpc;
#[cfg(test)]
mod tests;
mod wire;
mod write;

pub use wire::send_resp;

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

enum WriteResult {
    Ok(usize),
    Error,
    ReadOnly,
    NotSupported,
    /// The operation has been deferred (e.g. awaiting async DNS resolution).
    /// The response will be sent later; `op_write` must not send one now.
    Deferred,
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
