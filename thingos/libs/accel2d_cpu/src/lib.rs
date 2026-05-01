//! CPU fallback pixel-operation library for 2D display acceleration.
//!
//! Provides the pure pixel manipulation routines used by display drivers to
//! implement `DISPLAY_OP_ACCEL2D` commands without GPU assistance.  All
//! operations work on raw byte buffers described by [`PixelBuf`]; the driver
//! is responsible for validation (buffer ownership, capability checks) before
//! calling these functions.
//!
//! # Format assumption
//!
//! All buffers handled by these routines use 4 bytes per pixel in the
//! canonical `0xAARRGGBB` little-endian word layout (i.e. BGRA8888 or
//! BGRX8888 memory order).  Colour constants in tests use the same layout.
//!
//! # Safety
//!
//! Every function that touches pixel memory is marked `unsafe`.  The caller
//! must guarantee that:
//! - `ptr` points to at least `size` bytes of valid, writable (for `dst`)
//!   or readable (for `src` / `mask`) memory.
//! - The buffer is not concurrently mutated from another thread while an
//!   operation is in progress.
#![no_std]
extern crate alloc;

use abi::display_protocol::Rect;
use abi::pixel::PixelFormat;

// ─── Batch validation ──────────────────────────────────────────────────────

pub use abi::display::accel2d::{
    ACCEL2D_COMMAND_SIZE, ACCEL2D_MAX_DAMAGE_RECTS, Accel2dBatch, FlushDamageCmd,
};

/// Validate the wire payload of a `DISPLAY_OP_ACCEL2D` batch.
///
/// Returns `Ok(cmd_count)` when the payload is well-formed and `cmd_count`
/// does not exceed `max_cmds`.  Returns `Err(BatchError::*)` otherwise.
pub fn validate_accel2d_batch(
    payload: &[u8],
    max_cmds: usize,
) -> Result<usize, BatchError> {
    let batch_size = core::mem::size_of::<Accel2dBatch>();
    if payload.len() < batch_size {
        return Err(BatchError::TooShort);
    }
    let header: Accel2dBatch =
        unsafe { core::ptr::read_unaligned(payload.as_ptr() as *const Accel2dBatch) };
    let cmd_count = header.cmd_count as usize;
    if cmd_count > max_cmds {
        return Err(BatchError::TooManyCommands);
    }
    let needed =
        batch_size.saturating_add(cmd_count.saturating_mul(ACCEL2D_COMMAND_SIZE));
    if payload.len() < needed {
        return Err(BatchError::TooShort);
    }
    Ok(cmd_count)
}

/// Error type returned by [`validate_accel2d_batch`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchError {
    /// The payload is shorter than required for the declared command count.
    TooShort,
    /// The declared command count exceeds the caller-supplied maximum.
    TooManyCommands,
}

// ─── Damage-rect helpers ───────────────────────────────────────────────────

/// Return `true` when the rectangle has zero area.
#[inline]
pub fn rect_is_empty(r: Rect) -> bool {
    r.w == 0 || r.h == 0
}

/// Clamp a rectangle to lie within `[0, surface_w) × [0, surface_h)`.
#[inline]
pub fn rect_clamp_to_surface(r: Rect, surface_w: u32, surface_h: u32) -> Rect {
    let x = r.x.min(surface_w);
    let y = r.y.min(surface_h);
    Rect {
        x,
        y,
        w: r.w.min(surface_w.saturating_sub(x)),
        h: r.h.min(surface_h.saturating_sub(y)),
    }
}

/// Compute the effective damage rectangles described by `cmd`.
///
/// A `rect_count` of 0 is interpreted as *full-surface flush*: the returned
/// list contains a single rectangle covering `[0, surface_w) × [0, surface_h)`.
/// Non-zero `rect_count` entries are clamped to the surface bounds; entries
/// that become empty after clamping are discarded.
pub fn compute_damage_rects(
    cmd: &FlushDamageCmd,
    surface_w: u32,
    surface_h: u32,
) -> alloc::vec::Vec<Rect> {
    if cmd.rect_count == 0 {
        return alloc::vec![Rect { x: 0, y: 0, w: surface_w, h: surface_h }];
    }
    let count = (cmd.rect_count as usize).min(ACCEL2D_MAX_DAMAGE_RECTS);
    let mut out = alloc::vec::Vec::with_capacity(count);
    for i in 0..count {
        let clamped = rect_clamp_to_surface(cmd.rects[i], surface_w, surface_h);
        if !rect_is_empty(clamped) {
            out.push(clamped);
        }
    }
    out
}

// ─── Pixel blending helpers ────────────────────────────────────────────────

/// Standard Porter-Duff "source over destination" composite.
///
/// `plane_alpha` is an additional per-plane opacity applied to the source
/// alpha before blending.  Returns a fully opaque result.
#[inline]
pub fn alpha_over_argb(src: u32, dst: u32, plane_alpha: u8) -> u32 {
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

/// Normalise a source pixel for blending.
///
/// If `format` has no alpha channel (e.g. `Bgrx8888`) the alpha is forced to
/// 255 (fully opaque) so the pixel participates in blending correctly.
#[inline]
pub fn source_argb_for_blend(src: u32, format: PixelFormat) -> u32 {
    if format.has_alpha() { src } else { 0xff00_0000 | (src & 0x00ff_ffff) }
}

/// Multiply two alpha values, normalising the result back to [0, 255].
#[inline]
pub fn scale_alpha(a: u8, b: u8) -> u8 {
    ((a as u32 * b as u32 + 127) / 255) as u8
}

/// Per-pixel coverage for a rounded-rectangle clip mask.
///
/// Returns 255 (fully inside) for pixels in the central body and axis-aligned
/// bands, 0 (fully outside) for pixels in the corners that lie outside the
/// inscribed circle, and an intermediate value for pixels on the anti-aliased
/// edge (computed via 4×4 supersampling).
pub fn rounded_clip_coverage(radius: u32, x: u32, y: u32, w: u32, h: u32) -> u8 {
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
        let cy = if py < top { top } else if py >= bottom { bottom } else { py };
        for sx in 0..4i64 {
            let px = x as i64 * 8 + sx * 2 + 1;
            let cx = if px < left { left } else if px >= right { right } else { px };
            let dx = px - cx;
            let dy = py - cy;
            if dx * dx + dy * dy <= r * r {
                inside += 1;
            }
        }
    }
    ((inside * 255 + 8) / 16) as u8
}

// ─── PixelBuf — buffer descriptor ─────────────────────────────────────────

