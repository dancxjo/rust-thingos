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
//!
//! # Main → Wayland (events)
//!
//! | Discriminant         | Meaning                                       |
//! |----------------------|-----------------------------------------------|
//! | `WEVT_BUFFER_RELEASE`| The compositor no longer needs a buffer       |
//! | `WEVT_FRAME_DONE`    | A frame has been presented                    |
//! | `WEVT_CONFIGURE_SURFACE` | The compositor requests a toplevel size   |

// ── Discriminants ────────────────────────────────────────────────────────────

pub const WCMD_CREATE_SURFACE: u8 = 1;
pub const WCMD_DESTROY_SURFACE: u8 = 2;
pub const WCMD_IMPORT_ATTACH: u8 = 3;
pub const WCMD_DAMAGE: u8 = 4;
pub const WCMD_COMMIT: u8 = 5;
pub const WCMD_SET_CHROME: u8 = 6;
pub const WCMD_SET_TITLE: u8 = 7;
pub const MAX_TITLE_BYTES: usize = 64;

pub const WEVT_BUFFER_RELEASE: u8 = 1;
pub const WEVT_FRAME_DONE: u8 = 2;
pub const WEVT_CONFIGURE_SURFACE: u8 = 3;

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
    pub format: u32,
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
) -> [u8; 32] {
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
    };
    let mut out = [0u8; 32];
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
