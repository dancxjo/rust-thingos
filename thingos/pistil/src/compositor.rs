use alloc::vec::Vec;

use abi::syscall::vfs_flags::O_RDONLY;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BlitRect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub fn blit_centered_nearest(
    dst: &mut [u32],
    dst_stride_pixels: usize,
    dst_w: usize,
    dst_h: usize,
    src: &[u32],
    src_stride_pixels: usize,
    src_w: usize,
    src_h: usize,
) -> BlitRect {
    if src_w == 0 || src_h == 0 || dst_w == 0 || dst_h == 0 {
        return BlitRect::default();
    }

    let scale_x = (dst_w as u64 * 1_000_000) / src_w as u64;
    let scale_y = (dst_h as u64 * 1_000_000) / src_h as u64;
    let scale = scale_x.min(scale_y).max(1);

    let scaled_w = ((src_w as u64 * scale) / 1_000_000) as usize;
    let scaled_h = ((src_h as u64 * scale) / 1_000_000) as usize;
    let offset_x = (dst_w.saturating_sub(scaled_w)) / 2;
    let offset_y = (dst_h.saturating_sub(scaled_h)) / 2;

    if scaled_w == src_w && scaled_h == src_h {
        for y in 0..src_h {
            let dst_row = (offset_y + y) * dst_stride_pixels + offset_x;
            let src_row = y * src_stride_pixels;
            let dst_span = &mut dst[dst_row..dst_row + src_w];
            let src_span = &src[src_row..src_row + src_w];
            copy_row_u32_fast(dst_span, src_span);
        }
        return BlitRect { x: offset_x, y: offset_y, width: scaled_w, height: scaled_h };
    }

    let mut sx_lut = Vec::with_capacity(scaled_w);
    sx_lut.resize(scaled_w, 0);
    for (dx, sx) in sx_lut.iter_mut().enumerate() {
        *sx = ((dx as u64 * src_w as u64) / scaled_w as u64) as usize;
    }

    let mut sy_lut = Vec::with_capacity(scaled_h);
    sy_lut.resize(scaled_h, 0);
    for (dy, sy) in sy_lut.iter_mut().enumerate() {
        *sy = ((dy as u64 * src_h as u64) / scaled_h as u64) as usize;
    }

    for dy in 0..scaled_h {
        let sy = sy_lut[dy];
        let dst_row = (offset_y + dy) * dst_stride_pixels;
        let src_row = sy * src_stride_pixels;
        for dx in 0..scaled_w {
            dst[dst_row + offset_x + dx] = src[src_row + sx_lut[dx]];
        }
    }

    BlitRect { x: offset_x, y: offset_y, width: scaled_w, height: scaled_h }
}

pub fn blit_cover_nearest(
    dst: &mut [u32],
    dst_stride_pixels: usize,
    dst_w: usize,
    dst_h: usize,
    src: &[u32],
    src_stride_pixels: usize,
    src_w: usize,
    src_h: usize,
) {
    if src_w == 0 || src_h == 0 || dst_w == 0 || dst_h == 0 {
        return;
    }

    let scale_x_fp = (dst_w as u64 * 1_000_000) / src_w as u64;
    let scale_y_fp = (dst_h as u64 * 1_000_000) / src_h as u64;
    let scale_fp = scale_x_fp.max(scale_y_fp);

    let sw = (src_w as u64 * scale_fp) / 1_000_000;
    let sh = (src_h as u64 * scale_fp) / 1_000_000;
    let ox = (dst_w as i64 - sw as i64) / 2;
    let oy = (dst_h as i64 - sh as i64) / 2;

    for dy in 0..dst_h {
        let sy_fp = ((dy as i64 - oy) * 1_000_000) / scale_fp as i64;
        let sy = (sy_fp as usize).min(src_h - 1);
        let dst_row = dy * dst_stride_pixels;
        let src_row = sy * src_stride_pixels;

        for dx in 0..dst_w {
            let sx_fp = ((dx as i64 - ox) * 1_000_000) / scale_fp as i64;
            let sx = (sx_fp as usize).min(src_w - 1);
            dst[dst_row + dx] = src[src_row + sx];
        }
    }
}

#[inline(always)]
pub fn copy_row_u32_fast(dst: &mut [u32], src: &[u32]) {
    debug_assert_eq!(dst.len(), src.len());

    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    unsafe {
        use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};

        let len = dst.len();
        let mut i = 0usize;
        while i + 4 <= len {
            let s = _mm_loadu_si128(src.as_ptr().add(i) as *const __m128i);
            _mm_storeu_si128(dst.as_mut_ptr().add(i) as *mut __m128i, s);
            i += 4;
        }
        if i < len {
            dst[i..].copy_from_slice(&src[i..]);
        }
        return;
    }

    #[cfg(all(target_arch = "x86", target_feature = "sse2"))]
    unsafe {
        use core::arch::x86::{__m128i, _mm_loadu_si128, _mm_storeu_si128};

        let len = dst.len();
        let mut i = 0usize;
        while i + 4 <= len {
            let s = _mm_loadu_si128(src.as_ptr().add(i) as *const __m128i);
            _mm_storeu_si128(dst.as_mut_ptr().add(i) as *mut __m128i, s);
            i += 4;
        }
        if i < len {
            dst[i..].copy_from_slice(&src[i..]);
        }
        return;
    }

    #[allow(unreachable_code)]
    dst.copy_from_slice(src);
}

