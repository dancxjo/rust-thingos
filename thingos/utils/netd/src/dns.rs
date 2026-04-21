//! Simple DNS client for A record lookups.
//!
//! Provides both a blocking `lookup_a` function (used during boot for DHCP-time
//! resolution) and a non-blocking `AsyncDnsQuery` state machine for use inside
//! the main event loop where blocking would starve VFS servicing.
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::default::Default;

use smoltcp::iface::{Interface, SocketHandle, SocketSet, SocketStorage};
use smoltcp::phy::Device;
use smoltcp::socket::udp::{self, PacketMetadata, Socket as UdpSocket};
use smoltcp::time::{Duration, Instant};
use smoltcp::wire::{IpAddress, IpEndpoint, Ipv4Address};
use stem::warn;

const DNS_TIMEOUT_SECS: u64 = 5;
const DNS_RESEND_INTERVAL_MS: u64 = 500;
const DNS_EPHEMERAL_PORT_BASE: u16 = 49152;
const DNS_EPHEMERAL_PORT_SPAN: u16 = 16384;
const DNS_BIND_ATTEMPTS: u16 = 32;

fn now() -> Instant {
    Instant::from_millis(stem::time::now().as_millis() as i64)
}

#[derive(Debug)]
pub enum DnsError {
    Timeout,
    InvalidResponse,
    NoAnswer,
}

// ── Blocking lookup (used only during boot / DHCP) ───────────────────────────

pub fn lookup_a<D: Device>(
    iface: &mut Interface,
    device: &mut D,
    dns_server: Ipv4Address,
    name: &str,
) -> Result<Ipv4Address, DnsError> {
    let mut rx_meta = [PacketMetadata::EMPTY; 4];
    let mut rx_data = [0u8; 2048];
    let mut tx_meta = [PacketMetadata::EMPTY; 4];
    let mut tx_data = [0u8; 2048];

    let udp_rx_buffer = udp::PacketBuffer::new(&mut rx_meta[..], &mut rx_data[..]);
    let udp_tx_buffer = udp::PacketBuffer::new(&mut tx_meta[..], &mut tx_data[..]);
    let mut udp_socket = UdpSocket::new(udp_rx_buffer, udp_tx_buffer);

    let txid = next_txid();
    let local_port = bind_dns_socket(&mut udp_socket)?;

    let mut sockets_storage: [SocketStorage; 1] = Default::default();
    let mut socket_set = SocketSet::new(&mut sockets_storage[..]);
    let udp_handle = socket_set.add(udp_socket);

    let query = build_dns_query(name, txid);
    let endpoint = IpEndpoint::new(IpAddress::Ipv4(dns_server), 53);

    stem::debug!(
        "DNS: Querying {} for {} (txid=0x{:04x}, local_port={})",
        dns_server,
        name,
        txid,
        local_port
    );

    let start = now();
    let timeout = start + Duration::from_secs(DNS_TIMEOUT_SECS);
    let mut next_send_at = start;
    let mut poll_count = 0u32;
    let mut send_count = 0u32;

    loop {
        let ts = now();
        if ts > timeout {
            stem::debug!("DNS: Timeout after {} polls and {} sends", poll_count, send_count);
            return Err(DnsError::Timeout);
        }

        let _ = iface.poll(ts, device, &mut socket_set);
        poll_count += 1;

        let socket = socket_set.get_mut::<UdpSocket>(udp_handle);
        if ts >= next_send_at && socket.can_send() {
            match socket.send_slice(&query, endpoint) {
                Ok(()) => {
                    send_count = send_count.saturating_add(1);
                    next_send_at = ts + Duration::from_millis(DNS_RESEND_INTERVAL_MS);
                    stem::debug!(
                        "DNS: Query sent to {}:53 (txid=0x{:04x}, {} bytes, send={})",
                        dns_server,
                        txid,
                        query.len(),
                        send_count
                    );
                }
                Err(e) => {
                    warn!("DNS: Failed to send query to {}:53: {:?}", dns_server, e);
                }
            }
        }

        if socket.can_recv() {
            let (data, _) = socket.recv().map_err(|_| DnsError::InvalidResponse)?;
            stem::debug!("DNS: Response received ({} bytes)", data.len());
            if let Ok(ip) = parse_dns_response(data, txid) {
                return Ok(ip);
            }
            stem::debug!("DNS: Ignoring non-matching or invalid response");
        }

        stem::time::sleep_ms(10);
    }
}

// ── Non-blocking async DNS query ─────────────────────────────────────────────

/// Dedicated static buffers for the async DNS socket.
/// Only one async DNS query is active at a time, so a single buffer pair suffices.
static mut DNS_RX_META: [PacketMetadata; 4] = [PacketMetadata::EMPTY; 4];
static mut DNS_RX_DATA: [u8; 2048] = [0u8; 2048];
static mut DNS_TX_META: [PacketMetadata; 4] = [PacketMetadata::EMPTY; 4];
static mut DNS_TX_DATA: [u8; 2048] = [0u8; 2048];

