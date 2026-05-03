//! `wlr-layer-shell` (zwlr_layer_shell_v1 / zwlr_layer_surface_v1) types and
//! pure placement logic.
//!
//! This module is intentionally a small, side-effect-free state container so
//! that layer-shell layout decisions can be unit tested without dragging in
//! the rest of the compositor.  The Bloom Wayland dispatcher tracks per-client
//! layer surfaces and feeds their accumulated state into
//! [`compute_layer_placement`] which produces the output-relative rectangle
//! and `z_order` that the scene layer should apply.
//!
//! # Protocol coverage (v1, minimal)
//!
//! - `zwlr_layer_shell_v1.get_layer_surface` — role assignment
//! - `zwlr_layer_surface_v1.set_size` / `set_anchor` / `set_exclusive_zone`
//!   `set_margin` / `set_layer` — accumulated client state
//! - `zwlr_layer_surface_v1.ack_configure` — lifecycle handshake
//! - Initial configure on first commit (compositor → client)

use crate::ConfigureSerial;

/// Layer assignment from `zwlr_layer_shell_v1.layer`.
///
/// The numeric values match the protocol enum so they can be parsed straight
/// from the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum LayerShellLayer {
    /// Below all other surfaces (e.g. wallpaper).
    Background = 0,
    /// Above background, below normal toplevels (e.g. dock).
    Bottom = 1,
    /// Above normal toplevels (e.g. notification bubbles).
    Top = 2,
    /// Above everything except the cursor (e.g. lock screen).
    Overlay = 3,
}

/// Keyboard focus request from `zwlr_layer_surface_v1.set_keyboard_interactivity`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum LayerKeyboardInteractivity {
    None = 0,
    Exclusive = 1,
    OnDemand = 2,
}

impl LayerKeyboardInteractivity {
    pub fn from_wire(v: u32) -> Option<Self> {
        match v {
            0 => Some(Self::None),
            1 => Some(Self::Exclusive),
            2 => Some(Self::OnDemand),
            _ => None,
        }
    }
}

impl LayerShellLayer {
    /// Decode from a `zwlr_layer_shell_v1.layer` wire value.  Returns `None`
    /// for out-of-range integers (the dispatcher should send a protocol
    /// error in that case).
    pub fn from_wire(v: u32) -> Option<Self> {
        match v {
            0 => Some(Self::Background),
            1 => Some(Self::Bottom),
            2 => Some(Self::Top),
            3 => Some(Self::Overlay),
            _ => None,
        }
    }

    /// The compositor-assigned `z_order` base for this layer.
    ///
    /// Layers are kept far apart in the integer line so that any per-layer
    /// stacking offsets (`get_popup`, sub-stacking inside a layer) cannot
    /// collide with neighbouring layers.  The values straddle the typical
    /// xdg_toplevel z range that starts at small positive integers (see
    /// [`crate::compute_layer_placement`]).
    pub const fn z_order(self) -> i32 {
        match self {
            Self::Background => -2_000_000,
            Self::Bottom => -1_000_000,
            Self::Top => 1_000_000,
            Self::Overlay => 2_000_000,
        }
    }
}

/// Anchor edge bitflags from `zwlr_layer_surface_v1.anchor`.  Numeric values
/// match the protocol enum.
pub mod anchor {
    pub const TOP: u32 = 1;
    pub const BOTTOM: u32 = 2;
    pub const LEFT: u32 = 4;
    pub const RIGHT: u32 = 8;
}

/// `zwlr_layer_surface_v1.error` codes.
pub mod layer_surface_error {
    pub const INVALID_SURFACE_STATE: u32 = 0;
    pub const INVALID_SIZE: u32 = 1;
    pub const INVALID_ANCHOR: u32 = 2;
    pub const INVALID_KEYBOARD_INTERACTIVITY: u32 = 3;
    pub const INVALID_EXCLUSIVE_ZONE: u32 = 4;
}

/// Accumulated client-supplied state for a single `zwlr_layer_surface_v1`.
///
/// All fields default to the protocol's documented defaults: anchor 0
/// (centered), exclusive_zone 0 (no exclusion), all margins 0, size (0, 0)
/// (let the compositor pick the size on the anchored axis).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayerSurfaceConfig {
    pub layer: LayerShellLayer,
    pub anchor: u32,
    pub size: (u32, u32),
    pub exclusive_zone: i32,
    pub keyboard_interactivity: LayerKeyboardInteractivity,
    pub margin_top: i32,
    pub margin_right: i32,
    pub margin_bottom: i32,
    pub margin_left: i32,
}

impl LayerSurfaceConfig {
    /// Construct a config with protocol defaults for a given layer.
    pub const fn new(layer: LayerShellLayer) -> Self {
        Self {
            layer,
            anchor: 0,
            size: (0, 0),
            exclusive_zone: 0,
            keyboard_interactivity: LayerKeyboardInteractivity::None,
            margin_top: 0,
            margin_right: 0,
            margin_bottom: 0,
            margin_left: 0,
        }
    }
}

