#![no_std]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;
use alloc::collections::BTreeMap;

use abi::display::{
    BufferId, CommitFlags, CommitRequest, DEFAULT_REFRESH_MHZ, DisplayInfo, PlaneCommit, PlaneId,
    accel2d::{
        Accel2dBatch, Accel2dCommand, ACCEL2D_CMD_ALPHA_BLIT, ACCEL2D_CMD_CLEAR_RECT,
        ACCEL2D_CMD_COPY_RECT, ACCEL2D_CMD_FLUSH_DAMAGE, ACCEL2D_CMD_MASKED_BLIT,
        ACCEL2D_CMD_ROUNDED_CLIP_BLIT, ACCEL2D_CMD_STRETCH_BLIT,
    },
};
use abi::display_driver_protocol::FB_INFO_PAYLOAD_SIZE;
use abi::display_protocol::Rect;
use abi::errors::{Errno, SysResult};
use abi::pixel::PixelFormat;
use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
use stem::syscall::{vfs_close, vfs_open, vfs_read, vm_map};
use stem::{debug, info};

/// HW Framebuffer description
pub struct Framebuffer {
    pub base: *mut u8,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub bpp: u32,
}

/// A buffer imported from userspace and mapped into driver memory
pub struct MappedBuffer {
    pub ptr: *mut u8,
    pub size: usize,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: PixelFormat,
}

pub struct BootFbDriver {
    pub fb: Framebuffer,
    pub buffers: BTreeMap<BufferId, MappedBuffer>,
    pub next_buffer_id: u32,
    /// Monotonically increasing sequence number for display DeviceCall RPCs.
    /// Used as the correlation ID in entry/exit traces.
    pub rpc_seq: u64,
    /// Monotonic timestamp (ns) recorded at the most recent DeviceCall entry.
    /// Used to compute per-RPC duration for the watchdog.
    pub rpc_enter_ns: u64,
}

impl BootFbDriver {
    pub fn new() -> Option<Self> {
        let fb = find_framebuffer()?;
        Some(Self { fb, buffers: BTreeMap::new(), next_buffer_id: 1, rpc_seq: 0, rpc_enter_ns: 0 })
    }

    pub fn get_info(&self) -> DisplayInfo {
        DisplayInfo {
            card_id: 0,
            preferred_mode: abi::display::DisplayMode {
                width: self.fb.width,
                height: self.fb.height,
                refresh_mhz: DEFAULT_REFRESH_MHZ,
            },
            plane_count: 1,
            max_buffers: 32,
            supported_formats: (1 << (PixelFormat::Bgra8888 as u8))
                | (1 << (PixelFormat::Bgrx8888 as u8)),
            // This provider replies to DISPLAY_OP_COMMIT synchronously.
            // Advertising VBLANK would make clients sleep inside the VFS RPC
            // response path instead of returning to their event loops.
            // PARTIAL_FLUSH: commit() clips blits to client-supplied damage
            // rectangles, so only the damaged regions are updated each frame.
            // ACCEL2D_*: CPU fallback for all 2D acceleration commands is
            // available when the destination is the output framebuffer.
            caps: abi::display::DisplayCaps::PARTIAL_FLUSH
                | abi::display::DisplayCaps::ACCEL2D_CLEAR
                | abi::display::DisplayCaps::ACCEL2D_COPY
                | abi::display::DisplayCaps::ACCEL2D_STRETCH
                | abi::display::DisplayCaps::ACCEL2D_ALPHA_BLIT
                | abi::display::DisplayCaps::ACCEL2D_MASKED_BLIT
                | abi::display::DisplayCaps::ACCEL2D_ROUNDED_CLIP_BLIT
                | abi::display::DisplayCaps::ACCEL2D_FLUSH_DAMAGE,
        }
    }

