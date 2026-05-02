use alloc::boxed::Box;
use alloc::vec;

use abi::syscall::vfs_flags::O_RDONLY;
use fontdue::{Font, FontSettings};
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_stat};

use crate::Canvas;

pub const DEFAULT_FONT_PATH: &str = "/public/fonts/Inter-Regular.ttf";
pub const SYMBOL_FONT_PATH: &str = "/public/fonts/NotoSansSymbol2-Regular.ttf";
pub const DSEG7_FONT_PATH: &str = "/public/fonts/DSEG7Classic-Regular.ttf";

pub struct TextRenderer {
    pub font: Font,
}

impl TextRenderer {
    pub fn load_default() -> Option<Self> {
        Self::load_from_boot(DEFAULT_FONT_PATH)
    }

    pub fn load_from_boot(path: &str) -> Option<Self> {
        stem::debug!("pistil: opening font {}", path);
        let fd = vfs_open(path, O_RDONLY).ok()?;

        // Get size
        stem::debug!("pistil: stat font {}", path);
        let stat = vfs_stat(fd).ok()?;
        let size = stat.size as usize;

        stem::debug!("pistil: reading font {} bytes from {}", size, path);
        let mut data = vec![0u8; size];
        let n = vfs_read(fd, &mut data).ok()?;
        stem::debug!("pistil: font read returned {} of {} bytes", n, size);
        if n < size {
            let _ = vfs_close(fd);
            return None;
        }
        let _ = vfs_close(fd);

        stem::debug!("pistil: parsing font {}", path);
        let font = Font::from_bytes(data, FontSettings::default()).ok()?;
        stem::debug!("pistil: parsed font {}", path);
        Some(Self { font })
    }

    pub fn draw_text(
        &self,
        canvas: &mut Canvas,
        text: &str,
        x: i32,
        y: i32,
        px_size: f32,
        color: u32,
    ) {
        let mut cur_x = x as f32;

        for c in text.chars() {
            let (metrics, bitmap) = self.font.rasterize(c, px_size);

            // Blit glyph
            let glyph_x = (cur_x + metrics.xmin as f32) as i32;
            let glyph_y = (y as f32 - metrics.ymin as f32 - metrics.height as f32) as i32;

            for gy in 0..metrics.height {
                let dy = glyph_y + gy as i32;
                if dy < 0 || dy >= canvas.height as i32 {
                    continue;
                }

                for gx in 0..metrics.width {
                    let dx = glyph_x + gx as i32;
                    if dx < 0 || dx >= canvas.width as i32 {
                        continue;
                    }

                    let coverage = bitmap[gy * metrics.width + gx];
                    if coverage > 0 {
                        // Blend color with background
                        let alpha = (((color >> 24) * coverage as u32) / 255) as u8;
                        let effective_color = (color & 0x00FFFFFF) | ((alpha as u32) << 24);

                        let dst_idx = (dy as u32 * canvas.stride_pixels + dx as u32) as usize;
                        let dst_color = canvas.buffer[dst_idx];
                        canvas.buffer[dst_idx] = blend(dst_color, effective_color);
                    }
                }
            }

            cur_x += metrics.advance_width;
        }
    }
}

pub fn default_text_renderer() -> Option<&'static TextRenderer> {
    static mut RENDERER: *const TextRenderer = core::ptr::null();

    let renderer = unsafe { RENDERER };
    if !renderer.is_null() {
        return Some(unsafe { &*renderer });
    }

    let renderer = Box::leak(Box::new(TextRenderer::load_default()?)) as *const TextRenderer;
    unsafe {
        RENDERER = renderer;
        Some(&*renderer)
    }
}

pub fn symbol_text_renderer() -> Option<&'static TextRenderer> {
    static mut RENDERER: *const TextRenderer = core::ptr::null();

    let renderer = unsafe { RENDERER };
    if !renderer.is_null() {
        return Some(unsafe { &*renderer });
    }

    let renderer =
        Box::leak(Box::new(TextRenderer::load_from_boot(SYMBOL_FONT_PATH)?)) as *const TextRenderer;
    unsafe {
        RENDERER = renderer;
        Some(&*renderer)
    }
}

pub fn dseg7_text_renderer() -> Option<&'static TextRenderer> {
    static mut RENDERER: *const TextRenderer = core::ptr::null();

    let renderer = unsafe { RENDERER };
    if !renderer.is_null() {
        return Some(unsafe { &*renderer });
    }

    let renderer =
        Box::leak(Box::new(TextRenderer::load_from_boot(DSEG7_FONT_PATH)?)) as *const TextRenderer;
    unsafe {
        RENDERER = renderer;
        Some(&*renderer)
    }
}

fn blend(dst: u32, src: u32) -> u32 {
    let sa = (src >> 24) as u32;
    if sa == 0 {
        return dst;
    }
    if sa == 255 {
        return src;
    }

    let da = (dst >> 24) as u32;
    let inv_sa = 255 - sa;

    // out_a = sa + da * (255 - sa) / 255
    let out_a = sa + (da * inv_sa + 127) / 255;
    if out_a == 0 {
        return 0;
    }

    let sr = (src >> 16) & 0xFF;
    let sg = (src >> 8) & 0xFF;
    let sb = src & 0xFF;

    let dr = (dst >> 16) & 0xFF;
    let dg = (dst >> 8) & 0xFF;
    let db = dst & 0xFF;

    // Standard Porter-Duff "Over" (non-premultiplied src, non-premultiplied dst)
    // C_out = (C_src * a_src + C_dst * a_dst * (1 - a_src)) / a_out
    let r = (sr * sa + (dr * da * inv_sa + 127) / 255 + out_a / 2) / out_a;
    let g = (sg * sa + (dg * da * inv_sa + 127) / 255 + out_a / 2) / out_a;
    let b = (sb * sa + (db * da * inv_sa + 127) / 255 + out_a / 2) / out_a;

    (out_a << 24) | (r.min(255) << 16) | (g.min(255) << 8) | b.min(255)
}
