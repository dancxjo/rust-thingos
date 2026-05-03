//! VFS provider dispatch for the boot framebuffer driver.
//!
//! This module uses [`ipc_helpers::provider`] to decode incoming VFS RPC
//! requests and return typed [`ProviderResponse`] values.  The caller is
//! responsible for creating a [`ProviderLoop`] and forwarding its decoded
//! requests here.
extern crate alloc;

use alloc::vec::Vec;

use abi::attrs::{AttrListEntryHeader, AttrNameHeader, AttrType, AttrValueHeader};
use abi::device::{DeviceCall, DeviceKind};
use abi::display::accel2d::{ACCEL2D_COMMAND_SIZE, Accel2dBatch, Accel2dCommand};
use abi::display::{
    BufferHandle, BufferId, CommitRequest, DISPLAY_OP_ACCEL2D, DISPLAY_OP_COMMIT,
    DISPLAY_OP_GET_INFO, DISPLAY_OP_IMPORT_BUFFER, DISPLAY_OP_MOVE_CURSOR,
    DISPLAY_OP_RELEASE_BUFFER, DISPLAY_OP_SET_CURSOR, PlaneCommit,
};
use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp;
use ipc_helpers::provider::{ProviderRequest, ProviderResponse};

use crate::driver::BootFbDriver;

// Handle IDs for this driver.
pub const HANDLE_ROOT: u64 = 0;
pub const HANDLE_CARD: u64 = 1;
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
        VfsRpcOp::DeviceCall => device_call(driver, req),
        VfsRpcOp::SubscribeReady | VfsRpcOp::UnsubscribeReady => ProviderResponse::ok_empty(),
        VfsRpcOp::Rename => ProviderResponse::err(Errno::ENOSYS),
        VfsRpcOp::AttrGet => handle_attr_get(driver, &req.payload),
        VfsRpcOp::AttrList => handle_attr_list(driver, &req.payload),
        VfsRpcOp::AttrSet | VfsRpcOp::AttrRemove | VfsRpcOp::Readlink => {
            ProviderResponse::err(Errno::ENOTSUP)
        }
        VfsRpcOp::Read | VfsRpcOp::Write | VfsRpcOp::Readdir | VfsRpcOp::Poll => {
            ProviderResponse::err(Errno::ENOSYS)
        }
        VfsRpcOp::ReadIntoFd => ProviderResponse::err(Errno::ENOSYS),
    }
}