    pub fn import_buffer(&mut self, handle: &abi::display::BufferHandle) -> SysResult<BufferId> {
        if handle.modifier != 0 {
            return Err(Errno::EINVAL);
        }
        if !matches!(handle.format, PixelFormat::Bgra8888 | PixelFormat::Bgrx8888) {
            return Err(Errno::EINVAL);
        }
        let bpp = handle.format.bytes_per_pixel();
        let min_stride = (handle.width as usize).checked_mul(bpp).ok_or(Errno::EINVAL)?;
        if handle.width == 0 || handle.height == 0 || (handle.stride as usize) < min_stride {
            return Err(Errno::EINVAL);
        }
        let size =
            (handle.height as usize).checked_mul(handle.stride as usize).ok_or(Errno::EINVAL)?;
        let req = VmMapReq {
            addr_hint: 0,
            len: size,
            prot: VmProt::READ | VmProt::USER,
            // Bloom updates compositor-owned planes such as the pointer debug
            // overlay in place after import. The display driver must observe
            // those writes on every commit.
            flags: VmMapFlags::SHARED,
            backing: VmBacking::File { thing: handle.handle, offset: handle.offset },
        };

        let resp = vm_map(&req).map_err(|_| Errno::ENOMEM)?;
        let id = BufferId(self.next_buffer_id);
        self.next_buffer_id += 1;

        self.buffers.insert(
            id,
            MappedBuffer {
                ptr: resp.addr as *mut u8,
                size,
                width: handle.width,
                height: handle.height,
                stride: handle.stride,
                format: handle.format,
            },
        );

        info!(
            "display_bootfb: imported buffer {} ({}x{} @ {:p})",
            id.0, handle.width, handle.height, resp.addr as *mut u8
        );
        Ok(id)
    }

    pub fn release_buffer(&mut self, id: BufferId) -> SysResult<()> {
        if let Some(buf) = self.buffers.remove(&id) {
            debug!(
                "display_bootfb: released buffer {} (size={}, mapping retained)",
                id.0, buf.size
            );
            Ok(())
        } else {
            Err(Errno::ENOENT)
        }
    }

    pub fn commit(&mut self, req: &CommitRequest) -> SysResult<()> {
        let damage = req.damage_rects();
        // Blit all planes, with or without damage rects.
        if damage.is_empty() {
            for plane in req.planes() {
                self.blit_plane(plane)?;
            }
        } else {
            for plane in req.planes() {
                for rect in damage {
                    if let Some(clip) = rect_intersect(plane.dest_rect, *rect) {
                        self.blit_plane_clipped(plane, clip)?;
                    }
                }
            }
        }

        if req.flags.contains(CommitFlags::VSYNC) {
            stem::trace!("display_bootfb: ignoring VSYNC flag on synchronous commit path");
        }

        Ok(())
    }

    fn blit_plane(&mut self, commit: &PlaneCommit) -> SysResult<()> {
        self.blit_plane_clipped(commit, commit.dest_rect)
    }

