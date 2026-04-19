//! Simple ICMP echo utility.
//!
//! Usage: ping [-c count] <host>
#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use abi::syscall::vfs_flags::{O_RDONLY, O_WRONLY};
use smoltcp::phy::ChecksumCapabilities;
use smoltcp::wire::{Icmpv4Packet, Icmpv4Repr, Ipv4Address};
use stem::syscall::{argv_get, vfs_close, vfs_open, vfs_read, vfs_write};

const DEFAULT_COUNT: u32 = 4;
const DEFAULT_TIMEOUT_MS: u64 = 5_000;
const DEFAULT_INTERVAL_MS: u64 = 1_000;
const DEFAULT_PAYLOAD_LEN: usize = 56;

fn get_args() -> Vec<String> {
    let mut len = 0;
    if let Ok(l) = argv_get(&mut []) {
        len = l;
    }
    if len == 0 {
        return Vec::new();
    }

    let mut buf = alloc::vec![0u8; len];
    if argv_get(&mut buf).is_err() {
        return Vec::new();
    }

    let mut args = Vec::new();
    if buf.len() >= 4 {
        let count = u32::from_le_bytes(buf[0..4].try_into().unwrap()) as usize;
        let mut offset = 4;
        for _ in 0..count {
            if offset + 4 > buf.len() {
                break;
            }
            let str_len = u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            if offset + str_len > buf.len() {
                break;
            }
            if let Ok(s) = core::str::from_utf8(&buf[offset..offset + str_len]) {
                args.push(String::from(s));
            }
            offset += str_len;
        }
    }

    args
}

fn print(fd: u32, s: &str) {
    let _ = vfs_write(fd, s.as_bytes());
}

fn wait_until(deadline: stem::time::Instant) {
    while stem::time::now() < deadline {
        stem::yield_now();
    }
}

fn parse_ipv4(text: &str) -> Option<Ipv4Address> {
    let mut parts = [0u8; 4];
    let mut iter = text.trim().split('.');
    for part in &mut parts {
        *part = iter.next()?.parse().ok()?;
    }
    if iter.next().is_some() {
        return None;
    }
    Some(Ipv4Address::new(parts[0], parts[1], parts[2], parts[3]))
}

fn resolve(name: &str) -> Result<Ipv4Address, &'static str> {
    if let Some(ip) = parse_ipv4(name) {
        return Ok(ip);
    }

    let fd = vfs_open("/net/dns/lookup", O_WRONLY).map_err(|_| "cannot open /net/dns/lookup")?;
    let write_ok = vfs_write(fd, name.as_bytes()).is_ok();
    let _ = vfs_close(fd);
    if !write_ok {
        return Err("cannot write /net/dns/lookup");
    }

    let deadline = stem::time::now() + stem::time::Duration::from_millis(3_000);
    loop {
        let fd =
            vfs_open("/net/dns/lookup", O_RDONLY).map_err(|_| "cannot open /net/dns/lookup")?;
        let mut buf = [0u8; 64];
        let read_result = vfs_read(fd, &mut buf);
        let _ = vfs_close(fd);

        if let Ok(n) = read_result {
            if n > 0 {
                let text = String::from(String::from_utf8_lossy(&buf[..n]).trim());
                if text == "error" {
                    return Err("DNS failed");
                }
                if let Some(ip) = parse_ipv4(&text) {
                    return Ok(ip);
                }
                return Err("invalid DNS response");
            }
        }

        if stem::time::now() >= deadline {
            return Err("DNS timeout");
        }
        wait_until(stem::time::now() + stem::time::Duration::from_millis(50));
    }
}

fn read_socket_id(path: &str) -> Result<u32, &'static str> {
    let fd = vfs_open(path, O_RDONLY).map_err(|_| "cannot open socket allocator")?;
    let mut buf = [0u8; 32];
    let n = vfs_read(fd, &mut buf).map_err(|_| "cannot read socket id")?;
    let _ = vfs_close(fd);
    let text = String::from(String::from_utf8_lossy(&buf[..n]).trim());
    text.parse().map_err(|_| "bad socket id")
}

fn write_ctl(path: &str, cmd: &str) -> Result<(), &'static str> {
    let fd = vfs_open(path, O_WRONLY).map_err(|_| "cannot open socket ctl")?;
    let result = vfs_write(fd, cmd.as_bytes()).map(|_| ());
    let _ = vfs_close(fd);
    result.map_err(|_| "cannot write socket ctl")
}

fn build_echo_request(ident: u16, seq_no: u16, payload_len: usize) -> Vec<u8> {
    let payload: Vec<u8> = (0..payload_len).map(|i| (i & 0xff) as u8).collect();
    let repr = Icmpv4Repr::EchoRequest { ident, seq_no, data: &payload };
    let mut packet_bytes = alloc::vec![0u8; repr.buffer_len()];
    let mut packet = Icmpv4Packet::new_unchecked(&mut packet_bytes[..]);
    repr.emit(&mut packet, &ChecksumCapabilities::default());
    packet_bytes
}

fn send_echo(data_path: &str, dest_ip: Ipv4Address, packet: &[u8]) -> Result<(), &'static str> {
    let fd = vfs_open(data_path, O_WRONLY).map_err(|_| "cannot open icmp data")?;
    let mut wire = alloc::vec![0u8; 8 + packet.len()];
    wire[..4].copy_from_slice(dest_ip.as_bytes());
    wire[4..8].copy_from_slice(&(packet.len() as u32).to_le_bytes());
    wire[8..].copy_from_slice(packet);
    let result = vfs_write(fd, &wire).map(|_| ());
    let _ = vfs_close(fd);
    result.map_err(|_| "cannot send echo request")
}

