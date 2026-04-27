use alloc::string::ToString;
use alloc::vec::Vec;

use smoltcp::iface::{Interface, SocketSet};
use smoltcp::wire::{IpAddress, IpCidr, Ipv4Address};
use stem::syscall::port::PortHandle;

use super::{
    DeferredConnect, HANDLE_DNS_LOOKUP, HANDLE_ETH0_ADDR, HANDLE_ETH0_EVENTS, HANDLE_ETH0_FLAGS,
    HANDLE_ETH0_MTU, HANDLE_ETH0_STATS, HANDLE_ETH0_STATUS, HANDLE_ROUTES, ICMP_DYN_BASE, IpConfig,
    NetVfsProvider, SF_CTL, SF_DATA, TCP_DYN_BASE, UDP_DYN_BASE, WriteResult, parse_ipv4,
};
use crate::socket_api::SocketApi;

impl NetVfsProvider {
    // ── Handle writes ─────────────────────────────────────────────────────────

    pub(super) fn write_handle<D: smoltcp::phy::Device>(
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
        _device: &mut D,
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
        _resp_port: PortHandle,
        _req_id: u16,
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
        _resp_port: PortHandle,
        _req_id: u16,
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
}