    fn blit_plane_clipped(&mut self, commit: &PlaneCommit, clip: Rect) -> SysResult<()> {
        let buffer = self.buffers.get(&commit.buffer_id).ok_or(Errno::ENOENT)?;

        // Determine bytes-per-pixel from framebuffer metadata.
        let pitch_bpp =
            if self.fb.width > 0 { (self.fb.stride / self.fb.width) as usize } else { 0 };
        let mut bpp = (self.fb.bpp / 8) as usize;
        if pitch_bpp >= bpp && pitch_bpp > 0 {
            bpp = pitch_bpp;
        }

        // Clip src_rect to buffer bounds.
        let src_x = commit.src_rect.x.min(buffer.width) as usize;
        let src_y = commit.src_rect.y.min(buffer.height) as usize;
        let src_w = commit.src_rect.w.min(buffer.width.saturating_sub(commit.src_rect.x)) as usize;
        let src_h = commit.src_rect.h.min(buffer.height.saturating_sub(commit.src_rect.y)) as usize;

        let rel_x = clip.x.saturating_sub(commit.dest_rect.x);
        let rel_y = clip.y.saturating_sub(commit.dest_rect.y);
        let src_x = src_x.saturating_add(rel_x as usize).min(buffer.width as usize);
        let src_y = src_y.saturating_add(rel_y as usize).min(buffer.height as usize);
        let src_w = src_w.saturating_sub(rel_x as usize);
        let src_h = src_h.saturating_sub(rel_y as usize);

        // Clip destination to the framebuffer and requested damage bounds.
        let dst_x = clip.x.min(self.fb.width) as usize;
        let dst_y = clip.y.min(self.fb.height) as usize;
        let dst_w = clip.w.min(self.fb.width.saturating_sub(clip.x)) as usize;
        let dst_h = clip.h.min(self.fb.height.saturating_sub(clip.y)) as usize;

        // Copy extent is the intersection of the clipped src and dst dimensions.
        // Scaling is not supported; a 1:1 pixel mapping is performed.
        let copy_w = src_w.min(dst_w);
        let copy_h = src_h.min(dst_h);
        let row_bytes = copy_w * bpp;

        if row_bytes == 0 || copy_h == 0 {
            return Ok(());
        }

        let clip_radius = commit.rounded_clip_radius().map(u32::from).unwrap_or(0);
        let should_blend = commit.plane_id != PlaneId(0)
            || commit.alpha < 255
            || buffer.format.has_alpha()
            || clip_radius > 0;
        if should_blend && bpp == 4 {
            for row in 0..copy_h {
                for col in 0..copy_w {
                    let coverage = rounded_clip_coverage(
                        clip_radius,
                        rel_x.saturating_add(col as u32),
                        rel_y.saturating_add(row as u32),
                        commit.dest_rect.w,
                        commit.dest_rect.h,
                    );
                    if coverage == 0 {
                        continue;
                    }
                    unsafe {
                        let src_ptr = buffer
                            .ptr
                            .add((src_y + row) * buffer.stride as usize + (src_x + col) * bpp);
                        let dst_ptr = self
                            .fb
                            .base
                            .add((dst_y + row) * self.fb.stride as usize + (dst_x + col) * bpp);
                        let src = source_argb_for_blend(
                            core::ptr::read_unaligned(src_ptr as *const u32),
                            buffer.format,
                        );
                        let dst = core::ptr::read_unaligned(dst_ptr as *const u32);
                        core::ptr::write_unaligned(
                            dst_ptr as *mut u32,
                            alpha_over_argb(src, dst, scale_alpha(commit.alpha, coverage)),
                        );
                    }
                }
            }
        } else {
            for row in 0..copy_h {
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        buffer.ptr.add((src_y + row) * buffer.stride as usize + src_x * bpp),
                        self.fb.base.add((dst_y + row) * self.fb.stride as usize + dst_x * bpp),
                        row_bytes,
                    );
                }
            }
        }

        Ok(())
    }

    // ─── 2D Acceleration CPU Fallback ────────────────────────────────────────

    /// Execute a batch of 2D acceleration commands.
    ///
    /// Each command targets the output framebuffer (`BufferId(0)`) or an
    /// imported buffer.  Operations that would write to an imported buffer
    /// (which is mapped read-only) return `ENOSYS`; all writes go to the
    /// output framebuffer.
    pub fn execute_accel2d(
        &mut self,
        _header: &Accel2dBatch,
        commands: &[Accel2dCommand],
    ) -> SysResult<()> {
        for cmd in commands {
            self.execute_accel2d_cmd(cmd)?;
        }
        Ok(())
    }

    fn execute_accel2d_cmd(&mut self, cmd: &Accel2dCommand) -> SysResult<()> {
        match cmd.kind {
            ACCEL2D_CMD_CLEAR_RECT => {
                let c = unsafe { cmd.body.clear_rect };
                self.accel2d_clear_rect(c.dst_buffer, c.rect, c.color)
            }
            ACCEL2D_CMD_COPY_RECT => {
                let c = unsafe { cmd.body.copy_rect };
                self.accel2d_copy_rect(c.src_buffer, c.dst_buffer, c.src_rect, c.dst_rect)
            }
            ACCEL2D_CMD_STRETCH_BLIT => {
                let c = unsafe { cmd.body.stretch_blit };
                self.accel2d_stretch_blit(c.src_buffer, c.dst_buffer, c.src_rect, c.dst_rect)
            }
            ACCEL2D_CMD_ALPHA_BLIT => {
                let c = unsafe { cmd.body.alpha_blit };
                self.accel2d_alpha_blit(
                    c.src_buffer,
                    c.dst_buffer,
                    c.src_rect,
                    c.dst_rect,
                    c.global_alpha,
                )
            }
            ACCEL2D_CMD_MASKED_BLIT => {
                let c = unsafe { cmd.body.masked_blit };
                self.accel2d_masked_blit(
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
                self.accel2d_rounded_clip_blit(
                    c.src_buffer,
                    c.dst_buffer,
                    c.src_rect,
                    c.dst_rect,
                    c.radius,
                )
            }
            ACCEL2D_CMD_FLUSH_DAMAGE => {
                // Writes to the boot framebuffer are immediately visible; a
                // flush hint is a no-op for this backend.
                Ok(())
            }
            _ => Err(Errno::ENOSYS),
        }
    }

    /// Fill `rect` in the output framebuffer with `color` (ARGB8888).
    ///
    /// Only `dst_buffer == BufferId(0)` (the output framebuffer) is supported.
    fn accel2d_clear_rect(
        &mut self,
        dst_buffer: BufferId,
        rect: Rect,
        color: u32,
    ) -> SysResult<()> {
        if dst_buffer != BufferId(0) {
            return Err(Errno::ENOSYS);
        }
        let bpp = (self.fb.bpp / 8) as usize;
        if bpp != 4 {
            return Err(Errno::ENOSYS);
        }
        // Return early if the rectangle is entirely outside the framebuffer.
        if rect.x >= self.fb.width || rect.y >= self.fb.height {
            return Ok(());
        }
        let x = rect.x as usize;
        let y = rect.y as usize;
        let w = rect.w.min(self.fb.width - rect.x) as usize;
        let h = rect.h.min(self.fb.height - rect.y) as usize;
        for row in 0..h {
            for col in 0..w {
                unsafe {
                    let ptr = self
                        .fb
                        .base
                        .add((y + row) * self.fb.stride as usize + (x + col) * bpp)
                        as *mut u32;
                    core::ptr::write_unaligned(ptr, color);
                }
            }
        }
        Ok(())
    }

    /// Copy pixels from `src_rect` in `src_buffer` to `dst_rect` in the
    /// output framebuffer.  Rectangles must be the same size.
    fn accel2d_copy_rect(
        &mut self,
        src_buffer: BufferId,
        dst_buffer: BufferId,
        src_rect: Rect,
        dst_rect: Rect,
    ) -> SysResult<()> {
        if dst_buffer != BufferId(0) {
            return Err(Errno::ENOSYS);
        }
        let buf = self.buffers.get(&src_buffer).ok_or(Errno::ENOENT)?;
        let bpp = buf.format.bytes_per_pixel();
        let fb_bpp = (self.fb.bpp / 8) as usize;
        if bpp != fb_bpp || bpp == 0 {
            return Err(Errno::ENOSYS);
        }
        let copy_w =
            src_rect.w.min(dst_rect.w).min(buf.width.saturating_sub(src_rect.x)).min(
                self.fb.width.saturating_sub(dst_rect.x),
            ) as usize;
        let copy_h =
            src_rect.h.min(dst_rect.h).min(buf.height.saturating_sub(src_rect.y)).min(
                self.fb.height.saturating_sub(dst_rect.y),
            ) as usize;
        let row_bytes = copy_w * bpp;
        if row_bytes == 0 || copy_h == 0 {
            return Ok(());
        }
        let src_x = src_rect.x as usize;
        let src_y = src_rect.y as usize;
        let dst_x = dst_rect.x as usize;
        let dst_y = dst_rect.y as usize;
        let src_ptr = buf.ptr;
        let src_stride = buf.stride as usize;
        let fb_ptr = self.fb.base;
        let fb_stride = self.fb.stride as usize;
        for row in 0..copy_h {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    src_ptr.add((src_y + row) * src_stride + src_x * bpp),
                    fb_ptr.add((dst_y + row) * fb_stride + dst_x * fb_bpp),
                    row_bytes,
                );
            }
        }
        Ok(())
    }

    /// Scale-copy from `src_rect` in `src_buffer` to `dst_rect` in the output
    /// framebuffer using nearest-neighbour sampling.
    fn accel2d_stretch_blit(
        &mut self,
        src_buffer: BufferId,
        dst_buffer: BufferId,
        src_rect: Rect,
        dst_rect: Rect,
    ) -> SysResult<()> {
        if dst_buffer != BufferId(0) {
            return Err(Errno::ENOSYS);
        }
        let buf = self.buffers.get(&src_buffer).ok_or(Errno::ENOENT)?;
        let bpp = buf.format.bytes_per_pixel();
        let fb_bpp = (self.fb.bpp / 8) as usize;
        if bpp != fb_bpp || bpp != 4 {
            return Err(Errno::ENOSYS);
        }
        if src_rect.w == 0 || src_rect.h == 0 || dst_rect.w == 0 || dst_rect.h == 0 {
            return Ok(());
        }
        let dst_w = dst_rect.w.min(self.fb.width.saturating_sub(dst_rect.x)) as usize;
        let dst_h = dst_rect.h.min(self.fb.height.saturating_sub(dst_rect.y)) as usize;
        // dst_w/dst_h can be zero if dst_rect starts outside the framebuffer.
        if dst_w == 0 || dst_h == 0 {
            return Ok(());
        }
        let src_w = src_rect.w as usize;
        let src_h = src_rect.h as usize;
        let src_x0 = src_rect.x as usize;
        let src_y0 = src_rect.y as usize;
        let dst_x0 = dst_rect.x as usize;
        let dst_y0 = dst_rect.y as usize;
        let src_ptr = buf.ptr;
        let src_stride = buf.stride as usize;
        let fb_ptr = self.fb.base;
        let fb_stride = self.fb.stride as usize;
        for dy in 0..dst_h {
            // Nearest-neighbour row mapping.
            let sy = (dy * src_h / dst_h).min(src_h.saturating_sub(1));
            for dx in 0..dst_w {
                let sx = (dx * src_w / dst_w).min(src_w.saturating_sub(1));
                unsafe {
                    let src_pixel = core::ptr::read_unaligned(
                        src_ptr.add((src_y0 + sy) * src_stride + (src_x0 + sx) * bpp)
                            as *const u32,
                    );
                    let dst_ptr = fb_ptr
                        .add((dst_y0 + dy) * fb_stride + (dst_x0 + dx) * fb_bpp)
                        as *mut u32;
                    core::ptr::write_unaligned(dst_ptr, src_pixel);
                }
            }
        }
        Ok(())
    }

    /// Blend `src_buffer` over the output framebuffer at `dst_rect` with the
    /// given `global_alpha` multiplier (0 = transparent, 255 = opaque).
    fn accel2d_alpha_blit(
        &mut self,
        src_buffer: BufferId,
        dst_buffer: BufferId,
        src_rect: Rect,
        dst_rect: Rect,
        global_alpha: u8,
    ) -> SysResult<()> {
        if dst_buffer != BufferId(0) {
            return Err(Errno::ENOSYS);
        }
        let buf = self.buffers.get(&src_buffer).ok_or(Errno::ENOENT)?;
        let bpp = buf.format.bytes_per_pixel();
        let fb_bpp = (self.fb.bpp / 8) as usize;
        if bpp != 4 || fb_bpp != 4 {
            return Err(Errno::ENOSYS);
        }
        let copy_w = src_rect
            .w
            .min(dst_rect.w)
            .min(buf.width.saturating_sub(src_rect.x))
            .min(self.fb.width.saturating_sub(dst_rect.x)) as usize;
        let copy_h = src_rect
            .h
            .min(dst_rect.h)
            .min(buf.height.saturating_sub(src_rect.y))
            .min(self.fb.height.saturating_sub(dst_rect.y)) as usize;
        if copy_w == 0 || copy_h == 0 {
            return Ok(());
        }
        let src_x = src_rect.x as usize;
        let src_y = src_rect.y as usize;
        let dst_x = dst_rect.x as usize;
        let dst_y = dst_rect.y as usize;
        let src_ptr = buf.ptr;
        let src_stride = buf.stride as usize;
        let src_format = buf.format;
        let fb_ptr = self.fb.base;
        let fb_stride = self.fb.stride as usize;
        for row in 0..copy_h {
            for col in 0..copy_w {
                unsafe {
                    let s_ptr = src_ptr.add((src_y + row) * src_stride + (src_x + col) * bpp);
                    let d_ptr =
                        fb_ptr.add((dst_y + row) * fb_stride + (dst_x + col) * fb_bpp) as *mut u32;
                    let src_px =
                        source_argb_for_blend(core::ptr::read_unaligned(s_ptr as *const u32), src_format);
                    let dst_px = core::ptr::read_unaligned(d_ptr);
                    core::ptr::write_unaligned(d_ptr, alpha_over_argb(src_px, dst_px, global_alpha));
                }
            }
        }
        Ok(())
    }

    /// Blend `src_buffer` over the output framebuffer using `mask_buffer` as
    /// per-pixel alpha coverage.
    fn accel2d_masked_blit(
        &mut self,
        src_buffer: BufferId,
        mask_buffer: BufferId,
        dst_buffer: BufferId,
        src_rect: Rect,
        mask_rect: Rect,
        dst_rect: Rect,
    ) -> SysResult<()> {
        if dst_buffer != BufferId(0) {
            return Err(Errno::ENOSYS);
        }
        let src = self.buffers.get(&src_buffer).ok_or(Errno::ENOENT)?;
        let src_bpp = src.format.bytes_per_pixel();
        let src_ptr = src.ptr;
        let src_stride = src.stride as usize;
        let src_format = src.format;
        let src_w = src.width;
        let src_h = src.height;

        let msk = self.buffers.get(&mask_buffer).ok_or(Errno::ENOENT)?;
        let msk_bpp = msk.format.bytes_per_pixel();
        let msk_ptr = msk.ptr;
        let msk_stride = msk.stride as usize;
        let msk_format = msk.format;
        let msk_w = msk.width;
        let msk_h = msk.height;

        let fb_bpp = (self.fb.bpp / 8) as usize;
        if src_bpp != 4 || msk_bpp != 4 || fb_bpp != 4 {
            return Err(Errno::ENOSYS);
        }
        let copy_w = src_rect
            .w
            .min(mask_rect.w)
            .min(dst_rect.w)
            .min(src_w.saturating_sub(src_rect.x))
            .min(msk_w.saturating_sub(mask_rect.x))
            .min(self.fb.width.saturating_sub(dst_rect.x)) as usize;
        let copy_h = src_rect
            .h
            .min(mask_rect.h)
            .min(dst_rect.h)
            .min(src_h.saturating_sub(src_rect.y))
            .min(msk_h.saturating_sub(mask_rect.y))
            .min(self.fb.height.saturating_sub(dst_rect.y)) as usize;
        if copy_w == 0 || copy_h == 0 {
            return Ok(());
        }
        let sx0 = src_rect.x as usize;
        let sy0 = src_rect.y as usize;
        let mx0 = mask_rect.x as usize;
        let my0 = mask_rect.y as usize;
        let dx0 = dst_rect.x as usize;
        let dy0 = dst_rect.y as usize;
        let fb_ptr = self.fb.base;
        let fb_stride = self.fb.stride as usize;
        for row in 0..copy_h {
            for col in 0..copy_w {
                unsafe {
                    let s_px = source_argb_for_blend(
                        core::ptr::read_unaligned(
                            src_ptr.add((sy0 + row) * src_stride + (sx0 + col) * src_bpp)
                                as *const u32,
                        ),
                        src_format,
                    );
                    let m_raw = core::ptr::read_unaligned(
                        msk_ptr.add((my0 + row) * msk_stride + (mx0 + col) * msk_bpp)
                            as *const u32,
                    );
                    // Use mask alpha channel; fall back to luminance for opaque formats.
                    let mask_a = if msk_format.has_alpha() {
                        ((m_raw >> 24) & 0xff) as u8
                    } else {
                        // Approximate luminance from RGB.
                        let r = (m_raw >> 16) & 0xff;
                        let g = (m_raw >> 8) & 0xff;
                        let b = m_raw & 0xff;
                        ((r * 77 + g * 150 + b * 29) >> 8) as u8
                    };
                    let d_ptr =
                        fb_ptr.add((dy0 + row) * fb_stride + (dx0 + col) * fb_bpp) as *mut u32;
                    let dst_px = core::ptr::read_unaligned(d_ptr);
                    let blended_alpha = scale_alpha((s_px >> 24) as u8, mask_a);
                    let src_with_mask = (s_px & 0x00ff_ffff) | ((blended_alpha as u32) << 24);
                    core::ptr::write_unaligned(d_ptr, alpha_over_argb(src_with_mask, dst_px, 255));
                }
            }
        }
        Ok(())
    }

    /// Blit `src_buffer` to the output framebuffer at `dst_rect`, clipped to a
    /// rounded rectangle with `radius` pixels at each corner.
    fn accel2d_rounded_clip_blit(
        &mut self,
        src_buffer: BufferId,
        dst_buffer: BufferId,
        src_rect: Rect,
        dst_rect: Rect,
        radius: u8,
    ) -> SysResult<()> {
        if dst_buffer != BufferId(0) {
            return Err(Errno::ENOSYS);
        }
        let buf = self.buffers.get(&src_buffer).ok_or(Errno::ENOENT)?;
        let bpp = buf.format.bytes_per_pixel();
        let fb_bpp = (self.fb.bpp / 8) as usize;
        if bpp != 4 || fb_bpp != 4 {
            return Err(Errno::ENOSYS);
        }
        let copy_w = src_rect
            .w
            .min(dst_rect.w)
            .min(buf.width.saturating_sub(src_rect.x))
            .min(self.fb.width.saturating_sub(dst_rect.x)) as usize;
        let copy_h = src_rect
            .h
            .min(dst_rect.h)
            .min(buf.height.saturating_sub(src_rect.y))
            .min(self.fb.height.saturating_sub(dst_rect.y)) as usize;
        if copy_w == 0 || copy_h == 0 {
            return Ok(());
        }
        let sx0 = src_rect.x as usize;
        let sy0 = src_rect.y as usize;
        let dx0 = dst_rect.x as usize;
        let dy0 = dst_rect.y as usize;
        let radius_u32 = radius as u32;
        let src_ptr = buf.ptr;
        let src_stride = buf.stride as usize;
        let src_format = buf.format;
        let fb_ptr = self.fb.base;
        let fb_stride = self.fb.stride as usize;
        for row in 0..copy_h {
            for col in 0..copy_w {
                let coverage = rounded_clip_coverage(
                    radius_u32,
                    col as u32,
                    row as u32,
                    dst_rect.w,
                    dst_rect.h,
                );
                if coverage == 0 {
                    continue;
                }
                unsafe {
                    let s_px = source_argb_for_blend(
                        core::ptr::read_unaligned(
                            src_ptr.add((sy0 + row) * src_stride + (sx0 + col) * bpp)
                                as *const u32,
                        ),
                        src_format,
                    );
                    let d_ptr =
                        fb_ptr.add((dy0 + row) * fb_stride + (dx0 + col) * fb_bpp) as *mut u32;
                    let dst_px = core::ptr::read_unaligned(d_ptr);
                    core::ptr::write_unaligned(
                        d_ptr,
                        alpha_over_argb(s_px, dst_px, scale_alpha(255, coverage)),
                    );
                }
            }
        }
        Ok(())
    }
}

