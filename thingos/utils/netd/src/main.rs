//! # Network Service (netd) — Phase 3: /net/ VFS provider
//!
//! Replaces the graph-based driver IPC and port-based socket API with:
//! - Driver access via `/dev/net/virtio0/{rx,tx,mac,mtu}` VFS files (issue #540)
//! - Application socket API via `/net/` VFS tree (issue #541)
//! Provides networking capabilities using smoltcp TCP/IP stack.
//!
//! ## Thread architecture
//!
//! netd runs two kernel tasks to keep TCP retransmit/poll cadence stable
//! even under heavy RPC or DNS load:
//!
//! **Network poll thread** (this thread, `main`):
//! - Owns all smoltcp state (`iface`, `device`, `socket_set`, `socket_api`)
//!   behind a `spin::Mutex<NetworkPollState>`.
//! - Calls `iface.poll` on a tight ≤4 ms schedule.
//! - Drives async DNS queries (add/poll/cleanup the DNS UDP socket).
//! - Processes `NetCommand`s from the shared command queue.
//! - Pushes `NetEvent`s (DNS results) to the shared event queue.
//!
//! **RPC/dispatch thread** (spawned by `run_rpc_thread`):
//! - Owns `NetVfsProvider` (not shared).
//! - Processes **one** VFS RPC at a time, acquiring the `NetworkPollState`
//!   mutex only for the duration of that single request.  The lock is
//!   released between requests so the poll thread can always interleave.
//! - Forwards DNS/deferred-connect requests as `NetCommand`s to the poll
//!   thread and consumes `NetEvent` responses to update provider state.
//! - Monitors link-state changes by briefly inspecting device state under
//!   the mutex.
//!
//! **Lock ordering** (must never be violated):
//!  1. `CmdQueue` lock — acquire/release independently.
//!  2. `EventQueue` lock — acquire/release independently.
//!  3. `NetworkPollState` mutex — acquire/release independently.
//! No two of these locks are ever held simultaneously.
#![no_std]
#![no_main]
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::default::Default;

#[macro_use]
extern crate stem;

mod cmd_queue;
mod dhcp;
mod dns;
mod socket_api;
mod vfs_device;
mod vfs_provider;

use abi::seed::{
    HOST_PROGRAM, HOST_VFS_PROVIDER, INTERFACE_PROGRAM_V1, INTERFACE_VFS_PROVIDER_MOUNT_V1,
    INTERFACE_VFS_PROVIDER_UNMOUNT_V1, SEED_ABI_VERSION, Seed, SeedInterface,
};
use abi::syscall::vfs_flags::{O_CREAT, O_NONBLOCK, O_RDONLY, O_TRUNC, O_WRONLY};
use abi::syscall::{PollHandle, poll_flags};
use abi::vfs_rpc::VfsRpcOp;
use abi::vfs_watch::{flags as watch_flags, mask as watch_mask};
use cmd_queue::{CmdQueue, EventQueue, NetCommand, NetEvent, new_queues};
use smoltcp::iface::{Config, Interface, SocketSet, SocketStorage};
use smoltcp::wire::EthernetAddress;
use socket_api::SocketApi;
use spin::Mutex;
use stem::syscall::vfs::{
    vfs_close, vfs_open, vfs_poll, vfs_read, vfs_umount, vfs_unlink, vfs_watch_path, vfs_write,
};
use stem::syscall::{argv_get, exit};
use stem::{debug, info, warn};
use vfs_device::VfsNicDevice;
use vfs_provider::{E_OK, ICMP_DYN_BASE, NetVfsProvider, TCP_DYN_BASE, UDP_DYN_BASE};

/// Path prefix for the virtio NIC VFS provider (published by virtio_netd).
const VIRTIO_PATH_PREFIX: &str = "/dev/net/virtio";
/// Maximum virtioN unit index to probe during startup.
const MAX_VIRTIO_UNITS: u32 = 16;
const DEFAULT_MOUNT_POINT: &str = "/net";
const READY_PATH: &str = "/run/netd.ready";
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
    let mut len = 0;
    if let Ok(l) = argv_get(&mut []) {
        len = l;
    }
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
    stem::debug!("NETD: parsed {} argv bytes into {} args", len, args.len());
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

