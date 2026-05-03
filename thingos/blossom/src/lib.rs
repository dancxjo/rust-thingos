//! `blossom` — xdg-shell protocol layer for the bloom Wayland compositor.
//!
//! `blossom` implements the xdg-shell state machine as a pure library with no
//! OS dependencies.  It tracks per-surface and per-toplevel protocol state,
//! enforces lifecycle rules (configure/ack, role uniqueness, commit ordering),
//! and emits [`BlossomCommand`]s that the compositor executes.
//!
//! # Protocol coverage (v1)
//!
//! - `xdg_wm_base`: `get_xdg_surface`, `create_positioner`, `pong`, `destroy`
//! - `xdg_surface`: `get_toplevel`, `get_popup`, `set_window_geometry`,
//!   `ack_configure`, `destroy`
//! - `xdg_toplevel`: all `set_*` / `unset_*` / `show_window_menu` / `move` /
//!   `resize` requests; most are recorded as client intent or accepted as
//!   no-ops in v1
//! - `xdg_positioner`: `set_size`, `set_anchor_rect`, `set_anchor`,
//!   `set_gravity`, `set_constraint_adjustment`, `set_offset`, `destroy`
//!
//! # Initial configure sequence
//!
//! When `get_toplevel` is called, `blossom` immediately emits:
//! 1. [`BlossomCommand::SetToplevelChrome`] — declares v1 title-bar hit geometry
//! 2. [`BlossomCommand::SendXdgToplevelConfigure`] — `(width=0, height=0, states=[])`
//! 3. [`BlossomCommand::SendXdgSurfaceConfigure`] — carries the pending serial
//!
//! After the client calls `ack_configure(serial)` and then commits a buffer,
//! blossom emits [`BlossomCommand::MarkSurfaceReadyForMapping`].
//!
//! # Popup placement
//!
//! Popup positions are computed from the positioner via [`compute_popup_placement`].
//! Gravity and anchor determine the popup's position relative to the anchor
//! rectangle on the parent surface.  Constraint-adjustment flags (slide, flip,
//! resize) keep the popup within the compositor's output bounds.

#![no_std]

extern crate alloc;

use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

pub mod launcher;
pub mod layer_shell;
pub mod runbox;
pub mod wm;

pub use layer_shell::{
    LayerKeyboardInteractivity, LayerShellLayer, LayerSurfaceConfig, LayerSurfacePlacement,
    LayerSurfaceState, anchor as layer_anchor, compute_layer_placement, layer_surface_error,
};

// ── Public type aliases ───────────────────────────────────────────────────────

/// Per-client Wayland object identifier (within a single client's namespace).
pub type ObjectId = u32;
/// Identifies a `wl_surface` in the compositor's surface table.
pub type SurfaceId = u32;
/// Identifies a mapped `xdg_toplevel`.
pub type ToplevelId = ObjectId;
/// Identifies a mapped `xdg_popup`.
pub type PopupId = ObjectId;
/// Identifies a Wayland client connection.
pub type ClientId = u32;
/// A monotonically increasing serial used to pair configure/ack_configure.
pub type ConfigureSerial = u32;

/// Initial compositor-known title bar height for v1 toplevel chrome.
pub const DEFAULT_TITLEBAR_HEIGHT: u32 = 28;
/// Initial compositor-known frame thickness for v1 toplevel chrome.
pub const DEFAULT_FRAME_THICKNESS: u32 = 6;

// ── xdg_positioner constants ─────────────────────────────────────────────────

/// `xdg_positioner` anchor / gravity enum values (same set for both fields).
///
/// These match the wire encoding from `xdg-shell.xml`.
pub mod positioner_anchor {
    pub const NONE: u32 = 0;
    pub const TOP: u32 = 1;
    pub const BOTTOM: u32 = 2;
    pub const LEFT: u32 = 3;
    pub const RIGHT: u32 = 4;
    pub const TOP_LEFT: u32 = 5;
    pub const BOTTOM_LEFT: u32 = 6;
    pub const TOP_RIGHT: u32 = 7;
    pub const BOTTOM_RIGHT: u32 = 8;
}

/// `xdg_positioner` constraint-adjustment bitmask values.
pub mod positioner_constraint {
    pub const NONE: u32 = 0;
    /// Slide the popup along the X axis until it fits.
    pub const SLIDE_X: u32 = 1;
    /// Slide the popup along the Y axis until it fits.
    pub const SLIDE_Y: u32 = 2;
    /// Flip the anchor and gravity horizontally if it produces a better fit.
    pub const FLIP_X: u32 = 4;
    /// Flip the anchor and gravity vertically if it produces a better fit.
    pub const FLIP_Y: u32 = 8;
    /// Shrink the popup width to fit the available space.
    pub const RESIZE_X: u32 = 16;
    /// Shrink the popup height to fit the available space.
    pub const RESIZE_Y: u32 = 32;
}

// ── xdg_positioner state ─────────────────────────────────────────────────────

/// Accumulated client-supplied state for a single `xdg_positioner` object.
///
/// Positions a popup surface relative to the parent's anchor rectangle,
/// taking anchor/gravity/offset into account and applying constraint
/// adjustments to keep the popup within the compositor's output bounds.
#[derive(Debug, Clone, Default)]
pub struct PositionerState {
    /// Desired popup size `(width, height)` in surface-local coordinates.
    /// `None` means the client has not yet called `set_size`.
    pub size: Option<(i32, i32)>,
    /// Anchor rectangle `(x, y, width, height)` relative to the parent's
    /// window geometry.  Defaults to `(0, 0, 0, 0)`.
    pub anchor_rect: (i32, i32, i32, i32),
    /// Which edge or corner of the anchor rect the popup anchors to.
    /// One of the `positioner_anchor::*` constants.  Defaults to `NONE`.
    pub anchor: u32,
    /// Which direction the popup extends from the anchor point.
    /// One of the `positioner_anchor::*` constants.  Defaults to `NONE`.
    pub gravity: u32,
    /// Bitmask of `positioner_constraint::*` flags.  Defaults to `NONE`.
    pub constraint_adjustment: u32,
    /// Additional offset `(x, y)` applied after anchor/gravity placement.
    pub offset: (i32, i32),
}

// ── Popup placement calculation ───────────────────────────────────────────────

/// Return the anchor point on `anchor_rect` for the given `anchor` value.
fn positioner_anchor_point(rect: (i32, i32, i32, i32), anchor: u32) -> (i32, i32) {
    let (rx, ry, rw, rh) = rect;
    match anchor {
        positioner_anchor::TOP => (rx + rw / 2, ry),
        positioner_anchor::BOTTOM => (rx + rw / 2, ry + rh),
        positioner_anchor::LEFT => (rx, ry + rh / 2),
        positioner_anchor::RIGHT => (rx + rw, ry + rh / 2),
        positioner_anchor::TOP_LEFT => (rx, ry),
        positioner_anchor::BOTTOM_LEFT => (rx, ry + rh),
        positioner_anchor::TOP_RIGHT => (rx + rw, ry),
        positioner_anchor::BOTTOM_RIGHT => (rx + rw, ry + rh),
        _ => (rx + rw / 2, ry + rh / 2), // NONE → center
    }
}

/// Return the popup top-left corner given an anchor point and gravity.
///
/// `gravity` controls which side of the anchor point the popup "sticks out"
/// toward, i.e. which face of the popup touches the anchor point.
fn positioner_apply_gravity(ax: i32, ay: i32, pw: i32, ph: i32, gravity: u32) -> (i32, i32) {
    let x = match gravity {
        // RIGHT, TOP_RIGHT, BOTTOM_RIGHT → popup left edge at anchor
        positioner_anchor::RIGHT
        | positioner_anchor::TOP_RIGHT
        | positioner_anchor::BOTTOM_RIGHT => ax,
        // LEFT, TOP_LEFT, BOTTOM_LEFT → popup right edge at anchor
        positioner_anchor::LEFT | positioner_anchor::TOP_LEFT | positioner_anchor::BOTTOM_LEFT => {
            ax - pw
        }
        // NONE / TOP / BOTTOM → horizontally centered
        _ => ax - pw / 2,
    };
    let y = match gravity {
        // BOTTOM, BOTTOM_LEFT, BOTTOM_RIGHT → popup top edge at anchor
        positioner_anchor::BOTTOM
        | positioner_anchor::BOTTOM_LEFT
        | positioner_anchor::BOTTOM_RIGHT => ay,
        // TOP, TOP_LEFT, TOP_RIGHT → popup bottom edge at anchor
        positioner_anchor::TOP | positioner_anchor::TOP_LEFT | positioner_anchor::TOP_RIGHT => {
            ay - ph
        }
        // NONE / LEFT / RIGHT → vertically centered
        _ => ay - ph / 2,
    };
    (x, y)
}

