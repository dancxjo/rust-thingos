//! vfs_test_provider — synthetic VFS provider for ServiceLoop stress and fault testing.
//!
//! Mounts at `/dev/test/provider` and exposes a set of test endpoints designed
//! to exercise edge cases in ServiceLoop and VFS provider handling:
//!
//! ```text
//! /dev/test/provider/
//!   fast       — responds immediately
//!   slow       — delayed response (~50 ms)
//!   hang       — never responds (tests timeout/stuck-waiter handling)
//!   burst      — high-frequency responses (stress test)
//!   malformed  — returns invalid/unexpected payloads
//!   large      — returns a large (~32 KiB) response
//! ```
//!
//! # Purpose
//!
//! This is a **test-only** daemon.  It is intended to:
//!
//! - Stress-test `ServiceProviderLoop` under concurrent load.
//! - Reproduce and diagnose timeout, hang, and malformed-payload failure modes.
//! - Serve as a target for BDD scenarios in `docs/behavior/features/vfs-test-provider.feature`.
//!
//! # Usage
//!
//! ```text
//! # In the Thing-OS shell:
//! $ vfs_test_provider &
//! $ cat /dev/test/provider/fast
//! $ cat /dev/test/provider/large
//! ```
//!
//! # See Also
//!
//! `libs/ipc_helpers/src/service_provider.rs` — `ServiceProviderLoop`.
//! `docs/behavior/features/vfs-test-provider.feature` — BDD acceptance scenarios.
#![no_std]
#![no_main]
extern crate alloc;

use alloc::vec::Vec;

use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp;
use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
use ipc_helpers::service_provider::{ServiceProviderEvent, ServiceProviderLoop};
use stem::syscall::{port_create, vfs_mount};
use stem::{info, warn};

// ─── Mount point ─────────────────────────────────────────────────────────────

const MOUNT_POINT: &str = "/dev/test/provider";

// ─── File handles ────────────────────────────────────────────────────────────
//
// Handle 0 is reserved for the root directory.
// Each virtual file gets a stable non-zero handle.

const HANDLE_ROOT: u64 = 0;
const HANDLE_FAST: u64 = 1;
const HANDLE_SLOW: u64 = 2;
const HANDLE_HANG: u64 = 3;
const HANDLE_BURST: u64 = 4;
const HANDLE_MALFORMED: u64 = 5;
const HANDLE_LARGE: u64 = 6;

// ─── Endpoint content ────────────────────────────────────────────────────────

const FAST_CONTENT: &[u8] = b"FAST: immediate response\n";
const SLOW_DELAY_MS: u64 = 50;
const SLOW_CONTENT: &[u8] = b"SLOW: delayed response\n";
const BURST_CONTENT: &[u8] = b"BURST: high-frequency response\n";

/// A deliberately oversized payload (~32 KiB) for the `/large` endpoint.
///
/// We fill the static buffer with a repeating ASCII pattern so content is
/// predictable in tests.
const LARGE_SIZE: usize = 32 * 1024;

// ─── Directory listing ───────────────────────────────────────────────────────

/// All virtual entries in the root directory.
static ENTRIES: &[(&[u8], u64)] = &[
    (b"fast", HANDLE_FAST),
    (b"slow", HANDLE_SLOW),
    (b"hang", HANDLE_HANG),
    (b"burst", HANDLE_BURST),
    (b"malformed", HANDLE_MALFORMED),
    (b"large", HANDLE_LARGE),
];

// ─── Provider state ──────────────────────────────────────────────────────────

struct ProviderState {
    request_count: u64,
    /// Pending hang requests — stored as (resp_port, req_id).
    /// These are intentionally never answered to simulate a hung client.
    hang_pending: Vec<(u32, u16)>,
}

