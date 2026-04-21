//! Simple DNS client for A record lookups.
extern crate alloc;
use alloc::vec::Vec;
use core::default::Default;

use smoltcp::iface::{Interface, SocketSet, SocketStorage};
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
        let local_port =
            DNS_EPHEMERAL_PORT_BASE + (start.wrapping_sub(DNS_EPHEMERAL_PORT_BASE) + attempt)
                % DNS_EPHEMERAL_PORT_SPAN;
        match socket.bind(local_port) {
            Ok(()) => return Ok(local_port),
            Err(e) => {
                stem::debug!("DNS: bind port {} failed on attempt {}: {:?}", local_port, attempt, e);
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
        0x01, 0x00, // standard query
        0x00, 0x01, // qdcount
        0x00, 0x00, // ancount
        0x00, 0x00, // nscount
        0x00, 0x00, // arcount
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