fn clear_ready_flag() {
    let _ = vfs_unlink(READY_PATH);
}

fn publish_ready_flag(mount_point: &str) {
    if let Ok(fd) = vfs_open(READY_PATH, O_WRONLY | O_CREAT | O_TRUNC) {
        let _ = vfs_write(fd, mount_point.as_bytes());
        let _ = vfs_close(fd);
    }
}

fn probe_socket_allocator_ready(socket_api: &mut SocketApi, socket_set: &mut SocketSet) -> bool {
    let Some(buf_idx) = socket_api.alloc_buffer() else {
        return false;
    };

    match socket_api.alloc_icmp_socket_raw(socket_set, buf_idx) {
        Some(handle) => {
            let _ = socket_api.handle_close(socket_set, handle);
            true
        }
        None => {
            socket_api.free_buffer(buf_idx);
            false
        }
    }
}

// ── NetworkPollState ──────────────────────────────────────────────────────────

/// All smoltcp state shared between the poll thread and the RPC thread.
///
/// Wrapped in `Arc<spin::Mutex<NetworkPollState>>` so the RPC/dispatch
/// thread can acquire the lock to process a single VFS request, then
/// immediately release it so `iface.poll` can continue without delay.
///
/// All methods that need to borrow multiple fields simultaneously are defined
/// on this struct so that Rust's borrow checker can see they are distinct
/// field borrows rather than multiple borrows through a single `DerefMut`.
struct NetworkPollState {
    iface: Interface,
    device: VfsNicDevice,
    socket_set: SocketSet<'static>,
    socket_api: SocketApi,
    /// Last observed link state; updated on each poll iteration.
    last_link_state: bool,
    /// Set once after DHCP completes to enable the ready-flag probe.
    ip_configured: bool,
    /// Set once the socket allocator probe passes.
    ready_announced: bool,
    /// Write-end of the VFS provider port — needed to send push notifications.
    req_write: u32,
    /// The VFS mount point, used for `publish_ready_flag`.
    mount_point: String,
}

// SAFETY: `NetworkPollState` is always accessed behind `spin::Mutex`, ensuring
// exclusive access at any time.
//
// `socket_api.rs` declares `pub static mut CONN_RX` and `CONN_TX` arrays used
// as TCP/UDP socket ring-buffers.  Every access to those buffers goes through
// `SocketApi` methods (e.g. `alloc_tcp_socket_raw`, `alloc_udp_socket_raw`),
// which are all invoked through `NetworkPollState` methods while the mutex is
// held.  Therefore at most one mutable reference to those buffers is live at
// any point in time, which satisfies the `&mut` aliasing rules.
//
// The smoltcp `Interface` and `SocketSet` types contain no thread-local state
// and no `UnsafeCell`/interior-mutability outside what the mutex already guards.
unsafe impl Send for NetworkPollState {}

impl NetworkPollState {
    /// Run one smoltcp poll iteration.  Returns `true` if any packets were
    /// processed.
    fn poll_network(&mut self) -> bool {
        let now = VfsNicDevice::now();
        self.iface.poll(now, &mut self.device, &mut self.socket_set)
    }

    /// Start a new async DNS query.  Returns `Some(query)` on success.
    fn start_dns(
        &mut self,
        hostname: String,
        dns_server: smoltcp::wire::Ipv4Address,
    ) -> Option<dns::AsyncDnsQuery> {
        dns::AsyncDnsQuery::start(&mut self.socket_set, dns_server, hostname)
    }

    /// Poll an in-progress async DNS query.
    fn poll_dns(&mut self, query: &mut dns::AsyncDnsQuery) -> dns::DnsProgress {
        query.poll(&mut self.socket_set)
    }

    /// Clean up an async DNS query after completion.
    fn cleanup_dns(&mut self, query: dns::AsyncDnsQuery) {
        query.cleanup(&mut self.socket_set);
    }