/// Flip the horizontal component of an anchor/gravity value.
fn flip_x(v: u32) -> u32 {
    match v {
        positioner_anchor::LEFT => positioner_anchor::RIGHT,
        positioner_anchor::RIGHT => positioner_anchor::LEFT,
        positioner_anchor::TOP_LEFT => positioner_anchor::TOP_RIGHT,
        positioner_anchor::TOP_RIGHT => positioner_anchor::TOP_LEFT,
        positioner_anchor::BOTTOM_LEFT => positioner_anchor::BOTTOM_RIGHT,
        positioner_anchor::BOTTOM_RIGHT => positioner_anchor::BOTTOM_LEFT,
        _ => v,
    }
}

/// Flip the vertical component of an anchor/gravity value.
fn flip_y(v: u32) -> u32 {
    match v {
        positioner_anchor::TOP => positioner_anchor::BOTTOM,
        positioner_anchor::BOTTOM => positioner_anchor::TOP,
        positioner_anchor::TOP_LEFT => positioner_anchor::BOTTOM_LEFT,
        positioner_anchor::BOTTOM_LEFT => positioner_anchor::TOP_LEFT,
        positioner_anchor::TOP_RIGHT => positioner_anchor::BOTTOM_RIGHT,
        positioner_anchor::BOTTOM_RIGHT => positioner_anchor::TOP_RIGHT,
        _ => v,
    }
}

/// Compute the raw popup position `(x, y)` from positioner fields with no
/// constraint adjustment applied.
fn positioner_raw_position(
    anchor_rect: (i32, i32, i32, i32),
    anchor: u32,
    gravity: u32,
    pw: i32,
    ph: i32,
    off_x: i32,
    off_y: i32,
) -> (i32, i32) {
    let (ax, ay) = positioner_anchor_point(anchor_rect, anchor);
    let (x, y) = positioner_apply_gravity(ax, ay, pw, ph, gravity);
    (x + off_x, y + off_y)
}

/// Compute the final popup placement `(x, y, width, height)` from a
/// `PositionerState`, constraining to the given output bounds.
///
/// `output_w` and `output_h` are the compositor output dimensions used for
/// constraint adjustment.  When the output size is unknown, pass large values
/// (e.g. `i32::MAX`) to disable effective constraint adjustment.
///
/// The returned coordinates are in the same coordinate space as the positioner's
/// anchor_rect (i.e. relative to the parent surface's window geometry).
pub fn compute_popup_placement(
    positioner: &PositionerState,
    output_w: i32,
    output_h: i32,
) -> (i32, i32, i32, i32) {
    let (pw, ph) = positioner.size.unwrap_or((1, 1));
    let pw = pw.max(1);
    let ph = ph.max(1);
    let (off_x, off_y) = positioner.offset;
    let adj = positioner.constraint_adjustment;

    let mut anchor = positioner.anchor;
    let mut gravity = positioner.gravity;

    let (mut x, mut y) =
        positioner_raw_position(positioner.anchor_rect, anchor, gravity, pw, ph, off_x, off_y);
    let mut w = pw;
    let mut h = ph;

    // ── FLIP_X ───────────────────────────────────────────────────────────
    // If the popup overflows on the X axis, try flipping anchor+gravity
    // horizontally.  Accept the flip only when it produces a better fit.
    if adj & positioner_constraint::FLIP_X != 0 && (x < 0 || x + w > output_w) {
        let fa = flip_x(anchor);
        let fg = flip_x(gravity);
        let (nx, _) = positioner_raw_position(positioner.anchor_rect, fa, fg, pw, ph, off_x, off_y);
        // Accept the flip if the new position overflows less than the original.
        let orig_overflow = (-x).max(0).max((x + w - output_w).max(0));
        let new_overflow = (-nx).max(0).max((nx + w - output_w).max(0));
        if new_overflow < orig_overflow {
            x = nx;
            anchor = fa;
            gravity = fg;
        }
    }

    // ── FLIP_Y ───────────────────────────────────────────────────────────
    if adj & positioner_constraint::FLIP_Y != 0 && (y < 0 || y + h > output_h) {
        let fa = flip_y(anchor);
        let fg = flip_y(gravity);
        let (_, ny) = positioner_raw_position(positioner.anchor_rect, fa, fg, pw, ph, off_x, off_y);
        let orig_overflow = (-y).max(0).max((y + h - output_h).max(0));
        let new_overflow = (-ny).max(0).max((ny + h - output_h).max(0));
        if new_overflow < orig_overflow {
            y = ny;
            anchor = fa;
            gravity = fg;
        }
    }

    // ── SLIDE_X ──────────────────────────────────────────────────────────
    if adj & positioner_constraint::SLIDE_X != 0 {
        if x + w > output_w {
            x = (output_w - w).max(0);
        }
        if x < 0 {
            x = 0;
        }
    }

    // ── SLIDE_Y ──────────────────────────────────────────────────────────
    if adj & positioner_constraint::SLIDE_Y != 0 {
        if y + h > output_h {
            y = (output_h - h).max(0);
        }
        if y < 0 {
            y = 0;
        }
    }

    // ── RESIZE_X ─────────────────────────────────────────────────────────
    if adj & positioner_constraint::RESIZE_X != 0 {
        if x < 0 {
            x = 0;
        }
        if x + w > output_w {
            w = (output_w - x).max(1);
        }
    }

    // ── RESIZE_Y ─────────────────────────────────────────────────────────
    if adj & positioner_constraint::RESIZE_Y != 0 {
        if y < 0 {
            y = 0;
        }
        if y + h > output_h {
            h = (output_h - y).max(1);
        }
    }

    (x, y, w, h)
}

// ── Geometry ─────────────────────────────────────────────────────────────────

/// An axis-aligned rectangle in surface-local coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

/// A width/height pair (used for min/max size hints).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub w: i32,
    pub h: i32,
}

// ── Serial generator ─────────────────────────────────────────────────────────

/// Monotonically increasing serial number source.
///
/// Serials are handed out by the compositor to pair `xdg_surface.configure`
/// events with `xdg_surface.ack_configure` requests.
#[derive(Debug, Default)]
pub struct SerialGenerator(u32);

impl SerialGenerator {
    pub fn new() -> Self {
        Self(0)
    }

    /// Return the next serial.
    ///
    /// The counter wraps on overflow, which is safe: the Wayland xdg-shell
    /// spec only requires that pending serials be tracked within a single
    /// configure/ack cycle.  A wrap would only cause confusion if more than
    /// 2³² configures were outstanding simultaneously, which is not a realistic
    /// scenario.  Outstanding pending serials are validated by set membership,
    /// not by magnitude, so wrap-around does not introduce security issues.
    pub fn next(&mut self) -> ConfigureSerial {
        self.0 = self.0.wrapping_add(1);
        self.0
    }
}

// ── Error types ──────────────────────────────────────────────────────────────

/// Protocol-level errors raised by the blossom state machine.
///
/// Many of these correspond directly to Wayland protocol error enums; the
/// compositor is responsible for encoding them as `wl_display.error` events
/// before closing the offending client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlossomError {
    /// A second `xdg_surface` was requested for a `wl_surface` that already
    /// has one.  `xdg_wm_base` error `role`.
    XdgSurfaceAlreadyExists { surface: SurfaceId },
    /// A role (toplevel or popup) was assigned to an `xdg_surface` that
    /// already has a role.  `xdg_surface` error `already_constructed`.
    XdgSurfaceAlreadyHasRole { xdg_surface: ObjectId },
    /// A second `xdg_toplevel` was requested from an `xdg_surface` that
    /// already emitted one (same as `AlreadyHasRole` but more specific).
    ToplevelAlreadyCreated { xdg_surface: ObjectId },
    /// The referenced `xdg_surface` object ID is not known to blossom.
    UnknownXdgSurface { id: ObjectId },
    /// The referenced `xdg_toplevel` object ID is not known to blossom.
    UnknownToplevel { id: ObjectId },
    /// `ack_configure(serial)` referred to a serial not in the pending queue.
    /// `xdg_surface` error `invalid_serial`.
    UnknownSerial { serial: ConfigureSerial },
    /// The client committed a buffer before completing the initial
    /// configure/ack_configure handshake.  `xdg_surface` error
    /// `unconfigured_buffer`.
    CommitBeforeAckConfigure { xdg_surface: ObjectId },
    /// The referenced `xdg_popup` object ID is not known to blossom.
    UnknownPopup { id: ObjectId },
}

