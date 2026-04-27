use abi::vfs_rpc::VfsRpcReqHeader;
use smoltcp::wire::Ipv4Address;

use super::{IpConfig, NetVfsProvider, parse_rpc_header};

fn provider_with_config(dns_server: Ipv4Address, gateway: Ipv4Address) -> NetVfsProvider {
    NetVfsProvider {
        req_read: 0,
        req_write: 0,
        mac: [0; 6],
        mtu: 1500,
        link_up: true,
        ip_config: Some(IpConfig {
            ip: Ipv4Address::new(10, 0, 2, 15),
            prefix_len: 24,
            gateway,
            dns_server,
        }),
        rx_bytes: 0,
        tx_bytes: 0,
        rx_packets: 0,
        tx_packets: 0,
        req_buf: alloc::vec![0u8; 32],
        pending: alloc::vec![],
        dns_pending: None,
        dns_result: None,
        deferred_connects: alloc::vec![],
    }
}

#[test]
fn effective_dns_server_prefers_configured_dns() {
    let provider =
        provider_with_config(Ipv4Address::new(1, 1, 1, 1), Ipv4Address::new(10, 0, 2, 2));
    assert_eq!(provider.effective_dns_server(), Some(Ipv4Address::new(1, 1, 1, 1)));
}

#[test]
fn effective_dns_server_maps_qemu_usernet_gateway_to_slirp_dns() {
    let provider = provider_with_config(Ipv4Address::UNSPECIFIED, Ipv4Address::new(10, 0, 2, 2));
    assert_eq!(provider.effective_dns_server(), Some(Ipv4Address::new(10, 0, 2, 3)));
}

#[test]
fn effective_dns_server_falls_back_to_gateway_for_other_networks() {
    let provider = provider_with_config(Ipv4Address::UNSPECIFIED, Ipv4Address::new(192, 168, 1, 1));
    assert_eq!(provider.effective_dns_server(), Some(Ipv4Address::new(192, 168, 1, 1)));
}

#[test]
fn parse_rpc_header_rejects_short_frames() {
    let short = [0u8; 4];
    assert!(parse_rpc_header(&short).is_none());
}

#[test]
fn parse_rpc_header_returns_port_op_and_payload() {
    let mut req = alloc::vec![0u8; core::mem::size_of::<VfsRpcReqHeader>() + 3];
    req[0..4].copy_from_slice(&7u32.to_le_bytes());
    req[4] = 3;
    req[5..7].copy_from_slice(&0x1234u16.to_le_bytes());
    req[core::mem::size_of::<VfsRpcReqHeader>()..].copy_from_slice(&[0xAA, 0xBB, 0xCC]);

    let (resp_port, op, req_id, payload) = parse_rpc_header(&req).expect("expected valid header");
    assert_eq!(resp_port, 7);
    assert_eq!(op, 3);
    assert_eq!(req_id, 0x1234);
    assert_eq!(payload, &[0xAA, 0xBB, 0xCC]);
}
