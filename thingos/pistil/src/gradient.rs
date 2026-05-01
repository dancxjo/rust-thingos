//! Cached gradient strips for title bars and panels.
//!
//! Vertical linear gradients are cheap to compute but can add up across many
//! windows per frame.  This module maintains a small LRU-like cache of
//! rendered gradient strips so repeated calls with the same parameters pay
//! only a slice-copy cost.

use alloc::vec;
use alloc::vec::Vec;

const GRADIENT_CACHE_CAPACITY: usize = 8;

struct GradientEntry {
    width: u32,
    height: u32,
    color_top: u32,
    color_bottom: u32,
    pixels: Vec<u32>,
}

/// Cache of rendered gradient strips.
///
/// Create one instance per compositor session and pass it to every frame's
/// painting calls.
pub struct GradientCache {
    entries: Vec<GradientEntry>,
}

impl GradientCache {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// Return a reference to a cached gradient strip, generating and caching
    /// one if it does not exist yet.
    ///
    /// The returned slice contains `width * height` pixels in row-major order
    /// with stride == `width`.
    pub fn get_or_create(
        &mut self,
        width: u32,
        height: u32,
        color_top: u32,
        color_bottom: u32,
    ) -> &[u32] {
        if let Some(pos) = self.entries.iter().position(|e| {
            e.width == width
                && e.height == height
                && e.color_top == color_top
                && e.color_bottom == color_bottom
        }) {
            return &self.entries[pos].pixels;
        }

        // Evict the oldest entry when at capacity.
        if self.entries.len() >= GRADIENT_CACHE_CAPACITY {
            self.entries.remove(0);
        }

        let pixels_len = (width as usize).saturating_mul(height as usize);
        let mut pixels = vec![0u32; pixels_len];
        render_vertical_gradient(&mut pixels, width, height, color_top, color_bottom);
        self.entries.push(GradientEntry { width, height, color_top, color_bottom, pixels });
        &self.entries.last().unwrap().pixels
    }

    /// Blit a vertical gradient into `dst` at screen position (`dx`, `dy`).
    ///
    /// Uses the cache so the gradient is only computed once per unique
    /// `(width, height, color_top, color_bottom)` combination.
    pub fn draw(
        &mut self,
        dst: &mut [u32],
        dst_stride: u32,
        dst_h: u32,
        dx: i32,
        dy: i32,
        width: u32,
        height: u32,
        color_top: u32,
        color_bottom: u32,
    ) {
        let strip = self.get_or_create(width, height, color_top, color_bottom);
        blit_strip(dst, dst_stride, dst_h, dx, dy, width, height, strip);
    }

    /// Returns the number of entries currently in the cache.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if the cache contains no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for GradientCache {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Internal rendering helpers
// ---------------------------------------------------------------------------

fn render_vertical_gradient(
    pixels: &mut [u32],
    width: u32,
    height: u32,
    color_top: u32,
    color_bottom: u32,
) {
    if width == 0 || height == 0 {
        return;
    }
    for row in 0..height {
        let t = if height > 1 {
            (row * 255 + (height - 1) / 2) / (height - 1)
        } else {
            0
        };
        let color = lerp_argb(color_top, color_bottom, t);
        let row_start = (row * width) as usize;
        if row_start + width as usize <= pixels.len() {
            pixels[row_start..row_start + width as usize].fill(color);
        }
    }
}

/// Linear interpolation between two ARGB colours.
///
/// `t` ranges from 0 (all `a`) to 255 (all `b`).
fn lerp_argb(a: u32, b: u32, t: u32) -> u32 {
    let inv_t = 255 - t;
    let aa = (a >> 24) & 0xFF;
    let ar = (a >> 16) & 0xFF;
    let ag = (a >> 8) & 0xFF;
    let ab = a & 0xFF;
    let ba = (b >> 24) & 0xFF;
    let br = (b >> 16) & 0xFF;
    let bg = (b >> 8) & 0xFF;
    let bb = b & 0xFF;
    let oa = (aa * inv_t + ba * t + 127) / 255;
    let or_ = (ar * inv_t + br * t + 127) / 255;
    let og = (ag * inv_t + bg * t + 127) / 255;
    let ob = (ab * inv_t + bb * t + 127) / 255;
    (oa << 24) | (or_ << 16) | (og << 8) | ob
}

fn blit_strip(
    dst: &mut [u32],
    dst_stride: u32,
    dst_h: u32,
    dx: i32,
    dy: i32,
    width: u32,
    height: u32,
    strip: &[u32],
) {
    for row in 0..height {
        let out_y = dy + row as i32;
        if out_y < 0 || out_y >= dst_h as i32 {
            continue;
        }
        let src_off = (row * width) as usize;
        let dst_row_off = (out_y as u32 * dst_stride) as usize;
        for col in 0..width {
            let out_x = dx + col as i32;
            if out_x < 0 || out_x >= dst_stride as i32 {
                continue;
            }
            let si = src_off + col as usize;
            let di = dst_row_off + out_x as usize;
            if si < strip.len() && di < dst.len() {
                dst[di] = strip[si];
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_key_returns_identical_strip() {
        let mut cache = GradientCache::new();
        let a = cache.get_or_create(4, 2, 0xFF000000, 0xFFFFFFFF).to_vec();
        let b = cache.get_or_create(4, 2, 0xFF000000, 0xFFFFFFFF).to_vec();
        assert_eq!(a, b);
    }

    #[test]
    fn top_row_matches_top_color() {
        let mut cache = GradientCache::new();
        let pix = cache.get_or_create(4, 4, 0xFFFF0000, 0xFF0000FF);
        assert_eq!(pix[0], 0xFFFF0000, "top-left pixel should be the top color");
    }

    #[test]
    fn bottom_row_matches_bottom_color() {
        let mut cache = GradientCache::new();
        let pix = cache.get_or_create(4, 4, 0xFFFF0000, 0xFF0000FF);
        assert_eq!(pix[4 * 3], 0xFF0000FF, "first pixel of bottom row should be the bottom color");
    }

    #[test]
    fn cache_evicts_when_full() {
        let mut cache = GradientCache::new();
        // Fill past capacity
        for i in 0..=GRADIENT_CACHE_CAPACITY {
            cache.get_or_create(4, 4, i as u32, i as u32 + 1);
        }
        assert!(cache.entries.len() <= GRADIENT_CACHE_CAPACITY);
    }

    #[test]
    fn single_row_gradient_does_not_panic() {
        let mut cache = GradientCache::new();
        let pix = cache.get_or_create(8, 1, 0xFFAAAAAA, 0xFF555555);
        assert_eq!(pix.len(), 8);
        // Single-row gradient: top colour wins
        assert_eq!(pix[0], 0xFFAAAAAA);
    }
}
