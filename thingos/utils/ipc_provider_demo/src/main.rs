//! ipc_provider_demo — inbox-aware VFS provider using `ServiceProviderLoop`.
//!
//! This program demonstrates how to expose a virtual filesystem tree under a
//! mount point using the Thing-OS VFS provider protocol **with inbox-backed
//! actor semantics**.  A provider running under `ServiceProviderLoop` can
//! receive typed control messages through its inbox (lifecycle, config reload,
//! graceful shutdown) while simultaneously serving VFS RPC requests from the
//! kernel.
//!
//! # What this shows
//!
//! 1. **Port creation** — create the provider port pair.
//! 2. **Mount** — register the port write-end with the kernel via
//!    `vfs_mount(provider_write_handle, path)`.
//! 3. **`ServiceProviderLoop`** — unified event loop that handles both:
//!    - Inbox control messages (control-plane priority).
//!    - VFS RPC requests from the kernel (data-plane).
//!
//! # Virtual filesystem exposed
//!
//! Mount point: `/run/cookbook`
//!
//! ```text
//! /run/cookbook/
//!   hello.txt   — contains "Hello from the VFS provider!\n"
//! ```
//!
//! # Running
//!
//! ```text
//! # In the Thing-OS shell:
//! $ ipc_provider_demo &
//! $ cat /run/cookbook/hello.txt
//! Hello from the VFS provider!
//! ```
//!
//! # App developer's view
//!
//! Using `ServiceProviderLoop` the event loop looks like:
//!
//! ```ignore
//! loop {
//!     match svc.next_event(None).unwrap() {
//!         ServiceProviderEvent::ProviderRequest(req) => {
//!             // Handle VFS RPC (Lookup, Read, Stat, …) exactly as before.
//!             let resp = dispatch(&mut state, &req.op, &req.payload);
//!             svc.send_response(&req, resp).unwrap();
//!         }
//!         ServiceProviderEvent::Message { kind, payload } => {
//!             // Handle inbox control message (shutdown, reload, …).
//!             handle_control_message(kind, payload, &mut state);
//!         }
//!         ServiceProviderEvent::InboxClosed => break,   // clean shutdown
//!         ServiceProviderEvent::Timeout => {}           // periodic work here
//!         ServiceProviderEvent::Ready { .. } => {}      // extra FDs / IRQs
//!     }
//! }
//! ```
//!
//! # See Also
//!
//! `docs/concepts/ipc_cookbook.md` Recipe 4 — VFS provider implementation.
//! `libs/ipc_helpers/src/provider.rs` — ProviderLoop helper.
//! `libs/ipc_helpers/src/service_provider.rs` — ServiceProviderLoop.
#![no_std]
#![no_main]
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::default::Default;
extern crate alloc;

use abi::attrs::{AttrListEntryHeader, AttrSetHeader, AttrType};
use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp;
use abi::wire::KindId;
use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
use ipc_helpers::service_provider::{ServiceProviderEvent, ServiceProviderLoop};
use stem::syscall::{port_create, vfs_mount};
use stem::{info, warn};

/// Mount point for this provider.
const MOUNT_POINT: &str = "/run/cookbook";

/// The single file this provider exposes.
const FILE_NAME: &[u8] = b"hello.txt";

/// Synthetic handle value for hello.txt (any non-zero u64 is fine).
const HELLO_HANDLE: u64 = 1;
const ATTR_PROVIDER_NAME: &str = "ipc_provider_demo";

/// Content served by READ.
const HELLO_CONTENT: &[u8] = b"Hello from the VFS provider!\n";

struct AttrValue {
    ty: AttrType,
    data: Vec<u8>,
}

