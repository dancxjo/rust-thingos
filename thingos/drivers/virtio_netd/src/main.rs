//! VirtIO-NET userspace driver
//!
//! This service owns the VirtIO-NET device hardware and exposes it as a VFS
//! provider mounted at `/dev/net/virtio0/`.
//!
//! Architecture:
//! - virtio_netd: Hardware driver (RX/TX queues, DMA buffers, interrupts)
//!   Exposes files: ctl, status, mac, mtu, rx, tx, features, events
//! - netd / other consumers: talk to the driver purely through file paths
#![no_std]
#![no_main]
extern crate alloc;
use alloc::string::{String, ToString};

mod driver;
mod vfs_provider;

use abi::driver_interface::{
    BusKind, DRIVER_DESCRIPTOR_ABI_VERSION, DRIVER_FLAG_PCI, DRIVER_INTERFACE_ABI_VERSION,
    DeviceInfo, DriverClass, DriverDescriptor, DriverEntryCtx, DriverInterfaceV1,
    DriverStartContext, ProbeResult, Status,
};
use abi::vfs_rpc::VFS_RPC_MAX_REQ;
use driver::VirtioNetDriver;
use ipc_helpers::provider::ProviderLoop;
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind, device_kind_bytes};
use stem::syscall::port_create;
use stem::syscall::vfs::vfs_mount;
use stem::{error, warn};
use vfs_provider::{NetVfsState, handle_vfs_rpc};

const DEFAULT_MOUNT_PATH: &str = "/dev/net/virtio0";
const THINGOS_DRIVER_NAME: &[u8] = b"virtio_netd";

#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    fn thingos_driver_start_safe(ctx: *const DriverStartContext) -> Status;
}

#[unsafe(no_mangle)]
#[used]
pub static THING_DRIVER_V1: DriverInterfaceV1 = DriverInterfaceV1 {
    abi_version: DRIVER_INTERFACE_ABI_VERSION,
    flags: DRIVER_FLAG_PCI,
    vendor_id: 0x1af4,
    device_id: 0,
    class_code: 0x020000,
    class_mask: 0xffff00,
    entry_symbol: [0; 32],
};

#[unsafe(no_mangle)]
#[used]
pub static THINGOS_DRIVER: DriverDescriptor = DriverDescriptor {
    abi_version: DRIVER_DESCRIPTOR_ABI_VERSION,
    driver_name_ptr: THINGOS_DRIVER_NAME.as_ptr(),
    driver_name_len: THINGOS_DRIVER_NAME.len(),
    driver_class: DriverClass::Net,
    flags: 0,
    probe: thingos_driver_probe,
    #[cfg(target_arch = "x86_64")]
    start: thingos_driver_start_safe,
    #[cfg(not(target_arch = "x86_64"))]
    start: thingos_driver_start_rust,
};

unsafe extern "C" fn thingos_driver_probe(dev: *const DeviceInfo, out: *mut ProbeResult) -> Status {
    if dev.is_null() || out.is_null() {
        return Status::InvalidArgument;
    }
    let dev = &*dev;
    let out = &mut *out;
    let class_major = ((dev.class_code >> 16) & 0xff) as u8;
    let is_match =
        dev.bus == BusKind::Pci as u32 && dev.vendor_id as u16 == 0x1af4 && class_major == 0x02;
    out.matched = if is_match { 1 } else { 0 };
    out.score = if is_match { 1000 } else { 0 };
    out.claimed_class = DriverClass::Net;
    out.flags = 0;
    if is_match { Status::Ok } else { Status::NoMatch }
}

#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(
    r#"
    .section .text
    .global thingos_driver_start_safe
    thingos_driver_start_safe:
        // RSP = 16n (kernel spawn)
        sub rsp, 8
        push rdi
        // Call std initialization (TLS, etc)
        call thingos_runtime_setup
        // Restore RDI and realign for the next call.
        pop rdi
        add rsp, 8
        // CALL will push 8 bytes, so inside Rust entry RSP = 16n + 8.
        call thingos_driver_start_rust
        ret
"#
);

#[unsafe(no_mangle)]
unsafe extern "C" fn thingos_driver_start_rust(boot_fd: usize) -> Status {
    main(boot_fd)
}

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: device_kind_bytes(b"dev.net.Nic"),
    version: 1,
    _reserved: 0,
};

struct SupervisorBootstrap {
    drv_req_read: u32,
    drv_resp_write: u32,
    bind_instance_id: u64,
}

#[unsafe(no_mangle)]
pub extern "C" fn thing_driver_entry_v1(ctx_ptr: u64, ctx_len: u32) -> i32 {
    let claimed_path =
        if ctx_ptr != 0 && (ctx_len as usize) >= core::mem::size_of::<DriverEntryCtx>() {
            let ctx = unsafe { &*(ctx_ptr as *const DriverEntryCtx) };
            let s = ctx.device_path_str();
            if s.is_empty() { None } else { Some(s.to_string()) }
        } else {
            None
        };

    run_driver(claimed_path, None)
}