/// Descriptor for a CPU-accessible pixel buffer.
///
/// Used to pass framebuffer and imported-buffer addresses to the pixel
/// operations below without tying the library to any driver data structures.
pub struct PixelBuf {
    /// Pointer to the first byte of pixel data.
    pub ptr: *mut u8,
    /// Total byte length of the buffer (`height * stride`).
    pub size: usize,
    /// Width of the buffer in pixels.
    pub width: u32,
    /// Height of the buffer in pixels.
    pub height: u32,
    /// Row pitch in bytes (`>= width * bytes_per_pixel`).
    pub stride: u32,
    /// Pixel format; determines bytes-per-pixel and alpha interpretation.
    pub format: PixelFormat,
}

// Safety: the raw pointer is only dereferenced from a single thread during
// driver operations.  Callers must uphold the aliasing invariant.
unsafe impl Send for PixelBuf {}
unsafe impl Sync for PixelBuf {}

// ─── ACCEL2D_CMD_CLEAR_RECT ────────────────────────────────────────────────

/// Fill `rect` in `dst` with `color` (0xAARRGGBB little-endian word).
///
/// # Safety
///
/// `dst.ptr` must be valid and writable for `dst.size` bytes.
pub unsafe fn clear_rect(dst: &PixelBuf, rect: Rect, color: u32) {
    let bpp = 4usize;
    let stride = dst.stride as usize;
    if stride == 0 || dst.size == 0 {
        return;
    }
    if rect.x >= dst.width || rect.y >= dst.height {
        return;
    }
    let x = rect.x as usize;
    let y = rect.y as usize;
    let w = rect.w.min(dst.width.saturating_sub(rect.x)) as usize;
    let h = rect.h.min(dst.height.saturating_sub(rect.y)) as usize;
    if w == 0 || h == 0 {
        return;
    }
    for row in 0..h {
        for col in 0..w {
            let off = (y + row).saturating_mul(stride) + (x + col).saturating_mul(bpp);
            if off + bpp > dst.size {
                break;
            }
            core::ptr::write_unaligned(dst.ptr.add(off) as *mut u32, color);
        }
    }
}

// ─── ACCEL2D_CMD_COPY_RECT ────────────────────────────────────────────────

/// Copy pixels from `src_rect` in `src` to `dst_rect` in `dst` (no blending).
///
/// Both rectangles must be the same size; the copy extent is clamped to the
/// smaller of the two.  Mismatched bytes-per-pixel is a no-op.
///
/// # Safety
///
/// `src.ptr` must be valid and readable for `src.size` bytes.
/// `dst.ptr` must be valid and writable for `dst.size` bytes.
pub unsafe fn copy_rect(
    src: &PixelBuf,
    dst: &PixelBuf,
    src_rect: Rect,
    dst_rect: Rect,
) {
    let bpp = src.format.bytes_per_pixel();
    let dst_bpp = dst.format.bytes_per_pixel();
    if bpp != dst_bpp || bpp == 0 {
        return;
    }
    let copy_w = src_rect
        .w
        .min(dst_rect.w)
        .min(src.width.saturating_sub(src_rect.x))
        .min(dst.width.saturating_sub(dst_rect.x)) as usize;
    let copy_h = src_rect
        .h
        .min(dst_rect.h)
        .min(src.height.saturating_sub(src_rect.y))
        .min(dst.height.saturating_sub(dst_rect.y)) as usize;
    let row_bytes = copy_w * bpp;
    if row_bytes == 0 || copy_h == 0 {
        return;
    }
    let sx = src_rect.x as usize;
    let sy = src_rect.y as usize;
    let dx = dst_rect.x as usize;
    let dy = dst_rect.y as usize;
    let src_stride = src.stride as usize;
    let dst_stride = dst.stride as usize;
    for row in 0..copy_h {
        let src_off = (sy + row).saturating_mul(src_stride) + sx.saturating_mul(bpp);
        let dst_off = (dy + row).saturating_mul(dst_stride) + dx.saturating_mul(bpp);
        if src_off + row_bytes > src.size || dst_off + row_bytes > dst.size {
            break;
        }
        core::ptr::copy_nonoverlapping(src.ptr.add(src_off), dst.ptr.add(dst_off), row_bytes);
    }
}

// ─── ACCEL2D_CMD_STRETCH_BLIT ─────────────────────────────────────────────

/// Scale-copy from `src_rect` in `src` to `dst_rect` in `dst` using
/// nearest-neighbour sampling.
///
/// Mismatched or non-4-byte-per-pixel formats are a no-op.
///
/// # Safety
///
/// `src.ptr` must be valid and readable for `src.size` bytes.
/// `dst.ptr` must be valid and writable for `dst.size` bytes.
pub unsafe fn stretch_blit(
    src: &PixelBuf,
    dst: &PixelBuf,
    src_rect: Rect,
    dst_rect: Rect,
) {
    let bpp = src.format.bytes_per_pixel();
    if bpp != 4 || dst.format.bytes_per_pixel() != 4 {
        return;
    }
    if src_rect.w == 0 || src_rect.h == 0 || dst_rect.w == 0 || dst_rect.h == 0 {
        return;
    }
    let dst_w = dst_rect.w.min(dst.width.saturating_sub(dst_rect.x)) as usize;
    let dst_h = dst_rect.h.min(dst.height.saturating_sub(dst_rect.y)) as usize;
    if dst_w == 0 || dst_h == 0 {
        return;
    }
    let src_w = src_rect.w as usize;
    let src_h = src_rect.h as usize;
    let sx0 = src_rect.x as usize;
    let sy0 = src_rect.y as usize;
    let dx0 = dst_rect.x as usize;
    let dy0 = dst_rect.y as usize;
    let src_stride = src.stride as usize;
    let dst_stride = dst.stride as usize;
    // Fixed-point 16.16 scale ratios.
    let scale_x = (src_w << 16) / dst_w;
    let scale_y = (src_h << 16) / dst_h;
    for dy in 0..dst_h {
        let sy = ((dy * scale_y) >> 16).min(src_h.saturating_sub(1));
        for dx in 0..dst_w {
            let sx = ((dx * scale_x) >> 16).min(src_w.saturating_sub(1));
            let src_off = (sy0 + sy).saturating_mul(src_stride) + (sx0 + sx).saturating_mul(bpp);
            let dst_off = (dy0 + dy).saturating_mul(dst_stride) + (dx0 + dx).saturating_mul(bpp);
            if src_off + bpp > src.size || dst_off + bpp > dst.size {
                continue;
            }
            let px = core::ptr::read_unaligned(src.ptr.add(src_off) as *const u32);
            core::ptr::write_unaligned(dst.ptr.add(dst_off) as *mut u32, px);
        }
    }
}