#[unsafe(no_mangle)]
pub extern "C" fn pistil_prepare_background(
    path_ptr: *const u8,
    dst_ptr: *mut u32,
    dst_w: u32,
    dst_h: u32,
    dst_stride_pixels: u32,
) -> i32 {
    let mut len = 0;
    while unsafe { *path_ptr.add(len) } != 0 {
        len += 1;
    }
    let path = unsafe { core::slice::from_raw_parts(path_ptr, len) };
    let Ok(path_str) = core::str::from_utf8(path) else {
        return -2;
    };

    let dst =
        unsafe { core::slice::from_raw_parts_mut(dst_ptr, (dst_h * dst_stride_pixels) as usize) };

    // Paint periwinkle first as diagnostic baseline
    dst.fill(0xFFCCCCFF);

    let result = draw_bmp_cover_into(
        path_str,
        dst,
        dst_stride_pixels as usize,
        dst_w as usize,
        dst_h as usize,
    );

    match result {
        Ok(()) => 0,
        Err(code) => code,
    }
}

fn draw_bmp_cover_into(
    path: &str,
    dst: &mut [u32],
    dst_stride_pixels: usize,
    dst_w: usize,
    dst_h: usize,
) -> Result<(), i32> {
    const MAX_ROW_BYTES: usize = 16 * 1024;

    let fd = vfs_open(path, O_RDONLY).map_err(|_| -3)?;
    let result = (|| {
        let mut header = [0u8; 54];
        read_exact(fd, &mut header).map_err(|_| -3)?;

        if &header[0..2] != b"BM" {
            return Err(-3);
        }

        let pixel_offset =
            u32::from_le_bytes([header[10], header[11], header[12], header[13]]) as usize;
        let width_i = i32::from_le_bytes([header[18], header[19], header[20], header[21]]);
        let height_i = i32::from_le_bytes([header[22], header[23], header[24], header[25]]);
        let bpp = u16::from_le_bytes([header[28], header[29]]);
        let compression = u32::from_le_bytes([header[30], header[31], header[32], header[33]]);

        if width_i <= 0 || height_i == 0 || compression != 0 {
            return Err(-3);
        }

        let src_w = width_i as usize;
        let src_h = height_i.checked_abs().ok_or(-3)? as usize;
        let bottom_up = height_i > 0;
        if src_w == 0 || src_h == 0 || dst_w == 0 || dst_h == 0 {
            return Err(-3);
        }

        let row_bytes = match bpp {
            24 => src_w.checked_mul(3).and_then(|n| n.checked_add(3)).map(|n| n & !3),
            32 => src_w.checked_mul(4),
            _ => None,
        }
        .ok_or(-3)?;
        if row_bytes > MAX_ROW_BYTES {
            return Err(-3);
        }

        let mut remaining_to_skip = pixel_offset.checked_sub(54).ok_or(-3)?;
        let mut skip = [0u8; 512];
        while remaining_to_skip > 0 {
            let n = remaining_to_skip.min(skip.len());
            read_exact(fd, &mut skip[..n]).map_err(|_| -3)?;
            remaining_to_skip -= n;
        }

        let scale_x_fp = (dst_w as u64 * 1_000_000) / src_w as u64;
        let scale_y_fp = (dst_h as u64 * 1_000_000) / src_h as u64;
        let scale_fp = scale_x_fp.max(scale_y_fp).max(1);
        let scaled_w = (src_w as u64 * scale_fp) / 1_000_000;
        let scaled_h = (src_h as u64 * scale_fp) / 1_000_000;
        let ox = (dst_w as i64 - scaled_w as i64) / 2;
        let oy = (dst_h as i64 - scaled_h as i64) / 2;

        let mut row = [0u8; MAX_ROW_BYTES];
        for file_y in 0..src_h {
            read_exact(fd, &mut row[..row_bytes]).map_err(|_| -3)?;
            let src_y = if bottom_up { src_h - 1 - file_y } else { file_y };

            for dy in 0..dst_h {
                let sy_fp = ((dy as i64 - oy) * 1_000_000) / scale_fp as i64;
                if sy_fp < 0 {
                    continue;
                }
                let mapped_y = (sy_fp as usize).min(src_h - 1);
                if mapped_y != src_y {
                    continue;
                }

                let dst_row = dy * dst_stride_pixels;
                for dx in 0..dst_w {
                    let sx_fp = ((dx as i64 - ox) * 1_000_000) / scale_fp as i64;
                    if sx_fp < 0 {
                        continue;
                    }
                    let sx = (sx_fp as usize).min(src_w - 1);
                    dst[dst_row + dx] = match bpp {
                        24 => {
                            let off = sx * 3;
                            let b = row[off] as u32;
                            let g = row[off + 1] as u32;
                            let r = row[off + 2] as u32;
                            0xFF00_0000 | (r << 16) | (g << 8) | b
                        }
                        32 => {
                            let off = sx * 4;
                            let b = row[off] as u32;
                            let g = row[off + 1] as u32;
                            let r = row[off + 2] as u32;
                            let a = row[off + 3] as u32;
                            (a << 24) | (r << 16) | (g << 8) | b
                        }
                        _ => return Err(-3),
                    };
                }
            }
        }

        Ok(())
    })();

    let _ = vfs_close(fd);
    result
}

fn read_exact(fd: u32, mut buf: &mut [u8]) -> Result<(), ()> {
    while !buf.is_empty() {
        let n = vfs_read(fd, buf).map_err(|_| ())?;
        if n == 0 {
            return Err(());
        }
        let rest = buf;
        buf = &mut rest[n..];
    }
    Ok(())
}
