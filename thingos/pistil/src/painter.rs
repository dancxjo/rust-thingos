//! High-level visual primitive API for Bloom chrome and other UI code.
//!
//! [`Painter`] is the single entry point for cached visual operations.
//! It owns the gradient cache, the rounded-mask cache, and a glyph atlas
//! placeholder.  Bloom describes *intent* by calling named methods;
//! Pistil handles the visual details and ensures expensive raster work is
//! performed at most once per unique set of parameters.
//!
//! ## Usage pattern
//!
//! ```rust,ignore
//! let mut painter = Painter::new();
//! // … per-frame:
//! painter.draw_window_shadow(&mut buf, stride, height, x, y, w, h, active);
//! painter.draw_window_frame(&mut buf, stride, height, x, y, w, h, active);
//! painter.draw_title_bar(&mut buf, stride, height, x, y, w, tb_h, active);
//! painter.draw_panel(&mut buf, stride, height, x, y, w, h, PanelStyle::Solid);
//! ```
//!
//! ## Built-in fallback colours
//!
//! `Painter::new()` uses neutral Obsidian Bloom fallback colours for callers
//! that draw primitives directly.  Compositor chrome should use `stile` theme
//! paint lists and treat Pistil as the executor.

use alloc::vec::Vec;

use crate::gradient::GradientCache;
use crate::glyph::GlyphAtlas;
use crate::mask::MaskCache;
use crate::shadow::draw_nine_slice_shadow;

// ---------------------------------------------------------------------------
// Built-in primitive fallback constants
// ---------------------------------------------------------------------------

const DEFAULT_TITLE_TOP_ACTIVE: u32 = 0xFF1A1424;
const DEFAULT_TITLE_TOP_INACTIVE: u32 = 0xFF14101C;
const DEFAULT_FRAME_FILL: u32 = 0xFF1A1424;
const DEFAULT_FRAME_FILL_INACTIVE: u32 = 0xFF14101C;
const DEFAULT_BODY_TOP: u32 = 0xFF0B0A10;
const DEFAULT_OUTER_STROKE: u32 = 0xFF0F0C18;
/// Default title-bar height in pixels.
pub const DEFAULT_TITLEBAR_HEIGHT: u32 = 28;
/// Default frame (border) thickness in pixels.
pub const DEFAULT_FRAME_THICKNESS: u32 = 4;

// ---------------------------------------------------------------------------
// PanelStyle
// ---------------------------------------------------------------------------

/// Selects the visual style for a [`Painter::draw_panel`] call.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PanelStyle {
    /// Flat solid fill with the theme body colour.
    Solid,
    /// Vertical linear gradient between the two provided ARGB colours.
    Gradient {
        /// Colour at the top edge (ARGB u32).
        color_top: u32,
        /// Colour at the bottom edge (ARGB u32).
        color_bottom: u32,
    },
}

// ---------------------------------------------------------------------------
// Painter
// ---------------------------------------------------------------------------

/// High-level cached visual painter for Pistil.
///
/// Holds the [`GradientCache`], [`MaskCache`], and [`GlyphAtlas`] so they
/// accumulate across frames.  Create one instance per compositor session and
/// reuse it for every frame.
pub struct Painter {
    /// Gradient strip cache (public for advanced callers).
    pub gradient_cache: GradientCache,
    /// Rounded-corner mask cache (public for advanced callers).
    pub mask_cache: MaskCache,
    /// Glyph atlas placeholder (public for future text-rendering integration).
    pub glyph_atlas: GlyphAtlas,

    // Active / inactive title-bar gradient colours.
    title_top_active: u32,
    title_top_inactive: u32,
    // Frame (border) fill colours.
    frame_fill: u32,
    frame_fill_inactive: u32,
    // Content area background colour.
    body_top: u32,
    // Outer border stroke colour.
    outer_stroke: u32,
    /// Title-bar height in pixels (read-only access via accessor).
    pub titlebar_height: u32,
    /// Frame (border) thickness in pixels (read-only access via accessor).
    pub frame_thickness: u32,
}

