use alloc::vec::Vec;

use smoltcp::iface::SocketSet;
use stem::warn;

use super::{
    HANDLE_DNS_DIR, HANDLE_DNS_LOOKUP, HANDLE_DNS_SERVER, HANDLE_ETH0_ADDR, HANDLE_ETH0_DIR,
    HANDLE_ETH0_EVENTS, HANDLE_ETH0_MTU, HANDLE_ETH0_STATS, HANDLE_ETH0_STATUS, HANDLE_ICMP_DIR,
    HANDLE_ICMP_NEW, HANDLE_INTERFACES_DIR, HANDLE_ROOT, HANDLE_ROUTES, HANDLE_TCP_DIR,
    HANDLE_TCP_NEW, HANDLE_UDP_DIR, HANDLE_UDP_NEW, ICMP_DYN_BASE, NetVfsProvider, ReadResult,
    SF_ACCEPT, SF_DATA, SF_DIR, SF_EVENTS, SF_STATUS, TCP_DYN_BASE, UDP_DYN_BASE,
};
use crate::socket_api::SocketApi;

impl NetVfsProvider {
    // ── Handle reads ─────────────────────────────────────────────────────────

    pub(super) fn read_handle(
        &mut self,
        handle: u64,
        offset: u64,
        len: usize,
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) -> ReadResult {
        match handle {
            HANDLE_ETH0_STATUS => ReadResult::text_offset(&self.eth0_status(), offset),
            HANDLE_ETH0_ADDR => ReadResult::text_offset(&self.eth0_addr_text(), offset),
            HANDLE_ETH0_MTU => ReadResult::text_offset(&alloc::format!("{}\n", self.mtu), offset),
            HANDLE_ETH0_STATS => ReadResult::text_offset(&self.eth0_stats(), offset),
            HANDLE_ETH0_EVENTS => {
                // Events are single-shot; subsequent reads return EOF until next event.
                if offset == 0 {
                    let s = if self.link_up { "link-up\n" } else { "link-down\n" };
                    ReadResult::Data(s.as_bytes().to_vec())
                } else {
                    ReadResult::EOF
                }
            }
            HANDLE_ROUTES => ReadResult::text_offset(&self.routes_text(socket_api), offset),
            HANDLE_DNS_SERVER => ReadResult::text_offset(&self.dns_server_text(), offset),
            // tcp/new: allocate a new TCP socket, return its id as text
            HANDLE_TCP_NEW => {
                if offset > 0 {
                    return ReadResult::EOF;
                }
                let Some(buf_idx) = socket_api.alloc_buffer() else {
                    warn!("NetVfsProvider: out of socket buffers for tcp/new");
                    return ReadResult::Error;
                };
                match socket_api.alloc_tcp_socket_raw(socket_set, buf_idx) {
                    Some(id) => ReadResult::Data(alloc::format!("{}\n", id).into_bytes()),
                    None => ReadResult::Error,
                }
            }
            // udp/new: allocate a new UDP socket
            HANDLE_UDP_NEW => {
                if offset > 0 {
                    return ReadResult::EOF;
                }
                let Some(buf_idx) = socket_api.alloc_buffer() else {
                    warn!("NetVfsProvider: out of socket buffers for udp/new");
                    return ReadResult::Error;
                };
                let api_handle = socket_api.alloc_udp_socket_raw(socket_set, buf_idx);
                match api_handle {
                    Some(id) => {
                        let text = alloc::format!("{}\n", id);
                        ReadResult::Data(text.into_bytes())
                    }
                    None => ReadResult::Error,
                }
            }
            HANDLE_ICMP_NEW => {
                if offset > 0 {
                    return ReadResult::EOF;
                }
                let Some(buf_idx) = socket_api.alloc_buffer() else {
                    warn!("NetVfsProvider: out of socket buffers for icmp/new");
                    return ReadResult::Error;
                };
                match socket_api.alloc_icmp_socket_raw(socket_set, buf_idx) {
                    Some(id) => ReadResult::Data(alloc::format!("{}\n", id).into_bytes()),
                    None => ReadResult::Error,
                }
            }
            // Dynamic TCP data
            h if h >= TCP_DYN_BASE && h < UDP_DYN_BASE => {
                let sf = (h & 0xFF) as u8;
                let api_handle = ((h - TCP_DYN_BASE) >> 8) as u32;
                self.read_tcp(api_handle, sf, offset, len, socket_set, socket_api)
            }
            // Dynamic UDP data
            h if h >= UDP_DYN_BASE && h < ICMP_DYN_BASE => {
                let sf = (h & 0xFF) as u8;
                let api_handle = ((h - UDP_DYN_BASE) >> 8) as u32;
                self.read_udp(api_handle, sf, offset, socket_set, socket_api)
            }
            h if h >= ICMP_DYN_BASE => {
                let sf = (h & 0xFF) as u8;
                let api_handle = ((h - ICMP_DYN_BASE) >> 8) as u32;
                self.read_icmp(api_handle, sf, offset, socket_set, socket_api)
            }
            // Directories are not readable as byte streams
            HANDLE_ROOT
            | HANDLE_INTERFACES_DIR
            | HANDLE_ETH0_DIR
            | HANDLE_TCP_DIR
            | HANDLE_UDP_DIR
            | HANDLE_ICMP_DIR
            | HANDLE_DNS_DIR => ReadResult::NotSupported,
            // dns/lookup: returns the resolved IP (EAGAIN if not yet resolved)
            HANDLE_DNS_LOOKUP => {
                if offset > 0 {
                    return ReadResult::EOF;
                }
                match &self.dns_result {
                    Some(result) if result != "error" => {
                        let text = result.clone() + "\n";
                        self.dns_result = None; // consume result
                        self.dns_pending = None;
                        ReadResult::Data(text.into_bytes())
                    }
                    Some(_) => {
                        self.dns_result = None;
                        self.dns_pending = None;
                        ReadResult::Error
                    }
                    None => ReadResult::Again, // EAGAIN: resolution in progress
                }
            }
            _ => ReadResult::Error,
        }
    }

