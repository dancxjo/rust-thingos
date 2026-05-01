//! VirtIO GPU driver library
//!
//! This module provides a reusable VirtioGpu driver that can be used by
//! display_virtio_gpu or as a standalone program.
#![no_std]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;
use core::cmp::Ord;
use core::iter::Iterator;
use core::ptr::{read_volatile, write_volatile};

use abi::errors::Errno;

pub mod commands;
pub mod virtio;
pub mod virtqueue;

pub use commands::*;
pub use virtqueue::Virtqueue;

/// Rectangle for partial updates
#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

const DMA_PAGE_SIZE: usize = 4096;
const COMMAND_TIMEOUT_NS: u64 = 500_000_000;
const COMMAND_SLEEP_NS: u64 = 50_000;
/// Alignment requirement for virtio descriptor buffers per the virtio spec
/// (§2.7.2 Virtqueue Descriptor Format) and QEMU's DMA engine.
const VIRTIO_DESCRIPTOR_ALIGNMENT: usize = 16;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DisplayScanout {
    pub width: u32,
    pub height: u32,
    pub enabled: bool,
}

/// Virtio GPU driver state
pub struct VirtioGpu {
    claim_handle: usize,

    // MMIO regions
    common_cfg: u64,
    notify_cfg: u64,
    notify_off_multiplier: u32,

    // Virtqueues
    controlq: Option<Virtqueue>,
    controlq_notify_off: u16,
    controlq_faulted: bool,

    /// Cursor virtqueue (queue index 1). Optional: some QEMU configurations
    /// may not expose a cursor queue.
    cursorq: Option<Virtqueue>,
    cursorq_notify_off: u16,
    /// Separate DMA buffer used exclusively for cursor queue commands so
    /// cursor traffic does not interfere with ongoing controlq transactions.
    cursor_cmd_buf: u64,
    cursor_cmd_buf_phys: u64,

    // Display info
    display_width: u32,
    display_height: u32,

    // Resource tracking
    framebuffer: u64, // Virtual address of framebuffer (for standalone mode)
    fb_phys: u64,     // Physical address
    fb_size: usize,   // Size in bytes
    fb_stride: u32,   // Stride in bytes for offset calculation

    // Command buffer for sending commands
    cmd_buf: u64,      // Virtual address
    cmd_buf_phys: u64, // Physical address

    // Track if using external shared memory (display driver mode)
    external_backing: bool,

    // Virgl 3D support
    // Virgl 3D support
    virgl_supported: bool,
    next_resource_id: u32,
    sysfs_path: alloc::string::String,
}

impl VirtioGpu {
    /// Create a new VirtioGpu driver instance
    pub fn new(sysfs_path: &str) -> Result<Self, Errno> {
        use stem::syscall::{device_alloc_dma, device_claim, device_dma_phys, device_map_mmio};

        // Claim the device using its sysfs path as the primary key.
        let claim_handle = device_claim(sysfs_path)?;

        // Read VirtIO capability offsets from sysfs
        let common_bar =
            read_sys_u32(&alloc::format!("{}/virtio/common_bar", sysfs_path)).unwrap_or(0) as usize;
        let common_offset = read_sys_u32(&alloc::format!("{}/virtio/common_offset", sysfs_path))
            .unwrap_or(0) as u64;
        let notify_bar =
            read_sys_u32(&alloc::format!("{}/virtio/notify_bar", sysfs_path)).unwrap_or(0) as usize;
        let notify_offset = read_sys_u32(&alloc::format!("{}/virtio/notify_offset", sysfs_path))
            .unwrap_or(0) as u64;
        let notify_multiplier =
            read_sys_u32(&alloc::format!("{}/virtio/notify_multiplier", sysfs_path)).unwrap_or(4);

        stem::info!(
            "virtio_gpu: caps from sysfs - common BAR{} off=0x{:x}, notify BAR{} off=0x{:x} mult={}",
            common_bar,
            common_offset,
            notify_bar,
            notify_offset,
            notify_multiplier
        );

        // Map the BAR containing common config
        let common_bar_base = device_map_mmio(claim_handle, common_bar)?;
        let common_cfg = common_bar_base + common_offset;

        // Map notify BAR (may be same as common BAR)
        let notify_cfg = if notify_bar == common_bar {
            common_bar_base + notify_offset
        } else {
            let notify_bar_base = device_map_mmio(claim_handle, notify_bar)?;
            notify_bar_base + notify_offset
        };

        // Allocate command buffer (1 page for commands + responses)
        let cmd_buf = device_alloc_dma(claim_handle, 1).map_err(|_| Errno::ENOMEM)?;
        let cmd_buf_phys = device_dma_phys(cmd_buf).map_err(|_| Errno::EFAULT)?;

        // Allocate a separate DMA page for cursor queue commands so they don't
        // share storage with in-flight controlq transactions.
        let cursor_cmd_buf = device_alloc_dma(claim_handle, 1).unwrap_or_else(|e| {
            stem::warn!(
                "virtio_gpu: cursor command DMA alloc failed: {:?}; hw cursor unavailable",
                e
            );
            0
        });
        let cursor_cmd_buf_phys =
            if cursor_cmd_buf != 0 { device_dma_phys(cursor_cmd_buf).unwrap_or(0) } else { 0 };

        Ok(Self {
            claim_handle,
            common_cfg,
            notify_cfg,
            notify_off_multiplier: notify_multiplier,
            controlq: None,
            controlq_notify_off: 0,
            controlq_faulted: false,
            cursorq: None,
            cursorq_notify_off: 0,
            cursor_cmd_buf,
            cursor_cmd_buf_phys,
            display_width: 1024,
            display_height: 768,
            framebuffer: 0,
            fb_phys: 0,
            fb_size: 0,
            fb_stride: 0,
            cmd_buf,
            cmd_buf_phys,
            external_backing: false,
            virgl_supported: false,
            next_resource_id: 1,
            sysfs_path: alloc::string::String::from(sysfs_path),
        })
    }

