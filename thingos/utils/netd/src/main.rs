//! # Network Service (netd) — Phase 3: /net/ VFS provider
//!
//! Replaces the graph-based driver IPC and port-based socket API with:
//! - Driver access via `/dev/net/virtio0/{rx,tx,mac,mtu}` VFS files (issue #540)
//! - Application socket API via `/net/` VFS tree (issue #541)
//! Provides networking capabilities using smoltcp TCP/IP stack.
#![no_std]
#![no_main]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::default::Default;

#[macro_use]
extern crate stem;

mod dhcp;
mod dns;
mod socket_api;
mod vfs_device;
mod vfs_provider;

use abi::seed::{
    HOST_PROGRAM, HOST_VFS_PROVIDER, INTERFACE_PROGRAM_V1, INTERFACE_VFS_PROVIDER_MOUNT_V1,
    INTERFACE_VFS_PROVIDER_UNMOUNT_V1, SEED_ABI_VERSION, Seed, SeedInterface,
};
use abi::syscall::vfs_flags::{O_NONBLOCK, O_RDONLY, O_WRONLY};
use abi::syscall::{PollHandle, poll_flags};
use abi::vfs_watch::{flags as watch_flags, mask as watch_mask};
use smoltcp::iface::{Config, Interface, SocketSet, SocketStorage};
use smoltcp::wire::EthernetAddress;
use socket_api::SocketApi;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_poll, vfs_read, vfs_umount, vfs_watch_path};
use stem::syscall::{argv_get, exit};
use stem::{debug, info, warn};
use vfs_device::VfsNicDevice;
use vfs_provider::NetVfsProvider;

/// Path prefix for the virtio NIC VFS provider (published by virtio_netd).
const VIRTIO_PATH_PREFIX: &str = "/dev/net/virtio";
/// Maximum virtioN unit index to probe during startup.
const MAX_VIRTIO_UNITS: u32 = 16;
const DEFAULT_MOUNT_POINT: &str = "/net";
const SEED_NAME: &[u8] = b"netd";
const HOOK_MOUNT_V1: &[u8] = b"_start";
const HOOK_UNMOUNT_V1: &[u8] = b"thingos_vfs_unmount_v1";

#[unsafe(no_mangle)]
#[used]
pub static THINGOS_SEED: Seed = Seed {
    abi_version: SEED_ABI_VERSION,
    interface_count: 3,
    hosting_modes: HOST_PROGRAM | HOST_VFS_PROVIDER,
    capabilities: 0,
    name_ptr: SEED_NAME.as_ptr(),
    name_len: SEED_NAME.len(),
    interfaces: [
        SeedInterface {
            interface_id: INTERFACE_PROGRAM_V1,
            interface_version: 1,
            flags: 0,
            reserved: 0,
            entry_symbol_ptr: core::ptr::null(),
            entry_symbol_len: 0,
        },
        SeedInterface {
            interface_id: INTERFACE_VFS_PROVIDER_MOUNT_V1,
            interface_version: 1,
            flags: 0,
            reserved: 0,
            entry_symbol_ptr: HOOK_MOUNT_V1.as_ptr(),
            entry_symbol_len: HOOK_MOUNT_V1.len(),
        },
        SeedInterface {
            interface_id: INTERFACE_VFS_PROVIDER_UNMOUNT_V1,
            interface_version: 1,
            flags: 0,
            reserved: 0,
            entry_symbol_ptr: HOOK_UNMOUNT_V1.as_ptr(),
            entry_symbol_len: HOOK_UNMOUNT_V1.len(),
        },
        SeedInterface::zero(),
    ],
};

#[derive(Default)]
struct NetdConfig {
    oneshot: bool,
    help: bool,
    mount_point: String,
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
    let mut cfg =
        NetdConfig { mount_point: DEFAULT_MOUNT_POINT.to_string(), ..NetdConfig::default() };
    parse_config_from_args(&get_args(), &mut cfg);
    cfg
}

fn parse_config_from_args(args: &[String], cfg: &mut NetdConfig) {
    let mut i = 0;
    while i < args.len() {
        let arg = args[i].as_str();
        match arg {
            "--oneshot" | "--once" => cfg.oneshot = true,
            "-h" | "--help" => cfg.help = true,
            "--mount" => {
                if i + 1 < args.len() {
                    let candidate = args[i + 1].as_str();
                    if candidate.starts_with('/') {
                        cfg.mount_point = args[i + 1].clone();
                    } else {
                        warn!(
                            "NETD: ignoring non-absolute --mount argument '{}' ; using {}",
                            candidate, cfg.mount_point
                        );
                    }
                    i += 1;
                }
            }
            _ if arg.starts_with('-') => {}
            // mount(8) commonly passes "<device> <target>"; only treat
            // absolute path args as mount targets.
            _ if arg.starts_with('/') => cfg.mount_point = args[i].clone(),
            _ => {
                debug!("NETD: ignoring positional arg '{}'", arg);
            }
        }
        i += 1;
    }
}

