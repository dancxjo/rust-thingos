//! IPC message types shared between the Wayland server thread and the main
//! bloom rendering thread.
//!
//! All messages are fixed-size packed structs transmitted over a port pair
//! with `port_send_all` / `vfs_read`.  The first byte is a discriminant
//! (`WCMD_*` or `WEVT_*`).
//!
//! # Wayland → Main (commands)
//!
//! | Discriminant         | Meaning                                       |
//! |----------------------|-----------------------------------------------|
//! | `WCMD_CREATE_SURFACE`| Register a new Wayland surface in the scene   |
//! | `WCMD_DESTROY_SURFACE`| Remove a surface from the scene              |
//! | `WCMD_IMPORT_ATTACH` | Import a shm buffer and attach it to surface  |
//! | `WCMD_DAMAGE`        | Mark a damage region on a surface             |
//! | `WCMD_COMMIT`        | Commit pending surface state                  |
//! | `WCMD_SET_CHROME`    | Mark compositor-known shell chrome geometry   |
//! | `WCMD_SET_TITLE`     | Update compositor-owned shell chrome title    |
//! | `WCMD_SET_SUBSURFACE`| Update parent/position/stacking for a subsurface |
//! | `WCMD_SET_LAYER_SURFACE`| Update wlr-layer-shell state for a surface |
//! | `WCMD_SET_OPAQUE_REGION`| Update the committed opaque region for a surface |
//! | `WCMD_SET_INPUT_REGION`| Update the committed input region for a surface |
//!
//! # Main → Wayland (events)
//!
//! | Discriminant         | Meaning                                       |
//! |----------------------|-----------------------------------------------|
//! | `WEVT_BUFFER_RELEASE`| The compositor no longer needs a buffer       |
//! | `WEVT_FRAME_DONE`    | A frame has been presented                    |
//! | `WEVT_CONFIGURE_SURFACE` | The compositor requests a toplevel size   |
//! | `WEVT_TOPLEVEL_ACTION` | The compositor requests a toplevel action  |
//! | `WEVT_POINTER_*` | Focused pointer events from Bristle/Bloom    |
//! | `WEVT_KEYBOARD_*` | Focused keyboard events from Bristle/Bloom  |
//! | `WEVT_CLOSE_LAYER_SURFACE` | Compositor closes a layer surface       |
//! | `WEVT_POINTER_SCROLL` | Scroll-wheel event for the focused surface |

// ── Discriminants ────────────────────────────────────────────────────────────

pub const WCMD_CREATE_SURFACE: u8 = 1;
pub const WCMD_DESTROY_SURFACE: u8 = 2;
pub const WCMD_IMPORT_ATTACH: u8 = 3;
pub const WCMD_DAMAGE: u8 = 4;
pub const WCMD_COMMIT: u8 = 5;
pub const WCMD_SET_CHROME: u8 = 6;
pub const WCMD_SET_TITLE: u8 = 7;
pub const WCMD_SET_SUBSURFACE: u8 = 8;
pub const WCMD_SET_LAYER_SURFACE: u8 = 9;
pub const WCMD_SET_OPAQUE_REGION: u8 = 10;
pub const WCMD_SET_INPUT_REGION: u8 = 11;
pub const MAX_TITLE_BYTES: usize = 64;

pub const WEVT_BUFFER_RELEASE: u8 = 1;
pub const WEVT_FRAME_DONE: u8 = 2;
pub const WEVT_CONFIGURE_SURFACE: u8 = 3;
pub const WEVT_TOPLEVEL_ACTION: u8 = 4;
pub const WEVT_POINTER_ENTER: u8 = 5;
pub const WEVT_POINTER_LEAVE: u8 = 6;
pub const WEVT_POINTER_MOTION: u8 = 7;
pub const WEVT_POINTER_BUTTON: u8 = 8;
pub const WEVT_KEYBOARD_ENTER: u8 = 9;
pub const WEVT_KEYBOARD_LEAVE: u8 = 10;
pub const WEVT_KEYBOARD_KEY: u8 = 11;
pub const WEVT_OUTPUT_INFO: u8 = 12;
pub const WEVT_CLOSE_LAYER_SURFACE: u8 = 13;
pub const WEVT_POINTER_SCROLL: u8 = 92;

