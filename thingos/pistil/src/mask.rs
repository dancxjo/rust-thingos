//! Cached rounded-corner alpha masks.
//!
//! Each mask covers a *bucketed* window size and a given corner radius.
//! The alpha values are `u8` (0 = fully transparent, 255 = fully opaque).
//! Bloom or the `Painter` can use these masks to clip painted content to
//! rounded rectangles without recomputing the coverage every frame.
//!
//! ## Size buckets
//!
//! To avoid cache thrashing as window sizes change continuously, dimensions
//! are rounded up to the nearest bucket before rendering.  The caller can
//! then scale the mask to the actual window size at blit time.

use alloc::vec;
use alloc::vec::Vec;

const MASK_CACHE_CAPACITY: usize = 12;

/// Round `n` up to the nearest size bucket.
pub fn size_bucket(n: u32) -> u32 {
    const SMALL_BUCKETS: &[u32] = &[16, 24, 32, 48, 64, 96, 128, 192, 256, 384, 512];
    for &b in SMALL_BUCKETS {
        if n <= b {
            return b;
        }
    }
    // Above 512 px: round up to the next 128-px multiple.
    ((n + 127) / 128) * 128
}

/// A cached rounded-rectangle alpha mask.
pub struct MaskEntry {
    /// Bucketed width (may be larger than the requested width).
    pub bw: u32,
    /// Bucketed height (may be larger than the requested height).
    pub bh: u32,
    /// Corner radius that was used to render this mask.
    pub radius: u32,
    /// Alpha values in row-major order, stride = `bw`.
    pub pixels: Vec<u8>,
}

/// Cache of rounded-corner alpha masks.
///
/// Create one instance per compositor session and reuse it every frame.
pub struct MaskCache {
    entries: Vec<MaskEntry>,
}

impl MaskCache {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// Return a reference to a cached mask for the given size and corner radius.
    ///
    /// The mask is rendered at the bucketed size (≥ `width` × `height`).
    /// Use `sample` for easy per-pixel lookup that handles the scaling.
    pub fn get_or_create(&mut self, width: u32, height: u32, radius: u32) -> &MaskEntry {
        let bw = size_bucket(width);
        let bh = size_bucket(height);
        let r = radius.min(bw / 2).min(bh / 2);

        if let Some(pos) =
            self.entries.iter().position(|e| e.bw == bw && e.bh == bh && e.radius == r)
        {
            return &self.entries[pos];
        }

        if self.entries.len() >= MASK_CACHE_CAPACITY {
            self.entries.remove(0);
        }

        let mut pixels = vec![0u8; (bw * bh) as usize];
        render_rounded_mask(&mut pixels, bw, bh, r);
        self.entries.push(MaskEntry { bw, bh, radius: r, pixels });
        self.entries.last().unwrap()
    }

    /// Return the alpha value (0 = outside, 255 = inside) for logical pixel
    /// `(x, y)` within a rounded rect of the given logical size and radius.
    ///
    /// The coordinates are scaled into the bucketed mask automatically.
    pub fn sample(&mut self, width: u32, height: u32, radius: u32, x: u32, y: u32) -> u8 {
        let entry = self.get_or_create(width, height, radius);
        let bw = entry.bw;
        let bh = entry.bh;
        let mx = if width > 0 { (x as u64 * bw as u64 / width as u64) as u32 } else { 0 };
        let my = if height > 0 { (y as u64 * bh as u64 / height as u64) as u32 } else { 0 };
        let mx = mx.min(bw.saturating_sub(1));
        let my = my.min(bh.saturating_sub(1));
        let idx = my as usize * bw as usize + mx as usize;
        if idx < entry.pixels.len() { entry.pixels[idx] } else { 0 }
    }
}

impl Default for MaskCache {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Internal rendering
// ---------------------------------------------------------------------------

/// Render a rounded-rectangle alpha mask.
///
/// Pixels inside the rounded rectangle are set to 255; pixels outside are 0.
fn render_rounded_mask(pixels: &mut [u8], w: u32, h: u32, radius: u32) {
    if w == 0 || h == 0 {
        return;
    }
    let r = radius.min(w / 2).min(h / 2);
    for row in 0..h {
        let inset = rounded_rect_row_inset(r, row, h);
        for col in 0..w {
            let alpha = if col >= inset && col < w.saturating_sub(inset) { 255u8 } else { 0u8 };
            pixels[(row * w + col) as usize] = alpha;
        }
    }
}

fn rounded_rect_row_inset(radius: u32, row: u32, height: u32) -> u32 {
    if radius == 0 || height == 0 {
        return 0;
    }
    let r = radius as i64;
    let dy = if row < radius {
        r - row as i64
    } else if row >= height - radius {
        row as i64 - (height as i64 - r - 1)
    } else {
        return 0;
    };
    let dx_sq = r * r - dy * dy;
    if dx_sq <= 0 {
        return radius;
    }
    let dx = isqrt(dx_sq as u64);
    radius.saturating_sub(dx as u32)
}

fn isqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center_pixel_is_opaque() {
        let mut cache = MaskCache::new();
        let alpha = cache.sample(64, 64, 8, 32, 32);
        assert_eq!(alpha, 255, "center pixel should be opaque");
    }

    #[test]
    fn corner_pixel_is_transparent_with_large_radius() {
        let mut cache = MaskCache::new();
        let alpha = cache.sample(64, 64, 32, 0, 0);
        assert_eq!(alpha, 0, "top-left corner should be transparent with a large radius");
    }

    #[test]
    fn no_radius_gives_fully_opaque_mask() {
        let mut cache = MaskCache::new();
        for x in 0..16u32 {
            for y in 0..16u32 {
                let alpha = cache.sample(16, 16, 0, x, y);
                assert_eq!(alpha, 255, "no radius: pixel ({x},{y}) should be opaque");
            }
        }
    }

    #[test]
    fn size_bucket_quantises_small_values() {
        assert_eq!(size_bucket(1), 16);
        assert_eq!(size_bucket(16), 16);
        assert_eq!(size_bucket(17), 24);
        assert_eq!(size_bucket(32), 32);
        assert_eq!(size_bucket(512), 512);
    }

    #[test]
    fn size_bucket_quantises_large_values() {
        assert_eq!(size_bucket(513), 640);
        assert_eq!(size_bucket(600), 640);
        assert_eq!(size_bucket(640), 640);
        assert_eq!(size_bucket(641), 768);
    }

    #[test]
    fn cache_evicts_when_full() {
        let mut cache = MaskCache::new();
        for r in 0..=MASK_CACHE_CAPACITY as u32 {
            cache.get_or_create(64, 64, r);
        }
        assert!(cache.entries.len() <= MASK_CACHE_CAPACITY);
    }
}
