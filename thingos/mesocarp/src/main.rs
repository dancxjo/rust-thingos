//! # mesocarp — mDNS client & server exposing peers as a VFS tree
//!
//! Mesocarp is the link-local hostname service for ThingOS. It does two
//! jobs at once:
//!
//! 1. **mDNS client**: listens on `224.0.0.251:5353` for announcements and
//!    answers from peers, caches their `A` records, and exposes them as a
//!    VFS tree at the mount point (typically `/hosts`).
//!
//!    ```text
//!    $ mount -t mdns /hosts
//!    $ ls /hosts
//!    forebrain.local
//!    selfo.local
//!    $ attr_get /hosts/forebrain.local net.ip.ipv4
//!    # → 192.168.2.23
//!    ```
//!
//! 2. **mDNS server**: reads our own hostname from `/etc/hostname`, our
//!    IPv4 from `/net/interfaces/eth0/addr`, and answers queries from
//!    peers asking for `<hostname>.local.`. It also sends unsolicited
//!    announcements on startup and periodically thereafter, per
//!    RFC 6762 §8.
//!
//! The program is spawned by `mount -t mdns <target>` (see `utils/mount`),
//! which forwards the mount point as `argv[1]`.

#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

mod mdns;
mod provider;

use abi::errors::Errno;
use abi::seed::{
    HOST_PROGRAM, HOST_VFS_PROVIDER, INTERFACE_PROGRAM_V1, INTERFACE_VFS_PROVIDER_MOUNT_V1,
    INTERFACE_VFS_PROVIDER_UNMOUNT_V1, SEED_ABI_VERSION, Seed, SeedInterface,
};
use abi::syscall::vfs_flags::{O_NONBLOCK, O_RDONLY, O_WRONLY};
use abi::vfs_rpc::VfsRpcOp;
use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
use provider::{HostEntry, HostsProvider};
use stem::syscall::vfs::{vfs_mount, vfs_umount};
use stem::syscall::{argv_get, vfs_close, vfs_open, vfs_read, vfs_write};
use stem::{debug, info, warn};

const DEFAULT_MOUNT_POINT: &str = "/hosts";
const HOSTNAME_PATH: &str = "/etc/hostname";
const DEFAULT_HOSTNAME: &str = "thingos";
/// Where we try to read our own IPv4 address. CIDR formatted, e.g. `192.168.2.42/24`.
const IFACE_ADDR_PATH: &str = "/net/interfaces/eth0/addr";
/// IANA-assigned mDNS multicast group.
const MDNS_IPV4: [u8; 4] = [224, 0, 0, 251];
/// IANA-assigned mDNS port.
const MDNS_PORT: u16 = 5353;
/// Interface name we pass to `join_multicast_v4`.
const MDNS_IFACE: &str = "eth0";
/// How often (ms) to send an unsolicited announcement of our own hostname.
const ANNOUNCE_INTERVAL_MS: u64 = 30_000;
/// How often (ms) to retry opening the optional mDNS UDP socket.
const NETD_PROBE_INTERVAL_MS: u64 = 1_000;
/// How often (ms) we sweep the host cache for expired entries.
const EXPIRE_SWEEP_MS: u64 = 1_000;
/// Port buffer capacity for this provider.
const PORT_CAPACITY_BYTES: usize = 32_768;
/// Seed identity.
const SEED_NAME: &[u8] = b"mesocarp";
const HOOK_MOUNT_V1: &[u8] = b"_start";
const HOOK_UNMOUNT_V1: &[u8] = b"thingos_vfs_unmount_v1";

#[no_mangle]
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

#[used]
static KEEP_THINGOS_VFS_MOUNT_V1: extern "C" fn(usize) -> ! = thingos_vfs_mount_v1;
#[used]
static KEEP_THINGOS_VFS_UNMOUNT_V1: extern "C" fn(usize) -> i32 = thingos_vfs_unmount_v1;