pub const TOPLEVEL_ACTION_CLOSE: u8 = 1;
pub const TOPLEVEL_ACTION_MINIMIZE: u8 = 2;
pub const TOPLEVEL_ACTION_MAXIMIZE: u8 = 3;
pub const TOPLEVEL_ACTION_FULLSCREEN: u8 = 4;

// ── Message structs (repr C, fixed size) ─────────────────────────────────────

/// [`WCMD_CREATE_SURFACE`] — ask the main thread to allocate a scene surface.
///
/// The main thread replies by `port_send_all`-ing a `[u8; 4]` containing the
/// `bloom_surface_id` (u32 LE) back on `reply_port`.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WCmdCreateSurface {
    pub msg_type: u8, // = WCMD_CREATE_SURFACE
    pub _pad: [u8; 3],
    pub reply_port: u32,
}

/// [`WCMD_DESTROY_SURFACE`] — remove a surface from the scene.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WCmdDestroySurface {
    pub msg_type: u8, // = WCMD_DESTROY_SURFACE
    pub _pad: [u8; 3],
    pub bloom_surface_id: u32,
}

/// [`WCMD_IMPORT_ATTACH`] — import a shm buffer and attach it to a surface.
///
/// The main thread calls `display.import_buffer(handle, ...)` and then
/// `scene.attach_pending_buffer(...)`.  When the buffer is released, the main
/// thread sends [`WEvtBufferRelease`] back.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WCmdImportAttach {
    pub msg_type: u8, // = WCMD_IMPORT_ATTACH
    pub _pad: [u8; 3],
    pub bloom_surface_id: u32,
    /// Opaque key assigned by the Wayland server; echoed back in WEVT_BUFFER_RELEASE.
    pub wl_buf_key: u32,
    /// Kernel shared-memory handle (ThingId).
    pub handle: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    /// Canonical abi::pixel::PixelFormat discriminant.
    pub format: u32,
    pub offset: u64,
    pub modifier: u64,
}

/// [`WCMD_DAMAGE`] — add a damage rectangle to a surface.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WCmdDamage {
    pub msg_type: u8, // = WCMD_DAMAGE
    pub _pad: [u8; 3],
    pub bloom_surface_id: u32,
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

/// [`WCMD_COMMIT`] — commit the pending surface state.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WCmdCommit {
    pub msg_type: u8, // = WCMD_COMMIT
    /// `1` if a frame callback was registered with this commit.
    pub has_frame_callback: u8,
    pub _pad: [u8; 2],
    pub bloom_surface_id: u32,
    /// Frame callback key (meaningful only when `has_frame_callback == 1`).
    pub cb_key: u32,
}

/// [`WCMD_SET_CHROME`] — attach compositor-known shell chrome to a surface.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WCmdSetChrome {
    pub msg_type: u8, // = WCMD_SET_CHROME
    pub _pad: [u8; 3],
    pub bloom_surface_id: u32,
    pub titlebar_height: u32,
    pub frame_thickness: u32,
}

/// [`WCMD_SET_TITLE`] — update the compositor-owned title text for a surface.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WCmdSetTitle {
    pub msg_type: u8, // = WCMD_SET_TITLE
    pub title_len: u8,
    pub _pad: [u8; 2],
    pub bloom_surface_id: u32,
    pub title: [u8; MAX_TITLE_BYTES],
}

/// [`WCMD_SET_SUBSURFACE`] — declare or update a subsurface relationship.
///
/// `parent_surface_id == 0` detaches the child (used when the subsurface is
/// destroyed).  `x`/`y` is the offset of the child relative to the parent in
/// surface-local coordinates.  `z_above` is the stacking offset relative to
/// the parent's z-order: positive places the subsurface above its parent,
/// negative places it below.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WCmdSetSubsurface {
    pub msg_type: u8, // = WCMD_SET_SUBSURFACE
    pub _pad: [u8; 3],
    pub child_surface_id: u32,
    pub parent_surface_id: u32,
    pub x: i32,
    pub y: i32,
    pub z_above: i32,
}