// ── Command output ────────────────────────────────────────────────────────────

/// Toplevel state atoms sent inside `xdg_toplevel.configure` events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdgToplevelStateAtom {
    Maximized,
    Fullscreen,
    Resizing,
    Activated,
    TiledLeft,
    TiledRight,
    TiledTop,
    TiledBottom,
}

/// An action that the compositor must carry out in response to a blossom
/// state-machine transition.
#[derive(Debug, Clone)]
pub enum BlossomCommand {
    /// Send `xdg_toplevel.configure(width, height, states)` to the client.
    SendXdgToplevelConfigure {
        client: ClientId,
        xdg_toplevel: ObjectId,
        width: i32,
        height: i32,
        states: Vec<XdgToplevelStateAtom>,
    },
    /// Send `xdg_popup.configure(x, y, width, height)` to the client.
    SendXdgPopupConfigure {
        client: ClientId,
        xdg_popup: ObjectId,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    },
    /// Send `xdg_surface.configure(serial)` to the client.
    SendXdgSurfaceConfigure { client: ClientId, xdg_surface: ObjectId, serial: ConfigureSerial },
    /// The surface has completed the configure/ack handshake and committed a
    /// buffer; it is now eligible to be made visible (mapped).
    MarkSurfaceReadyForMapping { surface: SurfaceId },
    /// The compositor should treat the top strip of this toplevel surface as
    /// shell chrome that can receive compositor-owned pointer gestures.
    SetToplevelChrome { surface: SurfaceId, titlebar_height: u32, frame_thickness: u32 },
    /// The compositor should send `xdg_toplevel.close` and initiate teardown.
    CloseToplevel { toplevel: ToplevelId },
    /// Send `xdg_popup.popup_done` to the client, dismissing the popup.
    DismissPopup { client: ClientId, xdg_popup: PopupId },
    /// Send `xdg_wm_base.ping(serial)` to the client.
    SendPing { client: ClientId, wm_base: ObjectId, serial: ConfigureSerial },
}

// ── Internal state ────────────────────────────────────────────────────────────

/// The role assigned to an `xdg_surface`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XdgRole {
    Toplevel(ToplevelId),
    Popup(PopupId),
}

/// Per-`xdg_surface` protocol state tracked by blossom.
#[derive(Debug, Clone)]
pub struct XdgSurfaceState {
    /// Which client connection owns this object.
    pub client: ClientId,
    /// The underlying `wl_surface` this xdg_surface wraps.
    pub wl_surface: SurfaceId,
    /// Role assigned (toplevel or popup), if any.
    pub role: Option<XdgRole>,
    /// `true` after at least one `ack_configure` has been received.
    pub configured: bool,
    /// `true` after the first valid (post-configure) buffer commit.
    pub mapped: bool,
    /// Serials for which `configure` events have been sent but not yet
    /// acknowledged by the client.
    pub pending_configures: VecDeque<ConfigureSerial>,
    /// The window geometry hint most recently set by the client.
    pub window_geometry: Option<Rect>,
}

/// Client intent flags for a toplevel — accepted as hints, not enforced.
#[derive(Debug, Clone, Default)]
pub struct ToplevelRequested {
    pub maximized: bool,
    pub fullscreen: bool,
    pub minimized: bool,
}

/// Per-`xdg_toplevel` protocol state tracked by blossom.
#[derive(Debug, Clone)]
pub struct XdgToplevelState {
    /// The client connection that owns this toplevel.
    pub client: ClientId,
    /// Human-readable window title (may be empty).
    pub title: Option<String>,
    /// Application identifier (e.g. `"org.example.App"`).
    pub app_id: Option<String>,
    /// Parent toplevel, if this is a child window.
    pub parent: Option<ToplevelId>,
    /// Minimum size hint set by the client.
    pub min_size: Option<Size>,
    /// Maximum size hint set by the client.
    pub max_size: Option<Size>,
    /// Current client-requested window state.
    pub requested: ToplevelRequested,
}

/// Per-`xdg_popup` protocol state tracked by blossom.
#[derive(Debug, Clone)]
pub struct XdgPopupState {
    /// The client connection that owns this popup.
    pub client: ClientId,
    /// Parent xdg_surface, if the client supplied one.
    pub parent: Option<ObjectId>,
    /// Positioner object used to create the popup.
    pub positioner: ObjectId,
}

// ── Main state machine ────────────────────────────────────────────────────────

/// The `blossom` xdg-shell state machine.
///
/// One `Blossom` instance lives in the compositor.  All xdg-shell requests
/// are routed through it; it validates lifecycle rules and returns a list of
/// [`BlossomCommand`]s for the compositor to execute.
///
/// # Object IDs
///
/// `blossom` uses the Wayland `ObjectId` (u32) assigned by the client as the
/// key for both `xdg_surface` and `xdg_toplevel` objects.  These IDs are
/// per-client, so a multi-client compositor must either namespace them (e.g.
/// by `(ClientId, ObjectId)`) or create one `Blossom` per client.  For
/// simplicity in v1 the caller is responsible for any scoping.
pub struct Blossom {
    /// xdg_surface ObjectId → state.
    surfaces: BTreeMap<ObjectId, XdgSurfaceState>,
    /// xdg_toplevel ObjectId → state.
    toplevels: BTreeMap<ObjectId, XdgToplevelState>,
    /// xdg_popup ObjectId → state.
    popups: BTreeMap<ObjectId, XdgPopupState>,
    /// xdg_positioner ObjectId → state.
    positioners: BTreeMap<ObjectId, PositionerState>,
    /// wl_surface SurfaceId → xdg_surface ObjectId (uniqueness check).
    surface_to_xdg: BTreeMap<SurfaceId, ObjectId>,
    /// Monotonically increasing serial for configure events.
    serial: SerialGenerator,
}

impl Default for Blossom {
    fn default() -> Self {
        Self::new()
    }
}

impl Blossom {
    /// Create a new, empty blossom state machine.
    pub fn new() -> Self {
        Self {
            surfaces: BTreeMap::new(),
            toplevels: BTreeMap::new(),
            popups: BTreeMap::new(),
            positioners: BTreeMap::new(),
            surface_to_xdg: BTreeMap::new(),
            serial: SerialGenerator::new(),
        }
    }

    // ── xdg_wm_base requests ─────────────────────────────────────────────

    /// `xdg_wm_base.get_xdg_surface(new_id, surface)`.
    ///
    /// Creates a new `xdg_surface` wrapping `wl_surface`.  Errors if
    /// `wl_surface` already has an `xdg_surface`.
    pub fn get_xdg_surface(
        &mut self,
        client: ClientId,
        xdg_surface_id: ObjectId,
        wl_surface: SurfaceId,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        if self.surface_to_xdg.contains_key(&wl_surface) {
            return Err(BlossomError::XdgSurfaceAlreadyExists { surface: wl_surface });
        }
        self.surface_to_xdg.insert(wl_surface, xdg_surface_id);
        self.surfaces.insert(
            xdg_surface_id,
            XdgSurfaceState {
                client,
                wl_surface,
                role: None,
                configured: false,
                mapped: false,
                pending_configures: VecDeque::new(),
                window_geometry: None,
            },
        );
        Ok(vec![])
    }

    /// `xdg_wm_base.create_positioner(new_id)`.
    ///
    /// Allocates a new positioner object with default state.  The client
    /// populates it via `set_size`, `set_anchor_rect`, etc. before passing it
    /// to `get_popup`.
    pub fn create_positioner(&mut self, id: ObjectId) {
        self.positioners.insert(id, PositionerState::default());
    }

    /// `xdg_positioner.set_size(width, height)`.
    pub fn positioner_set_size(&mut self, id: ObjectId, w: i32, h: i32) {
        if let Some(p) = self.positioners.get_mut(&id) {
            p.size = Some((w.max(1), h.max(1)));
        }
    }

