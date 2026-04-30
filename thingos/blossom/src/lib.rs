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

#![no_std]

extern crate alloc;

use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

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
pub const DEFAULT_TITLEBAR_HEIGHT: u32 = 26;
/// Initial compositor-known frame thickness for v1 toplevel chrome.
pub const DEFAULT_FRAME_THICKNESS: u32 = 6;

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

    /// `xdg_wm_base.create_positioner(new_id)` — accepted as a no-op in v1.
    pub fn create_positioner(&mut self) -> Vec<BlossomCommand> {
        vec![]
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
    pub fn get_popup(
        &mut self,
        client: ClientId,
        xdg_surface_id: ObjectId,
        popup_id: ObjectId,
        parent: Option<ObjectId>,
        positioner: ObjectId,
    ) -> Result<Vec<BlossomCommand>, BlossomError> {
        let surface = self
            .surfaces
            .get_mut(&xdg_surface_id)
            .ok_or(BlossomError::UnknownXdgSurface { id: xdg_surface_id })?;

        if surface.role.is_some() {
            return Err(BlossomError::XdgSurfaceAlreadyHasRole { xdg_surface: xdg_surface_id });
        }

        surface.role = Some(XdgRole::Popup(popup_id));
        self.popups.insert(popup_id, XdgPopupState { client, parent, positioner });

        let serial = self.serial.next();
        surface.pending_configures.push_back(serial);

        Ok(vec![
            BlossomCommand::SendXdgPopupConfigure {
                client,
                xdg_popup: popup_id,
                x: 0,
                y: 0,
                width: 160,
                height: 96,
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

    #[test]
    fn get_popup_assigns_role_and_emits_configure() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        let cmds = b.get_popup(CLIENT, XDG_SURF, POPUP, None, POSITIONER).unwrap();

        assert_eq!(cmds.len(), 2, "get_popup must emit popup + surface configure commands");
        assert!(matches!(
            cmds[0],
            BlossomCommand::SendXdgPopupConfigure {
                xdg_popup: POPUP,
                x: 0,
                y: 0,
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
    fn get_popup_on_surface_with_role_errors() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        make_toplevel(&mut b);
        let err = b.get_popup(CLIENT, XDG_SURF, POPUP, None, POSITIONER).unwrap_err();
        assert_eq!(err, BlossomError::XdgSurfaceAlreadyHasRole { xdg_surface: XDG_SURF });
    }

    #[test]
    fn destroy_xdg_surface_removes_popup_transitively() {
        let mut b = setup();
        make_xdg_surface(&mut b);
        b.get_popup(CLIENT, XDG_SURF, POPUP, None, POSITIONER).unwrap();

        b.destroy_xdg_surface(XDG_SURF).unwrap();
        assert!(b.xdg_surface(XDG_SURF).is_none());
        assert!(b.xdg_surface_for_wl_surface(SURFACE_ID).is_none());
    }

    // ── multiple surfaces ────────────────────────────────────────────────

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