#[stem::main]
fn main(arg: usize) -> ! {
    let (claimed_path, bootstrap) = parse_supervisor_bootstrap(arg);
    run_driver(claimed_path, bootstrap)
}

fn parse_supervisor_bootstrap(arg: usize) -> (Option<String>, Option<SupervisorBootstrap>) {
    let mut claimed_path = None;
    let mut bootstrap = None;

    if arg != 0 {
        use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
        let req = VmMapReq {
            addr_hint: 0,
            len: 4096,
            prot: VmProt::READ | VmProt::USER,
            flags: VmMapFlags::empty(),
            backing: VmBacking::File { thing: arg as u32, offset: 0 },
        };
        if let Ok(resp) = stem::syscall::vm_map(&req) {
            // New cambium path: boot arg contains DriverEntryCtx bytes.
            let entry_ctx = unsafe { &*(resp.addr as *const DriverEntryCtx) };
            if entry_ctx.version == 1 {
                let s = entry_ctx.device_path_str();
                if !s.is_empty() {
                    claimed_path = Some(s.to_string());
                }
                return (claimed_path, None);
            }

            // Legacy sovereign bootstrap payload.
            let slice = unsafe { core::slice::from_raw_parts(resp.addr as *const u32, 1024) };
            let req_read = slice[0];
            let resp_write = slice[1];
            let id_low = slice[3] as u64;
            let id_high = slice[4] as u64;
            let bind_instance_id = id_low | (id_high << 32);

            let path_bytes =
                unsafe { core::slice::from_raw_parts((resp.addr + 512) as *const u8, 128) };
            let path_len = path_bytes.iter().position(|&b| b == 0).unwrap_or(128);
            let path = core::str::from_utf8(&path_bytes[..path_len]).unwrap_or("").to_string();
            if !path.is_empty() {
                claimed_path = Some(path);
            }

            if req_read != 0 && resp_write != 0 && bind_instance_id != 0 {
                bootstrap = Some(SupervisorBootstrap {
                    drv_req_read: req_read,
                    drv_resp_write: resp_write,
                    bind_instance_id,
                });
            }
        } else {
            warn!("VIRTIO_NETD: Failed to map bootstrap memfd");
        }
    }

    (claimed_path, bootstrap)
}

