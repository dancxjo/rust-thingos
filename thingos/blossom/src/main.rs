#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::convert::TryInto;

use libdl::{RTLD_NOW, dlopen_str, dlsym_bytes};
use stem::application::{
    AppAction, Application, ApplicationContext, ServiceLooper, run_application,
};
use stem::info;
use stem::service_loop::ServiceEvent;
use stem::syscall::socket::{connect, sendmsg, socket};
use stem::syscall::socket_domain::AF_UNIX;
use stem::syscall::socket_type::SOCK_STREAM;
use stem::syscall::vfs::{pipe, vfs_close, vfs_open, vfs_read, vfs_watch_path, vfs_write};
use stem::syscall::{memfd_create, sleep_ms, vm_map};
use stem::time::Duration;
use stem::wait_set::WaitToken;

const REGISTRY_ID: u32 = 2;
const COMPOSITOR_ID: u32 = 3;
const SEAT_ID: u32 = 4;
const LAYER_SHELL_ID: u32 = 5;
const KEYBOARD_ID: u32 = 6;
const SHM_ID: u32 = 7;
const SHORTCUT_SURFACE_ID: u32 = 10;
const SHORTCUT_LAYER_ID: u32 = 11;
const SHORTCUT_POOL_ID: u32 = 12;
const SHORTCUT_BUFFER_ID: u32 = 13;

// Wallpaper background surface objects
const WP_SURFACE_ID: u32 = 20;
const WP_LAYER_ID: u32 = 21;
// Dynamic pool/buffer object IDs are allocated starting here
const WP_FIRST_DYN_ID: u32 = 30;

const GLOBAL_WL_COMPOSITOR: u32 = 1;
const GLOBAL_WL_SHM: u32 = 2;
const GLOBAL_WL_SEAT: u32 = 4;
const GLOBAL_ZWLR_LAYER_SHELL: u32 = 10;

const MOD_ALT: u32 = 1 << 3;
const MOD_META: u32 = 1 << 6;
const EVDEV_R: u32 = 19;
const EVDEV_F2: u32 = 60;

// ── Wallpaper constants ───────────────────────────────────────────────────────

const WALLPAPER_CONFIG_PATH: &str = "/session/desktop/wallpaper";
const DEFAULT_WALLPAPER_PATH: &str = "/public/wallpapers/flower.png";
const PISTIL_PATH: &str = "/lib/libpistil.so";
const PREPARE_BG_SYMBOL: &[u8] = b"pistil_prepare_background\0";
const PISTIL_RETRY_DELAY_MS: u64 = 1500;
const PISTIL_MAX_RETRIES: u32 = 40;

/// Bytes per pixel for the wallpaper buffer (BGRA / ARGB 32-bit).
const BYTES_PER_PIXEL: u32 = 4;

type PrepareBackgroundFn = extern "C" fn(*const u8, *mut u32, u32, u32, u32) -> i32;

// ── Wallpaper state ───────────────────────────────────────────────────────────

struct WallpaperState {
    /// Read end of the decode-notification pipe (registered with the looper).
    decode_pipe_read: u32,
    /// Write end passed to decode threads.
    decode_pipe_write: u32,
    /// Loaded pistil function, or `None` if not yet available.
    pistil: Option<PrepareBackgroundFn>,
    /// Number of failed pistil load attempts.
    pistil_retries: u32,
    /// Output dimensions from the last layer-surface configure event.
    configured_w: u32,
    configured_h: u32,
    /// True after the first configure event has been ack'd.
    surface_configured: bool,
    /// True while a background decode thread is running.
    decode_pending: bool,
    /// Dimensions of the currently in-flight decode.
    decode_w: u32,
    decode_h: u32,
    /// The memfd backing the pixel buffer being decoded.
    decode_memfd: Option<u32>,
    /// Mapped address of the decode buffer (stored as usize to be Send).
    decode_mapped: usize,
    /// The memfd that was most recently committed to the compositor.
    committed_memfd: Option<u32>,
    /// Next dynamically-allocated Wayland object ID.
    next_dyn_id: u32,
    /// VFS watch FD for the wallpaper config file.
    watch_fd: Option<u32>,
    watch_token: Option<WaitToken>,
    /// Path currently showing (set after a successful commit).
    committed_path: Option<String>,
    /// Path that has been submitted for in-flight decode (set when decode starts).
    pending_path: Option<String>,
}