/// Map a raw display op code to a human-readable name for log messages.
fn display_op_name(op: u32) -> &'static str {
    match op {
        DISPLAY_OP_GET_INFO => "GET_INFO",
        DISPLAY_OP_IMPORT_BUFFER => "IMPORT_BUFFER",
        DISPLAY_OP_RELEASE_BUFFER => "RELEASE_BUFFER",
        DISPLAY_OP_COMMIT => "COMMIT",
        DISPLAY_OP_SET_CURSOR => "SET_CURSOR",
        DISPLAY_OP_MOVE_CURSOR => "MOVE_CURSOR",
        DISPLAY_OP_ACCEL2D => "ACCEL2D",
        _ => "UNKNOWN",
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

fn device_call(driver: &mut BootFbDriver, req: &ProviderRequest) -> ProviderResponse {
    let payload = &req.payload;
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

    // Entry instrumentation: record correlation ID, timestamp, and provider thread.
    driver.rpc_seq = driver.rpc_seq.saturating_add(1);
    let corr = driver.rpc_seq;
    let enter_ns = stem::time::monotonic_ns();
    driver.rpc_enter_ns = enter_ns;
    let provider_tid = stem::syscall::get_tid().unwrap_or(0);
    let op_name = display_op_name(call.op);
    stem::trace!(
        "display_bootfb: rpc.enter corr={} op={} req_id={} resp_port={} provider_tid={}",
        corr,
        op_name,
        req.req_id,
        req.resp_port,
        provider_tid,
    );

    let response = match call.op {
        DISPLAY_OP_GET_INFO => {
            stem::debug!("display.phase=device_call_enter op=GET_INFO");
            let info = driver.get_info();
            let out_bytes = unsafe {
                core::slice::from_raw_parts(
                    &info as *const _ as *const u8,
                    core::mem::size_of::<abi::display::DisplayInfo>(),
                )
            };
            stem::debug!("display.phase=device_call_exit op=GET_INFO result=ok");
            ProviderResponse::ok_device_call(0, out_bytes)
        }
        DISPLAY_OP_IMPORT_BUFFER => {
            if call_payload.len() < core::mem::size_of::<BufferHandle>() {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let buffer_handle: BufferHandle =
                unsafe { core::ptr::read_unaligned(call_payload.as_ptr() as *const _) };
            stem::debug!(
                "display.phase=import_buffer_begin memfd={} size={}x{}",
                buffer_handle.handle,
                buffer_handle.width,
                buffer_handle.height
            );
            match driver.import_buffer(&buffer_handle) {
                Ok(id) => {
                    stem::debug!("display.phase=import_buffer_done id={}", id.0);
                    ProviderResponse::ok_device_call(id.0, &id.0.to_le_bytes())
                }
                Err(e) => {
                    stem::debug!("display.phase=device_call_exit op=IMPORT_BUFFER result=err");
                    ProviderResponse::err(e)
                }
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

            stem::debug!("display.phase=commit_begin planes={}", req.commit_count);

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
                Ok(()) => {
                    stem::debug!("display.phase=commit_done");
                    ProviderResponse::ok_device_call(0, &[])
                }
                Err(e) => ProviderResponse::err(e),
            }
        }
        DISPLAY_OP_SET_CURSOR | DISPLAY_OP_MOVE_CURSOR => {
            // display_bootfb does not advertise HARDWARE_CURSOR capability.
            // Return ENOSYS so callers fall back to the software cursor path.
            ProviderResponse::err(Errno::ENOSYS)
        }
        DISPLAY_OP_ACCEL2D => {
            let batch_size = core::mem::size_of::<Accel2dBatch>();
            if call_payload.len() < batch_size {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let header: Accel2dBatch =
                unsafe { core::ptr::read_unaligned(call_payload.as_ptr() as *const _) };
            let cmd_count = header.cmd_count as usize;
            // Reject unreasonably large batches before arithmetic to prevent
            // malformed requests from exhausting memory.
            if cmd_count > 4096 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let needed = batch_size.saturating_add(cmd_count.saturating_mul(ACCEL2D_COMMAND_SIZE));
            if call_payload.len() < needed {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let mut commands = alloc::vec::Vec::with_capacity(cmd_count);
            for i in 0..cmd_count {
                let off = batch_size + i * ACCEL2D_COMMAND_SIZE;
                let cmd: Accel2dCommand = unsafe {
                    core::ptr::read_unaligned(
                        call_payload[off..off + ACCEL2D_COMMAND_SIZE].as_ptr() as *const _,
                    )
                };
                commands.push(cmd);
            }
            match driver.execute_accel2d(&header, &commands) {
                Ok(()) => ProviderResponse::ok_device_call(0, &[]),
                Err(e) => ProviderResponse::err(e),
            }
        }
        _ => ProviderResponse::err(Errno::ENOSYS),
    };

    // Exit instrumentation: log duration and fire per-RPC watchdog.
    let exit_ns = stem::time::monotonic_ns();
    let duration_ms = exit_ns.saturating_sub(enter_ns) / 1_000_000;
    stem::trace!(
        "display_bootfb: rpc.exit corr={} op={} req_id={} status={} duration_ms={}",
        corr,
        op_name,
        req.req_id,
        response.status,
        duration_ms,
    );
    // Per-RPC watchdog: warn on high-latency or stalled operations.
    if duration_ms >= 1000 {
        stem::warn!(
            "display_bootfb: rpc.stall corr={} op={} duration_ms={} status={} provider_tid={} req_id={} resp_port={}",
            corr,
            op_name,
            duration_ms,
            response.status,
            provider_tid,
            req.req_id,
            req.resp_port,
        );
    } else if duration_ms >= 200 {
        stem::warn!(
            "display_bootfb: rpc.slow corr={} op={} duration_ms={} status={} provider_tid={} req_id={} resp_port={}",
            corr,
            op_name,
            duration_ms,
            response.status,
            provider_tid,
            req.req_id,
            req.resp_port,
        );
    } else if duration_ms >= 50 {
        stem::debug!(
            "display RPC latency: corr={} op={} duration_ms={} status={} provider_tid={} req_id={} resp_port={}.",
            corr,
            op_name,
            duration_ms,
            response.status,
            provider_tid,
            req.req_id,
            req.resp_port,
        );
    }
    response
}

fn handle_attr_get(driver: &BootFbDriver, payload: &[u8]) -> ProviderResponse {
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
        "driver.name" => (AttrType::Utf8, driver.driver_name.as_bytes()),
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

fn handle_attr_list(driver: &BootFbDriver, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 8 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
    if handle != HANDLE_CARD {
        return ProviderResponse::err(Errno::EINVAL);
    }

    let mut out = Vec::new();
    let entries = [
        ("driver.name", AttrType::Utf8, driver.driver_name.len() as u32),
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
