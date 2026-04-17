#![no_std]
#![no_main]
use alloc::string::ToString;
use alloc::vec::Vec;
extern crate alloc;

use abi::display::{
    BufferHandle, BufferId, CommitRequest, DISPLAY_OP_COMMIT, DISPLAY_OP_GET_INFO,
    DISPLAY_OP_IMPORT_BUFFER, DISPLAY_OP_RELEASE_BUFFER, DisplayCaps, DisplayInfo, DisplayMode,
    PlaneCommit, PlaneId,
};
use abi::display_driver_protocol as drvproto;
use abi::driver_interface::{
    BusKind, DeviceInfo, DriverClass, DriverDescriptor, DriverStartContext, ProbeResult, Status,
    DRIVER_DESCRIPTOR_ABI_VERSION,
};
use abi::vfs_rpc::{VfsRpcOp, VfsRpcReqHeader};
use abi::driver_frame::FrameReader;
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind};
use stem::syscall::{ChannelThing, channel_create, channel_send};
use stem::{info, warn};
use virtio_gpu::{Rect, VirtioGpu};
const THINGOS_DRIVER_NAME: &[u8] = b"display_virtio_gpu";

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
    start: thingos_driver_start_rust,
};

unsafe extern "C" fn thingos_driver_probe(dev: *const DeviceInfo, out: *mut ProbeResult) -> Status {
    if dev.is_null() || out.is_null() {
        return Status::InvalidArgument;
    }
    let dev = &*dev;
    let out = &mut *out;
    let is_match = dev.bus == BusKind::Pci as u32
        && dev.vendor_id as u16 == 0x1af4
        && ((dev.class_code >> 16) & 0xff) == 0x03;
    out.matched = if is_match { 1 } else { 0 };
    out.score = if is_match { 1000 } else { 0 };
    out.claimed_class = DriverClass::Display;
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

// ============================================================================
// Rect Utilities - no allocations, fast inline helpers
// ============================================================================

/// Compute the bounding box (union) of two rectangles
#[inline]
fn rect_union(a: Rect, b: Rect) -> Rect {
    let x1 = a.x.min(b.x);
    let y1 = a.y.min(b.y);
    let x2 = (a.x + a.w).max(b.x + b.w);
    let y2 = (a.y + a.h).max(b.y + b.h);
    Rect { x: x1, y: y1, w: x2.saturating_sub(x1), h: y2.saturating_sub(y1) }
}

/// Compute the area of a rectangle
#[inline]
fn rect_area(r: Rect) -> u64 {
    (r.w as u64) * (r.h as u64)
}

/// Check if a rectangle is empty (zero width or height)
#[inline]
fn rect_is_empty(r: Rect) -> bool {
    r.w == 0 || r.h == 0
}

/// Clamp a rectangle to screen bounds
#[inline]
fn rect_clamp_to_bounds(r: Rect, w: u32, h: u32) -> Rect {
    // Clamp origin to screen
    let x = r.x.min(w);
    let y = r.y.min(h);
    // Clamp extent to remaining screen space
    let max_w = w.saturating_sub(x);
    let max_h = h.saturating_sub(y);
    Rect { x, y, w: r.w.min(max_w), h: r.h.min(max_h) }
}

// ============================================================================
// Instrumentation counters for verification
// ============================================================================

struct Buffer {
    fd: u32,
    res_id: u32,
    phys: u64,
    ptr: *mut u8,
    size: usize,
    last_present_seq: u64,
}

struct ImportedBuffer {
    ptr: *mut u8,
    size: usize,
    width: u32,
    height: u32,
    stride: u32,
}

struct PresentStats {
    frame_count: u32,
    total_rects_in: u32,
    total_transfers: u32,
    total_flushes: u32,
    union_flush_count: u32,
    per_rect_flush_count: u32,
    using_frame_pool: bool,
}

/// Entry in the texture registry mapping client IDs to GPU resource IDs
struct TextureEntry {
    resource_id: u32,
    width: u32,
    height: u32,
}

/// Next resource ID for texture allocation
static NEXT_TEXTURE_RESOURCE_ID: core::sync::atomic::AtomicU32 =
    core::sync::atomic::AtomicU32::new(1000);

impl PresentStats {
    const fn new(frame_pool: bool) -> Self {
        Self {
            frame_count: 0,
            total_rects_in: 0,
            total_transfers: 0,
            total_flushes: 0,
            union_flush_count: 0,
            per_rect_flush_count: 0,
            using_frame_pool: frame_pool,
        }
    }

    fn log_and_reset(&mut self) {
        if self.frame_count > 0 {
            info!(
                "display_virtio_gpu stats: frames={}, rects_in={}, transfers={}, flushes={}, union_flush={}, per_rect_flush={}, frame_pool={}",
                self.frame_count,
                self.total_rects_in,
                self.total_transfers,
                self.total_flushes,
                self.union_flush_count,
                self.per_rect_flush_count,
                self.using_frame_pool
            );
        }
        let fp = self.using_frame_pool;
        *self = Self::new(fp);
    }
}

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: *b"dev.display.Gpu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    version: 1,
    _reserved: 0,
};

fn unpack_handle(arg: usize, index: u32) -> ChannelThing {
    ((arg >> (index * 16)) & 0xFFFF) as ChannelThing
}

fn send_msg(handle: ChannelThing, msg_type: u16, payload: &[u8]) {
    let mut buf = [0u8; 256];
    if let Some(len) = drvproto::encode_message(&mut buf, msg_type, payload) {
        let mut status = stem::syscall::channel_send_all(handle, &buf[..len]);
        if let Err(abi::errors::Errno::EAGAIN) = status {
            // Bridge the handle to a VFS FD for FD-first write-readiness polling.
            if let Ok(fd) = stem::syscall::vfs::vfs_thing_from_channel(handle) {
                let mut pollfds = [abi::syscall::PollThing {
                    thing: fd as i32,
                    events: abi::syscall::poll_flags::POLLOUT,
                    revents: 0,
                }];
                while let Err(abi::errors::Errno::EAGAIN) = status {
                    let _ = stem::syscall::vfs::vfs_poll(&mut pollfds, u64::MAX);
                    status = stem::syscall::channel_send_all(handle, &buf[..len]);
                }
            }
        }
        stem::trace!(
            "display_virtio_gpu: sent msg_type={} handle={} size={} status={:?}",
            msg_type,
            handle,
            len,
            status
        );
    }
}

