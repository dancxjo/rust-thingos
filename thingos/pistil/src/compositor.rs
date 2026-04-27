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

    let result = match generated_wallpaper_variant(path_str) {
        Some(variant) => draw_generated_wallpaper_cover(
            variant,
            dst,
            dst_stride_pixels as usize,
            dst_w as usize,
            dst_h as usize,
        ),
        None => draw_bmp_cover_into(
            path_str,
            dst,
            dst_stride_pixels as usize,
            dst_w as usize,
            dst_h as usize,
        ),
    };

    match result {
        Ok(()) => 0,
        Err(code) => code,
    }
}

fn generated_wallpaper_variant(path: &str) -> Option<u8> {
    if path.ends_with("/share/wallpapers/flower.bmp") {
        Some(0)
    } else if path.ends_with("/share/wallpapers/clouds.bmp") {
        Some(1)
    } else if path.ends_with("/share/wallpapers/leather.bmp") {
        Some(2)
    } else if path.ends_with("/share/wallpapers/linen.bmp") {
        Some(3)
    } else {
        None
    }
}

fn draw_generated_wallpaper_cover(
    variant: u8,
    dst: &mut [u32],
    dst_stride_pixels: usize,
    dst_w: usize,
    dst_h: usize,
) -> Result<(), i32> {
    const SRC_W: usize = 320;
    const SRC_H: usize = 180;
    const MAX_FAST_DST_W: usize = 4096;

    if dst_w == 0 || dst_h == 0 {
        return Err(-3);
    }

    let scale_x_fp = (dst_w as u64 * 1_000_000) / SRC_W as u64;
    let scale_y_fp = (dst_h as u64 * 1_000_000) / SRC_H as u64;
    let scale_fp = scale_x_fp.max(scale_y_fp).max(1);
    let scaled_w = (SRC_W as u64 * scale_fp) / 1_000_000;
    let scaled_h = (SRC_H as u64 * scale_fp) / 1_000_000;
    let ox = (dst_w as i64 - scaled_w as i64) / 2;
    let oy = (dst_h as i64 - scaled_h as i64) / 2;

    if dst_w <= MAX_FAST_DST_W {
        let mut sx_map = [0u16; MAX_FAST_DST_W];
        for dx in 0..dst_w {
            let sx_fp = ((dx as i64 - ox) * 1_000_000) / scale_fp as i64;
            sx_map[dx] = if sx_fp < 0 { 0 } else { (sx_fp as usize).min(SRC_W - 1) as u16 };
        }

        for dy in 0..dst_h {
            let sy_fp = ((dy as i64 - oy) * 1_000_000) / scale_fp as i64;
            let sy = if sy_fp < 0 { 0 } else { (sy_fp as usize).min(SRC_H - 1) };
            let dst_row = dy * dst_stride_pixels;
            for dx in 0..dst_w {
                let sx = sx_map[dx] as u32;
                let (r, g, b) = generated_wallpaper_pixel(variant, sx, sy as u32);
                dst[dst_row + dx] = 0xFF00_0000 | ((r as u32) << 16) | ((g as u32) << 8) | b as u32;
            }
        }

        return Ok(());
    }

    for dy in 0..dst_h {
        let sy_fp = ((dy as i64 - oy) * 1_000_000) / scale_fp as i64;
        let sy = if sy_fp < 0 { 0 } else { (sy_fp as usize).min(SRC_H - 1) };
        let dst_row = dy * dst_stride_pixels;
        for dx in 0..dst_w {
            let sx_fp = ((dx as i64 - ox) * 1_000_000) / scale_fp as i64;
            let sx = if sx_fp < 0 { 0 } else { (sx_fp as usize).min(SRC_W - 1) };
            let (r, g, b) = generated_wallpaper_pixel(variant, sx as u32, sy as u32);
            dst[dst_row + dx] = 0xFF00_0000 | ((r as u32) << 16) | ((g as u32) << 8) | b as u32;
        }
    }

    Ok(())
}

fn generated_wallpaper_pixel(variant: u8, x: u32, y: u32) -> (u8, u8, u8) {
    const WIDTH: u32 = 320;
    const HEIGHT: u32 = 180;

    match variant {
        0 => {
            let mut r = 22 + (x * 36 / WIDTH) as u8 + (y * 18 / HEIGHT) as u8;
            let mut g = 68 + (x * 54 / WIDTH) as u8;
            let mut b = 104 + (y * 44 / HEIGHT) as u8;
            let cx = WIDTH as i32 / 2;
            let cy = HEIGHT as i32 / 2;
            let xi = x as i32;
            let yi = y as i32;
            for (px, py) in [(0, -28), (27, -10), (18, 24), (-18, 24), (-27, -10)] {
                let dx = xi - (cx + px);
                let dy = yi - (cy + py);
                if dx * dx + dy * dy < 24 * 24 {
                    r = 236;
                    g = 168 + ((px + 28) as u8 % 40);
                    b = 198;
                }
            }
            let dx = xi - cx;
            let dy = yi - cy;
            if dx * dx + dy * dy < 18 * 18 {
                r = 245;
                g = 205;
                b = 78;
            }
            (r, g, b)
        }
        1 => {
            let base = 150 + (y * 70 / HEIGHT) as u8;
            let mut r = base.saturating_sub(20);
            let mut g = base;
            let mut b = 230;
            let xi = x as i32;
            let yi = y as i32;
            for (cx, cy, rx, ry) in [(80, 72, 48, 18), (145, 58, 58, 24), (220, 82, 52, 20)] {
                let dx = (xi - cx) * 100 / rx;
                let dy = (yi - cy) * 100 / ry;
                if dx * dx + dy * dy < 10000 {
                    r = 238;
                    g = 244;
                    b = 250;
                }
            }
            (r, g, b)
        }
        2 => {
            let grain = ((x * 37 + y * 19 + (x ^ y) * 11) & 31) as u8;
            (88 + grain, 56 + grain / 2, 34 + grain / 3)
        }
        _ => {
            let weave = (((x / 6) + (y / 4)) & 1) as u8 * 18;
            let thread = ((x * 5 + y * 3) & 15) as u8;
            (184 + weave + thread / 3, 188 + weave / 2 + thread / 4, 174 + thread / 2)
        }
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