    /// Compute smoltcp's next-poll delay hint.
    fn next_poll_delay_ms(&mut self) -> u64 {
        let now = VfsNicDevice::now();
        self.iface.poll_delay(now, &self.socket_set).map(|d| d.total_millis()).unwrap_or(10)
    }

    /// GC closed sockets and push VFS readiness notifications to all
    /// dynamic socket sub-files.
    fn gc_and_notify(&mut self) {
        self.socket_api.gc_closed_sockets(&mut self.socket_set);
        let rw = self.req_write;
        self.socket_api.push_notifications(&mut self.socket_set, rw, TCP_DYN_BASE);
        self.socket_api.push_notifications(&mut self.socket_set, rw, UDP_DYN_BASE);
        self.socket_api.push_notifications(&mut self.socket_set, rw, ICMP_DYN_BASE);
    }

    /// Probe whether the socket allocator is warm (ready flag condition).
    fn probe_ready(&mut self) -> bool {
        probe_socket_allocator_ready(&mut self.socket_api, &mut self.socket_set)
    }

    /// Drain exactly one VFS RPC from `provider`, dispatching it through
    /// the smoltcp stack.  Returns `true` if work was done.
    fn drain_one_rpc(&mut self, provider: &mut NetVfsProvider) -> bool {
        provider.drain_one_rpc(
            &mut self.iface,
            &mut self.device,
            &mut self.socket_set,
            &mut self.socket_api,
        )
    }

    /// Complete a deferred TCP connect (called after DNS resolves).
    fn complete_deferred_connect(
        &mut self,
        provider: &mut NetVfsProvider,
        resolved_ip: Option<smoltcp::wire::Ipv4Address>,
    ) {
        provider.complete_deferred_connect(
            resolved_ip,
            &mut self.iface,
            &mut self.socket_set,
            &mut self.socket_api,
        )
    }
}

// ── Network poll thread ───────────────────────────────────────────────────────