    /// `xdg_positioner.set_anchor_rect(x, y, width, height)`.
    pub fn positioner_set_anchor_rect(&mut self, id: ObjectId, x: i32, y: i32, w: i32, h: i32) {
        if let Some(p) = self.positioners.get_mut(&id) {
            p.anchor_rect = (x, y, w.max(0), h.max(0));
        }
    }

    /// `xdg_positioner.set_anchor(anchor)`.
    pub fn positioner_set_anchor(&mut self, id: ObjectId, anchor: u32) {
        if let Some(p) = self.positioners.get_mut(&id) {
            p.anchor = anchor;
        }
    }

    /// `xdg_positioner.set_gravity(gravity)`.
    pub fn positioner_set_gravity(&mut self, id: ObjectId, gravity: u32) {
        if let Some(p) = self.positioners.get_mut(&id) {
            p.gravity = gravity;
        }
    }

    /// `xdg_positioner.set_constraint_adjustment(constraint_adjustment)`.
    pub fn positioner_set_constraint_adjustment(&mut self, id: ObjectId, adj: u32) {
        if let Some(p) = self.positioners.get_mut(&id) {
            p.constraint_adjustment = adj;
        }
    }

    /// `xdg_positioner.set_offset(x, y)`.
    pub fn positioner_set_offset(&mut self, id: ObjectId, x: i32, y: i32) {
        if let Some(p) = self.positioners.get_mut(&id) {
            p.offset = (x, y);
        }
    }

    /// `xdg_positioner.destroy` — remove positioner state.
    pub fn positioner_destroy(&mut self, id: ObjectId) {
        self.positioners.remove(&id);
    }

    /// `xdg_wm_base.pong(serial)` — consume a pending ping.
    pub fn pong(&mut self, _serial: ConfigureSerial) -> Vec<BlossomCommand> {
        // In v1 we do not track pending pings strictly; just consume silently.
        vec![]
    }

    /// `xdg_wm_base.destroy` — no-op (caller is responsible for cleanup).
    pub fn destroy_wm_base(&mut self) -> Vec<BlossomCommand> {
        vec![]
    }

    // ── xdg_surface requests ─────────────────────────────────────────────

    /// `xdg_surface.get_toplevel(new_id)`.
    ///
    /// Assigns the toplevel role to `xdg_surface_id` and emits an initial
    /// `configure` sequence immediately.
    pub fn get_toplevel(
        &mut self,
        client: ClientId,
        xdg_surface_id: ObjectId,
        toplevel_id: ObjectId,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        let surface = self
            .surfaces
            .get_mut(&xdg_surface_id)
            .ok_or(BlossomError::UnknownXdgSurface { id: xdg_surface_id })?;

        if surface.role.is_some() {
            return Err(BlossomError::XdgSurfaceAlreadyHasRole { xdg_surface: xdg_surface_id });
        }

        surface.role = Some(XdgRole::Toplevel(toplevel_id));
        let wl_surface = surface.wl_surface;

        self.toplevels.insert(
            toplevel_id,
            XdgToplevelState {
                client,
                title: None,
                app_id: None,
                parent: None,
                min_size: None,
                max_size: None,
                requested: ToplevelRequested::default(),
            },
        );

        // Emit initial configure sequence immediately.
        let serial = self.serial.next();
        let surf = self.surfaces.get_mut(&xdg_surface_id).unwrap();
        surf.pending_configures.push_back(serial);

        Ok(vec![
            BlossomCommand::SetToplevelChrome {
                surface: wl_surface,
                titlebar_height: DEFAULT_TITLEBAR_HEIGHT,
                frame_thickness: DEFAULT_FRAME_THICKNESS,
            },
            BlossomCommand::SendXdgToplevelConfigure {
                client,
                xdg_toplevel: toplevel_id,
                width: 0,
                height: 0,
                states: vec![],
            },
            BlossomCommand::SendXdgSurfaceConfigure { client, xdg_surface: xdg_surface_id, serial },
        ])
    }

    /// `xdg_surface.get_popup(new_id, parent, positioner)`.
    ///
    /// `output_size` — compositor output dimensions `(width, height)` used for
    /// constraint adjustment.  Pass `None` (or large values) to skip bounds
    /// clamping.
    pub fn get_popup(
        &mut self,
        client: ClientId,
        xdg_surface_id: ObjectId,
        popup_id: ObjectId,
        parent: Option<ObjectId>,
        positioner_id: ObjectId,
        output_size: Option<(i32, i32)>,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        let surface = self
            .surfaces
            .get_mut(&xdg_surface_id)
            .ok_or(BlossomError::UnknownXdgSurface { id: xdg_surface_id })?;

        if surface.role.is_some() {
            return Err(BlossomError::XdgSurfaceAlreadyHasRole { xdg_surface: xdg_surface_id });
        }

        surface.role = Some(XdgRole::Popup(popup_id));
        self.popups.insert(popup_id, XdgPopupState { client, parent, positioner: positioner_id });

        let serial = self.serial.next();
        surface.pending_configures.push_back(serial);

        // Compute placement from the positioner, falling back to a 1×1 popup
        // centered at the origin when the positioner is unknown.
        let default_pos = PositionerState::default();
        let pos = self.positioners.get(&positioner_id).unwrap_or(&default_pos);
        let (out_w, out_h) = output_size.unwrap_or((i32::MAX, i32::MAX));
        let (x, y, w, h) = compute_popup_placement(pos, out_w, out_h);

        Ok(vec![
            BlossomCommand::SendXdgPopupConfigure {
                client,
                xdg_popup: popup_id,
                x,
                y,
                width: w,
                height: h,
            },
            BlossomCommand::SendXdgSurfaceConfigure { client, xdg_surface: xdg_surface_id, serial },
        ])
    }

    /// `xdg_surface.set_window_geometry(x, y, w, h)`.
    pub fn set_window_geometry(
        &mut self,
        xdg_surface_id: ObjectId,
        rect: Rect,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        let surface = self
            .surfaces
            .get_mut(&xdg_surface_id)
            .ok_or(BlossomError::UnknownXdgSurface { id: xdg_surface_id })?;
        surface.window_geometry = Some(rect);
        Ok(vec![])
    }

    /// `xdg_surface.ack_configure(serial)`.
    ///
    /// Validates that `serial` is in the pending queue and marks the surface
    /// as configured (clears all serials up to and including `serial`).
    pub fn ack_configure(
        &mut self,
        xdg_surface_id: ObjectId,
        serial: ConfigureSerial,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        let surface = self
            .surfaces
            .get_mut(&xdg_surface_id)
            .ok_or(BlossomError::UnknownXdgSurface { id: xdg_surface_id })?;

        // The serial must appear somewhere in the pending queue.
        let pos = surface
            .pending_configures
            .iter()
            .position(|&s| s == serial)
            .ok_or(BlossomError::UnknownSerial { serial })?;

        // Drain all serials up to and including the acknowledged one.
        for _ in 0..=pos {
            surface.pending_configures.pop_front();
        }

        surface.configured = true;
        Ok(vec![])
    }

    /// `xdg_surface.destroy`.
    ///
    /// Removes the surface and its associated toplevel (if any) from blossom's
    /// tables.
    pub fn destroy_xdg_surface(
        &mut self,
        xdg_surface_id: ObjectId,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        let surface = self
            .surfaces
            .remove(&xdg_surface_id)
            .ok_or(BlossomError::UnknownXdgSurface { id: xdg_surface_id })?;

        self.surface_to_xdg.remove(&surface.wl_surface);

        match surface.role {
            Some(XdgRole::Toplevel(tid)) => {
                self.toplevels.remove(&tid);
            }
            Some(XdgRole::Popup(pid)) => {
                self.popups.remove(&pid);
            }
            None => {}
        }

        Ok(vec![])
    }

    /// `xdg_popup.destroy`.
    pub fn destroy_popup(
        &mut self,
        popup_id: ObjectId,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        self.popups.remove(&popup_id).ok_or(BlossomError::UnknownPopup { id: popup_id })?;

        for surface in self.surfaces.values_mut() {
            if surface.role == Some(XdgRole::Popup(popup_id)) {
                surface.role = None;
                break;
            }
        }

        Ok(vec![])
    }

