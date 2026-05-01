#![no_std]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;
use alloc::collections::BTreeMap;

use abi::display::{
    BufferId, CommitFlags, CommitRequest, DEFAULT_REFRESH_MHZ, DisplayInfo, PlaneCommit, PlaneId,
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
}

impl BootFbDriver {
    pub fn new() -> Option<Self> {
        let fb = find_framebuffer()?;
        Some(Self { fb, buffers: BTreeMap::new(), next_buffer_id: 1 })
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
            caps: abi::display::DisplayCaps::PARTIAL_FLUSH,
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