struct ProviderState {
    request_count: u64,
    attrs: BTreeMap<String, AttrValue>,
    /// Set to `true` when a `shutdown` control message is received.
    shutdown_requested: bool,
}

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("ipc_provider_demo: starting up");

    let mut state =
        ProviderState { request_count: 0, attrs: BTreeMap::new(), shutdown_requested: false };

    // ── 1. Create the provider port pair ──────────────────────────────────
    let (write_h, read_h) = match port_create(65536) {
        Ok(pair) => pair,
        Err(e) => {
            warn!("ipc_provider_demo: port_create failed: {:?}", e);
            stem::syscall::exit(1);
        }
    };

    // ── 2. Mount the provider at MOUNT_POINT ─────────────────────────────
    match vfs_mount(write_h, MOUNT_POINT) {
        Ok(()) => info!(
            "ipc_provider_demo: mounted at {} (write_h={} read_h={})",
            MOUNT_POINT, write_h, read_h
        ),
        Err(e) => {
            warn!("ipc_provider_demo: vfs_mount failed: {:?} (continuing anyway for demo)", e);
        }
    }

    // ── 3. Inbox-aware provider loop ──────────────────────────────────────
    //
    // `ServiceProviderLoop` combines:
    //   • the VFS RPC port (provider side)  → ServiceProviderEvent::ProviderRequest
    //   • the task inbox (control plane)    → ServiceProviderEvent::Message
    //
    // Inbox messages are dispatched *before* VFS RPC when both are ready at
    // the same wake, preserving the inbox-first control-priority contract.
    let mut svc = match ServiceProviderLoop::new(ProviderLoop::new(read_h), 4096) {
        Ok(s) => s,
        Err(e) => {
            warn!("ipc_provider_demo: ServiceProviderLoop::new failed: {:?}", e);
            stem::syscall::exit(1);
        }
    };

    loop {
        match svc.next_event(None) {
            Ok(ServiceProviderEvent::ProviderRequest(req)) => {
                state.request_count += 1;
                info!("ipc_provider_demo: request #{} op={:?}", state.request_count, req.op);

                let resp = dispatch(&mut state, &req.op, &req.payload);
                if let Err(e) = svc.send_response(&req, resp) {
                    warn!("ipc_provider_demo: send_response failed: {:?}", e);
                }

                if state.shutdown_requested {
                    info!("ipc_provider_demo: shutdown after pending request — exiting");
                    break;
                }
            }

            Ok(ServiceProviderEvent::Message { kind, payload }) => {
                // Inbox control message — handle lifecycle, config reload, etc.
                handle_control_message(kind, payload, &mut state);
                if state.shutdown_requested {
                    info!("ipc_provider_demo: shutdown requested via inbox — exiting");
                    break;
                }
            }

            Ok(ServiceProviderEvent::InboxClosed) => {
                info!(
                    "ipc_provider_demo: inbox closed after {} requests — exiting",
                    state.request_count
                );
                break;
            }

            Ok(ServiceProviderEvent::Timeout) => {
                // No work — could do periodic housekeeping here.
            }

            Ok(ServiceProviderEvent::Ready { token, .. }) => {
                // A secondary FD or IRQ fired.  This demo registers no extra
                // sources so this arm is unreachable in practice.
                info!("ipc_provider_demo: unexpected secondary event token={:?}", token);
            }

            Err(e) => {
                warn!("ipc_provider_demo: next_event error: {:?}", e);
                break;
            }
        }
    }

    info!("ipc_provider_demo: clean shutdown");
    stem::syscall::exit(0);
}

/// Handle a typed inbox control message.
///
/// This is the control-plane entry point.  In a real driver you would match
/// on `kind` to distinguish message types (shutdown, reload-config, …).
fn handle_control_message(kind: KindId, payload: &[u8], state: &mut ProviderState) {
    info!("ipc_provider_demo: inbox message kind={:?} payload_len={}", kind.0, payload.len());

    // As a simple demo, treat any inbox message whose first byte is b'q' as a
    // "please shut down gracefully" signal.
    if payload.first() == Some(&b'q') {
        info!("ipc_provider_demo: graceful shutdown requested via inbox");
        state.shutdown_requested = true;
    }
}

