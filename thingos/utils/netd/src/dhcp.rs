//! DHCPv4 client using smoltcp.
extern crate alloc;
use alloc::string::ToString;
use core::default::Default;

use smoltcp::iface::{Interface, SocketSet, SocketStorage};
use smoltcp::phy::Device;
use smoltcp::socket::dhcpv4::{Event, Socket as Dhcpv4Socket};
use smoltcp::time::{Duration, Instant};
use smoltcp::wire::{IpCidr, Ipv4Address};

const DHCP_TIMEOUT_SECS: u64 = 30;
const DHCP_PROGRESS_LOG_SECS: u64 = 5;
const DHCP_MAX_BACKOFF_MS: i64 = 4_000;
const DHCP_POLL_SLICE_MS: i64 = 100;

#[derive(Debug)]
#[allow(dead_code)]
pub enum DhcpError {
    Timeout,
    Failed,
}

pub struct DhcpConfig {
    pub ip: Ipv4Address,
    pub prefix_len: u8,
    pub gateway: Ipv4Address,
    pub dns: Ipv4Address,
}

fn now() -> Instant {
    Instant::from_millis(stem::time::now().as_millis() as i64)
}

fn wait_until(deadline: Instant) {
    while now() < deadline {
        stem::yield_now();
    }
}

pub fn run_dhcp<D: Device>(iface: &mut Interface, device: &mut D) -> Result<DhcpConfig, DhcpError> {
    let mut sockets_storage: [SocketStorage; 1] = Default::default();
    let mut socket_set = SocketSet::new(&mut sockets_storage[..]);
    let mut dhcp_handle = socket_set.add(Dhcpv4Socket::new());

    stem::debug!("DHCP: Starting discovery...");

    let start = now();
    let timeout = start + Duration::from_secs(DHCP_TIMEOUT_SECS);
    let mut next_progress_log = start + Duration::from_secs(DHCP_PROGRESS_LOG_SECS);
    let mut reset_count = 0u32;
    let mut last_poll_delay_ms = i64::MIN;

    loop {
        let ts = now();
        if ts > timeout {
            return Err(DhcpError::Timeout);
        }

        if ts >= next_progress_log {
            let elapsed_ms = (ts - start).total_millis();
            stem::debug!("DHCP: Still waiting for lease ({} ms elapsed)", elapsed_ms);
            next_progress_log += Duration::from_secs(DHCP_PROGRESS_LOG_SECS);
        }

        let _ = iface.poll(ts, device, &mut socket_set);

        let dhcp_socket = socket_set.get_mut::<Dhcpv4Socket>(dhcp_handle);
        if let Some(event) = dhcp_socket.poll() {
            match event {
                Event::Configured(config) => {
                    stem::debug!("DHCP: Configuration received");

                    let ip = config.address.address();
                    let gateway = config.router.unwrap_or(Ipv4Address::UNSPECIFIED);
                    let dns =
                        config.dns_servers.first().copied().unwrap_or(Ipv4Address::UNSPECIFIED);
                    let prefix_len = config.address.prefix_len();

                    iface.update_ip_addrs(|addrs| {
                        addrs.clear();
                        let _ = addrs.push(IpCidr::Ipv4(config.address));
                    });

                    if let Some(route) = config.router {
                        let _ = iface.routes_mut().add_default_ipv4_route(route);
                    }

                    return Ok(DhcpConfig { ip, prefix_len, gateway, dns });
                }
                Event::Deconfigured => {
                    stem::warn!("DHCP: Deconfigured");
                }
            }
        }

        let delay = iface.poll_delay(ts, &socket_set);
        let poll_delay_ms = delay.map(|d| d.total_millis()).unwrap_or(-1);
        if poll_delay_ms != last_poll_delay_ms {
            let elapsed_ms = (ts - start).total_millis();
            let next_retry_deadline_ms =
                if poll_delay_ms >= 0 { elapsed_ms + poll_delay_ms } else { -1 };
            stem::debug!(
                "DHCP: state=waiting_lease elapsed_ms={} poll_delay_ms={} next_retry_deadline_ms={} resets={}",
                elapsed_ms,
                poll_delay_ms,
                next_retry_deadline_ms,
                reset_count
            );
            last_poll_delay_ms = poll_delay_ms;
        }

        if poll_delay_ms > DHCP_MAX_BACKOFF_MS {
            reset_count = reset_count.saturating_add(1);
            stem::warn!(
                "DHCP: poll_delay_ms={} exceeds cap={}, resetting DHCP socket (resets={})",
                poll_delay_ms,
                DHCP_MAX_BACKOFF_MS,
                reset_count
            );
            let _ = socket_set.remove::<Dhcpv4Socket>(dhcp_handle);
            dhcp_handle = socket_set.add(Dhcpv4Socket::new());
            continue;
        }

        let wait_ms = delay.map(|d| d.total_millis()).unwrap_or(DHCP_POLL_SLICE_MS).min(DHCP_POLL_SLICE_MS);
        let deadline = ts + Duration::from_millis(wait_ms);
        wait_until(deadline);
    }
}
