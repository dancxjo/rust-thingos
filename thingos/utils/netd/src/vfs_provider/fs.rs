use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use smoltcp::iface::{Interface, SocketSet};
use smoltcp::wire::Ipv4Address;
use stem::syscall::port::PortHandle;
use stem::warn;

use super::wire::{send_data, send_err, send_handle, send_resp, send_write_ok};
use super::{
    E_INVAL, E_IO, E_NOENT, E_NOTSUP, E_OK, E_ROFS, HANDLE_DNS_DIR, HANDLE_DNS_LOOKUP,
    HANDLE_DNS_SERVER, HANDLE_ETH0_ADDR, HANDLE_ETH0_DIR, HANDLE_ETH0_EVENTS, HANDLE_ETH0_FLAGS,
    HANDLE_ETH0_MTU, HANDLE_ETH0_STATS, HANDLE_ETH0_STATUS, HANDLE_ICMP_DIR, HANDLE_ICMP_NEW,
    HANDLE_INTERFACES_DIR, HANDLE_ROOT, HANDLE_ROUTES, HANDLE_TCP_DIR, HANDLE_TCP_NEW,
    HANDLE_UDP_DIR, HANDLE_UDP_NEW, ICMP_DYN_BASE, NetVfsProvider, POLLIN, ReadResult, S_IFDIR,
    S_IFREG, SF_ACCEPT, SF_CTL, SF_DATA, SF_DIR, SF_EVENTS, SF_STATUS, TCP_DYN_BASE, UDP_DYN_BASE,
    WriteResult,
};
use crate::socket_api::SocketApi;

impl NetVfsProvider {
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

    pub(super) fn op_read(
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

    pub(super) fn op_write<D: smoltcp::phy::Device>(
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

    pub(super) fn op_readdir(
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

    pub(super) fn op_stat(
        &self,
        resp_port: PortHandle,
        req_id: u16,
        payload: &[u8],
        socket_api: &SocketApi,
    ) {
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

    pub(super) fn op_close(
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

    pub(super) fn op_poll(
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

    pub(super) fn eth0_status(&self) -> String {
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

    pub(super) fn eth0_addr_text(&self) -> String {
        match &self.ip_config {
            Some(c) => {
                let b = c.ip.as_bytes();
                alloc::format!("{}.{}.{}.{}/{}\n", b[0], b[1], b[2], b[3], c.prefix_len)
            }
            None => "0.0.0.0/0\n".into(),
        }
    }

    pub(super) fn eth0_stats(&self) -> String {
        alloc::format!(
            "rx_bytes: {}\ntx_bytes: {}\nrx_packets: {}\ntx_packets: {}\n",
            self.rx_bytes,
            self.tx_bytes,
            self.rx_packets,
            self.tx_packets
        )
    }

    pub(super) fn routes_text(&self, _socket_api: &SocketApi) -> String {
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

    pub(super) fn dns_server_text(&self) -> String {
        match &self.ip_config {
            Some(c) => {
                let d = c.dns_server.as_bytes();
                alloc::format!("{}.{}.{}.{}\n", d[0], d[1], d[2], d[3])
            }
            None => "0.0.0.0\n".into(),
        }
    }

    pub(super) fn effective_dns_server(&self) -> Option<Ipv4Address> {
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
