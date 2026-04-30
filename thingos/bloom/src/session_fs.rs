//! Read-only-intent Wayland/session introspection files.
//!
//! Bloom owns these snapshots under `/session/wayland`.  They are ordinary
//! session files for now, so callers should treat them as diagnostics rather
//! than a control API.

use alloc::format;
use alloc::string::String;

use abi::syscall::vfs_flags::{O_CREAT, O_RDWR, O_TRUNC};
use stem::syscall::vfs::{vfs_close, vfs_mkdir, vfs_open, vfs_rmdir, vfs_unlink, vfs_write};

use crate::scene::{Scene, SurfaceSnapshot};

const ROOT: &str = "/session/wayland";
const SURFACES: &str = "/session/wayland/surfaces";
const WINDOWS: &str = "/session/wayland/windows";
const EVENTS: &str = "/session/wayland/events";

pub fn init() {
    let _ = vfs_mkdir("/session");
    let _ = vfs_mkdir(ROOT);
    let _ = vfs_mkdir(SURFACES);
    let _ = vfs_mkdir(WINDOWS);
    let _ = vfs_mkdir(EVENTS);
    write_file(
        "/session/wayland/components",
        b"wl_display\nwl_registry\nwl_compositor\nwl_shm\nxdg_wm_base\nxdg_surface\nxdg_toplevel\nxdg_popup\n",
    );
    write_file("/session/wayland/status", b"mode=snapshot\nwrite_support=false\n");
}

pub fn sync_scene(scene: &Scene, event: &str) {
    init();

    let snapshots = scene.surface_snapshots();
    write_file("/session/wayland/surfaces/index", surface_index(&snapshots).as_bytes());
    write_file("/session/wayland/windows/index", window_index(&snapshots).as_bytes());
    write_file("/session/wayland/events/latest", event.as_bytes());

    for surface in &snapshots {
        write_surface(surface);
        if surface.is_window {
            write_window(surface);
        } else {
            remove_window(surface.id);
        }
    }
}

pub fn remove_surface(surface_id: u32, event: &str) {
    remove_tree(SURFACES, surface_id, &["info", "geometry", "state", "title"]);
    remove_window(surface_id);
    write_file("/session/wayland/events/latest", event.as_bytes());
}

fn write_surface(surface: &SurfaceSnapshot) {
    let dir = format!("{}/{}", SURFACES, surface.id);
    let _ = vfs_mkdir(&dir);
    write_file(&format!("{}/info", dir), surface_info(surface).as_bytes());
    write_file(&format!("{}/geometry", dir), geometry(surface).as_bytes());
    write_file(&format!("{}/state", dir), state(surface).as_bytes());
    write_file(&format!("{}/title", dir), title(surface).as_bytes());
}

fn write_window(surface: &SurfaceSnapshot) {
    let dir = format!("{}/{}", WINDOWS, surface.id);
    let _ = vfs_mkdir(&dir);
    write_file(&format!("{}/info", dir), surface_info(surface).as_bytes());
    write_file(&format!("{}/geometry", dir), geometry(surface).as_bytes());
    write_file(&format!("{}/state", dir), state(surface).as_bytes());
    write_file(&format!("{}/title", dir), title(surface).as_bytes());
}

fn remove_window(surface_id: u32) {
    remove_tree(WINDOWS, surface_id, &["info", "geometry", "state", "title"]);
}

fn remove_tree(parent: &str, id: u32, files: &[&str]) {
    let dir = format!("{}/{}", parent, id);
    for file in files {
        let _ = vfs_unlink(&format!("{}/{}", dir, file));
    }
    let _ = vfs_rmdir(&dir);
}

fn write_file(path: &str, data: &[u8]) {
    if let Ok(fd) = vfs_open(path, O_CREAT | O_TRUNC | O_RDWR) {
        let _ = vfs_write(fd, data);
        let _ = vfs_close(fd);
    }
}

fn surface_index(surfaces: &[SurfaceSnapshot]) -> String {
    let mut out = String::new();
    for surface in surfaces {
        out.push_str(&format!(
            "{} client={} mapped={} visible={} window={} title=\"{}\"\n",
            surface.id,
            surface.client_id,
            bool_text(surface.mapped),
            bool_text(surface.visible),
            bool_text(surface.is_window),
            surface.title.as_deref().unwrap_or("")
        ));
    }
    out
}

fn window_index(surfaces: &[SurfaceSnapshot]) -> String {
    let mut out = String::new();
    for surface in surfaces.iter().filter(|surface| surface.is_window) {
        out.push_str(&format!(
            "{} {}x{}+{},{} z={} title=\"{}\"\n",
            surface.id,
            surface.rect.w,
            surface.rect.h,
            surface.rect.x,
            surface.rect.y,
            surface.z_order,
            surface.title.as_deref().unwrap_or("")
        ));
    }
    out
}

fn surface_info(surface: &SurfaceSnapshot) -> String {
    format!(
        "id={}\nclient_id={}\nbuffer_id={}\nframe_serial={}\ntype={}\n",
        surface.id,
        surface.client_id,
        surface.buffer_id.map(|id| format!("{}", id)).unwrap_or_else(|| String::from("none")),
        surface.frame_serial,
        if surface.is_window { "window" } else { "surface" }
    )
}

fn geometry(surface: &SurfaceSnapshot) -> String {
    format!(
        "x={}\ny={}\nwidth={}\nheight={}\nz_order={}\n",
        surface.rect.x, surface.rect.y, surface.rect.w, surface.rect.h, surface.z_order
    )
}

fn state(surface: &SurfaceSnapshot) -> String {
    format!(
        "mapped={}\nvisible={}\nfocus_eligible={}\nwindow={}\n",
        bool_text(surface.mapped),
        bool_text(surface.visible),
        bool_text(surface.focus_eligible),
        bool_text(surface.is_window)
    )
}

fn title(surface: &SurfaceSnapshot) -> String {
    let mut out = surface.title.clone().unwrap_or_default();
    out.push('\n');
    out
}

fn bool_text(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}