#[no_mangle]
pub extern "C" fn thingos_vfs_mount_v1(arg: usize) -> ! {
    unsafe { stem::rt::entry_impl(arg) }
}

#[no_mangle]
pub extern "C" fn thingos_vfs_unmount_v1(_arg: usize) -> i32 {
    let mount_point = mount_point_from_args();
    match vfs_umount(&mount_point) {
        Ok(()) => 0,
        Err(_) => 1,
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let mount_point = mount_point_from_args();
    info!("mesocarp: starting (mount_point={})", mount_point);
    run(&mount_point)
}

fn mount_point_from_args() -> String {
    let len = match argv_get(&mut []) {
        Ok(l) if l > 0 => l,
        _ => return DEFAULT_MOUNT_POINT.to_string(),
    };
    let mut buf = alloc::vec![0u8; len];
    if argv_get(&mut buf).is_err() {
        return DEFAULT_MOUNT_POINT.to_string();
    }
    let args = stem::utils::parse_argv(&buf);
    // mount(8) launches providers as:
    //   /bin/mesocarp <device> <target>
    // while direct/debug invocations may only pass <target>. Prefer the
    // target argument when it is present so fstab's "none /hosts mdns" mounts
    // at /hosts, not at a literal "none" path.
    let mount_arg = if args.len() >= 3 {
        Some(args[2])
    } else if args.len() >= 2 {
        Some(args[1])
    } else {
        None
    };
    if let Some(arg) = mount_arg {
        if let Ok(path) = core::str::from_utf8(arg) {
            let trimmed = path.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }
    DEFAULT_MOUNT_POINT.to_string()
}

/// Read a file in full (up to `max_bytes`), returning its contents or
/// `None` on error. Small helper to avoid pulling in a full FS utility.
fn slurp(path: &str, max_bytes: usize) -> Option<Vec<u8>> {
    let fd = vfs_open(path, O_RDONLY).ok()?;
    let mut out: Vec<u8> = Vec::new();
    let mut buf = [0u8; 512];
    loop {
        match vfs_read(fd, &mut buf) {
            Ok(0) => break,
            Ok(n) => {
                if out.len() + n > max_bytes {
                    let _ = vfs_close(fd);
                    return None;
                }
                out.extend_from_slice(&buf[..n]);
            }
            Err(Errno::EAGAIN) => break,
            Err(_) => {
                let _ = vfs_close(fd);
                return None;
            }
        }
    }
    let _ = vfs_close(fd);
    Some(out)
}

/// Load the configured hostname from `/etc/hostname`. Falls back to
/// [`DEFAULT_HOSTNAME`]. Returns a bare label (without `.local`) that
/// has been normalized to lowercase ASCII.
fn read_hostname() -> String {
    let raw = match slurp(HOSTNAME_PATH, 256) {
        Some(bytes) => bytes,
        None => return DEFAULT_HOSTNAME.to_string(),
    };
    let text = match core::str::from_utf8(&raw) {
        Ok(s) => s,
        Err(_) => return DEFAULT_HOSTNAME.to_string(),
    };
    // Take the first non-empty line, strip whitespace, lowercase, drop a
    // trailing ".local" if someone included one.
    let line = text.lines().map(str::trim).find(|l| !l.is_empty()).unwrap_or(DEFAULT_HOSTNAME);
    let lower = line.to_ascii_lowercase();
    let bare = lower.strip_suffix(".local").unwrap_or(&lower);
    if bare.is_empty() { DEFAULT_HOSTNAME.to_string() } else { bare.to_string() }
}

/// Parse the first dotted-decimal IPv4 in `text`, ignoring an optional
/// `/prefix` suffix.
fn parse_ipv4_line(text: &str) -> Option<[u8; 4]> {
    let addr = text.trim().split('/').next()?.trim();
    let mut out = [0u8; 4];
    let mut parts = addr.split('.');
    for slot in &mut out {
        *slot = parts.next()?.parse().ok()?;
    }
    if parts.next().is_some() {
        return None;
    }
    Some(out)
}

/// Read our current IPv4 address from the eth0 interface file. Returns
/// `None` while the stack is still coming up or the interface is down.
fn read_own_ipv4() -> Option<[u8; 4]> {
    let raw = slurp(IFACE_ADDR_PATH, 128)?;
    let text = core::str::from_utf8(&raw).ok()?;
    for line in text.lines() {
        if let Some(addr) = parse_ipv4_line(line) {
            if addr != [0, 0, 0, 0] {
                return Some(addr);
            }
        }
    }
    None
}

/// A handle to an open UDP socket exposed by `netd` at `/net/udp/<id>/…`.
struct UdpSocket {
    id: u32,
    data_path: String,
}

impl UdpSocket {
    /// Allocate a UDP socket, bind it to `0.0.0.0:5353`, enable broadcast
    /// and multicast, and join the mDNS multicast group.
    fn open_mdns() -> Result<Self, Errno> {
        let id = read_socket_id("/net/udp/new")?;
        let ctl_path = alloc::format!("/net/udp/{}/ctl", id);
        let data_path = alloc::format!("/net/udp/{}/data", id);

        // Best-effort configuration. We only propagate errors from the
        // mandatory `bind`; the other knobs may be no-ops on some stacks.
        write_ctl(&ctl_path, &alloc::format!("bind {}", MDNS_PORT))?;
        let _ = write_ctl(&ctl_path, "multicast_loop_v4 0");
        let _ = write_ctl(&ctl_path, "multicast_ttl_v4 255");
        let group =
            alloc::format!("{}.{}.{}.{}", MDNS_IPV4[0], MDNS_IPV4[1], MDNS_IPV4[2], MDNS_IPV4[3]);
        let _ = write_ctl(&ctl_path, &alloc::format!("join_multicast_v4 {} {}", group, MDNS_IFACE));
        Ok(Self { id, data_path })
    }

    /// Send a UDP payload to `(ipv4, port)`. The netd wire format is
    /// `[4: dest_ipv4][2: dest_port_le][4: payload_len_le][payload]`.
    fn send_to(&self, ipv4: [u8; 4], port: u16, payload: &[u8]) -> Result<(), Errno> {
        let fd = vfs_open(&self.data_path, O_WRONLY)?;
        let mut pkt = alloc::vec![0u8; 10 + payload.len()];
        pkt[..4].copy_from_slice(&ipv4);
        pkt[4..6].copy_from_slice(&port.to_le_bytes());
        pkt[6..10].copy_from_slice(&(payload.len() as u32).to_le_bytes());
        pkt[10..].copy_from_slice(payload);
        let res = vfs_write(fd, &pkt).map(|_| ());
        let _ = vfs_close(fd);
        res
    }

    /// Try to receive one datagram without blocking. Returns
    /// `Ok(Some((src_ip, src_port, payload)))` on a datagram,
    /// `Ok(None)` on "would block".
    fn try_recv(&self) -> Result<Option<([u8; 4], u16, Vec<u8>)>, Errno> {
        let fd = match vfs_open(&self.data_path, O_RDONLY | O_NONBLOCK) {
            Ok(fd) => fd,
            Err(e) => return Err(e),
        };
        let mut buf = alloc::vec![0u8; 4096];
        let result = vfs_read(fd, &mut buf);
        let _ = vfs_close(fd);
        match result {
            Ok(n) if n >= 10 => {
                let src = [buf[0], buf[1], buf[2], buf[3]];
                let port = u16::from_le_bytes([buf[4], buf[5]]);
                let plen = u32::from_le_bytes([buf[6], buf[7], buf[8], buf[9]]) as usize;
                if n < 10 + plen {
                    return Ok(None);
                }
                let payload = buf[10..10 + plen].to_vec();
                Ok(Some((src, port, payload)))
            }
            Ok(_) => Ok(None),
            Err(Errno::EAGAIN) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

impl Drop for UdpSocket {
    fn drop(&mut self) {
        let ctl_path = alloc::format!("/net/udp/{}/ctl", self.id);
        let _ = write_ctl(&ctl_path, "close");
    }
}

/// Read a short decimal integer from a file (as produced by `/net/udp/new`).
fn read_socket_id(path: &str) -> Result<u32, Errno> {
    let fd = vfs_open(path, O_RDONLY)?;
    let mut buf = [0u8; 32];
    let n = vfs_read(fd, &mut buf).unwrap_or(0);
    let _ = vfs_close(fd);
    let s = core::str::from_utf8(&buf[..n]).map_err(|_| Errno::EINVAL)?;
    s.trim().parse::<u32>().map_err(|_| Errno::EINVAL)
}

fn write_ctl(path: &str, cmd: &str) -> Result<(), Errno> {
    let fd = vfs_open(path, O_WRONLY)?;
    let res = vfs_write(fd, cmd.as_bytes()).map(|_| ());
    let _ = vfs_close(fd);
    res
}

/// One-shot setup + main event loop.
fn run(mount_point: &str) -> ! {
    // Stand up the VFS side first — if we're invoked by `mount` it expects
    // the target path to become openable within a bounded timeout.
    let (req_write, req_read) = match stem::syscall::port::port_create(PORT_CAPACITY_BYTES) {
        Ok(p) => p,
        Err(e) => {
            warn!("mesocarp: port_create failed: {:?}", e);
            stem::syscall::exit(1);
        }
    };
    if let Err(e) = vfs_mount(req_write, mount_point) {
        warn!("mesocarp: mount at {} failed: {:?}", mount_point, e);
        stem::syscall::exit(1);
    }
    info!("mesocarp: mounted at {}", mount_point);

    let mut provider = HostsProvider::new();
    let mut prov_loop = ProviderLoop::new(req_read);

    // Publish our own hostname as an always-present entry as soon as we
    // know our IP; refresh it on every loop iteration.
    let hostname_label = read_hostname();
    let self_name = alloc::format!("{}.local", hostname_label);
    info!("mesocarp: advertising hostname {}", self_name);
    provider.upsert(HostEntry { name: self_name.clone(), ipv4: [0, 0, 0, 0], expires_at_ms: None });

    info!("mesocarp: serving cache immediately; mDNS socket will attach when netd is ready");

    let mut udp: Option<UdpSocket> = None;
    let mut last_netd_probe_ms: u64 = 0;
    let mut last_announce_ms: u64 = 0;
    let mut last_expire_ms: u64 = 0;

    loop {
        let now = stem::time::monotonic_ns() / 1_000_000;

        // 1. Handle any pending VFS RPC requests. Non-blocking so we can
        //    interleave with network I/O.
        loop {
            match prov_loop.try_next_request() {
                Ok(Some(req)) => {
                    let resp = dispatch(&mut provider, req.op, &req.payload);
                    if let Err(e) = prov_loop.send_response(req.resp_port, resp) {
                        debug!("mesocarp: send_response failed: {:?}", e);
                    }
                }
                Ok(None) => break,
                Err(Errno::EPIPE) => {
                    info!("mesocarp: request port closed, exiting");
                    stem::syscall::exit(0);
                }
                Err(e) => {
                    warn!("mesocarp: rpc loop error: {:?}", e);
                    break;
                }
            }
        }

        // 2. Refresh self-entry so the cached name→ip mapping tracks IP
        //    changes (e.g. DHCP renewals).
        if let Some(ip) = read_own_ipv4() {
            provider.upsert(HostEntry {
                name: self_name.clone(),
                ipv4: ip,
                expires_at_ms: None, // pinned; we own this
            });
        }

        // 3. Attach to netd once it is ready. This must not block VFS serving:
        // `/hosts` should answer local cache requests even while networking
        // is still starting.
        if udp.is_none()
            && (last_netd_probe_ms == 0
                || now.saturating_sub(last_netd_probe_ms) >= NETD_PROBE_INTERVAL_MS)
        {
            last_netd_probe_ms = now;
            match UdpSocket::open_mdns() {
                Ok(s) => {
                    info!("mesocarp: mDNS socket ready");
                    udp = Some(s);
                }
                Err(Errno::ENOENT) | Err(Errno::ENODEV) | Err(Errno::EAGAIN) => {}
                Err(e) => debug!("mesocarp: mDNS socket not ready: {:?}", e),
            }
        }

        // 4. Drain inbound mDNS datagrams.
        if let Some(sock) = udp.as_mut() {
            loop {
                match sock.try_recv() {
                    Ok(Some((src, port, payload))) => {
                        if port != MDNS_PORT {
                            continue;
                        }
                        handle_packet(&mut provider, sock, &self_name, &payload, src, now);
                    }
                    Ok(None) => break,
                    Err(e) => {
                        debug!("mesocarp: udp recv error: {:?}", e);
                        break;
                    }
                }
            }
        }

        // 5. Periodic unsolicited announcement of our own hostname.
        if let Some(sock) = udp.as_ref() {
            if now.saturating_sub(last_announce_ms) >= ANNOUNCE_INTERVAL_MS || last_announce_ms == 0
            {
                if let Some(ip) = read_own_ipv4() {
                    let pkt = mdns::build_a_response(&self_name, ip, mdns::DEFAULT_TTL);
                    if let Err(e) = sock.send_to(MDNS_IPV4, MDNS_PORT, &pkt) {
                        debug!("mesocarp: announce send failed: {:?}", e);
                    } else {
                        debug!(
                            "mesocarp: announced {} → {}.{}.{}.{}",
                            self_name, ip[0], ip[1], ip[2], ip[3]
                        );
                    }
                }
                last_announce_ms = now;
            }
        }

        // 6. Expire stale peers.
        if now.saturating_sub(last_expire_ms) >= EXPIRE_SWEEP_MS {
            provider.expire(now);
            last_expire_ms = now;
        }

        // Cooperative idle — keep the loop responsive but not busy.
        stem::time::sleep_ms(50);
    }
}

/// Handle one inbound mDNS UDP payload. Updates the cache for answers we
/// care about and replies to queries about our own hostname.
fn handle_packet(
    provider: &mut HostsProvider,
    sock: &UdpSocket,
    self_name: &str,
    payload: &[u8],
    src: [u8; 4],
    now_ms: u64,
) {
    let msg = match mdns::parse(payload) {
        Some(m) => m,
        None => return,
    };

    // Respond to A-record queries for our own name.
    if !msg.is_response() {
        let self_key = mdns::normalize_name(self_name);
        for q in &msg.questions {
            if q.rtype != mdns::RTYPE_A {
                continue;
            }
            if mdns::normalize_name(&q.name) != self_key {
                continue;
            }
            if let Some(ip) = read_own_ipv4() {
                let pkt = mdns::build_a_response(self_name, ip, mdns::DEFAULT_TTL);
                let _ = sock.send_to(MDNS_IPV4, MDNS_PORT, &pkt);
                debug!("mesocarp: answered query from {:?} for {}", src, self_name);
            }
        }
        return;
    }

    // Cache A-record answers from peers. Ignore records that aren't in the
    // `.local` domain — those belong to unicast DNS, not us.
    for a in &msg.answers {
        if a.rtype != mdns::RTYPE_A || a.rdata.len() != 4 {
            continue;
        }
        if !mdns::is_local_name(&a.name) {
            continue;
        }
        let ip = [a.rdata[0], a.rdata[1], a.rdata[2], a.rdata[3]];
        let name = mdns::normalize_name(&a.name);
        if name.is_empty() {
            continue;
        }
        // Don't clobber our own pinned self entry if a peer echoes it.
        if name == mdns::normalize_name(self_name) {
            continue;
        }
        // Clamp TTL to a sane upper bound to keep the cache bounded.
        let ttl_ms = (a.ttl.min(3_600) as u64) * 1_000;
        let expires = now_ms.saturating_add(ttl_ms.max(1_000));
        let is_new = provider.upsert(HostEntry {
            name: name.clone(),
            ipv4: ip,
            expires_at_ms: Some(expires),
        });
        if is_new {
            info!(
                "mesocarp: learned {} → {}.{}.{}.{} (from {:?})",
                name, ip[0], ip[1], ip[2], ip[3], src
            );
        }
    }
}

/// Dispatch one VFS RPC request to the [`HostsProvider`].
fn dispatch(provider: &mut HostsProvider, op: VfsRpcOp, payload: &[u8]) -> ProviderResponse {
    match op {
        VfsRpcOp::Lookup => dispatch_lookup(provider, payload),
        VfsRpcOp::Stat => dispatch_stat(provider, payload),
        VfsRpcOp::Readdir => dispatch_readdir(provider, payload),
        VfsRpcOp::Read => dispatch_read(provider, payload),
        VfsRpcOp::Close => ProviderResponse::ok_empty(),
        VfsRpcOp::Poll => dispatch_poll(provider, payload),
        VfsRpcOp::AttrGet => dispatch_attr_get(provider, payload),
        VfsRpcOp::AttrList => dispatch_attr_list(provider, payload),
        VfsRpcOp::SubscribeReady | VfsRpcOp::UnsubscribeReady => ProviderResponse::ok_empty(),
        VfsRpcOp::Write | VfsRpcOp::Rename | VfsRpcOp::AttrSet | VfsRpcOp::AttrRemove => {
            ProviderResponse::err(Errno::EROFS)
        }
        VfsRpcOp::DeviceCall | VfsRpcOp::Readlink => ProviderResponse::err(Errno::ENOSYS),
    }
}

fn dispatch_lookup(provider: &mut HostsProvider, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 4 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let path_len = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]) as usize;
    if payload.len() < 4 + path_len {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let Ok(path) = core::str::from_utf8(&payload[4..4 + path_len]) else {
        return ProviderResponse::err(Errno::EINVAL);
    };
    provider.handle_lookup(path)
}

fn dispatch_stat(provider: &HostsProvider, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 8 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
    provider.handle_stat(handle)
}

fn dispatch_readdir(provider: &HostsProvider, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap_or([0; 8]));
    provider.handle_readdir(handle, offset)
}

fn dispatch_read(provider: &HostsProvider, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap_or([0; 8]));
    let max_len = u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4]));
    provider.handle_read(handle, offset, max_len)
}

fn dispatch_poll(provider: &HostsProvider, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 12 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    provider.handle_poll(handle)
}

fn dispatch_attr_get(provider: &HostsProvider, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 8 + 2 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
    let name_len = u16::from_le_bytes([payload[8], payload[9]]) as usize;
    if payload.len() < 8 + 2 + name_len {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let Ok(name) = core::str::from_utf8(&payload[10..10 + name_len]) else {
        return ProviderResponse::err(Errno::EINVAL);
    };
    provider.handle_attr_get(handle, name)
}

fn dispatch_attr_list(provider: &HostsProvider, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 8 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
    provider.handle_attr_list(handle)
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;

    #[test]
    fn parse_ipv4_line_handles_cidr_suffix() {
        assert_eq!(parse_ipv4_line("192.168.2.23/24"), Some([192, 168, 2, 23]));
        assert_eq!(parse_ipv4_line(" 10.0.0.1 "), Some([10, 0, 0, 1]));
        assert_eq!(parse_ipv4_line("bogus"), None);
        assert_eq!(parse_ipv4_line("1.2.3.4.5"), None);
    }
}