// ─── Entry point ─────────────────────────────────────────────────────────────

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("vfs_test_provider: starting up");

    let mut state = ProviderState { request_count: 0, hang_pending: Vec::new() };

    // 1. Create provider port pair.
    let (write_h, read_h) = match port_create(65536) {
        Ok(pair) => pair,
        Err(e) => {
            warn!("vfs_test_provider: port_create failed: {:?}", e);
            stem::syscall::exit(1);
        }
    };
    info!("vfs_test_provider: port pair write_h={} read_h={}", write_h, read_h);

    // 2. Mount at /dev/test/provider.
    match vfs_mount(write_h, MOUNT_POINT) {
        Ok(()) => info!("vfs_test_provider: mounted at {}", MOUNT_POINT),
        Err(e) => {
            warn!("vfs_test_provider: vfs_mount failed: {:?} (continuing anyway for testing)", e);
        }
    }

    // 3. Build inbox-aware event loop.
    let mut svc = match ServiceProviderLoop::new(ProviderLoop::new(read_h), 4096) {
        Ok(s) => s,
        Err(e) => {
            warn!("vfs_test_provider: ServiceProviderLoop::new failed: {:?}", e);
            stem::syscall::exit(1);
        }
    };

    svc.register_mount_path(MOUNT_POINT);
    info!("vfs_test_provider: entering event loop");

    // Manual event loop — `run_until_shutdown` is not used here because we
    // need to call `svc.send_response()` from inside the loop body, which
    // requires a mutable borrow of `svc` that a closure cannot provide.
    loop {
        match svc.next_event(None) {
            Ok(ServiceProviderEvent::ProviderRequest(req)) => {
                state.request_count += 1;
                info!("vfs_test_provider: request #{} op={:?}", state.request_count, req.op);

                // Hang requests: deliberately skip send_response so the
                // client waits indefinitely — this tests timeout/stuck-waiter
                // handling in callers.
                if is_hang_request(&req.op, &req.payload) {
                    info!("vfs_test_provider: hang request — not responding (intentional)");
                    state.hang_pending.push((req.resp_port, req.req_id));
                    continue;
                }

                // Slow requests: sleep briefly before replying to simulate
                // a provider that takes time to answer.
                if is_slow_request(&req.op, &req.payload) {
                    info!("vfs_test_provider: slow request — sleeping {}ms", SLOW_DELAY_MS);
                    stem::time::sleep_ms(SLOW_DELAY_MS);
                }

                let resp = dispatch(&req.op, &req.payload);
                if let Err(e) = svc.send_response(&req, resp) {
                    warn!("vfs_test_provider: send_response failed: {:?}", e);
                }
            }

            Ok(ServiceProviderEvent::Message { kind, payload }) => {
                info!("vfs_test_provider: inbox message kind={:?} len={}", kind.0, payload.len());
                if payload.first() == Some(&b'q') {
                    info!("vfs_test_provider: shutdown requested via inbox");
                    break;
                }
            }

            Ok(ServiceProviderEvent::InboxClosed) => {
                info!(
                    "vfs_test_provider: inbox closed after {} requests — shutting down",
                    state.request_count
                );
                break;
            }

            Ok(ServiceProviderEvent::Timeout) => {}

            Ok(ServiceProviderEvent::Ready { token, .. }) => {
                info!("vfs_test_provider: unexpected secondary event token={:?}", token);
            }

            Err(e) => {
                warn!("vfs_test_provider: next_event error: {:?}", e);
                break;
            }
        }
    }

    svc.shutdown_sequence();
    info!("vfs_test_provider: clean shutdown");
    stem::syscall::exit(0);
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Extract a `u64` handle from the first 8 bytes of `payload`.
///
/// Returns 0 on under-length payloads (the kernel always supplies at least
/// 8 bytes for handle-bearing operations, so this is a safe default rather
/// than a real fallback).
#[inline]
fn extract_handle(payload: &[u8]) -> u64 {
    if payload.len() >= 8 {
        u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]))
    } else {
        0
    }
}

/// Returns `true` if this request targets the `/hang` file handle.
fn is_hang_request(op: &VfsRpcOp, payload: &[u8]) -> bool {
    match op {
        VfsRpcOp::Read | VfsRpcOp::Stat | VfsRpcOp::Close => {
            payload.len() >= 8 && extract_handle(payload) == HANDLE_HANG
        }
        _ => false,
    }
}

/// Returns `true` if this request targets the `/slow` file handle.
fn is_slow_request(op: &VfsRpcOp, payload: &[u8]) -> bool {
    match op {
        VfsRpcOp::Read | VfsRpcOp::Stat => {
            payload.len() >= 8 && extract_handle(payload) == HANDLE_SLOW
        }
        _ => false,
    }
}

