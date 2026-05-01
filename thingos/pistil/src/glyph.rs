//! Placeholder glyph atlas cache.
//!
//! This module sketches the future glyph atlas design and reserves the API
//! surface so call sites in Bloom and Pistil can be written today without
//! waiting for the full implementation.
//!
//! ## Intended design
//!
//! * A single `GlyphAtlas` is owned by `Painter` and evicted when the font
//!   or display DPI changes.
//! * Glyphs are rasterised with `fontdue` at the requested pixel size and
//!   packed into the atlas texture with `Atlas::pack`.
//! * Cache lookups will be O(1) via a hash-map keyed on
//!   `(char, font_id, px_size_bits)`.
//! * On a cache miss the glyph is rasterised, packed, and the UV rect is
//!   stored for subsequent frames.
//! * When the atlas is full, `evict()` clears it entirely and packing
//!   restarts from scratch.

use crate::Atlas;

/// Maximum number of glyphs the atlas will hold before it must be evicted.
///
/// This is a conservative limit for the initial implementation.  A
/// production-quality implementation would use multiple atlas pages or a
/// proper per-glyph eviction policy.
pub const GLYPH_ATLAS_MAX_GLYPHS: usize = 512;

/// Default atlas width in texels.
pub const GLYPH_ATLAS_W: u32 = 512;
/// Default atlas height in texels.
pub const GLYPH_ATLAS_H: u32 = 256;

/// A UV rectangle inside the atlas texture (in texels).
///
/// This is the location of a single cached glyph within the atlas.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GlyphUv {
    /// Left edge of the glyph within the atlas, in texels.
    pub x: u32,
    /// Top edge of the glyph within the atlas, in texels.
    pub y: u32,
    /// Width of the glyph in texels.
    pub w: u32,
    /// Height of the glyph in texels.
    pub h: u32,
}

/// Placeholder glyph atlas.
///
/// Currently this is a stub that holds an `Atlas` allocation and exposes the
/// API that future rendering code will call.  Call sites in Bloom and Pistil
/// should target this API today so that upgrading to the full implementation
/// requires only this file to change.
pub struct GlyphAtlas {
    atlas: Option<Atlas>,
    glyph_count: usize,
}

impl GlyphAtlas {
    /// Create a new (empty) glyph atlas backed by a memory-mapped texture.
    ///
    /// Returns `None` when the underlying kernel VFS / memory-mapping
    /// syscall is unavailable (e.g. host-side unit tests).  Use
    /// `new_stub()` in those environments.
    pub fn new() -> Option<Self> {
        let atlas = Atlas::new("pistil:glyphs", GLYPH_ATLAS_W, GLYPH_ATLAS_H, 1)?;
        Some(Self { atlas: Some(atlas), glyph_count: 0 })
    }

    /// Create an atlas stub that holds no backing texture.
    ///
    /// All pack calls return `None`.  Useful in environments where the
    /// kernel VFS is unavailable (e.g. host-side unit tests).
    pub fn new_stub() -> Self {
        Self { atlas: None, glyph_count: 0 }
    }

    /// Returns `true` if this atlas has a backing texture.
    pub fn is_available(&self) -> bool {
        self.atlas.is_some()
    }

    /// Current number of glyphs packed into the atlas.
    pub fn glyph_count(&self) -> usize {
        self.glyph_count
    }

    /// Evict all cached glyphs and reset the atlas to empty.
    ///
    /// Call this when the font family, weight, or display DPI changes so
    /// that glyph metrics are recomputed at the new size.
    pub fn evict(&mut self) {
        if let Some(ref mut atlas) = self.atlas {
            atlas.texture.as_bytes_mut().fill(0);
            atlas.next_x = 0;
            atlas.next_y = 0;
            atlas.row_h = 0;
        }
        self.glyph_count = 0;
    }

    /// Attempt to pack a pre-rasterised glyph bitmap into the atlas.
    ///
    /// `pixels` must be a `w * h` byte slice of 8-bit coverage values
    /// (0 = fully transparent, 255 = fully covered).
    ///
    /// Returns the `GlyphUv` rectangle on success, or `None` when the atlas
    /// is full.  Callers should call `evict()` and retry after a `None`.
    pub fn pack_glyph(&mut self, w: u32, h: u32, pixels: &[u8]) -> Option<GlyphUv> {
        let atlas = self.atlas.as_mut()?;
        let (x, y) = atlas.pack(w, h, pixels)?;
        self.glyph_count += 1;
        Some(GlyphUv { x, y, w, h })
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_atlas_is_not_available() {
        let atlas = GlyphAtlas::new_stub();
        assert!(!atlas.is_available());
    }

    #[test]
    fn stub_glyph_count_starts_at_zero() {
        let atlas = GlyphAtlas::new_stub();
        assert_eq!(atlas.glyph_count(), 0);
    }

    #[test]
    fn stub_pack_returns_none() {
        let mut atlas = GlyphAtlas::new_stub();
        let result = atlas.pack_glyph(8, 8, &[255u8; 64]);
        assert!(result.is_none());
    }

    #[test]
    fn evict_resets_glyph_count() {
        let mut atlas = GlyphAtlas::new_stub();
        atlas.glyph_count = 7;
        atlas.evict();
        assert_eq!(atlas.glyph_count(), 0);
    }
}