    fn read_tcp(
        &mut self,
        api_handle: u32,
        sf: u8,
        offset: u64,
        len: usize,
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) -> ReadResult {
        match sf {
            SF_DIR => ReadResult::NotSupported,
            SF_STATUS => {
                ReadResult::text_offset(&socket_api.tcp_status_text(api_handle, socket_set), offset)
            }
            SF_DATA => {
                if len == 0 {
                    return ReadResult::Data(Vec::new());
                }
                let recv_len = len.min(u16::MAX as usize) as u16;
                let recv = socket_api.handle_recv(socket_set, api_handle, recv_len);
                // handle_recv returns [resp_type: u16][data...]
                if recv.len() < 2 {
                    return ReadResult::Error;
                }
                let resp_type = u16::from_le_bytes([recv[0], recv[1]]);
                match resp_type {
                    crate::socket_api::RESP_DATA => {
                        self.rx_bytes += (recv.len() - 2) as u64;
                        self.rx_packets += 1;
                        ReadResult::Data(recv[2..].to_vec())
                    }
                    crate::socket_api::RESP_EMPTY => ReadResult::Again,
                    crate::socket_api::RESP_CLOSED => ReadResult::EOF,
                    _ => ReadResult::Error,
                }
            }
            SF_EVENTS => {
                ReadResult::text_offset(&socket_api.tcp_events_text(api_handle, socket_set), offset)
            }
            SF_ACCEPT => {
                // Listener sockets: returns "<conn_id> <ip> <port>\n" when a
                // connection is ready, or EAGAIN when none is queued.
                let Some(buf_idx) = socket_api.alloc_buffer() else {
                    return ReadResult::Error;
                };
                let result = socket_api.handle_accept(socket_set, api_handle, 0, buf_idx);
                if result.len() < 2 {
                    socket_api.free_buffer(buf_idx);
                    return ReadResult::Error;
                }
                let resp_type = u16::from_le_bytes([result[0], result[1]]);
                match resp_type {
                    crate::socket_api::RESP_ACCEPT => {
                        // [2: RESP_ACCEPT][4: conn_handle][4: remote_ip][2: remote_port]
                        if result.len() < 12 {
                            return ReadResult::Error;
                        }
                        let conn_handle = u32::from_le_bytes(result[2..6].try_into().unwrap());
                        let ip = &result[6..10];
                        let port = u16::from_le_bytes(result[10..12].try_into().unwrap());
                        let text = alloc::format!(
                            "{} {}.{}.{}.{} {}\n",
                            conn_handle,
                            ip[0],
                            ip[1],
                            ip[2],
                            ip[3],
                            port
                        );
                        ReadResult::Data(text.into_bytes())
                    }
                    crate::socket_api::RESP_EMPTY => ReadResult::Again,
                    _ => ReadResult::Error,
                }
            }
            _ => ReadResult::Error,
        }
    }