/// Entry point for the network poll thread (runs on `main`).
///
/// Runs `iface.poll` on a ≤4 ms schedule, drives async DNS queries, and
/// pushes socket-readiness notifications.  Commands from the RPC thread
/// arrive via `cmd_queue`; DNS results are returned via `event_queue`.
fn run_poll_thread(
    net_state: Arc<Mutex<NetworkPollState>>,
    cmd_queue: CmdQueue,
    event_queue: EventQueue,
) -> ! {
    // Async DNS state lives entirely on this thread — no sharing needed.
    let mut active_dns: Option<dns::AsyncDnsQuery> = None;
    let mut dns_for_deferred_connect = false;

    loop {
        // ── 1. Drain incoming commands (cmd_queue lock, no net_state lock) ──
        let mut pending_start: Option<(String, smoltcp::wire::Ipv4Address, bool)> = None;
        {
            let mut q = cmd_queue.lock();
            while let Some(cmd) = q.pop_front() {
                if active_dns.is_none() && pending_start.is_none() {
                    match cmd {
                        NetCommand::StartDnsLookup { hostname, dns_server } => {
                            pending_start = Some((hostname, dns_server, false));
                        }
                        NetCommand::StartDeferredConnect { hostname, dns_server } => {
                            pending_start = Some((hostname, dns_server, true));
                        }
                    }
                }
                // Any additional commands while DNS is in-flight are silently
                // dropped here; the RPC thread will re-enqueue once it sees the
                // result and clears its `dns_in_flight` flag.
            }
        }

        // ── 2. Start new DNS query if requested ─────────────────────────────
        if let Some((hostname, dns_server, for_connect)) = pending_start {
            match net_state.lock().start_dns(hostname, dns_server) {
                Some(query) => {
                    active_dns = Some(query);
                    dns_for_deferred_connect = for_connect;
                }
                None => {
                    if for_connect {
                        event_queue
                            .lock()
                            .push_back(NetEvent::DeferredConnectResult { resolved_ip: None });
                    } else {
                        event_queue
                            .lock()
                            .push_back(NetEvent::DnsResult { result: "error".into() });
                    }
                }
            }
        }

        // ── 3. Poll smoltcp network stack ────────────────────────────────────
        let did_poll = net_state.lock().poll_network();

        // ── 4. Poll active DNS query ─────────────────────────────────────────
        let dns_outcome = if let Some(query) = &mut active_dns {
            match net_state.lock().poll_dns(query) {
                dns::DnsProgress::Pending => None,
                dns::DnsProgress::Resolved(ip) => Some(Ok(ip)),
                dns::DnsProgress::Failed(e) => Some(Err(e)),
            }
        } else {
            None
        };

        // ── 5. Handle DNS completion ─────────────────────────────────────────
        if let Some(outcome) = dns_outcome {
            let q = active_dns.take().unwrap();
            net_state.lock().cleanup_dns(q);
            match outcome {
                Ok(ip) if dns_for_deferred_connect => {
                    event_queue
                        .lock()
                        .push_back(NetEvent::DeferredConnectResult { resolved_ip: Some(ip) });
                }
                Ok(ip) => {
                    let b = ip.as_bytes();
                    let ip_str = alloc::format!("{}.{}.{}.{}", b[0], b[1], b[2], b[3]);
                    debug!("NETD poll: DNS resolved → {}", ip_str);
                    event_queue.lock().push_back(NetEvent::DnsResult { result: ip_str });
                }
                Err(_) if dns_for_deferred_connect => {
                    warn!("NETD poll: DNS failed for deferred connect");
                    event_queue
                        .lock()
                        .push_back(NetEvent::DeferredConnectResult { resolved_ip: None });
                }
                Err(e) => {
                    warn!("NETD poll: DNS failed: {:?}", e);
                    event_queue.lock().push_back(NetEvent::DnsResult { result: "error".into() });
                }
            }
        }

        // ── 6. GC, notifications, link state, and ready flag ─────────────────
        net_state.lock().gc_and_notify();

        // Publish ready flag once the socket allocator is warm.
        {
            let mut state = net_state.lock();
            let needs_check = !state.ready_announced && state.ip_configured;
            if needs_check && state.probe_ready() {
                let mp = state.mount_point.clone();
                drop(state);
                publish_ready_flag(&mp);
                info!("NETD: Network ready");
                net_state.lock().ready_announced = true;
                continue; // skip delay — we have fresh work
            }
        }

        // ── 7. Sleep or yield ────────────────────────────────────────────────
        let delay_ms = net_state.lock().next_poll_delay_ms();
        if did_poll || active_dns.is_some() {
            stem::syscall::yield_now();
        } else {
            let sleep_ms = delay_ms.min(4).max(1) as u64;
            stem::time::sleep_ms(sleep_ms);
        }
    }
}

// ── RPC/dispatch thread ───────────────────────────────────────────────────────