impl WallpaperState {
    fn new(decode_pipe_read: u32, decode_pipe_write: u32) -> Self {
        Self {
            decode_pipe_read,
            decode_pipe_write,
            pistil: None,
            pistil_retries: 0,
            configured_w: 0,
            configured_h: 0,
            surface_configured: false,
            decode_pending: false,
            decode_w: 0,
            decode_h: 0,
            decode_memfd: None,
            decode_mapped: 0,
            committed_memfd: None,
            next_dyn_id: WP_FIRST_DYN_ID,
            watch_fd: None,
            watch_token: None,
            committed_path: None,
            pending_path: None,
        }
    }

    fn alloc_id(&mut self) -> u32 {
        let id = self.next_dyn_id;
        self.next_dyn_id += 1;
        id
    }

    /// True while we should still keep a short retry timer running.
    fn needs_retry(&self) -> bool {
        !self.surface_configured
            || (self.pistil.is_none() && self.pistil_retries < PISTIL_MAX_RETRIES)
    }

    /// Attempt to load pistil.  Returns true if newly loaded.
    fn try_load_pistil(&mut self) -> bool {
        if self.pistil.is_some() {
            return false;
        }
        self.pistil_retries += 1;
        let handle = dlopen_str(PISTIL_PATH, RTLD_NOW);
        if handle.is_null() {
            return false;
        }
        let sym = dlsym_bytes(handle, PREPARE_BG_SYMBOL);
        if sym.is_null() {
            return false;
        }
        let f: PrepareBackgroundFn = unsafe { core::mem::transmute(sym) };
        self.pistil = Some(f);
        stem::debug!("Blossom: pistil background renderer loaded");
        true
    }

    /// Spawn a background thread to decode the wallpaper at `path` into a new
    /// memfd-backed pixel buffer sized `w × h × 4` bytes.
    /// Returns false if pistil is unavailable or the dimensions are zero.
    fn start_decode(&mut self, path: &str, w: u32, h: u32) -> bool {
        let Some(prepare_bg) = self.pistil else {
            return false;
        };
        if w == 0 || h == 0 {
            return false;
        }

        // Drain any stale completion byte from a previous cycle.
        let mut drain = [0u8; 4];
        let _ = vfs_read(self.decode_pipe_read, &mut drain);

        let total = (w * h * BYTES_PER_PIXEL) as usize;
        let memfd = match memfd_create("blossom.wallpaper", total) {
            Ok(fd) => fd,
            Err(e) => {
                stem::warn!("Blossom: wallpaper memfd failed: {:?}", e);
                return false;
            }
        };

        use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
        let req = VmMapReq {
            addr_hint: 0,
            len: total,
            prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
            flags: VmMapFlags::empty(),
            backing: VmBacking::File { thing: memfd, offset: 0 },
        };
        let resp = match vm_map(&req) {
            Ok(r) => r,
            Err(e) => {
                stem::warn!("Blossom: wallpaper vm_map failed: {:?}", e);
                let _ = vfs_close(memfd);
                return false;
            }
        };
        let mapped = resp.addr;

        // Build a null-terminated path buffer (Copy, so closure captures by value).
        let path_bytes = path.as_bytes();
        let path_len = path_bytes.len().min(255);
        let mut path_c = [0u8; 256];
        path_c[..path_len].copy_from_slice(&path_bytes[..path_len]);

        let pipe_write = self.decode_pipe_write;
        let handle = stem::thread::spawn_task(move || {
            let ptr = mapped as *mut u32;
            let result = prepare_bg(path_c.as_ptr(), ptr, w, h, w);
            let status: u8 = if result == 0 { 1 } else { 0 };
            let _ = vfs_write(pipe_write, &[status]);
        });
        match handle {
            Ok(h) => h.detach(),
            Err(e) => {
                stem::warn!("Blossom: failed to spawn decode thread: {:?}", e);
                let _ = vfs_close(memfd);
                return false;
            }
        }

        // Drop the previous decode memfd — the compositor has finished with it
        // by the time a new decode starts.
        if let Some(old) = self.decode_memfd.take() {
            let _ = vfs_close(old);
        }

        self.decode_memfd = Some(memfd);
        self.decode_mapped = mapped;
        self.decode_w = w;
        self.decode_h = h;
        self.decode_pending = true;
        true
    }

