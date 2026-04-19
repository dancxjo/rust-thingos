//! # Network Service (netd) — Phase 3: /net/ VFS provider
//!
//! Replaces the graph-based driver IPC and port-based socket API with:
//! - Driver access via `/dev/net/virtio0/{rx,tx,mac,mtu}` VFS files (issue #540)
//! - Application socket API via `/net/` VFS tree (issue #541)
//! Provides networking capabilities using smoltcp TCP/IP stack.
#![no_std]
#![no_main]
extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use core::default::Default;

#[macro_use]
extern crate stem;

mod dhcp;
mod dns;
mod socket_api;
mod vfs_device;
mod vfs_provider;

use abi::syscall::vfs_flags::{O_NONBLOCK, O_RDONLY, O_WRONLY};
use smoltcp::iface::{Config, Interface, SocketSet, SocketStorage};
use smoltcp::wire::EthernetAddress;
use socket_api::SocketApi;
use stem::syscall::{argv_get, exit};
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};
use stem::{info, warn};
use vfs_device::VfsNicDevice;
use vfs_provider::NetVfsProvider;

/// Path prefix for the virtio NIC VFS provider (published by virtio_netd).
const VIRTIO_PATH_PREFIX: &str = "/dev/net/virtio";
/// Maximum virtioN unit index to probe during startup.
const MAX_VIRTIO_UNITS: u32 = 16;

#[derive(Default)]
struct NetdConfig {
    oneshot: bool,
    help: bool,
}

fn get_args() -> Vec<String> {
    stem::debug!("NETD: get_args starting...");
    let mut len = 0;
    if let Ok(l) = argv_get(&mut []) {
        len = l;
    }
    stem::debug!("NETD: argv_get len={}", len);
    if len == 0 {
        return Vec::new();
    }

    let mut buf = alloc::vec![0u8; len];
    if let Err(e) = argv_get(&mut buf) {
        stem::error!("NETD: argv_get failed: {:?}", e);
        return Vec::new();
    }

    let args: Vec<String> = stem::utils::parse_argv(&buf)
        .into_iter()
        .skip(1)
        .filter_map(|arg| core::str::from_utf8(arg).ok().map(String::from))
        .collect();
    stem::debug!("NETD: get_args returning {} args", args.len());
    args
}

fn parse_config() -> NetdConfig {
    let mut cfg = NetdConfig::default();
    for arg in get_args() {
        match arg.as_str() {
            "--oneshot" | "--once" => cfg.oneshot = true,
            "-h" | "--help" => cfg.help = true,
            _ => {}
        }
    }
    cfg
}

fn print_usage() {
    let _ = stem::syscall::write(
        1,
        b"usage: netd [--oneshot|--once] [--help]\n\
--oneshot  probe the NIC, run DHCP once, print the result, and exit\n",
    );
}