impl Painter {
    /// Create a [`Painter`] with the built-in primitive fallback values.
    pub fn new() -> Self {
        Self {
            gradient_cache: GradientCache::new(),
            mask_cache: MaskCache::new(),
            glyph_atlas: GlyphAtlas::new_stub(),
            title_top_active: DEFAULT_TITLE_TOP_ACTIVE,
            title_top_inactive: DEFAULT_TITLE_TOP_INACTIVE,
            frame_fill: DEFAULT_FRAME_FILL,
            frame_fill_inactive: DEFAULT_FRAME_FILL_INACTIVE,
            body_top: DEFAULT_BODY_TOP,
            outer_stroke: DEFAULT_OUTER_STROKE,
            titlebar_height: DEFAULT_TITLEBAR_HEIGHT,
            frame_thickness: DEFAULT_FRAME_THICKNESS,
        }
    }

    // -----------------------------------------------------------------------
    // Visual primitives
    // -----------------------------------------------------------------------

    /// Draw the nine-slice drop shadow behind a window.
    ///
    /// `dst` is the full-screen shadow overlay buffer (ARGB u32 pixels).
    /// `win_x/y/w/h` is the window rect in screen coordinates.
    /// `active` selects the focused (brighter) or unfocused (dimmer) shadow.
    pub fn draw_window_shadow(
        &mut self,
        dst: &mut [u32],
        dst_stride: u32,
        dst_h: u32,
        win_x: i32,
        win_y: i32,
        win_w: u32,
        win_h: u32,
        active: bool,
    ) {
        draw_nine_slice_shadow(dst, dst_stride, dst_h, win_x, win_y, win_w, win_h, active);
    }

    /// Draw the full window frame (border + title bar area).
    ///
    /// Paints the decorative chrome border around a window rectangle.
    /// Pass `active = true` for the focused window, `false` otherwise.
    pub fn draw_window_frame(
        &mut self,
        dst: &mut [u32],
        dst_stride: u32,
        dst_h: u32,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        active: bool,
    ) {
        if w == 0 || h == 0 {
            return;
        }
        let frame = self.frame_thickness.min(w / 2).min(h / 2);
        let tb_h = self.titlebar_height.min(h);
        let fill = if active { self.frame_fill } else { self.frame_fill_inactive };

        if tb_h > 0 {
            self.draw_title_bar(dst, dst_stride, dst_h, x, y, w, tb_h, active);
        } else {
            fill_rect(dst, dst_stride, x, y, w, frame, fill);
        }

        // Left, right, and bottom frame strips.
        fill_rect(dst, dst_stride, x, y, frame, h, fill);
        fill_rect(dst, dst_stride, x + w.saturating_sub(frame) as i32, y, frame, h, fill);
        fill_rect(dst, dst_stride, x, y + h.saturating_sub(frame) as i32, w, frame, fill);

        // Outer stroke.
        draw_rect_stroke(dst, dst_stride, dst_h, x, y, w, h, 2, self.outer_stroke);
    }

    /// Draw a title bar strip, using the gradient cache.
    ///
    /// Renders a vertical gradient title bar at `(x, y)` with size `(w, h)`.
    /// The gradient is cached, so the computation only happens once per
    /// unique `(w, h, active)` combination.
    pub fn draw_title_bar(
        &mut self,
        dst: &mut [u32],
        dst_stride: u32,
        dst_h: u32,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        active: bool,
    ) {
        if w == 0 || h == 0 {
            return;
        }
        let (color_top, color_bottom) = if active {
            (self.title_top_active, darken(self.title_top_active, 20))
        } else {
            (self.title_top_inactive, darken(self.title_top_inactive, 10))
        };
        self.gradient_cache.draw(dst, dst_stride, dst_h, x, y, w, h, color_top, color_bottom);
    }

    /// Draw a panel (content area background).
    ///
    /// `style` controls whether the panel is a flat solid fill or a gradient.
    pub fn draw_panel(
        &mut self,
        dst: &mut [u32],
        dst_stride: u32,
        dst_h: u32,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        style: PanelStyle,
    ) {
        if w == 0 || h == 0 {
            return;
        }
        match style {
            PanelStyle::Solid => {
                fill_rect(dst, dst_stride, x, y, w, h, self.body_top);
            }
            PanelStyle::Gradient { color_top, color_bottom } => {
                self.gradient_cache.draw(
                    dst, dst_stride, dst_h, x, y, w, h, color_top, color_bottom,
                );
            }
        }
    }
}