    /// Called by the compositor when `wl_surface.commit` is received for a
    /// surface that has an `xdg_surface` role.
    ///
    /// `has_buffer`: whether the surface has a pending buffer attached.
    ///
    /// Returns an error if the client is violating the protocol (e.g.
    /// committing a buffer before `ack_configure`).  Returns
    /// [`BlossomCommand::MarkSurfaceReadyForMapping`] on the first valid
    /// commit with a buffer.
    pub fn on_surface_commit(
        &mut self,
        xdg_surface_id: ObjectId,
        has_buffer: bool,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        let surface = self
            .surfaces
            .get_mut(&xdg_surface_id)
            .ok_or(BlossomError::UnknownXdgSurface { id: xdg_surface_id })?;

        if has_buffer && !surface.configured {
            return Err(BlossomError::CommitBeforeAckConfigure { xdg_surface: xdg_surface_id });
        }

        if has_buffer && surface.configured && !surface.mapped {
            surface.mapped = true;
            return Ok(vec![BlossomCommand::MarkSurfaceReadyForMapping {
                surface: surface.wl_surface,
            }]);
        }

        Ok(vec![])
    }

    // ── xdg_toplevel requests ────────────────────────────────────────────

    /// `xdg_toplevel.set_title(title)`.
    pub fn set_title(
        &mut self,
        toplevel_id: ObjectId,
        title: String,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        let tl = self
            .toplevels
            .get_mut(&toplevel_id)
            .ok_or(BlossomError::UnknownToplevel { id: toplevel_id })?;
        tl.title = Some(title);
        Ok(vec![])
    }

    /// `xdg_toplevel.set_app_id(app_id)`.
    pub fn set_app_id(
        &mut self,
        toplevel_id: ObjectId,
        app_id: String,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        let tl = self
            .toplevels
            .get_mut(&toplevel_id)
            .ok_or(BlossomError::UnknownToplevel { id: toplevel_id })?;
        tl.app_id = Some(app_id);
        Ok(vec![])
    }

    /// `xdg_toplevel.set_parent(parent_id)`.
    pub fn set_parent(
        &mut self,
        toplevel_id: ObjectId,
        parent: Option<ToplevelId>,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        let tl = self
            .toplevels
            .get_mut(&toplevel_id)
            .ok_or(BlossomError::UnknownToplevel { id: toplevel_id })?;
        tl.parent = parent;
        Ok(vec![])
    }

    /// `xdg_toplevel.set_min_size(w, h)`.
    pub fn set_min_size(
        &mut self,
        toplevel_id: ObjectId,
        size: Size,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        let tl = self
            .toplevels
            .get_mut(&toplevel_id)
            .ok_or(BlossomError::UnknownToplevel { id: toplevel_id })?;
        tl.min_size = Some(size);
        Ok(vec![])
    }

    /// `xdg_toplevel.set_max_size(w, h)`.
    pub fn set_max_size(
        &mut self,
        toplevel_id: ObjectId,
        size: Size,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        let tl = self
            .toplevels
            .get_mut(&toplevel_id)
            .ok_or(BlossomError::UnknownToplevel { id: toplevel_id })?;
        tl.max_size = Some(size);
        Ok(vec![])
    }

    /// `xdg_toplevel.set_maximized` — record client intent.
    pub fn set_maximized(
        &mut self,
        toplevel_id: ObjectId,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        let tl = self
            .toplevels
            .get_mut(&toplevel_id)
            .ok_or(BlossomError::UnknownToplevel { id: toplevel_id })?;
        tl.requested.maximized = true;
        Ok(vec![])
    }

    /// `xdg_toplevel.unset_maximized` — clear client intent.
    pub fn unset_maximized(
        &mut self,
        toplevel_id: ObjectId,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        let tl = self
            .toplevels
            .get_mut(&toplevel_id)
            .ok_or(BlossomError::UnknownToplevel { id: toplevel_id })?;
        tl.requested.maximized = false;
        Ok(vec![])
    }

    /// `xdg_toplevel.set_fullscreen` — record client intent.
    pub fn set_fullscreen(
        &mut self,
        toplevel_id: ObjectId,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        let tl = self
            .toplevels
            .get_mut(&toplevel_id)
            .ok_or(BlossomError::UnknownToplevel { id: toplevel_id })?;
        tl.requested.fullscreen = true;
        Ok(vec![])
    }

    /// `xdg_toplevel.unset_fullscreen` — clear client intent.
    pub fn unset_fullscreen(
        &mut self,
        toplevel_id: ObjectId,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        let tl = self
            .toplevels
            .get_mut(&toplevel_id)
            .ok_or(BlossomError::UnknownToplevel { id: toplevel_id })?;
        tl.requested.fullscreen = false;
        Ok(vec![])
    }

    /// `xdg_toplevel.set_minimized` — record client intent.
    pub fn set_minimized(
        &mut self,
        toplevel_id: ObjectId,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        let tl = self
            .toplevels
            .get_mut(&toplevel_id)
            .ok_or(BlossomError::UnknownToplevel { id: toplevel_id })?;
        tl.requested.minimized = true;
        Ok(vec![])
    }

    /// `xdg_toplevel.show_window_menu` — accepted as no-op in v1.
    pub fn show_window_menu(
        &mut self,
        toplevel_id: ObjectId,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        self.toplevels
            .get(&toplevel_id)
            .ok_or(BlossomError::UnknownToplevel { id: toplevel_id })?;
        Ok(vec![])
    }

    /// `xdg_toplevel.move` — accepted as no-op in v1.
    pub fn request_move(
        &mut self,
        toplevel_id: ObjectId,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        self.toplevels
            .get(&toplevel_id)
            .ok_or(BlossomError::UnknownToplevel { id: toplevel_id })?;
        Ok(vec![])
    }

    /// `xdg_toplevel.resize` — accepted as no-op in v1.
    pub fn request_resize(
        &mut self,
        toplevel_id: ObjectId,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        self.toplevels
            .get(&toplevel_id)
            .ok_or(BlossomError::UnknownToplevel { id: toplevel_id })?;
        Ok(vec![])
    }

    /// `xdg_toplevel.destroy`.
    ///
    /// Removes the toplevel from blossom's table.  The owning `xdg_surface`
    /// must be destroyed separately.
    pub fn destroy_toplevel(
        &mut self,
        toplevel_id: ObjectId,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        self.toplevels
            .remove(&toplevel_id)
            .ok_or(BlossomError::UnknownToplevel { id: toplevel_id })?;
        Ok(vec![])
    }

    // ── Compositor-initiated ─────────────────────────────────────────────

    /// Emit a ping request (compositor-initiated liveness check).
    pub fn generate_ping(
        &mut self,
        client: ClientId,
        wm_base: ObjectId,
    ) -> (ConfigureSerial, BlossomCommand) {
        let serial = self.serial.next();
        (serial, BlossomCommand::SendPing { client, wm_base, serial })
    }

    /// Emit a compositor-initiated configure for the toplevel wrapping
    /// `wl_surface`.
    pub fn configure_toplevel_for_surface(
        &mut self,
        wl_surface: SurfaceId,
        width: i32,
        height: i32,
        states: Vec<XdgToplevelStateAtom>,
    ) -> Option<Vec<BlossomCommand>> {
        let xdg_surface = *self.surface_to_xdg.get(&wl_surface)?;
        let surface = self.surfaces.get_mut(&xdg_surface)?;
        let toplevel = match surface.role {
            Some(XdgRole::Toplevel(id)) => id,
            _ => return None,
        };
        let client = surface.client;
        let serial = self.serial.next();
        surface.pending_configures.push_back(serial);
        Some(vec![
            BlossomCommand::SendXdgToplevelConfigure {
                client,
                xdg_toplevel: toplevel,
                width,
                height,
                states,
            },
            BlossomCommand::SendXdgSurfaceConfigure { client, xdg_surface, serial },
        ])
    }