#[stem::main]
fn main(arg: usize) -> ! {
    stem::debug!("NETD: main entry point, arg={}", arg);
    let cfg = parse_config();
    if cfg.help {
        print_usage();
        exit(0);
    }

    info!("NETD: Starting network service (Phase 3 — /net/ VFS provider)");

    info!("NETD: Waiting for virtio_netd VFS provider at {}*...", VIRTIO_PATH_PREFIX);
    let (provider_path, rx_fd, tx_fd, events_fd, mac, iface_mtu, initial_link_up) =
        open_nic_device();
    let mtu = iface_mtu as usize;

    info!(
        "NETD: Driver online at {} — MAC {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}  MTU {}",
        provider_path, mac[0], mac[1], mac[2], mac[3], mac[4], mac[5], mtu
    );

    let mut device = VfsNicDevice::new(rx_fd, tx_fd, events_fd, mac, mtu, initial_link_up);
    let config = Config::new(EthernetAddress(mac).into());
    let mut iface = Interface::new(config, &mut device, VfsNicDevice::now());

    if cfg.oneshot {
        info!("NETD: oneshot mode enabled — DHCP probe will exit after completion");
        match dhcp::run_dhcp(&mut iface, &mut device) {
            Ok(cfg) => {
                info!("NETD: DHCP — IP: {}, GW: {}, DNS: {}", cfg.ip, cfg.gateway, cfg.dns);
                exit(0);
            }
            Err(e) => {
                warn!("NETD: DHCP failed in oneshot mode: {:?}", e);
                exit(1);
            }
        }
    }

    let mut net_provider = loop {
        match NetVfsProvider::new(mac, mtu, initial_link_up) {
            Some(provider) => break provider,
            None => {
                warn!("NETD: Failed to mount /net/, retrying...");
                stem::time::sleep_ms(200);
            }
        }
    };

    info!("NETD: Running DHCP...");
    let dhcp_config = match dhcp::run_dhcp(&mut iface, &mut device) {
        Ok(cfg) => {
            info!("NETD: DHCP — IP: {}, GW: {}, DNS: {}", cfg.ip, cfg.gateway, cfg.dns);
            cfg
        }
        Err(e) => {
            warn!("NETD: DHCP failed: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    net_provider.set_ip_config(
        dhcp_config.ip,
        dhcp_config.prefix_len,
        dhcp_config.gateway,
        dhcp_config.dns,
    );
    info!("NETD: Network ready — entering VFS service loop");

    let mut socket_api = SocketApi::new();
    let mut sockets_storage: [SocketStorage; 256] = [SocketStorage::EMPTY; 256];
    let mut socket_set = SocketSet::new(&mut sockets_storage[..]);
    let mut last_link_state = device.link_up();

    // Bridge the request-read port to an FD for FD-first polling.
    let req_fd =
        stem::syscall::vfs::vfs_handle_from_port(net_provider.req_read_port()).unwrap_or(0);

    loop {
        let mut did_work = false;

        let now = VfsNicDevice::now();
        if iface.poll(now, &mut device, &mut socket_set) {
            did_work = true;
        }

        let before_len = socket_api.socket_count();
        net_provider.drain_rpcs(&mut iface, &mut device, &mut socket_set, &mut socket_api);
        if socket_api.socket_count() != before_len {
            did_work = true;
        }

        let now = VfsNicDevice::now();
        if iface.poll(now, &mut device, &mut socket_set) {
            did_work = true;
        }

        let current_link = device.link_up();
        if current_link != last_link_state {
            last_link_state = current_link;
            net_provider.link_up = current_link;
            did_work = true;
            info!("NETD: Link state changed → {}", if current_link { "UP" } else { "DOWN" });
        }

        socket_api.gc_closed_sockets(&mut socket_set);

        if !did_work {
            let mut pollfds = [abi::syscall::PollThing {
                thing: req_fd as i32,
                events: abi::syscall::poll_flags::POLLIN,
                revents: 0,
            }];
            let _ = stem::syscall::vfs::vfs_poll(&mut pollfds, 1);
        }
    }
}

/// Open a virtio NIC device fileset, retrying until any `/dev/net/virtioN`
/// provider is ready.
fn open_nic_device() -> (alloc::string::String, u32, u32, u32, [u8; 6], u32, bool) {
    let mut probe_round = 0u32;

    loop {
        stem::debug!("NETD: NIC probe loop starting (round={})", probe_round);
        for unit in 0..MAX_VIRTIO_UNITS {
            let provider_path = alloc::format!("{}{}", VIRTIO_PATH_PREFIX, unit);
            let rx_path = alloc::format!("{}/rx", provider_path);
            let tx_path = alloc::format!("{}/tx", provider_path);
            let events_path = alloc::format!("{}/events", provider_path);
            let mac_path = alloc::format!("{}/mac", provider_path);
            let mtu_path = alloc::format!("{}/mtu", provider_path);
            let status_path = alloc::format!("{}/status", provider_path);

            stem::debug!("NETD: Probing {}...", provider_path);
            let rx_fd = match vfs_open(&rx_path, O_RDONLY | O_NONBLOCK) {
                Ok(fd) => fd,
                Err(_) => continue,
            };

            stem::debug!("NETD: Found {}/rx, opening others...", provider_path);
            let tx_fd = match vfs_open(&tx_path, O_WRONLY) {
                Ok(fd) => fd,
                Err(e) => {
                    warn!("NETD: Failed to open {}: {:?}", tx_path, e);
                    let _ = vfs_close(rx_fd);
                    continue;
                }
            };

            let events_fd = match vfs_open(&events_path, O_RDONLY | O_NONBLOCK) {
                Ok(fd) => fd,
                Err(e) => {
                    warn!("NETD: Failed to open {}: {:?}", events_path, e);
                    let _ = vfs_close(rx_fd);
                    let _ = vfs_close(tx_fd);
                    continue;
                }
            };

            let mac = read_mac_file(&mac_path).unwrap_or([0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
            let mtu = read_u32_file(&mtu_path).unwrap_or(1500);
            let initial_link_up = read_link_state_file(&status_path).unwrap_or(false);

            info!(
                "NETD: Opened VFS NIC device at {} (rx={}, tx={}, events={}, mtu={}, link={})",
                provider_path,
                rx_fd,
                tx_fd,
                events_fd,
                mtu,
                if initial_link_up { "up" } else { "down" }
            );
            return (provider_path, rx_fd, tx_fd, events_fd, mac, mtu, initial_link_up);
        }

        probe_round = probe_round.saturating_add(1);
        if probe_round == 1 || probe_round % 20 == 0 {
            warn!(
                "NETD: No virtio VFS provider ready under {}[0..{}], retrying (round={})",
                VIRTIO_PATH_PREFIX,
                MAX_VIRTIO_UNITS.saturating_sub(1),
                probe_round
            );
        }
        stem::time::sleep_ms(100);
    }
}

fn read_mac_file(path: &str) -> Option<[u8; 6]> {
    let mut buf = [0u8; 24];
    let n = read_file_bytes(path, &mut buf)?;
    let s = core::str::from_utf8(&buf[..n]).ok()?.trim();
    parse_mac(s)
}

fn parse_mac(s: &str) -> Option<[u8; 6]> {
    let mut mac = [0u8; 6];
    let mut count = 0usize;
    for (i, hex) in s.split(':').enumerate() {
        if i >= 6 {
            return None;
        }
        mac[i] = u8::from_str_radix(hex.trim(), 16).ok()?;
        count += 1;
    }
    if count != 6 {
        return None;
    }
    Some(mac)
}

fn read_u32_file(path: &str) -> Option<u32> {
    let mut buf = [0u8; 16];
    let n = read_file_bytes(path, &mut buf)?;
    let s = core::str::from_utf8(&buf[..n]).ok()?.trim();
    s.parse().ok()
}

fn read_link_state_file(path: &str) -> Option<bool> {
    let mut buf = [0u8; 128];
    let n = read_file_bytes(path, &mut buf)?;
    let s = core::str::from_utf8(&buf[..n]).ok()?;
    for line in s.lines() {
        let line = line.trim();
        if let Some(value) = line.strip_prefix("link:") {
            return Some(value.trim() == "up");
        }
        if let Some(value) = line.strip_prefix("state:") {
            return Some(value.trim() == "up");
        }
    }
    None
}

fn read_file_bytes(path: &str, buf: &mut [u8]) -> Option<usize> {
    let fd = vfs_open(path, O_RDONLY).ok()?;
    let result = vfs_read(fd, buf).ok();
    let _ = vfs_close(fd);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mac_valid() {
        let mac = parse_mac("52:54:00:12:34:56").unwrap();
        assert_eq!(mac, [0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
    }

    #[test]
    fn test_parse_mac_zeros() {
        let mac = parse_mac("00:00:00:00:00:00").unwrap();
        assert_eq!(mac, [0u8; 6]);
    }

    #[test]
    fn test_parse_mac_broadcast() {
        let mac = parse_mac("ff:ff:ff:ff:ff:ff").unwrap();
        assert_eq!(mac, [0xff; 6]);
    }

    #[test]
    fn test_parse_mac_too_short() {
        assert!(parse_mac("52:54:00:12:34").is_none());
    }

    #[test]
    fn test_parse_mac_too_long() {
        assert!(parse_mac("52:54:00:12:34:56:78").is_none());
    }

    #[test]
    fn test_parse_mac_invalid_hex() {
        assert!(parse_mac("52:54:00:12:ZZ:56").is_none());
    }

    #[test]
    fn test_parse_mac_empty() {
        assert!(parse_mac("").is_none());
    }

    #[test]
    fn test_parse_mac_with_whitespace() {
        // Trimming whitespace around each octet should work
        let mac = parse_mac("52:54: 00:12:34:56").unwrap();
        assert_eq!(mac, [0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
    }
}