fn recv_echo(data_path: &str) -> Result<Option<(Ipv4Address, Vec<u8>)>, &'static str> {
    let fd = vfs_open(data_path, O_RDONLY).map_err(|_| "cannot open icmp data")?;
    let mut buf = alloc::vec![0u8; 2048];
    let read_result = vfs_read(fd, &mut buf);
    let _ = vfs_close(fd);

    match read_result {
        Ok(n) => {
            if n < 8 {
                return Ok(None);
            }
            let src_ip = Ipv4Address::from_bytes(&buf[..4]);
            let payload_len = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]) as usize;
            if n < 8 + payload_len {
                return Err("short icmp packet");
            }
            buf.truncate(8 + payload_len);
            Ok(Some((src_ip, buf[8..].to_vec())))
        }
        Err(_) => Ok(None),
    }
}

fn is_matching_echo_reply(packet_bytes: &[u8], ident: u16, seq_no: u16) -> bool {
    let packet = match Icmpv4Packet::new_checked(packet_bytes) {
        Ok(packet) => packet,
        Err(_) => return false,
    };
    match Icmpv4Repr::parse(&packet, &ChecksumCapabilities::default()) {
        Ok(Icmpv4Repr::EchoReply { ident: reply_ident, seq_no: reply_seq, .. }) => {
            reply_ident == ident && reply_seq == seq_no
        }
        _ => false,
    }
}

fn create_icmp_socket(ident: u16) -> Result<(u32, String), &'static str> {
    let id = read_socket_id("/net/icmp/new")?;
    let ctl_path = alloc::format!("/net/icmp/{}/ctl", id);
    write_ctl(&ctl_path, &alloc::format!("bind {}", ident))?;
    Ok((id, ctl_path))
}

fn close_icmp_socket(ctl_path: &str) {
    let _ = write_ctl(ctl_path, "close");
}

fn ping_once(
    data_path: &str,
    dest_ip: Ipv4Address,
    ident: u16,
    seq_no: u16,
    payload_len: usize,
) -> Result<u64, &'static str> {
    let packet = build_echo_request(ident, seq_no, payload_len);
    let started = stem::time::now();
    send_echo(data_path, dest_ip, &packet)?;

    let deadline = started + stem::time::Duration::from_millis(DEFAULT_TIMEOUT_MS);
    loop {
        if let Some((src_ip, reply)) = recv_echo(data_path)? {
            if src_ip == dest_ip && is_matching_echo_reply(&reply, ident, seq_no) {
                return Ok((stem::time::now() - started).as_millis() as u64);
            }
        }

        if stem::time::now() >= deadline {
            return Err("timeout");
        }
        wait_until(stem::time::now() + stem::time::Duration::from_millis(20));
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();

    let mut count = DEFAULT_COUNT;
    let mut host = String::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-c" => {
                i += 1;
                if i < args.len() {
                    count = args[i].parse().unwrap_or(DEFAULT_COUNT);
                }
            }
            _ => host = args[i].clone(),
        }
        i += 1;
    }

    if host.is_empty() {
        print(2, "usage: ping [-c count] <host>\n");
        stem::syscall::exit(1);
    }

    let ip = match resolve(&host) {
        Ok(ip) => ip,
        Err(e) => {
            let msg = alloc::format!("ping: {}: {}\n", host, e);
            print(2, &msg);
            stem::syscall::exit(1);
        }
    };

    let ident = (stem::time::now().as_millis() as u16).wrapping_add(1);
    let (socket_id, ctl_path) = match create_icmp_socket(ident) {
        Ok(socket) => socket,
        Err(e) => {
            let msg = alloc::format!("ping: cannot create icmp socket: {}\n", e);
            print(2, &msg);
            stem::syscall::exit(1);
        }
    };
    let data_path = alloc::format!("/net/icmp/{}/data", socket_id);

    let header = alloc::format!("PING {} ({}) {} bytes of data\n", host, ip, DEFAULT_PAYLOAD_LEN);
    print(1, &header);

    let mut transmitted = 0u32;
    let mut received = 0u32;
    let mut total_ms = 0u64;
    let mut min_ms = u64::MAX;
    let mut max_ms = 0u64;

    for seq in 1..=count {
        match ping_once(&data_path, ip, ident, seq as u16, DEFAULT_PAYLOAD_LEN) {
            Ok(ms) => {
                received += 1;
                total_ms += ms;
                if ms < min_ms {
                    min_ms = ms;
                }
                if ms > max_ms {
                    max_ms = ms;
                }
                let line = alloc::format!(
                    "{} bytes from {}: icmp_seq={} time={}ms\n",
                    DEFAULT_PAYLOAD_LEN + 8,
                    ip,
                    seq,
                    ms
                );
                print(1, &line);
            }
            Err(e) => {
                let line = alloc::format!("icmp_seq={} {}\n", seq, e);
                print(1, &line);
            }
        }

        transmitted += 1;
        if seq < count {
            wait_until(stem::time::now() + stem::time::Duration::from_millis(DEFAULT_INTERVAL_MS));
        }
    }

    close_icmp_socket(&ctl_path);

    let loss = if transmitted > 0 { 100 * (transmitted - received) / transmitted } else { 100 };

    let summary = alloc::format!(
        "\n--- {} ping statistics ---\n{} packets transmitted, {} received, {}% packet loss\n",
        host,
        transmitted,
        received,
        loss
    );
    print(1, &summary);

    if received > 0 {
        let avg = total_ms / received as u64;
        let stats = alloc::format!("rtt min/avg/max = {}/{}/{} ms\n", min_ms, avg, max_ms);
        print(1, &stats);
    }

    stem::syscall::exit(if received > 0 { 0 } else { 1 })
}
