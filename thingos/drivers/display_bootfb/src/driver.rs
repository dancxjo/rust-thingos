#![no_std]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;
use alloc::collections::BTreeMap;

use abi::display::{BufferId, CommitRequest, DisplayInfo, PlaneCommit, PlaneId};
use abi::display_driver_protocol::FB_INFO_PAYLOAD_SIZE;
use abi::errors::{Errno, SysResult};
use abi::pixel::PixelFormat;
use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
use stem::syscall::{vfs_close, vfs_open, vfs_read, vm_map, vm_unmap};
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
                refresh_mhz: 60000,
            },
            plane_count: 1,
            max_buffers: 32,
            supported_formats: 1 << (PixelFormat::Bgra8888 as u8),
            caps: abi::display::DisplayCaps::empty(),
        }
    }

    pub fn import_buffer(&mut self, handle: &abi::display::BufferHandle) -> SysResult<BufferId> {
        let size = (handle.height as usize) * (handle.stride as usize);
        let req = VmMapReq {
            addr_hint: 0,
            len: size,
            prot: VmProt::READ | VmProt::USER,
            flags: VmMapFlags::PRIVATE,
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

        debug!(
            "display_bootfb: imported buffer {} ({}x{} @ {:p})",
            id.0, handle.width, handle.height, resp.addr as *mut u8
        );
        Ok(id)
    }

    pub fn release_buffer(&mut self, id: BufferId) -> SysResult<()> {
        if let Some(buf) = self.buffers.remove(&id) {
            let _ = vm_unmap(buf.ptr as usize, buf.size);
            Ok(())
        } else {
            Err(Errno::ENOENT)
        }
    }

    pub fn commit(&mut self, req: &CommitRequest) -> SysResult<()> {
        for plane in req.planes() {
            self.blit_plane(plane)?;
        }
        Ok(())
    }

    fn blit_plane(&mut self, commit: &PlaneCommit) -> SysResult<()> {
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

        // Clip dest_rect to framebuffer bounds.
        let dst_x = commit.dest_rect.x.min(self.fb.width) as usize;
        let dst_y = commit.dest_rect.y.min(self.fb.height) as usize;
        let dst_w =
            commit.dest_rect.w.min(self.fb.width.saturating_sub(commit.dest_rect.x)) as usize;
        let dst_h =
            commit.dest_rect.h.min(self.fb.height.saturating_sub(commit.dest_rect.y)) as usize;

        // Copy extent is the intersection of the clipped src and dst dimensions.
        // Scaling is not supported; a 1:1 pixel mapping is performed.
        let copy_w = src_w.min(dst_w);
        let copy_h = src_h.min(dst_h);
        let row_bytes = copy_w * bpp;

        if row_bytes == 0 || copy_h == 0 {
            return Ok(());
        }

        let should_blend = commit.plane_id != PlaneId(0) || commit.alpha < 255;
        if should_blend && bpp == 4 {
            for row in 0..copy_h {
                for col in 0..copy_w {
                    unsafe {
                        let src_ptr = buffer
                            .ptr
                            .add((src_y + row) * buffer.stride as usize + (src_x + col) * bpp);
                        let dst_ptr = self
                            .fb
                            .base
                            .add((dst_y + row) * self.fb.stride as usize + (dst_x + col) * bpp);
                        let src = core::ptr::read_unaligned(src_ptr as *const u32);
                        let dst = core::ptr::read_unaligned(dst_ptr as *const u32);
                        core::ptr::write_unaligned(
                            dst_ptr as *mut u32,
                            alpha_over_argb(src, dst, commit.alpha),
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