/// [`WCMD_SET_LAYER_SURFACE`] — declare or update wlr-layer-shell state for a
/// surface.  The main thread combines the carried fields with the current
/// output dimensions to compute the absolute placement and z-order.
///
/// `active = 0` means the layer surface has been destroyed; the main thread
/// should clear any layer-shell side state and let the surface drop back
/// into the regular toplevel stacking band.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WCmdSetLayerSurface {
    pub msg_type: u8, // = WCMD_SET_LAYER_SURFACE
    pub active: u8,
    pub _pad: [u8; 2],
    pub bloom_surface_id: u32,
    /// `blossom::LayerShellLayer` discriminant (0..=3).
    pub layer: u32,
    /// `zwlr_layer_surface_v1.anchor` bitfield.
    pub anchor: u32,
    pub exclusive_zone: i32,
    pub margin_top: i32,
    pub margin_right: i32,
    pub margin_bottom: i32,
    pub margin_left: i32,
    pub width: u32,
    pub height: u32,
}

/// [`WCMD_SET_OPAQUE_REGION`] — update the committed opaque region for a surface.
///
/// Sent before [`WCMD_COMMIT`] within the same commit batch so the main thread
/// can apply the region atomically with the buffer attach.  When `has_region`
/// is `0` the region is cleared (the surface has no declared opaque area);
/// when `has_region` is `1` the rect `(x, y, w, h)` describes the opaque
/// rectangle in surface-local coordinates.
///
/// All coordinate fields are `u32`: negative Wayland region coordinates are
/// clamped to `0` by the Wayland server thread before encoding (see
/// `region_bounding_rect`).  This is intentionally conservative — the reported
/// rectangle never exceeds the actual opaque area.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WCmdSetOpaqueRegion {
    pub msg_type: u8, // = WCMD_SET_OPAQUE_REGION
    /// `1` if a region rect follows; `0` to clear the opaque region.
    pub has_region: u8,
    pub _pad: [u8; 2],
    pub bloom_surface_id: u32,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// [`WCMD_SET_INPUT_REGION`] — update the committed input region for a surface.
///
/// Sent before [`WCMD_COMMIT`] within the same commit batch so the main thread
/// can apply the region atomically with the buffer attach.  When `has_region`
/// is `0` the input region is cleared, restoring the default where the entire
/// surface receives pointer input; when `has_region` is `1` the rect
/// `(x, y, w, h)` describes the hit-testable area in surface-local coordinates.
///
/// All coordinate fields are `u32`: negative Wayland region coordinates are
/// clamped to `0` by the Wayland server thread before encoding (see
/// `region_bounding_rect`).
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WCmdSetInputRegion {
    pub msg_type: u8, // = WCMD_SET_INPUT_REGION
    /// `1` if a region rect follows; `0` to clear the input region.
    pub has_region: u8,
    pub _pad: [u8; 2],
    pub bloom_surface_id: u32,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// [`WEVT_BUFFER_RELEASE`] — the compositor no longer references a buffer.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WEvtBufferRelease {
    pub msg_type: u8, // = WEVT_BUFFER_RELEASE
    pub _pad: [u8; 3],
    pub wl_buf_key: u32,
}

/// [`WEVT_FRAME_DONE`] — a frame has been presented for the given surface.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WEvtFrameDone {
    pub msg_type: u8, // = WEVT_FRAME_DONE
    pub _pad: [u8; 3],
    pub bloom_surface_id: u32,
    pub timestamp_ms: u32,
}

/// [`WEVT_CONFIGURE_SURFACE`] — compositor-initiated xdg toplevel configure.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WEvtConfigureSurface {
    pub msg_type: u8, // = WEVT_CONFIGURE_SURFACE
    pub resizing: u8,
    pub _pad: [u8; 2],
    pub bloom_surface_id: u32,
    pub width: i32,
    pub height: i32,
}

