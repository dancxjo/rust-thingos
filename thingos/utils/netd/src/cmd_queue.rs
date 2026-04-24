//! Thread-safe command and event queues for netd's two-thread model.
//!
//! The **network poll thread** owns the smoltcp stack (`iface`, `device`,
//! `socket_set`, `socket_api`) and runs `iface.poll` on a tight schedule.
//! The **RPC/dispatch thread** decodes VFS provider RPCs, drives DNS
//! requests, and handles link-state events.  The two threads communicate
//! exclusively through these queues — the poll thread never blocks on RPC
//! work and the RPC thread never starves the TCP retransmit engine.
//!
//! Synchronisation rule: **the `CmdQueue` lock and the `EventQueue` lock
//! must never be held at the same time**, and neither may be held while
//! the `NetworkPollState` mutex is also held.  Always acquire, use, and
//! release each lock independently.
extern crate alloc;

use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::sync::Arc;

use smoltcp::wire::Ipv4Address;
use spin::Mutex;

/// Commands sent from the RPC/dispatch thread to the network poll thread.
#[derive(Debug)]
pub enum NetCommand {
    /// Start an async DNS A-record lookup triggered by a client write to
    /// `/net/dns/lookup`.  The resolved address (or "error") is returned
    /// via [`NetEvent::DnsResult`].
    StartDnsLookup { hostname: String, dns_server: Ipv4Address },

    /// Start a DNS A-record lookup on behalf of a deferred TCP connect
    /// (hostname written to `/net/tcp/<id>/ctl connect <host> <port>`).
    /// The outcome is returned via [`NetEvent::DeferredConnectResult`].
    StartDeferredConnect { hostname: String, dns_server: Ipv4Address },
}

/// Events sent from the network poll thread back to the RPC/dispatch thread.
#[derive(Debug)]
pub enum NetEvent {
    /// Result of a [`NetCommand::StartDnsLookup`] query.  `result` is a
    /// dotted-decimal IPv4 address string on success, or `"error"` on failure.
    DnsResult { result: String },

    /// Outcome of a [`NetCommand::StartDeferredConnect`] DNS resolution.
    /// `resolved_ip` is `Some(ip)` on success and `None` on timeout/failure.
    /// The RPC/dispatch thread uses this to call
    /// `NetVfsProvider::complete_deferred_connect`.
    DeferredConnectResult { resolved_ip: Option<Ipv4Address> },
}

/// Thread-safe command queue shared between the RPC thread (producer) and
/// the network poll thread (consumer).
pub type CmdQueue = Arc<Mutex<VecDeque<NetCommand>>>;

/// Thread-safe event queue shared between the network poll thread (producer)
/// and the RPC thread (consumer).
pub type EventQueue = Arc<Mutex<VecDeque<NetEvent>>>;

/// Create a fresh, linked (`CmdQueue`, `EventQueue`) pair.
pub fn new_queues() -> (CmdQueue, EventQueue) {
    (Arc::new(Mutex::new(VecDeque::new())), Arc::new(Mutex::new(VecDeque::new())))
}