/// Per-`zwlr_layer_surface_v1` lifecycle state managed by the dispatcher.
#[derive(Debug, Clone)]
pub struct LayerSurfaceState {
    pub config: LayerSurfaceConfig,
    /// `true` after the client performed the initial commit (with no buffer)
    /// that must trigger the compositor's first `configure` event.
    pub initial_commit_done: bool,
    /// `true` once at least one `ack_configure` has been received.
    pub configured: bool,
    /// `true` after the first valid post-configure buffer commit.
    pub mapped: bool,
    /// Outstanding configure serials awaiting `ack_configure`.
    pub pending_configures: alloc::collections::VecDeque<ConfigureSerial>,
}

impl LayerSurfaceState {
    pub fn new(layer: LayerShellLayer) -> Self {
        Self {
            config: LayerSurfaceConfig::new(layer),
            initial_commit_done: false,
            configured: false,
            mapped: false,
            pending_configures: alloc::collections::VecDeque::new(),
        }
    }
}

/// Result of laying out a layer surface against an output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayerSurfacePlacement {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub z_order: i32,
    /// The size the compositor wants the client to commit to in the next
    /// `configure` event.  May differ from the client-requested size when
    /// anchors stretch along an axis (size 0 on that axis).
    pub configure_width: u32,
    pub configure_height: u32,
}