    fn read_udp(
        &mut self,
        api_handle: u32,
        sf: u8,
        offset: u64,
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) -> ReadResult {
        match sf {
            SF_DIR => ReadResult::NotSupported,
            SF_STATUS => ReadResult::text_offset(&socket_api.udp_status_text(api_handle), offset),
            SF_DATA => {
                let recv = socket_api.handle_udp_recv_from(socket_set, api_handle);
                if recv.len() < 2 {
                    return ReadResult::Error;
                }
                let resp_type = u16::from_le_bytes([recv[0], recv[1]]);
                match resp_type {
                    crate::socket_api::RESP_DATA => {
                        // encode_udp_data format: [2: RESP_DATA][4: src_ip][2: src_port][payload]
                        // New wire format: [4: src_ip][2: src_port][4: payload_len][payload]
                        if recv.len() < 8 {
                            return ReadResult::Error;
                        }
                        let src_ip = &recv[2..6];
                        let src_port = u16::from_le_bytes([recv[6], recv[7]]);
                        let payload = &recv[8..];
                        self.rx_bytes += payload.len() as u64;
                        self.rx_packets += 1;
                        let mut out = alloc::vec![0u8; 4 + 2 + 4 + payload.len()];
                        out[..4].copy_from_slice(src_ip);
                        out[4..6].copy_from_slice(&src_port.to_le_bytes());
                        out[6..10].copy_from_slice(&(payload.len() as u32).to_le_bytes());
                        out[10..].copy_from_slice(payload);
                        ReadResult::Data(out)
                    }
                    crate::socket_api::RESP_EMPTY => ReadResult::Again,
                    _ => ReadResult::Error,
                }
            }
            _ => ReadResult::Error,
        }
    }

    fn read_icmp(
        &mut self,
        api_handle: u32,
        sf: u8,
        offset: u64,
        socket_set: &mut SocketSet,
        socket_api: &mut SocketApi,
    ) -> ReadResult {
        match sf {
            SF_DIR => ReadResult::NotSupported,
            SF_STATUS => ReadResult::text_offset(
                &socket_api.icmp_status_text(api_handle, socket_set),
                offset,
            ),
            SF_DATA => {
                let recv = socket_api.handle_icmp_recv_from(socket_set, api_handle);
                if recv.len() < 2 {
                    return ReadResult::Error;
                }
                let resp_type = u16::from_le_bytes([recv[0], recv[1]]);
                match resp_type {
                    crate::socket_api::RESP_DATA => {
                        if recv.len() < 6 {
                            return ReadResult::Error;
                        }
                        let src_ip = &recv[2..6];
                        let payload = &recv[6..];
                        self.rx_bytes += payload.len() as u64;
                        self.rx_packets += 1;
                        let mut out = alloc::vec![0u8; 4 + 4 + payload.len()];
                        out[..4].copy_from_slice(src_ip);
                        out[4..8].copy_from_slice(&(payload.len() as u32).to_le_bytes());
                        out[8..].copy_from_slice(payload);
                        ReadResult::Data(out)
                    }
                    crate::socket_api::RESP_EMPTY => ReadResult::Again,
                    _ => ReadResult::Error,
                }
            }
            _ => ReadResult::Error,
        }
    }
}