    /// Emit configure events for all toplevels that should react to a display
    /// resize (e.g. maximized or fullscreen windows).
    pub fn generate_resize_configures(
        &mut self,
        new_width: i32,
        new_height: i32,
    ) -> Vec<BlossomCommand> {
        let mut cmds = Vec::new();
        for (&tid, tl) in &self.toplevels {
            if tl.requested.maximized || tl.requested.fullscreen {
                let mut states = Vec::new();
                if tl.requested.maximized {
                    states.push(XdgToplevelStateAtom::Maximized);
                }
                if tl.requested.fullscreen {
                    states.push(XdgToplevelStateAtom::Fullscreen);
                }

                // Find the xdg_surface wrapping this toplevel.
                let entry =
                    self.surfaces.iter_mut().find(|(_, s)| s.role == Some(XdgRole::Toplevel(tid)));
                if let Some((&xs_id, xs)) = entry {
                    let serial = self.serial.next();
                    xs.pending_configures.push_back(serial);
                    cmds.push(BlossomCommand::SendXdgToplevelConfigure {
                        client: tl.client,
                        xdg_toplevel: tid,
                        width: new_width,
                        height: new_height,
                        states,
                    });
                    cmds.push(BlossomCommand::SendXdgSurfaceConfigure {
                        client: tl.client,
                        xdg_surface: xs_id,
                        serial,
                    });
                }
            }
        }
        cmds
    }

    /// Emit compositor-initiated `xdg_toplevel.close` for the toplevel wrapping
    /// `wl_surface`.
    pub fn close_toplevel_for_surface(&self, wl_surface: SurfaceId) -> Option<Vec<BlossomCommand>> {
        let xdg_surface = *self.surface_to_xdg.get(&wl_surface)?;
        let surface = self.surfaces.get(&xdg_surface)?;
        let toplevel = match surface.role {
            Some(XdgRole::Toplevel(id)) => id,
            _ => return None,
        };
        Some(vec![BlossomCommand::CloseToplevel { toplevel }])
    }

    /// Emit compositor-initiated `xdg_popup.popup_done` for the popup wrapping
    /// `wl_surface`.
    ///
    /// Returns `Some(commands)` when a popup role is found for the surface, or
    /// `None` when the surface is not a popup.
    pub fn dismiss_popup_for_surface(&self, wl_surface: SurfaceId) -> Option<Vec<BlossomCommand>> {
        let xdg_surface = *self.surface_to_xdg.get(&wl_surface)?;
        let surface = self.surfaces.get(&xdg_surface)?;
        let popup_id = match surface.role {
            Some(XdgRole::Popup(id)) => id,
            _ => return None,
        };
        let popup = self.popups.get(&popup_id)?;
        Some(vec![BlossomCommand::DismissPopup { client: popup.client, xdg_popup: popup_id }])
    }

    /// Return the `wl_surface` IDs of all xdg_surfaces that currently have a
    /// popup role (i.e. active `xdg_popup` objects tracked by blossom).
    pub fn active_popup_wl_surfaces(&self) -> Vec<SurfaceId> {
        self.surfaces
            .values()
            .filter(|s| matches!(s.role, Some(XdgRole::Popup(_))))
            .map(|s| s.wl_surface)
            .collect()
    }

    // ── Accessors ────────────────────────────────────────────────────────

    /// Read-only access to an `xdg_surface` state record.
    pub fn xdg_surface(&self, id: ObjectId) -> Option<&XdgSurfaceState> {
        self.surfaces.get(&id)
    }

    /// Read-only access to an `xdg_toplevel` state record.
    pub fn toplevel(&self, id: ObjectId) -> Option<&XdgToplevelState> {
        self.toplevels.get(&id)
    }

    /// `true` if the surface has completed at least one configure/ack cycle.
    pub fn xdg_surface_configured(&self, id: ObjectId) -> bool {
        self.surfaces.get(&id).map_or(false, |s| s.configured)
    }

    /// Look up which `xdg_surface` object wraps a given `wl_surface`.
    pub fn xdg_surface_for_wl_surface(&self, wl_surface: SurfaceId) -> Option<ObjectId> {
        self.surface_to_xdg.get(&wl_surface).copied()
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── helpers ──────────────────────────────────────────────────────────

    const CLIENT: ClientId = 1;
    const SURFACE_ID: SurfaceId = 10;
    const XDG_SURF: ObjectId = 11;
    const TOPLEVEL: ObjectId = 12;
    const XDG_SURF2: ObjectId = 21;
    const TOPLEVEL2: ObjectId = 22;
    const SURFACE_ID2: SurfaceId = 20;
    const POPUP: ObjectId = 31;
    const POSITIONER: ObjectId = 32;

    fn setup() -> Blossom {
        Blossom::new()
    }

    fn make_xdg_surface(b: &mut Blossom) -> Vec<BlossomCommand> {
        b.get_xdg_surface(CLIENT, XDG_SURF, SURFACE_ID).unwrap()
    }

    fn make_toplevel(b: &mut Blossom) -> Vec<BlossomCommand> {
        b.get_toplevel(CLIENT, XDG_SURF, TOPLEVEL).unwrap()
    }

    fn extract_configure_serial(cmds: &[BlossomCommand]) -> ConfigureSerial {
        for cmd in cmds {
            if let BlossomCommand::SendXdgSurfaceConfigure { serial, .. } = cmd {
                return *serial;
            }
        }
        panic!("no SendXdgSurfaceConfigure in commands");
    }

    // ── xdg_surface lifecycle ────────────────────────────────────────────

    #[test]
    fn create_xdg_surface_succeeds() {
        let mut b = setup();
        let cmds = make_xdg_surface(&mut b);
        assert!(cmds.is_empty(), "get_xdg_surface should emit no commands");
        assert!(b.xdg_surface(XDG_SURF).is_some());
    }

    #[test]
    fn duplicate_xdg_surface_for_same_wl_surface_errors() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        let err = b.get_xdg_surface(CLIENT, XDG_SURF2, SURFACE_ID).unwrap_err();
        assert_eq!(err, BlossomError::XdgSurfaceAlreadyExists { surface: SURFACE_ID });
    }

    #[test]
    fn get_toplevel_assigns_role_and_emits_configure() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        let cmds = make_toplevel(&mut b);

        assert_eq!(cmds.len(), 3, "get_toplevel must emit chrome + configure commands");

        let has_chrome = cmds.iter().any(|c| {
            matches!(c, BlossomCommand::SetToplevelChrome { surface, titlebar_height, frame_thickness }
                if *surface == SURFACE_ID
                    && *titlebar_height == DEFAULT_TITLEBAR_HEIGHT
                    && *frame_thickness == DEFAULT_FRAME_THICKNESS)
        });
        assert!(has_chrome, "must emit toplevel chrome metadata");

        let has_tl_configure = cmds.iter().any(|c| {
            matches!(c, BlossomCommand::SendXdgToplevelConfigure { xdg_toplevel, width, height, states, .. }
                if *xdg_toplevel == TOPLEVEL && *width == 0 && *height == 0 && states.is_empty())
        });
        assert!(has_tl_configure, "must emit xdg_toplevel.configure(0,0,[])");

        let has_surf_configure = cmds.iter().any(|c| {
            matches!(c, BlossomCommand::SendXdgSurfaceConfigure { xdg_surface, .. }
                if *xdg_surface == XDG_SURF)
        });
        assert!(has_surf_configure, "must emit xdg_surface.configure(serial)");