    /// Called when the decode-notification pipe fires.  Creates a new wl_shm
    /// buffer from the decoded memfd and commits it to the wallpaper surface.
    fn on_decode_done(&mut self, wayland_fd: u32, success: bool) -> bool {
        self.decode_pending = false;
        if !success {
            stem::warn!("Blossom: wallpaper decode failed; keeping previous background");
            return false;
        }
        let Some(memfd) = self.decode_memfd else {
            return false;
        };
        let w = self.decode_w;
        let h = self.decode_h;
        let stride = w * BYTES_PER_PIXEL;
        let total = w * h * BYTES_PER_PIXEL;

        let pool_id = self.alloc_id();
        let buffer_id = self.alloc_id();

        create_pool(wayland_fd, SHM_ID, pool_id, memfd, total);
        create_buffer(wayland_fd, pool_id, buffer_id, w, h, stride);
        attach_buffer(wayland_fd, WP_SURFACE_ID, buffer_id);
        damage_surface(wayland_fd, WP_SURFACE_ID, 0, 0, w, h);
        commit_surface(wayland_fd, WP_SURFACE_ID);

        // Replace the committed memfd.  The old one is closed here; the pixel
        // data remains valid because the kernel keeps the underlying anonymous
        // file alive as long as the mapping exists.
        if let Some(old) = self.committed_memfd.replace(memfd) {
            let _ = vfs_close(old);
        }
        self.decode_memfd = None;
        // Only record this as the committed path once the commit is actually sent.
        self.committed_path = self.pending_path.take();

        stem::info!("Blossom wallpaper ready: {}x{}", w, h);
        true
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    run_application::<BlossomApp>()
}

struct BlossomApp {
    fd: u32,
    wayland_token: WaitToken,
    decode_pipe_token: WaitToken,
    state: ShellState,
    wallpaper: WallpaperState,
}

struct ShellState {
    modifiers: u32,
    shortcut_configured: bool,
    shortcut_mapped: bool,
}

impl Application for BlossomApp {
    const NAME: &'static str = "blossom";