// ─── ACCEL2D_CMD_ALPHA_BLIT ───────────────────────────────────────────────

/// Alpha-blend `src` over `dst` with a per-command `global_alpha` multiplier.
///
/// `global_alpha` is multiplied with the source pixel's own alpha before
/// compositing.  A value of 0 produces a no-op; 255 composites with the
/// source's intrinsic alpha only.
///
/// # Safety
///
/// `src.ptr` must be valid and readable for `src.size` bytes.
/// `dst.ptr` must be valid and writable for `dst.size` bytes.
pub unsafe fn alpha_blit(
    src: &PixelBuf,
    dst: &PixelBuf,
    src_rect: Rect,
    dst_rect: Rect,
    global_alpha: u8,
) {
    let bpp = src.format.bytes_per_pixel();
    if bpp != 4 || dst.format.bytes_per_pixel() != 4 {
        return;
    }
    let copy_w = src_rect
        .w
        .min(dst_rect.w)
        .min(src.width.saturating_sub(src_rect.x))
        .min(dst.width.saturating_sub(dst_rect.x)) as usize;
    let copy_h = src_rect
        .h
        .min(dst_rect.h)
        .min(src.height.saturating_sub(src_rect.y))
        .min(dst.height.saturating_sub(dst_rect.y)) as usize;
    if copy_w == 0 || copy_h == 0 {
        return;
    }
    let sx = src_rect.x as usize;
    let sy = src_rect.y as usize;
    let dx = dst_rect.x as usize;
    let dy = dst_rect.y as usize;
    let src_stride = src.stride as usize;
    let dst_stride = dst.stride as usize;
    for row in 0..copy_h {
        for col in 0..copy_w {
            let src_off = (sy + row).saturating_mul(src_stride) + (sx + col).saturating_mul(bpp);
            let dst_off = (dy + row).saturating_mul(dst_stride) + (dx + col).saturating_mul(bpp);
            if src_off + bpp > src.size || dst_off + bpp > dst.size {
                continue;
            }
            let s_px = source_argb_for_blend(
                core::ptr::read_unaligned(src.ptr.add(src_off) as *const u32),
                src.format,
            );
            let d_ptr = dst.ptr.add(dst_off) as *mut u32;
            let dst_px = core::ptr::read_unaligned(d_ptr);
            core::ptr::write_unaligned(d_ptr, alpha_over_argb(s_px, dst_px, global_alpha));
        }
    }
}

// ─── ACCEL2D_CMD_MASKED_BLIT ──────────────────────────────────────────────

/// Blend `src` over `dst` using `mask` as per-pixel alpha coverage.
///
/// The mask's alpha channel drives coverage when `mask.format.has_alpha()`;
/// otherwise the mask's ITU-R BT.601 luma approximation is used.
///
/// # Safety
///
/// `src.ptr`, `mask.ptr` must be valid and readable for their respective
/// `size` bytes.  `dst.ptr` must be valid and writable for `dst.size` bytes.
pub unsafe fn masked_blit(
    src: &PixelBuf,
    mask: &PixelBuf,
    dst: &PixelBuf,
    src_rect: Rect,
    mask_rect: Rect,
    dst_rect: Rect,
) {
    let bpp = src.format.bytes_per_pixel();
    let msk_bpp = mask.format.bytes_per_pixel();
    if bpp != 4 || msk_bpp != 4 || dst.format.bytes_per_pixel() != 4 {
        return;
    }
    let copy_w = src_rect
        .w
        .min(mask_rect.w)
        .min(dst_rect.w)
        .min(src.width.saturating_sub(src_rect.x))
        .min(mask.width.saturating_sub(mask_rect.x))
        .min(dst.width.saturating_sub(dst_rect.x)) as usize;
    let copy_h = src_rect
        .h
        .min(mask_rect.h)
        .min(dst_rect.h)
        .min(src.height.saturating_sub(src_rect.y))
        .min(mask.height.saturating_sub(mask_rect.y))
        .min(dst.height.saturating_sub(dst_rect.y)) as usize;
    if copy_w == 0 || copy_h == 0 {
        return;
    }
    let sx0 = src_rect.x as usize;
    let sy0 = src_rect.y as usize;
    let mx0 = mask_rect.x as usize;
    let my0 = mask_rect.y as usize;
    let dx0 = dst_rect.x as usize;
    let dy0 = dst_rect.y as usize;
    let src_stride = src.stride as usize;
    let msk_stride = mask.stride as usize;
    let dst_stride = dst.stride as usize;
    let msk_format = mask.format;
    for row in 0..copy_h {
        for col in 0..copy_w {
            let src_off = (sy0 + row).saturating_mul(src_stride) + (sx0 + col).saturating_mul(bpp);
            let msk_off =
                (my0 + row).saturating_mul(msk_stride) + (mx0 + col).saturating_mul(msk_bpp);
            let dst_off = (dy0 + row).saturating_mul(dst_stride) + (dx0 + col).saturating_mul(bpp);
            if src_off + bpp > src.size
                || msk_off + msk_bpp > mask.size
                || dst_off + bpp > dst.size
            {
                continue;
            }
            let s_px = source_argb_for_blend(
                core::ptr::read_unaligned(src.ptr.add(src_off) as *const u32),
                src.format,
            );
            let m_raw = core::ptr::read_unaligned(mask.ptr.add(msk_off) as *const u32);
            let mask_a = if msk_format.has_alpha() {
                ((m_raw >> 24) & 0xff) as u8
            } else {
                // ITU-R BT.601 luma approximation: Y ≈ 0.299R + 0.587G + 0.114B.
                // Integer coefficients: 77 + 150 + 29 = 256 → divide by 256 ≈ /255.
                let r = (m_raw >> 16) & 0xff;
                let g = (m_raw >> 8) & 0xff;
                let b = m_raw & 0xff;
                ((r * 77 + g * 150 + b * 29) >> 8) as u8
            };
            let d_ptr = dst.ptr.add(dst_off) as *mut u32;
            let dst_px = core::ptr::read_unaligned(d_ptr);
            let blended_alpha = scale_alpha((s_px >> 24) as u8, mask_a);
            let src_with_mask = (s_px & 0x00ff_ffff) | ((blended_alpha as u32) << 24);
            core::ptr::write_unaligned(d_ptr, alpha_over_argb(src_with_mask, dst_px, 255));
        }
    }
}