/// Dispatch a VFS RPC operation and return the appropriate response.
fn dispatch(op: &VfsRpcOp, payload: &[u8]) -> ProviderResponse {
    match op {
        // ── Lookup ────────────────────────────────────────────────────────
        VfsRpcOp::Lookup => {
            // The Lookup payload is: [path_len: u32][path_bytes...].
            // Offset 4 skips the 4-byte little-endian path-length prefix.
            let path = if payload.len() > 4 {
                core::str::from_utf8(&payload[4..]).unwrap_or("")
            } else {
                ""
            };
            // Strip leading slash if present.
            let name = path.trim_start_matches('/').as_bytes();
            for &(entry_name, handle) in ENTRIES {
                if name == entry_name {
                    return ProviderResponse::ok_u64(handle);
                }
            }
            // Root directory lookup (empty path or "/").
            if name.is_empty() {
                return ProviderResponse::ok_u64(HANDLE_ROOT);
            }
            ProviderResponse::err(Errno::ENOENT)
        }

        // ── Stat ──────────────────────────────────────────────────────────
        VfsRpcOp::Stat => {
            if payload.len() < 8 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let handle = extract_handle(payload);
            match handle {
                HANDLE_ROOT => ProviderResponse::ok_stat(0o040_555, 0, HANDLE_ROOT),
                HANDLE_FAST => {
                    ProviderResponse::ok_stat(0o100_444, FAST_CONTENT.len() as u64, HANDLE_FAST)
                }
                HANDLE_SLOW => {
                    ProviderResponse::ok_stat(0o100_444, SLOW_CONTENT.len() as u64, HANDLE_SLOW)
                }
                HANDLE_HANG => ProviderResponse::ok_stat(0o100_444, 0, HANDLE_HANG),
                HANDLE_BURST => {
                    ProviderResponse::ok_stat(0o100_444, BURST_CONTENT.len() as u64, HANDLE_BURST)
                }
                HANDLE_MALFORMED => ProviderResponse::ok_stat(0o100_444, 0, HANDLE_MALFORMED),
                HANDLE_LARGE => {
                    ProviderResponse::ok_stat(0o100_444, LARGE_SIZE as u64, HANDLE_LARGE)
                }
                _ => ProviderResponse::err(Errno::EBADF),
            }
        }

        // ── Read ──────────────────────────────────────────────────────────
        VfsRpcOp::Read => {
            if payload.len() < 8 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let handle = extract_handle(payload);
            match handle {
                HANDLE_FAST => ProviderResponse::ok_read(FAST_CONTENT),
                HANDLE_SLOW => ProviderResponse::ok_read(SLOW_CONTENT),
                // HANG is handled before dispatch — should never reach here.
                HANDLE_HANG => ProviderResponse::err(Errno::EIO),
                HANDLE_BURST => ProviderResponse::ok_read(BURST_CONTENT),
                HANDLE_MALFORMED => {
                    // The four 0xff bytes are an intentionally invalid length
                    // prefix (claiming 0xFFFFFFFF = 4 GiB of data follows)
                    // while only a short string actually follows.  This makes
                    // the payload structurally malformed from any consumer that
                    // interprets the first four bytes as a byte count.
                    let garbage: &[u8] = b"\xff\xff\xff\xffMALFORMED_PAYLOAD";
                    ProviderResponse::ok_bytes(garbage)
                }
                HANDLE_LARGE => {
                    let mut data = alloc::vec![0u8; LARGE_SIZE];
                    for (i, b) in data.iter_mut().enumerate() {
                        *b = (i % 256) as u8;
                    }
                    ProviderResponse::ok_read(&data)
                }
                _ => ProviderResponse::err(Errno::EBADF),
            }
        }

        // ── Readdir ───────────────────────────────────────────────────────
        VfsRpcOp::Readdir => {
            let handle = if payload.len() >= 8 { extract_handle(payload) } else { 0 };
            if handle == HANDLE_ROOT {
                let mut out = Vec::new();
                for &(name, entry_handle) in ENTRIES {
                    out.extend_from_slice(&entry_handle.to_le_bytes());
                    out.push(name.len() as u8);
                    out.extend_from_slice(name);
                }
                ProviderResponse::ok_read(&out)
            } else {
                // Non-root handles are regular files; return empty listing.
                ProviderResponse::ok_read(&[])
            }
        }

        // ── Close ─────────────────────────────────────────────────────────
        VfsRpcOp::Close => ProviderResponse::ok_empty(),

        // ── Write ─────────────────────────────────────────────────────────
        VfsRpcOp::Write => ProviderResponse::err(Errno::EROFS),

        // ── Poll ──────────────────────────────────────────────────────────
        VfsRpcOp::Poll => ProviderResponse::ok_poll(1),

        // ── Everything else ───────────────────────────────────────────────
        _ => ProviderResponse::err(Errno::ENOSYS),
    }
}