    fn init(
        ctx: &mut ApplicationContext,
        looper: &mut ServiceLooper,
    ) -> Result<Self, stem::errors::Errno> {
        let fd = connect_wayland();
        let wayland_token = looper.add_fd_readable(fd)?;
        send_get_registry(fd, REGISTRY_ID);
        read_initial_globals(fd);
        bind_global(fd, GLOBAL_WL_COMPOSITOR, "wl_compositor", 4, COMPOSITOR_ID);
        bind_global(fd, GLOBAL_WL_SHM, "wl_shm", 1, SHM_ID);
        bind_global(fd, GLOBAL_WL_SEAT, "wl_seat", 5, SEAT_ID);
        bind_global(fd, GLOBAL_ZWLR_LAYER_SHELL, "zwlr_layer_shell_v1", 4, LAYER_SHELL_ID);
        get_keyboard(fd, SEAT_ID, KEYBOARD_ID);
        register_shortcut_surface(fd);

        // Create the decode-notification pipe.  The read end is registered with
        // the looper so the decode thread's completion wakes the event loop.
        let mut pipe_fds = [0u32; 2];
        pipe(&mut pipe_fds)?;
        let decode_pipe_token = looper.add_fd_readable(pipe_fds[0])?;
        let mut wallpaper = WallpaperState::new(pipe_fds[0], pipe_fds[1]);

        // Watch the wallpaper config for live path changes.
        match vfs_watch_path(WALLPAPER_CONFIG_PATH, abi::vfs_watch::mask::ALL_EVENTS, 0) {
            Ok(watch_fd) => match looper.add_fd_readable(watch_fd) {
                Ok(tok) => {
                    wallpaper.watch_fd = Some(watch_fd);
                    wallpaper.watch_token = Some(tok);
                }
                Err(_) => {
                    let _ = vfs_close(watch_fd);
                }
            },
            Err(_) => {}
        }

        // Create the wallpaper background surface.  The compositor will send a
        // configure event with the output dimensions, which triggers the decode.
        setup_wallpaper_surface(fd);

        ctx.register_window("shortcut layer surface", move || close_shortcut_surface(fd));
        ctx.register_cleanup("wayland fd", move || {
            let _ = vfs_close(fd);
        });
        Ok(Self {
            fd,
            wayland_token,
            decode_pipe_token,
            state: ShellState { modifiers: 0, shortcut_configured: false, shortcut_mapped: false },
            wallpaper,
        })
    }

    fn ready(&mut self, _ctx: &mut ApplicationContext) {
        info!("Blossom shell connected to Wayland");
    }

    /// Keep a short retry timer running while pistil is not yet available or
    /// the wallpaper surface has not received its first configure event.
    fn timeout(&self) -> Option<Duration> {
        if self.wallpaper.needs_retry() {
            Some(Duration::from_millis(PISTIL_RETRY_DELAY_MS))
        } else {
            None
        }
    }

    fn handle_event(
        &mut self,
        _ctx: &mut ApplicationContext,
        event: ServiceEvent<'_>,
    ) -> AppAction {
        match event {
            // Wayland socket readable — dispatch protocol messages.
            ServiceEvent::Ready { token, event }
                if token == self.wayland_token && event.is_readable() =>
            {
                read_events(self.fd, &mut self.state, &mut self.wallpaper);
                AppAction::Continue
            }
            ServiceEvent::Ready { token, event }
                if token == self.wayland_token && (event.is_hangup() || event.is_error()) =>
            {
                AppAction::Quit
            }
            // Decode-notification pipe — background decode finished.
            ServiceEvent::Ready { token, event }
                if token == self.decode_pipe_token && event.is_readable() =>
            {
                let mut buf = [0u8; 1];
                let n = vfs_read(self.wallpaper.decode_pipe_read, &mut buf).unwrap_or(0);
                let success = n > 0 && buf[0] != 0;
                self.wallpaper.on_decode_done(self.fd, success);
                AppAction::Continue
            }
            // Wallpaper config changed — reload.
            ServiceEvent::Ready { token, .. } if Some(token) == self.wallpaper.watch_token => {
                let mut drain = [0u8; 128];
                if let Some(wfd) = self.wallpaper.watch_fd {
                    let _ = vfs_read(wfd, &mut drain);
                }
                self.trigger_wallpaper_reload();
                AppAction::Continue
            }
            // Retry timer — attempt to load pistil and/or kick off the decode.
            ServiceEvent::Timeout => {
                self.on_retry_timeout();
                AppAction::Continue
            }
            ServiceEvent::Message { .. } | ServiceEvent::Ready { .. } => AppAction::Continue,
            ServiceEvent::InboxClosed => AppAction::Quit,
        }
    }
}

impl BlossomApp {
    /// Try to load pistil and start the initial decode if all conditions are met.
    fn on_retry_timeout(&mut self) {
        if self.wallpaper.pistil.is_none() {
            self.wallpaper.try_load_pistil();
        }
        if self.wallpaper.pistil.is_some()
            && self.wallpaper.surface_configured
            && !self.wallpaper.decode_pending
            && self.wallpaper.committed_path.is_none()
            && self.wallpaper.pending_path.is_none()
        {
            self.trigger_wallpaper_reload();
        }
    }