fn run_driver(claimed_path: Option<String>, bootstrap: Option<SupervisorBootstrap>) -> ! {
    stem::debug!("VIRTIO_NETD: Starting VirtIO-NET driver service...");

    let claimed_path = claimed_path.unwrap_or_default();

    stem::debug!("VIRTIO_NETD: Initializing hardware driver...");

    // Initialize VirtIO-NET driver.
    let mut driver: VirtioNetDriver = match if !claimed_path.is_empty() {
        VirtioNetDriver::claim_device(&claimed_path)
    } else {
        VirtioNetDriver::find_and_claim()
    } {
        Ok(d) => {
            stem::debug!("VIRTIO_NETD: Driver initialized successfully");
            d
        }
        Err(e) => {
            error!("VIRTIO_NETD: Failed to initialize driver: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    let mac = driver.mac();
    stem::debug!(
        "VIRTIO_NETD: MAC {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0],
        mac[1],
        mac[2],
        mac[3],
        mac[4],
        mac[5]
    );

    let initial_link_up = driver.link_up();
    stem::debug!(
        "VIRTIO_NETD: Initial link state is {}",
        if initial_link_up { "UP" } else { "DOWN" }
    );

    let features = driver.device_features();

    // Create the VFS provider port pair.
    //   req_write → kernel sends VFS RPCs here
    //   req_read  → this daemon reads RPCs here
    let (req_write, req_read) = match port_create(VFS_RPC_MAX_REQ * 8) {
        Ok(handles) => {
            stem::debug!("VIRTIO_NETD: Created VFS provider port");
            handles
        }
        Err(e) => {
            error!("VIRTIO_NETD: Failed to create provider port: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    if let Some(bootstrap) = bootstrap {
        let drv_resp_write_fd =
            stem::syscall::vfs::vfs_handle_from_port(bootstrap.drv_resp_write)
                .expect("virtio_netd: vfs_handle_from_port(drv_resp_write)");

        // Sovereign Handshake
        use abi::display_driver_protocol;
        use abi::supervisor_protocol::{self, classes};

        let ready = supervisor_protocol::BindReadyPayload {
            bind_instance_id: bootstrap.bind_instance_id,
            class_mask: classes::NETWORK_INTERFACE,
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
                // Bundle the VFS provider handle and the BIND_READY notification atomically.
                let _ = stem::syscall::socket::sendmsg(
                    drv_resp_write_fd,
                    &buf[..total_len],
                    &[req_write],
                );
                stem::debug!("VIRTIO_NETD: Sent MSG_BIND_READY, waiting for MSG_BIND_ASSIGNED...");
            }
        }

        // Wait for MSG_BIND_ASSIGNED or MSG_BIND_FAILED
        let mut wait_buf = [0u8; 512];
        let assigned_bind_id = loop {
            if let Ok(n) = stem::syscall::port_try_recv(bootstrap.drv_req_read, &mut wait_buf) {
                if let Some((header, payload)) =
                    display_driver_protocol::parse_message(&wait_buf[..n])
                {
                    if header.msg_type == supervisor_protocol::MSG_BIND_ASSIGNED {
                        if let Some(assigned) =
                            supervisor_protocol::decode_bind_assigned_le(payload)
                        {
                            let path_len =
                                assigned.primary_path.iter().position(|&b| b == 0).unwrap_or(64);
                            let path = core::str::from_utf8(&assigned.primary_path[..path_len])
                                .unwrap_or("?");
                            stem::debug!(
                                "VIRTIO_NETD: Sovereign registration COMPLETE. Assigned: {}",
                                path
                            );
                            break assigned.bind_instance_id;
                        }
                    } else if header.msg_type == supervisor_protocol::MSG_BIND_FAILED {
                        if let Some(failed) = supervisor_protocol::decode_bind_failed_le(payload) {
                            let reason_len =
                                failed.reason.iter().position(|&b| b == 0).unwrap_or(64);
                            let reason =
                                core::str::from_utf8(&failed.reason[..reason_len]).unwrap_or("?");
                            stem::warn!(
                                "VIRTIO_NETD: Registration REJECTED by supervisor (code={}, reason={}). Halting.",
                                failed.error_code,
                                reason
                            );
                            loop {
                                stem::syscall::yield_now();
                            }
                        }
                    }
                }
            }
            stem::syscall::yield_now();
        };

        // Notify supervisor that this service is fully operational.
        let svc_ready = supervisor_protocol::ServiceReadyPayload {
            bind_instance_id: assigned_bind_id,
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
                stem::debug!("VIRTIO_NETD: Sent MSG_SERVICE_READY.");
            }
        }
    } else {
        match vfs_mount(req_write, DEFAULT_MOUNT_PATH) {
            Ok(()) => stem::debug!("VIRTIO_NETD: Mounted at {}", DEFAULT_MOUNT_PATH),
            Err(e) => warn!("VIRTIO_NETD: vfs_mount({}) failed: {:?}", DEFAULT_MOUNT_PATH, e),
        }
    }

    // Initialize shared VFS state.
    let mut state = NetVfsState::new(mac, initial_link_up, features);
    stem::debug!(
        "VIRTIO_NETD: Entering VFS provider service loop at {}",
        DEFAULT_MOUNT_PATH
    );

    // Main loop: interleave hardware polling with VFS RPC handling.
    let mut provider_loop = ProviderLoop::new(req_read);
    loop {
        // 1. Poll for link-state changes and queue events.
        if let Some(link_up) = driver.poll_link_change() {
            state.link_up = link_up;
            let event = if link_up { "link-up" } else { "link-down" };
            state.push_event(event);
            stem::debug!("VIRTIO_NETD: Link state changed: {}", event);
        }

        // 2. Poll hardware for received frames and buffer them.
        if let Some(frame) = driver.poll_rx() {
            let frame_vec: alloc::vec::Vec<u8> = frame.to_vec();
            state.push_rx_frame(frame_vec);
        }

        // 3. Service any pending VFS RPC (non-blocking).
        match provider_loop.try_next_request() {
            Ok(Some(req)) => {
                let op = req.op;
                let resp_port = req.resp_port;
                let req_payload_len = req.payload.len();
                stem::trace!(
                    "VIRTIO_NETD: dispatch begin op={:?} resp_port={} payload_len={}",
                    op,
                    resp_port,
                    req_payload_len
                );
                let resp = handle_vfs_rpc(&mut state, &mut driver, &req);
                let resp_status = resp.status;
                let resp_payload_len = resp.payload.len();
                stem::trace!(
                    "VIRTIO_NETD: dispatch end op={:?} resp_port={} status={} resp_payload_len={}",
                    op,
                    resp_port,
                    resp_status,
                    resp_payload_len
                );
                stem::trace!(
                    "VIRTIO_NETD: send_response begin op={:?} resp_port={}",
                    op,
                    resp_port
                );
                if let Err(e) = provider_loop.send_response(resp_port, resp) {
                    warn!("VIRTIO_NETD: send_response failed: {:?}", e);
                } else {
                    stem::trace!(
                        "VIRTIO_NETD: send_response end op={:?} resp_port={}",
                        op,
                        resp_port
                    );
                }
            }
            Ok(None) => {} // no request ready
            Err(e) => {
                warn!("VIRTIO_NETD: provider port error: {:?}", e);
            }
        }

        stem::yield_now();
    }
}
