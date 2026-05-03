use alloc::vec::Vec;

use abi::syscall::vfs_flags::O_RDONLY;
use pistil_types::Canvas;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_stat};
use tiny_skia::Pixmap;

use crate::font::{
    DEFAULT_FONT_PATH, DSEG7_FONT_PATH, SYMBOL_FONT_PATH, TextRenderer, default_text_renderer,
    dseg7_text_renderer, symbol_text_renderer,
};

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

    let result = draw_image_cover_into(
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

#[unsafe(no_mangle)]
pub extern "C" fn pistil_draw_debug_text(
    text_ptr: *const u8,
    dst_ptr: *mut u32,
    dst_w: u32,
    dst_h: u32,
    dst_stride_pixels: u32,
) -> i32 {
    if text_ptr.is_null() || dst_ptr.is_null() || dst_w == 0 || dst_h == 0 {
        return -3;
    }

    let mut len = 0usize;
    while unsafe { *text_ptr.add(len) } != 0 && len < 512 {
        len += 1;
    }
    let text = unsafe { core::slice::from_raw_parts(text_ptr, len) };
    let dst =
        unsafe { core::slice::from_raw_parts_mut(dst_ptr, (dst_h * dst_stride_pixels) as usize) };

    draw_debug_background(dst, dst_stride_pixels as usize, dst_w as usize, dst_h as usize);
    let Ok(text) = core::str::from_utf8(text) else {
        return -2;
    };

    let Some(renderer) = default_text_renderer() else {
        stem::error!("pistil: failed to load default font {}", DEFAULT_FONT_PATH);
        return -5;
    };

    stem::info!("pistil: drawing debug text with {}", DEFAULT_FONT_PATH);
    let mut canvas = Canvas::new(dst, dst_w, dst_h, dst_stride_pixels);
    draw_debug_text_lines(&renderer, &mut canvas, text);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn pistil_draw_text(
    text_ptr: *const u8,
    dst_ptr: *mut u32,
    dst_w: u32,
    dst_h: u32,
    dst_stride_pixels: u32,
    x: i32,
    y: i32,
    px_size: f32,
    color: u32,
) -> i32 {
    if text_ptr.is_null()
        || dst_ptr.is_null()
        || dst_w == 0
        || dst_h == 0
        || dst_stride_pixels < dst_w
        || px_size <= 0.0
    {
        return -3;
    }

    let mut len = 0usize;
    while unsafe { *text_ptr.add(len) } != 0 && len < 512 {
        len += 1;
    }
    let text = unsafe { core::slice::from_raw_parts(text_ptr, len) };
    let Ok(text) = core::str::from_utf8(text) else {
        return -2;
    };

    let Some(renderer) = default_text_renderer() else {
        stem::error!("pistil: failed to load default font {}", DEFAULT_FONT_PATH);
        return -5;
    };

    let dst =
        unsafe { core::slice::from_raw_parts_mut(dst_ptr, (dst_h * dst_stride_pixels) as usize) };
    let mut canvas = Canvas::new(dst, dst_w, dst_h, dst_stride_pixels);
    renderer.draw_text(&mut canvas, text, x, y, px_size * 1.25, color);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn pistil_default_font_ready() -> i32 {
    if default_text_renderer().is_some() { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub extern "C" fn pistil_draw_symbol_text(
    text_ptr: *const u8,
    dst_ptr: *mut u32,
    dst_w: u32,
    dst_h: u32,
    dst_stride_pixels: u32,
    x: i32,
    y: i32,
    px_size: f32,
    color: u32,
) -> i32 {
    if text_ptr.is_null()
        || dst_ptr.is_null()
        || dst_w == 0
        || dst_h == 0
        || dst_stride_pixels < dst_w
        || px_size <= 0.0
    {
        return -3;
    }

    let mut len = 0usize;
    while unsafe { *text_ptr.add(len) } != 0 && len < 512 {
        len += 1;
    }
    let text = unsafe { core::slice::from_raw_parts(text_ptr, len) };
    let Ok(text) = core::str::from_utf8(text) else {
        return -2;
    };

    let Some(renderer) = symbol_text_renderer() else {
        stem::error!("pistil: failed to load symbol font {}", SYMBOL_FONT_PATH);
        return -5;
    };

    let dst =
        unsafe { core::slice::from_raw_parts_mut(dst_ptr, (dst_h * dst_stride_pixels) as usize) };
    let mut canvas = Canvas::new(dst, dst_w, dst_h, dst_stride_pixels);
    renderer.draw_text(&mut canvas, text, x, y, px_size * 1.25, color);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn pistil_symbol_font_ready() -> i32 {
    if symbol_text_renderer().is_some() { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub extern "C" fn pistil_draw_dseg7_text(
    text_ptr: *const u8,
    dst_ptr: *mut u32,
    dst_w: u32,
    dst_h: u32,
    dst_stride_pixels: u32,
    x: i32,
    y: i32,
    px_size: f32,
    color: u32,
) -> i32 {
    if text_ptr.is_null()
        || dst_ptr.is_null()
        || dst_w == 0
        || dst_h == 0
        || dst_stride_pixels < dst_w
        || px_size <= 0.0
    {
        return -3;
    }

    let mut len = 0usize;
    while unsafe { *text_ptr.add(len) } != 0 && len < 512 {
        len += 1;
    }
    let text = unsafe { core::slice::from_raw_parts(text_ptr, len) };
    let Ok(text) = core::str::from_utf8(text) else {
        return -2;
    };

    let Some(renderer) = dseg7_text_renderer() else {
        stem::error!("pistil: failed to load DSEG7 font {}", DSEG7_FONT_PATH);
        return -5;
    };

    let dst =
        unsafe { core::slice::from_raw_parts_mut(dst_ptr, (dst_h * dst_stride_pixels) as usize) };
    let mut canvas = Canvas::new(dst, dst_w, dst_h, dst_stride_pixels);
    renderer.draw_text(&mut canvas, text, x, y, px_size * 1.25, color);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn pistil_prepare_cursor(
    path_ptr: *const u8,
    dst_ptr: *mut u32,
    dst_w: u32,
    dst_h: u32,
    dst_stride_pixels: u32,
    hotspot_out: *mut u32,
) -> i32 {
    if dst_ptr.is_null() || dst_w == 0 || dst_h == 0 || dst_stride_pixels < dst_w {
        return -3;
    }

    let path = if path_ptr.is_null() {
        None
    } else {
        let mut len = 0usize;
        while unsafe { *path_ptr.add(len) } != 0 && len < 256 {
            len += 1;
        }
        let bytes = unsafe { core::slice::from_raw_parts(path_ptr, len) };
        core::str::from_utf8(bytes).ok()
    };

    let dst =
        unsafe { core::slice::from_raw_parts_mut(dst_ptr, (dst_h * dst_stride_pixels) as usize) };
    let svg_bytes = path.and_then(|p| read_vfs_file(p).ok());
    let bytes = svg_bytes.as_deref().unwrap_or(svg::DEFAULT_CURSOR_SVG);

    match svg::rasterize_cursor(bytes, dst, dst_w, dst_h, dst_stride_pixels) {
        Ok(hotspot) => {
            if !hotspot_out.is_null() {
                unsafe {
                    *hotspot_out.add(0) = hotspot.x;
                    *hotspot_out.add(1) = hotspot.y;
                }
            }
            0
        }
        Err(_) => -3,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pistil_draw_svg_icon(
    path_ptr: *const u8,
    dst_ptr: *mut u32,
    dst_w: u32,
    dst_h: u32,
    dst_stride_pixels: u32,
    x: i32,
    y: i32,
    icon_w: u32,
    icon_h: u32,
    color: u32,
) -> i32 {
    if path_ptr.is_null()
        || dst_ptr.is_null()
        || dst_w == 0
        || dst_h == 0
        || dst_stride_pixels < dst_w
        || icon_w == 0
        || icon_h == 0
    {
        return -3;
    }

    let mut len = 0usize;
    while unsafe { *path_ptr.add(len) } != 0 && len < 256 {
        len += 1;
    }
    let path = unsafe { core::slice::from_raw_parts(path_ptr, len) };
    let Ok(path) = core::str::from_utf8(path) else {
        return -2;
    };
    let Ok(svg_bytes) = read_vfs_file(path) else {
        return -3;
    };

    let scale = 4u32;
    let raster_w = icon_w.saturating_mul(scale);
    let raster_h = icon_h.saturating_mul(scale);
    if raster_w == 0 || raster_h == 0 {
        return -3;
    }

    let mut scratch = Vec::new();
    let scratch_len = raster_w.saturating_mul(raster_h) as usize;
    if scratch.try_reserve_exact(scratch_len).is_err() {
        return -4;
    }
    scratch.resize(scratch_len, 0u32);
    if svg::rasterize(&svg_bytes, &mut scratch, raster_w, raster_h, raster_w).is_err() {
        return -3;
    }

    let dst =
        unsafe { core::slice::from_raw_parts_mut(dst_ptr, (dst_h * dst_stride_pixels) as usize) };
    blend_supersampled_alpha_icon(
        dst,
        dst_stride_pixels,
        dst_w,
        dst_h,
        &scratch,
        raster_w,
        x,
        y,
        icon_w,
        icon_h,
        scale,
        color,
    );
    0
}

fn blend_supersampled_alpha_icon(
    dst: &mut [u32],
    dst_stride_pixels: u32,
    dst_w: u32,
    dst_h: u32,
    src: &[u32],
    src_stride_pixels: u32,
    x: i32,
    y: i32,
    icon_w: u32,
    icon_h: u32,
    scale: u32,
    color: u32,
) {
    let color_alpha = (color >> 24) & 0xFF;
    let rgb = color & 0x00FF_FFFF;
    let samples = scale.saturating_mul(scale).max(1);

    for dy in 0..icon_h {
        let out_y = y.saturating_add(dy as i32);
        if out_y < 0 || out_y >= dst_h as i32 {
            continue;
        }

        for dx in 0..icon_w {
            let out_x = x.saturating_add(dx as i32);
            if out_x < 0 || out_x >= dst_w as i32 {
                continue;
            }

            let mut alpha_sum = 0u32;
            let src_x0 = dx.saturating_mul(scale);
            let src_y0 = dy.saturating_mul(scale);
            for sy in 0..scale {
                let src_row = (src_y0 + sy).saturating_mul(src_stride_pixels);
                for sx in 0..scale {
                    let idx = src_row.saturating_add(src_x0 + sx) as usize;
                    alpha_sum = alpha_sum.saturating_add(src.get(idx).copied().unwrap_or(0) >> 24);
                }
            }

            let alpha = ((alpha_sum / samples) * color_alpha + 127) / 255;
            if alpha == 0 {
                continue;
            }

            let src_px = (alpha << 24) | rgb;
            let dst_idx = (out_y as u32)
                .saturating_mul(dst_stride_pixels)
                .saturating_add(out_x as u32) as usize;
            if let Some(dst_px) = dst.get_mut(dst_idx) {
                *dst_px = blend_argb_over(*dst_px, src_px);
            }
        }
    }
}

fn blend_argb_over(dst: u32, src: u32) -> u32 {
    let sa = (src >> 24) & 0xFF;
    if sa == 0 {
        return dst;
    }
    if sa == 255 {
        return src;
    }

    let inv_sa = 255 - sa;
    let sr = (src >> 16) & 0xFF;
    let sg = (src >> 8) & 0xFF;
    let sb = src & 0xFF;
    let da = (dst >> 24) & 0xFF;
    let dr = (dst >> 16) & 0xFF;
    let dg = (dst >> 8) & 0xFF;
    let db = dst & 0xFF;

    let out_a = sa + (da * inv_sa + 127) / 255;
    if out_a == 0 {
        return 0;
    }
    let r = (sr * sa + (dr * da * inv_sa + 127) / 255 + out_a / 2) / out_a;
    let g = (sg * sa + (dg * da * inv_sa + 127) / 255 + out_a / 2) / out_a;
    let b = (sb * sa + (db * da * inv_sa + 127) / 255 + out_a / 2) / out_a;
    (out_a << 24) | (r.min(255) << 16) | (g.min(255) << 8) | b.min(255)
}

fn draw_debug_text_lines(renderer: &TextRenderer, canvas: &mut Canvas, text: &str) {
    let mut y = 96;
    for line in text.lines() {
        renderer.draw_text(canvas, line, 48, y, 42.0, 0xFFFFF4B0);
        y += 56;
    }
}

fn draw_debug_background(dst: &mut [u32], stride: usize, width: usize, height: usize) {
    for y in 0..height {
        let row = y * stride;
        for x in 0..width {
            let band = ((x / 48) + (y / 48)) & 1;
            let base = if band == 0 { 0xFF102030 } else { 0xFF17324A };
            dst[row + x] = base;
        }
    }
}

fn draw_image_cover_into(
    path: &str,
    dst: &mut [u32],
    dst_stride_pixels: usize,
    dst_w: usize,
    dst_h: usize,
) -> Result<(), i32> {
    if path.ends_with(".png") {
        draw_png_cover_into(path, dst, dst_stride_pixels, dst_w, dst_h)
    } else {
        draw_bmp_cover_into(path, dst, dst_stride_pixels, dst_w, dst_h)
    }
}

fn draw_png_cover_into(
    path: &str,
    dst: &mut [u32],
    dst_stride_pixels: usize,
    dst_w: usize,
    dst_h: usize,
) -> Result<(), i32> {
    let t_read_start = stem::time::monotonic_ns();
    let data = read_vfs_file(path)?;
    let t_decode_start = stem::time::monotonic_ns();
    let pixmap = Pixmap::decode_png(&data).map_err(|_| -3)?;
    let t_blit_start = stem::time::monotonic_ns();
    let src_w = pixmap.width() as usize;
    let src_h = pixmap.height() as usize;
    if src_w == 0 || src_h == 0 || dst_w == 0 || dst_h == 0 {
        return Err(-3);
    }

    blit_cover_tiny_skia(dst, dst_stride_pixels, dst_w, dst_h, &pixmap);
    let t_done = stem::time::monotonic_ns();
    stem::debug!(
        "Wallpaper PNG phases: read={}ms decode={}ms blit={}ms ({}x{} -> {}x{})",
        (t_decode_start - t_read_start) / 1_000_000,
        (t_blit_start - t_decode_start) / 1_000_000,
        (t_done - t_blit_start) / 1_000_000,
        src_w, src_h, dst_w, dst_h,
    );
    Ok(())
}

fn read_vfs_file(path: &str) -> Result<Vec<u8>, i32> {
    const MAX_IMAGE_BYTES: u64 = 16 * 1024 * 1024;

    let fd = vfs_open(path, O_RDONLY).map_err(|_| -3)?;
    let result = (|| {
        let stat = vfs_stat(fd).map_err(|_| -3)?;
        if stat.size == 0 || stat.size > MAX_IMAGE_BYTES {
            return Err(-3);
        }

        let mut data = Vec::new();
        data.resize(stat.size as usize, 0);
        let mut filled = 0usize;
        while filled < data.len() {
            let n = vfs_read(fd, &mut data[filled..]).map_err(|_| -3)?;
            if n == 0 {
                return Err(-3);
            }
            filled += n;
        }
        Ok(data)
    })();

    let _ = vfs_close(fd);
    result
}

fn blit_cover_tiny_skia(
    dst: &mut [u32],
    dst_stride_pixels: usize,
    dst_w: usize,
    dst_h: usize,
    src: &Pixmap,
) {
    let src_w = src.width() as usize;
    let src_h = src.height() as usize;
    let scale_x_fp = (dst_w as u64 * 1_000_000) / src_w as u64;
    let scale_y_fp = (dst_h as u64 * 1_000_000) / src_h as u64;
    let scale_fp = scale_x_fp.max(scale_y_fp).max(1);

    let scaled_w = (src_w as u64 * scale_fp) / 1_000_000;
    let scaled_h = (src_h as u64 * scale_fp) / 1_000_000;
    let ox = (dst_w as i64 - scaled_w as i64) / 2;
    let oy = (dst_h as i64 - scaled_h as i64) / 2;
    let pixels = src.pixels();

    for dy in 0..dst_h {
        let sy_fp = ((dy as i64 - oy) * 1_000_000) / scale_fp as i64;
        let sy = if sy_fp < 0 { 0 } else { (sy_fp as usize).min(src_h - 1) };
        let dst_row = dy * dst_stride_pixels;
        let src_row = sy * src_w;

        for dx in 0..dst_w {
            let sx_fp = ((dx as i64 - ox) * 1_000_000) / scale_fp as i64;
            let sx = if sx_fp < 0 { 0 } else { (sx_fp as usize).min(src_w - 1) };
            let c = pixels[src_row + sx];
            dst[dst_row + dx] = premultiplied_rgba_to_argb(c.red(), c.green(), c.blue(), c.alpha());
        }
    }
}

fn premultiplied_rgba_to_argb(r: u8, g: u8, b: u8, a: u8) -> u32 {
    let a = a as u32;
    if a == 0 {
        return 0xFFCCCCFF;
    }

    let r = r as u32;
    let g = g as u32;
    let b = b as u32;
    let inv_a = 255 - a;
    let bg_r = 0xCC;
    let bg_g = 0xCC;
    let bg_b = 0xFF;

    let out_r = r + (bg_r * inv_a + 127) / 255;
    let out_g = g + (bg_g * inv_a + 127) / 255;
    let out_b = b + (bg_b * inv_a + 127) / 255;
    0xFF00_0000 | (out_r.min(255) << 16) | (out_g.min(255) << 8) | out_b.min(255)
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