    /// Read the current wallpaper target path and start a decode if it has
    /// changed or no wallpaper has been committed yet.
    fn trigger_wallpaper_reload(&mut self) {
        let path = read_wallpaper_target();
        // Skip if this path is already committed or already in-flight.
        if self.wallpaper.committed_path.as_deref() == Some(path.as_str()) {
            return;
        }
        if self.wallpaper.pending_path.as_deref() == Some(path.as_str()) {
            return;
        }
        let w = self.wallpaper.configured_w;
        let h = self.wallpaper.configured_h;
        if self.wallpaper.start_decode(&path, w, h) {
            stem::info!("Preparing wallpaper {}", path);
            self.wallpaper.pending_path = Some(path);
        }
    }
}

fn connect_wayland() -> u32 {
    loop {
        let fd = match socket(AF_UNIX, SOCK_STREAM, 0) {
            Ok(fd) => fd,
            Err(e) => {
                stem::warn!("blossom: socket(AF_UNIX) failed: {:?}", e);
                sleep_ms(250);
                continue;
            }
        };
        match connect(fd, "/run/wayland-0") {
            Ok(()) => return fd,
            Err(_) => {
                let _ = vfs_close(fd);
                sleep_ms(50);
            }
        }
    }
}

fn register_shortcut_surface(fd: u32) {
    create_surface(fd, COMPOSITOR_ID, SHORTCUT_SURFACE_ID);
    get_layer_surface(
        fd,
        LAYER_SHELL_ID,
        SHORTCUT_LAYER_ID,
        SHORTCUT_SURFACE_ID,
        3,
        "thingos.blossom.shortcuts",
    );
    set_layer_size(fd, SHORTCUT_LAYER_ID, 1, 1);
    set_layer_anchor(
        fd,
        SHORTCUT_LAYER_ID,
        blossom::layer_anchor::TOP | blossom::layer_anchor::LEFT,
    );
    set_layer_keyboard_interactivity(fd, SHORTCUT_LAYER_ID, 2);
    commit_surface(fd, SHORTCUT_SURFACE_ID);
}

fn close_shortcut_surface(fd: u32) {
    destroy_object(fd, SHORTCUT_LAYER_ID);
    destroy_object(fd, SHORTCUT_SURFACE_ID);
}

/// Create the full-screen background layer surface used by blossom to display
/// the wallpaper.  The compositor will send a configure event with the actual
/// output dimensions.
fn setup_wallpaper_surface(fd: u32) {
    create_surface(fd, COMPOSITOR_ID, WP_SURFACE_ID);
    get_layer_surface(fd, LAYER_SHELL_ID, WP_LAYER_ID, WP_SURFACE_ID, 0, "thingos.blossom.wallpaper");
    // Anchor to all four edges so the compositor fills the full output.
    set_layer_anchor(
        fd,
        WP_LAYER_ID,
        blossom::layer_anchor::TOP
            | blossom::layer_anchor::BOTTOM
            | blossom::layer_anchor::LEFT
            | blossom::layer_anchor::RIGHT,
    );
    // Size 0,0 means "compositor decides" — it will report real dimensions in configure.
    set_layer_size(fd, WP_LAYER_ID, 0, 0);
    // No keyboard interactivity for a background surface.
    set_layer_keyboard_interactivity(fd, WP_LAYER_ID, 0);
    commit_surface(fd, WP_SURFACE_ID);
}

/// Read the wallpaper image path from the config file, returning the default
/// if the file is absent or empty.
fn read_wallpaper_target() -> String {
    // O_RDONLY = 0; use the numeric value directly to avoid an abi import.
    let Ok(fd) = vfs_open(WALLPAPER_CONFIG_PATH, 0) else {
        return String::from(DEFAULT_WALLPAPER_PATH);
    };
    let mut buf = [0u8; 512];
    let n = vfs_read(fd, &mut buf).unwrap_or(0);
    let _ = vfs_close(fd);
    let s = core::str::from_utf8(&buf[..n]).unwrap_or("").trim();
    if s.is_empty() {
        String::from(DEFAULT_WALLPAPER_PATH)
    } else {
        String::from(s)
    }
}

fn read_events(fd: u32, state: &mut ShellState, wallpaper: &mut WallpaperState) {
    let mut buf = [0u8; 4096];
    let len = match vfs_read(fd, &mut buf) {
        Ok(n) if n > 0 => n,
        _ => return,
    };
    let mut offset = 0usize;
    while offset + 8 <= len {
        let (object_id, opcode, size) = decode_header(&buf[offset..len]);
        if size < 8 || offset + size as usize > len {
            break;
        }
        let payload = &buf[offset + 8..offset + size as usize];
        match (object_id, opcode) {
            // Wallpaper layer-surface configure: save dimensions and ack.
            (WP_LAYER_ID, 0) if payload.len() >= 12 => {
                let serial = read_u32(payload, 0);
                let w = read_u32(payload, 4);
                let h = read_u32(payload, 8);
                ack_layer_configure(fd, WP_LAYER_ID, serial);
                if w > 0 && h > 0 {
                    wallpaper.configured_w = w;
                    wallpaper.configured_h = h;
                    wallpaper.surface_configured = true;
                    stem::debug!("Blossom wallpaper surface configured: {}x{}", w, h);
                }
            }
            (SHORTCUT_LAYER_ID, 0) if payload.len() >= 12 => {
                let serial = read_u32(payload, 0);
                ack_layer_configure(fd, SHORTCUT_LAYER_ID, serial);
                state.shortcut_configured = true;
                if !state.shortcut_mapped {
                    map_shortcut_surface(fd);
                    state.shortcut_mapped = true;
                    info!("Blossom shortcut surface ready");
                }
            }
            (KEYBOARD_ID, 4) if payload.len() >= 20 => {
                state.modifiers = read_u32(payload, 4);
            }
            (KEYBOARD_ID, 3) if payload.len() >= 16 => {
                let key = read_u32(payload, 8);
                let pressed = read_u32(payload, 12) != 0;
                if pressed && key == EVDEV_R && (state.modifiers & MOD_META) != 0 {
                    info!("Blossom received runbox shortcut");
                }
                if pressed && key == EVDEV_F2 && (state.modifiers & MOD_ALT) != 0 {
                    info!("Blossom received runbox shortcut");
                }
            }
            _ => {}
        }
        offset += size as usize;
    }
}

fn map_shortcut_surface(fd: u32) {
    let memfd = memfd_create("blossom.shortcut", 4).expect("create shortcut memfd");
    use abi::vm::{VmBacking, VmMapReq, VmProt};
    let req = VmMapReq {
        addr_hint: 0,
        len: 4,
        prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
        flags: abi::vm::VmMapFlags::empty(),
        backing: VmBacking::File { thing: memfd, offset: 0 },
    };
    let resp = vm_map(&req).expect("map shortcut memfd");
    unsafe {
        *(resp.addr as *mut u32) = 0;
    }
    create_pool(fd, SHM_ID, SHORTCUT_POOL_ID, memfd, 4);
    create_buffer(fd, SHORTCUT_POOL_ID, SHORTCUT_BUFFER_ID, 1, 1, 4);
    attach_buffer(fd, SHORTCUT_SURFACE_ID, SHORTCUT_BUFFER_ID);
    damage_surface(fd, SHORTCUT_SURFACE_ID, 0, 0, 1, 1);
    commit_surface(fd, SHORTCUT_SURFACE_ID);
}

fn send_get_registry(fd: u32, new_id: u32) {
    let mut buf = Vec::new();
    encode_header(1, 1, 12, &mut buf);
    buf.extend_from_slice(&new_id.to_ne_bytes());
    send_request(fd, &buf);
}

fn bind_global(fd: u32, name: u32, interface: &str, version: u32, new_id: u32) {
    let mut buf = Vec::new();
    let len = interface.len() as u32 + 1;
    let mut bytes = interface.as_bytes().to_vec();
    bytes.push(0);
    while bytes.len() % 4 != 0 {
        bytes.push(0);
    }
    let size = 8 + 4 + 4 + bytes.len() as u16 + 4 + 4;
    encode_header(REGISTRY_ID, 0, size, &mut buf);
    buf.extend_from_slice(&name.to_ne_bytes());
    buf.extend_from_slice(&len.to_ne_bytes());
    buf.extend_from_slice(&bytes);
    buf.extend_from_slice(&version.to_ne_bytes());
    buf.extend_from_slice(&new_id.to_ne_bytes());
    send_request(fd, &buf);
}

fn create_surface(fd: u32, compositor_id: u32, new_id: u32) {
    let mut buf = Vec::new();
    encode_header(compositor_id, 0, 12, &mut buf);
    buf.extend_from_slice(&new_id.to_ne_bytes());
    send_request(fd, &buf);
}

fn get_keyboard(fd: u32, seat_id: u32, new_id: u32) {
    let mut buf = Vec::new();
    encode_header(seat_id, 1, 12, &mut buf);
    buf.extend_from_slice(&new_id.to_ne_bytes());
    send_request(fd, &buf);
}

fn get_layer_surface(
    fd: u32,
    layer_shell_id: u32,
    new_id: u32,
    surface_id: u32,
    layer: u32,
    namespace: &str,
) {
    let mut buf = Vec::new();
    let mut bytes = namespace.as_bytes().to_vec();
    bytes.push(0);
    let len = bytes.len() as u32;
    while bytes.len() % 4 != 0 {
        bytes.push(0);
    }
    let size = 8 + 4 + 4 + 4 + 4 + 4 + bytes.len() as u16;
    encode_header(layer_shell_id, 0, size, &mut buf);
    buf.extend_from_slice(&new_id.to_ne_bytes());
    buf.extend_from_slice(&surface_id.to_ne_bytes());
    buf.extend_from_slice(&0u32.to_ne_bytes());
    buf.extend_from_slice(&layer.to_ne_bytes());
    buf.extend_from_slice(&len.to_ne_bytes());
    buf.extend_from_slice(&bytes);
    send_request(fd, &buf);
}

fn set_layer_size(fd: u32, layer_surface_id: u32, width: u32, height: u32) {
    let mut buf = Vec::new();
    encode_header(layer_surface_id, 0, 16, &mut buf);
    buf.extend_from_slice(&width.to_ne_bytes());
    buf.extend_from_slice(&height.to_ne_bytes());
    send_request(fd, &buf);
}

fn set_layer_anchor(fd: u32, layer_surface_id: u32, anchor: u32) {
    let mut buf = Vec::new();
    encode_header(layer_surface_id, 1, 12, &mut buf);
    buf.extend_from_slice(&anchor.to_ne_bytes());
    send_request(fd, &buf);
}

fn set_layer_keyboard_interactivity(fd: u32, layer_surface_id: u32, mode: u32) {
    let mut buf = Vec::new();
    encode_header(layer_surface_id, 4, 12, &mut buf);
    buf.extend_from_slice(&mode.to_ne_bytes());
    send_request(fd, &buf);
}

fn ack_layer_configure(fd: u32, layer_surface_id: u32, serial: u32) {
    let mut buf = Vec::new();
    encode_header(layer_surface_id, 6, 12, &mut buf);
    buf.extend_from_slice(&serial.to_ne_bytes());
    send_request(fd, &buf);
}

fn create_pool(fd: u32, shm_id: u32, pool_id: u32, memfd: u32, size: u32) {
    let mut buf = Vec::new();
    encode_header(shm_id, 0, 16, &mut buf);
    buf.extend_from_slice(&pool_id.to_ne_bytes());
    buf.extend_from_slice(&size.to_ne_bytes());
    if let Err(e) = sendmsg(fd, &buf, &[memfd]) {
        stem::warn!("blossom: sendmsg failed: {:?}", e);
    }
}

fn create_buffer(fd: u32, pool_id: u32, buffer_id: u32, width: u32, height: u32, stride: u32) {
    let mut buf = Vec::new();
    encode_header(pool_id, 0, 32, &mut buf);
    buf.extend_from_slice(&buffer_id.to_ne_bytes());
    buf.extend_from_slice(&0u32.to_ne_bytes());
    buf.extend_from_slice(&width.to_ne_bytes());
    buf.extend_from_slice(&height.to_ne_bytes());
    buf.extend_from_slice(&stride.to_ne_bytes());
    buf.extend_from_slice(&0u32.to_ne_bytes());
    send_request(fd, &buf);
}

fn attach_buffer(fd: u32, surface_id: u32, buffer_id: u32) {
    let mut buf = Vec::new();
    encode_header(surface_id, 1, 20, &mut buf);
    buf.extend_from_slice(&buffer_id.to_ne_bytes());
    buf.extend_from_slice(&0u32.to_ne_bytes());
    buf.extend_from_slice(&0u32.to_ne_bytes());
    send_request(fd, &buf);
}

fn damage_surface(fd: u32, surface_id: u32, x: i32, y: i32, width: u32, height: u32) {
    let mut buf = Vec::new();
    encode_header(surface_id, 2, 24, &mut buf);
    buf.extend_from_slice(&x.to_ne_bytes());
    buf.extend_from_slice(&y.to_ne_bytes());
    buf.extend_from_slice(&(width as i32).to_ne_bytes());
    buf.extend_from_slice(&(height as i32).to_ne_bytes());
    send_request(fd, &buf);
}

fn commit_surface(fd: u32, surface_id: u32) {
    let mut buf = Vec::new();
    encode_header(surface_id, 6, 8, &mut buf);
    send_request(fd, &buf);
}

fn destroy_object(fd: u32, object_id: u32) {
    let mut buf = Vec::new();
    encode_header(object_id, 0, 8, &mut buf);
    send_request(fd, &buf);
}

fn read_initial_globals(fd: u32) {
    let mut buf = [0u8; 512];
    let _ = vfs_read(fd, &mut buf);
}

fn send_request(fd: u32, buf: &[u8]) {
    if let Err(e) = vfs_write(fd, buf) {
        stem::warn!("blossom: write failed: {:?}", e);
    }
}

fn encode_header(object_id: u32, opcode: u16, size: u16, buf: &mut Vec<u8>) {
    buf.extend_from_slice(&object_id.to_ne_bytes());
    buf.extend_from_slice(&(((size as u32) << 16) | opcode as u32).to_ne_bytes());
}

fn decode_header(buf: &[u8]) -> (u32, u16, u16) {
    let object_id = read_u32(buf, 0);
    let size_op = read_u32(buf, 4);
    (object_id, (size_op & 0xFFFF) as u16, (size_op >> 16) as u16)
}

fn read_u32(buf: &[u8], offset: usize) -> u32 {
    u32::from_ne_bytes(buf[offset..offset + 4].try_into().unwrap())
}