/// Result of a single `poll()` call on an async DNS query.
pub enum DnsProgress {
    /// Query is still in progress; call `poll()` again next iteration.
    Pending,
    /// Resolved successfully.
    Resolved(Ipv4Address),
    /// Resolution failed.
    Failed(DnsError),
}

/// Non-blocking DNS query that integrates with the main event loop's
/// `SocketSet` and `Interface`.  Each `poll()` call does at most one
/// send/receive cycle, returning immediately so the VFS provider can
/// continue servicing other requests.
pub struct AsyncDnsQuery {
    pub hostname: String,
    dns_server: Ipv4Address,
    txid: u16,
    query: Vec<u8>,
    start: Instant,
    next_send_at: Instant,
    send_count: u32,
    udp_handle: SocketHandle,
}

impl AsyncDnsQuery {
    /// Start a new async DNS query.  Allocates a UDP socket inside
    /// `socket_set` using the dedicated static DNS buffers.
    ///
    /// Returns `None` if the UDP socket cannot be bound.
    pub fn start(
        socket_set: &mut SocketSet<'_>,
        dns_server: Ipv4Address,
        hostname: String,
    ) -> Option<Self> {
        // SAFETY: only one AsyncDnsQuery exists at a time (enforced by the
        // caller — the main loop creates at most one).  The static buffers
        // are not accessed from any other context while the query is live.
        let udp_socket = unsafe {
            // Re-initialise metadata slots for reuse across queries.
            for slot in DNS_RX_META.iter_mut() {
                *slot = PacketMetadata::EMPTY;
            }
            for slot in DNS_TX_META.iter_mut() {
                *slot = PacketMetadata::EMPTY;
            }
            let rx_buf = udp::PacketBuffer::new(&mut DNS_RX_META[..], &mut DNS_RX_DATA[..]);
            let tx_buf = udp::PacketBuffer::new(&mut DNS_TX_META[..], &mut DNS_TX_DATA[..]);
            UdpSocket::new(rx_buf, tx_buf)
        };

        let txid = next_txid();
        let query = build_dns_query(&hostname, txid);
        let ts = now();

        // Bind to an ephemeral port.
        let mut bound_socket = udp_socket;
        let mut local_port = 0u16;
        let start_port = preferred_local_port();
        let mut bound = false;
        for attempt in 0..DNS_BIND_ATTEMPTS {
            let port = DNS_EPHEMERAL_PORT_BASE
                + (start_port.wrapping_sub(DNS_EPHEMERAL_PORT_BASE) + attempt)
                    % DNS_EPHEMERAL_PORT_SPAN;
            match bound_socket.bind(port) {
                Ok(()) => {
                    local_port = port;
                    bound = true;
                    break;
                }
                Err(_) => {}
            }
        }
        if !bound {
            warn!("DNS async: failed to bind ephemeral port for '{}'", hostname);
            return None;
        }

        stem::debug!(
            "DNS async: starting query for '{}' via {} (txid=0x{:04x}, port={})",
            hostname,
            dns_server,
            txid,
            local_port
        );

        let udp_handle = socket_set.add(bound_socket);

        Some(Self {
            hostname,
            dns_server,
            txid,
            query,
            start: ts,
            next_send_at: ts, // send immediately on first poll
            send_count: 0,
            udp_handle,
        })
    }

    /// Drive one iteration of the DNS query.  Returns immediately with
    /// `Pending` if no response is available yet.
    ///
    /// The caller must ensure `iface.poll()` is called before this (which
    /// the main loop already does).
    pub fn poll(&mut self, socket_set: &mut SocketSet<'_>) -> DnsProgress {
        let ts = now();

        // Check timeout
        if ts > self.start + Duration::from_secs(DNS_TIMEOUT_SECS) {
            stem::debug!(
                "DNS async: timeout for '{}' after {} sends",
                self.hostname,
                self.send_count
            );
            return DnsProgress::Failed(DnsError::Timeout);
        }

        let endpoint = IpEndpoint::new(IpAddress::Ipv4(self.dns_server), 53);
        let socket = socket_set.get_mut::<UdpSocket>(self.udp_handle);

        // Send/resend query if needed
        if ts >= self.next_send_at && socket.can_send() {
            match socket.send_slice(&self.query, endpoint) {
                Ok(()) => {
                    self.send_count = self.send_count.saturating_add(1);
                    self.next_send_at = ts + Duration::from_millis(DNS_RESEND_INTERVAL_MS);
                    stem::debug!(
                        "DNS async: query sent for '{}' (send={})",
                        self.hostname,
                        self.send_count
                    );
                }
                Err(e) => {
                    warn!("DNS async: send failed for '{}': {:?}", self.hostname, e);
                }
            }
        }

        // Check for response
        if socket.can_recv() {
            match socket.recv() {
                Ok((data, _)) => {
                    stem::debug!("DNS async: response received ({} bytes)", data.len());
                    match parse_dns_response(data, self.txid) {
                        Ok(ip) => return DnsProgress::Resolved(ip),
                        Err(_) => {
                            stem::debug!("DNS async: ignoring non-matching response");
                        }
                    }
                }
                Err(_) => {}
            }
        }

        DnsProgress::Pending
    }