// ─── ACCEL2D_CMD_ROUNDED_CLIP_BLIT ────────────────────────────────────────

/// Blit `src` onto `dst`, clipped to a rounded rectangle with `radius` pixels
/// at each corner.
///
/// Corner pixels are anti-aliased via 4×4 supersampling.  Pixels fully outside
/// the inscribed circle are skipped.
///
/// # Safety
///
/// `src.ptr` must be valid and readable for `src.size` bytes.
/// `dst.ptr` must be valid and writable for `dst.size` bytes.
pub unsafe fn rounded_clip_blit(
    src: &PixelBuf,
    dst: &PixelBuf,
    src_rect: Rect,
    dst_rect: Rect,
    radius: u8,
) {
    let bpp = src.format.bytes_per_pixel();
    if bpp != 4 || dst.format.bytes_per_pixel() != 4 {
        return;
    }
    let copy_w = src_rect
        .w
        .min(dst_rect.w)
        .min(src.width.saturating_sub(src_rect.x))
        .min(dst.width.saturating_sub(dst_rect.x)) as usize;
    let copy_h = src_rect
        .h
        .min(dst_rect.h)
        .min(src.height.saturating_sub(src_rect.y))
        .min(dst.height.saturating_sub(dst_rect.y)) as usize;
    if copy_w == 0 || copy_h == 0 {
        return;
    }
    let sx0 = src_rect.x as usize;
    let sy0 = src_rect.y as usize;
    let dx0 = dst_rect.x as usize;
    let dy0 = dst_rect.y as usize;
    let radius_u32 = radius as u32;
    let src_stride = src.stride as usize;
    let dst_stride = dst.stride as usize;
    for row in 0..copy_h {
        for col in 0..copy_w {
            let coverage =
                rounded_clip_coverage(radius_u32, col as u32, row as u32, dst_rect.w, dst_rect.h);
            if coverage == 0 {
                continue;
            }
            let src_off = (sy0 + row).saturating_mul(src_stride) + (sx0 + col).saturating_mul(bpp);
            let dst_off = (dy0 + row).saturating_mul(dst_stride) + (dx0 + col).saturating_mul(bpp);
            if src_off + bpp > src.size || dst_off + bpp > dst.size {
                continue;
            }
            let s_px = source_argb_for_blend(
                core::ptr::read_unaligned(src.ptr.add(src_off) as *const u32),
                src.format,
            );
            let d_ptr = dst.ptr.add(dst_off) as *mut u32;
            let dst_px = core::ptr::read_unaligned(d_ptr);
            core::ptr::write_unaligned(d_ptr, alpha_over_argb(s_px, dst_px, coverage));
        }
    }
}