fn find_gpu() -> Option<alloc::string::String> {
    use abi::syscall::vfs_flags::O_RDONLY;
    use stem::syscall::vfs::{vfs_close, vfs_open, vfs_readdir};
    stem::trace!("display_virtio_gpu: Scanning /sys/devices for PCI GPU...");
    let fd = match vfs_open("/sys/devices", O_RDONLY) {
        Ok(fd) => fd,
        Err(e) => {
            stem::error!("display_virtio_gpu: Failed to open /sys/devices: {:?}", e);
            return None;
        }
    };
    let mut buf = [0u8; 4096];
    let n = vfs_readdir(fd, &mut buf).ok()?;
    let _ = vfs_close(fd);

    let mut offset = 0;
    while offset < n {
        let mut end = offset;
        while end < n && buf[end] != 0 {
            end += 1;
        }
        if end > offset {
            if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
                let path = alloc::format!("/sys/devices/{}", name);
                let vendor = read_sys_u32(&alloc::format!("{}/vendor", path)).unwrap_or(0);
                let device = read_sys_u32(&alloc::format!("{}/device", path)).unwrap_or(0);
                let class = read_sys_u32(&alloc::format!("{}/class", path)).unwrap_or(0);

                // VirtIO Vendor = 0x1af4, Display Class = 0x0300xx,
                // or specifically device 0x1050 or 0x1011
                if vendor == 0x1af4
                    && ((class >> 8) == 0x0300 || device == 0x1050 || device == 0x1011)
                {
                    return Some(path);
                }
            }
        }
        offset = end + 1;
    }
    None
}

fn read_sys_u32(path: &str) -> Option<u32> {
    use abi::syscall::vfs_flags::O_RDONLY;
    use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};

    let fd = vfs_open(path, O_RDONLY).ok()?;
    let mut buf = [0u8; 32];
    let n = vfs_read(fd, &mut buf).ok()?;
    let _ = vfs_close(fd);

    let s = core::str::from_utf8(&buf[..n]).ok()?;
    let trimmed = s.trim();
    if trimmed.starts_with("0x") {
        u32::from_str_radix(&trimmed[2..], 16).ok()
    } else {
        trimmed.parse::<u32>().ok()
    }
}

fn read_sys_string(path: &str) -> Option<alloc::string::String> {
    use abi::syscall::vfs_flags::O_RDONLY;
    use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};

    let fd = vfs_open(path, O_RDONLY).ok()?;
    let mut buf = [0u8; 256];
    let n = vfs_read(fd, &mut buf).ok()?;
    let _ = vfs_close(fd);

    Some(alloc::string::String::from_utf8_lossy(&buf[..n]).to_string())
}

/// Query the boot framebuffer for display dimensions via /sys/firmware/framebuffer.
/// Falls back to 1024x768 if not found.
fn get_display_dimensions() -> (u32, u32, u32, u32) {
    if let Some(info) = read_sys_string("/sys/firmware/framebuffer") {
        let mut width = 1024;
        let mut height = 768;
        let mut stride = 4096;
        let mut format = 1;

        for line in info.lines() {
            if let Some(val) = line.strip_prefix("width=") {
                width = val.parse().unwrap_or(width);
            } else if let Some(val) = line.strip_prefix("height=") {
                height = val.parse().unwrap_or(height);
            } else if let Some(val) = line.strip_prefix("stride=") {
                stride = val.parse().unwrap_or(stride);
            } else if let Some(val) = line.strip_prefix("format=") {
                format = val.parse().unwrap_or(format);
            }
        }
        return (width, height, stride, format);
    }

    // Fallback defaults
    (1024, 768, 1024 * 4, 1)
}

