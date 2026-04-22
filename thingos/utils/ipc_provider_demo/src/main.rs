//! ipc_provider_demo — IPC Cookbook Recipe 4: minimal VFS provider.
//!
//! This program demonstrates how to expose a virtual filesystem tree under a
//! mount point using the Thing-OS VFS provider protocol.
//!
//! # What this shows
//!
//! 1. **Port creation** — create the provider port pair.
//! 2. **Mount** — register the port write-end with the kernel via
//!    `vfs_mount(provider_write_handle, path)`.
//! 3. **Provider loop** — use [`ProviderLoop`] to read typed kernel requests
//!    and dispatch them with [`ProviderResponse`] values.
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
//! # See Also
//!
//! `docs/concepts/ipc_cookbook.md` Recipe 4 — VFS provider implementation.
//! `libs/ipc_helpers/src/provider.rs` — ProviderLoop helper.
//! `userspace/iso9660d/` — full reference implementation.
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
use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
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
}

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("ipc_provider_demo: starting up");

    let mut state = ProviderState { request_count: 0, attrs: BTreeMap::new() };

    // ── 1. Create the provider port pair ──────────────────────────────
    let (write_h, read_h) = match port_create(65536) {
        Ok(pair) => pair,
        Err(e) => {
            warn!("ipc_provider_demo: port_create failed: {:?}", e);
            stem::syscall::exit(1);
        }
    };
    info!("ipc_provider_demo: port pair write_h={} read_h={}", write_h, read_h);

    // ── 2. Mount the provider at MOUNT_POINT ─────────────────────────────
    match vfs_mount(write_h, MOUNT_POINT) {
        Ok(()) => info!("ipc_provider_demo: mounted at {}", MOUNT_POINT),
        Err(e) => {
            warn!("ipc_provider_demo: vfs_mount failed: {:?} (continuing anyway for demo)", e);
        }
    }

    // ── 3. Provider loop ──────────────────────────────────────────────────
    let mut lp = ProviderLoop::new(read_h);

    loop {
        let req = match lp.next_request() {
            Ok(r) => r,
            Err(e) => {
                info!(
                    "ipc_provider_demo: port closed ({:?}) after {} requests — exiting",
                    e, state.request_count
                );
                break;
            }
        };

        state.request_count += 1;
        info!("ipc_provider_demo: request #{} op={:?}", state.request_count, req.op);

        let resp = dispatch(&mut state, &req.op, &req.payload);
        if let Err(e) = lp.send_response(req.resp_port, resp) {
            warn!("ipc_provider_demo: send_response failed: {:?}", e);
        }
    }

    info!("ipc_provider_demo: clean shutdown");
    stem::syscall::exit(0);
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