    /// Remove the DNS socket from the socket set.
    /// Must be called when the query completes (resolved or failed).
    pub fn cleanup(self, socket_set: &mut SocketSet<'_>) {
        socket_set.remove(self.udp_handle);
    }
}

// ── Shared helpers ───────────────────────────────────────────────────────────

fn next_txid() -> u16 {
    let millis = stem::time::now().as_millis() as u64;
    let mixed = millis ^ millis.rotate_right(17);
    (mixed as u16).wrapping_add(1)
}

fn preferred_local_port() -> u16 {
    let millis = stem::time::now().as_millis() as u64;
    DNS_EPHEMERAL_PORT_BASE + (millis as u16 % DNS_EPHEMERAL_PORT_SPAN)
}

fn bind_dns_socket(socket: &mut UdpSocket) -> Result<u16, DnsError> {
    let start = preferred_local_port();
    for attempt in 0..DNS_BIND_ATTEMPTS {
        let local_port = DNS_EPHEMERAL_PORT_BASE
            + (start.wrapping_sub(DNS_EPHEMERAL_PORT_BASE) + attempt) % DNS_EPHEMERAL_PORT_SPAN;
        match socket.bind(local_port) {
            Ok(()) => return Ok(local_port),
            Err(e) => {
                stem::debug!(
                    "DNS: bind port {} failed on attempt {}: {:?}",
                    local_port,
                    attempt,
                    e
                );
            }
        }
    }
    Err(DnsError::Timeout)
}

fn build_dns_query(name: &str, txid: u16) -> Vec<u8> {
    let mut query = Vec::new();
    query.extend_from_slice(&[
        (txid >> 8) as u8,
        txid as u8,
        0x01,
        0x00, // standard query
        0x00,
        0x01, // qdcount
        0x00,
        0x00, // ancount
        0x00,
        0x00, // nscount
        0x00,
        0x00, // arcount
    ]);

    for part in name.split('.') {
        query.push(part.len() as u8);
        query.extend_from_slice(part.as_bytes());
    }
    query.push(0);
    query.extend_from_slice(&[
        0x00, 0x01, // A
        0x00, 0x01, // IN
    ]);
    query
}

fn parse_dns_response(data: &[u8], expected_txid: u16) -> Result<Ipv4Address, DnsError> {
    if data.len() < 12 {
        return Err(DnsError::InvalidResponse);
    }

    let txid = u16::from_be_bytes([data[0], data[1]]);
    if txid != expected_txid {
        return Err(DnsError::InvalidResponse);
    }

    let flags = u16::from_be_bytes([data[2], data[3]]);
    let rcode = flags & 0x000f;
    if rcode != 0 {
        return Err(DnsError::NoAnswer);
    }

    let ancount = u16::from_be_bytes([data[6], data[7]]);
    if ancount == 0 {
        return Err(DnsError::NoAnswer);
    }

    let mut pos = 12;
    while pos < data.len() && data[pos] != 0 {
        let len = data[pos] as usize;
        if len >= 192 {
            pos += 2;
            break;
        }
        pos += 1 + len;
    }
    if pos < data.len() && data[pos] == 0 {
        pos += 1;
    }

    pos += 4;

    for _ in 0..ancount {
        if pos >= data.len() {
            return Err(DnsError::InvalidResponse);
        }

        if data[pos] >= 192 {
            pos += 2;
        } else {
            while pos < data.len() && data[pos] != 0 {
                let len = data[pos] as usize;
                pos += 1 + len;
            }
            pos += 1;
        }

        if pos + 10 > data.len() {
            return Err(DnsError::InvalidResponse);
        }

        let rtype = u16::from_be_bytes([data[pos], data[pos + 1]]);
        let rdlength = u16::from_be_bytes([data[pos + 8], data[pos + 9]]);
        pos += 10;

        if rtype == 1 && rdlength == 4 {
            if pos + 4 > data.len() {
                return Err(DnsError::InvalidResponse);
            }
            return Ok(Ipv4Address::from_bytes(&data[pos..pos + 4]));
        }

        pos += rdlength as usize;
    }

    Err(DnsError::NoAnswer)
}