#[stem::main]
fn main(boot_arg: usize) -> ! {
    stem::info!("display_virtio_gpu: starting v0.4.1 (boot_arg={})", boot_arg);

    if boot_arg == 0 {
        stem::error!("display_virtio_gpu: No boot argument provided! Standard driver entry required.");
        stem::syscall::exit(1);
    }    stem::info!("display_virtio_gpu: Starting VFS-native VirtIO GPU driver...");
    stem::info!("display_virtio_gpu: boot_arg={}", boot_arg);

    let mut drv_req_read = 0;
    let mut drv_resp_write = 0;
    let mut supervisor_port = 0;
    let mut bind_instance_id = 0;

    let boot_size = 4096;
    let req = abi::vm::VmMapReq {
        addr_hint: 0,
        len: boot_size,
        prot: abi::vm::VmProt::READ | abi::vm::VmProt::USER,
        flags: abi::vm::VmMapFlags::empty(),
        backing: abi::vm::VmBacking::File { thing: boot_arg as u32, offset: 0 },
    };

    stem::info!("display_virtio_gpu: Mapping bootstrap memfd {} size={}...", boot_arg, boot_size);
    match stem::syscall::vm_map(&req) {
        Ok(resp) => {
            stem::info!("display_virtio_gpu: vm_map success at 0x{:x}", resp.addr);
            let slice = unsafe { core::slice::from_raw_parts(resp.addr as *const u32, 1024) };
            // slice[3..5]: bind_instance_id (u64)

            drv_req_read = slice[0];
            drv_resp_write = slice[1];
            supervisor_port = slice[2];

            let id_low = slice[3] as u64;
            let id_high = slice[4] as u64;
            bind_instance_id = id_low | (id_high << 32);

            stem::info!(
                "display_virtio_gpu: Recovered handles: req_read={}, resp_write={}, svc={}, id={}",
                drv_req_read,
                drv_resp_write,
                supervisor_port,
                bind_instance_id
            );
        }
        Err(e) => {
            stem::info!("display_virtio_gpu: ERROR: Failed to vm_map bootstrap memfd {}: {:?}", boot_arg, e);
        }
    }

    if drv_req_read == 0 || drv_resp_write == 0 || supervisor_port == 0 || bind_instance_id == 0 {
        stem::info!(
            "DISP: ERROR: Invalid/Missing bootstrap components (req={}, resp={}, svc={}, id={})",
            drv_req_read,
            drv_resp_write,
            supervisor_port,
            bind_instance_id
        );
        loop {
            stem::yield_now();
        }
    }

    info!(
        "display_virtio_gpu: starting (drv_req_r={}, drv_resp_w={}, svc={}, id={})",
        drv_req_read, drv_resp_write, supervisor_port, bind_instance_id
    );

    // Find and initialize GPU
    let gpu_path = match find_gpu() {
        Some(path) => path,
        None => {
            info!("display_virtio_gpu: GPU device not found");
            loop {
                stem::time::sleep_ms(1);
            }
        }
    };

    let mut gpu = match VirtioGpu::new(&gpu_path) {
        Ok(g) => g,
        Err(e) => {
            info!("display_virtio_gpu: Failed to initialize GPU: {:?}", e);
            loop {
                stem::time::sleep_ms(1);
            }
        }
    };

    if let Err(e) = gpu.init_virtio() {
        info!("display_virtio_gpu: Virtio init failed: {}", e);
        loop {
            stem::time::sleep_ms(1);
        }
    }

    info!("display_virtio_gpu: GPU initialized successfully");

    if gpu.has_3d_feature() {
        info!("display_virtio_gpu: Virgl 3D supported");
    } else {
        info!("display_virtio_gpu: Virgl 3D not supported, using 2D only");
    }

    // =========================================================================
    // FRAME POOL SETUP: Create GPU resources and bytespaces for triple buffering
    // =========================================================================
    let (disp_width, disp_height, disp_stride, disp_format) = get_display_dimensions();
    let disp_size = (disp_height as usize) * (disp_stride as usize);

    info!(
        "display_virtio_gpu: creating frame pool 1x {}x{} stride={} format={}",
        disp_width, disp_height, disp_stride, disp_format
    );

    // Single buffer for now - multi-buffer requires cross-process bytespace access
    let frame_pool_count = 1;
    let mut frame_pool_buffers = alloc::vec::Vec::new();
    for i in 0..frame_pool_count {
        let fd = match stem::syscall::memfd_create("frame_pool", disp_size) {
            Ok(id) => id,
            Err(e) => {
                info!("display_virtio_gpu: memfd_create failed: {:?}", e);
                loop {
                    stem::time::sleep_ms(1);
                }
            }
        };

        let phys = match stem::syscall::shared_memory_phys(fd) {
            Ok(phys) => phys,
            Err(e) => {
                info!("display_virtio_gpu: shared_memory_phys failed: {:?}", e);
                loop {
                    stem::time::sleep_ms(1);
                }
            }
        };

        // Explicitly map it locally so it stays pinned/resident
        let mut req: abi::vm::VmMapReq = unsafe { core::mem::zeroed() };
        req.backing = abi::vm::VmBacking::File { thing: fd, offset: 0 };
        req.len = disp_size;
        req.prot = abi::vm::VmProt::READ | abi::vm::VmProt::WRITE | abi::vm::VmProt::USER;
        let map_resp = match stem::syscall::vm_map(&req) {
            Ok(resp) => resp,
            Err(e) => {
                info!("display_virtio_gpu: vm_map(frame_pool) failed: {:?}", e);
                loop {
                    stem::time::sleep_ms(1);
                }
            }
        };

        let res_id = (i + 1) as u32;
        gpu.set_dimensions(disp_width, disp_height);
        if let Err(e) = gpu.create_resource_2d(res_id) {
            info!("display_virtio_gpu: create_resource_2d failed: {}", e);
            loop {
                stem::time::sleep_ms(1);
            }
        }
        if let Err(e) = gpu.attach_backing(res_id, phys, disp_size, disp_stride) {
            info!("display_virtio_gpu: attach_backing failed: {}", e);
            loop {
                stem::time::sleep_ms(1);
            }
        }

        frame_pool_buffers.push(Buffer {
            fd,
            res_id,
            phys,
            ptr: map_resp.addr as *mut u8,
            size: disp_size,
            last_present_seq: 0,
        });
    }

    // Set initial scanout to first buffer
    if let Err(e) = gpu.set_scanout(frame_pool_buffers[0].res_id, disp_width, disp_height) {
        info!("display_virtio_gpu: set_scanout failed: {}", e);
        loop {
            stem::time::sleep_ms(1);
        }
    }

    info!(
        "display_virtio_gpu: frame pool ready ({} buffer{})",
        frame_pool_count,
        if frame_pool_count == 1 { "" } else { "s" }
    );

    // =========================================================================
    // SOVEREIGN REGISTRATION: Handshake with sprout supervisor
    // =========================================================================
    use abi::supervisor_protocol::{self, classes};
    use abi::vfs_rpc::VFS_RPC_MAX_REQ;

    // Create VFS provider port
    let (vfs_write, vfs_read) =
        channel_create(VFS_RPC_MAX_REQ * 8).expect("Failed to create VFS port");

    // Bridge the supervisor channel handle to a VFS FD for sendmsg.
    let supervisor_port_fd = stem::syscall::vfs::vfs_thing_from_channel(supervisor_port)
        .expect("display_virtio_gpu: vfs_thing_from_channel(supervisor_port)");

    // Send MSG_BIND_READY to supervisor instead of legacy MSG_REGISTER
    let ready = supervisor_protocol::BindReadyPayload {
        bind_instance_id,
        class_mask: classes::DISPLAY_CARD | classes::FRAMEBUFFER,
        _reserved: 0,
    };
    let mut ready_bytes = [0u8; supervisor_protocol::BIND_READY_PAYLOAD_SIZE];
    if let Some(len) = supervisor_protocol::encode_bind_ready_le(&ready, &mut ready_bytes) {
        // Wrap in common driver header
        let mut buf = [0u8; 256];
        if let Some(total_len) = drvproto::encode_message(
            &mut buf,
            supervisor_protocol::MSG_BIND_READY,
            &ready_bytes[..len],
        ) {
            // Bundle the VFS provider handle and BIND_READY notification atomically.
            let _ =
                stem::syscall::socket::sendmsg(supervisor_port_fd, &buf[..total_len], &[vfs_write]);
            info!("display_virtio_gpu: Sent MSG_BIND_READY (ID: {})", bind_instance_id);
        }
    }

    let mut buf = [0u8; 512];
    let mut frames = FrameReader::<4096>::new();

    let mut current_fd: Option<u32> = None;
    let mut current_res_id: u32 = 1;
    let mut next_buffer_idx = 0;
    let mut present_seq: u64 = 0;
    let mut last_presented_idx: Option<usize> = None;
    let mut imported_buffers: alloc::collections::BTreeMap<BufferId, ImportedBuffer> =
        alloc::collections::BTreeMap::new();
    let mut next_import_id: u32 = 1;

    let mut stats = PresentStats::new(true);
    const STATS_LOG_INTERVAL: u32 = 120;

    // Texture registry for 3D textures (client_id → TextureEntry)
    let mut texture_registry: alloc::collections::BTreeMap<u64, TextureEntry> =
        alloc::collections::BTreeMap::new();

    // Wait for MSG_BIND_ASSIGNED or MSG_BIND_FAILED
    let mut wait_buf = [0u8; 512];
    info!("display_virtio_gpu: Waiting for BIND_ASSIGNED...");
    let assigned_bind_id = loop {
        if let Ok(n) = stem::syscall::channel_try_recv(drv_req_read, &mut wait_buf) {
            if let Some((header, payload)) = drvproto::parse_message(&wait_buf[..n]) {
                if header.msg_type == supervisor_protocol::MSG_BIND_ASSIGNED {
                    if let Some(assigned) = supervisor_protocol::decode_bind_assigned_le(payload) {
                        let path_len =
                            assigned.primary_path.iter().position(|&b| b == 0).unwrap_or(64);
                        let assigned_path = alloc::string::String::from_utf8_lossy(
                            &assigned.primary_path[..path_len],
                        )
                        .to_string();
                        info!(
                            "display_virtio_gpu: Sovereign registration COMPLETE. Assigned: {}",
                            assigned_path
                        );
                        break assigned.bind_instance_id;
                    }
                } else if header.msg_type == supervisor_protocol::MSG_BIND_FAILED {
                    if let Some(failed) = supervisor_protocol::decode_bind_failed_le(payload) {
                        let reason_len = failed.reason.iter().position(|&b| b == 0).unwrap_or(64);
                        let reason =
                            core::str::from_utf8(&failed.reason[..reason_len]).unwrap_or("?");
                        warn!(
                            "display_virtio_gpu: Registration REJECTED by supervisor (code={}, reason={}). Halting.",
                            failed.error_code, reason
                        );
                        loop {
                            stem::yield_now();
                        }
                    }
                }
            }
        }
        stem::time::sleep_ms(10);
    };

    // Notify supervisor that this service is now fully operational.
    {
        let svc_ready = supervisor_protocol::ServiceReadyPayload {
            bind_instance_id: assigned_bind_id,
            _reserved: 0,
        };
        let mut payload_bytes = [0u8; supervisor_protocol::SERVICE_READY_PAYLOAD_SIZE];
        let mut svc_buf = [0u8; 64];
        if let Some(p_len) =
            supervisor_protocol::encode_service_ready_le(&svc_ready, &mut payload_bytes)
        {
            if let Some(total_len) = drvproto::encode_message(
                &mut svc_buf,
                supervisor_protocol::MSG_SERVICE_READY,
                &payload_bytes[..p_len],
            ) {
                let _ =
                    stem::syscall::socket::sendmsg(supervisor_port_fd, &svc_buf[..total_len], &[]);
                info!("display_virtio_gpu: Sent MSG_SERVICE_READY.");
            }
        }
    }

    let drv_req_fd = stem::syscall::vfs::vfs_thing_from_channel(drv_req_read)
        .expect("display_virtio_gpu: fd_from_handle(drv_req_read)");
    let drv_resp_write_fd = stem::syscall::vfs::vfs_thing_from_channel(drv_resp_write)
        .expect("display_virtio_gpu: fd_from_handle(drv_resp_write)");
    let vfs_read_fd = stem::syscall::vfs::vfs_thing_from_channel(vfs_read)
        .expect("display_virtio_gpu: fd_from_handle(vfs_read)");
    let mut ws = stem::wait_set::WaitSet::new();
    let drv_req_read_tok = ws.add_fd_readable(drv_req_fd).unwrap();
    let vfs_read_tok = ws.add_fd_readable(vfs_read_fd).unwrap();

    loop {
        stem::trace!("display_virtio_gpu: waiting on WaitSet...");
        match ws.wait(Some(core::time::Duration::from_millis(10))) {
            Ok(events) => {
                for ev in events {
                    if ev.token() == drv_req_read_tok && ev.is_readable() {
                        stem::trace!("display_virtio_gpu: drv_req readable token fired");
                    }
                    if ev.token() == vfs_read_tok && ev.is_readable() {
                        stem::trace!("display_virtio_gpu: vfs_read readable token fired");
                    }
                }

                // Opportunistically drain VFS RPCs every loop. A short timed wait
                // plus non-blocking drain avoids deadlocks from missed readiness edges.
                let mut vfs_buf = [0u8; VFS_RPC_MAX_REQ];
                while let Ok(n) = stem::syscall::channel_try_recv(vfs_read, &mut vfs_buf) {
                    if n >= 5 {
                        let resp_port =
                            u32::from_le_bytes([vfs_buf[0], vfs_buf[1], vfs_buf[2], vfs_buf[3]])
                                as ChannelThing;
                        let op = VfsRpcOp::from_u8(vfs_buf[4]);
                        stem::trace!("display_virtio_gpu: VFS RPC recv n={} op={:?}", n, op);
                        match op {
                            Some(VfsRpcOp::Lookup) => {
                                // We are a terminal node for card0
                                let mut resp = [0u8; 9];
                                resp[0] = 0; // E_OK
                                let handle: u64 = 1; // card
                                resp[1..9].copy_from_slice(&handle.to_le_bytes());
                                let _ = channel_send(resp_port, &resp);
                            }
                            Some(VfsRpcOp::Stat) => {
                                let mut resp = [0u8; 21];
                                resp[0] = 0; // E_OK
                                let mode: u32 = 0o020000 | 0o666; // S_IFCHR
                                let size: u64 = 0;
                                let handle: u64 = 1;
                                resp[1..5].copy_from_slice(&mode.to_le_bytes());
                                resp[5..13].copy_from_slice(&size.to_le_bytes());
                                resp[13..21].copy_from_slice(&handle.to_le_bytes());
                                let _ = channel_send(resp_port, &resp);
                            }
                            Some(VfsRpcOp::DeviceCall) => {
                                let payload = &vfs_buf[core::mem::size_of::<VfsRpcReqHeader>()..n];
                                if payload.len()
                                    >= 8 + core::mem::size_of::<abi::device::DeviceCall>()
                                {
                                    let call: abi::device::DeviceCall = unsafe {
                                        core::ptr::read_unaligned(
                                            payload[8..8 + core::mem::size_of::<
                                                abi::device::DeviceCall,
                                            >(
                                            )]
                                                .as_ptr()
                                                as *const _,
                                        )
                                    };
                                    let call_payload = &payload
                                        [8 + core::mem::size_of::<abi::device::DeviceCall>()..];

                                    match call.op {
                                        DISPLAY_OP_GET_INFO => {
                                            let info = DisplayInfo {
                                                card_id: 0,
                                                preferred_mode: DisplayMode {
                                                    width: disp_width,
                                                    height: disp_height,
                                                    refresh_mhz: 60000,
                                                },
                                                plane_count: 1,
                                                max_buffers: 32,
                                                supported_formats: 1 << 1,
                                                caps: DisplayCaps::ATOMIC,
                                            };
                                            let out_bytes = unsafe {
                                                core::slice::from_raw_parts(
                                                    &info as *const _ as *const u8,
                                                    core::mem::size_of::<DisplayInfo>(),
                                                )
                                            };
                                            let mut resp = Vec::with_capacity(9 + out_bytes.len());
                                            resp.push(0); // E_OK
                                            resp.extend_from_slice(&0u32.to_le_bytes()); // ret_val
                                            resp.extend_from_slice(
                                                &(out_bytes.len() as u32).to_le_bytes(),
                                            );
                                            resp.extend_from_slice(out_bytes);
                                            let _ = channel_send(resp_port, &resp);
                                        }
                                        DISPLAY_OP_IMPORT_BUFFER => {
                                            if call_payload.len()
                                                < core::mem::size_of::<BufferHandle>()
                                            {
                                                let _ = channel_send(resp_port, &[22]); // EINVAL
                                                continue;
                                            }

                                            let handle: BufferHandle = unsafe {
                                                core::ptr::read_unaligned(
                                                    call_payload.as_ptr() as *const _
                                                )
                                            };
                                            let size = (handle.height as usize)
                                                .saturating_mul(handle.stride as usize);
                                            let req = abi::vm::VmMapReq {
                                                addr_hint: 0,
                                                len: size,
                                                prot: abi::vm::VmProt::READ | abi::vm::VmProt::USER,
                                                flags: abi::vm::VmMapFlags::PRIVATE,
                                                backing: abi::vm::VmBacking::File {
                                                    thing: handle.thing,
                                                    offset: handle.offset,
                                                },
                                            };

                                            match stem::syscall::vm_map(&req) {
                                                Ok(map_resp) => {
                                                    let id = BufferId(next_import_id);
                                                    next_import_id =
                                                        next_import_id.saturating_add(1);
                                                    imported_buffers.insert(
                                                        id,
                                                        ImportedBuffer {
                                                            ptr: map_resp.addr as *mut u8,
                                                            size,
                                                            width: handle.width,
                                                            height: handle.height,
                                                            stride: handle.stride,
                                                        },
                                                    );

                                                    let mut resp = Vec::with_capacity(9);
                                                    resp.push(0); // E_OK
                                                    resp.extend_from_slice(&id.0.to_le_bytes()); // ret_val
                                                    resp.extend_from_slice(&0u32.to_le_bytes()); // out_data_len
                                                    let _ = channel_send(resp_port, &resp);
                                                }
                                                Err(_) => {
                                                    let _ = channel_send(resp_port, &[12]); // ENOMEM
                                                }
                                            }
                                        }
                                        DISPLAY_OP_RELEASE_BUFFER => {
                                            if call_payload.len() < 4 {
                                                let _ = channel_send(resp_port, &[22]); // EINVAL
                                                continue;
                                            }

                                            let id = BufferId(u32::from_le_bytes(
                                                call_payload[..4].try_into().unwrap(),
                                            ));
                                            if let Some(buf) = imported_buffers.remove(&id) {
                                                let _ = stem::syscall::vm_unmap(
                                                    buf.ptr as usize,
                                                    buf.size,
                                                );
                                                let mut resp = Vec::with_capacity(9);
                                                resp.push(0); // E_OK
                                                resp.extend_from_slice(&0u32.to_le_bytes()); // ret_val
                                                resp.extend_from_slice(&0u32.to_le_bytes()); // out_data_len
                                                let _ = channel_send(resp_port, &resp);
                                            } else {
                                                let _ = channel_send(resp_port, &[2]); // ENOENT
                                            }
                                        }
                                        DISPLAY_OP_COMMIT => {
                                            let header_size = core::mem::size_of::<CommitRequest>();
                                            if call_payload.len() < header_size {
                                                let _ = channel_send(resp_port, &[22]); // EINVAL
                                                continue;
                                            }

                                            let req: CommitRequest = unsafe {
                                                core::ptr::read_unaligned(
                                                    call_payload.as_ptr() as *const _
                                                )
                                            };

                                            let plane_size = core::mem::size_of::<PlaneCommit>();
                                            let plane_count = req.commit_count as usize;
                                            let needed = header_size.saturating_add(
                                                plane_count.saturating_mul(plane_size),
                                            );
                                            if plane_count > 0 && call_payload.len() < needed {
                                                let _ = channel_send(resp_port, &[22]); // EINVAL
                                                continue;
                                            }

                                            let mut planes = Vec::with_capacity(plane_count);
                                            if plane_count > 0 {
                                                let raw_planes = &call_payload[header_size..needed];
                                                for i in 0..plane_count {
                                                    let off = i * plane_size;
                                                    let plane: PlaneCommit = unsafe {
                                                        core::ptr::read_unaligned(
                                                            raw_planes[off..off + plane_size]
                                                                .as_ptr()
                                                                as *const _,
                                                        )
                                                    };
                                                    planes.push(plane);
                                                }
                                            }

                                            if !planes.is_empty() {
                                                let idx = next_buffer_idx;
                                                next_buffer_idx = (next_buffer_idx + 1)
                                                    % frame_pool_buffers.len();

                                                let target = &mut frame_pool_buffers[idx];
                                                let mut damage: Option<Rect> = None;
                                                let bpp = if disp_width > 0 {
                                                    (disp_stride / disp_width).max(1) as usize
                                                } else {
                                                    4usize
                                                };

                                                for plane in &planes {
                                                    if plane.plane_id != PlaneId(0) {
                                                        continue;
                                                    }

                                                    let src = if let Some(src) =
                                                        imported_buffers.get(&plane.buffer_id)
                                                    {
                                                        src
                                                    } else {
                                                        continue;
                                                    };

                                                    let src_x =
                                                        plane.src_rect.x.min(src.width) as usize;
                                                    let src_y =
                                                        plane.src_rect.y.min(src.height) as usize;
                                                    let src_w = plane.src_rect.w.min(
                                                        src.width.saturating_sub(plane.src_rect.x),
                                                    )
                                                        as usize;
                                                    let src_h = plane.src_rect.h.min(
                                                        src.height.saturating_sub(plane.src_rect.y),
                                                    )
                                                        as usize;

                                                    let dst_x =
                                                        plane.dest_rect.x.min(disp_width) as usize;
                                                    let dst_y =
                                                        plane.dest_rect.y.min(disp_height) as usize;
                                                    let dst_w = plane.dest_rect.w.min(
                                                        disp_width
                                                            .saturating_sub(plane.dest_rect.x),
                                                    )
                                                        as usize;
                                                    let dst_h = plane.dest_rect.h.min(
                                                        disp_height
                                                            .saturating_sub(plane.dest_rect.y),
                                                    )
                                                        as usize;

                                                    let copy_w = src_w.min(dst_w);
                                                    let copy_h = src_h.min(dst_h);
                                                    if copy_w == 0 || copy_h == 0 {
                                                        continue;
                                                    }
                                                    let row_bytes = copy_w.saturating_mul(bpp);

                                                    for row in 0..copy_h {
                                                        unsafe {
                                                            core::ptr::copy_nonoverlapping(
                                                                src.ptr.add(
                                                                    (src_y + row).saturating_mul(
                                                                        src.stride as usize,
                                                                    ) + src_x.saturating_mul(bpp),
                                                                ),
                                                                target.ptr.add(
                                                                    (dst_y + row).saturating_mul(
                                                                        disp_stride as usize,
                                                                    ) + dst_x.saturating_mul(bpp),
                                                                ),
                                                                row_bytes,
                                                            );
                                                        }
                                                    }

                                                    let rect = Rect {
                                                        x: dst_x as u32,
                                                        y: dst_y as u32,
                                                        w: copy_w as u32,
                                                        h: copy_h as u32,
                                                    };
                                                    damage = Some(if let Some(old) = damage {
                                                        rect_union(old, rect)
                                                    } else {
                                                        rect
                                                    });
                                                }

                                                if let Some(dmg) = damage {
                                                    let _ =
                                                        gpu.transfer_to_host(target.res_id, dmg);
                                                    let _ = gpu.flush_resource(target.res_id, dmg);
                                                    let _ = gpu.set_scanout(
                                                        target.res_id,
                                                        disp_width,
                                                        disp_height,
                                                    );

                                                    present_seq += 1;
                                                    target.last_present_seq = present_seq;
                                                    last_presented_idx = Some(idx);
                                                    current_res_id = target.res_id;
                                                    current_fd = Some(target.fd);
                                                }
                                            }

                                            let mut resp = Vec::with_capacity(9);
                                            resp.push(0); // E_OK
                                            resp.extend_from_slice(&0u32.to_le_bytes()); // ret_val
                                            resp.extend_from_slice(&0u32.to_le_bytes()); // out_data_len
                                            let _ = channel_send(resp_port, &resp);
                                        }
                                        _ => {
                                            let _ = channel_send(resp_port, &[38]); // E_NOTSUP
                                        }
                                    }
                                } else {
                                    let _ = channel_send(resp_port, &[22]); // E_INVAL
                                }
                            }
                            Some(VfsRpcOp::SubscribeReady) => {
                                let _ = channel_send(resp_port, &[0]); // E_OK
                            }
                            Some(VfsRpcOp::UnsubscribeReady) => {
                                let _ = channel_send(resp_port, &[0]); // E_OK
                            }
                            Some(VfsRpcOp::Rename) => {
                                let _ = channel_send(resp_port, &[38]); // E_NOTSUP
                            }
                            _ => {
                                let _ = channel_send(resp_port, &[38]); // E_NOTSUP
                            }
                        }
                    }
                }

                let mut read_total = 0;
                // Drain with non-blocking receives only. A blocking recv here can
                // starve VFS RPC handling and wedge /dev/display/card0 clients.
                loop {
                    match stem::syscall::channel_try_recv(drv_req_read, &mut buf) {
                        Ok(n) => {
                            if n == 0 {
                                break;
                            }
                            frames.push(&buf[..n]);
                            read_total += n;
                        }
                        Err(abi::errors::Errno::EAGAIN) => break,
                        Err(e) => {
                            stem::error!("display_virtio_gpu: channel_try_recv ERR: {:?}", e);
                            break;
                        }
                    }
                }
                if read_total > 0 {
                    stem::trace!(
                        "display_virtio_gpu: WaitSet read {} bytes, dropped={}",
                        read_total,
                        frames.dropped_bytes()
                    );
                }
            }
            Err(e) => {
                stem::trace!("display_virtio_gpu: WaitSet returned ERR: {:?}", e);
            }
        }

        while let Some((header, payload)) = frames.next_message() {
            stem::trace!(
                "display_virtio_gpu: next_message -> msg_type={}, len={}",
                header.msg_type,
                payload.len()
            );
            match header.msg_type {
                drvproto::MSG_HELLO => {
                    info!("display_virtio_gpu: received MSG_HELLO");
                    let want_caps = drvproto::decode_hello_payload_le(payload)
                        .map(|hello| hello.want_caps)
                        .unwrap_or(0);
                    let supported_caps = drvproto::CAP_DIRTY_RECTS | drvproto::CAP_FULLFRAME;
                    let welcome = drvproto::WelcomePayload {
                        proto_major: drvproto::PROTO_MAJOR,
                        proto_minor: drvproto::PROTO_MINOR,
                        have_caps: supported_caps & want_caps,
                        max_rects: 8,
                        reserved: 0,
                    };
                    let mut welcome_bytes = [0u8; drvproto::WELCOME_PAYLOAD_WIRE_SIZE];
                    if let Some(len) =
                        drvproto::encode_welcome_payload_le(&welcome, &mut welcome_bytes)
                    {
                        send_msg(drv_resp_write, drvproto::MSG_WELCOME, &welcome_bytes[..len]);
                    }
                }
                drvproto::MSG_ACQUIRE => {
                    stem::info!("display_virtio_gpu: received MSG_ACQUIRE");
                    let mut buffer_age = 0;
                    let idx = next_buffer_idx;

                    if last_presented_idx.is_some() {
                        let age =
                            present_seq.saturating_sub(frame_pool_buffers[idx].last_present_seq);
                        buffer_age = if frame_pool_buffers[idx].last_present_seq == 0 {
                            0 // Never presented
                        } else {
                            age as u32
                        };
                    }

                    next_buffer_idx = (next_buffer_idx + 1) % frame_pool_buffers.len();

                    let acquired = drvproto::AcquiredPayload {
                        thing: frame_pool_buffers[idx].fd,
                        _pad1: 0,
                        width: disp_width,
                        height: disp_height,
                        stride: disp_stride,
                        format: disp_format,
                        buffer_age,
                        _pad2: 0,
                    };

                    current_fd = Some(frame_pool_buffers[idx].fd);
                    current_res_id = frame_pool_buffers[idx].res_id;

                    let mut acq_bytes = [0u8; drvproto::ACQUIRED_PAYLOAD_WIRE_SIZE];
                    if let Some(len) =
                        drvproto::encode_acquired_payload_le(&acquired, &mut acq_bytes)
                    {
                        // Wrap payload in the common driver message header.
                        let mut framed = [0u8; 256];
                        if let Some(framed_len) = drvproto::encode_message(
                            &mut framed,
                            drvproto::MSG_ACQUIRED,
                            &acq_bytes[..len],
                        ) {
                            // Send MSG_ACQUIRED data and the frame buffer FD atomically.
                            let _ = stem::syscall::socket::sendmsg(
                                drv_resp_write_fd,
                                &framed[..framed_len],
                                &[frame_pool_buffers[idx].fd],
                            );
                        }
                    }
                }
                drvproto::MSG_BIND => {
                    if let Some(bind) = drvproto::decode_bind_payload_le(payload) {
                        // Receive the framebuffer FD from the message queue (FD-first).
                        let mut fds = [0u32; 1];
                        let fd = match stem::syscall::socket::recvmsg(drv_req_fd, &mut [], &mut fds)
                        {
                            Ok((_, n_fds)) if n_fds > 0 => fds[0],
                            _ => bind.fb_fd,
                        };
                        current_fd = Some(fd);
                        // In legacy mode, we just stay on the first buffer's resource
                        current_res_id = frame_pool_buffers[0].res_id;
                        send_msg(drv_resp_write, drvproto::MSG_ACK, &[]);
                    }
                }
                drvproto::MSG_PRESENT => {
                    if current_fd.is_none() {
                        stem::error!("display_virtio_gpu: current_fd is NONE during MSG_PRESENT!");
                        let err = drvproto::ErrResp { code: 1 };
                        let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                        if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
                            send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                        }
                        continue;
                    }

                    if let Some(present) = drvproto::decode_present_header_le(payload) {
                        let rects_payload = &payload[drvproto::PRESENT_HEADER_WIRE_SIZE..];

                        // Handle full-frame present (rect_count==0 or FULLFRAME flag)
                        if present.rect_count == 0
                            || (present._pad & drvproto::PRESENT_FLAG_FULLFRAME != 0)
                        {
                            let full_rect = Rect { x: 0, y: 0, w: disp_width, h: disp_height };
                            stem::trace!(
                                "display_virtio_gpu: calling present_rect for full_rect..."
                            );
                            let _ = gpu.present_rect(current_res_id, full_rect);
                            stem::trace!("display_virtio_gpu: returned from present_rect!");
                            stats.frame_count += 1;
                            stats.total_transfers += 1;
                            stats.total_flushes += 1;
                        } else {
                            // ============================================================
                            // GPU-Fast Present Path: batch transfers, smart flush
                            // ============================================================

                            // Phase 1: Decode and clamp all rects, skip empty ones
                            let mut valid_rects: alloc::vec::Vec<Rect> = alloc::vec::Vec::new();
                            let rect_size = drvproto::RECT_WIRE_SIZE;

                            for i in 0..present.rect_count as usize {
                                let off = i * rect_size;
                                if rects_payload.len() < off + rect_size {
                                    break;
                                }
                                if let Some(rect) =
                                    drvproto::decode_rect_le(&rects_payload[off..off + rect_size])
                                {
                                    let gpu_rect =
                                        Rect { x: rect.x, y: rect.y, w: rect.w, h: rect.h };
                                    // Clamp to screen bounds and skip empty rects
                                    let clamped =
                                        rect_clamp_to_bounds(gpu_rect, disp_width, disp_height);
                                    if !rect_is_empty(clamped) {
                                        valid_rects.push(clamped);
                                    }
                                }
                            }

                            stats.total_rects_in += valid_rects.len() as u32;

                            if !valid_rects.is_empty() {
                                // Phase 2: Transfer all rects (bandwidth follows true damage)
                                for &rect in &valid_rects {
                                    let _ = gpu.transfer_to_host(current_res_id, rect);
                                }
                                stats.total_transfers += valid_rects.len() as u32;

                                // Phase 3: Compute union and sum of areas for flush policy
                                let mut union_rect = valid_rects[0];
                                let mut sum_area: u64 = 0;
                                for &rect in &valid_rects {
                                    union_rect = rect_union(union_rect, rect);
                                    sum_area += rect_area(rect);
                                }
                                let union_area = rect_area(union_rect);

                                // Phase 4: Smart flush policy
                                // If union is much larger than sum of individual rects,
                                // flush each rect separately to avoid giant flush area
                                if valid_rects.len() > 1 && union_area > sum_area * 2 {
                                    // Distant rects case: per-rect flush
                                    for &rect in &valid_rects {
                                        let _ = gpu.flush_resource(current_res_id, rect);
                                    }
                                    stats.total_flushes += valid_rects.len() as u32;
                                    stats.per_rect_flush_count += 1;
                                } else {
                                    // Common case: single union flush
                                    let _ = gpu.flush_resource(current_res_id, union_rect);
                                    stats.total_flushes += 1;
                                    stats.union_flush_count += 1;
                                }
                            }
                            stats.frame_count += 1;
                        }

                        // ============================================================
                        // FLIP SCANOUT
                        // ============================================================
                        // Now that transfers and flushes for THIS resource are done,
                        // flip the hardware scanout to this resource ID.
                        let _ = gpu.set_scanout(current_res_id, disp_width, disp_height);

                        // Update sequence and age bookkeeping
                        present_seq += 1;
                        let mut presented_idx = 0;
                        for (i, buf) in frame_pool_buffers.iter_mut().enumerate() {
                            if buf.res_id == current_res_id {
                                buf.last_present_seq = present_seq;
                                presented_idx = i;
                                break;
                            }
                        }
                        last_presented_idx = Some(presented_idx);

                        // Rate-limited stats logging
                        if stats.frame_count >= STATS_LOG_INTERVAL {
                            stats.log_and_reset();
                        }
                    }
                    send_msg(drv_resp_write, drvproto::MSG_ACK, &[]);
                }
                drvproto::MSG_SUBMIT_3D => {
                    // Parse Submit3d header
                    if let Some(hdr) = drvproto::decode_submit_3d_header_le(payload) {
                        let cmd_buf = &payload[drvproto::SUBMIT_3D_HEADER_WIRE_SIZE..];
                        if cmd_buf.len() >= hdr.cmd_len as usize {
                            // Submit the virgl commands to the GPU
                            match gpu.submit_3d(hdr.ctx_id, &cmd_buf[..hdr.cmd_len as usize]) {
                                Ok(()) => {
                                    send_msg(drv_resp_write, drvproto::MSG_ACK, &[]);
                                }
                                Err(_e) => {
                                    let err = drvproto::ErrResp { code: 3 };
                                    let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                    if let Some(len) =
                                        drvproto::encode_err_resp_le(&err, &mut err_bytes)
                                    {
                                        send_msg(
                                            drv_resp_write,
                                            drvproto::MSG_ERR,
                                            &err_bytes[..len],
                                        );
                                    }
                                }
                            }
                        } else {
                            // Buffer too short
                            let err = drvproto::ErrResp { code: 2 };
                            let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                            if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
                                send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                            }
                        }
                    }
                }
                drvproto::MSG_CREATE_TEXTURE_3D => {
                    // Create a GPU texture resource
                    if let Some(hdr) = drvproto::decode_create_texture_3d_header_le(payload) {
                        let resource_id = NEXT_TEXTURE_RESOURCE_ID
                            .fetch_add(1, core::sync::atomic::Ordering::Relaxed);

                        // Create 3D resource via VirtIO GPU
                        // target=0 (PIPE_TEXTURE_2D), format=2 (B8G8R8X8), bind=2 (RENDER_TARGET) | 8 (SAMPLER)
                        let tex_target = 0; // PIPE_TEXTURE_2D
                        let tex_format = hdr.format; // Usually 2 for BGRA
                        let tex_bind = 2 | 8; // RENDER_TARGET | SAMPLER_VIEW

                        let result = gpu.create_resource_3d(
                            resource_id,
                            tex_target,
                            tex_format,
                            tex_bind,
                            hdr.width,
                            hdr.height,
                            1, // depth
                        );

                        let status = if result.is_ok() {
                            // Attach resource to virgl context
                            let ctx_id = 1; // Main virgl context
                            if gpu.ctx_attach_resource(ctx_id, resource_id).is_ok() {
                                texture_registry.insert(
                                    hdr.client_id,
                                    TextureEntry {
                                        resource_id,
                                        width: hdr.width,
                                        height: hdr.height,
                                    },
                                );
                                0 // Success
                            } else {
                                2 // Attach failed
                            }
                        } else {
                            1 // Create failed
                        };

                        // Send response
                        let resp = drvproto::TextureCreatedResponse {
                            client_id: hdr.client_id,
                            resource_id,
                            status,
                        };
                        let mut resp_bytes = [0u8; drvproto::TEXTURE_CREATED_RESPONSE_WIRE_SIZE];
                        if let Some(len) =
                            drvproto::encode_texture_created_response_le(&resp, &mut resp_bytes)
                        {
                            send_msg(
                                drv_resp_write,
                                drvproto::MSG_TEXTURE_CREATED,
                                &resp_bytes[..len],
                            );
                        }
                    }
                }
                drvproto::MSG_UPLOAD_TEXTURE_3D => {
                    // Upload pixel data to an existing texture
                    if let Some(hdr) = drvproto::decode_upload_texture_3d_header_le(payload) {
                        let pixel_data = &payload[drvproto::UPLOAD_TEXTURE_3D_HEADER_WIRE_SIZE..];

                        if pixel_data.len() >= hdr.data_len as usize {
                            let data_slice = &pixel_data[..hdr.data_len as usize];

                            // Allocate DMA-accessible memory for texture data
                            match stem::syscall::memfd_create("tex_upload", hdr.data_len as usize) {
                                Ok(fd) => {
                                    // Map the memfd to get a writable pointer
                                    let mut req: abi::vm::VmMapReq = unsafe { core::mem::zeroed() };
                                    req.backing = abi::vm::VmBacking::File { thing: fd, offset: 0 };
                                    req.len = hdr.data_len as usize;
                                    req.prot = abi::vm::VmProt::READ
                                        | abi::vm::VmProt::WRITE
                                        | abi::vm::VmProt::USER;
                                    match stem::syscall::vm_map(&req) {
                                        Ok(resp) => {
                                            let ptr = resp.addr;
                                            // Copy pixel data to DMA buffer
                                            unsafe {
                                                core::ptr::copy_nonoverlapping(
                                                    data_slice.as_ptr(),
                                                    ptr as *mut u8,
                                                    hdr.data_len as usize,
                                                );
                                            }

                                            // Get physical address for attach_backing_3d
                                            match stem::syscall::shared_memory_phys(fd) {
                                                Ok(phys_addr) => {
                                                    // Attach backing and transfer
                                                    if gpu
                                                        .attach_backing_3d(
                                                            hdr.resource_id,
                                                            phys_addr,
                                                            hdr.data_len as usize,
                                                        )
                                                        .is_ok()
                                                    {
                                                        if gpu
                                                            .transfer_to_host_3d(
                                                                1,
                                                                hdr.resource_id,
                                                                hdr.width,
                                                                hdr.height,
                                                                hdr.x as u64,
                                                                hdr.stride,
                                                            )
                                                            .is_ok()
                                                        {
                                                            send_msg(
                                                                drv_resp_write,
                                                                drvproto::MSG_ACK,
                                                                &[],
                                                            );
                                                        } else {
                                                            let err = drvproto::ErrResp { code: 4 };
                                                            let mut err_bytes =
                                                                [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                                            if let Some(len) =
                                                                drvproto::encode_err_resp_le(
                                                                    &err,
                                                                    &mut err_bytes,
                                                                )
                                                            {
                                                                send_msg(
                                                                    drv_resp_write,
                                                                    drvproto::MSG_ERR,
                                                                    &err_bytes[..len],
                                                                );
                                                            }
                                                        }
                                                    } else {
                                                        let err = drvproto::ErrResp { code: 5 };
                                                        let mut err_bytes =
                                                            [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                                        if let Some(len) =
                                                            drvproto::encode_err_resp_le(
                                                                &err,
                                                                &mut err_bytes,
                                                            )
                                                        {
                                                            send_msg(
                                                                drv_resp_write,
                                                                drvproto::MSG_ERR,
                                                                &err_bytes[..len],
                                                            );
                                                        }
                                                    }
                                                }
                                                Err(_) => {
                                                    let err = drvproto::ErrResp { code: 6 };
                                                    let mut err_bytes =
                                                        [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                                    if let Some(len) = drvproto::encode_err_resp_le(
                                                        &err,
                                                        &mut err_bytes,
                                                    ) {
                                                        send_msg(
                                                            drv_resp_write,
                                                            drvproto::MSG_ERR,
                                                            &err_bytes[..len],
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            let err = drvproto::ErrResp { code: 7 };
                                            let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                            if let Some(len) =
                                                drvproto::encode_err_resp_le(&err, &mut err_bytes)
                                            {
                                                send_msg(
                                                    drv_resp_write,
                                                    drvproto::MSG_ERR,
                                                    &err_bytes[..len],
                                                );
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    let err = drvproto::ErrResp { code: 8 };
                                    let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                                    if let Some(len) =
                                        drvproto::encode_err_resp_le(&err, &mut err_bytes)
                                    {
                                        send_msg(
                                            drv_resp_write,
                                            drvproto::MSG_ERR,
                                            &err_bytes[..len],
                                        );
                                    }
                                }
                            }
                        } else {
                            let err = drvproto::ErrResp { code: 2 };
                            let mut err_bytes = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
                            if let Some(len) = drvproto::encode_err_resp_le(&err, &mut err_bytes) {
                                send_msg(drv_resp_write, drvproto::MSG_ERR, &err_bytes[..len]);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        stem::yield_now();
    }
}