    /// Initialize the virtio device
    pub fn init_virtio(&mut self) -> Result<(), &'static str> {
        // 1. Reset device
        self.write_common_u8(virtio::VIRTIO_COMMON_STATUS, 0);

        // 2. Set ACKNOWLEDGE status
        self.write_common_u8(virtio::VIRTIO_COMMON_STATUS, virtio::VIRTIO_STATUS_ACKNOWLEDGE as u8);

        // 3. Set DRIVER status
        let status = self.read_common_u8(virtio::VIRTIO_COMMON_STATUS);
        self.write_common_u8(
            virtio::VIRTIO_COMMON_STATUS,
            status | virtio::VIRTIO_STATUS_DRIVER as u8,
        );

        // 4. Read device features (feature bank 0 for GPU-specific features)
        self.write_common(virtio::VIRTIO_COMMON_DEVICE_FEATURE_SELECT, 0);
        let device_features = self.read_common(virtio::VIRTIO_COMMON_DEVICE_FEATURE);

        // Check for virgl 3D support
        self.virgl_supported = (device_features & (1 << virtio::VIRTIO_GPU_F_VIRGL)) != 0;
        stem::info!(
            "virtio_gpu: device features=0x{:08x} virgl={}",
            device_features,
            self.virgl_supported
        );

        // 5. Write driver features - request virgl if available
        self.write_common(virtio::VIRTIO_COMMON_DRIVER_FEATURE_SELECT, 0);
        let driver_features =
            if self.virgl_supported { 1 << virtio::VIRTIO_GPU_F_VIRGL } else { 0 };
        self.write_common(virtio::VIRTIO_COMMON_DRIVER_FEATURE, driver_features);

        // 6. Set FEATURES_OK
        let status = self.read_common_u8(virtio::VIRTIO_COMMON_STATUS);
        self.write_common_u8(
            virtio::VIRTIO_COMMON_STATUS,
            status | virtio::VIRTIO_STATUS_FEATURES_OK as u8,
        );

        // 7. Verify FEATURES_OK
        let status = self.read_common_u8(virtio::VIRTIO_COMMON_STATUS);
        if (status & virtio::VIRTIO_STATUS_FEATURES_OK as u8) == 0 {
            return Err("Features not accepted");
        }

        // 8. Setup virtqueues
        self.setup_controlq()?;
        // Queue 1 is the cursor queue; ignore failure — not all QEMU
        // configurations expose it, and display still works without it.
        let _ = self.setup_cursorq();

        // 9. Set DRIVER_OK
        let status = self.read_common_u8(virtio::VIRTIO_COMMON_STATUS);
        self.write_common_u8(
            virtio::VIRTIO_COMMON_STATUS,
            status | virtio::VIRTIO_STATUS_DRIVER_OK as u8,
        );

