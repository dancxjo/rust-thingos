//! DHCPv4 client using smoltcp.
extern crate alloc;
use core::default::Default;

use smoltcp::iface::{Interface, SocketSet, SocketStorage};
use smoltcp::phy::Device;
use smoltcp::socket::dhcpv4::{Event, Socket as Dhcpv4Socket};
use smoltcp::time::{Duration, Instant};
use smoltcp::wire::{IpCidr, Ipv4Address};

/// Overall DHCP attempt budget before declaring timeout.
const DHCP_TIMEOUT_SECS: u64 = 30;
/// Periodic progress log cadence while waiting for a lease.
const DHCP_PROGRESS_LOG_SECS: u64 = 5;
/// Cap unusually long smoltcp backoff delays so retry cadence stays observable.
const DHCP_MAX_BACKOFF_MS: u64 = 4_000;
/// Keep wakeups responsive while waiting for DHCP events.
const DHCP_POLL_SLICE_MS: u64 = 100;
/// Avoid immediate repeated socket resets while over-cap backoff is observed.
const DHCP_RESET_COOLDOWN_MS: u64 = DHCP_MAX_BACKOFF_MS;

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
    let computed_max_socket_resets = ((DHCP_TIMEOUT_SECS * 1_000) / DHCP_MAX_BACKOFF_MS).max(1) as u32;
    let mut next_progress_log = start + Duration::from_secs(DHCP_PROGRESS_LOG_SECS);
    let mut reset_count = 0u32;
    let mut next_reset_allowed_at = start;
    let mut last_poll_delay_ms: Option<u64> = None;

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
        let poll_delay_ms = delay.map(|d| d.total_millis());
        let wait_ms = poll_delay_ms
            .map(|ms| ms.min(DHCP_POLL_SLICE_MS))
            .unwrap_or(DHCP_POLL_SLICE_MS);
        if poll_delay_ms != last_poll_delay_ms {
            let elapsed_ms = (ts - start).total_millis();
            let now_absolute_ms = ts.total_millis();
            let next_retry_deadline_ms = poll_delay_ms.map(|ms| elapsed_ms + ms);
            stem::debug!(
                "DHCP: state=waiting_lease now_ms={} elapsed_ms={} poll_delay_ms={:?} next_retry_deadline_ms={:?} next_wake_ms={} resets={}/{}",
                now_absolute_ms,
                elapsed_ms,
                poll_delay_ms,
                next_retry_deadline_ms,
                now_absolute_ms + wait_ms,
                reset_count,
                computed_max_socket_resets
            );
            last_poll_delay_ms = poll_delay_ms;
        }

        if let Some(poll_delay_ms) = poll_delay_ms {
            let exceeds_backoff_cap = poll_delay_ms > DHCP_MAX_BACKOFF_MS;
            if exceeds_backoff_cap
                && ts >= next_reset_allowed_at
                && reset_count < computed_max_socket_resets
            {
                reset_count += 1;
                next_reset_allowed_at = ts + Duration::from_millis(DHCP_RESET_COOLDOWN_MS);
                stem::warn!(
                    "DHCP: poll_delay_ms={} exceeds cap={}, resetting DHCP socket (resets={}/{} next_reset_allowed_ms={})",
                    poll_delay_ms,
                    DHCP_MAX_BACKOFF_MS,
                    reset_count,
                    computed_max_socket_resets,
                    next_reset_allowed_at.total_millis()
                );
                // `dhcp_handle` is owned by this loop and always valid here; any
                // failure would indicate a logic bug rather than runtime recovery.
                let _ = socket_set.remove(dhcp_handle);
                dhcp_handle = socket_set.add(Dhcpv4Socket::new());
                continue;
            } else if exceeds_backoff_cap && ts < next_reset_allowed_at {
                stem::debug!(
                    "DHCP: poll_delay_ms={} exceeds cap={}, waiting until reset cooldown expires at {} ms",
                    poll_delay_ms,
                    DHCP_MAX_BACKOFF_MS,
                    next_reset_allowed_at.total_millis()
                );
            } else if exceeds_backoff_cap {
                stem::warn!(
                    "DHCP: poll_delay_ms={} exceeds cap={} but reset budget exhausted (resets={}/{})",
                    poll_delay_ms,
                    DHCP_MAX_BACKOFF_MS,
                    reset_count,
                    computed_max_socket_resets
                );
            }
        }
        let deadline = ts + Duration::from_millis(wait_ms);
        wait_until(deadline);
    }
}
