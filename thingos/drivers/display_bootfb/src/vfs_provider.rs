//! VFS provider dispatch for the boot framebuffer driver.
//!
//! This module uses [`ipc_helpers::provider`] to decode incoming VFS RPC
//! requests and return typed [`ProviderResponse`] values.  The caller is
//! responsible for creating a [`ProviderLoop`] and forwarding its decoded
//! requests here.
#![no_std]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;

use alloc::vec::Vec;

use abi::device::DeviceCall;
use abi::display::{
    BufferHandle, BufferId, CommitRequest, DISPLAY_OP_COMMIT, DISPLAY_OP_GET_INFO,
    DISPLAY_OP_IMPORT_BUFFER, DISPLAY_OP_RELEASE_BUFFER, PlaneCommit,
};
use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp;
use ipc_helpers::provider::{ProviderRequest, ProviderResponse};

use crate::driver::BootFbDriver;

// Handle IDs for this driver.
pub const HANDLE_ROOT: u64 = 0;
pub const HANDLE_CARD: u64 = 1;

const S_IFDIR: u32 = 0o040000;
const S_IFCHR: u32 = 0o020000;

/// Dispatch one decoded VFS RPC request and return the appropriate response.
///
/// Pass the returned [`ProviderResponse`] to
/// [`ProviderLoop::send_response`][ipc_helpers::provider::ProviderLoop::send_response].
pub fn dispatch_vfs_rpc(driver: &mut BootFbDriver, req: &ProviderRequest) -> ProviderResponse {
    match req.op {
        VfsRpcOp::Lookup => lookup(&req.payload),
        VfsRpcOp::Stat => stat(&req.payload),
        VfsRpcOp::Close => ProviderResponse::ok_empty(),
        VfsRpcOp::DeviceCall => device_call(driver, &req.payload),
        VfsRpcOp::SubscribeReady | VfsRpcOp::UnsubscribeReady => ProviderResponse::ok_empty(),
        VfsRpcOp::Rename => ProviderResponse::err(Errno::ENOSYS),
        _ => ProviderResponse::err(Errno::ENOSYS),
    }
}

fn lookup(payload: &[u8]) -> ProviderResponse {
    if payload.len() < 4 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let path_len = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]) as usize;
    if payload.len() < 4 + path_len {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let path = match core::str::from_utf8(&payload[4..4 + path_len]) {
        Ok(s) => s,
        Err(_) => return ProviderResponse::err(Errno::EINVAL),
    };
    let path = path.trim_matches('/');

    let handle: u64 = match path {
        "" => HANDLE_ROOT,
        "card0" => HANDLE_CARD,
        _ => return ProviderResponse::err(Errno::ENOENT),
    };

    ProviderResponse::ok_u64(handle)
}

fn stat(payload: &[u8]) -> ProviderResponse {
    if payload.len() < 8 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[..8].try_into().unwrap());

    let (mode, size): (u32, u64) = match handle {
        HANDLE_ROOT => (S_IFDIR | 0o755, 0),
        HANDLE_CARD => (S_IFCHR | 0o666, 0),
        _ => return ProviderResponse::err(Errno::ENOENT),
    };

    ProviderResponse::ok_stat(mode, size, handle)
}

fn device_call(driver: &mut BootFbDriver, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 8 + core::mem::size_of::<DeviceCall>() {
        return ProviderResponse::err(Errno::EINVAL);
    }

    let handle = u64::from_le_bytes(payload[..8].try_into().unwrap());
    if handle != HANDLE_CARD {
        return ProviderResponse::err(Errno::EINVAL);
    }

    let call: DeviceCall = unsafe {
        core::ptr::read_unaligned(
            payload[8..8 + core::mem::size_of::<DeviceCall>()].as_ptr() as *const _
        )
    };

    let call_payload = &payload[8 + core::mem::size_of::<DeviceCall>()..];

    match call.op {
        DISPLAY_OP_GET_INFO => {
            let info = driver.get_info();
            let out_bytes = unsafe {
                core::slice::from_raw_parts(
                    &info as *const _ as *const u8,
                    core::mem::size_of::<abi::display::DisplayInfo>(),
                )
            };
            ok_device_call(0, out_bytes)
        }
        DISPLAY_OP_IMPORT_BUFFER => {
            if call_payload.len() < core::mem::size_of::<BufferHandle>() {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let buffer_handle: BufferHandle =
                unsafe { core::ptr::read_unaligned(call_payload.as_ptr() as *const _) };
            match driver.import_buffer(&buffer_handle) {
                Ok(id) => ok_device_call(id.0, &[]),
                Err(e) => ProviderResponse::err(e),
            }
        }
        DISPLAY_OP_RELEASE_BUFFER => {
            if call_payload.len() < 4 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let id = BufferId(u32::from_le_bytes(call_payload[..4].try_into().unwrap()));
            match driver.release_buffer(id) {
                Ok(()) => ok_device_call(0, &[]),
                Err(e) => ProviderResponse::err(e),
            }
        }
        DISPLAY_OP_COMMIT => {
            let header_size = core::mem::size_of::<CommitRequest>();
            if call_payload.len() < header_size {
                return ProviderResponse::err(Errno::EINVAL);
            }

            let req: CommitRequest =
                unsafe { core::ptr::read_unaligned(call_payload.as_ptr() as *const _) };

            // For provider RPC calls, plane commits are serialized inline after
            // CommitRequest because raw pointers are not valid cross-process.
            let mut inline_planes = Vec::new();
            if req.commit_count > 0 {
                let plane_size = core::mem::size_of::<PlaneCommit>();
                let count = req.commit_count as usize;
                let needed = header_size.saturating_add(count.saturating_mul(plane_size));
                if call_payload.len() >= needed {
                    let base = &call_payload[header_size..needed];
                    inline_planes.reserve(count);
                    for i in 0..count {
                        let off = i * plane_size;
                        let plane: PlaneCommit = unsafe {
                            core::ptr::read_unaligned(
                                base[off..off + plane_size].as_ptr() as *const _
                            )
                        };
                        inline_planes.push(plane);
                    }
                }
            }

            let req_owned;
            let req_ref = if !inline_planes.is_empty() {
                req_owned = CommitRequest {
                    commit_count: inline_planes.len() as u32,
                    flags: req.flags,
                    commits_ptr: inline_planes.as_ptr() as u64,
                };
                &req_owned
            } else {
                &req
            };

            match driver.commit(req_ref) {
                Ok(()) => ok_device_call(0, &[]),
                Err(e) => ProviderResponse::err(e),
            }
        }
        _ => ProviderResponse::err(Errno::ENOSYS),
    }
}

/// Build a DeviceCall OK response payload: `[ret_val: u32][out_data_len: u32][out_data...]`.
fn ok_device_call(ret_val: u32, out_data: &[u8]) -> ProviderResponse {
    let mut payload = Vec::with_capacity(8 + out_data.len());
    payload.extend_from_slice(&ret_val.to_le_bytes());
    payload.extend_from_slice(&(out_data.len() as u32).to_le_bytes());
    payload.extend_from_slice(out_data);
    ProviderResponse::ok_bytes(&payload)
}