        Ok(())
    }

    /// Get display dimensions
    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.display_width, self.display_height)
    }

    /// Set display dimensions (must be called before creating resource)
    pub fn set_dimensions(&mut self, width: u32, height: u32) {
        self.display_width = width;
        self.display_height = height;
    }

    /// Query the host-visible scanout dimensions reported by virtio-gpu.
    pub fn query_display_info(&mut self) -> Result<Option<DisplayScanout>, &'static str> {
        let cmd = VirtioGpuCtrlHdr {
            type_: VIRTIO_GPU_CMD_GET_DISPLAY_INFO,
            flags: 0,
            fence_id: 0,
            ctx_id: 0,
            padding: 0,
        };
        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<VirtioGpuCtrlHdr>(),
            )
        };
        let resp_offset = align_up(cmd_bytes.len(), VIRTIO_DESCRIPTOR_ALIGNMENT);
        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuRespDisplayInfo>())?;

        let base = self.cmd_buf.saturating_add(resp_offset as u64);
        let pmodes_base = base.saturating_add(core::mem::size_of::<VirtioGpuCtrlHdr>() as u64);
        let mode_size = core::mem::size_of::<VirtioGpuDisplayOne>() as u64;
        let mut first_active = None;
        let mut first_nonzero = None;
        for i in 0..16u64 {
            let mode = pmodes_base.saturating_add(i.saturating_mul(mode_size));
            let width = read_dma_u32(mode.saturating_add(8));
            let height = read_dma_u32(mode.saturating_add(12));
            let enabled = read_dma_u32(mode.saturating_add(16)) != 0;
            if width == 0 || height == 0 {
                continue;
            }
            let scanout = DisplayScanout { width, height, enabled };
            if first_nonzero.is_none() {
                first_nonzero = Some(scanout);
            }
            if enabled {
                first_active = Some(scanout);
                break;
            }
        }

        if let Some(scanout) = first_active.or(first_nonzero) {
            self.display_width = scanout.width;
            self.display_height = scanout.height;
            Ok(Some(scanout))
        } else {
            Ok(None)
        }
    }

    /// Create a 2D resource with current dimensions
    pub fn create_resource_2d(&mut self, resource_id: u32) -> Result<(), &'static str> {
        self.create_resource_2d_sized_with_format(
            resource_id,
            self.display_width,
            self.display_height,
            VIRTIO_GPU_FORMAT_B8G8R8X8_UNORM,
        )
    }

    /// Create a 2D resource with current dimensions and an explicit virtio-gpu format.
    pub fn create_resource_2d_with_format(
        &mut self,
        resource_id: u32,
        format: u32,
    ) -> Result<(), &'static str> {
        self.create_resource_2d_sized_with_format(
            resource_id,
            self.display_width,
            self.display_height,
            format,
        )
    }

    /// Create a 2D resource with explicit dimensions and format.
    pub fn create_resource_2d_sized_with_format(
        &mut self,
        resource_id: u32,
        width: u32,
        height: u32,
        format: u32,
    ) -> Result<(), &'static str> {
        let cmd = VirtioGpuResourceCreate2d {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_RESOURCE_CREATE_2D,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            resource_id,
            format,
            width,
            height,
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<VirtioGpuResourceCreate2d>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    pub fn resource_unref(&mut self, resource_id: u32) -> Result<(), &'static str> {
        let cmd = VirtioGpuResourceUnref {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_RESOURCE_UNREF,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            resource_id,
            padding: 0,
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<VirtioGpuResourceUnref>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    /// Attach backing memory to the resource
    pub fn attach_backing(
        &mut self,
        resource_id: u32,
        phys_addr: u64,
        size: usize,
        stride: u32,
    ) -> Result<(), &'static str> {
        self.fb_phys = phys_addr;
        self.fb_size = size;
        self.fb_stride = stride;
        self.external_backing = true;

        self.attach_backing_preserve_state(resource_id, phys_addr, size)
    }

    /// Attach backing memory without modifying the helper's scanout bookkeeping.
    pub fn attach_backing_preserve_state(
        &mut self,
        resource_id: u32,
        phys_addr: u64,
        size: usize,
    ) -> Result<(), &'static str> {
        // Need to send header + 1 memory entry
        #[repr(C, packed)]
        struct AttachCmd {
            hdr: VirtioGpuCtrlHdr,
            resource_id: u32,
            nr_entries: u32,
            entry: VirtioGpuMemEntry,
        }

        let cmd = AttachCmd {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_RESOURCE_ATTACH_BACKING,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            resource_id,
            nr_entries: 1,
            entry: VirtioGpuMemEntry { addr: phys_addr, length: size as u32, padding: 0 },
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<AttachCmd>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    /// Set the scanout to use the resource
    pub fn set_scanout(
        &mut self,
        resource_id: u32,
        width: u32,
        height: u32,
    ) -> Result<(), &'static str> {
        let cmd = VirtioGpuSetScanout {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_SET_SCANOUT,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            r_x: 0,
            r_y: 0,
            r_width: width,
            r_height: height,
            scanout_id: 0,
            resource_id,
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<VirtioGpuSetScanout>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    /// Transfer a rectangle from backing memory to host
    pub fn transfer_to_host(&mut self, resource_id: u32, rect: Rect) -> Result<(), &'static str> {
        self.transfer_to_host_with_stride(resource_id, rect, self.fb_stride)
    }

    /// Transfer a rectangle from backing memory to host using the backing stride for this resource.
    pub fn transfer_to_host_with_stride(
        &mut self,
        resource_id: u32,
        rect: Rect,
        stride: u32,
    ) -> Result<(), &'static str> {
        // Calculate byte offset into backing memory for this rectangle
        // Format is BGRA32 (4 bytes per pixel)
        const BPP: u32 = 4;
        let offset = (rect.y as u64) * (stride as u64) + (rect.x as u64) * (BPP as u64);

        let cmd = VirtioGpuTransferToHost2d {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_TRANSFER_TO_HOST_2D,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            r_x: rect.x,
            r_y: rect.y,
            r_width: rect.w,
            r_height: rect.h,
            offset,
            resource_id,
            padding: 0,
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<VirtioGpuTransferToHost2d>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    /// Flush a rectangle to display
    pub fn flush_resource(&mut self, resource_id: u32, rect: Rect) -> Result<(), &'static str> {
        let cmd = VirtioGpuResourceFlush {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_RESOURCE_FLUSH,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            r_x: rect.x,
            r_y: rect.y,
            r_width: rect.w,
            r_height: rect.h,
            resource_id,
            padding: 0,
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<VirtioGpuResourceFlush>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    /// Transfer and flush a rectangle (convenience method)
    pub fn present_rect(&mut self, resource_id: u32, rect: Rect) -> Result<(), &'static str> {
        self.transfer_to_host(resource_id, rect)?;
        self.flush_resource(resource_id, rect)
    }

    /// Flush the union of multiple rectangles as a single operation.
    ///
    /// Computes the bounding box (union) of all provided rects, clamps it to bounds,
    /// and issues a single flush command. Returns Ok(()) even if rects is empty.
    pub fn flush_union(
        &mut self,
        resource_id: u32,
        rects: &[Rect],
        bounds: (u32, u32),
    ) -> Result<(), &'static str> {
        if rects.is_empty() {
            return Ok(());
        }

        // Compute union of all rects
        let mut union = rects[0];
        for &r in &rects[1..] {
            let x1 = union.x.min(r.x);
            let y1 = union.y.min(r.y);
            let x2 = (union.x + union.w).max(r.x + r.w);
            let y2 = (union.y + union.h).max(r.y + r.h);
            union = Rect { x: x1, y: y1, w: x2.saturating_sub(x1), h: y2.saturating_sub(y1) };
        }

        // Clamp to bounds
        let x = union.x.min(bounds.0);
        let y = union.y.min(bounds.1);
        let max_w = bounds.0.saturating_sub(x);
        let max_h = bounds.1.saturating_sub(y);
        let clamped = Rect { x, y, w: union.w.min(max_w), h: union.h.min(max_h) };

        // Skip empty rect
        if clamped.w == 0 || clamped.h == 0 {
            return Ok(());
        }

        self.flush_resource(resource_id, clamped)
    }

    // =========================================================================
    // Virgl 3D methods
    // =========================================================================

    /// Check if 3D (virgl) rendering is supported
    pub fn has_3d_feature(&self) -> bool {
        self.virgl_supported
    }

    /// Allocate a new unique resource ID
    pub fn alloc_resource_id(&mut self) -> u32 {
        let id = self.next_resource_id;
        self.next_resource_id += 1;
        id
    }

    /// Create a 3D rendering context
    pub fn create_context(&mut self, ctx_id: u32, debug_name: &[u8]) -> Result<(), &'static str> {
        if !self.virgl_supported {
            return Err("Virgl not supported");
        }

        let mut name_buf = [0u8; 64];
        let copy_len = debug_name.len().min(64);
        name_buf[..copy_len].copy_from_slice(&debug_name[..copy_len]);

        let cmd = VirtioGpuCtxCreate {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_CTX_CREATE,
                flags: 0,
                fence_id: 0,
                ctx_id,
                padding: 0,
            },
            nlen: copy_len as u32,
            context_init: 0, // Use default capset
            debug_name: name_buf,
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<VirtioGpuCtxCreate>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    /// Destroy a 3D rendering context
    pub fn destroy_context(&mut self, ctx_id: u32) -> Result<(), &'static str> {
        let cmd = VirtioGpuCtxDestroy {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_CTX_DESTROY,
                flags: 0,
                fence_id: 0,
                ctx_id,
                padding: 0,
            },
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<VirtioGpuCtxDestroy>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    /// Attach a resource to a 3D context
    pub fn ctx_attach_resource(
        &mut self,
        ctx_id: u32,
        resource_id: u32,
    ) -> Result<(), &'static str> {
        let cmd = VirtioGpuCtxResource {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_CTX_ATTACH_RESOURCE,
                flags: 0,
                fence_id: 0,
                ctx_id,
                padding: 0,
            },
            resource_id,
            padding: 0,
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<VirtioGpuCtxResource>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    /// Submit a virgl command buffer for 3D execution
    pub fn submit_3d(&mut self, ctx_id: u32, commands: &[u8]) -> Result<(), &'static str> {
        if !self.virgl_supported {
            return Err("Virgl not supported");
        }

        // Build command header + command data in cmd_buf
        let header = VirtioGpuCmdSubmit3d {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_SUBMIT_3D,
                flags: 0,
                fence_id: 0,
                ctx_id,
                padding: 0,
            },
            size: commands.len() as u32,
            padding: 0,
        };

        let header_size = core::mem::size_of::<VirtioGpuCmdSubmit3d>();
        let total_size = header_size + commands.len();
        let resp_offset = ((total_size + 15) / 16) * 16;
        let resp_size = core::mem::size_of::<VirtioGpuCtrlHdr>();
        if resp_offset.saturating_add(resp_size) > DMA_PAGE_SIZE {
            return Err("Submit 3D command too large");
        }
        if self.controlq_faulted {
            return Err("Control queue faulted");
        }

        // Copy header and command data to DMA buffer
        let cmd_ptr = self.cmd_buf as *mut u8;
        let resp_ptr = (self.cmd_buf + resp_offset as u64) as *mut u8;
        unsafe {
            // Write header
            let header_bytes =
                core::slice::from_raw_parts(&header as *const _ as *const u8, header_size);
            for (i, byte) in header_bytes.iter().enumerate() {
                core::ptr::write_volatile(cmd_ptr.add(i), *byte);
            }
            // Write command payload
            for (i, byte) in commands.iter().enumerate() {
                core::ptr::write_volatile(cmd_ptr.add(header_size + i), *byte);
            }
            for i in 0..resp_size {
                core::ptr::write_volatile(resp_ptr.add(i), 0);
            }
        }

        let resp_phys = self.cmd_buf_phys + resp_offset as u64;

        // Send via virtqueue
        {
            let vq = self.controlq.as_mut().ok_or("No controlq")?;
            let bufs = [
                (self.cmd_buf_phys, total_size as u32, false),
                (resp_phys, resp_size as u32, true),
            ];
            vq.add_buffer(&bufs).ok_or("Queue full")?;
        }

        self.notify_queue(0);
        self.wait_for_response(resp_offset, VIRTIO_GPU_CMD_SUBMIT_3D, "Submit 3D command")
    }

    /// Create a 3D resource (texture, render target, etc.)
    pub fn create_resource_3d(
        &mut self,
        resource_id: u32,
        target: u32,
        format: u32,
        bind: u32,
        width: u32,
        height: u32,
        depth: u32,
    ) -> Result<(), &'static str> {
        let cmd = VirtioGpuResourceCreate3d {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_RESOURCE_CREATE_3D,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            resource_id,
            target,
            format,
            bind,
            width,
            height,
            depth,
            array_size: 1,
            last_level: 0,
            nr_samples: 0,
            flags: 0,
            padding: 0,
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<VirtioGpuResourceCreate3d>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    /// Attach backing memory to a 3D resource
    pub fn attach_backing_3d(
        &mut self,
        resource_id: u32,
        phys_addr: u64,
        size: usize,
    ) -> Result<(), &'static str> {
        // Same structure as 2D attach_backing
        #[repr(C, packed)]
        struct AttachCmd3d {
            hdr: VirtioGpuResourceAttachBacking,
            entry: VirtioGpuMemEntry,
        }

        let cmd = AttachCmd3d {
            hdr: VirtioGpuResourceAttachBacking {
                hdr: VirtioGpuCtrlHdr {
                    type_: VIRTIO_GPU_CMD_RESOURCE_ATTACH_BACKING,
                    flags: 0,
                    fence_id: 0,
                    ctx_id: 0,
                    padding: 0,
                },
                resource_id,
                nr_entries: 1,
            },
            entry: VirtioGpuMemEntry { addr: phys_addr, length: size as u32, padding: 0 },
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<AttachCmd3d>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    /// Transfer texture data from backing memory to 3D resource (texture upload)
    pub fn transfer_to_host_3d(
        &mut self,
        ctx_id: u32,
        resource_id: u32,
        width: u32,
        height: u32,
        offset: u64,
        stride: u32,
    ) -> Result<(), &'static str> {
        let cmd = VirtioGpuTransferToHost3d {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_TRANSFER_TO_HOST_3D,
                flags: 0,
                fence_id: 0,
                ctx_id,
                padding: 0,
            },
            box_x: 0,
            box_y: 0,
            box_z: 0,
            box_w: width,
            box_h: height,
            box_d: 1,
            offset,
            resource_id,
            level: 0,
            stride,
            layer_stride: 0,
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<VirtioGpuTransferToHost3d>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    // === Internal methods ===

    fn setup_controlq(&mut self) -> Result<(), &'static str> {
        use stem::syscall::{device_alloc_dma, device_dma_phys};

        let vq_virt =
            device_alloc_dma(self.claim_handle, 4).map_err(|_| "Failed to alloc virtqueue")?;
        let vq_phys = device_dma_phys(vq_virt).map_err(|_| "Failed to get vq phys")?;

        let vq = Virtqueue::new(vq_virt, vq_phys, 128);

        // Configure the queue in device
        self.write_common_u16(virtio::VIRTIO_COMMON_QUEUE_SELECT, 0);
        self.write_common_u16(virtio::VIRTIO_COMMON_QUEUE_SIZE, 128);
        self.controlq_notify_off = self.read_common_u16(virtio::VIRTIO_COMMON_QUEUE_NOTIFY_OFF);

        // Write queue addresses
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_DESC_LO, (vq_phys & 0xFFFFFFFF) as u32);
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_DESC_HI, (vq_phys >> 32) as u32);

        let avail_offset = 128 * 16;
        let avail_phys = vq_phys + avail_offset as u64;
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_AVAIL_LO, (avail_phys & 0xFFFFFFFF) as u32);
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_AVAIL_HI, (avail_phys >> 32) as u32);

        let used_unaligned = avail_offset + 6 + 128 * 2;
        let used_offset = (used_unaligned + 3) & !3;
        let used_phys = vq_phys + used_offset as u64;
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_USED_LO, (used_phys & 0xFFFFFFFF) as u32);
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_USED_HI, (used_phys >> 32) as u32);

        // Enable the queue
        self.write_common_u16(virtio::VIRTIO_COMMON_QUEUE_ENABLE, 1);

        self.controlq = Some(vq);
        Ok(())
    }

    /// Set up the cursor virtqueue (queue index 1).
    ///
    /// Returns `Ok(())` on success, `Err` if the device does not support a
    /// cursor queue.  The result is intentionally ignored by `init_virtio`.
    fn setup_cursorq(&mut self) -> Result<(), &'static str> {
        use stem::syscall::{device_alloc_dma, device_dma_phys};

        // Only set up the cursor queue if we successfully allocated a dedicated
        // DMA buffer for it.
        if self.cursor_cmd_buf == 0 || self.cursor_cmd_buf_phys == 0 {
            return Err("No cursor DMA buffer");
        }

        let vq_virt =
            device_alloc_dma(self.claim_handle, 4).map_err(|_| "Failed to alloc cursor vq")?;
        let vq_phys = device_dma_phys(vq_virt).map_err(|_| "Failed to get cursor vq phys")?;

        let vq = Virtqueue::new(vq_virt, vq_phys, 16);

        // Select queue 1 (cursor queue)
        self.write_common_u16(virtio::VIRTIO_COMMON_QUEUE_SELECT, 1);

        // Check the device actually exposes a second queue.
        let max_size = self.read_common_u16(virtio::VIRTIO_COMMON_QUEUE_SIZE);
        if max_size == 0 {
            return Err("Device has no cursor queue");
        }

        let size = max_size.min(16);
        self.write_common_u16(virtio::VIRTIO_COMMON_QUEUE_SIZE, size);
        self.cursorq_notify_off = self.read_common_u16(virtio::VIRTIO_COMMON_QUEUE_NOTIFY_OFF);

        self.write_common(virtio::VIRTIO_COMMON_QUEUE_DESC_LO, (vq_phys & 0xFFFFFFFF) as u32);
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_DESC_HI, (vq_phys >> 32) as u32);

        // Descriptor table: `size` entries × 16 bytes each.
        let avail_offset = (size as usize) * 16;
        let avail_phys = vq_phys + avail_offset as u64;
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_AVAIL_LO, (avail_phys & 0xFFFFFFFF) as u32);
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_AVAIL_HI, (avail_phys >> 32) as u32);

        // Available ring: 2 bytes (flags) + 2 bytes (idx) = 6-byte header,
        // followed by `size` × 2-byte ring slots.  Align up to 4 bytes for
        // the used ring (virtio spec §2.7.2 alignment requirement).
        let used_unaligned = avail_offset + 6 + (size as usize) * 2;
        let used_offset = (used_unaligned + 3) & !3;
        let used_phys = vq_phys + used_offset as u64;
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_USED_LO, (used_phys & 0xFFFFFFFF) as u32);
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_USED_HI, (used_phys >> 32) as u32);

        self.write_common_u16(virtio::VIRTIO_COMMON_QUEUE_ENABLE, 1);

        self.cursorq = Some(vq);
        stem::info!("virtio_gpu: cursor queue (queue 1) configured");
        Ok(())
    }

    /// Returns `true` when the cursor virtqueue is available and ready.
    pub fn has_cursorq(&self) -> bool {
        self.cursorq.is_some()
    }

    /// Upload a new cursor image to the hardware cursor plane.
    ///
    /// - `resource_id`: ID of a 2D resource (created with
    ///   [`create_resource_2d`] and backed via [`attach_backing`]) that holds
    ///   the ARGB cursor pixels.  Pass `0` to hide the cursor.
    /// - `hot_x`, `hot_y`: hotspot within the image (pixels from top-left).
    /// - `pos_x`, `pos_y`: initial screen position of the hotspot.
    pub fn update_cursor(
        &mut self,
        resource_id: u32,
        hot_x: u32,
        hot_y: u32,
        pos_x: u32,
        pos_y: u32,
    ) -> Result<(), &'static str> {
        self.send_cursor_cmd(resource_id, hot_x, hot_y, pos_x, pos_y, false)
    }

    /// Move the hardware cursor hotspot to a new screen position.
    ///
    /// Faster than [`update_cursor`]: no image data is transferred.
    pub fn move_cursor_hw(&mut self, pos_x: u32, pos_y: u32) -> Result<(), &'static str> {
        self.send_cursor_cmd(0, 0, 0, pos_x, pos_y, true)
    }

    /// Send an `UPDATE_CURSOR` or `MOVE_CURSOR` command on the cursor queue.
    ///
    /// When `move_only` is true `VIRTIO_GPU_CMD_MOVE_CURSOR` is sent (only the
    /// position changes; `resource_id`/hotspot are ignored by the host).
    fn send_cursor_cmd(
        &mut self,
        resource_id: u32,
        hot_x: u32,
        hot_y: u32,
        pos_x: u32,
        pos_y: u32,
        move_only: bool,
    ) -> Result<(), &'static str> {
        if self.cursor_cmd_buf == 0 || self.cursor_cmd_buf_phys == 0 {
            return Err("No cursor DMA buffer");
        }

        let cmd_type =
            if move_only { VIRTIO_GPU_CMD_MOVE_CURSOR } else { VIRTIO_GPU_CMD_UPDATE_CURSOR };

        let cmd = VirtioGpuUpdateCursor {
            hdr: VirtioGpuCtrlHdr { type_: cmd_type, flags: 0, fence_id: 0, ctx_id: 0, padding: 0 },
            pos: VirtioGpuCursorPos { scanout_id: 0, x: pos_x, y: pos_y, padding: 0 },
            resource_id,
            hot_x,
            hot_y,
            padding: 0,
        };

        let cmd_size = core::mem::size_of::<VirtioGpuUpdateCursor>();
        let resp_size = core::mem::size_of::<VirtioGpuCtrlHdr>();
        // Align response offset per VIRTIO_DESCRIPTOR_ALIGNMENT.
        let resp_offset = ((cmd_size + VIRTIO_DESCRIPTOR_ALIGNMENT - 1)
            / VIRTIO_DESCRIPTOR_ALIGNMENT)
            * VIRTIO_DESCRIPTOR_ALIGNMENT;
        if resp_offset.saturating_add(resp_size) > DMA_PAGE_SIZE {
            return Err("Cursor command too large");
        }

        let cmd_bytes =
            unsafe { core::slice::from_raw_parts(&cmd as *const _ as *const u8, cmd_size) };

        let cmd_ptr = self.cursor_cmd_buf as *mut u8;
        let resp_ptr = (self.cursor_cmd_buf + resp_offset as u64) as *mut u8;
        unsafe {
            for (i, byte) in cmd_bytes.iter().enumerate() {
                write_volatile(cmd_ptr.add(i), *byte);
            }
            for i in 0..resp_size {
                write_volatile(resp_ptr.add(i), 0);
            }
        }

        let resp_phys = self.cursor_cmd_buf_phys + resp_offset as u64;

        // Drain any previously completed cursor descriptors so the queue does
        // not fill up over time.
        if let Some(vq) = self.cursorq.as_mut() {
            while vq.poll_used().is_some() {}
        }

        {
            let vq = self.cursorq.as_mut().ok_or("No cursorq")?;
            let bufs = [
                (self.cursor_cmd_buf_phys, cmd_size as u32, false),
                (resp_phys, resp_size as u32, true),
            ];
            vq.add_buffer(&bufs).ok_or("Cursor queue full")?;
        }

        // Notify cursor queue (queue index 1).
        let notify_off = self.cursorq_notify_off;
        let notify_addr = self.notify_cfg + (notify_off as u64 * self.notify_off_multiplier as u64);
        unsafe { write_volatile(notify_addr as *mut u16, 1u16) }

        Ok(())
    }

    fn read_common(&self, offset: u32) -> u32 {
        unsafe { read_volatile((self.common_cfg + offset as u64) as *const u32) }
    }

    fn write_common(&self, offset: u32, value: u32) {
        unsafe { write_volatile((self.common_cfg + offset as u64) as *mut u32, value) }
    }

    fn read_common_u8(&self, offset: u32) -> u8 {
        unsafe { read_volatile((self.common_cfg + offset as u64) as *const u8) }
    }

    fn write_common_u8(&self, offset: u32, value: u8) {
        unsafe { write_volatile((self.common_cfg + offset as u64) as *mut u8, value) }
    }

    fn read_common_u16(&self, offset: u32) -> u16 {
        unsafe { read_volatile((self.common_cfg + offset as u64) as *const u16) }
    }

    fn write_common_u16(&self, offset: u32, value: u16) {
        unsafe { write_volatile((self.common_cfg + offset as u64) as *mut u16, value) }
    }

    fn notify_queue(&self, queue_idx: u16) {
        // Write queue index to notify register
        let notify_off =
            if queue_idx == 0 { self.controlq_notify_off } else { self.cursorq_notify_off };
        let notify_addr = self.notify_cfg + (notify_off as u64 * self.notify_off_multiplier as u64);
        unsafe { write_volatile(notify_addr as *mut u16, queue_idx) }
    }

    /// Send a command and wait for response
    fn send_cmd(&mut self, cmd: &[u8], resp_size: usize) -> Result<(), &'static str> {
        if self.controlq_faulted {
            return Err("Control queue faulted");
        }
        let cmd_type = unsafe { *(cmd.as_ptr() as *const u32) };
        // stem::info!("VirtioGpu: sending cmd type=0x{:x}", cmd_type);

        let resp_offset = ((cmd.len() + VIRTIO_DESCRIPTOR_ALIGNMENT - 1)
            / VIRTIO_DESCRIPTOR_ALIGNMENT)
            * VIRTIO_DESCRIPTOR_ALIGNMENT;
        if resp_offset.saturating_add(resp_size) > DMA_PAGE_SIZE {
            return Err("Command too large");
        }

        // Copy command to DMA buffer
        let cmd_ptr = self.cmd_buf as *mut u8;
        let resp_ptr = (self.cmd_buf + resp_offset as u64) as *mut u8;
        unsafe {
            for (i, byte) in cmd.iter().enumerate() {
                write_volatile(cmd_ptr.add(i), *byte);
            }
            for i in 0..resp_size {
                write_volatile(resp_ptr.add(i), 0);
            }
        }

        // Response goes after command
        let resp_phys = self.cmd_buf_phys + resp_offset as u64;

        // Get mutable ref to controlq, add buffer, then release borrow
        {
            let vq = self.controlq.as_mut().ok_or("No controlq")?;

            // Add buffer chain: command (read by device), response (written by device)
            let bufs =
                [(self.cmd_buf_phys, cmd.len() as u32, false), (resp_phys, resp_size as u32, true)];

            vq.add_buffer(&bufs).ok_or("Queue full")?;
        }

        // Notify device (no borrow conflict now)
        self.notify_queue(0);
        self.wait_for_response(resp_offset, cmd_type, "Command")
    }

    fn wait_for_response(
        &mut self,
        resp_offset: usize,
        cmd_type: u32,
        label: &'static str,
    ) -> Result<(), &'static str> {
        let start = stem::time::monotonic_ns();
        let deadline = start.saturating_add(COMMAND_TIMEOUT_NS);
        let mut polls = 0u32;

        loop {
            let completed = {
                let vq = self.controlq.as_mut().ok_or("No controlq")?;
                vq.poll_used().is_some()
            };

            if completed {
                // Read response type safely from packed struct
                let resp_ptr = (self.cmd_buf + resp_offset as u64) as *const u8;
                let resp_type = unsafe {
                    let type_bytes: [u8; 4] = [
                        read_volatile(resp_ptr),
                        read_volatile(resp_ptr.add(1)),
                        read_volatile(resp_ptr.add(2)),
                        read_volatile(resp_ptr.add(3)),
                    ];
                    u32::from_le_bytes(type_bytes)
                };

                if is_ok_response(resp_type) {
                    return Ok(());
                } else {
                    stem::error!(
                        "VirtioGpu: {} 0x{:x} failed with resp_type=0x{:x}",
                        label,
                        cmd_type,
                        resp_type
                    );
                    return Err("Command failed");
                }
            }

            if stem::time::monotonic_ns() >= deadline {
                self.controlq_faulted = true;
                stem::error!(
                    "VirtioGpu: {} 0x{:x} timeout after {}ns; status=0x{:02x}; controlq faulted",
                    label,
                    cmd_type,
                    COMMAND_TIMEOUT_NS,
                    self.read_common_u8(virtio::VIRTIO_COMMON_STATUS)
                );
                return Err("Command timeout");
            }

            polls = polls.wrapping_add(1);
            if polls % 1024 == 0 {
                stem::time::sleep_ns(COMMAND_SLEEP_NS);
            } else if polls % 64 == 0 {
                stem::yield_now();
            } else {
                core::hint::spin_loop();
            }
        }
    }

    /// Get claim handle for device operations
    pub fn claim_handle(&self) -> usize {
        self.claim_handle
    }

    /// Allocate `pages` pages of device-DMA memory.
    ///
    /// Returns `(virtual_addr, physical_addr)` on success.  The returned
    /// virtual address is mapped into the driver's address space and the
    /// physical address is suitable for passing to [`attach_backing`] or
    /// [`attach_backing_3d`].
    pub fn alloc_dma(&mut self, pages: usize) -> Result<(u64, u64), &'static str> {
        use stem::syscall::{device_alloc_dma, device_dma_phys};
        let virt = device_alloc_dma(self.claim_handle, pages)
            .map_err(|_| "device_alloc_dma failed")?;
        let phys =
            device_dma_phys(virt).map_err(|_| "device_dma_phys failed")?;
        Ok((virt, phys))
    }
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

fn align_up(value: usize, alignment: usize) -> usize {
    if alignment == 0 { value } else { ((value + alignment - 1) / alignment) * alignment }
}

fn read_dma_u32(addr: u64) -> u32 {
    unsafe {
        let ptr = addr as *const u8;
        let bytes = [
            read_volatile(ptr),
            read_volatile(ptr.add(1)),
            read_volatile(ptr.add(2)),
            read_volatile(ptr.add(3)),
        ];
        u32::from_le_bytes(bytes)
    }
}

fn is_ok_response(resp_type: u32) -> bool {
    matches!(resp_type, VIRTIO_GPU_RESP_OK_NODATA | VIRTIO_GPU_RESP_OK_DISPLAY_INFO)
}
