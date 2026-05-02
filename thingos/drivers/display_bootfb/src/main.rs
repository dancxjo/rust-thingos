#![no_std]
#![no_main]
use alloc::string::{String, ToString};
use core::default::Default;
extern crate alloc;

mod driver;
mod vfs_provider;

use abi::driver_interface::{
    DRIVER_DESCRIPTOR_ABI_VERSION, DeviceInfo, DriverClass, DriverDescriptor, DriverEntryCtx,
    ProbeResult, Status,
};
use abi::vfs_rpc::VFS_RPC_MAX_REQ;
use driver::BootFbDriver;
use ipc_helpers::provider::ProviderLoop;
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind, device_kind_bytes};
use stem::syscall::message::{KindId, msg_inbox_open, msg_recv_blocking, msg_sendmsg};
use stem::syscall::port_create;
use stem::syscall::vfs::{vfs_handle_from_port, vfs_mount};
use stem::{debug, info, warn};
use vfs_provider::dispatch_vfs_rpc;
const THINGOS_DRIVER_NAME: &[u8] = b"display_bootfb";
const DRIVER_DEVPATH_ENV: &[u8] = b"THINGOS_DRIVER_DEVPATH";

#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    fn thingos_driver_start_safe(ctx: *const DriverEntryCtx) -> Status;
}

#[unsafe(no_mangle)]
#[used]
pub static THINGOS_DRIVER: DriverDescriptor = DriverDescriptor {
    abi_version: DRIVER_DESCRIPTOR_ABI_VERSION,
    driver_name_ptr: THINGOS_DRIVER_NAME.as_ptr(),
    driver_name_len: THINGOS_DRIVER_NAME.len(),
    driver_class: DriverClass::Display,
    flags: 0,
    probe: thingos_driver_probe,
    #[cfg(target_arch = "x86_64")]
    start: thingos_driver_start_safe,
    #[cfg(not(target_arch = "x86_64"))]
    start: thingos_driver_start,
};

unsafe extern "C" fn thingos_driver_probe(
    _dev: *const DeviceInfo,
    out: *mut ProbeResult,
) -> Status {
    if out.is_null() {
        return Status::InvalidArgument;
    }
    let out = &mut *out;
    out.matched = 0;
    out.score = 0;
    out.claimed_class = DriverClass::Display;
    out.flags = 0;
    Status::NoMatch
}

unsafe extern "C" fn thingos_driver_start(ctx: *const DriverEntryCtx) -> Status {
    main(ctx as usize)
}

#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(
    r#"
    .section .text
    .global thingos_driver_start_safe
    thingos_driver_start_safe:
        // RSP = 16n (kernel spawn); sub+push keeps 16-byte alignment before call.
        sub rsp, 8
        push rdi
        // Call std initialization (TLS, etc)
        call thingos_runtime_setup
        // pop undoes push; add undoes initial sub so next call enters with SysV alignment.
        pop rdi
        add rsp, 8
        // CALL will push 8 bytes, so inside Rust entry RSP = 16n + 8.
        call thingos_driver_start_rust
        ret
"#
);

#[unsafe(no_mangle)]
unsafe extern "C" fn thingos_driver_start_rust(ctx: *const DriverEntryCtx) -> Status {
    thingos_driver_start(ctx)
}

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: device_kind_bytes(b"dev.display.Framebuffer"),
    version: 1,
    _reserved: 0,
};

