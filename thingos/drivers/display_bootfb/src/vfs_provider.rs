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

use abi::attrs::{
    ATTR_OP_GET, ATTR_OP_LIST, ATTR_OP_REMOVE, ATTR_OP_SET, AttrListEntryHeader, AttrNameHeader,
    AttrType, AttrValueHeader,
};
use abi::device::{DeviceCall, DeviceKind};
use abi::display::{
    BufferHandle, BufferId, CommitRequest, DISPLAY_OP_COMMIT, DISPLAY_OP_GET_INFO,
    DISPLAY_OP_IMPORT_BUFFER, DISPLAY_OP_MOVE_CURSOR, DISPLAY_OP_RELEASE_BUFFER,
    DISPLAY_OP_SET_CURSOR, PlaneCommit,
};
use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp;
use ipc_helpers::provider::{ProviderRequest, ProviderResponse};

use crate::driver::BootFbDriver;

// Handle IDs for this driver.
pub const HANDLE_ROOT: u64 = 0;
pub const HANDLE_CARD: u64 = 1;
const ATTR_DRIVER_NAME: &str = "display_bootfb";
const ATTR_DRIVER_CLASS: &str = "display";

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
        VfsRpcOp::AttrGet
        | VfsRpcOp::AttrSet
        | VfsRpcOp::AttrRemove
        | VfsRpcOp::AttrList
        | VfsRpcOp::Readlink => ProviderResponse::err(Errno::ENOTSUP),
        VfsRpcOp::Read | VfsRpcOp::Write | VfsRpcOp::Readdir | VfsRpcOp::Poll => {
            ProviderResponse::err(Errno::ENOSYS)
        }
        VfsRpcOp::ReadIntoFd => ProviderResponse::err(Errno::ENOSYS),
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
        "" => HANDLE_CARD,
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

    if call.kind != DeviceKind::Display {
        return ProviderResponse::err(Errno::ENOSYS);
    }

    match call.op {
        DISPLAY_OP_GET_INFO => {
            let info = driver.get_info();
            let out_bytes = unsafe {
                core::slice::from_raw_parts(
                    &info as *const _ as *const u8,
                    core::mem::size_of::<abi::display::DisplayInfo>(),
                )
            };
            ProviderResponse::ok_device_call(0, out_bytes)
        }
        DISPLAY_OP_IMPORT_BUFFER => {
            if call_payload.len() < core::mem::size_of::<BufferHandle>() {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let buffer_handle: BufferHandle =
                unsafe { core::ptr::read_unaligned(call_payload.as_ptr() as *const _) };
            match driver.import_buffer(&buffer_handle) {
                Ok(id) => ProviderResponse::ok_device_call(id.0, &id.0.to_le_bytes()),
                Err(e) => ProviderResponse::err(e),
            }
        }
        DISPLAY_OP_RELEASE_BUFFER => {
            if call_payload.len() < 4 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let id = BufferId(u32::from_le_bytes(call_payload[..4].try_into().unwrap()));
            match driver.release_buffer(id) {
                Ok(()) => ProviderResponse::ok_device_call(0, &[]),
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
            // Damage rects follow the plane array.
            let mut inline_planes = Vec::new();
            let mut inline_damage = Vec::new();
            let plane_size = core::mem::size_of::<PlaneCommit>();
            if req.commit_count > 0 {
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
            let damage_offset =
                header_size.saturating_add((req.commit_count as usize).saturating_mul(plane_size));
            if req.damage_count > 0 && call_payload.len() >= damage_offset {
                let rect_size = core::mem::size_of::<abi::display_protocol::Rect>();
                let count = req.damage_count as usize;
                let needed = damage_offset.saturating_add(count.saturating_mul(rect_size));
                if call_payload.len() >= needed {
                    let base = &call_payload[damage_offset..needed];
                    inline_damage.reserve(count);
                    for i in 0..count {
                        let off = i * rect_size;
                        let rect: abi::display_protocol::Rect = unsafe {
                            core::ptr::read_unaligned(
                                base[off..off + rect_size].as_ptr() as *const _
                            )
                        };
                        inline_damage.push(rect);
                    }
                }
            }

            let req_owned;
            let req_ref = if !inline_planes.is_empty() || !inline_damage.is_empty() {
                req_owned = CommitRequest {
                    commit_count: inline_planes.len() as u32,
                    flags: req.flags,
                    commits_ptr: inline_planes.as_ptr() as u64,
                    damage_count: inline_damage.len() as u32,
                    _reserved: 0,
                    damage_ptr: inline_damage.as_ptr() as u64,
                };
                &req_owned
            } else {
                &req
            };

            match driver.commit(req_ref) {
                Ok(()) => ProviderResponse::ok_device_call(0, &[]),
                Err(e) => ProviderResponse::err(e),
            }
        }
        DISPLAY_OP_SET_CURSOR | DISPLAY_OP_MOVE_CURSOR => {
            // display_bootfb does not advertise HARDWARE_CURSOR capability.
            // Return ENOSYS so callers fall back to the software cursor path.
            ProviderResponse::err(Errno::ENOSYS)
        }
        _ => ProviderResponse::err(Errno::ENOSYS),
    }
}

fn handle_attr_get(payload: &[u8]) -> ProviderResponse {
    if payload.len() < 8 + core::mem::size_of::<AttrNameHeader>() {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
    if handle != HANDLE_CARD {
        return ProviderResponse::err(Errno::EINVAL);
    }

    let header_start = 8;
    let name_len = u16::from_le_bytes([payload[header_start], payload[header_start + 1]]) as usize;
    if payload.len() < 8 + core::mem::size_of::<AttrNameHeader>() + name_len {
        return ProviderResponse::err(Errno::EINVAL);
    }

    let name = match core::str::from_utf8(
        &payload[8 + core::mem::size_of::<AttrNameHeader>()..][..name_len],
    ) {
        Ok(v) => v,
        Err(_) => return ProviderResponse::err(Errno::EINVAL),
    };

    let (value_type, value_bytes): (AttrType, &[u8]) = match name {
        "driver.name" => (AttrType::Utf8, ATTR_DRIVER_NAME.as_bytes()),
        "driver.class" => (AttrType::Utf8, ATTR_DRIVER_CLASS.as_bytes()),
        _ => return ProviderResponse::err(Errno::ENOENT),
    };

    let mut out = Vec::with_capacity(core::mem::size_of::<AttrValueHeader>() + value_bytes.len());
    out.push(value_type as u8);
    out.push(0);
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&(value_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(value_bytes);

    ProviderResponse::ok_bytes(&out)
}

fn handle_attr_list(payload: &[u8]) -> ProviderResponse {
    if payload.len() < 8 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
    if handle != HANDLE_CARD {
        return ProviderResponse::err(Errno::EINVAL);
    }

    let mut out = Vec::new();
    let entries = [
        ("driver.name", AttrType::Utf8, ATTR_DRIVER_NAME.len() as u32),
        ("driver.class", AttrType::Utf8, ATTR_DRIVER_CLASS.len() as u32),
    ];
    for (name, ty, value_len) in entries {
        let header = AttrListEntryHeader {
            name_len: name.len() as u16,
            value_type: ty as u8,
            flags: 0,
            value_len,
        };
        out.extend_from_slice(&header.name_len.to_le_bytes());
        out.push(header.value_type);
        out.push(0);
        out.extend_from_slice(&header.value_len.to_le_bytes());
        out.extend_from_slice(name.as_bytes());
    }
    ProviderResponse::ok_bytes(&out)
}