/// Dispatch a single VFS RPC operation and return the appropriate response.
fn dispatch(state: &mut ProviderState, op: &VfsRpcOp, payload: &[u8]) -> ProviderResponse {
    match op {
        VfsRpcOp::Lookup => {
            let path = if payload.len() > 4 {
                core::str::from_utf8(&payload[4..]).unwrap_or("")
            } else {
                ""
            };
            if path.as_bytes() == FILE_NAME {
                ProviderResponse::ok_u64(HELLO_HANDLE)
            } else {
                ProviderResponse::err(Errno::ENOENT)
            }
        }

        VfsRpcOp::Read => {
            if payload.len() >= 8 {
                let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
                if handle == HELLO_HANDLE {
                    return ProviderResponse::ok_read(HELLO_CONTENT);
                }
            }
            ProviderResponse::err(Errno::EBADF)
        }

        VfsRpcOp::Stat => {
            if payload.len() >= 8 {
                let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
                if handle == HELLO_HANDLE {
                    return ProviderResponse::ok_stat(
                        0o100_444,
                        HELLO_CONTENT.len() as u64,
                        HELLO_HANDLE,
                    );
                }
            }
            ProviderResponse::err(Errno::EBADF)
        }

        VfsRpcOp::Readdir => {
            let handle = if payload.len() >= 8 {
                u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]))
            } else {
                0
            };
            if handle == 0 {
                let mut entry = Vec::with_capacity(8 + 1 + FILE_NAME.len());
                entry.extend_from_slice(&HELLO_HANDLE.to_le_bytes());
                entry.push(FILE_NAME.len() as u8);
                entry.extend_from_slice(FILE_NAME);
                ProviderResponse::ok_read(&entry)
            } else {
                ProviderResponse::ok_read(&[])
            }
        }

        VfsRpcOp::Close => ProviderResponse::ok_empty(),
        VfsRpcOp::Write => ProviderResponse::err(Errno::EROFS),
        VfsRpcOp::Poll => ProviderResponse::ok_poll(1),
        VfsRpcOp::AttrGet => {
            if payload.len() < 10 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let name_len = u16::from_le_bytes([payload[8], payload[9]]) as usize;
            if payload.len() < 10 + name_len {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let name = match core::str::from_utf8(&payload[10..10 + name_len]) {
                Ok(v) => v,
                Err(_) => return ProviderResponse::err(Errno::EINVAL),
            };

            // 1. Check fixed/read-only attributes
            let (ty, data): (AttrType, Vec<u8>) = match name {
                "provider.is_demo" => (AttrType::Bool, Vec::from([1u8])),
                "provider.name" => (AttrType::Utf8, ATTR_PROVIDER_NAME.as_bytes().to_vec()),
                "provider.requests" => {
                    (AttrType::U64, Vec::from(state.request_count.to_le_bytes()))
                }
                _ => {
                    // 2. Check mutable attributes
                    if let Some(val) = state.attrs.get(name) {
                        (val.ty, val.data.clone())
                    } else {
                        return ProviderResponse::err(Errno::ENOENT);
                    }
                }
            };
            ProviderResponse::ok_attr_get(ty as u8, &data)
        }

        VfsRpcOp::AttrSet => {
            if payload.len() < 16 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let hdr: AttrSetHeader =
                unsafe { core::ptr::read_unaligned(payload[8..16].as_ptr() as *const _) };
            let name_len = hdr.name_len as usize;
            let val_len = hdr.value_len as usize;
            if payload.len() < 16 + name_len + val_len {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let name = match core::str::from_utf8(&payload[16..16 + name_len]) {
                Ok(v) => v,
                Err(_) => return ProviderResponse::err(Errno::EINVAL),
            };
            let data = payload[16 + name_len..16 + name_len + val_len].to_vec();
            let ty = match AttrType::from_u8(hdr.value_type) {
                Some(t) => t,
                None => return ProviderResponse::err(Errno::EINVAL),
            };

            info!("ipc_provider_demo: set attr '{}' type={:?} len={}", name, ty, val_len);
            state.attrs.insert(name.to_string(), AttrValue { ty, data });
            ProviderResponse::ok_empty()
        }

        VfsRpcOp::AttrRemove => {
            if payload.len() < 10 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let name_len = u16::from_le_bytes([payload[8], payload[9]]) as usize;
            if payload.len() < 10 + name_len {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let name = match core::str::from_utf8(&payload[10..10 + name_len]) {
                Ok(v) => v,
                Err(_) => return ProviderResponse::err(Errno::EINVAL),
            };

            if state.attrs.remove(name).is_some() {
                info!("ipc_provider_demo: removed attr '{}'", name);
                ProviderResponse::ok_empty()
            } else {
                ProviderResponse::err(Errno::ENOENT)
            }
        }

        VfsRpcOp::AttrList => {
            let mut out = Vec::new();
            // 1. List read-only attributes
            let ro_entries = [
                ("provider.is_demo", AttrType::Bool, 1u32),
                ("provider.name", AttrType::Utf8, ATTR_PROVIDER_NAME.len() as u32),
                ("provider.requests", AttrType::U64, 8u32),
            ];
            for (name, ty, val_len) in ro_entries {
                let hdr = AttrListEntryHeader {
                    name_len: name.len() as u16,
                    value_type: ty as u8,
                    flags: 0,
                    value_len: val_len,
                };
                unsafe {
                    let ptr = &hdr as *const _ as *const u8;
                    out.extend_from_slice(core::slice::from_raw_parts(ptr, 8));
                }
                out.extend_from_slice(name.as_bytes());
            }
            // 2. List mutable attributes
            for (name, val) in &state.attrs {
                let hdr = AttrListEntryHeader {
                    name_len: name.len() as u16,
                    value_type: val.ty as u8,
                    flags: 0,
                    value_len: val.data.len() as u32,
                };
                unsafe {
                    let ptr = &hdr as *const _ as *const u8;
                    out.extend_from_slice(core::slice::from_raw_parts(ptr, 8));
                }
                out.extend_from_slice(name.as_bytes());
            }
            ProviderResponse::ok_bytes(&out)
        }
        VfsRpcOp::DeviceCall => ProviderResponse::err(Errno::ENOSYS),
        _ => ProviderResponse::err(Errno::ENOSYS),
    }
}
