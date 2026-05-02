#![no_std]
#![no_main]
use alloc::string::ToString;
extern crate alloc;

use abi::display::accel2d::{
    ACCEL2D_CMD_ALPHA_BLIT, ACCEL2D_CMD_CLEAR_RECT, ACCEL2D_CMD_COPY_RECT,
    ACCEL2D_CMD_FLUSH_DAMAGE, ACCEL2D_CMD_MASKED_BLIT, ACCEL2D_CMD_ROUNDED_CLIP_BLIT,
    ACCEL2D_CMD_STRETCH_BLIT, ACCEL2D_COMMAND_SIZE, Accel2dBatch, Accel2dCommand,
};
use abi::display::{
    BufferHandle, BufferId, CommitFlags, CommitRequest, DEFAULT_REFRESH_MHZ, DISPLAY_OP_ACCEL2D,
    DISPLAY_OP_COMMIT, DISPLAY_OP_GET_INFO, DISPLAY_OP_IMPORT_BUFFER, DISPLAY_OP_MOVE_CURSOR,
    DISPLAY_OP_RELEASE_BUFFER, DISPLAY_OP_SET_CURSOR, DisplayCaps, DisplayInfo, DisplayMode,
    MoveCursorRequest, PlaneCommit, SetCursorRequest,
};
use abi::display_driver_protocol as drvproto;
use abi::driver_frame::FrameReader;
use abi::driver_interface::{
    BusKind, DRIVER_DESCRIPTOR_ABI_VERSION, DRIVER_INTERFACE_ABI_VERSION, DeviceInfo, DriverClass,
    DriverDescriptor, DriverEntryCtx, DriverInterfaceV1, ProbeResult, Status,
};
use abi::errors::Errno;
use abi::pixel::PixelFormat;
use abi::vfs_rpc::{VFS_RPC_MAX_REQ, VfsRpcOp};
use accel2d_cpu::PixelBuf;
use ipc_helpers::provider::{ProviderLoop, ProviderRequest, ProviderResponse};
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind};
use stem::syscall::message::{KindId, msg_inbox_open, msg_recv_blocking, msg_sendmsg};
use stem::syscall::{PortHandle, port_create, port_send};
use stem::{debug, error, info, trace, warn};
use virtio_gpu::{Rect, VirtioGpu};
const THINGOS_DRIVER_NAME: &[u8] = b"display_virtio_gpu";
const DISPLAY_PROVIDER_POLL_ISOLATION: bool = true;
const DISPLAY_BPP: u32 = 4;
const FRAME_POOL_COUNT: usize = 1;
/// Bytes per pixel for the frame pool buffers (always BGRX8888 / BGRA8888).
const FRAME_POOL_BPP: usize = DISPLAY_BPP as usize;
/// Maximum number of `Accel2dCommand` entries accepted in a single
/// `DISPLAY_OP_ACCEL2D` batch.  Limits CPU time and prevents DoS from
/// malformed large batches.
const MAX_ACCEL2D_BATCH_CMDS: usize = 4096;

/// Maximum cursor image side length (pixels). Allocating DMA pages up-front
/// for this many pixels × 4 bytes ensures the cursor pixel buffer is
/// physically contiguous and GPU-accessible regardless of how the cursor
/// image was originally imported.
///
/// 128 × 128 × 4 = 65 536 bytes → 16 × 4 KiB pages.
const MAX_CURSOR_SIDE: u32 = 128;
/// Bytes per pixel for the ARGB32 cursor image format.
const CURSOR_BPP: usize = 4;
/// Virtio-gpu hardware cursor resources are fixed-size 64x64 ARGB images.
const HW_CURSOR_SIDE: u32 = 64;
const MAX_CURSOR_BYTES: usize =
    (MAX_CURSOR_SIDE as usize) * (MAX_CURSOR_SIDE as usize) * CURSOR_BPP;
const MAX_CURSOR_PAGES: usize = (MAX_CURSOR_BYTES + 4095) / 4096; // = 16

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
    start: thingos_driver_start_rust,
};