fn rect_intersect(a: Rect, b: Rect) -> Option<Rect> {
    let x1 = a.x.max(b.x);
    let y1 = a.y.max(b.y);
    let x2 = a.x.saturating_add(a.w).min(b.x.saturating_add(b.w));
    let y2 = a.y.saturating_add(a.h).min(b.y.saturating_add(b.h));
    if x2 <= x1 || y2 <= y1 { None } else { Some(Rect { x: x1, y: y1, w: x2 - x1, h: y2 - y1 }) }
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

fn scale_alpha(alpha: u8, coverage: u8) -> u8 {
    ((alpha as u32 * coverage as u32 + 127) / 255) as u8
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

fn find_framebuffer() -> Option<Framebuffer> {
    use abi::display_driver_protocol::FbInfoPayload;
    use abi::errors::Errno;
    use abi::syscall::vfs_flags::O_RDWR;

    info!("display_bootfb: probing /dev/fb0...");
    let fd = match vfs_open("/dev/fb0", O_RDWR) {
        Ok(fd) => fd,
        Err(Errno::EACCES) => {
            stem::warn!(
                "[BOOTFB] display_bootfb: EACCES opening /dev/fb0 — \
                 driver process does not have read permission for the boot framebuffer device"
            );
            return None;
        }
        Err(Errno::ENOENT) => {
            stem::warn!(
                "[BOOTFB] display_bootfb: /dev/fb0 not found — \
                 boot framebuffer not registered in VFS"
            );
            return None;
        }
        Err(e) => {
            stem::warn!("[BOOTFB] display_bootfb: failed to open /dev/fb0: {:?}", e);
            return None;
        }
    };

    let mut payload = FbInfoPayload {
        device_handle: 0,
        width: 0,
        height: 0,
        stride: 0,
        bpp: 0,
        format: 0,
        _reserved: 0,
    };

    let slice = unsafe {
        core::slice::from_raw_parts_mut(&mut payload as *mut _ as *mut u8, FB_INFO_PAYLOAD_SIZE)
    };

    let n = match vfs_read(fd, slice) {
        Ok(n) => n,
        Err(e) => {
            stem::warn!("[BOOTFB] display_bootfb: failed to read /dev/fb0 info: {:?}", e);
            let _ = vfs_close(fd);
            return None;
        }
    };
    if n < FB_INFO_PAYLOAD_SIZE {
        stem::warn!(
            "[BOOTFB] display_bootfb: /dev/fb0 info read too short: got {} bytes, need {}",
            n,
            FB_INFO_PAYLOAD_SIZE
        );
        let _ = vfs_close(fd);
        return None;
    }

    let byte_len = (payload.height as usize) * (payload.stride as usize);
    let req = VmMapReq {
        addr_hint: 0,
        len: byte_len,
        prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
        flags: VmMapFlags::empty(),
        backing: VmBacking::File { thing: fd, offset: 0 },
    };
    let resp = match vm_map(&req) {
        Ok(resp) => resp,
        Err(e) => {
            stem::warn!("[BOOTFB] display_bootfb: failed to map /dev/fb0: {:?}", e);
            let _ = vfs_close(fd);
            return None;
        }
    };

    let _ = vfs_close(fd);

    Some(Framebuffer {
        base: resp.addr as *mut u8,
        width: payload.width,
        height: payload.height,
        stride: payload.stride,
        bpp: payload.bpp,
    })
}
