use alloc::vec::Vec;

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

    let Some(mut wallpaper) = crate::bmp::load_bmp(path_str) else {
        return -3;
    };

    let src_w = wallpaper.width as usize;
    let src_h = wallpaper.height as usize;
    let src_stride = (wallpaper.stride / 4) as usize;
    let src = wallpaper.as_slice_mut();

    blit_cover_nearest(
        dst,
        dst_stride_pixels as usize,
        dst_w as usize,
        dst_h as usize,
        src,
        src_stride,
        src_w,
        src_h,
    );

    0
}