/// Entry point for the RPC/dispatch thread.
///
/// Owns `net_provider` exclusively (not shared).  Processes one VFS RPC
/// at a time, releasing the `NetworkPollState` mutex between requests.
/// Forwards DNS/deferred-connect commands to the poll thread via
/// `cmd_queue` and consumes DNS results from `event_queue`.
fn run_rpc_thread(
    mut net_provider: NetVfsProvider,
    net_state: Arc<Mutex<NetworkPollState>>,
    cmd_queue: CmdQueue,
    event_queue: EventQueue,
    req_fd: u32,
    nic_watch_fd: Option<u32>,
) -> ! {
    // `dns_in_flight`: true while a StartDnsLookup or StartDeferredConnect
    // command has been sent to the poll thread and no result has come back.
    // Prevents enqueuing a second DNS command before the first completes.
    let mut dns_in_flight = false;

    loop {
        // ── Consume events from the poll thread ──────────────────────────────
        while let Some(event) = { event_queue.lock().pop_front() } {
            match event {
                NetEvent::DnsResult { result } => {
                    dns_in_flight = false;
                    net_provider.set_dns_result(result);
                }
                NetEvent::DeferredConnectResult { resolved_ip } => {
                    dns_in_flight = false;
                    net_state.lock().complete_deferred_connect(&mut net_provider, resolved_ip);
                }
            }
        }

        // ── Process one VFS RPC (acquire / release lock per RPC) ─────────────
        let did_rpc = match net_provider.try_next_request() {
            Ok(Some((resp_port, op, req_id, payload))) => {
                if op == VfsRpcOp::Lookup {
                    net_provider.op_lookup(resp_port, req_id, &payload);
                } else if op == VfsRpcOp::SubscribeReady {
                    vfs_provider::send_resp(resp_port, req_id, &[E_OK]);
                } else {
                    let mut state_guard = net_state.lock();
                    let state = &mut *state_guard;
                    net_provider.handle_decoded(
                        &mut state.iface,
                        &mut state.device,
                        &mut state.socket_set,
                        &mut state.socket_api,
                        resp_port,
                        op,
                        req_id,
                        &payload,
                    );
                    state.gc_and_notify();
                }
                true
            }
            _ => false,
        };

        // ── Enqueue DNS / deferred-connect commands if needed ────────────────
        if !dns_in_flight {
            if let Some((hostname, dns_ip)) = net_provider.take_deferred_connect_pending() {
                cmd_queue
                    .lock()
                    .push_back(NetCommand::StartDeferredConnect { hostname, dns_server: dns_ip });
                dns_in_flight = true;
            } else if let Some((hostname, dns_ip)) = net_provider.take_dns_pending() {
                cmd_queue
                    .lock()
                    .push_back(NetCommand::StartDnsLookup { hostname, dns_server: dns_ip });
                dns_in_flight = true;
            }
        }

        // ── Periodically sync link state from the device ─────────────────────
        {
            let mut state = net_state.lock();
            let current_link = state.device.link_up();
            if current_link != state.last_link_state {
                state.last_link_state = current_link;
                drop(state);
                net_provider.link_up = current_link;
                debug!("NETD RPC: link state → {}", if current_link { "UP" } else { "DOWN" });
            }
        }

        // ── Handle NIC watch events ───────────────────────────────────────────
        if let Some(fd) = nic_watch_fd {
            drain_watch_fd(fd);
        }

        if did_rpc {
            // Yield to let the poll thread run between RPCs.
            stem::syscall::yield_now();
        } else {
            // No RPC pending — block until the next request arrives (or timeout).
            let mut pollfds =
                [PollHandle { handle: req_fd as i32, events: poll_flags::POLLIN, revents: 0 }];
            let _ = vfs_poll(&mut pollfds, 5); // 5 ms timeout
        }
    }
}