#[unsafe(no_mangle)]
#[used]
pub static THING_DRIVER_V1: DriverInterfaceV1 = DriverInterfaceV1 {
    abi_version: DRIVER_INTERFACE_ABI_VERSION,
    flags: 0,
    vendor_id: 0x1af4,
    device_id: 0,
    class_code: 0x030000,
    class_mask: 0xffff00,
    entry_symbol: [0u8; 32],
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
unsafe extern "C" fn thingos_driver_start_rust(ctx: *const DriverEntryCtx) -> Status {
    main(ctx as usize)
}

// ============================================================================
// Rect Utilities - no allocations, fast inline helpers
// ============================================================================

/// Compute the bounding box (union) of two rectangles
#[inline]
fn rect_union(a: Rect, b: Rect) -> Rect {
    let x1 = a.x.min(b.x);
    let y1 = a.y.min(b.y);
    let x2 = a.x.saturating_add(a.w).max(b.x.saturating_add(b.w));
    let y2 = a.y.saturating_add(a.h).max(b.y.saturating_add(b.h));
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

#[inline]
fn rect_intersect(a: Rect, b: Rect) -> Option<Rect> {
    let x1 = a.x.max(b.x);
    let y1 = a.y.max(b.y);
    let x2 = a.x.saturating_add(a.w).min(b.x.saturating_add(b.w));
    let y2 = a.y.saturating_add(a.h).min(b.y.saturating_add(b.h));
    if x2 <= x1 || y2 <= y1 { None } else { Some(Rect { x: x1, y: y1, w: x2 - x1, h: y2 - y1 }) }
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

#[derive(Clone, Copy)]
struct ImportedBuffer {
    fd: u32,
    /// Raw pointer to the client-provided shared-memory region mapped into
    /// this process at import time.  Copying this struct copies the pointer
    /// value, not the underlying memory.
    ///
    /// # Safety
    ///
    /// The pointer is valid for the lifetime of the `ImportedBuffer` entry in
    /// `VirtioGpuDriver::imported_buffers`.  Callers that take a snapshot
    /// (`let snap = *entry`) for the purpose of releasing the `BTreeMap` borrow
    /// before mutably borrowing `driver` must ensure the entry is not released
    /// (via `DISPLAY_OP_RELEASE_BUFFER`) while the snapshot is in use.  In the
    /// single-threaded driver loop this is always satisfied because a single
    /// RPC handler runs to completion before another can begin.
    ptr: *mut u8,
    size: usize,
    width: u32,
    height: u32,
    stride: u32,
    format: PixelFormat,
}

#[inline]
fn bounded_copy_extent(
    src: &ImportedBuffer,
    dst_size: usize,
    dst_stride: usize,
    bpp: usize,
    src_x: usize,
    src_y: usize,
    dst_x: usize,
    dst_y: usize,
    width: usize,
    height: usize,
) -> Option<(usize, usize)> {
    if bpp == 0 || src.stride == 0 || dst_stride == 0 || width == 0 || height == 0 {
        return None;
    }

    let src_stride = src.stride as usize;
    let src_col = src_x.checked_mul(bpp)?;
    let dst_col = dst_x.checked_mul(bpp)?;
    if src_col >= src_stride || dst_col >= dst_stride {
        return None;
    }

    let max_w = width.min((src_stride - src_col) / bpp).min((dst_stride - dst_col) / bpp);
    if max_w == 0 {
        return None;
    }
    let row_bytes = max_w.checked_mul(bpp)?;

    let src_start = src_y.checked_mul(src_stride)?.checked_add(src_col)?;
    let dst_start = dst_y.checked_mul(dst_stride)?.checked_add(dst_col)?;
    let src_first_end = src_start.checked_add(row_bytes)?;
    let dst_first_end = dst_start.checked_add(row_bytes)?;
    if src_first_end > src.size || dst_first_end > dst_size {
        return None;
    }

    let src_rows = 1 + (src.size - src_first_end) / src_stride;
    let dst_rows = 1 + (dst_size - dst_first_end) / dst_stride;
    let max_h = height.min(src_rows).min(dst_rows);
    if max_h == 0 { None } else { Some((max_w, max_h)) }
}

#[inline]
fn cursor_hw_coord(dst: u32, src_len: u32) -> u32 {
    if src_len <= HW_CURSOR_SIDE {
        dst
    } else {
        ((dst as u64 * src_len as u64) / HW_CURSOR_SIDE as u64)
            .min(src_len.saturating_sub(1) as u64) as u32
    }
}

#[inline]
fn cursor_hw_hotspot(hotspot: u32, src_len: u32) -> u32 {
    if src_len == 0 {
        0
    } else if src_len <= HW_CURSOR_SIDE {
        hotspot.min(HW_CURSOR_SIDE.saturating_sub(1))
    } else {
        ((hotspot.min(src_len.saturating_sub(1)) as u64 * HW_CURSOR_SIDE as u64) / src_len as u64)
            .min(HW_CURSOR_SIDE.saturating_sub(1) as u64) as u32
    }
}

struct PresentStats {
    frame_count: u32,
    total_rects_in: u32,
    total_transfers: u32,
    total_flushes: u32,
    union_flush_count: u32,
    per_rect_flush_count: u32,
    using_frame_pool: bool,
    /// Snapshot of `VirtioGpuDriver::gpu_path_planes` at the last log reset,
    /// used to compute the per-interval delta.
    gpu_path_planes_at_last_log: u64,
    /// Snapshot of `VirtioGpuDriver::cpu_fallback_planes` at the last log reset,
    /// used to compute the per-interval delta.
    cpu_fallback_planes_at_last_log: u64,
    /// Snapshot of `VirtioGpuDriver::accel2d_gpu_cmds` at the last log reset.
    accel2d_gpu_cmds_at_last_log: u64,
    /// Snapshot of `VirtioGpuDriver::accel2d_cpu_cmds` at the last log reset.
    accel2d_cpu_cmds_at_last_log: u64,
}

/// Entry in the texture registry mapping client IDs to GPU resource IDs
struct TextureEntry {
    resource_id: u32,
    width: u32,
    height: u32,
}

#[derive(Clone, Copy)]
struct DisplayRpcDiag {
    last_entered_seq: u64,
    last_entered_op: u32,
    last_exited_seq: u64,
    last_exited_op: u32,
    last_exit_status: u8,
    last_enter_ns: u64,
    last_exit_ns: u64,
    last_req_id: u16,
    last_resp_port: u32,
    /// TID of the provider thread at the time of the last RPC entry.
    last_provider_tid: u64,
}

impl DisplayRpcDiag {
    const fn new() -> Self {
        Self {
            last_entered_seq: 0,
            last_entered_op: 0,
            last_exited_seq: 0,
            last_exited_op: 0,
            last_exit_status: 0,
            last_enter_ns: 0,
            last_exit_ns: 0,
            last_req_id: 0,
            last_resp_port: 0,
            last_provider_tid: 0,
        }
    }
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
            gpu_path_planes_at_last_log: 0,
            cpu_fallback_planes_at_last_log: 0,
            accel2d_gpu_cmds_at_last_log: 0,
            accel2d_cpu_cmds_at_last_log: 0,
        }
    }

    /// Log per-interval composition stats and reset interval counters.
    ///
    /// `gpu_planes_total` and `cpu_planes_total` are the *cumulative* totals
    /// from `VirtioGpuDriver`; this method computes the per-interval delta
    /// by subtracting the values stored from the previous call.
    fn log_and_reset(&mut self, gpu_planes_total: u64, cpu_planes_total: u64) {
        if self.frame_count > 0 {
            let gpu_interval = gpu_planes_total.saturating_sub(self.gpu_path_planes_at_last_log);
            let cpu_interval =
                cpu_planes_total.saturating_sub(self.cpu_fallback_planes_at_last_log);
            trace!(
                "display_virtio_gpu stats: frames={}, rects_in={}, transfers={}, flushes={}, union_flush={}, per_rect_flush={}, frame_pool={}, gpu_planes={}, cpu_planes={}",
                self.frame_count,
                self.total_rects_in,
                self.total_transfers,
                self.total_flushes,
                self.union_flush_count,
                self.per_rect_flush_count,
                self.using_frame_pool,
                gpu_interval,
                cpu_interval,
            );
        }
        let fp = self.using_frame_pool;
        let gpu_snapshot = gpu_planes_total;
        let cpu_snapshot = cpu_planes_total;
        *self = Self::new(fp);
        self.gpu_path_planes_at_last_log = gpu_snapshot;
        self.cpu_fallback_planes_at_last_log = cpu_snapshot;
    }

    /// Log per-interval ACCEL2D stats and reset interval counters.
    ///
    /// Called alongside `log_and_reset` when the ACCEL2D path is active so
    /// that GPU vs CPU command dispatch ratios are visible in trace logs.
    fn log_and_reset_accel2d(&mut self, gpu_cmds_total: u64, cpu_cmds_total: u64) {
        let gpu_interval = gpu_cmds_total.saturating_sub(self.accel2d_gpu_cmds_at_last_log);
        let cpu_interval = cpu_cmds_total.saturating_sub(self.accel2d_cpu_cmds_at_last_log);
        if gpu_interval > 0 || cpu_interval > 0 {
            trace!(
                "display_virtio_gpu accel2d stats: gpu_cmds={}, cpu_cmds={}",
                gpu_interval, cpu_interval,
            );
        }
        self.accel2d_gpu_cmds_at_last_log = gpu_cmds_total;
        self.accel2d_cpu_cmds_at_last_log = cpu_cmds_total;
    }
}

// ============================================================================
// VirtioGpuDriver — unified display driver state
//
// Mirrors the BootFbDriver pattern in display_bootfb so that all display
// drivers share one class of interface: import buffers, commit planes,
// release buffers — all through the standard VFS device call protocol.
// ============================================================================

const HANDLE_ROOT: u64 = 0;
const HANDLE_CARD: u64 = 1;
const S_IFDIR: u32 = 0o040000;
const S_IFCHR: u32 = 0o020000;

/// All mutable state for the virtio GPU display driver.
struct VirtioGpuDriver {
    gpu: VirtioGpu,
    disp_width: u32,
    disp_height: u32,
    disp_stride: u32,
    disp_format: u32,
    /// Pre-allocated pool of DMA-backed frame buffers created during driver
    /// initialization.  COMMIT operations round-robin through this pool as
    /// blit targets before the pixels are transferred to the GPU.
    frame_pool: alloc::vec::Vec<Buffer>,
    /// Round-robin index: next frame pool slot for the next COMMIT.
    next_buffer_idx: usize,
    /// Monotonically increasing counter incremented on every successful
    /// present.  Used with `Buffer::last_present_seq` to compute buffer age
    /// for MSG_ACQUIRE responses so clients can optimize damage regions.
    present_seq: u64,
    last_presented_idx: Option<usize>,
    /// Buffers imported from clients via DISPLAY_OP_IMPORT_BUFFER.  Each
    /// entry is a client-provided shared-memory region mapped read-only into
    /// this process.  Release currently retires the buffer ID without
    /// synchronously unmapping so the provider can always reply to clients.
    imported_buffers: alloc::collections::BTreeMap<BufferId, ImportedBuffer>,
    next_import_id: u32,
    /// Framebuffer FD used by the legacy MSG_BIND/MSG_PRESENT path.
    /// New clients should use DISPLAY_OP_IMPORT_BUFFER + DISPLAY_OP_COMMIT.
    current_fd: Option<u32>,
    /// Current GPU resource ID for the active scanout.
    current_res_id: u32,
    first_commit_logged: bool,
    cursor_commit_logged: bool,
    /// virtio-gpu resource ID reserved for the hardware cursor image, or 0 if
    /// no cursor resource has been allocated yet.
    cursor_resource_id: u32,
    /// Buffer ID of the cursor image that is currently loaded on the hardware
    /// cursor, or `None` if no cursor has been set.
    cursor_buffer_id: Option<BufferId>,
    /// Scaled hardware cursor hotspot for the active 64x64 cursor resource.
    cursor_hotspot: (u32, u32),
    /// Last requested hardware cursor hotspot position in screen coordinates.
    cursor_pos: (u32, u32),
    cursor_visible: bool,
    /// DMA-allocated pixel buffer for the hardware cursor image.
    ///
    /// Pixels from the Bloom-imported cursor buffer are *copied* here on every
    /// `DISPLAY_OP_SET_CURSOR` call so the GPU always DMA-reads from memory
    /// that is physically contiguous and device-accessible.  `0` means the
    /// allocation failed at init time (hardware cursor will be unavailable).
    cursor_dma_buf: u64,
    /// Physical address of `cursor_dma_buf`.
    cursor_dma_phys: u64,
    /// Dimensions of the cursor image that was last backed via `attach_backing`,
    /// so we can skip re-attaching when the size hasn't changed.
    cursor_attached_size: (u32, u32),
    /// Cumulative count of planes composed via the GPU fast-copy (opaque) path.
    /// Incremented each time a plane skips alpha arithmetic and uses a direct
    /// `copy_nonoverlapping` into the frame-pool buffer.
    gpu_path_planes: u64,
    /// Cumulative count of planes composed via the CPU fallback path.
    /// Incremented each time a plane requires per-pixel alpha blending or
    /// rounded-rectangle clipping that cannot be offloaded to the GPU.
    cpu_fallback_planes: u64,
    /// virgl 3D context ID used for GPU alpha blending (0 = not available).
    ///
    /// Initialized once at driver startup when the virtio-gpu device advertises
    /// the `VIRTIO_GPU_F_VIRGL` feature bit.  All virgl operations for alpha
    /// blending use this single context.
    virgl_ctx_id: u32,
    /// Virtual (driver-mapped) address of the DMA staging buffer used to hold
    /// source pixel data before uploading to the virgl source texture.  0 means
    /// the staging buffer was not successfully allocated.
    virgl_blend_staging_buf: u64,
    /// Physical address of `virgl_blend_staging_buf` (for GPU DMA).
    virgl_blend_staging_phys: u64,
    /// Byte capacity of `virgl_blend_staging_buf`
    /// (`disp_width * disp_height * 4`).
    virgl_blend_staging_size: usize,
    /// virgl 3D resource ID for the pre-created BGRA source texture that is
    /// backed by `virgl_blend_staging_buf` (0 = not created).
    virgl_src_res_id: u32,
    /// Cumulative count of ACCEL2D commands executed via the GPU (virgl) path.
    /// Incremented each time `ACCEL2D_CMD_COPY_RECT` or `ACCEL2D_CMD_ALPHA_BLIT`
    /// is dispatched to the virgl pipeline rather than the CPU fallback.
    accel2d_gpu_cmds: u64,
    /// Cumulative count of ACCEL2D commands executed via the CPU fallback path.
    /// Incremented when GPU is unavailable or the command has no GPU equivalent
    /// (e.g., `ACCEL2D_CMD_CLEAR_RECT`, `ACCEL2D_CMD_MASKED_BLIT`,
    /// `ACCEL2D_CMD_ROUNDED_CLIP_BLIT`, `ACCEL2D_CMD_STRETCH_BLIT`).
    accel2d_cpu_cmds: u64,
    display_rpc_seq: u64,
    rpc_diag: DisplayRpcDiag,
    total_imports: u64,
    total_commits: u64,
    failed_imports: u64,
    failed_commits: u64,
    last_damage_rect_count: u32,
    last_damage_area: u64,
}

/// Dispatch one VFS RPC request to the appropriate handler.
///
/// Follows the same pattern as `display_bootfb::vfs_provider::dispatch_vfs_rpc`
/// so both display drivers present an identical VFS device interface to bloom.
fn dispatch_vfs_rpc(driver: &mut VirtioGpuDriver, req: &ProviderRequest) -> ProviderResponse {
    match req.op {
        VfsRpcOp::Lookup => vfs_lookup(&req.payload),
        VfsRpcOp::Stat => vfs_stat(&req.payload),
        VfsRpcOp::Close | VfsRpcOp::SubscribeReady | VfsRpcOp::UnsubscribeReady => {
            ProviderResponse::ok_empty()
        }
        VfsRpcOp::DeviceCall => vfs_device_call(driver, req, &req.payload),
        _ => ProviderResponse::err(Errno::ENOSYS),
    }
}

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

fn display_rpc_payload_summary(op: u32, payload: &[u8]) -> (u32, u32, u32) {
    match op {
        DISPLAY_OP_IMPORT_BUFFER => {
            if payload.len() >= core::mem::size_of::<BufferHandle>() {
                let bh: BufferHandle =
                    unsafe { core::ptr::read_unaligned(payload.as_ptr() as *const _) };
                (bh.handle, bh.width, bh.height)
            } else {
                (0, 0, 0)
            }
        }
        DISPLAY_OP_RELEASE_BUFFER => {
            if payload.len() >= 4 {
                (u32::from_le_bytes(payload[..4].try_into().unwrap()), 0, 0)
            } else {
                (0, 0, 0)
            }
        }
        DISPLAY_OP_COMMIT => {
            let header_size = core::mem::size_of::<CommitRequest>();
            if payload.len() >= header_size {
                let req: CommitRequest =
                    unsafe { core::ptr::read_unaligned(payload.as_ptr() as *const _) };
                let plane_size = core::mem::size_of::<PlaneCommit>();
                let first_buffer = if req.commit_count > 0
                    && payload.len() >= header_size.saturating_add(plane_size)
                {
                    let plane: PlaneCommit = unsafe {
                        core::ptr::read_unaligned(payload[header_size..].as_ptr() as *const _)
                    };
                    plane.buffer_id.0
                } else {
                    0
                };
                (first_buffer, req.commit_count, req.damage_count)
            } else {
                (0, 0, 0)
            }
        }
        DISPLAY_OP_SET_CURSOR => {
            if payload.len() >= core::mem::size_of::<SetCursorRequest>() {
                let req: SetCursorRequest =
                    unsafe { core::ptr::read_unaligned(payload.as_ptr() as *const _) };
                (req.buffer_id.0, req.width, req.height)
            } else {
                (0, 0, 0)
            }
        }
        DISPLAY_OP_MOVE_CURSOR => {
            if payload.len() >= core::mem::size_of::<MoveCursorRequest>() {
                let req: MoveCursorRequest =
                    unsafe { core::ptr::read_unaligned(payload.as_ptr() as *const _) };
                (req.x.max(0) as u32, req.y.max(0) as u32, req.visible as u32)
            } else {
                (0, 0, 0)
            }
        }
        DISPLAY_OP_ACCEL2D => {
            if payload.len() >= core::mem::size_of::<Accel2dBatch>() {
                let batch: Accel2dBatch =
                    unsafe { core::ptr::read_unaligned(payload.as_ptr() as *const _) };
                (0, batch.cmd_count, 0)
            } else {
                (0, 0, 0)
            }
        }
        _ => (0, 0, 0),
    }
}

fn device_call_ret(response: &ProviderResponse) -> u32 {
    if response.status == 0 && response.payload.len() >= 4 {
        u32::from_le_bytes(response.payload[..4].try_into().unwrap_or([0; 4]))
    } else {
        0
    }
}

fn note_damage_snapshot(driver: &mut VirtioGpuDriver, rects: &[Rect]) {
    driver.last_damage_rect_count = rects.len() as u32;
    driver.last_damage_area = rects.iter().fold(0u64, |acc, r| acc.saturating_add(rect_area(*r)));
}

fn frame_pool_counts(driver: &VirtioGpuDriver) -> (usize, usize, usize) {
    let total = driver.frame_pool.len();
    let in_flight_idx = driver.last_presented_idx;
    let current_idx = driver.frame_pool.iter().position(|buf| buf.res_id == driver.current_res_id);
    let in_flight = usize::from(in_flight_idx.is_some());
    let acquired = usize::from(
        driver.current_fd.is_some() && current_idx.is_some() && current_idx != in_flight_idx,
    );
    let used = in_flight.saturating_add(acquired).min(total);
    (total.saturating_sub(used), acquired, in_flight)
}

fn maybe_log_display_watchdog(
    driver: &VirtioGpuDriver,
    last_watchdog_ns: &mut u64,
    vfs_pending_bytes: usize,
) {
    const WATCHDOG_INTERVAL_NS: u64 = 5_000_000_000;
    let now = stem::time::monotonic_ns();
    if now.saturating_sub(*last_watchdog_ns) < WATCHDOG_INTERVAL_NS {
        return;
    }
    *last_watchdog_ns = now;
    if driver.present_seq == 0 && driver.display_rpc_seq == 0 {
        return;
    }

    let (free, acquired, in_flight) = frame_pool_counts(driver);
    let current_frame = driver.last_presented_idx.unwrap_or(driver.next_buffer_idx);
    let open_rpc_ms = if driver.rpc_diag.last_entered_seq > driver.rpc_diag.last_exited_seq {
        now.saturating_sub(driver.rpc_diag.last_enter_ns) / 1_000_000
    } else {
        0
    };
    let last_rpc_ms =
        driver.rpc_diag.last_exit_ns.saturating_sub(driver.rpc_diag.last_enter_ns) / 1_000_000;
    // Classify the current RPC state for freeze diagnosis:
    //   open_rpc_ms > 0  → provider has entered but not exited (stuck inside provider)
    //   open_rpc_ms == 0 → all RPCs have completed (no stall in provider path)
    info!(
        "display_virtio_gpu: watchdog rpc_enter={}({}) rpc_exit={}({}) status={} corr={} req_id={} resp_port={} provider_tid={} open_rpc_ms={} last_rpc_ms={} frame={} present_seq={} pool free={} acquired={} in_flight={} imports={} commits={} failed_imports={} failed_commits={} damage_rects={} damage_area={} vfs_pending_bytes={}",
        driver.rpc_diag.last_entered_seq,
        display_op_name(driver.rpc_diag.last_entered_op),
        driver.rpc_diag.last_exited_seq,
        display_op_name(driver.rpc_diag.last_exited_op),
        driver.rpc_diag.last_exit_status,
        driver.rpc_diag.last_entered_seq,
        driver.rpc_diag.last_req_id,
        driver.rpc_diag.last_resp_port,
        driver.rpc_diag.last_provider_tid,
        open_rpc_ms,
        last_rpc_ms,
        current_frame,
        driver.present_seq,
        free,
        acquired,
        in_flight,
        driver.total_imports,
        driver.total_commits,
        driver.failed_imports,
        driver.failed_commits,
        driver.last_damage_rect_count,
        driver.last_damage_area,
        vfs_pending_bytes,
    );
}

fn alpha_over_argb(src: u32, dst: u32, plane_alpha: u8) -> u32 {
    let src_a = ((src >> 24) & 0xff) * plane_alpha as u32 / 255;
    if src_a == 0 {
        return dst;
    }
    if src_a == 255 {
        return 0xff00_0000 | (src & 0x00ff_ffff);
    }

    let inv = 255 - src_a;
    let sr = (src >> 16) & 0xff;
    let sg = (src >> 8) & 0xff;
    let sb = src & 0xff;
    let dr = (dst >> 16) & 0xff;
    let dg = (dst >> 8) & 0xff;
    let db = dst & 0xff;
    let r = (sr * src_a + dr * inv + 127) / 255;
    let g = (sg * src_a + dg * inv + 127) / 255;
    let b = (sb * src_a + db * inv + 127) / 255;
    0xff00_0000 | (r << 16) | (g << 8) | b
}

fn source_argb_for_blend(src: u32, format: PixelFormat) -> u32 {
    if format.has_alpha() { src } else { 0xff00_0000 | (src & 0x00ff_ffff) }
}

fn cursor_argb_to_host(src: u32) -> u32 {
    let a = src & 0xff00_0000;
    let r = (src >> 16) & 0xff;
    let g = (src >> 8) & 0xff;
    let b = src & 0xff;
    a | (b << 16) | (g << 8) | r
}

fn scale_alpha(alpha: u8, coverage: u8) -> u8 {
    ((alpha as u32 * coverage as u32 + 127) / 255) as u8
}

/// Returns `true` when a plane can take the GPU fast-copy (opaque) path.
///
/// A plane qualifies when it is fully opaque (plane-level alpha == 255, source
/// format has no alpha channel) and needs no rounded-rectangle clipping.  In
/// that case the composition is a plain row-by-row `memcpy` followed by the
/// GPU `TRANSFER_TO_HOST_2D` + `RESOURCE_FLUSH` commands — no per-pixel alpha
/// arithmetic is needed, so the CPU work is minimal.
///
/// `z_order` is intentionally not considered here: a fully opaque plane at any
/// z-level can use `memcpy` because it overwrites the destination pixels
/// completely.  Planes are sorted and drawn in z-order, so lower-z content is
/// already in the frame buffer when the opaque `memcpy` runs — the overwrite is
/// always correct.
///
/// Any other plane (transparent, alpha-blended, or clipped) falls back to the
/// CPU composition path which calls [`alpha_over_argb`] per pixel.
#[inline]
fn plane_can_use_gpu_path(plane: &PlaneCommit, src: &ImportedBuffer) -> bool {
    plane.alpha == 255 && !src.format.has_alpha() && plane.rounded_clip_radius().is_none()
}

fn rounded_clip_coverage(radius: u32, x: u32, y: u32, w: u32, h: u32) -> u8 {
    if radius == 0 {
        return 255;
    }
    let radius = radius.min(w / 2).min(h / 2);
    if radius == 0 {
        return 255;
    }
    if x >= w || y >= h {
        return 0;
    }
    if (x >= radius && x < w.saturating_sub(radius))
        || (y >= radius && y < h.saturating_sub(radius))
    {
        return 255;
    }

    let r = radius as i64 * 8;
    let left = r;
    let top = r;
    let right = w.saturating_sub(radius) as i64 * 8;
    let bottom = h.saturating_sub(radius) as i64 * 8;
    let mut inside = 0u32;

    for sy in 0..4i64 {
        let py = y as i64 * 8 + sy * 2 + 1;
        let cy = if py < top {
            top
        } else if py >= bottom {
            bottom
        } else {
            py
        };
        for sx in 0..4i64 {
            let px = x as i64 * 8 + sx * 2 + 1;
            let cx = if px < left {
                left
            } else if px >= right {
                right
            } else {
                px
            };
            let dx = px - cx;
            let dy = py - cy;
            if dx * dx + dy * dy <= r * r {
                inside += 1;
            }
        }
    }

    ((inside * 255 + 8) / 16) as u8
}

// ============================================================================
// Virgl GPU Alpha Blending Path
// ============================================================================

/// Initialize the virgl GPU alpha-blending subsystem.
///
/// Returns `(ctx_id, staging_buf_virt, staging_buf_phys, staging_size,
/// src_res_id)`.  All values are 0 / empty on failure.  The caller stores
/// these in [`VirtioGpuDriver`] fields prefixed `virgl_`.
///
/// ## Steps
/// 1. Creates a virgl 3D context (`VIRTIO_GPU_CMD_CTX_CREATE`).
/// 2. Attaches every frame-pool 2D resource to the context so they can be
///    used as render targets in `VIRGL_CCMD_BLIT`.
/// 3. Allocates a DMA-backed staging buffer sized for one full frame.
/// 4. Creates a `PIPE_TEXTURE_2D` / `PIPE_BIND_SAMPLER_VIEW` 3D source
///    texture backed by the staging buffer.
/// 5. Attaches the source texture to the context.
fn init_virgl_blend(
    gpu: &mut virtio_gpu::VirtioGpu,
    disp_width: u32,
    disp_height: u32,
    frame_pool: &[Buffer],
) -> (u32, u64, u64, usize, u32) {
    if !gpu.has_3d_feature() {
        return (0, 0, 0, 0, 0);
    }

    const BLEND_CTX_ID: u32 = 1;
    if let Err(e) = gpu.create_context(BLEND_CTX_ID, b"blend") {
        warn!(
            "display_virtio_gpu: virgl create_context failed: {}; GPU alpha blend unavailable",
            e
        );
        return (0, 0, 0, 0, 0);
    }

    // Attach all frame-pool 2D resources to the virgl context so they can
    // receive rendered output from VIRGL_CCMD_BLIT.
    for buf in frame_pool {
        if let Err(e) = gpu.ctx_attach_resource(BLEND_CTX_ID, buf.res_id) {
            warn!(
                "display_virtio_gpu: virgl ctx_attach frame_pool res={} failed: {}",
                buf.res_id, e
            );
        }
    }

    // Allocate a DMA-backed staging buffer (width × height × 4 bytes).
    let size = match (disp_width as usize)
        .checked_mul(disp_height as usize)
        .and_then(|n| n.checked_mul(4))
    {
        Some(s) if s > 0 => s,
        _ => {
            warn!("display_virtio_gpu: virgl staging size overflow; GPU alpha blend unavailable");
            return (0, 0, 0, 0, 0);
        }
    };
    let pages = (size + 4095) / 4096;
    let (buf_virt, buf_phys) = match gpu.alloc_dma(pages) {
        Ok(v) => v,
        Err(e) => {
            warn!(
                "display_virtio_gpu: virgl staging alloc failed: {}; GPU alpha blend unavailable",
                e
            );
            return (0, 0, 0, 0, 0);
        }
    };

    // Create the 3D source texture (BGRA, sampler-view, full display size).
    let src_res_id = gpu.alloc_resource_id();
    if let Err(e) = gpu.create_resource_3d(
        src_res_id,
        virtio_gpu::PIPE_TEXTURE_2D,
        virtio_gpu::VIRTIO_GPU_FORMAT_B8G8R8A8_UNORM,
        virtio_gpu::PIPE_BIND_SAMPLER_VIEW,
        disp_width,
        disp_height,
        1,
    ) {
        warn!(
            "display_virtio_gpu: virgl create_resource_3d failed: {}; GPU alpha blend unavailable",
            e
        );
        return (0, 0, 0, 0, 0);
    }
    if let Err(e) = gpu.attach_backing_3d(src_res_id, buf_phys, size) {
        warn!(
            "display_virtio_gpu: virgl attach_backing_3d failed: {}; GPU alpha blend unavailable",
            e
        );
        return (0, 0, 0, 0, 0);
    }
    if let Err(e) = gpu.ctx_attach_resource(BLEND_CTX_ID, src_res_id) {
        warn!(
            "display_virtio_gpu: virgl ctx_attach src_res failed: {}; GPU alpha blend unavailable",
            e
        );
        return (0, 0, 0, 0, 0);
    }

    info!(
        "display_virtio_gpu: virgl GPU alpha blend ready ctx_id={} src_res={} staging={}B ({}x{})",
        BLEND_CTX_ID, src_res_id, size, disp_width, disp_height
    );
    (BLEND_CTX_ID, buf_virt, buf_phys, size, src_res_id)
}

/// Composite `src_buffer` over the frame-pool buffer at `dst_rect` using the
/// virgl GPU alpha-blend pipeline.
///
/// ## Algorithm
/// 1. **Staging copy**: The source region is copied into the pre-allocated DMA
///    staging buffer (at offset 0, stride = `copy_w * 4`).  If `global_alpha`
///    is less than 255 the source alpha channel is pre-multiplied by it so
///    that the GPU BLIT produces the same result as the CPU path.
/// 2. **Texture upload**: `VIRTIO_GPU_CMD_TRANSFER_TO_HOST_3D` uploads the
///    staged pixels into the virgl source texture resource.
/// 3. **GPU BLIT**: `VIRGL_CCMD_BLIT` with `alpha_blend=1` composites the
///    source texture over the frame-pool 2D resource using
///    `GL_SRC_ALPHA / GL_ONE_MINUS_SRC_ALPHA` — the Porter-Duff "over"
///    operator.  The blend is performed entirely in GPU hardware with no
///    per-pixel CPU read-modify-write loop on the destination.
///
/// Callers must call `transfer_to_host_with_stride` for any OPAQUE planes
/// BEFORE calling this function so that the GPU resource already contains
/// the correct background layer for the blend to composite on top of.
///
/// Returns `Ok(())` on success.  On failure the caller should fall back to
/// the CPU blend path.
fn gpu_alpha_blit(
    driver: &mut VirtioGpuDriver,
    idx: usize,
    src_buffer: &ImportedBuffer,
    src_x: usize,
    src_y: usize,
    copy_w: usize,
    copy_h: usize,
    dst_x: usize,
    dst_y: usize,
    global_alpha: u8,
) -> abi::errors::SysResult<()> {
    if driver.virgl_ctx_id == 0 || driver.virgl_src_res_id == 0 {
        return Err(abi::errors::Errno::ENOSYS);
    }
    if copy_w == 0 || copy_h == 0 {
        return Ok(());
    }

    // Verify the source region fits within the staging buffer.
    let needed = match copy_w.checked_mul(copy_h).and_then(|n| n.checked_mul(4)) {
        Some(n) => n,
        None => return Err(abi::errors::Errno::EINVAL),
    };
    if needed > driver.virgl_blend_staging_size {
        return Err(abi::errors::Errno::ENOSYS);
    }

    let staging_ptr = driver.virgl_blend_staging_buf as *mut u8;
    let staging_stride = (copy_w as u32) * 4;
    let bpp = 4usize;

    // ── Pass 1: CPU copy source pixels → staging buffer ─────────────────────
    //
    // This is a sequential write-only pass (no destination read-modify-write).
    // global_alpha is pre-multiplied into the source alpha channel so that the
    // GPU BLIT (which uses raw src_alpha) produces the same blended output as
    // the CPU path's `alpha_over_argb(src, dst, plane_alpha)`.
    unsafe {
        for row in 0..copy_h {
            for col in 0..copy_w {
                let src_off = (src_y + row).saturating_mul(src_buffer.stride as usize)
                    + (src_x + col).saturating_mul(bpp);
                let stg_off = row.saturating_mul(staging_stride as usize) + col.saturating_mul(bpp);
                if src_off + bpp > src_buffer.size {
                    // Out-of-bounds source pixel — write transparent black.
                    core::ptr::write_unaligned(staging_ptr.add(stg_off) as *mut u32, 0u32);
                    continue;
                }
                // Normalise to BGRA (fill alpha=0xff for BGRX sources).
                let mut px = source_argb_for_blend(
                    core::ptr::read_unaligned(src_buffer.ptr.add(src_off) as *const u32),
                    src_buffer.format,
                );
                // Pre-multiply global_alpha into the source alpha channel so
                // that the GPU blend is equivalent to the CPU fallback.
                if global_alpha < 255 {
                    let a = ((px >> 24) & 0xff) * global_alpha as u32 / 255;
                    px = (px & 0x00ff_ffff) | (a << 24);
                }
                core::ptr::write_unaligned(staging_ptr.add(stg_off) as *mut u32, px);
            }
        }
    }

    // ── Pass 2: Upload staged pixels to the virgl source texture ─────────────
    driver
        .gpu
        .transfer_to_host_3d(
            driver.virgl_ctx_id,
            driver.virgl_src_res_id,
            copy_w as u32,
            copy_h as u32,
            0, // offset from start of backing memory
            staging_stride,
        )
        .map_err(|_| abi::errors::Errno::EIO)?;

    // ── Pass 3: GPU BLIT with alpha blending ─────────────────────────────────
    //
    // Composites the source texture (0,0,copy_w,copy_h) over the frame-pool
    // 2D resource at (dst_x,dst_y) using GL_SRC_ALPHA/GL_ONE_MINUS_SRC_ALPHA.
    let dst_res_id = driver.frame_pool[idx].res_id;
    let blit_cmd = virtio_gpu::virgl_encode_blit(
        driver.virgl_src_res_id,
        dst_res_id,
        0,
        0,
        copy_w as u32,
        copy_h as u32,
        dst_x as u32,
        dst_y as u32,
        copy_w as u32,
        copy_h as u32,
        virtio_gpu::VIRTIO_GPU_FORMAT_B8G8R8A8_UNORM,
        virtio_gpu::VIRTIO_GPU_FORMAT_B8G8R8X8_UNORM,
        true, // alpha_blend
    );
    driver.gpu.submit_3d(driver.virgl_ctx_id, &blit_cmd).map_err(|_| abi::errors::Errno::EIO)?;

    Ok(())
}

fn vfs_lookup(payload: &[u8]) -> ProviderResponse {
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
    stem::trace!("DISP: vfs_lookup path='{}'", path);
    let handle: u64 = match path.trim_matches('/') {
        "" => HANDLE_CARD,
        "card0" => HANDLE_CARD,
        _ => return ProviderResponse::err(Errno::ENOENT),
    };
    ProviderResponse::ok_u64(handle)
}

fn vfs_stat(payload: &[u8]) -> ProviderResponse {
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

fn vfs_device_call(
    driver: &mut VirtioGpuDriver,
    req: &ProviderRequest,
    payload: &[u8],
) -> ProviderResponse {
    use abi::device::{DeviceCall, DeviceKind};

    let dc_size = core::mem::size_of::<DeviceCall>();
    if payload.len() < 8 + dc_size {
        return ProviderResponse::err(Errno::EINVAL);
    }

    let handle = u64::from_le_bytes(payload[..8].try_into().unwrap());
    if handle != HANDLE_CARD {
        return ProviderResponse::err(Errno::EINVAL);
    }

    let call: DeviceCall =
        unsafe { core::ptr::read_unaligned(payload[8..8 + dc_size].as_ptr() as *const _) };
    let call_payload = &payload[8 + dc_size..];

    if call.kind != DeviceKind::Display {
        return ProviderResponse::err(Errno::ENOSYS);
    }

    driver.display_rpc_seq = driver.display_rpc_seq.saturating_add(1);
    let seq = driver.display_rpc_seq;
    let (summary0, summary1, summary2) = display_rpc_payload_summary(call.op, call_payload);
    let provider_tid = stem::syscall::get_tid().unwrap_or(0);
    let enter_ns = stem::time::monotonic_ns();
    driver.rpc_diag.last_entered_seq = seq;
    driver.rpc_diag.last_entered_op = call.op;
    driver.rpc_diag.last_enter_ns = enter_ns;
    driver.rpc_diag.last_req_id = req.req_id;
    driver.rpc_diag.last_resp_port = req.resp_port;
    driver.rpc_diag.last_provider_tid = provider_tid;
    trace!(
        "display_virtio_gpu: rpc.enter corr={} op={} req_id={} resp_port={} pid={} provider_tid={} arg0={} arg1={} arg2={}",
        seq,
        display_op_name(call.op),
        req.req_id,
        req.resp_port,
        stem::syscall::getpid(),
        provider_tid,
        summary0,
        summary1,
        summary2,
    );

    let response = dispatch_display_device_call(driver, call.op, call_payload);
    let ret = device_call_ret(&response);
    let ok = response.status == 0;
    match call.op {
        DISPLAY_OP_IMPORT_BUFFER => {
            if ok {
                driver.total_imports = driver.total_imports.saturating_add(1);
            } else {
                driver.failed_imports = driver.failed_imports.saturating_add(1);
            }
        }
        DISPLAY_OP_COMMIT | DISPLAY_OP_ACCEL2D => {
            if ok {
                driver.total_commits = driver.total_commits.saturating_add(1);
            } else {
                driver.failed_commits = driver.failed_commits.saturating_add(1);
            }
        }
        _ => {}
    }
    let exit_ns = stem::time::monotonic_ns();
    let duration_ms = exit_ns.saturating_sub(enter_ns) / 1_000_000;
    driver.rpc_diag.last_exited_seq = seq;
    driver.rpc_diag.last_exited_op = call.op;
    driver.rpc_diag.last_exit_status = response.status;
    driver.rpc_diag.last_exit_ns = exit_ns;
    trace!(
        "display_virtio_gpu: rpc.exit corr={} op={} status={} ret={} duration_ms={} present_seq={} imports={} commits={} failed_imports={} failed_commits={}",
        seq,
        display_op_name(call.op),
        response.status,
        ret,
        duration_ms,
        driver.present_seq,
        driver.total_imports,
        driver.total_commits,
        driver.failed_imports,
        driver.failed_commits,
    );
    // Per-RPC watchdog: warn on high-latency or stalled operations.
    // Thresholds are conservative to surface infrastructure stalls (not heavy rendering).
    if duration_ms >= 1000 {
        let (pool_free, pool_acquired, pool_in_flight) = frame_pool_counts(driver);
        warn!(
            "display_virtio_gpu: rpc.stall corr={} op={} duration_ms={} status={} provider_tid={} req_id={} resp_port={} present_seq={} pool_free={} pool_acquired={} pool_in_flight={} imports={} commits={} failed_imports={} failed_commits={}",
            seq,
            display_op_name(call.op),
            duration_ms,
            response.status,
            provider_tid,
            req.req_id,
            req.resp_port,
            driver.present_seq,
            pool_free,
            pool_acquired,
            pool_in_flight,
            driver.total_imports,
            driver.total_commits,
            driver.failed_imports,
            driver.failed_commits,
        );
    } else if duration_ms >= 200 {
        warn!(
            "display_virtio_gpu: rpc.slow corr={} op={} duration_ms={} status={} provider_tid={} req_id={} resp_port={}",
            seq,
            display_op_name(call.op),
            duration_ms,
            response.status,
            provider_tid,
            req.req_id,
            req.resp_port,
        );
    } else if duration_ms >= 50 {
        info!(
            "display_virtio_gpu: rpc.latency corr={} op={} duration_ms={} status={} provider_tid={} req_id={} resp_port={}",
            seq,
            display_op_name(call.op),
            duration_ms,
            response.status,
            provider_tid,
            req.req_id,
            req.resp_port,
        );
    }
    response
}

fn dispatch_display_device_call(
    driver: &mut VirtioGpuDriver,
    op: u32,
    call_payload: &[u8],
) -> ProviderResponse {
    match op {
        DISPLAY_OP_GET_INFO => {
            let _ = refresh_display_mode(driver);
            stem::debug!("display.phase=device_call_enter op=GET_INFO");
            stem::trace!("DISP: DISPLAY_OP_GET_INFO requested");
            let mut caps = DisplayCaps::ATOMIC
                | DisplayCaps::DMABUF_IMPORT
                | DisplayCaps::GPU_BLIT
                | DisplayCaps::DIRECT_SCANOUT
                | DisplayCaps::PARTIAL_FLUSH
                | DisplayCaps::RESOURCE_CACHE
                | DisplayCaps::ACCEL2D_CLEAR
                | DisplayCaps::ACCEL2D_COPY
                | DisplayCaps::ACCEL2D_STRETCH
                | DisplayCaps::ACCEL2D_ALPHA_BLIT
                | DisplayCaps::ACCEL2D_MASKED_BLIT
                | DisplayCaps::ACCEL2D_ROUNDED_CLIP_BLIT
                | DisplayCaps::ACCEL2D_FLUSH_DAMAGE;
            // Only advertise hardware cursor if the cursor queue is available
            // AND the DMA pixel buffer was successfully allocated at init time.
            if driver.gpu.has_cursorq() && driver.cursor_dma_buf != 0 && driver.cursor_dma_phys != 0
            {
                caps |= DisplayCaps::HARDWARE_CURSOR;
            }
            // Advertise GPU_ALPHA_BLEND when the virgl 3D context and staging
            // buffer are ready.  Bloom (and other compositors) can call
            // `supports_gpu_alpha_blend()` to branch on this capability and
            // avoid per-pixel CPU blend loops on the client side.
            if driver.virgl_ctx_id != 0 && driver.virgl_src_res_id != 0 {
                caps |= DisplayCaps::GPU_ALPHA_BLEND;
                // When virgl is available, ACCEL2D_CMD_COPY_RECT and
                // ACCEL2D_CMD_ALPHA_BLIT are dispatched to the GPU pipeline
                // (virgl BLIT command) rather than the CPU fallback path.
                caps |= DisplayCaps::ACCEL2D_GPU;
            }
            let info = DisplayInfo {
                card_id: 0,
                preferred_mode: DisplayMode {
                    width: driver.disp_width,
                    height: driver.disp_height,
                    refresh_mhz: DEFAULT_REFRESH_MHZ,
                },
                plane_count: 1,
                max_buffers: 32,
                supported_formats: (1 << (PixelFormat::Bgra8888 as u8))
                    | (1 << (PixelFormat::Bgrx8888 as u8)),
                // This provider replies to DISPLAY_OP_COMMIT synchronously.
                // Advertising VBLANK would make clients sleep inside the VFS
                // RPC response path instead of returning to their event loops.
                caps,
            };
            stem::trace!("DISP: Returning dimensions {}x{}", driver.disp_width, driver.disp_height);
            let out_bytes = unsafe {
                core::slice::from_raw_parts(
                    &info as *const _ as *const u8,
                    core::mem::size_of::<DisplayInfo>(),
                )
            };
            stem::debug!(
                "display.phase=device_call_exit op=GET_INFO result=ok width={} height={}",
                driver.disp_width,
                driver.disp_height
            );
            ProviderResponse::ok_device_call(0, out_bytes)
        }
        DISPLAY_OP_IMPORT_BUFFER => {
            if call_payload.len() < core::mem::size_of::<BufferHandle>() {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let bh: BufferHandle =
                unsafe { core::ptr::read_unaligned(call_payload.as_ptr() as *const _) };
            stem::debug!(
                "display.phase=import_buffer_begin memfd={} size={}x{}",
                bh.handle,
                bh.width,
                bh.height
            );
            stem::trace!(
                "DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd={}, size={}x{}",
                bh.handle,
                bh.width,
                bh.height
            );
            if bh.modifier != 0 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            if !matches!(bh.format, PixelFormat::Bgra8888 | PixelFormat::Bgrx8888) {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let bpp = bh.format.bytes_per_pixel();
            let min_stride = match (bh.width as usize).checked_mul(bpp) {
                Some(v) => v,
                None => return ProviderResponse::err(Errno::EINVAL),
            };
            if bh.width == 0 || bh.height == 0 || (bh.stride as usize) < min_stride {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let size = match (bh.height as usize).checked_mul(bh.stride as usize) {
                Some(v) => v,
                None => return ProviderResponse::err(Errno::EINVAL),
            };
            let req = abi::vm::VmMapReq {
                addr_hint: 0,
                len: size,
                prot: abi::vm::VmProt::READ | abi::vm::VmProt::USER,
                flags: abi::vm::VmMapFlags::SHARED,
                backing: abi::vm::VmBacking::File { thing: bh.handle, offset: bh.offset },
            };
            match stem::syscall::vm_map(&req) {
                Ok(map_resp) => {
                    let id = BufferId(driver.next_import_id);
                    driver.next_import_id = driver.next_import_id.saturating_add(1);
                    stem::debug!("DISP: imported buffer as ID={}", id.0);
                    driver.imported_buffers.insert(
                        id,
                        ImportedBuffer {
                            fd: bh.handle,
                            ptr: map_resp.addr as *mut u8,
                            size,
                            width: bh.width,
                            height: bh.height,
                            stride: bh.stride,
                            format: bh.format,
                        },
                    );
                    stem::debug!("display.phase=import_buffer_done id={}", id.0);
                    ProviderResponse::ok_device_call(id.0, &id.0.to_le_bytes())
                }
                Err(e) => {
                    stem::error!("DISP: Failed to vm_map imported buffer: {:?}", e);
                    stem::debug!("display.phase=device_call_exit op=IMPORT_BUFFER result=err");
                    ProviderResponse::err(Errno::ENOMEM)
                }
            }
        }
        DISPLAY_OP_RELEASE_BUFFER => {
            if call_payload.len() < 4 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let id = BufferId(u32::from_le_bytes(call_payload[..4].try_into().unwrap()));
            if let Some(buf) = driver.imported_buffers.remove(&id) {
                stem::trace!(
                    "DISP: DISPLAY_OP_RELEASE_BUFFER requested: id={} size={}",
                    id.0,
                    buf.size
                );
                if let Err(e) = stem::syscall::vm_unmap(buf.ptr as usize, buf.size) {
                    stem::warn!(
                        "DISP: failed to unmap released buffer id={} size={}: {:?}",
                        id.0,
                        buf.size,
                        e
                    );
                }
                if let Err(e) = stem::syscall::vfs::vfs_close(buf.fd) {
                    stem::warn!(
                        "DISP: failed to close released buffer id={} fd={}: {:?}",
                        id.0,
                        buf.fd,
                        e
                    );
                }
                ProviderResponse::ok_device_call(0, &[])
            } else {
                stem::trace!("DISP: DISPLAY_OP_RELEASE_BUFFER requested for unknown id={}", id.0);
                ProviderResponse::err(Errno::ENOENT)
            }
        }
        DISPLAY_OP_COMMIT => {
            let _ = refresh_display_mode(driver);
            let header_size = core::mem::size_of::<CommitRequest>();
            if call_payload.len() < header_size {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let req: CommitRequest =
                unsafe { core::ptr::read_unaligned(call_payload.as_ptr() as *const _) };

            let plane_size = core::mem::size_of::<PlaneCommit>();
            let plane_count = req.commit_count as usize;
            stem::debug!("display.phase=commit_begin planes={}", plane_count);
            stem::trace!("DISP: DISPLAY_OP_COMMIT requested: planes={}", plane_count);
            let needed = header_size.saturating_add(plane_count.saturating_mul(plane_size));
            if plane_count > 0 && call_payload.len() < needed {
                return ProviderResponse::err(Errno::EINVAL);
            }

            let mut planes = alloc::vec::Vec::with_capacity(plane_count);
            if plane_count > 0 {
                let raw_planes = &call_payload[header_size..needed];
                for i in 0..plane_count {
                    let off = i * plane_size;
                    let plane: PlaneCommit = unsafe {
                        core::ptr::read_unaligned(
                            raw_planes[off..off + plane_size].as_ptr() as *const _
                        )
                    };
                    planes.push(plane);
                }
            }
            let rect_size = core::mem::size_of::<abi::display_protocol::Rect>();
            let damage_count = req.damage_count as usize;
            let damage_offset = needed;
            let damage_needed =
                damage_offset.saturating_add(damage_count.saturating_mul(rect_size));
            if damage_count > 0 && call_payload.len() < damage_needed {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let mut damage_rects = alloc::vec::Vec::with_capacity(damage_count.max(1));
            if damage_count > 0 {
                let raw_damage = &call_payload[damage_offset..damage_needed];
                for i in 0..damage_count {
                    let off = i * rect_size;
                    let rect: abi::display_protocol::Rect = unsafe {
                        core::ptr::read_unaligned(
                            raw_damage[off..off + rect_size].as_ptr() as *const _
                        )
                    };
                    let rect = rect_clamp_to_bounds(
                        Rect { x: rect.x, y: rect.y, w: rect.w, h: rect.h },
                        driver.disp_width,
                        driver.disp_height,
                    );
                    if !rect_is_empty(rect) {
                        damage_rects.push(rect);
                    }
                }
            }
            if damage_rects.is_empty() {
                damage_rects.push(Rect { x: 0, y: 0, w: driver.disp_width, h: driver.disp_height });
            }
            if driver.last_presented_idx.is_none() {
                damage_rects.clear();
                damage_rects.push(Rect { x: 0, y: 0, w: driver.disp_width, h: driver.disp_height });
            }
            note_damage_snapshot(driver, &damage_rects);
            planes.sort_unstable_by_key(|plane| plane.z_order);

            if !planes.is_empty() {
                let full_damage = damage_rects.len() == 1
                    && damage_rects[0].x == 0
                    && damage_rects[0].y == 0
                    && damage_rects[0].w >= driver.disp_width
                    && damage_rects[0].h >= driver.disp_height;
                // Conservative buffer selection: for full-damage commits advance
                // to the next slot in the pool, preferring a buffer that is not
                // currently being scanned out.  The loop exits early as soon as
                // a suitable candidate is found; if no such candidate exists
                // (e.g. single-buffer pool where every slot is the last-presented
                // one) `chosen` stays at `start`, which is always safe because
                // `transfer_to_host` and `flush_resource` are synchronous — the
                // GPU has finished reading the resource before those calls return.
                let idx = if full_damage {
                    let pool_len = driver.frame_pool.len();
                    let start = driver.next_buffer_idx;
                    let mut chosen = start;
                    for i in 0..pool_len {
                        let candidate = (start + i) % pool_len;
                        // Prefer a buffer that is not the currently presented one
                        // so we avoid touching a resource the display may still
                        // be scanning out from the previous frame.
                        if Some(candidate) != driver.last_presented_idx {
                            chosen = candidate;
                            break;
                        }
                        // If all candidates are the last-presented slot (single-
                        // buffer pool), `chosen` remains `start` after the loop.
                    }
                    driver.next_buffer_idx = (chosen + 1) % pool_len;
                    chosen
                } else {
                    driver.last_presented_idx.unwrap_or_else(|| {
                        let idx = driver.next_buffer_idx;
                        driver.next_buffer_idx =
                            (driver.next_buffer_idx + 1) % driver.frame_pool.len();
                        idx
                    })
                };

                let bpp = if driver.disp_width > 0 {
                    (driver.disp_stride / driver.disp_width).max(1) as usize
                } else {
                    4usize
                };

                let mut damage: Option<Rect> = None;

                // Blit each plane's imported buffer into the DMA frame pool buffer.
                let mut first_copy_sample: Option<(u32, u32, u32, u32, u32)> = None;
                let mut saw_cursor_plane = false;
                let mut cursor_copy_sample: Option<(u32, u32, u32, u32, u32, u32)> = None;
                // Planes deferred to GPU alpha blend pass: (buffer_id, src_x, src_y,
                // copy_w, copy_h, dst_x, dst_y, global_alpha).
                let mut gpu_blend_deferred: alloc::vec::Vec<(
                    BufferId,
                    usize,
                    usize,
                    usize,
                    usize,
                    usize,
                    usize,
                    u8,
                )> = alloc::vec::Vec::new();
                for plane in &planes {
                    let src = match driver.imported_buffers.get(&plane.buffer_id) {
                        Some(s) => s,
                        None => continue,
                    };
                    if plane.z_order == i32::MAX {
                        saw_cursor_plane = true;
                    }

                    let src_x = plane.src_rect.x.min(src.width) as usize;
                    let src_y = plane.src_rect.y.min(src.height) as usize;
                    let src_w =
                        plane.src_rect.w.min(src.width.saturating_sub(plane.src_rect.x)) as usize;
                    let src_h =
                        plane.src_rect.h.min(src.height.saturating_sub(plane.src_rect.y)) as usize;
                    let dst_x = plane.dest_rect.x.min(driver.disp_width) as usize;
                    let dst_y = plane.dest_rect.y.min(driver.disp_height) as usize;
                    let dst_w =
                        plane.dest_rect.w.min(driver.disp_width.saturating_sub(plane.dest_rect.x))
                            as usize;
                    let dst_h =
                        plane.dest_rect.h.min(driver.disp_height.saturating_sub(plane.dest_rect.y))
                            as usize;
                    let copy_w = src_w.min(dst_w);
                    let copy_h = src_h.min(dst_h);
                    if copy_w == 0 || copy_h == 0 {
                        continue;
                    }
                    let plane_rect = Rect {
                        x: dst_x as u32,
                        y: dst_y as u32,
                        w: copy_w as u32,
                        h: copy_h as u32,
                    };
                    let target_ptr = driver.frame_pool[idx].ptr;
                    let target_size = driver.frame_pool[idx].size;
                    let clip_radius = plane.rounded_clip_radius().map(u32::from).unwrap_or(0);
                    // Determine composition path using `plane_can_use_gpu_path()`:
                    // GPU fast-copy path: fully opaque, no alpha channel, no
                    // rounded clipping → plain memcpy into the frame-pool buffer
                    // which is then uploaded to the GPU via TRANSFER_TO_HOST_2D.
                    // CPU fallback path: anything requiring per-pixel alpha
                    // arithmetic or rounded-rectangle masking.
                    //
                    // The counters below are incremented once per *plane* per
                    // commit.  They are cumulative across the driver's lifetime
                    // (not reset between frames), so they track how many plane
                    // instances in total have taken each path.  A plane in the
                    // commit list is counted once regardless of how many dirty
                    // rectangles overlap it.
                    let use_gpu_path = plane_can_use_gpu_path(plane, src);
                    let should_blend = !use_gpu_path;
                    // Use virgl GPU alpha blend when:
                    //   • virgl context + staging texture are ready
                    //   • plane requires blending (not opaque fast-copy)
                    //   • no rounded-corner clip (clip requires per-pixel CPU coverage)
                    // Virgl blend planes are deferred until after transfer_to_host_2d
                    // so that the GPU resource already contains the composited opaque
                    // layer before the blend pass runs.
                    let use_virgl_blend = should_blend
                        && driver.virgl_ctx_id != 0
                        && driver.virgl_src_res_id != 0
                        && clip_radius == 0;
                    if use_gpu_path || use_virgl_blend {
                        driver.gpu_path_planes = driver.gpu_path_planes.saturating_add(1);
                    } else {
                        driver.cpu_fallback_planes = driver.cpu_fallback_planes.saturating_add(1);
                    }
                    if use_virgl_blend {
                        // Defer to second (GPU) pass.  Still add the plane rect to
                        // `damage` so that transfer_to_host_2d uploads the background
                        // layer and flush_resource covers this plane's area.
                        gpu_blend_deferred.push((
                            plane.buffer_id,
                            src_x,
                            src_y,
                            copy_w,
                            copy_h,
                            dst_x,
                            dst_y,
                            plane.alpha,
                        ));
                        damage = Some(match damage {
                            Some(old) => rect_union(old, plane_rect),
                            None => plane_rect,
                        });
                        if first_copy_sample.is_none() {
                            // Record a sample for the first-commit log.
                            let src_off = src_y.saturating_mul(src.stride as usize)
                                + src_x.saturating_mul(bpp);
                            let dst_off = dst_y.saturating_mul(driver.disp_stride as usize)
                                + dst_x.saturating_mul(bpp);
                            let target_ptr = driver.frame_pool[idx].ptr;
                            let sp = if src_off + 4 <= src.size {
                                unsafe {
                                    core::ptr::read_unaligned(src.ptr.add(src_off) as *const u32)
                                }
                            } else {
                                0
                            };
                            let dp = if dst_off + 4 <= driver.frame_pool[idx].size {
                                unsafe {
                                    core::ptr::read_unaligned(target_ptr.add(dst_off) as *const u32)
                                }
                            } else {
                                0
                            };
                            first_copy_sample =
                                Some((plane.buffer_id.0, sp, dp, copy_w as u32, copy_h as u32));
                        }
                        continue;
                    }
                    for dirty in &damage_rects {
                        let Some(rect) = rect_intersect(plane_rect, *dirty) else {
                            continue;
                        };
                        let rel_x = rect.x.saturating_sub(plane_rect.x) as usize;
                        let rel_y = rect.y.saturating_sub(plane_rect.y) as usize;
                        let clip_src_x = src_x.saturating_add(rel_x);
                        let clip_src_y = src_y.saturating_add(rel_y);
                        let clip_dst_x = rect.x as usize;
                        let clip_dst_y = rect.y as usize;
                        let Some((clip_w, clip_h)) = bounded_copy_extent(
                            src,
                            target_size,
                            driver.disp_stride as usize,
                            bpp,
                            clip_src_x,
                            clip_src_y,
                            clip_dst_x,
                            clip_dst_y,
                            rect.w as usize,
                            rect.h as usize,
                        ) else {
                            continue;
                        };

                        if should_blend && bpp == 4 {
                            for row in 0..clip_h {
                                for col in 0..clip_w {
                                    let coverage = rounded_clip_coverage(
                                        clip_radius,
                                        rel_x.saturating_add(col) as u32,
                                        rel_y.saturating_add(row) as u32,
                                        plane.dest_rect.w,
                                        plane.dest_rect.h,
                                    );
                                    if coverage == 0 {
                                        continue;
                                    }
                                    unsafe {
                                        let src_ptr = src.ptr.add(
                                            (clip_src_y + row).saturating_mul(src.stride as usize)
                                                + (clip_src_x + col).saturating_mul(bpp),
                                        );
                                        let dst_ptr = target_ptr.add(
                                            (clip_dst_y + row)
                                                .saturating_mul(driver.disp_stride as usize)
                                                + (clip_dst_x + col).saturating_mul(bpp),
                                        );
                                        let src_px = source_argb_for_blend(
                                            core::ptr::read_unaligned(src_ptr as *const u32),
                                            src.format,
                                        );
                                        let dst_px =
                                            core::ptr::read_unaligned(dst_ptr as *const u32);
                                        let out_px = alpha_over_argb(
                                            src_px,
                                            dst_px,
                                            scale_alpha(plane.alpha, coverage),
                                        );
                                        core::ptr::write_unaligned(dst_ptr as *mut u32, out_px);
                                        if !driver.cursor_commit_logged
                                            && cursor_copy_sample.is_none()
                                            && plane.z_order == i32::MAX
                                            && ((src_px >> 24) & 0xff) != 0
                                        {
                                            cursor_copy_sample = Some((
                                                plane.buffer_id.0,
                                                src_px,
                                                dst_px,
                                                out_px,
                                                (clip_dst_x + col) as u32,
                                                (clip_dst_y + row) as u32,
                                            ));
                                        }
                                    }
                                }
                            }
                        } else {
                            let row_bytes = clip_w.saturating_mul(bpp);
                            for row in 0..clip_h {
                                unsafe {
                                    core::ptr::copy_nonoverlapping(
                                        src.ptr.add(
                                            (clip_src_y + row).saturating_mul(src.stride as usize)
                                                + clip_src_x.saturating_mul(bpp),
                                        ),
                                        target_ptr.add(
                                            (clip_dst_y + row)
                                                .saturating_mul(driver.disp_stride as usize)
                                                + clip_dst_x.saturating_mul(bpp),
                                        ),
                                        row_bytes,
                                    );
                                }
                            }
                        }

                        if first_copy_sample.is_none() {
                            let src_off = clip_src_y.saturating_mul(src.stride as usize)
                                + clip_src_x.saturating_mul(bpp);
                            let dst_off = clip_dst_y.saturating_mul(driver.disp_stride as usize)
                                + clip_dst_x.saturating_mul(bpp);
                            let src_px = unsafe {
                                core::ptr::read_unaligned(src.ptr.add(src_off) as *const u32)
                            };
                            let dst_px = unsafe {
                                core::ptr::read_unaligned(target_ptr.add(dst_off) as *const u32)
                            };
                            first_copy_sample = Some((
                                plane.buffer_id.0,
                                src_px,
                                dst_px,
                                clip_w as u32,
                                clip_h as u32,
                            ));
                        }

                        damage = Some(match damage {
                            Some(old) => rect_union(old, rect),
                            None => rect,
                        });
                    }
                }

                // Transfer blitted pixels to the GPU and flush to the display.
                if let Some(dmg) = damage {
                    let res_id = driver.frame_pool[idx].res_id;
                    let mut command_ok = true;
                    stem::debug!(
                        "display.phase=transfer_to_host_begin seq={} res_id={} damage={}x{}+{},{}",
                        driver.present_seq.saturating_add(1),
                        res_id,
                        dmg.w,
                        dmg.h,
                        dmg.x,
                        dmg.y
                    );
                    stem::trace!(
                        "DISP: COMMIT transfer begin seq={} res_id={} damage={}x{}+{},{}",
                        driver.present_seq.saturating_add(1),
                        res_id,
                        dmg.w,
                        dmg.h,
                        dmg.x,
                        dmg.y
                    );
                    if let Err(e) =
                        driver.gpu.transfer_to_host_with_stride(res_id, dmg, driver.disp_stride)
                    {
                        stem::error!("DISP: transfer_to_host failed: {}", e);
                        command_ok = false;
                    }
                    stem::debug!(
                        "display.phase=transfer_to_host_done seq={} res_id={}",
                        driver.present_seq.saturating_add(1),
                        res_id
                    );

                    // ── GPU alpha blend pass ─────────────────────────────────────
                    // For planes that were deferred above, composite each one onto
                    // the GPU resource (which now contains the opaque background
                    // layer uploaded by transfer_to_host_with_stride) using the
                    // virgl BLIT command with alpha_blend=1.
                    // CPU fallback on virgl error ensures no frame is silently lost.
                    for &(buf_id, sx, sy, cw, ch, dx, dy, ga) in &gpu_blend_deferred {
                        // Re-borrow the immutable source buffer (driver is &mut but
                        // imported_buffers is only read here; gpu_alpha_blit takes
                        // the buffer by reference so we snapshot the needed values
                        // to avoid the borrow-checker conflict).
                        let src_snapshot = driver
                            .imported_buffers
                            .get(&buf_id)
                            .map(|s| (s.ptr, s.size, s.stride, s.format));
                        if let Some((src_ptr, src_size, src_stride, src_format)) = src_snapshot {
                            // Build a temporary ImportedBuffer view from the snapshot.
                            let tmp_buf = ImportedBuffer {
                                fd: 0,
                                ptr: src_ptr,
                                size: src_size,
                                width: 0,
                                height: 0,
                                stride: src_stride,
                                format: src_format,
                            };
                            if let Err(e) =
                                gpu_alpha_blit(driver, idx, &tmp_buf, sx, sy, cw, ch, dx, dy, ga)
                            {
                                // Fall back to CPU blend for this plane.
                                stem::warn!(
                                    "DISP: GPU alpha blit failed ({:?}), applying CPU fallback",
                                    e
                                );
                                driver.cpu_fallback_planes =
                                    driver.cpu_fallback_planes.saturating_add(1);
                                driver.gpu_path_planes = driver.gpu_path_planes.saturating_sub(1);
                                let target_ptr = driver.frame_pool[idx].ptr;
                                let target_size = driver.frame_pool[idx].size;
                                let fb_stride = driver.disp_stride as usize;
                                unsafe {
                                    for row in 0..ch {
                                        for col in 0..cw {
                                            let s_off = (sy + row)
                                                .saturating_mul(src_stride as usize)
                                                + (sx + col).saturating_mul(4);
                                            let d_off = (dy + row).saturating_mul(fb_stride)
                                                + (dx + col).saturating_mul(4);
                                            if s_off + 4 > src_size || d_off + 4 > target_size {
                                                continue;
                                            }
                                            let sp = source_argb_for_blend(
                                                core::ptr::read_unaligned(
                                                    src_ptr.add(s_off) as *const u32
                                                ),
                                                src_format,
                                            );
                                            let dp = core::ptr::read_unaligned(
                                                target_ptr.add(d_off) as *const u32,
                                            );
                                            core::ptr::write_unaligned(
                                                target_ptr.add(d_off) as *mut u32,
                                                alpha_over_argb(sp, dp, ga),
                                            );
                                        }
                                    }
                                }
                                // Re-upload the CPU-blended region.
                                let fallback_rect =
                                    Rect { x: dx as u32, y: dy as u32, w: cw as u32, h: ch as u32 };
                                let _ = driver.gpu.transfer_to_host_with_stride(
                                    res_id,
                                    fallback_rect,
                                    driver.disp_stride,
                                );
                            }
                        }
                    }
                    stem::trace!(
                        "DISP: COMMIT flush begin seq={} res_id={}",
                        driver.present_seq.saturating_add(1),
                        res_id
                    );
                    if let Err(e) = driver.gpu.flush_resource(res_id, dmg) {
                        stem::error!("DISP: flush_resource failed: {}", e);
                        command_ok = false;
                    }
                    stem::trace!(
                        "DISP: COMMIT flush end seq={} res_id={}",
                        driver.present_seq.saturating_add(1),
                        res_id
                    );
                    if driver.current_res_id != res_id {
                        stem::trace!(
                            "DISP: COMMIT set_scanout begin seq={} old_res={} new_res={}",
                            driver.present_seq.saturating_add(1),
                            driver.current_res_id,
                            res_id
                        );
                        if let Err(e) =
                            driver.gpu.set_scanout(res_id, driver.disp_width, driver.disp_height)
                        {
                            stem::error!("DISP: set_scanout failed: {}", e);
                            command_ok = false;
                        }
                        stem::trace!(
                            "DISP: COMMIT set_scanout end seq={} new_res={}",
                            driver.present_seq.saturating_add(1),
                            res_id
                        );
                    }
                    if !command_ok {
                        return ProviderResponse::err(Errno::EIO);
                    }
                    driver.present_seq += 1;
                    driver.frame_pool[idx].last_present_seq = driver.present_seq;
                    driver.last_presented_idx = Some(idx);
                    driver.current_res_id = res_id;
                    if !driver.first_commit_logged {
                        if let Some((buffer_id, src_px, dst_px, copy_w, copy_h)) = first_copy_sample
                        {
                            // `gpu_planes` and `cpu_planes` are cumulative totals
                            // across all commits processed so far (including this
                            // one), not counts for this commit alone.
                            stem::info!(
                                "display_virtio_gpu: first commit copied buffer={} rect={}x{} src_px=0x{:08x} dst_px=0x{:08x} damage={}x{}+{},{} res_id={} gpu_planes={} cpu_planes={}",
                                buffer_id,
                                copy_w,
                                copy_h,
                                src_px,
                                dst_px,
                                dmg.w,
                                dmg.h,
                                dmg.x,
                                dmg.y,
                                res_id,
                                driver.gpu_path_planes,
                                driver.cpu_fallback_planes,
                            );
                        } else {
                            stem::warn!(
                                "display_virtio_gpu: first commit had damage but no copied sample"
                            );
                        }
                        driver.first_commit_logged = true;
                    }
                    if !driver.cursor_commit_logged && saw_cursor_plane {
                        if let Some((buffer_id, src_px, dst_before, dst_after, x, y)) =
                            cursor_copy_sample
                        {
                            stem::info!(
                                "display_virtio_gpu: cursor plane blended buffer={} at {},{} src_px=0x{:08x} dst_before=0x{:08x} dst_after=0x{:08x}",
                                buffer_id,
                                x,
                                y,
                                src_px,
                                dst_before,
                                dst_after
                            );
                        } else {
                            stem::warn!(
                                "display_virtio_gpu: cursor plane had no non-transparent sampled pixels"
                            );
                        }
                        driver.cursor_commit_logged = true;
                    }
                    stem::trace!("DISP: COMMIT complete (seq={})", driver.present_seq);
                    driver.current_fd = Some(driver.frame_pool[idx].fd);

                    if req.flags.contains(CommitFlags::VSYNC) {
                        stem::trace!(
                            "DISP: ignoring VSYNC flag on synchronous VFS commit response path"
                        );
                    }

                    if driver.cursor_resource_id != 0 && driver.cursor_buffer_id.is_some() {
                        let (pos_x, pos_y) = driver.cursor_pos;
                        let resource_id =
                            if driver.cursor_visible { driver.cursor_resource_id } else { 0 };
                        let (hot_x, hot_y) = driver.cursor_hotspot;
                        if let Err(e) =
                            driver.gpu.update_cursor(resource_id, hot_x, hot_y, pos_x, pos_y)
                        {
                            stem::warn!(
                                "display_virtio_gpu: cursor reassert after commit failed: {}",
                                e
                            );
                        }
                    }
                }
            }

            stem::debug!("display.phase=commit_done");
            ProviderResponse::ok_device_call(0, &[])
        }
        DISPLAY_OP_SET_CURSOR => {
            // Upload a cursor image and configure the hardware cursor hotspot.
            if call_payload.len() < core::mem::size_of::<SetCursorRequest>() {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let req: SetCursorRequest =
                unsafe { core::ptr::read_unaligned(call_payload.as_ptr() as *const _) };

            if !driver.gpu.has_cursorq() {
                return ProviderResponse::err(Errno::ENOSYS);
            }

            // Cursor DMA buffer must have been successfully pre-allocated.
            if driver.cursor_dma_buf == 0 || driver.cursor_dma_phys == 0 {
                stem::warn!("display_virtio_gpu: SET_CURSOR: no cursor DMA buffer available");
                return ProviderResponse::err(Errno::ENOMEM);
            }

            // Validate dimensions against the DMA buffer capacity.
            if req.width == 0
                || req.height == 0
                || req.width > MAX_CURSOR_SIDE
                || req.height > MAX_CURSOR_SIDE
            {
                return ProviderResponse::err(Errno::EINVAL);
            }

            // Look up the imported buffer backing the cursor image.
            let (src_ptr, src_stride, src_width, src_height, src_format) = {
                let src = match driver.imported_buffers.get(&req.buffer_id) {
                    Some(s) => s,
                    None => return ProviderResponse::err(Errno::ENOENT),
                };
                (src.ptr, src.stride, src.width, src.height, src.format)
            };
            if !src_format.has_alpha() {
                return ProviderResponse::err(Errno::EINVAL);
            }

            // Copy cursor pixels from the imported (client-mapped) buffer into
            // the pre-allocated DMA buffer so the GPU reads from physically
            // contiguous, device-accessible memory.
            let bpp = CURSOR_BPP;
            // Use checked arithmetic so malformed dimensions cannot cause
            // integer overflow and a subsequent out-of-bounds write.
            let dst_stride = match (HW_CURSOR_SIDE as usize).checked_mul(bpp) {
                Some(s) => s,
                None => return ProviderResponse::err(Errno::EINVAL),
            };
            let dma_size = match (HW_CURSOR_SIDE as usize).checked_mul(dst_stride) {
                Some(s) => s,
                None => return ProviderResponse::err(Errno::EINVAL),
            };
            // Guard: computed size must not exceed the pre-allocated DMA buffer.
            if dma_size > MAX_CURSOR_BYTES {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let src_w = req.width.min(src_width);
            let src_h = req.height.min(src_height);
            if src_w == 0 || src_h == 0 {
                return ProviderResponse::err(Errno::EINVAL);
            }

            // Zero the fixed-size hardware cursor resource first; transparent
            // padding is required when the requested cursor is smaller than
            // the virtio-mandated 64x64 image.
            unsafe {
                core::ptr::write_bytes(driver.cursor_dma_buf as *mut u8, 0, dma_size);
            }
            for dst_y in 0..HW_CURSOR_SIDE {
                if src_h <= HW_CURSOR_SIDE && dst_y >= src_h {
                    continue;
                }
                let src_y = cursor_hw_coord(dst_y, src_h) as usize;
                for dst_x in 0..HW_CURSOR_SIDE {
                    if src_w <= HW_CURSOR_SIDE && dst_x >= src_w {
                        continue;
                    }
                    let src_x = cursor_hw_coord(dst_x, src_w) as usize;
                    unsafe {
                        let src_px = core::ptr::read_unaligned(src_ptr.add(
                            src_y.saturating_mul(src_stride as usize) + src_x.saturating_mul(bpp),
                        )
                            as *const u32);
                        let cursor_px = cursor_argb_to_host(src_px);
                        core::ptr::write_unaligned(
                            (driver.cursor_dma_buf as *mut u8).add(
                                (dst_y as usize).saturating_mul(dst_stride)
                                    + (dst_x as usize).saturating_mul(bpp),
                            ) as *mut u32,
                            cursor_px,
                        );
                    }
                }
            }

            // Allocate a virtio-GPU resource for the cursor on first use;
            // re-create it when the image dimensions change.  An existing
            // resource can be reused when only the pixel data changes.
            let hw_cursor_size = (HW_CURSOR_SIDE, HW_CURSOR_SIDE);
            let size_changed = driver.cursor_attached_size != hw_cursor_size;
            let need_new_resource = driver.cursor_resource_id == 0 || size_changed;
            if need_new_resource {
                // Allocate a fresh resource ID only when we don't already have one.
                let res_id = if driver.cursor_resource_id == 0 {
                    driver.gpu.alloc_resource_id()
                } else {
                    // Size changed: reuse the existing ID to avoid ID exhaustion.
                    driver.cursor_resource_id
                };
                if let Err(e) = driver.gpu.create_resource_2d_sized_with_format(
                    res_id,
                    HW_CURSOR_SIDE,
                    HW_CURSOR_SIDE,
                    virtio_gpu::VIRTIO_GPU_FORMAT_B8G8R8A8_UNORM,
                ) {
                    stem::warn!("display_virtio_gpu: cursor resource create failed: {}", e);
                    return ProviderResponse::err(Errno::ENOMEM);
                }
                if let Err(e) = driver.gpu.attach_backing_preserve_state(
                    res_id,
                    driver.cursor_dma_phys,
                    dma_size,
                ) {
                    stem::warn!("display_virtio_gpu: cursor attach_backing failed: {}", e);
                    return ProviderResponse::err(Errno::ENOMEM);
                }
                driver.cursor_resource_id = res_id;
                driver.cursor_attached_size = hw_cursor_size;
            }

            let res_id = driver.cursor_resource_id;
            // Transfer the freshly-copied DMA pixels to the GPU resource.
            let transfer_rect =
                virtio_gpu::Rect { x: 0, y: 0, w: HW_CURSOR_SIDE, h: HW_CURSOR_SIDE };
            if let Err(e) =
                driver.gpu.transfer_to_host_with_stride(res_id, transfer_rect, dst_stride as u32)
            {
                stem::warn!("display_virtio_gpu: cursor transfer failed: {}", e);
                return ProviderResponse::err(Errno::EIO);
            }

            let resource_id = if req.visible != 0 { res_id } else { 0 };
            let hot_x = cursor_hw_hotspot(req.hotspot_x, src_w);
            let hot_y = cursor_hw_hotspot(req.hotspot_y, src_h);
            let (pos_x, pos_y) = driver.cursor_pos;
            if let Err(e) = driver.gpu.update_cursor(resource_id, hot_x, hot_y, pos_x, pos_y) {
                stem::warn!("display_virtio_gpu: update_cursor failed: {}", e);
                return ProviderResponse::err(Errno::EIO);
            }

            driver.cursor_buffer_id = Some(req.buffer_id);
            driver.cursor_hotspot = (hot_x, hot_y);
            driver.cursor_visible = req.visible != 0;
            stem::debug!(
                "display_virtio_gpu: hw cursor set buffer={} size={}x{} hw_size={}x{} hotspot={},{} hw_hotspot={},{} visible={} (dma_phys=0x{:x})",
                req.buffer_id.0,
                req.width,
                req.height,
                HW_CURSOR_SIDE,
                HW_CURSOR_SIDE,
                req.hotspot_x,
                req.hotspot_y,
                hot_x,
                hot_y,
                req.visible,
                driver.cursor_dma_phys,
            );
            ProviderResponse::ok_device_call(0, &[])
        }
        DISPLAY_OP_MOVE_CURSOR => {
            // Move the hardware cursor hotspot without a full scene commit.
            if call_payload.len() < core::mem::size_of::<MoveCursorRequest>() {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let req: MoveCursorRequest =
                unsafe { core::ptr::read_unaligned(call_payload.as_ptr() as *const _) };

            if !driver.gpu.has_cursorq() {
                return ProviderResponse::err(Errno::ENOSYS);
            }

            // Clamp to screen bounds; saturating cast to u32 handles negatives.
            let pos_x = req.x.max(0) as u32;
            let pos_y = req.y.max(0) as u32;
            driver.cursor_pos = (pos_x, pos_y);
            driver.cursor_visible = req.visible != 0;

            if req.visible != 0 {
                let res_id = driver.cursor_resource_id;
                if res_id == 0 || driver.cursor_buffer_id.is_none() {
                    return ProviderResponse::err(Errno::ENOENT);
                }
                let (hot_x, hot_y) = driver.cursor_hotspot;
                if let Err(e) = driver.gpu.update_cursor(res_id, hot_x, hot_y, pos_x, pos_y) {
                    stem::warn!("display_virtio_gpu: cursor move update failed: {}", e);
                    return ProviderResponse::err(Errno::EIO);
                }
            } else {
                // Hide cursor by sending UPDATE_CURSOR with resource_id = 0.
                if let Err(e) = driver.gpu.update_cursor(0, 0, 0, pos_x, pos_y) {
                    stem::warn!("display_virtio_gpu: cursor hide failed: {}", e);
                    return ProviderResponse::err(Errno::EIO);
                }
            }

            stem::trace!(
                "display_virtio_gpu: hw cursor moved to {},{} visible={}",
                req.x,
                req.y,
                req.visible,
            );
            ProviderResponse::ok_device_call(0, &[])
        }
        DISPLAY_OP_ACCEL2D => execute_accel2d(driver, call_payload),
        _ => ProviderResponse::err(Errno::ENOSYS),
    }
}

// ============================================================================
// DISPLAY_OP_ACCEL2D — 2D acceleration command batch handler
// ============================================================================

/// Validate that `dst` is a legal ACCEL2D v1 destination buffer.
///
/// Only `BufferId(0)` (the driver-owned output framebuffer) is writable in the
/// current CPU-fallback implementation.  Any other ID returns `EINVAL` because
/// the v1 ABI explicitly defines `BufferId(0)` as the sole writable target —
/// this is an invalid argument, not an unimplemented feature.
///
/// # v1 Contract
///
/// `BufferId(0)` = output framebuffer (writable).
/// All other IDs → `EINVAL`.
///
/// TODO(multi-buffer): Lift this restriction when the GPU path that supports
/// off-screen compositing targets is implemented.
#[inline]
fn validate_accel2d_dst_buffer(dst: BufferId) -> abi::errors::SysResult<()> {
    if dst != BufferId(0) {
        return Err(Errno::EINVAL);
    }
    Ok(())
}

/// Validate that `format` meets the ACCEL2D v1 source-buffer format contract.
///
/// Only 4-bytes-per-pixel formats (`Bgra8888`, `Bgrx8888`) are accepted.
/// Callers that pass any other format receive `EINVAL` — an explicit contract
/// violation, not a missing feature.
#[inline]
fn validate_accel2d_src_format(format: PixelFormat) -> abi::errors::SysResult<()> {
    if format.bytes_per_pixel() != 4 {
        return Err(Errno::EINVAL);
    }
    Ok(())
}

// ============================================================================
// ACCEL2D GPU execution path
// ============================================================================

/// GPU-backed opaque copy: stage `src_buffer[src_rect]` into the virgl
/// staging buffer and issue `VIRGL_CCMD_BLIT` (no alpha blend) to copy it
/// into the frame-pool 2D resource at `dst_rect`.
///
/// Returns `Ok(())` on success.  Returns `Err(Errno::ENOSYS)` when the virgl
/// subsystem is not ready, signalling the caller to fall back to the CPU path.
fn gpu_accel2d_copy_rect(
    driver: &mut VirtioGpuDriver,
    idx: usize,
    src_buffer: &ImportedBuffer,
    src_rect: abi::display_protocol::Rect,
    dst_rect: abi::display_protocol::Rect,
) -> abi::errors::SysResult<()> {
    if driver.virgl_ctx_id == 0 || driver.virgl_src_res_id == 0 {
        return Err(abi::errors::Errno::ENOSYS);
    }
    let copy_w = src_rect.w as usize;
    let copy_h = src_rect.h as usize;
    if copy_w == 0 || copy_h == 0 {
        return Ok(());
    }

    // Verify the source region fits within the staging buffer.
    let needed = match copy_w.checked_mul(copy_h).and_then(|n| n.checked_mul(4)) {
        Some(n) => n,
        None => return Err(abi::errors::Errno::EINVAL),
    };
    if needed > driver.virgl_blend_staging_size {
        // Region too large for the staging buffer; fall back to CPU.
        return Err(abi::errors::Errno::ENOSYS);
    }

    let staging_ptr = driver.virgl_blend_staging_buf as *mut u8;
    let staging_stride = (copy_w as u32) * 4;
    let bpp = 4usize;

    // ── Stage source pixels (CPU write-only pass) ─────────────────────────────
    // Per-pixel bounds checking ensures that a malformed (or shrunk) source
    // buffer cannot cause out-of-bounds reads even when src_rect extends
    // beyond the buffer's actual dimensions.  Out-of-bounds pixels are
    // replaced with opaque black so the GPU never reads uninitialised staging
    // memory.  Performance: this loop is write-only to the staging buffer
    // (no destination read-modify-write), matching the pattern used by
    // gpu_alpha_blit() for the COMMIT path.  For large regions the overhead
    // is dominated by the subsequent GPU texture upload, not this loop.
    unsafe {
        for row in 0..copy_h {
            for col in 0..copy_w {
                let src_off = (src_rect.y as usize + row)
                    .saturating_mul(src_buffer.stride as usize)
                    + (src_rect.x as usize + col).saturating_mul(bpp);
                let stg_off = row.saturating_mul(staging_stride as usize) + col.saturating_mul(bpp);
                let px = if src_off + bpp <= src_buffer.size {
                    // Normalise BGRX → BGRA by forcing alpha=0xff for opaque formats.
                    let raw = core::ptr::read_unaligned(src_buffer.ptr.add(src_off) as *const u32);
                    source_argb_for_blend(raw, src_buffer.format)
                } else {
                    // Out-of-bounds: write opaque black.
                    0xff00_0000u32
                };
                core::ptr::write_unaligned(staging_ptr.add(stg_off) as *mut u32, px);
            }
        }
    }

    // ── Upload staged pixels to virgl source texture ──────────────────────────
    driver
        .gpu
        .transfer_to_host_3d(
            driver.virgl_ctx_id,
            driver.virgl_src_res_id,
            copy_w as u32,
            copy_h as u32,
            0,
            staging_stride,
        )
        .map_err(|_| abi::errors::Errno::EIO)?;

    // ── GPU BLIT (opaque copy, no alpha blend) ────────────────────────────────
    let dst_res_id = driver.frame_pool[idx].res_id;
    let blit_cmd = virtio_gpu::virgl_encode_blit(
        driver.virgl_src_res_id,
        dst_res_id,
        0,
        0,
        copy_w as u32,
        copy_h as u32,
        dst_rect.x,
        dst_rect.y,
        dst_rect.w,
        dst_rect.h,
        virtio_gpu::VIRTIO_GPU_FORMAT_B8G8R8A8_UNORM,
        virtio_gpu::VIRTIO_GPU_FORMAT_B8G8R8X8_UNORM,
        false, // opaque copy — no alpha blending
    );
    driver.gpu.submit_3d(driver.virgl_ctx_id, &blit_cmd).map_err(|_| abi::errors::Errno::EIO)?;

    Ok(())
}

/// GPU-backed alpha blit: wraps the existing [`gpu_alpha_blit`] helper with
/// the `src_rect` / `dst_rect` ABI used by `ACCEL2D_CMD_ALPHA_BLIT`.
///
/// Returns `Err(Errno::ENOSYS)` when virgl is unavailable so the caller can
/// fall back to the CPU path transparently.
fn gpu_accel2d_alpha_blit(
    driver: &mut VirtioGpuDriver,
    idx: usize,
    src_buffer: &ImportedBuffer,
    src_rect: abi::display_protocol::Rect,
    dst_rect: abi::display_protocol::Rect,
    global_alpha: u8,
) -> abi::errors::SysResult<()> {
    gpu_alpha_blit(
        driver,
        idx,
        src_buffer,
        src_rect.x as usize,
        src_rect.y as usize,
        src_rect.w as usize,
        src_rect.h as usize,
        dst_rect.x as usize,
        dst_rect.y as usize,
        global_alpha,
    )
}

/// Parse and execute a batch of 2D acceleration commands sent via
/// `DISPLAY_OP_ACCEL2D`.
///
/// When the virgl GPU pipeline is available (i.e. `ACCEL2D_GPU` is advertised
/// in `DisplayCaps`), eligible commands (`ACCEL2D_CMD_COPY_RECT` and
/// `ACCEL2D_CMD_ALPHA_BLIT`) are dispatched to the GPU execution path.
/// Commands without a GPU equivalent (`ACCEL2D_CMD_CLEAR_RECT`,
/// `ACCEL2D_CMD_STRETCH_BLIT`, `ACCEL2D_CMD_MASKED_BLIT`,
/// `ACCEL2D_CMD_ROUNDED_CLIP_BLIT`) always use the CPU fallback.
/// `ACCEL2D_CMD_FLUSH_DAMAGE` uploads dirty CPU pixels and flushes them to
/// the display via the standard virtio-gpu 2D path.
fn execute_accel2d(driver: &mut VirtioGpuDriver, call_payload: &[u8]) -> ProviderResponse {
    let batch_size = core::mem::size_of::<Accel2dBatch>();
    if call_payload.len() < batch_size {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let header: Accel2dBatch =
        unsafe { core::ptr::read_unaligned(call_payload.as_ptr() as *const _) };
    let cmd_count = header.cmd_count as usize;
    // Reject unreasonably large batches before arithmetic to prevent
    // malformed requests from exhausting memory.
    if cmd_count > MAX_ACCEL2D_BATCH_CMDS {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let needed = batch_size.saturating_add(cmd_count.saturating_mul(ACCEL2D_COMMAND_SIZE));
    if call_payload.len() < needed {
        return ProviderResponse::err(Errno::EINVAL);
    }

    // Work on the currently displayed frame-pool buffer so incremental updates
    // are visible on top of the last committed scene.
    let idx = driver.last_presented_idx.unwrap_or(0);
    if driver.frame_pool.is_empty() {
        return ProviderResponse::err(Errno::ENXIO);
    }

    for i in 0..cmd_count {
        let off = batch_size + i * ACCEL2D_COMMAND_SIZE;
        let cmd: Accel2dCommand = unsafe {
            core::ptr::read_unaligned(
                call_payload[off..off + ACCEL2D_COMMAND_SIZE].as_ptr() as *const _
            )
        };
        if let Err(e) = execute_accel2d_cmd(driver, idx, &cmd) {
            return ProviderResponse::err(e);
        }
    }

    ProviderResponse::ok_device_call(0, &[])
}

/// Execute a single 2D acceleration command against `driver.frame_pool[idx]`.
///
/// For `ACCEL2D_CMD_COPY_RECT` and `ACCEL2D_CMD_ALPHA_BLIT`, the GPU (virgl)
/// path is attempted first; if the GPU subsystem is not ready (`ENOSYS`), the
/// command silently falls back to the CPU path.  All other commands always use
/// the CPU path.  The `accel2d_gpu_cmds` / `accel2d_cpu_cmds` counters are
/// updated accordingly for diagnostics.
fn execute_accel2d_cmd(
    driver: &mut VirtioGpuDriver,
    idx: usize,
    cmd: &Accel2dCommand,
) -> abi::errors::SysResult<()> {
    match cmd.kind {
        ACCEL2D_CMD_CLEAR_RECT => {
            let c = unsafe { cmd.body.clear_rect };
            driver.accel2d_cpu_cmds = driver.accel2d_cpu_cmds.saturating_add(1);
            accel2d_clear_rect(driver, idx, c.dst_buffer, c.rect, c.color)
        }
        ACCEL2D_CMD_COPY_RECT => {
            let c = unsafe { cmd.body.copy_rect };
            validate_accel2d_dst_buffer(c.dst_buffer)?;
            // Extract source buffer metadata before any mutable borrow.
            let src_snapshot =
                driver.imported_buffers.get(&c.src_buffer).ok_or(Errno::ENOENT).and_then(|s| {
                    validate_accel2d_src_format(s.format)?;
                    Ok(*s)
                })?;
            // Try GPU path first; fall back to CPU on ENOSYS.
            match gpu_accel2d_copy_rect(driver, idx, &src_snapshot, c.src_rect, c.dst_rect) {
                Ok(()) => {
                    trace!("display_virtio_gpu: [batch {}] copy_rect using GPU", idx);
                    driver.accel2d_gpu_cmds = driver.accel2d_gpu_cmds.saturating_add(1);
                    Ok(())
                }
                Err(abi::errors::Errno::ENOSYS) => {
                    trace!("display_virtio_gpu: [batch {}] copy_rect using CPU", idx);
                    driver.accel2d_cpu_cmds = driver.accel2d_cpu_cmds.saturating_add(1);
                    accel2d_copy_rect(
                        driver,
                        idx,
                        c.src_buffer,
                        c.dst_buffer,
                        c.src_rect,
                        c.dst_rect,
                    )
                }
                Err(e) => {
                    warn!(
                        "display_virtio_gpu: [batch {}] gpu_accel2d_copy_rect failed: {:?}",
                        idx, e
                    );
                    Err(e)
                }
            }
        }
        ACCEL2D_CMD_STRETCH_BLIT => {
            let c = unsafe { cmd.body.stretch_blit };
            driver.accel2d_cpu_cmds = driver.accel2d_cpu_cmds.saturating_add(1);
            accel2d_stretch_blit(driver, idx, c.src_buffer, c.dst_buffer, c.src_rect, c.dst_rect)
        }
        ACCEL2D_CMD_ALPHA_BLIT => {
            let c = unsafe { cmd.body.alpha_blit };
            validate_accel2d_dst_buffer(c.dst_buffer)?;
            // Extract source buffer metadata before any mutable borrow.
            let src_snapshot =
                driver.imported_buffers.get(&c.src_buffer).ok_or(Errno::ENOENT).and_then(|s| {
                    validate_accel2d_src_format(s.format)?;
                    Ok(*s)
                })?;
            // Try GPU path first; fall back to CPU on ENOSYS.
            match gpu_accel2d_alpha_blit(
                driver,
                idx,
                &src_snapshot,
                c.src_rect,
                c.dst_rect,
                c.global_alpha,
            ) {
                Ok(()) => {
                    trace!("display_virtio_gpu: [batch {}] alpha_blit using GPU", idx);
                    driver.accel2d_gpu_cmds = driver.accel2d_gpu_cmds.saturating_add(1);
                    Ok(())
                }
                Err(abi::errors::Errno::ENOSYS) => {
                    trace!("display_virtio_gpu: [batch {}] alpha_blit using CPU", idx);
                    driver.accel2d_cpu_cmds = driver.accel2d_cpu_cmds.saturating_add(1);
                    accel2d_alpha_blit(
                        driver,
                        idx,
                        c.src_buffer,
                        c.dst_buffer,
                        c.src_rect,
                        c.dst_rect,
                        c.global_alpha,
                    )
                }
                Err(e) => {
                    warn!(
                        "display_virtio_gpu: [batch {}] gpu_accel2d_alpha_blit failed: {:?}",
                        idx, e
                    );
                    Err(e)
                }
            }
        }
        ACCEL2D_CMD_MASKED_BLIT => {
            let c = unsafe { cmd.body.masked_blit };
            driver.accel2d_cpu_cmds = driver.accel2d_cpu_cmds.saturating_add(1);
            accel2d_masked_blit(
                driver,
                idx,
                c.src_buffer,
                c.mask_buffer,
                c.dst_buffer,
                c.src_rect,
                c.mask_rect,
                c.dst_rect,
            )
        }
        ACCEL2D_CMD_ROUNDED_CLIP_BLIT => {
            let c = unsafe { cmd.body.rounded_clip_blit };
            driver.accel2d_cpu_cmds = driver.accel2d_cpu_cmds.saturating_add(1);
            accel2d_rounded_clip_blit(
                driver,
                idx,
                c.src_buffer,
                c.dst_buffer,
                c.src_rect,
                c.dst_rect,
                c.radius,
            )
        }
        ACCEL2D_CMD_FLUSH_DAMAGE => {
            let c = unsafe { cmd.body.flush_damage };
            accel2d_flush_damage(driver, idx, &c)
        }
        _ => Err(Errno::ENOSYS),
    }
}

/// Fill a rectangle in the frame-pool buffer with a solid colour.
///
/// Only `BufferId(0)` (the driver framebuffer) is supported as destination.
/// Any other destination buffer ID returns `EINVAL` per the v1 ABI contract.
fn accel2d_clear_rect(
    driver: &mut VirtioGpuDriver,
    idx: usize,
    dst_buffer: BufferId,
    rect: abi::display_protocol::Rect,
    color: u32,
) -> abi::errors::SysResult<()> {
    validate_accel2d_dst_buffer(dst_buffer)?;
    let dst = PixelBuf {
        ptr: driver.frame_pool[idx].ptr,
        size: driver.frame_pool[idx].size,
        width: driver.disp_width,
        height: driver.disp_height,
        stride: driver.disp_stride,
        format: PixelFormat::Bgra8888,
    };
    unsafe { accel2d_cpu::clear_rect(&dst, rect, color) };
    Ok(())
}

/// Copy pixels from an imported buffer into the frame-pool buffer.
///
/// Only `BufferId(0)` as destination is supported (v1 contract).
/// Source buffer must use a 4-bpp format (`Bgra8888` or `Bgrx8888`);
/// any other format returns `EINVAL`.
fn accel2d_copy_rect(
    driver: &mut VirtioGpuDriver,
    idx: usize,
    src_buffer: BufferId,
    dst_buffer: BufferId,
    src_rect: abi::display_protocol::Rect,
    dst_rect: abi::display_protocol::Rect,
) -> abi::errors::SysResult<()> {
    validate_accel2d_dst_buffer(dst_buffer)?;
    let src = driver.imported_buffers.get(&src_buffer).ok_or(Errno::ENOENT)?;
    validate_accel2d_src_format(src.format)?;
    let src_buf = PixelBuf {
        ptr: src.ptr,
        size: src.size,
        width: src.width,
        height: src.height,
        stride: src.stride,
        format: src.format,
    };
    let dst_buf = PixelBuf {
        ptr: driver.frame_pool[idx].ptr,
        size: driver.frame_pool[idx].size,
        width: driver.disp_width,
        height: driver.disp_height,
        stride: driver.disp_stride,
        format: PixelFormat::Bgra8888,
    };
    unsafe { accel2d_cpu::copy_rect(&src_buf, &dst_buf, src_rect, dst_rect) };
    Ok(())
}

/// Scale-copy from an imported buffer to the frame-pool buffer using
/// nearest-neighbour sampling.
///
/// Only `BufferId(0)` as destination is supported (v1 contract).
/// Source buffer must use a 4-bpp format (`Bgra8888` or `Bgrx8888`);
/// any other format returns `EINVAL`.
fn accel2d_stretch_blit(
    driver: &mut VirtioGpuDriver,
    idx: usize,
    src_buffer: BufferId,
    dst_buffer: BufferId,
    src_rect: abi::display_protocol::Rect,
    dst_rect: abi::display_protocol::Rect,
) -> abi::errors::SysResult<()> {
    validate_accel2d_dst_buffer(dst_buffer)?;
    let src = driver.imported_buffers.get(&src_buffer).ok_or(Errno::ENOENT)?;
    validate_accel2d_src_format(src.format)?;
    let src_buf = PixelBuf {
        ptr: src.ptr,
        size: src.size,
        width: src.width,
        height: src.height,
        stride: src.stride,
        format: src.format,
    };
    let dst_buf = PixelBuf {
        ptr: driver.frame_pool[idx].ptr,
        size: driver.frame_pool[idx].size,
        width: driver.disp_width,
        height: driver.disp_height,
        stride: driver.disp_stride,
        format: PixelFormat::Bgra8888,
    };
    unsafe { accel2d_cpu::stretch_blit(&src_buf, &dst_buf, src_rect, dst_rect) };
    Ok(())
}

/// Alpha-blend an imported buffer over the frame-pool buffer.
///
/// Only `BufferId(0)` as destination is supported (v1 contract).
/// Source buffer must use a 4-bpp format (`Bgra8888` or `Bgrx8888`);
/// any other format returns `EINVAL`.
fn accel2d_alpha_blit(
    driver: &mut VirtioGpuDriver,
    idx: usize,
    src_buffer: BufferId,
    dst_buffer: BufferId,
    src_rect: abi::display_protocol::Rect,
    dst_rect: abi::display_protocol::Rect,
    global_alpha: u8,
) -> abi::errors::SysResult<()> {
    validate_accel2d_dst_buffer(dst_buffer)?;
    let src = driver.imported_buffers.get(&src_buffer).ok_or(Errno::ENOENT)?;
    validate_accel2d_src_format(src.format)?;
    let src_buf = PixelBuf {
        ptr: src.ptr,
        size: src.size,
        width: src.width,
        height: src.height,
        stride: src.stride,
        format: src.format,
    };
    let dst_buf = PixelBuf {
        ptr: driver.frame_pool[idx].ptr,
        size: driver.frame_pool[idx].size,
        width: driver.disp_width,
        height: driver.disp_height,
        stride: driver.disp_stride,
        format: PixelFormat::Bgra8888,
    };
    unsafe { accel2d_cpu::alpha_blit(&src_buf, &dst_buf, src_rect, dst_rect, global_alpha) };
    Ok(())
}

/// Blend an imported buffer over the frame-pool buffer using a mask buffer as
/// per-pixel alpha coverage.
///
/// Only `BufferId(0)` as destination is supported (v1 contract).
/// Both source and mask buffers must use a 4-bpp format (`Bgra8888` or
/// `Bgrx8888`); any other format returns `EINVAL`.
#[allow(clippy::too_many_arguments)]
fn accel2d_masked_blit(
    driver: &mut VirtioGpuDriver,
    idx: usize,
    src_buffer: BufferId,
    mask_buffer: BufferId,
    dst_buffer: BufferId,
    src_rect: abi::display_protocol::Rect,
    mask_rect: abi::display_protocol::Rect,
    dst_rect: abi::display_protocol::Rect,
) -> abi::errors::SysResult<()> {
    validate_accel2d_dst_buffer(dst_buffer)?;
    // Extract source and mask buffer metadata before borrowing driver for the
    // frame pool.
    let (src_ptr, src_size, src_width, src_height, src_stride, src_format) = {
        let src = driver.imported_buffers.get(&src_buffer).ok_or(Errno::ENOENT)?;
        validate_accel2d_src_format(src.format)?;
        (src.ptr, src.size, src.width, src.height, src.stride, src.format)
    };
    let (msk_ptr, msk_size, msk_width, msk_height, msk_stride, msk_format) = {
        let msk = driver.imported_buffers.get(&mask_buffer).ok_or(Errno::ENOENT)?;
        validate_accel2d_src_format(msk.format)?;
        (msk.ptr, msk.size, msk.width, msk.height, msk.stride, msk.format)
    };
    let src_buf = PixelBuf {
        ptr: src_ptr,
        size: src_size,
        width: src_width,
        height: src_height,
        stride: src_stride,
        format: src_format,
    };
    let msk_buf = PixelBuf {
        ptr: msk_ptr,
        size: msk_size,
        width: msk_width,
        height: msk_height,
        stride: msk_stride,
        format: msk_format,
    };
    let dst_buf = PixelBuf {
        ptr: driver.frame_pool[idx].ptr,
        size: driver.frame_pool[idx].size,
        width: driver.disp_width,
        height: driver.disp_height,
        stride: driver.disp_stride,
        format: PixelFormat::Bgra8888,
    };
    unsafe {
        accel2d_cpu::masked_blit(&src_buf, &msk_buf, &dst_buf, src_rect, mask_rect, dst_rect)
    };
    Ok(())
}

/// Blit an imported buffer into the frame-pool buffer, clipped to a rounded
/// rectangle.
///
/// Only `BufferId(0)` as destination is supported (v1 contract).
/// Source buffer must use a 4-bpp format (`Bgra8888` or `Bgrx8888`);
/// any other format returns `EINVAL`.
fn accel2d_rounded_clip_blit(
    driver: &mut VirtioGpuDriver,
    idx: usize,
    src_buffer: BufferId,
    dst_buffer: BufferId,
    src_rect: abi::display_protocol::Rect,
    dst_rect: abi::display_protocol::Rect,
    radius: u8,
) -> abi::errors::SysResult<()> {
    validate_accel2d_dst_buffer(dst_buffer)?;
    let src = driver.imported_buffers.get(&src_buffer).ok_or(Errno::ENOENT)?;
    validate_accel2d_src_format(src.format)?;
    let src_buf = PixelBuf {
        ptr: src.ptr,
        size: src.size,
        width: src.width,
        height: src.height,
        stride: src.stride,
        format: src.format,
    };
    let dst_buf = PixelBuf {
        ptr: driver.frame_pool[idx].ptr,
        size: driver.frame_pool[idx].size,
        width: driver.disp_width,
        height: driver.disp_height,
        stride: driver.disp_stride,
        format: PixelFormat::Bgra8888,
    };
    unsafe { accel2d_cpu::rounded_clip_blit(&src_buf, &dst_buf, src_rect, dst_rect, radius) };
    Ok(())
}

/// Transfer declared damage rectangles to the GPU resource and flush them to
/// the display.  A `rect_count` of 0 means the entire output surface is dirty.
fn accel2d_flush_damage(
    driver: &mut VirtioGpuDriver,
    idx: usize,
    cmd: &abi::display::accel2d::FlushDamageCmd,
) -> abi::errors::SysResult<()> {
    if driver.frame_pool.is_empty() {
        return Err(Errno::ENXIO);
    }
    let res_id = driver.frame_pool[idx].res_id;

    // Build the list of damage rects to flush.
    let mut damage_rects: alloc::vec::Vec<Rect> = alloc::vec::Vec::new();
    if cmd.rect_count == 0 {
        // Full-surface flush.
        damage_rects.push(Rect { x: 0, y: 0, w: driver.disp_width, h: driver.disp_height });
    } else {
        let count = (cmd.rect_count as usize).min(abi::display::accel2d::ACCEL2D_MAX_DAMAGE_RECTS);
        for i in 0..count {
            let r = cmd.rects[i];
            let clamped = rect_clamp_to_bounds(
                Rect { x: r.x, y: r.y, w: r.w, h: r.h },
                driver.disp_width,
                driver.disp_height,
            );
            if !rect_is_empty(clamped) {
                damage_rects.push(clamped);
            }
        }
        if damage_rects.is_empty() {
            // All rects were outside the screen; nothing to flush.
            return Ok(());
        }
    }
    note_damage_snapshot(driver, &damage_rects);

    // Transfer each damaged region to the GPU resource, then flush.
    let stride = driver.disp_stride;
    for dmg in &damage_rects {
        if let Err(e) = driver.gpu.transfer_to_host_with_stride(res_id, *dmg, stride) {
            stem::warn!("ACCEL2D: transfer_to_host failed: {}", e);
            return Err(Errno::EIO);
        }
        if let Err(e) = driver.gpu.flush_resource(res_id, *dmg) {
            stem::warn!("ACCEL2D: flush_resource failed: {}", e);
            return Err(Errno::EIO);
        }
    }

    // Ensure the scanout is pointed at the correct resource.
    if driver.current_res_id != res_id {
        if let Err(e) = driver.gpu.set_scanout(res_id, driver.disp_width, driver.disp_height) {
            stem::warn!("ACCEL2D: set_scanout failed: {}", e);
            return Err(Errno::EIO);
        }
        driver.current_res_id = res_id;
    }

    driver.last_presented_idx = Some(idx);
    driver.present_seq = driver.present_seq.saturating_add(1);
    Ok(())
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

fn unpack_handle(arg: usize, index: u32) -> PortHandle {
    ((arg >> (index * 16)) & 0xFFFF) as PortHandle
}

fn send_msg(handle: PortHandle, msg_type: u16, payload: &[u8]) {
    let mut buf = [0u8; 256];
    if let Some(len) = drvproto::encode_message(&mut buf, msg_type, payload) {
        let mut status = stem::syscall::port_send_all(handle, &buf[..len]);
        if let Err(abi::errors::Errno::EAGAIN) = status {
            // Bridge the handle to a VFS FD for FD-first write-readiness polling.
            if let Ok(fd) = stem::syscall::vfs::vfs_handle_from_port(handle) {
                let mut pollfds = [abi::syscall::PollHandle {
                    handle: fd as i32,
                    events: abi::syscall::poll_flags::POLLOUT,
                    revents: 0,
                }];
                while let Err(abi::errors::Errno::EAGAIN) = status {
                    let _ = stem::syscall::vfs::vfs_poll(&mut pollfds, u64::MAX);
                    status = stem::syscall::port_send_all(handle, &buf[..len]);
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

fn initial_display_dimensions(gpu: &mut VirtioGpu) -> (u32, u32, u32, u32) {
    let (boot_w, boot_h, _boot_stride, boot_format) = get_display_dimensions();
    match gpu.query_display_info() {
        Ok(Some(scanout)) if scanout.width > 0 && scanout.height > 0 => {
            let stride = scanout.width.saturating_mul(DISPLAY_BPP);
            info!(
                "display_virtio_gpu: host scanout {}x{} enabled={} (bootfb was {}x{})",
                scanout.width, scanout.height, scanout.enabled, boot_w, boot_h
            );
            (scanout.width, scanout.height, stride, boot_format)
        }
        Ok(_) => {
            let stride = boot_w.saturating_mul(DISPLAY_BPP);
            warn!(
                "display_virtio_gpu: host scanout unavailable; using bootfb {}x{}",
                boot_w, boot_h
            );
            (boot_w, boot_h, stride, boot_format)
        }
        Err(e) => {
            let stride = boot_w.saturating_mul(DISPLAY_BPP);
            warn!(
                "display_virtio_gpu: GET_DISPLAY_INFO failed ({}); using bootfb {}x{}",
                e, boot_w, boot_h
            );
            (boot_w, boot_h, stride, boot_format)
        }
    }
}

fn create_frame_pool_buffer(
    gpu: &mut VirtioGpu,
    width: u32,
    height: u32,
    stride: u32,
    res_id: u32,
) -> Result<Buffer, &'static str> {
    let size = (height as usize).checked_mul(stride as usize).ok_or("display size overflow")?;
    let fd = stem::syscall::memfd_create("frame_pool", size).map_err(|_| "memfd_create failed")?;
    let phys = stem::syscall::shared_memory_phys(fd).map_err(|_| "shared_memory_phys failed")?;

    let mut req: abi::vm::VmMapReq = unsafe { core::mem::zeroed() };
    req.backing = abi::vm::VmBacking::File { thing: fd, offset: 0 };
    req.len = size;
    req.prot = abi::vm::VmProt::READ | abi::vm::VmProt::WRITE | abi::vm::VmProt::USER;
    let map_resp = stem::syscall::vm_map(&req).map_err(|_| "vm_map(frame_pool) failed")?;

    gpu.set_dimensions(width, height);
    gpu.create_resource_2d_sized_with_format(
        res_id,
        width,
        height,
        virtio_gpu::VIRTIO_GPU_FORMAT_B8G8R8X8_UNORM,
    )?;
    gpu.attach_backing(res_id, phys, size, stride)?;

    Ok(Buffer { fd, res_id, phys, ptr: map_resp.addr as *mut u8, size, last_present_seq: 0 })
}

fn create_frame_pool(
    gpu: &mut VirtioGpu,
    width: u32,
    height: u32,
    stride: u32,
) -> Result<alloc::vec::Vec<Buffer>, &'static str> {
    let mut buffers = alloc::vec::Vec::new();
    for _ in 0..FRAME_POOL_COUNT {
        let res_id = gpu.alloc_resource_id();
        buffers.push(create_frame_pool_buffer(gpu, width, height, stride, res_id)?);
    }
    Ok(buffers)
}

fn release_frame_pool(gpu: &mut VirtioGpu, buffers: alloc::vec::Vec<Buffer>) {
    for buffer in buffers {
        let _ = gpu.resource_unref(buffer.res_id);
        if buffer.ptr as usize != 0 && buffer.size > 0 {
            let _ = stem::syscall::vm_unmap(buffer.ptr as usize, buffer.size);
        }
        let _ = stem::syscall::vfs::vfs_close(buffer.fd);
    }
}

fn resize_frame_pool(
    driver: &mut VirtioGpuDriver,
    width: u32,
    height: u32,
    stride: u32,
) -> Result<(), &'static str> {
    let new_pool = create_frame_pool(&mut driver.gpu, width, height, stride)?;
    let new_res_id = new_pool[0].res_id;
    driver.gpu.set_scanout(new_res_id, width, height)?;

    let old_pool = core::mem::replace(&mut driver.frame_pool, new_pool);
    release_frame_pool(&mut driver.gpu, old_pool);

    driver.disp_width = width;
    driver.disp_height = height;
    driver.disp_stride = stride;
    driver.next_buffer_idx = 0;
    driver.last_presented_idx = None;
    driver.current_fd = Some(driver.frame_pool[0].fd);
    driver.current_res_id = new_res_id;
    driver.present_seq = driver.present_seq.saturating_add(1);
    Ok(())
}

fn refresh_display_mode(driver: &mut VirtioGpuDriver) -> bool {
    let Ok(Some(scanout)) = driver.gpu.query_display_info() else {
        return false;
    };
    if scanout.width == 0 || scanout.height == 0 {
        return false;
    }
    let stride = scanout.width.saturating_mul(DISPLAY_BPP);
    if scanout.width == driver.disp_width
        && scanout.height == driver.disp_height
        && stride == driver.disp_stride
    {
        return false;
    }

    let old_w = driver.disp_width;
    let old_h = driver.disp_height;
    match resize_frame_pool(driver, scanout.width, scanout.height, stride) {
        Ok(()) => {
            info!(
                "display_virtio_gpu: output resized {}x{} -> {}x{}",
                old_w, old_h, driver.disp_width, driver.disp_height
            );
            true
        }
        Err(e) => {
            warn!(
                "display_virtio_gpu: output resize {}x{} -> {}x{} failed: {}",
                old_w, old_h, scanout.width, scanout.height, e
            );
            false
        }
    }
}

#[stem::main]
fn main(boot_arg: usize) -> ! {
    stem::info!("display_virtio_gpu: starting v0.4.1 (boot_arg={})", boot_arg);

    if boot_arg == 0 {
        stem::error!(
            "display_virtio_gpu: No boot argument provided! Standard driver entry required."
        );
        stem::syscall::exit(1);
    }
    stem::debug!("display_virtio_gpu: Starting VFS-native VirtIO GPU driver...");
    stem::debug!("display_virtio_gpu: boot_arg={}", boot_arg);

    let mut drv_req_read = 0;
    let mut drv_resp_write = 0;
    let mut reserved_supervisor_port = 0;
    let mut bind_instance_id = 0;
    let mut cambium_device_path: Option<alloc::string::String> = None;
    let mut cambium_direct_mount = false;

    let boot_size = 4096;
    let req = abi::vm::VmMapReq {
        addr_hint: 0,
        len: boot_size,
        prot: abi::vm::VmProt::READ | abi::vm::VmProt::USER,
        flags: abi::vm::VmMapFlags::empty(),
        backing: abi::vm::VmBacking::File { thing: boot_arg as u32, offset: 0 },
    };

    stem::debug!("display_virtio_gpu: Mapping bootstrap memfd {} size={}...", boot_arg, boot_size);
    match stem::syscall::vm_map(&req) {
        Ok(resp) => {
            stem::debug!("display_virtio_gpu: vm_map success at 0x{:x}", resp.addr);
            let entry_ctx = unsafe { &*(resp.addr as *const DriverEntryCtx) };
            if entry_ctx.version == 1 {
                let path = entry_ctx.device_path_str();
                if !path.is_empty() {
                    cambium_device_path = Some(path.to_string());
                }
                cambium_direct_mount = true;
                match port_create(VFS_RPC_MAX_REQ * 8) {
                    Ok((dummy_write, dummy_read)) => {
                        drv_req_read = dummy_read;
                        drv_resp_write = dummy_write;
                    }
                    Err(e) => {
                        stem::error!(
                            "display_virtio_gpu: failed to create direct-mode protocol ports: {:?}",
                            e
                        );
                        stem::syscall::exit(1);
                    }
                }
                stem::info!(
                    "display_virtio_gpu: recovered Cambium DriverEntryCtx device_path='{}'",
                    cambium_device_path.as_deref().unwrap_or("")
                );
            } else {
                let slice = unsafe { core::slice::from_raw_parts(resp.addr as *const u32, 1024) };
                // slice[3..5]: bind_instance_id (u64)

                drv_req_read = slice[0];
                drv_resp_write = slice[1];
                reserved_supervisor_port = slice[2];

                let id_low = slice[3] as u64;
                let id_high = slice[4] as u64;
                bind_instance_id = id_low | (id_high << 32);

                stem::debug!(
                    "display_virtio_gpu: Recovered handles: req_read={}, resp_write={}, svc={}, id={}",
                    drv_req_read,
                    drv_resp_write,
                    reserved_supervisor_port,
                    bind_instance_id
                );
            }
        }
        Err(e) => {
            stem::error!(
                "display_virtio_gpu: ERROR: Failed to vm_map bootstrap memfd {}: {:?}",
                boot_arg,
                e
            );
        }
    }

    if drv_req_read == 0 || drv_resp_write == 0 || (!cambium_direct_mount && bind_instance_id == 0)
    {
        stem::error!(
            "DISP: ERROR: Invalid/Missing bootstrap components (req={}, resp={}, svc={}, id={})",
            drv_req_read,
            drv_resp_write,
            reserved_supervisor_port,
            bind_instance_id
        );
        loop {
            stem::yield_now();
        }
    }

    debug!(
        "display_virtio_gpu: starting (drv_req_r={}, drv_resp_w={}, svc={}, id={})",
        drv_req_read, drv_resp_write, reserved_supervisor_port, bind_instance_id
    );

    // Find and initialize GPU
    let gpu_path = match cambium_device_path.clone().or_else(find_gpu) {
        Some(path) => path,
        None => {
            error!("display_virtio_gpu: GPU device not found");
            loop {
                stem::time::sleep_ms(1);
            }
        }
    };

    let mut gpu = match VirtioGpu::new(&gpu_path) {
        Ok(g) => g,
        Err(e) => {
            error!("display_virtio_gpu: Failed to initialize GPU: {:?}", e);
            loop {
                stem::time::sleep_ms(1);
            }
        }
    };

    if let Err(e) = gpu.init_virtio() {
        error!("display_virtio_gpu: Virtio init failed: {}", e);
        loop {
            stem::time::sleep_ms(1);
        }
    }

    info!("display_virtio_gpu: GPU initialized successfully");
    // Log which GPU composition features are enabled.
    // The opaque-copy (GPU fast-copy) path and CPU alpha-blend fallback
    // are always available; virgl 3D extends the GPU acceleration surface.
    info!("display_virtio_gpu: composition paths: gpu_opaque_copy=enabled cpu_alpha_blend=enabled");
    if gpu.has_3d_feature() {
        debug!("display_virtio_gpu: Virgl 3D supported — GPU alpha blend path will be initialized");
    } else {
        debug!("display_virtio_gpu: Virgl 3D not supported, using 2D only");
    }

    // =========================================================================
    // FRAME POOL SETUP: Create GPU resources and buffers for triple buffering
    // =========================================================================
    let (disp_width, disp_height, disp_stride, disp_format) = initial_display_dimensions(&mut gpu);

    debug!(
        "display_virtio_gpu: creating frame pool 1x {}x{} stride={} format={}",
        disp_width, disp_height, disp_stride, disp_format
    );

    let frame_pool_buffers = match create_frame_pool(&mut gpu, disp_width, disp_height, disp_stride)
    {
        Ok(pool) => pool,
        Err(e) => {
            error!("display_virtio_gpu: frame pool setup failed: {}", e);
            loop {
                stem::time::sleep_ms(1);
            }
        }
    };
    let driver_initial_res_id = frame_pool_buffers[0].res_id;

    // Set initial scanout to first buffer
    if let Err(e) = gpu.set_scanout(driver_initial_res_id, disp_width, disp_height) {
        error!("display_virtio_gpu: set_scanout failed: {}", e);
        loop {
            stem::time::sleep_ms(1);
        }
    }

    debug!(
        "display_virtio_gpu: frame pool ready ({} buffer{})",
        FRAME_POOL_COUNT,
        if FRAME_POOL_COUNT == 1 { "" } else { "s" }
    );

    // ── Cursor DMA pixel buffer ───────────────────────────────────────────────
    // Pre-allocate a DMA-backed buffer large enough to hold MAX_CURSOR_SIDE ×
    // MAX_CURSOR_SIDE × 4 bytes.  Pixels are copied into this region on every
    // DISPLAY_OP_SET_CURSOR so the GPU always reads from physically contiguous,
    // device-accessible memory rather than from the client's shared-memory FD.
    let (cursor_dma_buf, cursor_dma_phys) = {
        use stem::syscall::{device_alloc_dma, device_dma_phys};
        let claim = gpu.claim_handle();
        match device_alloc_dma(claim, MAX_CURSOR_PAGES) {
            Ok(virt) => match device_dma_phys(virt) {
                Ok(phys) => {
                    info!(
                        "display_virtio_gpu: cursor DMA buffer ready ({} pages at phys=0x{:x})",
                        MAX_CURSOR_PAGES, phys
                    );
                    (virt, phys)
                }
                Err(e) => {
                    warn!(
                        "display_virtio_gpu: cursor DMA phys lookup failed: {:?}; hw cursor unavailable",
                        e
                    );
                    (0u64, 0u64)
                }
            },
            Err(e) => {
                warn!(
                    "display_virtio_gpu: cursor DMA alloc failed: {:?}; hw cursor unavailable",
                    e
                );
                (0u64, 0u64)
            }
        }
    };

    // ── Virgl GPU Alpha Blend Context ─────────────────────────────────────────
    // Attempt to initialize the virgl 3D rendering context and DMA staging
    // buffer for GPU-backed alpha blending.  Failures are non-fatal; the
    // driver falls back to the CPU blend path.
    let (
        virgl_ctx_id,
        virgl_blend_staging_buf,
        virgl_blend_staging_phys,
        virgl_blend_staging_size,
        virgl_src_res_id,
    ) = init_virgl_blend(&mut gpu, disp_width, disp_height, &frame_pool_buffers);
    if virgl_ctx_id != 0 {
        info!(
            "display_virtio_gpu: composition paths: gpu_opaque_copy=enabled gpu_alpha_blend=enabled cpu_alpha_blend=enabled"
        );
        info!(
            "display_virtio_gpu: accel2d_gpu=enabled (COPY_RECT and ALPHA_BLIT dispatched to virgl GPU pipeline)"
        );
    }

    // =========================================================================
    // SOVEREIGN REGISTRATION: Handshake with sprout supervisor
    // =========================================================================
    use abi::supervisor_protocol::{self, classes};

    // Create VFS provider port
    let (vfs_write, vfs_read) =
        port_create(VFS_RPC_MAX_REQ * 8).expect("Failed to create VFS port");
    let vfs_write_fd = stem::syscall::vfs::vfs_handle_from_port(vfs_write)
        .expect("display_virtio_gpu: vfs_handle_from_port(vfs_write)");

    let mut buf = [0u8; 512];
    let mut frames = FrameReader::<4096>::new();

    let mut stats = PresentStats::new(true);
    const STATS_LOG_INTERVAL: u32 = 120;

    // Texture registry for 3D textures (client_id → TextureEntry)
    let mut texture_registry: alloc::collections::BTreeMap<u64, TextureEntry> =
        alloc::collections::BTreeMap::new();

    if cambium_direct_mount {
        match stem::syscall::vfs::vfs_mount(vfs_write, "/dev/display/card0") {
            Ok(()) => {
                info!("display_virtio_gpu: mounted VFS provider at /dev/display/card0 via cambium")
            }
            Err(e) => {
                error!("display_virtio_gpu: vfs_mount(/dev/display/card0) failed: {:?}", e);
                loop {
                    stem::yield_now();
                }
            }
        }
    } else {
        let sprout_pid = stem::syscall::getppid();
        let sprout_inbox_fd =
            msg_inbox_open(sprout_pid).expect("display_virtio_gpu: failed to open sprout inbox");

        // Send MSG_BIND_READY to supervisor instead of legacy MSG_REGISTER
        let ready = supervisor_protocol::BindReadyPayload {
            bind_instance_id,
            class_mask: classes::DISPLAY_CARD | classes::FRAMEBUFFER,
            _reserved: 0,
        };
        let mut ready_bytes = [0u8; supervisor_protocol::BIND_READY_PAYLOAD_SIZE];
        if let Some(len) = supervisor_protocol::encode_bind_ready_le(&ready, &mut ready_bytes) {
            // Wrap in common driver header
            let mut ready_buf = [0u8; 256];
            if let Some(total_len) = drvproto::encode_message(
                &mut ready_buf,
                supervisor_protocol::MSG_BIND_READY,
                &ready_bytes[..len],
            ) {
                // Bundle the VFS provider handle and BIND_READY notification atomically.
                let _ = msg_sendmsg(
                    sprout_inbox_fd,
                    KindId(drvproto::KIND_ID_DISPLAY_DRIVER_CONTROL),
                    &ready_buf[..total_len],
                    &[vfs_write_fd],
                );
                debug!("display_virtio_gpu: Sent MSG_BIND_READY (ID: {})", bind_instance_id);
            }
        }

        // Wait for MSG_BIND_ASSIGNED or MSG_BIND_FAILED
        let mut loop_count = 0;
        debug!("display_virtio_gpu: Waiting for BIND_ASSIGNED...");
        let assigned_bind_id = loop {
            loop_count += 1;
            if loop_count % 100 == 0 {
                debug!(
                    "display_virtio_gpu: Still waiting for BIND_ASSIGNED (loop={})...",
                    loop_count
                );
            }
            let msg = msg_recv_blocking(512);
            if msg.kind.0 != drvproto::KIND_ID_DISPLAY_DRIVER_CONTROL {
                continue;
            }
            if let Some((header, payload)) = drvproto::parse_message(&msg.payload) {
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
        };

        // Notify supervisor that this service is now fully operational.
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
                let _ = msg_sendmsg(
                    sprout_inbox_fd,
                    KindId(drvproto::KIND_ID_DISPLAY_DRIVER_CONTROL),
                    &svc_buf[..total_len],
                    &[],
                );
                debug!("display_virtio_gpu: Sent MSG_SERVICE_READY.");
            }
        }
    }

    let drv_req_fd = stem::syscall::vfs::vfs_handle_from_port(drv_req_read)
        .expect("display_virtio_gpu: fd_from_handle(drv_req_read)");
    let drv_resp_write_fd = stem::syscall::vfs::vfs_handle_from_port(drv_resp_write)
        .expect("display_virtio_gpu: fd_from_handle(drv_resp_write)");
    let vfs_read_fd = stem::syscall::vfs::vfs_handle_from_port(vfs_read)
        .expect("display_virtio_gpu: fd_from_handle(vfs_read)");
    let mut ws = stem::wait_set::WaitSet::new();
    let drv_req_read_tok = ws.add_fd_readable(drv_req_fd).unwrap();
    let vfs_read_tok = ws.add_fd_readable(vfs_read_fd).unwrap();

    // Consolidate all mutable driver state into VirtioGpuDriver so that
    // dispatch_vfs_rpc can access it through a single &mut reference — the
    // same "one class" pattern used by display_bootfb's BootFbDriver.
    let mut driver = VirtioGpuDriver {
        gpu,
        disp_width,
        disp_height,
        disp_stride,
        disp_format,
        frame_pool: frame_pool_buffers,
        next_buffer_idx: 0,
        present_seq: 0,
        last_presented_idx: None,
        imported_buffers: alloc::collections::BTreeMap::new(),
        next_import_id: 1,
        current_fd: None,
        current_res_id: driver_initial_res_id,
        first_commit_logged: false,
        cursor_commit_logged: false,
        cursor_resource_id: 0,
        cursor_buffer_id: None,
        cursor_hotspot: (0, 0),
        cursor_pos: (0, 0),
        cursor_visible: false,
        cursor_dma_buf,
        cursor_dma_phys,
        cursor_attached_size: (0, 0),
        gpu_path_planes: 0,
        cpu_fallback_planes: 0,
        virgl_ctx_id,
        virgl_blend_staging_buf,
        virgl_blend_staging_phys,
        virgl_blend_staging_size,
        virgl_src_res_id,
        accel2d_gpu_cmds: 0,
        accel2d_cpu_cmds: 0,
        display_rpc_seq: 0,
        rpc_diag: DisplayRpcDiag::new(),
        total_imports: 0,
        total_commits: 0,
        failed_imports: 0,
        failed_commits: 0,
        last_damage_rect_count: 0,
        last_damage_area: 0,
    };

    // ProviderLoop handles VFS RPC framing and correctly prefixes every
    // response with the req_id the kernel needs to route the reply.
    let mut vfs_loop = ProviderLoop::new(vfs_read);
    let mut last_watchdog_ns = stem::time::monotonic_ns();
    info!("display_virtio_gpu: VFS provider loop online");

    loop {
        let mut did_work = false;

        if DISPLAY_PROVIDER_POLL_ISOLATION {
            let mut read_total = 0;
            loop {
                match stem::syscall::port_try_recv(drv_req_read, &mut buf) {
                    Ok(n) => {
                        if n == 0 {
                            break;
                        }
                        frames.push(&buf[..n]);
                        read_total += n;
                    }
                    Err(abi::errors::Errno::EAGAIN) => break,
                    Err(e) => {
                        stem::error!("display_virtio_gpu: port_try_recv ERR: {:?}", e);
                        break;
                    }
                }
            }
            if read_total > 0 {
                did_work = true;
                stem::trace!(
                    "display_virtio_gpu: poll read {} bytes, dropped={}",
                    read_total,
                    frames.dropped_bytes()
                );
            }
        } else {
            stem::trace!("display_virtio_gpu: waiting on WaitSet...");
            match ws.wait(Some(core::time::Duration::from_millis(10))) {
                Ok(events) => {
                    if !events.is_empty() {
                        did_work = true;
                    }
                    for ev in events {
                        if ev.token() == drv_req_read_tok && ev.is_readable() {
                            stem::trace!("display_virtio_gpu: drv_req readable token fired");
                        }
                        if ev.token() == vfs_read_tok && ev.is_readable() {
                            stem::trace!("display_virtio_gpu: vfs_read readable token fired");
                        }
                    }

                    let mut read_total = 0;
                    // Drain with non-blocking receives only. A blocking recv here can
                    // starve VFS RPC handling and wedge /dev/display/card0 clients.
                    loop {
                        match stem::syscall::port_try_recv(drv_req_read, &mut buf) {
                            Ok(n) => {
                                if n == 0 {
                                    break;
                                }
                                frames.push(&buf[..n]);
                                read_total += n;
                            }
                            Err(abi::errors::Errno::EAGAIN) => break,
                            Err(e) => {
                                stem::error!("display_virtio_gpu: port_try_recv ERR: {:?}", e);
                                break;
                            }
                        }
                    }
                    if read_total > 0 {
                        did_work = true;
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
        }

        // Always drain VFS RPCs, even if readiness wait fails or times out.
        while let Ok(Some(req)) = vfs_loop.try_next_request() {
            did_work = true;
            stem::trace!("display_virtio_gpu: VFS RPC op={:?}", req.op);
            let resp = dispatch_vfs_rpc(&mut driver, &req);
            stem::trace!(
                "display_virtio_gpu: VFS RPC response begin op={:?} req_id={}",
                req.op,
                req.req_id
            );
            match vfs_loop.send_response(&req, resp) {
                Ok(()) => stem::trace!(
                    "display_virtio_gpu: VFS RPC response sent op={:?} req_id={}",
                    req.op,
                    req.req_id
                ),
                Err(e) => stem::error!(
                    "display_virtio_gpu: VFS RPC response failed op={:?} req_id={} err={:?}",
                    req.op,
                    req.req_id,
                    e
                ),
            }
        }

        while let Some((header, payload)) = frames.next_message() {
            did_work = true;
            stem::trace!(
                "display_virtio_gpu: next_message -> msg_type={}, len={}",
                header.msg_type,
                payload.len()
            );
            match header.msg_type {
                drvproto::MSG_HELLO => {
                    debug!("display_virtio_gpu: received MSG_HELLO");
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
                    let _ = refresh_display_mode(&mut driver);
                    stem::debug!("display_virtio_gpu: received MSG_ACQUIRE");
                    let mut buffer_age = 0;
                    let idx = driver.next_buffer_idx;

                    if driver.last_presented_idx.is_some() {
                        let age = driver
                            .present_seq
                            .saturating_sub(driver.frame_pool[idx].last_present_seq);
                        buffer_age = if driver.frame_pool[idx].last_present_seq == 0 {
                            0 // Never presented
                        } else {
                            age as u32
                        };
                    }

                    driver.next_buffer_idx = (driver.next_buffer_idx + 1) % driver.frame_pool.len();

                    let acquired = drvproto::AcquiredPayload {
                        handle: driver.frame_pool[idx].fd,
                        _pad1: 0,
                        width: driver.disp_width,
                        height: driver.disp_height,
                        stride: driver.disp_stride,
                        format: driver.disp_format,
                        buffer_age,
                        _pad2: 0,
                    };

                    driver.current_fd = Some(driver.frame_pool[idx].fd);
                    driver.current_res_id = driver.frame_pool[idx].res_id;

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
                                &[driver.frame_pool[idx].fd],
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
                        driver.current_fd = Some(fd);
                        // In legacy mode, we just stay on the first buffer's resource
                        driver.current_res_id = driver.frame_pool[0].res_id;
                        send_msg(drv_resp_write, drvproto::MSG_ACK, &[]);
                    }
                }
                drvproto::MSG_PRESENT => {
                    let _ = refresh_display_mode(&mut driver);
                    if driver.current_fd.is_none() {
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
                            let full_rect =
                                Rect { x: 0, y: 0, w: driver.disp_width, h: driver.disp_height };
                            stem::trace!(
                                "display_virtio_gpu: calling present_rect for full_rect..."
                            );
                            let _ = driver.gpu.present_rect(driver.current_res_id, full_rect);
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
                                    let clamped = rect_clamp_to_bounds(
                                        gpu_rect,
                                        driver.disp_width,
                                        driver.disp_height,
                                    );
                                    if !rect_is_empty(clamped) {
                                        valid_rects.push(clamped);
                                    }
                                }
                            }

                            stats.total_rects_in += valid_rects.len() as u32;

                            if !valid_rects.is_empty() {
                                // Phase 2: Transfer all rects (bandwidth follows true damage)
                                for &rect in &valid_rects {
                                    let _ = driver.gpu.transfer_to_host_with_stride(
                                        driver.current_res_id,
                                        rect,
                                        driver.disp_stride,
                                    );
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
                                        let _ =
                                            driver.gpu.flush_resource(driver.current_res_id, rect);
                                    }
                                    stats.total_flushes += valid_rects.len() as u32;
                                    stats.per_rect_flush_count += 1;
                                } else {
                                    // Common case: single union flush
                                    let _ = driver
                                        .gpu
                                        .flush_resource(driver.current_res_id, union_rect);
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
                        let _ = driver.gpu.set_scanout(
                            driver.current_res_id,
                            driver.disp_width,
                            driver.disp_height,
                        );

                        // Update sequence and age bookkeeping
                        driver.present_seq += 1;
                        let mut presented_idx = 0;
                        for (i, buf) in driver.frame_pool.iter_mut().enumerate() {
                            if buf.res_id == driver.current_res_id {
                                buf.last_present_seq = driver.present_seq;
                                presented_idx = i;
                                break;
                            }
                        }
                        driver.last_presented_idx = Some(presented_idx);

                        // Rate-limited stats logging
                        if stats.frame_count >= STATS_LOG_INTERVAL {
                            stats.log_and_reset(driver.gpu_path_planes, driver.cpu_fallback_planes);
                            stats.log_and_reset_accel2d(
                                driver.accel2d_gpu_cmds,
                                driver.accel2d_cpu_cmds,
                            );
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
                            match driver.gpu.submit_3d(hdr.ctx_id, &cmd_buf[..hdr.cmd_len as usize])
                            {
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

                        let result = driver.gpu.create_resource_3d(
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
                            if driver.gpu.ctx_attach_resource(ctx_id, resource_id).is_ok() {
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
                                                    if driver
                                                        .gpu
                                                        .attach_backing_3d(
                                                            hdr.resource_id,
                                                            phys_addr,
                                                            hdr.data_len as usize,
                                                        )
                                                        .is_ok()
                                                    {
                                                        if driver
                                                            .gpu
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

        if DISPLAY_PROVIDER_POLL_ISOLATION && !did_work {
            stem::sleep_ms(1);
        }
        maybe_log_display_watchdog(&driver, &mut last_watchdog_ns, vfs_loop.pending_len());
    }
}