        assert_eq!(b.xdg_surface(XDG_SURF).unwrap().role, Some(XdgRole::Toplevel(TOPLEVEL)));
    }

    #[test]
    fn second_get_toplevel_on_same_surface_errors() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        make_toplevel(&mut b);
        let err = b.get_toplevel(CLIENT, XDG_SURF, TOPLEVEL2).unwrap_err();
        assert_eq!(err, BlossomError::XdgSurfaceAlreadyHasRole { xdg_surface: XDG_SURF });
    }

    #[test]
    fn get_toplevel_emits_configure_in_correct_order() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        let cmds = make_toplevel(&mut b);
        // Rule: xdg_toplevel.configure must come BEFORE xdg_surface.configure.
        let tl_pos = cmds
            .iter()
            .position(|c| matches!(c, BlossomCommand::SendXdgToplevelConfigure { .. }))
            .unwrap();
        let surf_pos = cmds
            .iter()
            .position(|c| matches!(c, BlossomCommand::SendXdgSurfaceConfigure { .. }))
            .unwrap();
        assert!(tl_pos < surf_pos, "toplevel.configure must precede surface.configure");
    }

    // ── ack_configure ────────────────────────────────────────────────────

    #[test]
    fn ack_configure_with_valid_serial_marks_configured() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        let cmds = make_toplevel(&mut b);
        let serial = extract_configure_serial(&cmds);
        b.ack_configure(XDG_SURF, serial).unwrap();
        assert!(b.xdg_surface_configured(XDG_SURF));
    }

    #[test]
    fn ack_configure_with_unknown_serial_errors() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        make_toplevel(&mut b);
        let err = b.ack_configure(XDG_SURF, 9999).unwrap_err();
        assert_eq!(err, BlossomError::UnknownSerial { serial: 9999 });
    }

    // ── surface commit ───────────────────────────────────────────────────

    #[test]
    fn commit_with_buffer_before_ack_configure_errors() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        make_toplevel(&mut b);
        let err = b.on_surface_commit(XDG_SURF, true).unwrap_err();
        assert_eq!(err, BlossomError::CommitBeforeAckConfigure { xdg_surface: XDG_SURF });
    }

    #[test]
    fn commit_without_buffer_before_ack_configure_is_ok() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        make_toplevel(&mut b);
        let cmds = b.on_surface_commit(XDG_SURF, false).unwrap();
        assert!(cmds.is_empty());
    }

    #[test]
    fn first_valid_commit_emits_mark_ready() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        let cmds = make_toplevel(&mut b);
        let serial = extract_configure_serial(&cmds);
        b.ack_configure(XDG_SURF, serial).unwrap();

        let cmds = b.on_surface_commit(XDG_SURF, true).unwrap();
        assert_eq!(cmds.len(), 1);
        assert!(matches!(
            cmds[0],
            BlossomCommand::MarkSurfaceReadyForMapping { surface: SURFACE_ID }
        ));
    }

    #[test]
    fn subsequent_valid_commits_emit_nothing() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        let cmds = make_toplevel(&mut b);
        let serial = extract_configure_serial(&cmds);
        b.ack_configure(XDG_SURF, serial).unwrap();
        b.on_surface_commit(XDG_SURF, true).unwrap(); // first → MarkReady
        let cmds2 = b.on_surface_commit(XDG_SURF, true).unwrap();
        assert!(cmds2.is_empty(), "subsequent commits must not re-emit MarkReady");
    }

    #[test]
    fn compositor_resize_emits_resizing_configure_sequence() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        make_toplevel(&mut b);

        let cmds = b
            .configure_toplevel_for_surface(
                SURFACE_ID,
                640,
                480,
                vec![XdgToplevelStateAtom::Resizing],
            )
            .unwrap();

        assert_eq!(cmds.len(), 2);
        assert!(matches!(
            &cmds[0],
            BlossomCommand::SendXdgToplevelConfigure { xdg_toplevel, width, height, states, .. }
                if *xdg_toplevel == TOPLEVEL
                    && *width == 640
                    && *height == 480
                    && states == &vec![XdgToplevelStateAtom::Resizing]
        ));
        assert!(matches!(
            cmds[1],
            BlossomCommand::SendXdgSurfaceConfigure { xdg_surface: XDG_SURF, serial, .. }
                if serial > 0
        ));
    }

    #[test]
    fn compositor_close_emits_xdg_toplevel_close() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        make_toplevel(&mut b);

        let cmds = b.close_toplevel_for_surface(SURFACE_ID).unwrap();

        assert_eq!(cmds.len(), 1);
        assert!(matches!(cmds[0], BlossomCommand::CloseToplevel { toplevel: TOPLEVEL }));
    }

    // ── metadata ────────────────────────────────────────────────────────

    #[test]
    fn set_title_and_app_id_update_state() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        make_toplevel(&mut b);
        b.set_title(TOPLEVEL, String::from("My Window")).unwrap();
        b.set_app_id(TOPLEVEL, String::from("com.example.App")).unwrap();

        let tl = b.toplevel(TOPLEVEL).unwrap();
        assert_eq!(tl.title.as_deref(), Some("My Window"));
        assert_eq!(tl.app_id.as_deref(), Some("com.example.App"));
    }

    // ── destroy ──────────────────────────────────────────────────────────

    #[test]
    fn destroy_toplevel_then_xdg_surface_cleans_state() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        make_toplevel(&mut b);

        b.destroy_toplevel(TOPLEVEL).unwrap();
        assert!(b.toplevel(TOPLEVEL).is_none());

        b.destroy_xdg_surface(XDG_SURF).unwrap();
        assert!(b.xdg_surface(XDG_SURF).is_none());
        assert!(b.xdg_surface_for_wl_surface(SURFACE_ID).is_none());
    }

    #[test]
    fn destroy_xdg_surface_removes_toplevel_transitively() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        make_toplevel(&mut b);

        // Destroy xdg_surface first without explicitly destroying toplevel.
        b.destroy_xdg_surface(XDG_SURF).unwrap();
        assert!(b.toplevel(TOPLEVEL).is_none(), "transitively removed");
    }

    // ── ping/pong ────────────────────────────────────────────────────────

    #[test]
    fn ping_pong_round_trip_accepted() {
        let mut b = setup();
        let wm_base: ObjectId = 5;
        let (_serial, cmd) = b.generate_ping(CLIENT, wm_base);
        assert!(matches!(cmd, BlossomCommand::SendPing { client: CLIENT, wm_base: 5, .. }));

        // pong always succeeds
        let cmds = b.pong(42);
        assert!(cmds.is_empty());
    }

    // ── popup ────────────────────────────────────────────────────────────

    fn make_popup(b: &mut Blossom) -> Vec<BlossomCommand> {
        b.create_positioner(POSITIONER);
        b.positioner_set_size(POSITIONER, 160, 96);
        b.positioner_set_anchor_rect(POSITIONER, 24, 24, 100, 24);
        b.positioner_set_anchor(POSITIONER, positioner_anchor::BOTTOM);
        b.positioner_set_gravity(POSITIONER, positioner_anchor::BOTTOM);
        b.positioner_set_offset(POSITIONER, 0, 6);
        b.get_popup(CLIENT, XDG_SURF, POPUP, None, POSITIONER, None).unwrap()
    }

    #[test]
    fn get_popup_assigns_role_and_emits_configure() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        let cmds = make_popup(&mut b);

        assert_eq!(cmds.len(), 2, "get_popup must emit popup + surface configure commands");
        // Anchor is BOTTOM of the anchor_rect (x=24, y=24, w=100, h=24):
        //   anchor_x = 24 + 100/2 = 74; anchor_y = 24 + 24 = 48
        // Gravity BOTTOM → popup top at anchor_y; horizontally centered:
        //   popup_x = 74 - 160/2 = -6; popup_y = 48
        // Offset (0, 6) → x = -6, y = 54
        assert!(matches!(
            cmds[0],
            BlossomCommand::SendXdgPopupConfigure {
                xdg_popup: POPUP,
                x: -6,
                y: 54,
                width: 160,
                height: 96,
                ..
            }
        ));
        assert!(matches!(
            cmds[1],
            BlossomCommand::SendXdgSurfaceConfigure { xdg_surface: XDG_SURF, serial, .. }
                if serial > 0
        ));
        assert_eq!(b.xdg_surface(XDG_SURF).unwrap().role, Some(XdgRole::Popup(POPUP)));
    }

    #[test]
    fn get_popup_without_positioner_falls_back_to_defaults() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        // POSITIONER (32) was never registered — blossom should use defaults (1×1 at 0,0).
        let cmds = b.get_popup(CLIENT, XDG_SURF, POPUP, None, POSITIONER, None).unwrap();
        assert_eq!(cmds.len(), 2);
        assert!(matches!(
            cmds[0],
            BlossomCommand::SendXdgPopupConfigure {
                xdg_popup: POPUP,
                x: 0,
                y: 0,
                width: 1,
                height: 1,
                ..
            }
        ));
    }

    #[test]
    fn get_popup_on_surface_with_role_errors() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        make_toplevel(&mut b);
        let err = b.get_popup(CLIENT, XDG_SURF, POPUP, None, POSITIONER, None).unwrap_err();
        assert_eq!(err, BlossomError::XdgSurfaceAlreadyHasRole { xdg_surface: XDG_SURF });
    }

    #[test]
    fn destroy_xdg_surface_removes_popup_transitively() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        b.get_popup(CLIENT, XDG_SURF, POPUP, None, POSITIONER, None).unwrap();

        b.destroy_xdg_surface(XDG_SURF).unwrap();
        assert!(b.xdg_surface(XDG_SURF).is_none());
        assert!(b.xdg_surface_for_wl_surface(SURFACE_ID).is_none());
    }

    // ── popup dismissal ──────────────────────────────────────────────────

    #[test]
    fn dismiss_popup_for_surface_emits_popup_done() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        b.get_popup(CLIENT, XDG_SURF, POPUP, None, POSITIONER, None).unwrap();

        let cmds = b.dismiss_popup_for_surface(SURFACE_ID).unwrap();
        assert_eq!(cmds.len(), 1);
        assert!(
            matches!(cmds[0], BlossomCommand::DismissPopup { xdg_popup: POPUP, .. }),
            "must emit DismissPopup for the popup object"
        );
    }

    #[test]
    fn dismiss_popup_for_toplevel_surface_returns_none() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        make_toplevel(&mut b);

        // A toplevel surface is not a popup — dismiss_popup_for_surface must
        // return None rather than panicking or emitting an event.
        assert!(b.dismiss_popup_for_surface(SURFACE_ID).is_none());
    }

    #[test]
    fn dismiss_popup_for_unknown_surface_returns_none() {
        let b = setup();
        assert!(b.dismiss_popup_for_surface(999).is_none());
    }

    #[test]
    fn active_popup_wl_surfaces_lists_popup_surface() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        b.get_popup(CLIENT, XDG_SURF, POPUP, None, POSITIONER, None).unwrap();

        let surfaces = b.active_popup_wl_surfaces();
        assert_eq!(surfaces, vec![SURFACE_ID]);
    }

    #[test]
    fn active_popup_wl_surfaces_empty_when_only_toplevels() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        make_toplevel(&mut b);

        assert!(b.active_popup_wl_surfaces().is_empty());
    }

    // ── positioner placement ─────────────────────────────────────────────

    #[test]
    fn positioner_bottom_right_anchor_and_gravity() {
        // Popup anchors to bottom-right of anchor_rect and extends to bottom-right.
        let pos = PositionerState {
            size: Some((100, 50)),
            anchor_rect: (10, 20, 80, 40), // right=90, bottom=60
            anchor: positioner_anchor::BOTTOM_RIGHT,
            gravity: positioner_anchor::BOTTOM_RIGHT,
            ..PositionerState::default()
        };
        let (x, y, w, h) = compute_popup_placement(&pos, 1920, 1080);
        assert_eq!((x, y, w, h), (90, 60, 100, 50));
    }

    #[test]
    fn positioner_centered_none_anchor_gravity() {
        // NONE anchor → center of anchor_rect; NONE gravity → popup centered.
        let pos = PositionerState {
            size: Some((60, 40)),
            anchor_rect: (0, 0, 100, 100),
            anchor: positioner_anchor::NONE,
            gravity: positioner_anchor::NONE,
            ..PositionerState::default()
        };
        let (x, y, w, h) = compute_popup_placement(&pos, 1920, 1080);
        // anchor = center (50, 50); popup centered → (50 - 30, 50 - 20) = (20, 30)
        assert_eq!((x, y, w, h), (20, 30, 60, 40));
    }

    #[test]
    fn positioner_offset_applied() {
        let pos = PositionerState {
            size: Some((100, 50)),
            anchor_rect: (0, 0, 0, 0),
            anchor: positioner_anchor::TOP_LEFT,
            gravity: positioner_anchor::BOTTOM_RIGHT,
            offset: (5, 10),
            ..PositionerState::default()
        };
        let (x, y, w, h) = compute_popup_placement(&pos, 1920, 1080);
        // anchor at (0,0), gravity BOTTOM_RIGHT → (0,0), offset → (5,10)
        assert_eq!((x, y, w, h), (5, 10, 100, 50));
    }

    #[test]
    fn positioner_slide_x_clamps_right_overflow() {
        let pos = PositionerState {
            size: Some((200, 50)),
            anchor_rect: (900, 0, 0, 0),
            anchor: positioner_anchor::TOP_LEFT,
            gravity: positioner_anchor::BOTTOM_RIGHT,
            constraint_adjustment: positioner_constraint::SLIDE_X,
            ..PositionerState::default()
        };
        // Without constraint: x = 900, x+200 = 1100 > 800 (output_w)
        let (x, y, w, h) = compute_popup_placement(&pos, 800, 600);
        assert_eq!(x, 600, "slid left to fit: output_w - popup_w = 800 - 200");
        assert_eq!(w, 200);
        let _ = (y, h);
    }

    #[test]
    fn positioner_slide_y_clamps_bottom_overflow() {
        let pos = PositionerState {
            size: Some((100, 100)),
            anchor_rect: (0, 550, 0, 0),
            anchor: positioner_anchor::TOP_LEFT,
            gravity: positioner_anchor::BOTTOM_RIGHT,
            constraint_adjustment: positioner_constraint::SLIDE_Y,
            ..PositionerState::default()
        };
        let (x, y, w, h) = compute_popup_placement(&pos, 800, 600);
        assert_eq!(y, 500, "slid up to fit: output_h - popup_h = 600 - 100");
        let _ = (x, w, h);
    }

    #[test]
    fn positioner_flip_x_reduces_overflow() {
        // Popup placed to the right of anchor, overflows output right edge.
        // FLIP_X should flip it to the left side where it fits.
        let pos = PositionerState {
            size: Some((200, 50)),
            anchor_rect: (700, 0, 0, 0),
            anchor: positioner_anchor::TOP_RIGHT,
            gravity: positioner_anchor::BOTTOM_RIGHT,
            constraint_adjustment: positioner_constraint::FLIP_X,
            ..PositionerState::default()
        };
        // Original (no flip): anchor TOP_RIGHT on (700,0,0,0) → (700,0),
        //   gravity BOTTOM_RIGHT → popup at (700, 0). x+w = 900 > output_w=800 (100px overflow).
        // Flipped: anchor TOP_LEFT, gravity BOTTOM_LEFT →
        //   anchor_point = (700, 0), popup right edge at anchor → x = 700-200=500.
        //   x+w = 700 ≤ 800 → no overflow.  Flip accepted.
        let (x, y, w, h) = compute_popup_placement(&pos, 800, 600);
        assert_eq!(x, 500, "flip moved popup left of anchor so it fits in output");
        assert_eq!(w, 200, "width unchanged after flip");
        assert!(x + w <= 800, "popup must not overflow the right edge");
        assert!(x >= 0, "popup must not overflow the left edge");
        let _ = (y, h);
    }

    #[test]
    fn positioner_resize_x_shrinks_popup() {
        let pos = PositionerState {
            size: Some((300, 50)),
            anchor_rect: (600, 0, 0, 0),
            anchor: positioner_anchor::TOP_LEFT,
            gravity: positioner_anchor::BOTTOM_RIGHT,
            constraint_adjustment: positioner_constraint::RESIZE_X,
            ..PositionerState::default()
        };
        // x=600, w=300 → x+w=900 > 800. Resize: w = 800 - 600 = 200.
        let (x, _y, w, _h) = compute_popup_placement(&pos, 800, 600);
        assert_eq!(x, 600);
        assert_eq!(w, 200);
    }

    #[test]
    fn positioner_destroy_removes_state() {
        let mut b = setup();
        b.create_positioner(POSITIONER);
        b.positioner_set_size(POSITIONER, 100, 50);
        b.positioner_destroy(POSITIONER);
        // After destroy, get_popup with this ID falls back to defaults.
        make_xdg_surface(&mut b);
        let cmds = b.get_popup(CLIENT, XDG_SURF, POPUP, None, POSITIONER, None).unwrap();
        assert!(matches!(
            cmds[0],
            BlossomCommand::SendXdgPopupConfigure { width: 1, height: 1, .. }
        ));
    }

    #[test]
    fn two_independent_xdg_surfaces_do_not_conflict() {
        let mut b = setup();
        b.get_xdg_surface(CLIENT, XDG_SURF, SURFACE_ID).unwrap();
        b.get_xdg_surface(CLIENT, XDG_SURF2, SURFACE_ID2).unwrap();

        b.get_toplevel(CLIENT, XDG_SURF, TOPLEVEL).unwrap();
        b.get_toplevel(CLIENT, XDG_SURF2, TOPLEVEL2).unwrap();

        assert!(b.toplevel(TOPLEVEL).is_some());
        assert!(b.toplevel(TOPLEVEL2).is_some());
    }
}
pub mod input;