#[stem::main]
fn main(boot_fd: usize) -> ! {
    let self_tid = stem::syscall::get_tid().unwrap_or(0);
    let self_pid = stem::syscall::getpid();
    stem::info!(
        "display_bootfb: Starting VFS-native bootfb driver (v0.4.1) TID={} PID={}",
        self_tid,
        self_pid
    );
    stem::info!("display_bootfb: boot_arg={}", boot_fd);

    // 1. Map bootstrap memfd to get handles
    let mut drv_req_read = 0;
    let mut drv_resp_write = 0;
    let mut reserved_supervisor_port = 0;
    let mut bind_instance_id = 0u64;
    let mut cambium_direct_mount = false;

    let mut boot_fd = boot_fd;

    if boot_fd == 0 {
        let mut buf = [0u8; 1024];
        if let Ok(needed) = stem::syscall::argv_get(&mut buf) {
            if needed >= 4 {
                let count = u32::from_le_bytes(buf[0..4].try_into().unwrap());
                if count >= 2 {
                    let mut offset = 4;
                    // Skip argv[0]
                    let arg0_len =
                        u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap()) as usize;
                    offset += 4 + arg0_len;
                    // argv[1]
                    if offset + 4 <= buf.len() {
                        let arg1_len =
                            u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap())
                                as usize;
                        offset += 4;
                        if offset + arg1_len <= buf.len() {
                            if let Ok(s) = core::str::from_utf8(&buf[offset..offset + arg1_len]) {
                                if let Ok(val) = s.parse::<usize>() {
                                    boot_fd = val;
                                    debug!(
                                        "display_bootfb: Recovered boot_fd {} from argv[1]",
                                        boot_fd
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if boot_fd != 0 {
        let boot_size = 4096;
        let req = abi::vm::VmMapReq {
            addr_hint: 0,
            len: boot_size,
            prot: abi::vm::VmProt::READ | abi::vm::VmProt::USER,
            flags: abi::vm::VmMapFlags::empty(),
            backing: abi::vm::VmBacking::File { thing: boot_fd as u32, offset: 0 },
        };

        stem::info!("display_bootfb: Mapping bootstrap memfd {} size={}...", boot_fd, boot_size);
        match stem::syscall::vm_map(&req) {
            Ok(resp) => {
                stem::info!("display_bootfb: vm_map success at 0x{:x}", resp.addr);
                let entry_ctx = unsafe { &*(resp.addr as *const DriverEntryCtx) };
                if entry_ctx.version == 1 {
                    let device_path = entry_ctx.device_path_str();
                    cambium_direct_mount = true;
                    match port_create(VFS_RPC_MAX_REQ * 8) {
                        Ok((dummy_write, dummy_read)) => {
                            drv_req_read = dummy_read;
                            drv_resp_write = dummy_write;
                        }
                        Err(e) => {
                            stem::error!(
                                "display_bootfb: failed to create direct-mode protocol ports: {:?}",
                                e
                            );
                            stem::syscall::exit(1);
                        }
                    }
                    stem::info!(
                        "display_bootfb: recovered Cambium DriverEntryCtx device_path='{}'",
                        device_path
                    );
                } else {
                    let slice =
                        unsafe { core::slice::from_raw_parts(resp.addr as *const u32, 1024) };

                    drv_req_read = slice[0];
                    drv_resp_write = slice[1];
                    reserved_supervisor_port = slice[2];

                    let id_low = slice[3] as u64;
                    let id_high = slice[4] as u64;
                    bind_instance_id = id_low | (id_high << 32);

                    stem::info!(
                        "display_bootfb: Recovered handles: req_read={}, resp_write={}, svc={}, id={}",
                        drv_req_read,
                        drv_resp_write,
                        reserved_supervisor_port,
                        bind_instance_id
                    );
                }
            }
            Err(e) => {
                stem::info!(
                    "display_bootfb: ERROR: Failed to vm_map bootstrap memfd {}: {:?}",
                    boot_fd,
                    e
                );
            }
        }
    } else {
        stem::info!("display_bootfb: ERROR: No bootstrap memfd arg provided (boot_arg is 0)");
        stem::syscall::exit(1);
    }

    if drv_resp_write == 0 || (!cambium_direct_mount && bind_instance_id == 0) {
        stem::debug!(
            "display_bootfb: ERROR: Invalid/Missing bootstrap components (req={}, resp={}, reserved={}, id={})",
            drv_req_read,
            drv_resp_write,
            reserved_supervisor_port,
            bind_instance_id
        );
        stem::syscall::exit(1);
    }

    let mut driver = match BootFbDriver::new() {
        Some(d) => {
            debug!(
                "display_bootfb: Driver initialized successfully ({}x{})",
                d.fb.width, d.fb.height
            );
            debug!("display_bootfb: Mapping framebuffer (backing fd={})...", boot_fd);
            d
        }
        None => {
            stem::error!(
                "display_bootfb: ERROR: Failed to acquire hardware framebuffer (find_framebuffer returned None)"
            );
            stem::syscall::exit(1);
        }
    };

    // Create the VFS provider port pair.
    let (vfs_write, vfs_read) = match port_create(VFS_RPC_MAX_REQ * 8) {
        Ok(handles) => handles,
        Err(e) => {
            debug!("display_bootfb: ERROR: Failed to create provider port: {:?}", e);
            stem::syscall::exit(1);
        }
    };

    // Sovereign Handshake
    use abi::display_driver_protocol;
    use abi::supervisor_protocol::{self, classes};

    if cambium_direct_mount {
        let Some(dev_path) = assigned_dev_path() else {
            stem::error!("display_bootfb: Cambium did not assign a dev path");
            stem::syscall::exit(1);
        };
        match vfs_mount(vfs_write, &dev_path) {
            Ok(()) => {
                info!("display_bootfb: mounted VFS provider at {} via cambium", dev_path)
            }
            Err(e) => {
                stem::error!("display_bootfb: vfs_mount({}) failed: {:?}", dev_path, e);
                stem::syscall::exit(1);
            }
        }
    } else {
        let vfs_write_fd = match vfs_handle_from_port(vfs_write) {
            Ok(fd) => fd,
            Err(e) => {
                warn!("display_bootfb: failed to project provider write port to fd: {:?}", e);
                stem::syscall::exit(1);
            }
        };

        let sprout_pid = stem::syscall::getppid();
        let sprout_inbox_fd = match msg_inbox_open(sprout_pid) {
            Ok(fd) => fd,
            Err(e) => {
                warn!(
                    "display_bootfb: failed to open sprout inbox for pid {}: {:?}",
                    sprout_pid, e
                );
                stem::syscall::exit(1);
            }
        };

        let ready = supervisor_protocol::BindReadyPayload {
            bind_instance_id,
            class_mask: classes::DISPLAY_CARD | classes::FRAMEBUFFER,
            _reserved: 0,
        };
        let mut ready_bytes = [0u8; supervisor_protocol::BIND_READY_PAYLOAD_SIZE];
        if let Some(len) = supervisor_protocol::encode_bind_ready_le(&ready, &mut ready_bytes) {
            let mut buf = [0u8; 256];
            if let Some(total_len) = display_driver_protocol::encode_message(
                &mut buf,
                supervisor_protocol::MSG_BIND_READY,
                &ready_bytes[..len],
            ) {
                info!(
                    "display_bootfb: Sending MSG_BIND_READY handshake (class_mask=0x{:x}) to sprout inbox...",
                    ready.class_mask
                );
                // Bundle the VFS provider handle and the BIND_READY notification atomically.
                let res = msg_sendmsg(
                    sprout_inbox_fd,
                    KindId(display_driver_protocol::KIND_ID_DISPLAY_DRIVER_CONTROL),
                    &buf[..total_len],
                    &[vfs_write_fd],
                );
                info!(
                    "display_bootfb: Sent MSG_BIND_READY (result={:?}), waiting for MSG_BIND_ASSIGNED...",
                    res
                );
            }
        }

        // Wait for MSG_BIND_ASSIGNED or MSG_BIND_FAILED on our inbox.
        let mut bind_instance_id_confirmed = bind_instance_id;
        loop {
            let msg = msg_recv_blocking(512);
            if msg.kind.0 != display_driver_protocol::KIND_ID_DISPLAY_DRIVER_CONTROL {
                continue;
            }
            if let Some((header, payload)) = display_driver_protocol::parse_message(&msg.payload) {
                if header.msg_type == supervisor_protocol::MSG_BIND_ASSIGNED {
                    if let Some(assigned) = supervisor_protocol::decode_bind_assigned_le(payload) {
                        bind_instance_id_confirmed = assigned.bind_instance_id;
                        let path_len =
                            assigned.primary_path.iter().position(|&b| b == 0).unwrap_or(64);
                        let path =
                            core::str::from_utf8(&assigned.primary_path[..path_len]).unwrap_or("?");
                        debug!(
                            "display_bootfb: Sovereign registration COMPLETE. Assigned: {}",
                            path
                        );
                        break;
                    }
                } else if header.msg_type == supervisor_protocol::MSG_BIND_FAILED {
                    if let Some(failed) = supervisor_protocol::decode_bind_failed_le(payload) {
                        let reason_len = failed.reason.iter().position(|&b| b == 0).unwrap_or(64);
                        let reason =
                            core::str::from_utf8(&failed.reason[..reason_len]).unwrap_or("?");
                        warn!(
                            "display_bootfb: Registration REJECTED by supervisor (code={}, reason={}). Exiting.",
                            failed.error_code, reason
                        );
                        stem::syscall::exit(1);
                    }
                }
            }
        }

        // Notify supervisor that this service is now fully operational.
        {
            let svc_ready = supervisor_protocol::ServiceReadyPayload {
                bind_instance_id: bind_instance_id_confirmed,
                _reserved: 0,
            };
            let mut payload_bytes = [0u8; supervisor_protocol::SERVICE_READY_PAYLOAD_SIZE];
            let mut svc_buf = [0u8; 64];
            if let Some(p_len) =
                supervisor_protocol::encode_service_ready_le(&svc_ready, &mut payload_bytes)
            {
                if let Some(total_len) = display_driver_protocol::encode_message(
                    &mut svc_buf,
                    supervisor_protocol::MSG_SERVICE_READY,
                    &payload_bytes[..p_len],
                ) {
                    let _ = msg_sendmsg(
                        sprout_inbox_fd,
                        KindId(display_driver_protocol::KIND_ID_DISPLAY_DRIVER_CONTROL),
                        &svc_buf[..total_len],
                        &[],
                    );
                    debug!("display_bootfb: Sent MSG_SERVICE_READY.");
                }
            }
        }
    }

    // VFS provider service loop — ProviderLoop blocks on port_recv and
    // dispatches each decoded request to dispatch_vfs_rpc.
    info!("display_bootfb: entering VFS provider service loop");
    let mut lp = ProviderLoop::new(vfs_read);
    loop {
        let req = match lp.next_request() {
            Ok(r) => r,
            Err(_) => break,
        };
        let resp = dispatch_vfs_rpc(&mut driver, &req);
        match lp.send_response(&req, resp) {
            Ok(()) => stem::trace!(
                "display_bootfb: VFS RPC response sent op={:?} req_id={}",
                req.op,
                req.req_id
            ),
            Err(e) => stem::error!(
                "display_bootfb: VFS RPC response failed op={:?} req_id={} err={:?}",
                req.op,
                req.req_id,
                e
            ),
        }
    }

    info!("display_bootfb: VFS provider port closed — exiting");
    stem::syscall::exit(0);
}

fn assigned_dev_path() -> Option<String> {
    let mut buf = [0u8; 128];
    let len = stem::syscall::env_get(DRIVER_DEVPATH_ENV, &mut buf).ok()?;
    if len == 0 || len > buf.len() {
        return None;
    }
    let path = core::str::from_utf8(&buf[..len]).ok()?.trim();
    if path.is_empty() { None } else { Some(path.to_string()) }
}