impl Default for Painter {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Internal drawing helpers
// ---------------------------------------------------------------------------

fn fill_rect(dst: &mut [u32], stride: u32, x: i32, y: i32, w: u32, h: u32, color: u32) {
    for row in 0..h {
        let out_y = y + row as i32;
        if out_y < 0 {
            continue;
        }
        let row_off = (out_y as u32 * stride) as usize;
        for col in 0..w {
            let out_x = x + col as i32;
            if out_x < 0 || out_x >= stride as i32 {
                continue;
            }
            let idx = row_off + out_x as usize;
            if idx < dst.len() {
                dst[idx] = color;
            }
        }
    }
}

fn draw_rect_stroke(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    thickness: u32,
    color: u32,
) {
    if w == 0 || h == 0 || thickness == 0 {
        return;
    }
    let t = thickness.min(w / 2).min(h / 2);
    fill_rect_clamped(dst, stride, height, x, y, w as i32, t as i32, color);
    fill_rect_clamped(dst, stride, height, x, y + h.saturating_sub(t) as i32, w as i32, t as i32, color);
    fill_rect_clamped(dst, stride, height, x, y, t as i32, h as i32, color);
    fill_rect_clamped(dst, stride, height, x + w.saturating_sub(t) as i32, y, t as i32, h as i32, color);
}

fn fill_rect_clamped(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: u32,
) {
    if w <= 0 || h <= 0 {
        return;
    }
    for row in 0..h {
        let out_y = y + row;
        if out_y < 0 || out_y >= height as i32 {
            continue;
        }
        let row_off = (out_y as u32 * stride) as usize;
        for col in 0..w {
            let out_x = x + col;
            if out_x < 0 || out_x >= stride as i32 {
                continue;
            }
            let idx = row_off + out_x as usize;
            if idx < dst.len() {
                dst[idx] = color;
            }
        }
    }
}

/// Darken an ARGB colour by subtracting `amount` from each RGB channel.
fn darken(color: u32, amount: u32) -> u32 {
    let a = (color >> 24) & 0xFF;
    let r = ((color >> 16) & 0xFF).saturating_sub(amount);
    let g = ((color >> 8) & 0xFF).saturating_sub(amount);
    let b = (color & 0xFF).saturating_sub(amount);
    (a << 24) | (r << 16) | (g << 8) | b
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    extern crate alloc;
    use alloc::vec;

    use super::*;

    #[test]
    fn painter_builtin_fallback_constants() {
        let p = Painter::new();
        assert_eq!(p.titlebar_height, DEFAULT_TITLEBAR_HEIGHT);
        assert_eq!(p.frame_thickness, DEFAULT_FRAME_THICKNESS);
    }

    #[test]
    fn draw_window_frame_zero_size_is_noop() {
        let mut p = Painter::new();
        let mut buf = vec![0u32; 100];
        p.draw_window_frame(&mut buf, 10, 10, 0, 0, 0, 0, true);
        assert!(buf.iter().all(|&px| px == 0));
    }

    #[test]
    fn draw_panel_solid_fills_interior() {
        let mut p = Painter::new();
        let mut buf = vec![0u32; 10 * 10];
        p.draw_panel(&mut buf, 10, 10, 2, 2, 4, 4, PanelStyle::Solid);
        // The pixel at (3, 3) is inside the panel.
        assert_eq!(buf[3 * 10 + 3], DEFAULT_BODY_TOP);
        // The corner pixel (0, 0) is outside the panel.
        assert_eq!(buf[0], 0);
    }

    #[test]
    fn draw_panel_gradient_top_row_colour() {
        let mut p = Painter::new();
        let mut buf = vec![0u32; 16 * 8];
        p.draw_panel(
            &mut buf,
            16,
            8,
            0,
            0,
            16,
            8,
            PanelStyle::Gradient { color_top: 0xFFFF0000, color_bottom: 0xFF0000FF },
        );
        // Top-left pixel should be the top colour.
        assert_eq!(buf[0], 0xFFFF0000);
    }

    #[test]
    fn draw_title_bar_uses_gradient_cache() {
        let mut p = Painter::new();
        let mut buf = vec![0u32; 200 * 28];
        p.draw_title_bar(&mut buf, 200, 28, 0, 0, 200, 28, true);
        // After the call the gradient cache should have at least one entry.
        assert!(!p.gradient_cache.is_empty());
    }
}