fn print_usage() {
    let _ = stem::syscall::write(
        1,
        b"usage: netd [--oneshot|--once] [--mount /path] [--help] [mount-point]\n\
--oneshot  probe the NIC, run DHCP once, print the result, and exit\n",
    );
}

#[stem::main]
fn main(arg: usize) -> ! {
    info!("NETD: Starting network service...");
    debug!("NETD: binary v2 (with heap storage) starting...");
    debug!("NETD: main entry point, arg={}", arg);
    let cfg = parse_config();
    if cfg.help {
        print_usage();
        exit(0);
    }

    debug!("NETD: Starting network service (Phase 3 — /net/ VFS provider)");

    info!("NETD: Waiting for virtio_netd VFS provider at {}*...", VIRTIO_PATH_PREFIX);
    let (provider_path, rx_fd, tx_fd, events_fd, mac, iface_mtu, initial_link_up) =
        open_nic_device();
    let mtu = iface_mtu as usize;

    debug!(
        "NETD: Driver online at {} — MAC {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}  MTU {}",
        provider_path, mac[0], mac[1], mac[2], mac[3], mac[4], mac[5], mtu
    );

    let mut device = VfsNicDevice::new(rx_fd, tx_fd, events_fd, mac, mtu, initial_link_up);
    let config = Config::new(EthernetAddress(mac).into());
    let mut iface = Interface::new(config, &mut device, VfsNicDevice::now());

    if cfg.oneshot {
        debug!("NETD: oneshot mode enabled — DHCP probe will exit after completion");
        match dhcp::run_dhcp(&mut iface, &mut device) {
            Ok(cfg) => {
                debug!("NETD: DHCP — IP: {}, GW: {}, DNS: {}", cfg.ip, cfg.gateway, cfg.dns);
                exit(0);
            }
            Err(e) => {
                warn!("NETD: DHCP failed in oneshot mode: {:?}", e);
                exit(1);
            }
        }
    }

    debug!("NETD: Running DHCP...");
    let dhcp_config = match dhcp::run_dhcp(&mut iface, &mut device) {
        Ok(cfg) => {
            debug!("NETD: DHCP — IP: {}, GW: {}, DNS: {}", cfg.ip, cfg.gateway, cfg.dns);
            cfg
        }
        Err(e) => {
            warn!("NETD: DHCP failed: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    debug!("NETD: creating SocketApi...");
    let mut socket_api = SocketApi::new();
    debug!("NETD: allocating sockets_storage...");
    let mut sockets_storage: Vec<SocketStorage> = Vec::with_capacity(256);
    for i in 0..256 {
        if i % 64 == 0 {
            debug!("NETD: pushing socket storage {}...", i);
        }
        sockets_storage.push(SocketStorage::EMPTY);
    }
    debug!("NETD: creating SocketSet...");
    let mut socket_set = SocketSet::new(&mut sockets_storage[..]);
    debug!("NETD: getting link state...");
    let mut last_link_state = device.link_up();
    debug!("NETD: scanning NIC units...");
    let _known_nic_units = scan_registered_nic_units();
    debug!("NETD: NIC units scanned.");

    // Do not expose /net until the service can actively process RPCs.
    // Mounting before startup init finishes can let early clients issue
    // lookups while this thread is still preparing state, which may trigger
    // provider timeout taint.
    let mount_point = cfg.mount_point.clone();
    let mut net_provider = loop {
        match NetVfsProvider::new(&mount_point, mac, mtu, initial_link_up) {
            Some(provider) => break provider,
            None => {
                warn!("NETD: Failed to mount {}, retrying...", mount_point);
                stem::time::sleep_ms(200);
            }
        }
    };

    net_provider.set_ip_config(
        dhcp_config.ip,
        dhcp_config.prefix_len,
        dhcp_config.gateway,
        dhcp_config.dns,
    );

    debug!("NETD: bridging request port to fd...");
    let req_fd =
        stem::syscall::vfs::vfs_handle_from_port(net_provider.req_read_port()).unwrap_or(0);
    debug!("NETD: request fd={}", req_fd);
    debug!("NETD: setting up /dev/net watch...");
    let nic_watch_fd =
        match vfs_watch_path("/dev/net", watch_mask::ALL_EVENTS, watch_flags::NONBLOCK) {
            Ok(fd) => Some(fd),
            Err(e) => {
                warn!("NETD: Failed to watch /dev/net for NIC registrations: {:?}", e);
                None
            }
        };
    debug!("NETD: watch fd={:?}", nic_watch_fd);

    info!("NETD: Network ready");
    debug!("NETD: entering VFS service loop");

    // ── Async DNS state ──────────────────────────────────────────────────
    // At most one DNS query is active at a time.  The query object lives
    // across main-loop iterations so it can be polled incrementally.
    let mut active_dns: Option<dns::AsyncDnsQuery> = None;
    // Tracks whether the current DNS query is for a deferred TCP connect
    // (true) or a `/net/dns/lookup` file read (false).
    let mut dns_for_deferred_connect = false;

    loop {
        let mut did_work = false;
        // trace!("NETD: main loop iteration");

        let now = VfsNicDevice::now();
        if iface.poll(now, &mut device, &mut socket_set) {
            did_work = true;
        }

        if net_provider.drain_rpcs(&mut iface, &mut device, &mut socket_set, &mut socket_api) {
            did_work = true;
        }

        let now = VfsNicDevice::now();
        if iface.poll(now, &mut device, &mut socket_set) {
            did_work = true;
        }

        // ── Async DNS: start new queries if needed ───────────────────────
        if active_dns.is_none() {
            // Priority: deferred TCP connects first, then /net/dns/lookup
            if let Some((hostname, dns_ip)) = net_provider.take_deferred_connect_pending() {
                if let Some(query) = dns::AsyncDnsQuery::start(&mut socket_set, dns_ip, hostname) {
                    active_dns = Some(query);
                    dns_for_deferred_connect = true;
                    did_work = true;
                } else {
                    // Could not start DNS query — fail the deferred connect
                    net_provider.complete_deferred_connect(
                        None,
                        &mut iface,
                        &mut socket_set,
                        &mut socket_api,
                    );
                }
            } else if let Some((hostname, dns_ip)) = net_provider.take_dns_pending() {
                if let Some(query) = dns::AsyncDnsQuery::start(&mut socket_set, dns_ip, hostname) {
                    active_dns = Some(query);
                    dns_for_deferred_connect = false;
                    did_work = true;
                } else {
                    net_provider.set_dns_result("error".into());
                }
            }
        }

        // ── Async DNS: poll active query ─────────────────────────────────
        if let Some(query) = &mut active_dns {
            did_work = true; // keep the loop alive while DNS is in-flight
            match query.poll(&mut socket_set) {
                dns::DnsProgress::Pending => {} // still waiting
                dns::DnsProgress::Resolved(ip) => {
                    let b = ip.as_bytes();
                    let ip_str = alloc::format!("{}.{}.{}.{}", b[0], b[1], b[2], b[3]);
                    debug!("NETD: async DNS resolved → {}", ip_str);
                    if dns_for_deferred_connect {
                        net_provider.complete_deferred_connect(
                            Some(ip),
                            &mut iface,
                            &mut socket_set,
                            &mut socket_api,
                        );
                    } else {
                        net_provider.set_dns_result(ip_str);
                    }
                    // Take ownership to call cleanup
                    let q = active_dns.take().unwrap();
                    q.cleanup(&mut socket_set);
                }
                dns::DnsProgress::Failed(e) => {
                    warn!("NETD: async DNS failed: {:?}", e);
                    if dns_for_deferred_connect {
                        net_provider.complete_deferred_connect(
                            None,
                            &mut iface,
                            &mut socket_set,
                            &mut socket_api,
                        );
                    } else {
                        net_provider.set_dns_result("error".into());
                    }
                    let q = active_dns.take().unwrap();
                    q.cleanup(&mut socket_set);
                }
            }
        }

        let current_link = device.link_up();
        if current_link != last_link_state {
            last_link_state = current_link;
            net_provider.link_up = current_link;
            did_work = true;
            debug!("NETD: Link state changed → {}", if current_link { "UP" } else { "DOWN" });
        }

        socket_api.gc_closed_sockets(&mut socket_set);

        if did_work {
            trace!("NETD: did_work=true, pushing notifications");
            net_provider.push_notifications(&mut socket_set, &mut socket_api);
            stem::syscall::yield_now();
        } else {
            let delay_ms =
                iface.poll_delay(now, &socket_set).map(|d| d.total_millis()).unwrap_or(10);
            let timeout_ms = delay_ms.min(100).max(1) as u64;

            let mut pollfds = [
                PollHandle { handle: req_fd as i32, events: poll_flags::POLLIN, revents: 0 },
                PollHandle { handle: events_fd as i32, events: poll_flags::POLLIN, revents: 0 },
                PollHandle {
                    handle: nic_watch_fd.unwrap_or(0) as i32,
                    events: poll_flags::POLLIN,
                    revents: 0,
                },
            ];

            let n_fds = if nic_watch_fd.is_some() { 3 } else { 2 };
            let _ = vfs_poll(&mut pollfds[..n_fds], timeout_ms);

            if let Some(fd) = nic_watch_fd {
                if pollfds[2].revents & poll_flags::POLLIN != 0 {
                    drain_watch_fd(fd);
                    // Just drain it, no need to rescan in this simplified version,
                    // or we could call scan_registered_nic_units() here if needed.
                }
            }
        }
    }
}

#[used]
static KEEP_THINGOS_VFS_MOUNT_V1: extern "C" fn(usize) -> ! = thingos_vfs_mount_v1;

#[used]
static KEEP_THINGOS_VFS_UNMOUNT_V1: extern "C" fn(usize) -> i32 = thingos_vfs_unmount_v1;

#[unsafe(no_mangle)]
pub extern "C" fn thingos_vfs_mount_v1(arg: usize) -> ! {
    unsafe { stem::rt::entry_impl(arg) }
}

#[unsafe(no_mangle)]
pub extern "C" fn thingos_vfs_unmount_v1(_arg: usize) -> i32 {
    let mount_point = parse_config().mount_point;
    match vfs_umount(&mount_point) {
        Ok(()) => 0,
        Err(_) => 1,
    }
}

fn scan_registered_nic_units() -> [bool; MAX_VIRTIO_UNITS as usize] {
    debug!("NETD: scanning for registered NIC units...");
    let mut seen = [false; MAX_VIRTIO_UNITS as usize];
    for unit in 0..MAX_VIRTIO_UNITS {
        if nic_unit_ready(unit) {
            seen[unit as usize] = true;
        }
    }
    debug!("NETD: NIC scan complete: {:?}", seen);
    seen
}

fn report_new_nic_registrations(known_units: &mut [bool; MAX_VIRTIO_UNITS as usize]) -> bool {
    let mut detected = false;
    for unit in 0..MAX_VIRTIO_UNITS {
        let idx = unit as usize;
        let ready = nic_unit_ready(unit);
        if ready && !known_units[idx] {
            known_units[idx] = true;
            detected = true;
            debug!(
                "NETD: Detected new NIC registration from cambium at {}{}",
                VIRTIO_PATH_PREFIX, unit
            );
        } else if !ready {
            known_units[idx] = false;
        }
    }
    detected
}

fn nic_unit_ready(unit: u32) -> bool {
    let rx_path = alloc::format!("{}{}{}", VIRTIO_PATH_PREFIX, unit, "/rx");
    stem::trace!("NETD: checking nic unit {} at {}", unit, rx_path);
    match vfs_open(&rx_path, O_RDONLY | O_NONBLOCK) {
        Ok(fd) => {
            let _ = vfs_close(fd);
            true
        }
        Err(_) => false,
    }
}

fn drain_watch_fd(fd: u32) {
    let mut buf = [0u8; 512];
    loop {
        match vfs_read(fd, &mut buf) {
            Ok(0) => break,
            Ok(_) => {}
            Err(abi::errors::Errno::EAGAIN) => break,
            Err(err) => {
                warn!("NETD: failed draining /dev/net watch events: {:?}", err);
                break;
            }
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

            debug!(
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
            debug!(
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
    use alloc::string::ToString;

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

    #[test]
    fn test_parse_config_ignores_supervisor_control_arg() {
        let mut cfg =
            NetdConfig { mount_point: DEFAULT_MOUNT_POINT.to_string(), ..NetdConfig::default() };
        let args = alloc::vec!["0".to_string()];
        parse_config_from_args(&args, &mut cfg);
        assert_eq!(cfg.mount_point, DEFAULT_MOUNT_POINT);
    }

    #[test]
    fn test_parse_config_prefers_absolute_mount_path() {
        let mut cfg =
            NetdConfig { mount_point: DEFAULT_MOUNT_POINT.to_string(), ..NetdConfig::default() };
        let args = alloc::vec!["none".to_string(), "/net-alt".to_string()];
        parse_config_from_args(&args, &mut cfg);
        assert_eq!(cfg.mount_point, "/net-alt");
    }

    #[test]
    fn test_parse_config_mount_flag_requires_absolute_path() {
        let mut cfg =
            NetdConfig { mount_point: DEFAULT_MOUNT_POINT.to_string(), ..NetdConfig::default() };
        let args = alloc::vec!["--mount".to_string(), "net-alt".to_string()];
        parse_config_from_args(&args, &mut cfg);
        assert_eq!(cfg.mount_point, DEFAULT_MOUNT_POINT);
    }
}