// ─── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use abi::display_protocol::Rect;
    use abi::pixel::PixelFormat;

    // ── Test helpers ──────────────────────────────────────────────────────

    /// Create a 4-bytes-per-pixel buffer backed by a `Vec<u32>`.
    ///
    /// Returns `(storage, PixelBuf)`.  The caller must keep `storage` alive
    /// as long as `PixelBuf` is in use.
    fn argb_buf(
        width: u32,
        height: u32,
        format: PixelFormat,
        fill: u32,
    ) -> (alloc::vec::Vec<u32>, PixelBuf) {
        let count = (width * height) as usize;
        let mut v = alloc::vec![fill; count];
        let ptr = v.as_mut_ptr() as *mut u8;
        let stride = width * 4;
        let size = (height * stride) as usize;
        let buf = PixelBuf { ptr, size, width, height, stride, format };
        (v, buf)
    }

    /// Read a pixel from the backing store at `(x, y)`.
    fn read_px(data: &[u32], x: u32, y: u32, width: u32) -> u32 {
        data[(y * width + x) as usize]
    }

    /// Build an `Accel2dBatch` header + zero commands as a byte payload.
    fn batch_payload(cmd_count: u32) -> alloc::vec::Vec<u8> {
        let batch = Accel2dBatch { cmd_count, _pad: 0 };
        let header_bytes = unsafe {
            core::slice::from_raw_parts(
                &batch as *const Accel2dBatch as *const u8,
                core::mem::size_of::<Accel2dBatch>(),
            )
        };
        let mut v = alloc::vec::Vec::new();
        v.extend_from_slice(header_bytes);
        // Pad with zeros for `cmd_count` commands.
        v.resize(
            v.len() + cmd_count as usize * ACCEL2D_COMMAND_SIZE,
            0u8,
        );
        v
    }

    // ── alpha_over_argb ───────────────────────────────────────────────────

    #[test]
    fn alpha_over_fully_transparent_src_is_dst() {
        // Source alpha = 0 → result must equal dst unchanged.
        let dst = 0xFF_AA_BB_CC_u32;
        assert_eq!(alpha_over_argb(0x00_FF_00_00, dst, 255), dst);
    }

    #[test]
    fn alpha_over_fully_opaque_src_replaces_dst() {
        // Source alpha = 255 → result is the source (RGB only, alpha forced to FF).
        let src = 0xFF_12_34_56_u32;
        let dst = 0xFF_AA_BB_CC_u32;
        let result = alpha_over_argb(src, dst, 255);
        assert_eq!((result >> 24) & 0xff, 0xFF, "alpha should be FF");
        assert_eq!((result >> 16) & 0xff, 0x12, "red");
        assert_eq!((result >> 8) & 0xff, 0x34, "green");
        assert_eq!(result & 0xff, 0x56, "blue");
    }

    #[test]
    fn alpha_over_plane_alpha_zero_is_dst() {
        // plane_alpha = 0 → effective src_a = 0 → dst unchanged.
        let dst = 0xFF_AA_BB_CC_u32;
        assert_eq!(alpha_over_argb(0xFF_FF_FF_FF, dst, 0), dst);
    }

    // ── scale_alpha ───────────────────────────────────────────────────────

    #[test]
    fn scale_alpha_identity() {
        assert_eq!(scale_alpha(255, 255), 255);
        assert_eq!(scale_alpha(128, 255), 128);
        assert_eq!(scale_alpha(255, 128), 128);
        assert_eq!(scale_alpha(0, 255), 0);
        assert_eq!(scale_alpha(255, 0), 0);
    }

    // ── rounded_clip_coverage ────────────────────────────────────────────

    #[test]
    fn rounded_clip_coverage_radius_zero_always_255() {
        assert_eq!(rounded_clip_coverage(0, 0, 0, 10, 10), 255);
        assert_eq!(rounded_clip_coverage(0, 9, 9, 10, 10), 255);
    }

    #[test]
    fn rounded_clip_coverage_center_is_255() {
        // Centre of a 10×10 square with radius 2 → fully inside.
        assert_eq!(rounded_clip_coverage(2, 5, 5, 10, 10), 255);
    }

    #[test]
    fn rounded_clip_coverage_corner_is_zero() {
        // Exact corner pixel (0,0) of a 10×10 square with radius 3 → outside.
        assert_eq!(rounded_clip_coverage(3, 0, 0, 10, 10), 0);
    }

    // ── rect helpers ──────────────────────────────────────────────────────

    #[test]
    fn rect_clamp_keeps_in_bounds_rect_unchanged() {
        let r = Rect { x: 1, y: 2, w: 3, h: 4 };
        assert_eq!(rect_clamp_to_surface(r, 10, 10), r);
    }

    #[test]
    fn rect_clamp_clips_to_surface() {
        let r = Rect { x: 8, y: 8, w: 5, h: 5 };
        let c = rect_clamp_to_surface(r, 10, 10);
        assert_eq!(c, Rect { x: 8, y: 8, w: 2, h: 2 });
    }

    #[test]
    fn rect_clamp_origin_outside_surface_gives_empty() {
        let r = Rect { x: 20, y: 20, w: 5, h: 5 };
        let c = rect_clamp_to_surface(r, 10, 10);
        assert!(rect_is_empty(c));
    }

    #[test]
    fn rect_is_empty_zero_width() {
        assert!(rect_is_empty(Rect { x: 0, y: 0, w: 0, h: 5 }));
    }

    #[test]
    fn rect_is_empty_zero_height() {
        assert!(rect_is_empty(Rect { x: 0, y: 0, w: 5, h: 0 }));
    }

    // ── compute_damage_rects ─────────────────────────────────────────────

    #[test]
    fn flush_damage_zero_rect_count_is_full_surface() {
        let cmd = FlushDamageCmd { rect_count: 0, _pad: 0, rects: [Rect::default(); 4] };
        let rects = compute_damage_rects(&cmd, 1280, 720);
        assert_eq!(rects.len(), 1);
        assert_eq!(rects[0], Rect { x: 0, y: 0, w: 1280, h: 720 });
    }

    #[test]
    fn flush_damage_nonzero_rect_count_uses_provided_rects() {
        let r0 = Rect { x: 0, y: 0, w: 100, h: 100 };
        let r1 = Rect { x: 200, y: 0, w: 50, h: 50 };
        let mut rects_arr = [Rect::default(); 4];
        rects_arr[0] = r0;
        rects_arr[1] = r1;
        let cmd = FlushDamageCmd { rect_count: 2, _pad: 0, rects: rects_arr };
        let rects = compute_damage_rects(&cmd, 1280, 720);
        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0], r0);
        assert_eq!(rects[1], r1);
    }

    #[test]
    fn flush_damage_out_of_bounds_rects_are_discarded() {
        let mut rects_arr = [Rect::default(); 4];
        // Entirely outside
        rects_arr[0] = Rect { x: 2000, y: 2000, w: 10, h: 10 };
        let cmd = FlushDamageCmd { rect_count: 1, _pad: 0, rects: rects_arr };
        let rects = compute_damage_rects(&cmd, 1280, 720);
        assert!(rects.is_empty(), "out-of-bounds rect should be discarded");
    }

    // ── validate_accel2d_batch ────────────────────────────────────────────

    #[test]
    fn batch_validates_empty_batch() {
        let payload = batch_payload(0);
        assert_eq!(validate_accel2d_batch(&payload, 4096), Ok(0));
    }

    #[test]
    fn batch_rejects_payload_shorter_than_header() {
        let short = &[0u8; 2];
        assert_eq!(validate_accel2d_batch(short, 4096), Err(BatchError::TooShort));
    }

    #[test]
    fn batch_rejects_cmd_count_exceeding_max() {
        // cmd_count = 1 but max_cmds = 0 → too many.
        let payload = batch_payload(1);
        assert_eq!(
            validate_accel2d_batch(&payload, 0),
            Err(BatchError::TooManyCommands),
        );
    }

    #[test]
    fn batch_rejects_payload_too_short_for_declared_cmds() {
        // Declare 3 commands but only supply the header.
        let batch = Accel2dBatch { cmd_count: 3, _pad: 0 };
        let bytes = unsafe {
            core::slice::from_raw_parts(
                &batch as *const Accel2dBatch as *const u8,
                core::mem::size_of::<Accel2dBatch>(),
            )
        };
        let payload: alloc::vec::Vec<u8> = bytes.to_vec();
        assert_eq!(validate_accel2d_batch(&payload, 4096), Err(BatchError::TooShort));
    }

    #[test]
    fn batch_rejects_more_than_max_accel2d_batch_cmds() {
        const MAX: usize = 4096;
        // Build a payload that claims MAX+1 commands.
        let cmd_count = (MAX + 1) as u32;
        let batch = Accel2dBatch { cmd_count, _pad: 0 };
        let header_bytes = unsafe {
            core::slice::from_raw_parts(
                &batch as *const Accel2dBatch as *const u8,
                core::mem::size_of::<Accel2dBatch>(),
            )
        };
        let payload: alloc::vec::Vec<u8> = header_bytes.to_vec();
        // Even with a too-short payload the TooManyCommands error fires first.
        assert_eq!(
            validate_accel2d_batch(&payload, MAX),
            Err(BatchError::TooManyCommands),
        );
    }

    // ── clear_rect ────────────────────────────────────────────────────────

    #[test]
    fn clear_rect_fills_target_pixels() {
        let (mut data, buf) = argb_buf(4, 4, PixelFormat::Bgra8888, 0x00_00_00_00);
        let color = 0xFF_FF_00_00_u32; // opaque red
        unsafe {
            clear_rect(&buf, Rect { x: 1, y: 1, w: 2, h: 2 }, color);
        }
        // Inner 2×2 should be filled with red.
        assert_eq!(read_px(&data, 1, 1, 4), color, "(1,1)");
        assert_eq!(read_px(&data, 2, 1, 4), color, "(2,1)");
        assert_eq!(read_px(&data, 1, 2, 4), color, "(1,2)");
        assert_eq!(read_px(&data, 2, 2, 4), color, "(2,2)");
    }

    #[test]
    fn clear_rect_leaves_outside_pixels_unchanged() {
        let bg = 0xFF_AA_BB_CC_u32;
        let (mut data, buf) = argb_buf(4, 4, PixelFormat::Bgra8888, bg);
        unsafe {
            clear_rect(&buf, Rect { x: 1, y: 1, w: 2, h: 2 }, 0xFF_FF_00_00);
        }
        // Corners should still be background.
        assert_eq!(read_px(&data, 0, 0, 4), bg, "(0,0)");
        assert_eq!(read_px(&data, 3, 0, 4), bg, "(3,0)");
        assert_eq!(read_px(&data, 0, 3, 4), bg, "(0,3)");
        assert_eq!(read_px(&data, 3, 3, 4), bg, "(3,3)");
    }

    #[test]
    fn clear_rect_out_of_bounds_is_noop() {
        let bg = 0xFF_AA_BB_CC_u32;
        let (mut data, buf) = argb_buf(4, 4, PixelFormat::Bgra8888, bg);
        // Rectangle starts past the right edge.
        unsafe {
            clear_rect(&buf, Rect { x: 10, y: 10, w: 4, h: 4 }, 0xFF_FF_FF_FF);
        }
        assert!(data.iter().all(|&p| p == bg), "buffer must be unchanged");
    }

    // ── copy_rect ─────────────────────────────────────────────────────────

    #[test]
    fn copy_rect_basic() {
        // Source: 4×1 row = [red, green, blue, white]
        let (mut src_data, src) = argb_buf(4, 1, PixelFormat::Bgra8888, 0);
        src_data[0] = 0xFF_FF_00_00; // red
        src_data[1] = 0xFF_00_FF_00; // green
        src_data[2] = 0xFF_00_00_FF; // blue
        src_data[3] = 0xFF_FF_FF_FF; // white

        let (mut dst_data, dst) = argb_buf(4, 1, PixelFormat::Bgra8888, 0);
        unsafe {
            copy_rect(
                &src,
                &dst,
                Rect { x: 0, y: 0, w: 4, h: 1 },
                Rect { x: 0, y: 0, w: 4, h: 1 },
            );
        }
        assert_eq!(dst_data[0], 0xFF_FF_00_00, "pixel 0 red");
        assert_eq!(dst_data[1], 0xFF_00_FF_00, "pixel 1 green");
        assert_eq!(dst_data[2], 0xFF_00_00_FF, "pixel 2 blue");
        assert_eq!(dst_data[3], 0xFF_FF_FF_FF, "pixel 3 white");
    }

    #[test]
    fn copy_rect_partial_row() {
        let (mut src_data, src) = argb_buf(4, 1, PixelFormat::Bgra8888, 0xFF_AA_AA_AA);
        // Only pixels 1 and 2 are distinct.
        src_data[1] = 0xFF_11_22_33;
        src_data[2] = 0xFF_44_55_66;

        let bg = 0xFF_00_00_00_u32;
        let (mut dst_data, dst) = argb_buf(4, 1, PixelFormat::Bgra8888, bg);
        unsafe {
            copy_rect(
                &src,
                &dst,
                Rect { x: 1, y: 0, w: 2, h: 1 },
                Rect { x: 1, y: 0, w: 2, h: 1 },
            );
        }
        assert_eq!(dst_data[0], bg, "pixel 0 unchanged");
        assert_eq!(dst_data[1], 0xFF_11_22_33, "pixel 1 copied");
        assert_eq!(dst_data[2], 0xFF_44_55_66, "pixel 2 copied");
        assert_eq!(dst_data[3], bg, "pixel 3 unchanged");
    }

    #[test]
    fn copy_rect_clips_to_dst_bounds() {
        // Source is 2×2; destination rect partially extends outside the 2×2 dst buffer.
        let (mut src_data, src) = argb_buf(2, 2, PixelFormat::Bgra8888, 0xFF_FF_00_00);
        let bg = 0xFF_00_00_00_u32;
        let (mut dst_data, dst) = argb_buf(2, 2, PixelFormat::Bgra8888, bg);
        // Copy src_rect(0,0,2,2) to dst_rect(1,1,2,2) — only 1×1 fits.
        unsafe {
            copy_rect(
                &src,
                &dst,
                Rect { x: 0, y: 0, w: 2, h: 2 },
                Rect { x: 1, y: 1, w: 2, h: 2 },
            );
        }
        assert_eq!(dst_data[0], bg, "(0,0) unchanged");
        assert_eq!(dst_data[1], bg, "(1,0) unchanged");
        assert_eq!(dst_data[2], bg, "(0,1) unchanged");
        assert_eq!(dst_data[3], 0xFF_FF_00_00, "(1,1) copied");
    }

    #[test]
    fn copy_rect_invalid_dst_buffer_noop() {
        // Non-BufferId(0) rejection is the driver's job; the library itself
        // is a no-op when bpp values differ.
        let (mut src_data, src) = argb_buf(4, 1, PixelFormat::Bgra8888, 0xFF_FF_00_00);
        // Simulate an incompatible dst by creating one with Rgb565 (2 bpp).
        let (mut dst_data, mut dst) = argb_buf(4, 1, PixelFormat::Bgra8888, 0);
        // Override format to something incompatible.
        dst.format = PixelFormat::Rgb565;
        unsafe {
            copy_rect(
                &src,
                &dst,
                Rect { x: 0, y: 0, w: 4, h: 1 },
                Rect { x: 0, y: 0, w: 4, h: 1 },
            );
        }
        assert!(dst_data.iter().all(|&p| p == 0), "incompatible dst must be unchanged");
    }

    // ── stretch_blit ──────────────────────────────────────────────────────

    #[test]
    fn stretch_blit_2x_upscale_nearest_neighbour() {
        // Source: 2×2, four distinct colours.
        //   TL=red  TR=green
        //   BL=blue BR=white
        let (mut src_data, src) = argb_buf(2, 2, PixelFormat::Bgra8888, 0);
        src_data[0] = 0xFF_FF_00_00; // (0,0) red
        src_data[1] = 0xFF_00_FF_00; // (1,0) green
        src_data[2] = 0xFF_00_00_FF; // (0,1) blue
        src_data[3] = 0xFF_FF_FF_FF; // (1,1) white

        let (mut dst_data, dst) = argb_buf(4, 4, PixelFormat::Bgra8888, 0);
        unsafe {
            stretch_blit(
                &src,
                &dst,
                Rect { x: 0, y: 0, w: 2, h: 2 },
                Rect { x: 0, y: 0, w: 4, h: 4 },
            );
        }
        // Nearest-neighbour 2× should replicate each source pixel into a 2×2 block.
        let red = 0xFF_FF_00_00;
        let green = 0xFF_00_FF_00;
        let blue = 0xFF_00_00_FF;
        let white = 0xFF_FF_FF_FF;
        assert_eq!(read_px(&dst_data, 0, 0, 4), red, "TL block (0,0)");
        assert_eq!(read_px(&dst_data, 1, 0, 4), red, "TL block (1,0)");
        assert_eq!(read_px(&dst_data, 2, 0, 4), green, "TR block (2,0)");
        assert_eq!(read_px(&dst_data, 3, 0, 4), green, "TR block (3,0)");
        assert_eq!(read_px(&dst_data, 0, 2, 4), blue, "BL block (0,2)");
        assert_eq!(read_px(&dst_data, 3, 3, 4), white, "BR block (3,3)");
    }

    #[test]
    fn stretch_blit_1x_scale_equals_copy_rect() {
        let (mut src_data, src) = argb_buf(3, 3, PixelFormat::Bgra8888, 0xFF_AB_CD_EF);
        src_data[4] = 0xFF_12_34_56; // centre pixel

        let (mut dst_data, dst) = argb_buf(3, 3, PixelFormat::Bgra8888, 0);
        unsafe {
            stretch_blit(
                &src,
                &dst,
                Rect { x: 0, y: 0, w: 3, h: 3 },
                Rect { x: 0, y: 0, w: 3, h: 3 },
            );
        }
        assert_eq!(dst_data[4], 0xFF_12_34_56, "centre pixel");
        assert_eq!(dst_data[0], 0xFF_AB_CD_EF, "corner pixel");
    }

    // ── alpha_blit ────────────────────────────────────────────────────────

    #[test]
    fn alpha_blit_fully_opaque_global_alpha_writes_src() {
        let (mut src_data, src) = argb_buf(1, 1, PixelFormat::Bgra8888, 0xFF_FF_00_00);
        let (mut dst_data, dst) = argb_buf(1, 1, PixelFormat::Bgra8888, 0xFF_00_00_FF);
        unsafe {
            alpha_blit(
                &src,
                &dst,
                Rect { x: 0, y: 0, w: 1, h: 1 },
                Rect { x: 0, y: 0, w: 1, h: 1 },
                255,
            );
        }
        // Fully opaque red src over blue dst → red.
        let result = dst_data[0];
        assert_eq!((result >> 24) & 0xff, 0xFF, "alpha");
        assert_eq!((result >> 16) & 0xff, 0xFF, "red");
        assert_eq!((result >> 8) & 0xff, 0x00, "green");
        assert_eq!(result & 0xff, 0x00, "blue");
    }

    #[test]
    fn alpha_blit_fully_transparent_global_alpha_preserves_dst() {
        let (mut src_data, src) = argb_buf(1, 1, PixelFormat::Bgra8888, 0xFF_FF_00_00);
        let dst_color = 0xFF_00_00_FF_u32;
        let (mut dst_data, dst) = argb_buf(1, 1, PixelFormat::Bgra8888, dst_color);
        unsafe {
            alpha_blit(
                &src,
                &dst,
                Rect { x: 0, y: 0, w: 1, h: 1 },
                Rect { x: 0, y: 0, w: 1, h: 1 },
                0, // global_alpha = 0 → no-op
            );
        }
        assert_eq!(dst_data[0], dst_color, "dst must be unchanged with global_alpha=0");
    }

    #[test]
    fn alpha_blit_partial_global_alpha_blends() {
        // src = fully opaque white (0xFFFFFFFF)
        // dst = fully opaque black (0xFF000000)
        // global_alpha = 128 (≈ 50%)  →  result should be mid-grey
        let (mut src_data, src) = argb_buf(1, 1, PixelFormat::Bgra8888, 0xFF_FF_FF_FF);
        let (mut dst_data, dst) = argb_buf(1, 1, PixelFormat::Bgra8888, 0xFF_00_00_00);
        unsafe {
            alpha_blit(
                &src,
                &dst,
                Rect { x: 0, y: 0, w: 1, h: 1 },
                Rect { x: 0, y: 0, w: 1, h: 1 },
                128,
            );
        }
        let result = dst_data[0];
        let r = (result >> 16) & 0xff;
        let g = (result >> 8) & 0xff;
        let b = result & 0xff;
        // With global_alpha=128, effective src_a ≈ 128.  Result should be roughly
        // mid-grey (each channel ≈ 128).  Allow ±2 for integer rounding.
        assert!((120..=135).contains(&r), "red ~128, got {r}");
        assert!((120..=135).contains(&g), "green ~128, got {g}");
        assert!((120..=135).contains(&b), "blue ~128, got {b}");
    }

    #[test]
    fn alpha_blit_transparent_src_pixel_preserves_dst() {
        // src pixel has alpha=0 (transparent) regardless of global_alpha.
        let (mut src_data, src) = argb_buf(1, 1, PixelFormat::Bgra8888, 0x00_FF_FF_FF);
        let dst_color = 0xFF_AB_CD_EF_u32;
        let (mut dst_data, dst) = argb_buf(1, 1, PixelFormat::Bgra8888, dst_color);
        unsafe {
            alpha_blit(
                &src,
                &dst,
                Rect { x: 0, y: 0, w: 1, h: 1 },
                Rect { x: 0, y: 0, w: 1, h: 1 },
                255,
            );
        }
        assert_eq!(dst_data[0], dst_color, "transparent src must not modify dst");
    }

    // ── masked_blit ───────────────────────────────────────────────────────

    #[test]
    fn masked_blit_full_alpha_mask_writes_src() {
        // mask alpha = 255 → source written fully.
        let (mut src_data, src) = argb_buf(1, 1, PixelFormat::Bgra8888, 0xFF_FF_00_00);
        let (mut msk_data, msk) = argb_buf(1, 1, PixelFormat::Bgra8888, 0xFF_FF_FF_FF);
        let (mut dst_data, dst) = argb_buf(1, 1, PixelFormat::Bgra8888, 0xFF_00_00_FF);
        unsafe {
            masked_blit(
                &src,
                &msk,
                &dst,
                Rect { x: 0, y: 0, w: 1, h: 1 },
                Rect { x: 0, y: 0, w: 1, h: 1 },
                Rect { x: 0, y: 0, w: 1, h: 1 },
            );
        }
        let result = dst_data[0];
        assert_eq!((result >> 16) & 0xff, 0xFF, "red");
        assert_eq!((result >> 8) & 0xff, 0x00, "green");
        assert_eq!(result & 0xff, 0x00, "blue");
    }

    #[test]
    fn masked_blit_zero_alpha_mask_preserves_dst() {
        // mask alpha = 0 → destination unchanged.
        let (mut src_data, src) = argb_buf(1, 1, PixelFormat::Bgra8888, 0xFF_FF_00_00);
        let (mut msk_data, msk) = argb_buf(1, 1, PixelFormat::Bgra8888, 0x00_00_00_00);
        let dst_color = 0xFF_00_00_FF_u32;
        let (mut dst_data, dst) = argb_buf(1, 1, PixelFormat::Bgra8888, dst_color);
        unsafe {
            masked_blit(
                &src,
                &msk,
                &dst,
                Rect { x: 0, y: 0, w: 1, h: 1 },
                Rect { x: 0, y: 0, w: 1, h: 1 },
                Rect { x: 0, y: 0, w: 1, h: 1 },
            );
        }
        assert_eq!(dst_data[0], dst_color, "zero-alpha mask must leave dst unchanged");
    }

    #[test]
    fn masked_blit_opaque_mask_uses_luma() {
        // mask is Bgrx8888 (no alpha) → luma of pure white (R=G=B=255) should be 255.
        let (mut src_data, src) = argb_buf(1, 1, PixelFormat::Bgra8888, 0xFF_FF_00_00);
        let (mut msk_data, msk) = argb_buf(1, 1, PixelFormat::Bgrx8888, 0xFF_FF_FF_FF);
        let (mut dst_data, dst) = argb_buf(1, 1, PixelFormat::Bgra8888, 0xFF_00_00_FF);
        unsafe {
            masked_blit(
                &src,
                &msk,
                &dst,
                Rect { x: 0, y: 0, w: 1, h: 1 },
                Rect { x: 0, y: 0, w: 1, h: 1 },
                Rect { x: 0, y: 0, w: 1, h: 1 },
            );
        }
        // Luma of white = 255 → full coverage → red source overwrites blue dst.
        let result = dst_data[0];
        assert_eq!((result >> 16) & 0xff, 0xFF, "red channel");
    }

    #[test]
    fn masked_blit_black_luma_mask_preserves_dst() {
        // Opaque black mask → luma ≈ 0 → no coverage → dst unchanged.
        let (mut src_data, src) = argb_buf(1, 1, PixelFormat::Bgra8888, 0xFF_FF_00_00);
        let (mut msk_data, msk) = argb_buf(1, 1, PixelFormat::Bgrx8888, 0xFF_00_00_00);
        let dst_color = 0xFF_00_00_FF_u32;
        let (mut dst_data, dst) = argb_buf(1, 1, PixelFormat::Bgra8888, dst_color);
        unsafe {
            masked_blit(
                &src,
                &msk,
                &dst,
                Rect { x: 0, y: 0, w: 1, h: 1 },
                Rect { x: 0, y: 0, w: 1, h: 1 },
                Rect { x: 0, y: 0, w: 1, h: 1 },
            );
        }
        assert_eq!(dst_data[0], dst_color, "black luma mask must leave dst unchanged");
    }

    // ── rounded_clip_blit ─────────────────────────────────────────────────

    #[test]
    fn rounded_clip_blit_center_pixels_copied() {
        // 6×6 source filled with red; blit with radius=1 onto black dst.
        // Centre pixels (away from corners) should be red.
        let (mut src_data, src) = argb_buf(6, 6, PixelFormat::Bgra8888, 0xFF_FF_00_00);
        let (mut dst_data, dst) = argb_buf(6, 6, PixelFormat::Bgra8888, 0xFF_00_00_00);
        unsafe {
            rounded_clip_blit(
                &src,
                &dst,
                Rect { x: 0, y: 0, w: 6, h: 6 },
                Rect { x: 0, y: 0, w: 6, h: 6 },
                1,
            );
        }
        // Centre pixel (3,3) must have been copied (coverage = 255).
        let centre = read_px(&dst_data, 3, 3, 6);
        assert_eq!((centre >> 16) & 0xff, 0xFF, "centre pixel red channel");
    }

    #[test]
    fn rounded_clip_blit_corner_pixels_skipped() {
        // 6×6 with radius=3.  For radius=3, the subpixel radius r=24 and the
        // farthest subpixel sample of corner (0,0) has distance^2 = 2*17^2 =
        // 578 > 576 = r^2, so all 4×4 samples are outside → coverage = 0.
        // Corners (0,0), (5,0), (0,5), (5,5) must stay as background.
        let red = 0xFF_FF_00_00_u32;
        let black = 0xFF_00_00_00_u32;
        let (mut src_data, src) = argb_buf(6, 6, PixelFormat::Bgra8888, red);
        let (mut dst_data, dst) = argb_buf(6, 6, PixelFormat::Bgra8888, black);
        unsafe {
            rounded_clip_blit(
                &src,
                &dst,
                Rect { x: 0, y: 0, w: 6, h: 6 },
                Rect { x: 0, y: 0, w: 6, h: 6 },
                3,
            );
        }
        assert_eq!(read_px(&dst_data, 0, 0, 6), black, "(0,0) corner skipped");
        assert_eq!(read_px(&dst_data, 5, 0, 6), black, "(5,0) corner skipped");
        assert_eq!(read_px(&dst_data, 0, 5, 6), black, "(0,5) corner skipped");
        assert_eq!(read_px(&dst_data, 5, 5, 6), black, "(5,5) corner skipped");
    }

    #[test]
    fn rounded_clip_blit_radius_zero_copies_all_pixels() {
        // radius=0 means no rounding; all pixels including corners are copied.
        let red = 0xFF_FF_00_00_u32;
        let black = 0xFF_00_00_00_u32;
        let (mut src_data, src) = argb_buf(4, 4, PixelFormat::Bgra8888, red);
        let (mut dst_data, dst) = argb_buf(4, 4, PixelFormat::Bgra8888, black);
        unsafe {
            rounded_clip_blit(
                &src,
                &dst,
                Rect { x: 0, y: 0, w: 4, h: 4 },
                Rect { x: 0, y: 0, w: 4, h: 4 },
                0,
            );
        }
        // All pixels including exact corners must be red.
        for &px in dst_data.iter() {
            assert_eq!(px, red, "all pixels should be copied with radius=0");
        }
    }
}