/// [`WEVT_TOPLEVEL_ACTION`] — compositor-owned window button action.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WEvtToplevelAction {
    pub msg_type: u8, // = WEVT_TOPLEVEL_ACTION
    pub action: u8,
    pub _pad: [u8; 2],
    pub bloom_surface_id: u32,
    pub width: i32,
    pub height: i32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WEvtPointerEnter {
    pub msg_type: u8,
    pub _pad: [u8; 3],
    pub bloom_surface_id: u32,
    pub x: i32,
    pub y: i32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WEvtPointerLeave {
    pub msg_type: u8,
    pub _pad: [u8; 3],
    pub bloom_surface_id: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WEvtPointerMotion {
    pub msg_type: u8,
    pub _pad: [u8; 3],
    pub bloom_surface_id: u32,
    pub x: i32,
    pub y: i32,
    pub timestamp_ms: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WEvtPointerButton {
    pub msg_type: u8,
    pub button: u8,
    pub pressed: u8,
    pub _pad: u8,
    pub bloom_surface_id: u32,
    pub timestamp_ms: u32,
}

/// [`WEVT_POINTER_SCROLL`] — scroll-wheel event for the pointer-focused surface.
///
/// `dx` and `dy` are in the same device units emitted by the input driver's
/// `ScrollPayload` (typically ±1 per wheel detent).  The Wayland server maps
/// these to `wl_pointer.axis` values scaled to surface-local pixels.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WEvtPointerScroll {
    pub msg_type: u8, // = WEVT_POINTER_SCROLL
    pub _pad: [u8; 3],
    pub bloom_surface_id: u32,
    pub dx: i16,
    pub dy: i16,
    pub timestamp_ms: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WEvtKeyboardEnter {
    pub msg_type: u8,
    pub modifiers: u8,
    pub _pad: [u8; 2],
    pub bloom_surface_id: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WEvtKeyboardLeave {
    pub msg_type: u8,
    pub _pad: [u8; 3],
    pub bloom_surface_id: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WEvtKeyboardKey {
    pub msg_type: u8,
    pub pressed: u8,
    pub modifiers: u8,
    pub repeat: u8,
    pub bloom_surface_id: u32,
    pub key: u16,
    pub _pad: [u8; 2],
    pub timestamp_ms: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WEvtOutputInfo {
    pub msg_type: u8, // = WEVT_OUTPUT_INFO
    pub _pad: [u8; 3],
    pub width: u32,
    pub height: u32,
    pub refresh_mhz: u32,
}

/// [`WEVT_CLOSE_LAYER_SURFACE`] — compositor-initiated close of a layer surface.
///
/// The Wayland server should emit `zwlr_layer_surface_v1.closed` (opcode 1) to
/// the client owning the surface and then remove the layer-surface object.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WEvtCloseLayerSurface {
    pub msg_type: u8, // = WEVT_CLOSE_LAYER_SURFACE
    pub _pad: [u8; 3],
    pub bloom_surface_id: u32,
}

// ── Encoding helpers ─────────────────────────────────────────────────────────

macro_rules! as_bytes {
    ($v:expr, $T:ty) => {
        unsafe {
            core::slice::from_raw_parts(&$v as *const $T as *const u8, core::mem::size_of::<$T>())
        }
    };
}

pub fn encode_create_surface(reply_port: u32) -> [u8; 8] {
    let msg = WCmdCreateSurface { msg_type: WCMD_CREATE_SURFACE, _pad: [0; 3], reply_port };
    let mut out = [0u8; 8];
    out.copy_from_slice(as_bytes!(msg, WCmdCreateSurface));
    out
}

pub fn encode_destroy_surface(bloom_surface_id: u32) -> [u8; 8] {
    let msg = WCmdDestroySurface { msg_type: WCMD_DESTROY_SURFACE, _pad: [0; 3], bloom_surface_id };
    let mut out = [0u8; 8];
    out.copy_from_slice(as_bytes!(msg, WCmdDestroySurface));
    out
}

pub fn encode_import_attach(
    bloom_surface_id: u32,
    wl_buf_key: u32,
    handle: u32,
    width: u32,
    height: u32,
    stride: u32,
    format: u32,
    offset: u64,
    modifier: u64,
) -> [u8; 48] {
    let msg = WCmdImportAttach {
        msg_type: WCMD_IMPORT_ATTACH,
        _pad: [0; 3],
        bloom_surface_id,
        wl_buf_key,
        handle,
        width,
        height,
        stride,
        format,
        offset,
        modifier,
    };
    let mut out = [0u8; 48];
    out.copy_from_slice(as_bytes!(msg, WCmdImportAttach));
    out
}

pub fn encode_damage(bloom_surface_id: u32, x: i32, y: i32, w: u32, h: u32) -> [u8; 24] {
    let msg = WCmdDamage { msg_type: WCMD_DAMAGE, _pad: [0; 3], bloom_surface_id, x, y, w, h };
    let mut out = [0u8; 24];
    out.copy_from_slice(as_bytes!(msg, WCmdDamage));
    out
}

pub fn encode_commit(bloom_surface_id: u32, has_frame_callback: bool, cb_key: u32) -> [u8; 12] {
    let msg = WCmdCommit {
        msg_type: WCMD_COMMIT,
        has_frame_callback: has_frame_callback as u8,
        _pad: [0; 2],
        bloom_surface_id,
        cb_key,
    };
    let mut out = [0u8; 12];
    out.copy_from_slice(as_bytes!(msg, WCmdCommit));
    out
}

pub fn encode_set_chrome(
    bloom_surface_id: u32,
    titlebar_height: u32,
    frame_thickness: u32,
) -> [u8; 16] {
    let msg = WCmdSetChrome {
        msg_type: WCMD_SET_CHROME,
        _pad: [0; 3],
        bloom_surface_id,
        titlebar_height,
        frame_thickness,
    };
    let mut out = [0u8; 16];
    out.copy_from_slice(as_bytes!(msg, WCmdSetChrome));
    out
}

pub fn encode_set_title(bloom_surface_id: u32, title: &str) -> [u8; 72] {
    let mut title_bytes = [0u8; MAX_TITLE_BYTES];
    let mut title_len = title.len().min(MAX_TITLE_BYTES);
    while !title.is_char_boundary(title_len) {
        title_len -= 1;
    }
    title_bytes[..title_len].copy_from_slice(&title.as_bytes()[..title_len]);
    let msg = WCmdSetTitle {
        msg_type: WCMD_SET_TITLE,
        title_len: title_len as u8,
        _pad: [0; 2],
        bloom_surface_id,
        title: title_bytes,
    };
    let mut out = [0u8; 72];
    out.copy_from_slice(as_bytes!(msg, WCmdSetTitle));
    out
}

pub fn encode_set_subsurface(
    child_surface_id: u32,
    parent_surface_id: u32,
    x: i32,
    y: i32,
    z_above: i32,
) -> [u8; 24] {
    let msg = WCmdSetSubsurface {
        msg_type: WCMD_SET_SUBSURFACE,
        _pad: [0; 3],
        child_surface_id,
        parent_surface_id,
        x,
        y,
        z_above,
    };
    let mut out = [0u8; 24];
    out.copy_from_slice(as_bytes!(msg, WCmdSetSubsurface));
    out
}

#[allow(clippy::too_many_arguments)]
pub fn encode_set_layer_surface(
    bloom_surface_id: u32,
    layer: u32,
    anchor: u32,
    exclusive_zone: i32,
    margin_top: i32,
    margin_right: i32,
    margin_bottom: i32,
    margin_left: i32,
    width: u32,
    height: u32,
    active: u8,
) -> [u8; 44] {
    let msg = WCmdSetLayerSurface {
        msg_type: WCMD_SET_LAYER_SURFACE,
        active,
        _pad: [0; 2],
        bloom_surface_id,
        layer,
        anchor,
        exclusive_zone,
        margin_top,
        margin_right,
        margin_bottom,
        margin_left,
        width,
        height,
    };
    let mut out = [0u8; 44];
    out.copy_from_slice(as_bytes!(msg, WCmdSetLayerSurface));
    out
}

pub fn encode_buffer_release(wl_buf_key: u32) -> [u8; 8] {
    let msg = WEvtBufferRelease { msg_type: WEVT_BUFFER_RELEASE, _pad: [0; 3], wl_buf_key };
    let mut out = [0u8; 8];
    out.copy_from_slice(as_bytes!(msg, WEvtBufferRelease));
    out
}

pub fn encode_frame_done(bloom_surface_id: u32, timestamp_ms: u32) -> [u8; 12] {
    let msg =
        WEvtFrameDone { msg_type: WEVT_FRAME_DONE, _pad: [0; 3], bloom_surface_id, timestamp_ms };
    let mut out = [0u8; 12];
    out.copy_from_slice(as_bytes!(msg, WEvtFrameDone));
    out
}

pub fn encode_configure_surface(
    bloom_surface_id: u32,
    width: i32,
    height: i32,
    resizing: bool,
) -> [u8; 16] {
    let msg = WEvtConfigureSurface {
        msg_type: WEVT_CONFIGURE_SURFACE,
        resizing: resizing as u8,
        _pad: [0; 2],
        bloom_surface_id,
        width,
        height,
    };
    let mut out = [0u8; 16];
    out.copy_from_slice(as_bytes!(msg, WEvtConfigureSurface));
    out
}

pub fn encode_toplevel_action(
    bloom_surface_id: u32,
    action: u8,
    width: i32,
    height: i32,
) -> [u8; 16] {
    let msg = WEvtToplevelAction {
        msg_type: WEVT_TOPLEVEL_ACTION,
        action,
        _pad: [0; 2],
        bloom_surface_id,
        width,
        height,
    };
    let mut out = [0u8; 16];
    out.copy_from_slice(as_bytes!(msg, WEvtToplevelAction));
    out
}

pub fn encode_pointer_enter(bloom_surface_id: u32, x: i32, y: i32) -> [u8; 16] {
    let msg =
        WEvtPointerEnter { msg_type: WEVT_POINTER_ENTER, _pad: [0; 3], bloom_surface_id, x, y };
    let mut out = [0u8; 16];
    out.copy_from_slice(as_bytes!(msg, WEvtPointerEnter));
    out
}

pub fn encode_pointer_leave(bloom_surface_id: u32) -> [u8; 8] {
    let msg = WEvtPointerLeave { msg_type: WEVT_POINTER_LEAVE, _pad: [0; 3], bloom_surface_id };
    let mut out = [0u8; 8];
    out.copy_from_slice(as_bytes!(msg, WEvtPointerLeave));
    out
}

pub fn encode_pointer_motion(bloom_surface_id: u32, x: i32, y: i32, timestamp_ns: u64) -> [u8; 20] {
    let msg = WEvtPointerMotion {
        msg_type: WEVT_POINTER_MOTION,
        _pad: [0; 3],
        bloom_surface_id,
        x,
        y,
        timestamp_ms: (timestamp_ns / 1_000_000) as u32,
    };
    let mut out = [0u8; 20];
    out.copy_from_slice(as_bytes!(msg, WEvtPointerMotion));
    out
}

pub fn encode_pointer_button(
    bloom_surface_id: u32,
    button: u8,
    pressed: bool,
    timestamp_ns: u64,
) -> [u8; 12] {
    let msg = WEvtPointerButton {
        msg_type: WEVT_POINTER_BUTTON,
        button,
        pressed: pressed as u8,
        _pad: 0,
        bloom_surface_id,
        timestamp_ms: (timestamp_ns / 1_000_000) as u32,
    };
    let mut out = [0u8; 12];
    out.copy_from_slice(as_bytes!(msg, WEvtPointerButton));
    out
}

pub fn encode_pointer_scroll(
    bloom_surface_id: u32,
    dx: i16,
    dy: i16,
    timestamp_ns: u64,
) -> [u8; 16] {
    let msg = WEvtPointerScroll {
        msg_type: WEVT_POINTER_SCROLL,
        _pad: [0; 3],
        bloom_surface_id,
        dx,
        dy,
        timestamp_ms: (timestamp_ns / 1_000_000) as u32,
    };
    let mut out = [0u8; 16];
    out.copy_from_slice(as_bytes!(msg, WEvtPointerScroll));
    out
}

pub fn encode_keyboard_enter(bloom_surface_id: u32, modifiers: u8) -> [u8; 8] {
    let msg = WEvtKeyboardEnter {
        msg_type: WEVT_KEYBOARD_ENTER,
        modifiers,
        _pad: [0; 2],
        bloom_surface_id,
    };
    let mut out = [0u8; 8];
    out.copy_from_slice(as_bytes!(msg, WEvtKeyboardEnter));
    out
}

pub fn encode_keyboard_leave(bloom_surface_id: u32) -> [u8; 8] {
    let msg = WEvtKeyboardLeave { msg_type: WEVT_KEYBOARD_LEAVE, _pad: [0; 3], bloom_surface_id };
    let mut out = [0u8; 8];
    out.copy_from_slice(as_bytes!(msg, WEvtKeyboardLeave));
    out
}

pub fn encode_keyboard_key(
    bloom_surface_id: u32,
    key: u16,
    pressed: bool,
    modifiers: u8,
    repeat: bool,
    timestamp_ns: u64,
) -> [u8; 16] {
    let msg = WEvtKeyboardKey {
        msg_type: WEVT_KEYBOARD_KEY,
        pressed: pressed as u8,
        modifiers,
        repeat: repeat as u8,
        bloom_surface_id,
        key,
        _pad: [0; 2],
        timestamp_ms: (timestamp_ns / 1_000_000) as u32,
    };
    let mut out = [0u8; 16];
    out.copy_from_slice(as_bytes!(msg, WEvtKeyboardKey));
    out
}

pub fn encode_output_info(width: u32, height: u32, refresh_mhz: u32) -> [u8; 16] {
    let msg =
        WEvtOutputInfo { msg_type: WEVT_OUTPUT_INFO, _pad: [0; 3], width, height, refresh_mhz };
    let mut out = [0u8; 16];
    out.copy_from_slice(as_bytes!(msg, WEvtOutputInfo));
    out
}

pub fn encode_close_layer_surface(bloom_surface_id: u32) -> [u8; 8] {
    let msg = WEvtCloseLayerSurface {
        msg_type: WEVT_CLOSE_LAYER_SURFACE,
        _pad: [0; 3],
        bloom_surface_id,
    };
    let mut out = [0u8; 8];
    out.copy_from_slice(as_bytes!(msg, WEvtCloseLayerSurface));
    out
}

/// Encode a [`WCMD_SET_OPAQUE_REGION`] command.
///
/// Pass `rect = Some((x, y, w, h))` to set the opaque region, or `None` to
/// clear it (no opaque region on the surface).
pub fn encode_set_opaque_region(
    bloom_surface_id: u32,
    rect: Option<(u32, u32, u32, u32)>,
) -> [u8; 24] {
    let (has_region, x, y, w, h) = match rect {
        Some((x, y, w, h)) => (1u8, x, y, w, h),
        None => (0u8, 0, 0, 0, 0),
    };
    let msg = WCmdSetOpaqueRegion {
        msg_type: WCMD_SET_OPAQUE_REGION,
        has_region,
        _pad: [0; 2],
        bloom_surface_id,
        x,
        y,
        w,
        h,
    };
    let mut out = [0u8; 24];
    out.copy_from_slice(as_bytes!(msg, WCmdSetOpaqueRegion));
    out
}

/// Encode a [`WCMD_SET_INPUT_REGION`] command.
///
/// Pass `rect = Some((x, y, w, h))` to set the input region, or `None` to
/// clear it (the entire surface becomes the input region).
pub fn encode_set_input_region(
    bloom_surface_id: u32,
    rect: Option<(u32, u32, u32, u32)>,
) -> [u8; 24] {
    let (has_region, x, y, w, h) = match rect {
        Some((x, y, w, h)) => (1u8, x, y, w, h),
        None => (0u8, 0, 0, 0, 0),
    };
    let msg = WCmdSetInputRegion {
        msg_type: WCMD_SET_INPUT_REGION,
        has_region,
        _pad: [0; 2],
        bloom_surface_id,
        x,
        y,
        w,
        h,
    };
    let mut out = [0u8; 24];
    out.copy_from_slice(as_bytes!(msg, WCmdSetInputRegion));
    out
}
