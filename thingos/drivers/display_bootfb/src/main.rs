#![no_std]
#![no_main]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;

mod driver;
mod vfs_provider;

use abi::driver_interface::{
    DRIVER_DESCRIPTOR_ABI_VERSION, DeviceInfo, DriverClass, DriverDescriptor, DriverStartContext,
    ProbeResult, Status,
};
use abi::vfs_rpc::VFS_RPC_MAX_REQ;
use driver::BootFbDriver;
use ipc_helpers::provider::ProviderLoop;
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind, device_kind_bytes};
use stem::syscall::vfs::vfs_handle_from_port;
use stem::syscall::{port_create, port_recv};
use stem::{debug, info, warn};
use vfs_provider::dispatch_vfs_rpc;
const THINGOS_DRIVER_NAME: &[u8] = b"display_bootfb";

#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    fn thingos_driver_start_safe(ctx: *const DriverStartContext) -> Status;
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

unsafe extern "C" fn thingos_driver_start(boot_fd: usize) -> Status {
    main(boot_fd)
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
unsafe extern "C" fn thingos_driver_start_rust(boot_fd: usize) -> Status {
    thingos_driver_start(boot_fd)
}

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: device_kind_bytes(b"dev.display.Gpu"),
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
    let mut supervisor_port = 0;
    let mut bind_instance_id = 0u64;

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
                let slice = unsafe { core::slice::from_raw_parts(resp.addr as *const u32, 1024) };

                drv_req_read = slice[0];
                drv_resp_write = slice[1];
                supervisor_port = slice[2];

                let id_low = slice[3] as u64;
                let id_high = slice[4] as u64;
                bind_instance_id = id_low | (id_high << 32);

                stem::info!(
                    "display_bootfb: Recovered handles: req_read={}, resp_write={}, svc={}, id={}",
                    drv_req_read,
                    drv_resp_write,
                    supervisor_port,
                    bind_instance_id
                );
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

    if drv_req_read == 0 || drv_resp_write == 0 || supervisor_port == 0 || bind_instance_id == 0 {
        stem::debug!(
            "display_bootfb: ERROR: Invalid/Missing bootstrap components (req={}, resp={}, svc={}, id={})",
            drv_req_read,
            drv_resp_write,
            supervisor_port,
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

    // Bridge the response-port handle to a VFS FD once so we can use
    // sendmsg (FD-based) for capability transfer.
    let drv_resp_write_fd = vfs_handle_from_port(drv_resp_write)
        .expect("display_bootfb: vfs_handle_from_port(drv_resp_write)");

    // Sovereign Handshake
    use abi::display_driver_protocol;
    use abi::supervisor_protocol::{self, classes};

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
                "display_bootfb: Sending MSG_BIND_READY handshake (class_mask=0x{:x})...",
                ready.class_mask
            );
            // Bundle the VFS provider handle and the BIND_READY notification atomically.
            let res =
                stem::syscall::socket::sendmsg(drv_resp_write_fd, &buf[..total_len], &[vfs_write]);
            info!(
                "display_bootfb: Sent MSG_BIND_READY (result={:?}), waiting for MSG_BIND_ASSIGNED...",
                res
            );
        }
    }

    // Wait for MSG_BIND_ASSIGNED or MSG_BIND_FAILED
    let mut wait_buf = [0u8; 512];
    let mut bind_instance_id_confirmed = bind_instance_id;
    loop {
        // Read from drv_req_read, NOT supervisor_port. This should block rather than
        // spin so the CPU can schedule unrelated work while the driver waits.
        match port_recv(drv_req_read, &mut wait_buf) {
            Ok(n) => {
                if let Some((header, payload)) =
                    display_driver_protocol::parse_message(&wait_buf[..n])
                {
                    if header.msg_type == supervisor_protocol::MSG_BIND_ASSIGNED {
                        if let Some(assigned) =
                            supervisor_protocol::decode_bind_assigned_le(payload)
                        {
                            bind_instance_id_confirmed = assigned.bind_instance_id;
                            let path_len =
                                assigned.primary_path.iter().position(|&b| b == 0).unwrap_or(64);
                            let path = core::str::from_utf8(&assigned.primary_path[..path_len])
                                .unwrap_or("?");
                            debug!(
                                "display_bootfb: Sovereign registration COMPLETE. Assigned: {}",
                                path
                            );
                            break;
                        }
                    } else if header.msg_type == supervisor_protocol::MSG_BIND_FAILED {
                        if let Some(failed) = supervisor_protocol::decode_bind_failed_le(payload) {
                            let reason_len =
                                failed.reason.iter().position(|&b| b == 0).unwrap_or(64);
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
            Err(e) => {
                warn!("display_bootfb: failed waiting for bind assignment: {:?}", e);
                stem::time::sleep_ms(1);
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
                let _ =
                    stem::syscall::socket::sendmsg(drv_resp_write_fd, &svc_buf[..total_len], &[]);
                debug!("display_bootfb: Sent MSG_SERVICE_READY.");
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
        lp.send_response(req.resp_port, resp).ok();
    }

    info!("display_bootfb: VFS provider port closed — exiting");
    stem::syscall::exit(0);
}