#[stem::main]
fn main(arg: usize) -> ! {
    info!("NETD: Starting network service...");
    clear_ready_flag();
    let cfg = parse_config();
    if cfg.help {
        print_usage();
        exit(0);
    }

    debug!(
        "NETD: startup arg={} mount_point={} oneshot={} provider_prefix={}",
        arg, cfg.mount_point, cfg.oneshot, VIRTIO_PATH_PREFIX
    );

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

    let socket_api = SocketApi::new();

    let mount_point = cfg.mount_point.clone();
    let mut net_provider = loop {
        match NetVfsProvider::new(mac, mtu, initial_link_up) {
            Some(provider) => break provider,
            None => {
                warn!("NETD: Failed to create provider, retrying...");
                stem::time::sleep_ms(200);
            }
        }
    };

    let req_fd = loop {
        match stem::syscall::vfs::vfs_handle_from_port(net_provider.req_read_port()) {
            Ok(fd) => break fd,
            Err(e) => {
                warn!("NETD: failed to bridge request port to fd: {:?}; retrying", e);
                stem::time::sleep_ms(50);
            }
        }
    };

    let nic_watch_fd =
        match vfs_watch_path("/dev/net", watch_mask::ALL_EVENTS, watch_flags::NONBLOCK) {
            Ok(fd) => Some(fd),
            Err(e) => {
                warn!("NETD: Failed to watch /dev/net for NIC registrations: {:?}", e);
                None
            }
        };

    // Now mount the provider just before entering the service loop
    loop {
        if net_provider.mount(&mount_point) {
            debug!("NETD: mounted VFS provider at {} with req_fd={}", mount_point, req_fd);
            break;
        } else {
            warn!("NETD: Failed to mount {}, retrying...", mount_point);
            stem::time::sleep_ms(200);
        }
    }

    // ── Build NetworkPollState ───────────────────────────────────────────────
    // Leak the socket storage so `SocketSet<'static>` can live in the Arc.
    let socket_storage_slice: &'static mut [SocketStorage<'static>] = {
        let mut v: Vec<SocketStorage<'static>> = Vec::with_capacity(256);
        for _ in 0..256usize {
            v.push(SocketStorage::EMPTY);
        }
        Box::leak(v.into_boxed_slice())
    };
    let socket_set: SocketSet<'static> = SocketSet::new(socket_storage_slice);
    let last_link_state = device.link_up();
    let net_state = Arc::new(Mutex::new(NetworkPollState {
        iface,
        device,
        socket_set,
        socket_api,
        last_link_state,
        ip_configured: false,
        ready_announced: false,
        req_write: net_provider.req_write,
        mount_point: mount_point.clone(),
    }));

    let (cmd_queue, event_queue) = new_queues();

    // ── Spawn the RPC/dispatch thread early ──────────────────────────────────
    {
        let net_state_rpc = net_state.clone();
        let cmd_q = cmd_queue.clone();
        let evt_q = event_queue.clone();
        let _ = stem::thread::spawn_task_detached(move || {
            run_rpc_thread(net_provider, net_state_rpc, cmd_q, evt_q, req_fd, nic_watch_fd);
        });
    }

    if cfg.oneshot {
        let mut state_guard = net_state.lock();
        let state = &mut *state_guard;
        match dhcp::run_dhcp(&mut state.iface, &mut state.device) {
            Ok(cfg) => {
                debug!(
                    "NETD: oneshot DHCP result ip={} gateway={} dns={}",
                    cfg.ip, cfg.gateway, cfg.dns
                );
                exit(0);
            }
            Err(e) => {
                warn!("NETD: DHCP failed in oneshot mode: {:?}", e);
                exit(1);
            }
        }
    }

    info!("NETD: Configuring network with DHCP...");
    let dhcp_config = loop {
        let mut state_guard = net_state.lock();
        let state = &mut *state_guard;
        match dhcp::run_dhcp(&mut state.iface, &mut state.device) {
            Ok(cfg) => break cfg,
            Err(e) => {
                warn!("NETD: DHCP failed: {:?}; retrying in 5s", e);
                drop(state_guard);
                stem::time::sleep_ms(5000);
            }
        }
    };

    debug!(
        "NETD: DHCP configured ip={} gateway={} dns={}",
        dhcp_config.ip, dhcp_config.gateway, dhcp_config.dns
    );
    {
        let mut state = net_state.lock();
        state.ip_configured = true;
    }

    // ── Main thread becomes the network poll thread ──────────────────────────
    debug!("NETD: entering network poll loop mount_point={} req_fd={}", mount_point, req_fd);
    run_poll_thread(net_state, cmd_queue, event_queue)
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
    clear_ready_flag();
    match vfs_umount(&mount_point) {
        Ok(()) => 0,
        Err(_) => 1,
    }
}

fn scan_registered_nic_units() -> [bool; MAX_VIRTIO_UNITS as usize] {
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
        for unit in 0..MAX_VIRTIO_UNITS {
            let provider_path = alloc::format!("{}{}", VIRTIO_PATH_PREFIX, unit);
            let rx_path = alloc::format!("{}/rx", provider_path);
            let tx_path = alloc::format!("{}/tx", provider_path);
            let events_path = alloc::format!("{}/events", provider_path);
            let mac_path = alloc::format!("{}/mac", provider_path);
            let mtu_path = alloc::format!("{}/mtu", provider_path);
            let status_path = alloc::format!("{}/status", provider_path);

            stem::trace!("NETD: probing {}", provider_path);
            let rx_fd = match vfs_open(&rx_path, O_RDONLY | O_NONBLOCK) {
                Ok(fd) => fd,
                Err(_) => continue,
            };

            stem::trace!("NETD: found {}/rx, opening companion files", provider_path);
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