/// Compute the output-relative placement of a layer surface.
///
/// Behaviour follows the `zwlr_layer_surface_v1` spec for a single output:
///
/// - When the size on an axis is `0` and the surface is anchored to both
///   edges of that axis, it stretches across the output (minus margins).
/// - When the size on an axis is `0` but the surface is anchored to only
///   one (or neither) edge, the compositor cannot pick a size, so it asks
///   the client for a 1×1 surface (the spec says "any non-zero" — we use 1
///   so the client knows it must call `set_size`).
/// - When the size is non-zero on an axis, that size is honoured.
/// - Anchors then position the rectangle on the output, applying margins.
///
/// `output_w` / `output_h` are the usable dimensions of the output.  For
/// the minimal v1 implementation Bloom does not subtract exclusive zones of
/// other layer surfaces.
pub fn compute_layer_placement(
    output_w: u32,
    output_h: u32,
    cfg: &LayerSurfaceConfig,
) -> LayerSurfacePlacement {
    let z_order = cfg.layer.z_order();

    let anchored_left = cfg.anchor & anchor::LEFT != 0;
    let anchored_right = cfg.anchor & anchor::RIGHT != 0;
    let anchored_top = cfg.anchor & anchor::TOP != 0;
    let anchored_bottom = cfg.anchor & anchor::BOTTOM != 0;

    // ── Width ──
    let (width, configure_width) = if cfg.size.0 != 0 {
        (cfg.size.0, cfg.size.0)
    } else if anchored_left && anchored_right {
        let avail = (output_w as i64)
            .saturating_sub(cfg.margin_left as i64)
            .saturating_sub(cfg.margin_right as i64)
            .max(0) as u32;
        (avail, avail)
    } else {
        // Spec: with no width and no horizontal stretch anchor the
        // compositor cannot pick a size — it must ask the client.  We send
        // `configure(0, …)` (per protocol "0 means client-chosen") while
        // using a 1-pixel placeholder for the runtime width so the
        // surface stays well-defined until the client calls `set_size`.
        (1, 0)
    };

    // ── Height ──
    let (height, configure_height) = if cfg.size.1 != 0 {
        (cfg.size.1, cfg.size.1)
    } else if anchored_top && anchored_bottom {
        let avail = (output_h as i64)
            .saturating_sub(cfg.margin_top as i64)
            .saturating_sub(cfg.margin_bottom as i64)
            .max(0) as u32;
        (avail, avail)
    } else {
        (1, 0)
    };

    // ── X position ──
    let x = if anchored_left && !anchored_right {
        cfg.margin_left
    } else if anchored_right && !anchored_left {
        (output_w as i32) - (width as i32) - cfg.margin_right
    } else {
        // Centred (no horizontal anchor or both): margins shift the centre.
        let center = (output_w as i32) / 2;
        let left_origin = center - (width as i32) / 2;
        left_origin + cfg.margin_left - cfg.margin_right
    };

    // ── Y position ──
    let y = if anchored_top && !anchored_bottom {
        cfg.margin_top
    } else if anchored_bottom && !anchored_top {
        (output_h as i32) - (height as i32) - cfg.margin_bottom
    } else {
        let center = (output_h as i32) / 2;
        let top_origin = center - (height as i32) / 2;
        top_origin + cfg.margin_top - cfg.margin_bottom
    };

    LayerSurfacePlacement { x, y, width, height, z_order, configure_width, configure_height }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn out() -> (u32, u32) {
        (800, 600)
    }

    #[test]
    fn z_orders_strictly_separate_layers() {
        let bg = LayerShellLayer::Background.z_order();
        let bot = LayerShellLayer::Bottom.z_order();
        let top = LayerShellLayer::Top.z_order();
        let ovl = LayerShellLayer::Overlay.z_order();
        assert!(bg < bot, "background must stack below bottom");
        assert!(bot < 0, "bottom must stack below normal toplevels");
        assert!(top > 0, "top must stack above normal toplevels");
        assert!(ovl > top, "overlay must stack above top");
    }

    #[test]
    fn from_wire_decodes_known_values_only() {
        assert_eq!(LayerShellLayer::from_wire(0), Some(LayerShellLayer::Background));
        assert_eq!(LayerShellLayer::from_wire(3), Some(LayerShellLayer::Overlay));
        assert_eq!(LayerShellLayer::from_wire(4), None);
    }

    #[test]
    fn full_screen_background_stretches_across_output() {
        let (w, h) = out();
        let mut cfg = LayerSurfaceConfig::new(LayerShellLayer::Background);
        cfg.anchor = anchor::TOP | anchor::BOTTOM | anchor::LEFT | anchor::RIGHT;
        let p = compute_layer_placement(w, h, &cfg);
        assert_eq!((p.x, p.y, p.width, p.height), (0, 0, 800, 600));
        assert_eq!((p.configure_width, p.configure_height), (800, 600));
        assert_eq!(p.z_order, LayerShellLayer::Background.z_order());
    }

    #[test]
    fn top_panel_stretched_horizontally() {
        let (w, h) = out();
        let mut cfg = LayerSurfaceConfig::new(LayerShellLayer::Top);
        cfg.anchor = anchor::TOP | anchor::LEFT | anchor::RIGHT;
        cfg.size = (0, 30); // height fixed, width stretched
        let p = compute_layer_placement(w, h, &cfg);
        assert_eq!((p.x, p.y, p.width, p.height), (0, 0, 800, 30));
        // configure must propagate the stretched width back to the client
        assert_eq!((p.configure_width, p.configure_height), (800, 30));
    }

    #[test]
    fn bottom_dock_anchored_with_margin() {
        let (w, h) = out();
        let mut cfg = LayerSurfaceConfig::new(LayerShellLayer::Bottom);
        cfg.anchor = anchor::BOTTOM;
        cfg.size = (200, 40);
        cfg.margin_bottom = 10;
        let p = compute_layer_placement(w, h, &cfg);
        // Centred horizontally (no L/R anchor), y = output_h - h - margin
        assert_eq!(p.y, 600 - 40 - 10);
        assert_eq!(p.height, 40);
        assert_eq!(p.width, 200);
        // Centred horizontally
        assert_eq!(p.x, (800 / 2) - (200 / 2));
    }

    #[test]
    fn left_panel_anchored_with_top_bottom_stretch() {
        let (w, h) = out();
        let mut cfg = LayerSurfaceConfig::new(LayerShellLayer::Top);
        cfg.anchor = anchor::LEFT | anchor::TOP | anchor::BOTTOM;
        cfg.size = (50, 0);
        let p = compute_layer_placement(w, h, &cfg);
        assert_eq!((p.x, p.y, p.width, p.height), (0, 0, 50, 600));
    }

    #[test]
    fn margin_applied_to_anchored_edges() {
        let (w, h) = out();
        let mut cfg = LayerSurfaceConfig::new(LayerShellLayer::Top);
        cfg.anchor = anchor::TOP | anchor::RIGHT;
        cfg.size = (100, 50);
        cfg.margin_top = 8;
        cfg.margin_right = 12;
        let p = compute_layer_placement(w, h, &cfg);
        assert_eq!(p.y, 8);
        assert_eq!(p.x, 800 - 100 - 12);
    }

    #[test]
    fn unspecified_size_without_stretch_anchor_falls_back_to_one() {
        let (w, h) = out();
        let cfg = LayerSurfaceConfig::new(LayerShellLayer::Top);
        let p = compute_layer_placement(w, h, &cfg);
        assert_eq!((p.width, p.height), (1, 1));
        // configure must be 0 so the client knows it has to set_size.
        assert_eq!((p.configure_width, p.configure_height), (0, 0));
    }

    #[test]
    fn stretch_minus_margins_does_not_underflow() {
        let mut cfg = LayerSurfaceConfig::new(LayerShellLayer::Background);
        cfg.anchor = anchor::LEFT | anchor::RIGHT | anchor::TOP | anchor::BOTTOM;
        cfg.margin_left = 1000;
        cfg.margin_right = 1000;
        cfg.margin_top = 1000;
        cfg.margin_bottom = 1000;
        let p = compute_layer_placement(100, 100, &cfg);
        assert_eq!((p.width, p.height), (0, 0));
    }
}
