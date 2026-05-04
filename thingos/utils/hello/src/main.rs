#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
use core::default::Default;
extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

use libdl::{RTLD_NOW, dlerror, dlopen_str, dlsym_bytes};
use petals::{
    AlignItems, AttrValue, Description, JustifyContent, NodeId, PetalsEvent, PetalsShowcase,
    ShowcaseComponent, ShowcaseServices, State, UiTree,
};
use pistil_types::Texture;
use stem::application::{
    AppAction, Application, ApplicationContext, ServiceLooper, WindowToken, run_application,
};
use stem::info;
use stem::service_loop::ServiceEvent;
use stem::syscall::socket::{connect, sendmsg, socket};
use stem::syscall::socket_domain::AF_UNIX;
use stem::syscall::socket_type::SOCK_STREAM;
use stem::syscall::{sleep_ms, vfs_close, vfs_read, vfs_write};
use stem::time::Duration;
use stem::wait_set::WaitToken;

const REGISTRY_ID: u32 = 2;
const COMPOSITOR_ID: u32 = 3;
const SHM_ID: u32 = 4;
const WM_BASE_ID: u32 = 5;
const SEAT_ID: u32 = 6;
const POINTER_ID: u32 = 7;
const KEYBOARD_ID: u32 = 8;
const DMABUF_ID: u32 = 9;

const TOP_SURFACE_ID: u32 = 10;
const TOP_XDG_SURFACE_ID: u32 = 11;
const TOPLEVEL_ID: u32 = 12;

const POPUP_SURFACE_ID: u32 = 20;
const POPUP_XDG_SURFACE_ID: u32 = 21;
const POSITIONER_ID: u32 = 22;
const POPUP_ID: u32 = 23;

const PRESENTATION_ID: u32 = 30;

/// wl_region object ID used in the startup smoke test.
const REGION_ID: u32 = 40;

const DRM_FORMAT_ARGB8888: u32 = 0x3432_5241; // "AR24"

const PISTIL_PATH: &str = "/lib/libpistil.so";
const DRAW_TEXT_SYMBOL: &[u8] = b"pistil_draw_text";
const DEFAULT_FONT_PATH: &str = "/public/fonts/Inter-Regular.ttf";

type DrawTextFn = extern "C" fn(*const u8, *mut u32, u32, u32, u32, i32, i32, f32, u32) -> i32;

static POINTER_MOTION_LOGS: AtomicU32 = AtomicU32::new(0);
static SHOWCASE_RENDER_LOGS: AtomicU32 = AtomicU32::new(0);

struct TextRenderer {
    _handle: *mut core::ffi::c_void,
    draw_text: DrawTextFn,
}

struct BufferState {
    buffer_id: u32,
    texture: Texture,
    width: u32,
    height: u32,
}

#[derive(Clone, Copy, Default)]
struct InitialGlobals {
    dmabuf_name: Option<u32>,
    presentation_name: Option<u32>,
}

struct PendingSurface {
    serial: Option<u32>,
    width: u32,
    height: u32,
    dirty: bool,
}

#[stem::main]
fn main(_arg: usize) -> ! {
    run_application::<WaylandHelloApp>()
}

struct WaylandHelloApp {
    fd: u32,
    wayland_token: WaitToken,
    text_renderer: Option<TextRenderer>,
    showcase: PetalsShowcase,
    showcase_services: ShowcaseServices,
    dmabuf_id: Option<u32>,
    presentation_id: Option<u32>,
    top_pending: PendingSurface,
    popup_pending: PendingSurface,
    top_buffer: Option<BufferState>,
    popup_buffer: Option<BufferState>,
    popup_created: bool,
    popup_window: Option<WindowToken>,
    next_callback_id: u32,
    pending_frame_callbacks: Vec<u32>,
    pending_presentation_feedbacks: Vec<u32>,
}

impl Application for WaylandHelloApp {
    const NAME: &'static str = "hello";

    fn init(
        ctx: &mut ApplicationContext,
        looper: &mut ServiceLooper,
    ) -> Result<Self, stem::errors::Errno> {
        let fd = connect_wayland();
        let text_renderer = load_text_renderer();
        let wayland_token = looper.add_fd_readable(fd)?;

        let showcase = PetalsShowcase::default();
        let showcase_services = ShowcaseServices::default();

        send_get_registry(fd, REGISTRY_ID);
        let globals = read_initial_globals(fd);

        bind_global(fd, 1, "wl_compositor", 4, COMPOSITOR_ID);
        bind_global(fd, 2, "wl_shm", 1, SHM_ID);
        bind_global(fd, 3, "xdg_wm_base", 1, WM_BASE_ID);
        bind_global(fd, 4, "wl_seat", 5, SEAT_ID);
        let dmabuf_id = if let Some(name) = globals.dmabuf_name {
            bind_global(fd, name, "zwp_linux_dmabuf_v1", 3, DMABUF_ID);
            info!("hello: using zwp_linux_dmabuf_v1 buffers");
            Some(DMABUF_ID)
        } else {
            info!("hello: zwp_linux_dmabuf_v1 unavailable; using wl_shm buffers");
            None
        };
        let presentation_id = if let Some(name) = globals.presentation_name {
            bind_global(fd, name, "wp_presentation", 1, PRESENTATION_ID);
            info!("hello: bound wp_presentation");
            Some(PRESENTATION_ID)
        } else {
            info!("hello: wp_presentation unavailable");
            None
        };
        seat_get_pointer(fd, SEAT_ID, POINTER_ID);
        seat_get_keyboard(fd, SEAT_ID, KEYBOARD_ID);

        create_surface(fd, COMPOSITOR_ID, TOP_SURFACE_ID);

        // wl_region smoke test: exercise the full region lifecycle before commit.
        // This verifies that wl_compositor.create_region produces a live object
        // (not a tombstone) and that wl_region.add / set_opaque_region / destroy
        // are accepted by the compositor without triggering a protocol error.
        create_region(fd, COMPOSITOR_ID, REGION_ID);
        region_add(fd, REGION_ID, 0, 0, 480, 320);
        set_opaque_region(fd, TOP_SURFACE_ID, REGION_ID);
        region_destroy(fd, REGION_ID);
        info!("hello: wl_region smoke test: create+add+set_opaque+destroy");

        // input_region smoke test: verify set_input_region lifecycle.
        // Creates a region, adds a sub-rect, sets it as the input region, then
        // destroys the region object.  The compositor resolves the region at
        // commit time and applies it to pointer hit-testing.
        create_region(fd, COMPOSITOR_ID, REGION_ID);
        region_add(fd, REGION_ID, 0, 0, 480, 320);
        set_input_region(fd, TOP_SURFACE_ID, REGION_ID);
        region_destroy(fd, REGION_ID);
        info!("hello: wl_region smoke test: create+add+set_input+destroy");

        get_xdg_surface(fd, WM_BASE_ID, TOP_XDG_SURFACE_ID, TOP_SURFACE_ID);
        get_toplevel(fd, TOP_XDG_SURFACE_ID, TOPLEVEL_ID);
        set_toplevel_title(fd, TOPLEVEL_ID, "Thing-OS Wayland Lab");
        set_toplevel_app_id(fd, TOPLEVEL_ID, "thingos.hello");
        commit_surface(fd, TOP_SURFACE_ID);

        ctx.register_window("wayland toplevel", move || close_toplevel_objects(fd));
        ctx.register_cleanup("wayland fd", move || {
            let _ = vfs_close(fd);
        });

        Ok(Self {
            fd,
            wayland_token,
            text_renderer,
            showcase,
            showcase_services,
            dmabuf_id,
            presentation_id,
            top_pending: PendingSurface { serial: None, width: 760, height: 560, dirty: false },
            popup_pending: PendingSurface { serial: None, width: 260, height: 150, dirty: false },
            top_buffer: None,
            popup_buffer: None,
            popup_created: false,
            popup_window: None,
            next_callback_id: 1000,
            pending_frame_callbacks: Vec::new(),
            pending_presentation_feedbacks: Vec::new(),
        })
    }

    fn ready(&mut self, _ctx: &mut ApplicationContext) {
        info!("hello: service loop started");
    }

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(16))
    }

    fn handle_event(&mut self, ctx: &mut ApplicationContext, event: ServiceEvent<'_>) -> AppAction {
        match event {
            ServiceEvent::Ready { token, event }
                if token == self.wayland_token && event.is_readable() =>
            {
                self.read_wayland(ctx)
            }
            ServiceEvent::Ready { token, event }
                if token == self.wayland_token && (event.is_hangup() || event.is_error()) =>
            {
                AppAction::Quit
            }
            ServiceEvent::Message { .. } | ServiceEvent::Timeout | ServiceEvent::Ready { .. } => {
                AppAction::Continue
            }
            ServiceEvent::InboxClosed => AppAction::Quit,
        }
    }
}

impl WaylandHelloApp {
    fn read_wayland(&mut self, ctx: &mut ApplicationContext) -> AppAction {
        let mut in_buf = [0u8; 4096];
        let len = match vfs_read(self.fd, &mut in_buf) {
            Ok(n) if n > 0 => n,
            _ => return AppAction::Continue,
        };

        let mut offset = 0usize;
        while offset + 8 <= len {
            let (object_id, opcode, size) = decode_header(&in_buf[offset..len]);
            if size < 8 || offset + size as usize > len {
                break;
            }
            let payload = &in_buf[offset + 8..offset + size as usize];
            match (object_id, opcode) {
                (WM_BASE_ID, 0) if payload.len() >= 4 => {
                    send_pong(self.fd, WM_BASE_ID, read_u32(payload, 0));
                }
                (TOP_XDG_SURFACE_ID, 0) if payload.len() >= 4 => {
                    self.top_pending.serial = Some(read_u32(payload, 0));
                    self.top_pending.dirty = true;
                }
                (TOPLEVEL_ID, 0) if payload.len() >= 12 => {
                    let width = read_i32(payload, 0);
                    let height = read_i32(payload, 4);
                    if width > 0 {
                        self.top_pending.width = width as u32;
                    }
                    if height > 0 {
                        self.top_pending.height = height as u32;
                    }
                }
                (TOPLEVEL_ID, 1) => {
                    info!("hello: compositor requested close; exiting");
                    vfs_write(1, b"hello: explicit exit(0) call\n").ok();
                    return AppAction::Quit;
                }
                (POINTER_ID, 0) if payload.len() >= 16 => {
                    info!(
                        "hello: pointer enter surface={} x={} y={}",
                        read_u32(payload, 4),
                        wl_fixed_to_i32(read_i32(payload, 8)),
                        wl_fixed_to_i32(read_i32(payload, 12))
                    );
                }
                (POINTER_ID, 1) if payload.len() >= 8 => {
                    info!("hello: pointer leave surface={}", read_u32(payload, 4));
                }
                (POINTER_ID, 2) if payload.len() >= 12 => {
                    if POINTER_MOTION_LOGS.fetch_add(1, Ordering::Relaxed) < 4 {
                        info!(
                            "hello: pointer motion x={} y={}",
                            wl_fixed_to_i32(read_i32(payload, 4)),
                            wl_fixed_to_i32(read_i32(payload, 8))
                        );
                    }
                }
                (POINTER_ID, 3) if payload.len() >= 16 => {
                    info!(
                        "hello: pointer button button={} state={}",
                        read_u32(payload, 8),
                        read_u32(payload, 12)
                    );
                }
                (KEYBOARD_ID, 1) if payload.len() >= 8 => {
                    info!("hello: keyboard enter surface={}", read_u32(payload, 4));
                }
                (KEYBOARD_ID, 2) if payload.len() >= 8 => {
                    info!("hello: keyboard leave surface={}", read_u32(payload, 4));
                }
                (KEYBOARD_ID, 3) if payload.len() >= 16 => {
                    info!(
                        "hello: keyboard key key={} state={}",
                        read_u32(payload, 8),
                        read_u32(payload, 12)
                    );
                }
                (KEYBOARD_ID, 4) if payload.len() >= 20 => {
                    info!("hello: keyboard modifiers depressed={}", read_u32(payload, 4));
                }
                (POPUP_XDG_SURFACE_ID, 0) if payload.len() >= 4 => {
                    self.popup_pending.serial = Some(read_u32(payload, 0));
                    self.popup_pending.dirty = true;
                }
                (POPUP_ID, 0) if payload.len() >= 16 => {
                    let width = read_i32(payload, 8);
                    let height = read_i32(payload, 12);
                    if width > 0 {
                        self.popup_pending.width = width as u32;
                    }
                    if height > 0 {
                        self.popup_pending.height = height as u32;
                    }
                }
                (POPUP_ID, 1) => {
                    info!("hello: compositor dismissed popup (popup_done)");
                    self.close_popup(ctx);
                }
                (_, 0) if self.pending_frame_callbacks.iter().any(|&id| id == object_id) => {
                    let callback_data = if payload.len() >= 4 { read_u32(payload, 0) } else { 0 };
                    info!("hello: frame callback done object={} data={}", object_id, callback_data);
                    self.pending_frame_callbacks.retain(|&id| id != object_id);
                }
                (_, 1) if self.pending_presentation_feedbacks.iter().any(|&id| id == object_id) => {
                    // wp_presentation_feedback.presented(tv_sec_hi, tv_sec_lo,
                    //   tv_nsec, refresh, seq_hi, seq_lo, flags)
                    let tv_sec_hi = if payload.len() >= 4 { read_u32(payload, 0) } else { 0 };
                    let tv_sec_lo = if payload.len() >= 8 { read_u32(payload, 4) } else { 0 };
                    let tv_nsec = if payload.len() >= 12 { read_u32(payload, 8) } else { 0 };
                    let refresh = if payload.len() >= 16 { read_u32(payload, 12) } else { 0 };
                    let seq_lo = if payload.len() >= 24 { read_u32(payload, 20) } else { 0 };
                    info!(
                        "hello: wp_presentation_feedback.presented object={} tv={}.{:09} refresh_ns={} seq={}",
                        object_id,
                        ((tv_sec_hi as u64) << 32) | tv_sec_lo as u64,
                        tv_nsec,
                        refresh,
                        seq_lo
                    );
                    self.pending_presentation_feedbacks.retain(|&id| id != object_id);
                }
                (_, 2) if self.pending_presentation_feedbacks.iter().any(|&id| id == object_id) => {
                    info!("hello: wp_presentation_feedback.discarded object={}", object_id);
                    self.pending_presentation_feedbacks.retain(|&id| id != object_id);
                }
                _ => {}
            }
            offset += size as usize;
        }

        self.flush_pending(ctx);
        AppAction::Continue
    }

    fn flush_pending(&mut self, ctx: &mut ApplicationContext) {
        if self.top_pending.dirty {
            self.render_top(ctx);
        }

        if self.popup_created && self.popup_pending.dirty {
            self.render_popup();
        }
    }

    fn render_top(&mut self, ctx: &mut ApplicationContext) {
        let title = "Thing-OS Wayland";
        let buffer = ensure_buffer(
            self.fd,
            SHM_ID,
            self.dmabuf_id,
            &mut self.top_buffer,
            TOP_SURFACE_ID + 100,
            self.top_pending.width,
            self.top_pending.height,
        );
        render_window(
            buffer,
            title,
            &self.showcase,
            &mut self.showcase_services,
            self.text_renderer.as_ref(),
        );
        ack_configure(self.fd, TOP_XDG_SURFACE_ID, self.top_pending.serial.unwrap_or(0));
        attach_buffer(self.fd, TOP_SURFACE_ID, buffer.buffer_id);
        damage_surface(
            self.fd,
            TOP_SURFACE_ID,
            0,
            0,
            self.top_pending.width,
            self.top_pending.height,
        );
        let cb_id = alloc_callback_id(&mut self.next_callback_id);
        request_frame(self.fd, TOP_SURFACE_ID, cb_id);
        self.pending_frame_callbacks.push(cb_id);
        if let Some(pid) = self.presentation_id {
            let fb_id = alloc_callback_id(&mut self.next_callback_id);
            request_presentation_feedback(self.fd, pid, TOP_SURFACE_ID, fb_id);
            self.pending_presentation_feedbacks.push(fb_id);
        }
        commit_surface(self.fd, TOP_SURFACE_ID);
        self.top_pending.dirty = false;

        if !self.popup_created {
            self.create_popup(ctx);
        }
    }

    fn create_popup(&mut self, ctx: &mut ApplicationContext) {
        self.popup_created = true;
        create_surface(self.fd, COMPOSITOR_ID, POPUP_SURFACE_ID);
        get_xdg_surface(self.fd, WM_BASE_ID, POPUP_XDG_SURFACE_ID, POPUP_SURFACE_ID);
        create_positioner(self.fd, WM_BASE_ID, POSITIONER_ID);
        positioner_set_size(self.fd, POSITIONER_ID, 260, 150);
        positioner_set_anchor_rect(self.fd, POSITIONER_ID, 24, 24, 100, 24);
        positioner_set_offset(self.fd, POSITIONER_ID, 0, 6);
        get_popup(self.fd, TOP_XDG_SURFACE_ID, POPUP_XDG_SURFACE_ID, POPUP_ID, POSITIONER_ID);
        commit_surface(self.fd, POPUP_SURFACE_ID);
        let fd = self.fd;
        self.popup_window =
            Some(ctx.register_window("wayland popup", move || close_popup_objects(fd)));
    }

    fn render_popup(&mut self) {
        let buffer = ensure_buffer(
            self.fd,
            SHM_ID,
            self.dmabuf_id,
            &mut self.popup_buffer,
            POPUP_SURFACE_ID + 100,
            self.popup_pending.width,
            self.popup_pending.height,
        );
        render_popup(
            buffer,
            "Petals services",
            &self.showcase,
            &mut self.showcase_services,
            self.text_renderer.as_ref(),
        );
        ack_configure(self.fd, POPUP_XDG_SURFACE_ID, self.popup_pending.serial.unwrap_or(0));
        attach_buffer(self.fd, POPUP_SURFACE_ID, buffer.buffer_id);
        damage_surface(
            self.fd,
            POPUP_SURFACE_ID,
            0,
            0,
            self.popup_pending.width,
            self.popup_pending.height,
        );
        let cb_id = alloc_callback_id(&mut self.next_callback_id);
        request_frame(self.fd, POPUP_SURFACE_ID, cb_id);
        self.pending_frame_callbacks.push(cb_id);
        if let Some(pid) = self.presentation_id {
            let fb_id = alloc_callback_id(&mut self.next_callback_id);
            request_presentation_feedback(self.fd, pid, POPUP_SURFACE_ID, fb_id);
            self.pending_presentation_feedbacks.push(fb_id);
        }
        commit_surface(self.fd, POPUP_SURFACE_ID);
        self.popup_pending.dirty = false;
    }

    fn close_popup(&mut self, ctx: &mut ApplicationContext) {
        if let Some(token) = self.popup_window.take() {
            ctx.close_window(token);
        }
        self.popup_created = false;
        self.popup_pending.serial = None;
        self.popup_pending.dirty = false;
        self.popup_buffer = None;
    }
}

fn connect_wayland() -> u32 {
    loop {
        let fd = match socket(AF_UNIX, SOCK_STREAM, 0) {
            Ok(fd) => fd,
            Err(e) => {
                stem::error!("hello: socket(AF_UNIX) failed: {:?}", e);
                sleep_ms(250);
                continue;
            }
        };

        match connect(fd, "/run/wayland-0") {
            Ok(()) => {
                info!("hello: connected to /run/wayland-0");
                return fd;
            }
            Err(_) => {
                let _ = vfs_close(fd);
                sleep_ms(50);
            }
        }
    }
}

fn read_initial_globals(fd: u32) -> InitialGlobals {
    let mut globals = InitialGlobals::default();
    let mut buf = [0u8; 512];
    let mut rx: Vec<u8> = Vec::new();

    for _ in 0..32 {
        match vfs_read(fd, &mut buf) {
            Ok(n) if n > 0 => {
                rx.extend_from_slice(&buf[..n]);
                let mut offset = 0usize;
                while offset + 8 <= rx.len() {
                    let (object_id, opcode, size) = decode_header(&rx[offset..]);
                    if size < 8 || offset + size as usize > rx.len() {
                        break;
                    }
                    let payload = &rx[offset + 8..offset + size as usize];
                    if object_id == REGISTRY_ID && opcode == 0 && payload.len() >= 12 {
                        let name = read_u32(payload, 0);
                        if let Some((iface, consumed)) = read_wayland_string(payload, 4) {
                            let version_off = 4 + consumed;
                            if version_off + 4 <= payload.len() {
                                let ver = read_u32(payload, version_off);
                                if iface == b"zwp_linux_dmabuf_v1" && ver >= 3 {
                                    globals.dmabuf_name = Some(name);
                                } else if iface == b"wp_presentation" && ver >= 1 {
                                    globals.presentation_name = Some(name);
                                }
                            }
                        }
                    }
                    offset += size as usize;
                }
                if offset > 0 {
                    rx.drain(..offset);
                }
                if globals.dmabuf_name.is_some() && globals.presentation_name.is_some() {
                    break;
                }
            }
            _ => {
                sleep_ms(5);
            }
        }
    }
    globals
}

fn ensure_buffer(
    fd: u32,
    shm_id: u32,
    dmabuf_id: Option<u32>,
    current: &mut Option<BufferState>,
    base_id: u32,
    width: u32,
    height: u32,
) -> &mut BufferState {
    let reusable = match current.as_ref() {
        Some(buf) => buf.width == width && buf.height == height,
        None => false,
    };
    if reusable {
        return current.as_mut().unwrap();
    }

    let texture = Texture::new("wl.buffer", width, height, 4).expect("create wl texture");

    let pool_id = base_id;
    let buffer_id = base_id + 1;
    if let Some(dmabuf) = dmabuf_id {
        create_dmabuf_buffer(
            fd,
            dmabuf,
            pool_id,
            buffer_id,
            texture.fd,
            width,
            height,
            texture.stride,
        );
    } else {
        create_pool(fd, shm_id, pool_id, texture.fd, texture.size as u32);
        create_buffer(fd, pool_id, buffer_id, width, height, texture.stride);
    }

    *current = Some(BufferState { buffer_id, width, height, texture });
    current.as_mut().unwrap()
}

fn render_window(
    buffer: &mut BufferState,
    title: &str,
    showcase: &PetalsShowcase,
    services: &mut ShowcaseServices,
    text_renderer: Option<&TextRenderer>,
) {
    let width = buffer.width;
    let height = buffer.height;
    let pixels = buffer.texture.as_slice_mut();
    pixels.fill(showcase.theme.body_top);

    if SHOWCASE_RENDER_LOGS.fetch_add(1, Ordering::Relaxed) == 0 {
        info!("hello: petals and stile showcase rendering");
    }

    draw_text(
        text_renderer,
        pixels,
        width,
        height,
        16,
        28,
        20.0,
        title,
        showcase.theme.chrome_text,
    );
    draw_text(
        text_renderer,
        pixels,
        width,
        height,
        16,
        50,
        12.0,
        "Clock, calculator, launcher, chrome, logograms, and Stile state selectors",
        showcase.theme.chrome_text_inactive,
    );

    let margin = 16i32;
    let gap = 12i32;
    let top_y = 70i32;
    let top_h = 132u32.min(height.saturating_sub(top_y as u32 + margin as u32));
    let left_w = 250u32.min(width.saturating_sub((margin * 2) as u32));
    let right_x = margin + left_w as i32 + gap;
    let right_w = width.saturating_sub(right_x.max(0) as u32 + margin as u32);

    render_clock_demo(
        pixels,
        width,
        height,
        margin,
        top_y,
        left_w,
        top_h,
        showcase,
        services,
        text_renderer,
    );
    render_chrome_demo(
        pixels,
        width,
        height,
        right_x,
        top_y,
        right_w,
        top_h,
        showcase,
        services,
        text_renderer,
    );

    let lower_y = top_y + top_h as i32 + gap;
    let lower_h = height.saturating_sub(lower_y.max(0) as u32 + margin as u32);
    let calc_h = lower_h;
    render_calculator_demo(
        pixels,
        width,
        height,
        margin,
        lower_y,
        left_w,
        calc_h,
        showcase,
        services,
        text_renderer,
    );

    let state_h = 112u32.min(lower_h / 3);
    let launcher_h = lower_h.saturating_sub(state_h + gap as u32);
    render_launcher_demo(
        pixels,
        width,
        height,
        right_x,
        lower_y,
        right_w,
        launcher_h,
        showcase,
        services,
        text_renderer,
    );
    render_stile_demo(
        pixels,
        width,
        height,
        right_x,
        lower_y + launcher_h as i32 + gap,
        right_w,
        state_h,
        showcase,
        services,
        text_renderer,
    );
}

fn render_popup(
    buffer: &mut BufferState,
    label: &str,
    showcase: &PetalsShowcase,
    services: &mut ShowcaseServices,
    text_renderer: Option<&TextRenderer>,
) {
    let width = buffer.width;
    let height = buffer.height;
    let pixels = buffer.texture.as_slice_mut();
    pixels.fill(showcase.theme.frame_fill);
    draw_rect_stroke(pixels, width, height, 0, 0, width, height, 1, showcase.theme.focus_accent);
    draw_text(
        text_renderer,
        pixels,
        width,
        height,
        14,
        30,
        16.0,
        label,
        showcase.theme.chrome_text,
    );
    draw_text(
        text_renderer,
        pixels,
        width,
        height,
        14,
        50,
        11.0,
        "Each demo dispatches through a PetalsService",
        showcase.theme.chrome_text_inactive,
    );
    render_stile_demo(
        pixels,
        width,
        height,
        12,
        68,
        width.saturating_sub(24),
        height.saturating_sub(80),
        showcase,
        services,
        text_renderer,
    );
}

fn render_clock_demo(
    pixels: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    showcase: &PetalsShowcase,
    services: &mut ShowcaseServices,
    text_renderer: Option<&TextRenderer>,
) {
    let Ok((mut tree, _)) = showcase.clock_tree() else {
        return;
    };
    services.dispatch(ShowcaseComponent::Clock, &mut tree, PetalsEvent::Layout);
    if showcase
        .prepare_component(&mut tree, w, h, JustifyContent::Center, AlignItems::Center)
        .is_ok()
    {
        paint_tree(&tree, pixels, stride, height, x, y, Some((x, y, w, h)), text_renderer, showcase.theme.chrome_text);
    }
}

fn render_calculator_demo(
    pixels: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    showcase: &PetalsShowcase,
    services: &mut ShowcaseServices,
    text_renderer: Option<&TextRenderer>,
) {
    let Ok((mut tree, nodes)) = showcase.calculator_tree() else {
        return;
    };
    if let Some(key) = nodes.keys.get(14) {
        services.dispatch(
            ShowcaseComponent::Calculator,
            &mut tree,
            PetalsEvent::SetState { node: key.node, state: State::Active, enabled: true },
        );
    }
    let _ = tree.restyle(&petals::Calculator::new().rules_for_theme(showcase.theme));
    if showcase
        .prepare_component(&mut tree, w, h, JustifyContent::Start, AlignItems::Stretch)
        .is_ok()
    {
        paint_tree(&tree, pixels, stride, height, x, y, Some((x, y, w, h)), text_renderer, showcase.theme.chrome_text);
    }
}

fn render_launcher_demo(
    pixels: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    showcase: &PetalsShowcase,
    services: &mut ShowcaseServices,
    text_renderer: Option<&TextRenderer>,
) {
    let Ok((mut tree, nodes)) = showcase.launcher_tree() else {
        return;
    };
    if let Some(tile) = nodes.tiles.get(1) {
        services.dispatch(
            ShowcaseComponent::Launcher,
            &mut tree,
            PetalsEvent::SetState { node: tile.node, state: State::Focus, enabled: true },
        );
    }
    let _ = tree.restyle(&petals::ApplicationLauncher::new(alloc::vec![]).rules());
    if showcase
        .prepare_component(&mut tree, w, h, JustifyContent::Start, AlignItems::Stretch)
        .is_ok()
    {
        paint_tree(&tree, pixels, stride, height, x, y, Some((x, y, w, h)), text_renderer, showcase.theme.chrome_text);
    }
}

fn render_chrome_demo(
    pixels: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    showcase: &PetalsShowcase,
    services: &mut ShowcaseServices,
    text_renderer: Option<&TextRenderer>,
) {
    let Ok((mut tree, nodes)) = showcase.chrome_tree() else {
        return;
    };
    services.dispatch(
        ShowcaseComponent::WindowChrome,
        &mut tree,
        PetalsEvent::SetState { node: nodes.maximize, state: State::Active, enabled: true },
    );
    let _ = tree.restyle(&petals::window_chrome_rules_for_theme(showcase.theme));
    if showcase
        .prepare_component(&mut tree, w, h, JustifyContent::Start, AlignItems::Stretch)
        .is_ok()
    {
        paint_tree(&tree, pixels, stride, height, x, y, Some((x, y, w, h)), text_renderer, showcase.theme.chrome_text);
        draw_text(
            text_renderer,
            pixels,
            stride,
            height,
            x + 16,
            y + 72,
            12.0,
            "Chrome, titlebar, controls, content, resize edges",
            showcase.theme.chrome_text_inactive,
        );
    }
}

fn render_stile_demo(
    pixels: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    showcase: &PetalsShowcase,
    services: &mut ShowcaseServices,
    text_renderer: Option<&TextRenderer>,
) {
    let Ok((mut tree, _)) = showcase.stile_state_tree() else {
        return;
    };
    services.dispatch(ShowcaseComponent::StileStates, &mut tree, PetalsEvent::Layout);
    if showcase
        .prepare_component(&mut tree, w, h, JustifyContent::Start, AlignItems::Stretch)
        .is_ok()
    {
        paint_tree(&tree, pixels, stride, height, x, y, Some((x, y, w, h)), text_renderer, showcase.theme.chrome_text);
    }
}

/// Returns the intersection of `(x, y, w, h)` with `clip`, or `None` when they don't overlap.
fn intersect_clip(
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    clip: (i32, i32, u32, u32),
) -> Option<(i32, i32, u32, u32)> {
    let (cx, cy, cw, ch) = clip;
    let x0 = x.max(cx);
    let y0 = y.max(cy);
    let x1 = (x + w as i32).min(cx + cw as i32);
    let y1 = (y + h as i32).min(cy + ch as i32);
    if x1 <= x0 || y1 <= y0 {
        None
    } else {
        Some((x0, y0, (x1 - x0) as u32, (y1 - y0) as u32))
    }
}

/// Returns `true` when `(px, py)` falls inside `clip`, or when no clip is active.
fn point_in_clip(px: i32, py: i32, clip: Option<(i32, i32, u32, u32)>) -> bool {
    match clip {
        None => true,
        Some((cx, cy, cw, ch)) => {
            px >= cx && px < cx + cw as i32 && py >= cy && py < cy + ch as i32
        }
    }
}

/// Paint all nodes of `tree` into `pixels`.
///
/// `clip` restricts all drawing to the given rectangle `(x, y, w, h)` in buffer coordinates.
/// Nodes whose bounds do not intersect the clip rectangle are skipped entirely.
fn paint_tree(
    tree: &UiTree,
    pixels: &mut [u32],
    stride: u32,
    height: u32,
    origin_x: i32,
    origin_y: i32,
    clip: Option<(i32, i32, u32, u32)>,
    text_renderer: Option<&TextRenderer>,
    fallback_text: u32,
) {
    for node in tree.nodes() {
        let Ok(b) = tree.global_layout_box(node.id) else {
            continue;
        };
        let x = origin_x + b.x as i32;
        let y = origin_y + b.y as i32;
        let w = b.width.max(0.0) as u32;
        let h = b.height.max(0.0) as u32;

        // Restrict drawing to the clip rectangle when one is set.
        let (dx, dy, dw, dh) = match clip {
            Some(c) => match intersect_clip(x, y, w, h, c) {
                Some(r) => r,
                None => continue,
            },
            None => (x, y, w, h),
        };

        if let Some(color) = node.style.background_color {
            fill_rect(pixels, stride, height, dx, dy, dw, dh, color_argb(color));
        }
        if node.style.border_width.unwrap_or(0.0) > 0.0 {
            draw_rect_stroke(pixels, stride, height, dx, dy, dw, dh, 1, 0x66333B48);
        }
        if node.style.outline_color.is_some() {
            // Outline is drawn at the original node boundary, not the clipped one,
            // so the focus ring is always anchored to the actual widget edge.
            // fill_rect clips at the buffer boundary; overflow past the clip rect is
            // acceptable for the thin outline ring at V0.
            draw_rect_stroke(
                pixels,
                stride,
                height,
                x - 1,
                y - 1,
                w.saturating_add(2),
                h.saturating_add(2),
                1,
                color_argb(node.style.outline_color.unwrap()),
            );
        }

        if node.descriptions.contains(&Description::Textual)
            || node.descriptions.contains(&Description::Title)
            || node.descriptions.contains(&Description::Logogram)
        {
            if let Some(text) = node_text(tree, node.id) {
                let px = node.style.font_size.unwrap_or(
                    if node.descriptions.contains(&Description::Logogram) { 22.0 } else { 12.0 },
                );
                let tx = x;
                let ty = y + px as i32;
                if point_in_clip(tx, ty, clip) {
                    let color = node.style.color.map(color_argb).unwrap_or(fallback_text);
                    // Ellipsize to the full node width so text layout is unaffected
                    // by how much of the node happens to be visible.
                    let label = petals::ellipsize_ascii(text, text_capacity(w as f32, px));
                    draw_text(text_renderer, pixels, stride, height, tx, ty, px, &label, color);
                }
            }
        } else if node.descriptions.contains(&Description::Pressable) {
            if let Some(text) = pressable_label(node.id, tree) {
                let px = node.style.font_size.unwrap_or(12.0);
                let tx = x + 8;
                let ty = y + ((h as f32 + px) / 2.0) as i32 - 3;
                if point_in_clip(tx, ty, clip) {
                    let color = node.style.color.map(color_argb).unwrap_or(fallback_text);
                    // Ellipsize to the full node width (see comment above).
                    let label = petals::ellipsize_ascii(text, text_capacity(w as f32, px));
                    draw_text(text_renderer, pixels, stride, height, tx, ty, px, &label, color);
                }
            }
        }
    }
}

fn node_text<'a>(tree: &'a UiTree, node_id: NodeId) -> Option<&'a str> {
    let node = tree.node(node_id)?;
    match node.attrs.get("text").or_else(|| node.attrs.get("glyph")) {
        Some(AttrValue::Str(value)) => Some(value.as_str()),
        _ => None,
    }
}

fn pressable_label<'a>(node_id: NodeId, tree: &'a UiTree) -> Option<&'a str> {
    let node = tree.node(node_id)?;
    match node.attrs.get("label") {
        Some(AttrValue::Str(value)) => Some(value.as_str()),
        _ => None,
    }
}

fn fill_rect(
    pixels: &mut [u32],
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    color: u32,
) {
    let x0 = x.max(0) as u32;
    let y0 = y.max(0) as u32;
    let x1 = (x.saturating_add(w as i32)).max(0) as u32;
    let y1 = (y.saturating_add(h as i32)).max(0) as u32;
    let x1 = x1.min(width);
    let y1 = y1.min(height);
    for py in y0..y1 {
        let row = py as usize * width as usize;
        for px in x0..x1 {
            pixels[row + px as usize] = color;
        }
    }
}

fn draw_rect_stroke(
    pixels: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    thickness: u32,
    color: u32,
) {
    let t = thickness.max(1);
    fill_rect(pixels, stride, height, x, y, w, t, color);
    fill_rect(pixels, stride, height, x, y + h.saturating_sub(t) as i32, w, t, color);
    fill_rect(pixels, stride, height, x, y, t, h, color);
    fill_rect(pixels, stride, height, x + w.saturating_sub(t) as i32, y, t, h, color);
}

fn color_argb(color: petals::Color) -> u32 {
    ((color.a as u32) << 24) | ((color.r as u32) << 16) | ((color.g as u32) << 8) | color.b as u32
}

fn text_capacity(width: f32, px_size: f32) -> usize {
    let advance = (px_size * 0.62).max(6.0);
    ((width / advance) as usize).max(1)
}

fn draw_text(
    text_renderer: Option<&TextRenderer>,
    pixels: &mut [u32],
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    px_size: f32,
    text: &str,
    color: u32,
) {
    let Some(renderer) = text_renderer else {
        return;
    };

    let mut text_c = [0u8; 128];
    let bytes = text.as_bytes();
    if bytes.len() >= text_c.len() {
        stem::warn!("hello: text too long for pistil text call");
        return;
    }
    text_c[..bytes.len()].copy_from_slice(bytes);

    let rc = (renderer.draw_text)(
        text_c.as_ptr(),
        pixels.as_mut_ptr(),
        width,
        height,
        width,
        x,
        y,
        px_size,
        color,
    );
    if rc != 0 {
        stem::warn!("hello: pistil_draw_text failed: {}", rc);
    }
}

fn load_text_renderer() -> Option<TextRenderer> {
    let handle = dlopen_str(PISTIL_PATH, RTLD_NOW);
    if handle.is_null() {
        log_dlerror("hello: failed to load /lib/libpistil.so");
        return None;
    }

    let sym = dlsym_bytes(handle, DRAW_TEXT_SYMBOL);
    if sym.is_null() {
        log_dlerror("hello: failed to resolve pistil_draw_text");
        return None;
    }

    let draw_text: DrawTextFn = unsafe { core::mem::transmute(sym) };
    info!("hello: pistil text renderer loaded with default {}", DEFAULT_FONT_PATH);
    Some(TextRenderer { _handle: handle, draw_text })
}

fn log_dlerror(prefix: &str) {
    let err = dlerror();
    if err.is_null() {
        stem::warn!("{}", prefix);
        return;
    }

    let mut len = 0usize;
    unsafe {
        while *err.add(len) != 0 && len < 256 {
            len += 1;
        }
        let bytes = core::slice::from_raw_parts(err, len);
        match core::str::from_utf8(bytes) {
            Ok(msg) => stem::warn!("{}: {}", prefix, msg),
            Err(_) => stem::warn!("{}", prefix),
        }
    }
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

fn get_xdg_surface(fd: u32, wm_base_id: u32, new_id: u32, surface_id: u32) {
    let mut buf = Vec::new();
    encode_header(wm_base_id, 2, 16, &mut buf);
    buf.extend_from_slice(&new_id.to_ne_bytes());
    buf.extend_from_slice(&surface_id.to_ne_bytes());
    send_request(fd, &buf);
}

fn get_toplevel(fd: u32, xdg_surface_id: u32, new_id: u32) {
    let mut buf = Vec::new();
    encode_header(xdg_surface_id, 1, 12, &mut buf);
    buf.extend_from_slice(&new_id.to_ne_bytes());
    send_request(fd, &buf);
}

fn set_toplevel_title(fd: u32, toplevel_id: u32, title: &str) {
    send_string_request(fd, toplevel_id, 2, title);
}

fn set_toplevel_app_id(fd: u32, toplevel_id: u32, app_id: &str) {
    send_string_request(fd, toplevel_id, 3, app_id);
}

fn seat_get_pointer(fd: u32, seat_id: u32, new_id: u32) {
    let mut buf = Vec::new();
    encode_header(seat_id, 0, 12, &mut buf);
    buf.extend_from_slice(&new_id.to_ne_bytes());
    send_request(fd, &buf);
}

fn seat_get_keyboard(fd: u32, seat_id: u32, new_id: u32) {
    let mut buf = Vec::new();
    encode_header(seat_id, 1, 12, &mut buf);
    buf.extend_from_slice(&new_id.to_ne_bytes());
    send_request(fd, &buf);
}

fn create_positioner(fd: u32, wm_base_id: u32, new_id: u32) {
    let mut buf = Vec::new();
    encode_header(wm_base_id, 1, 12, &mut buf);
    buf.extend_from_slice(&new_id.to_ne_bytes());
    send_request(fd, &buf);
}

fn positioner_set_size(fd: u32, positioner_id: u32, width: i32, height: i32) {
    let mut buf = Vec::new();
    encode_header(positioner_id, 1, 16, &mut buf);
    buf.extend_from_slice(&width.to_ne_bytes());
    buf.extend_from_slice(&height.to_ne_bytes());
    send_request(fd, &buf);
}

fn positioner_set_anchor_rect(
    fd: u32,
    positioner_id: u32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    let mut buf = Vec::new();
    encode_header(positioner_id, 2, 24, &mut buf);
    buf.extend_from_slice(&x.to_ne_bytes());
    buf.extend_from_slice(&y.to_ne_bytes());
    buf.extend_from_slice(&width.to_ne_bytes());
    buf.extend_from_slice(&height.to_ne_bytes());
    send_request(fd, &buf);
}

fn positioner_set_offset(fd: u32, positioner_id: u32, x: i32, y: i32) {
    let mut buf = Vec::new();
    encode_header(positioner_id, 6, 16, &mut buf);
    buf.extend_from_slice(&x.to_ne_bytes());
    buf.extend_from_slice(&y.to_ne_bytes());
    send_request(fd, &buf);
}

fn get_popup(
    fd: u32,
    parent_xdg_surface_id: u32,
    xdg_surface_id: u32,
    popup_id: u32,
    positioner_id: u32,
) {
    let mut buf = Vec::new();
    encode_header(xdg_surface_id, 2, 20, &mut buf);
    buf.extend_from_slice(&popup_id.to_ne_bytes());
    buf.extend_from_slice(&parent_xdg_surface_id.to_ne_bytes());
    buf.extend_from_slice(&positioner_id.to_ne_bytes());
    send_request(fd, &buf);
}

fn ack_configure(fd: u32, xdg_surface_id: u32, serial: u32) {
    let mut buf = Vec::new();
    encode_header(xdg_surface_id, 4, 12, &mut buf);
    buf.extend_from_slice(&serial.to_ne_bytes());
    send_request(fd, &buf);
}

fn send_pong(fd: u32, wm_base_id: u32, serial: u32) {
    let mut buf = Vec::new();
    encode_header(wm_base_id, 3, 12, &mut buf);
    buf.extend_from_slice(&serial.to_ne_bytes());
    send_request(fd, &buf);
}

fn create_pool(fd: u32, shm_id: u32, pool_id: u32, bs_raw: u32, size: u32) {
    let mut buf = Vec::new();
    encode_header(shm_id, 0, 16, &mut buf);
    buf.extend_from_slice(&pool_id.to_ne_bytes());
    buf.extend_from_slice(&size.to_ne_bytes());
    send_request_with_fds(fd, &buf, &[bs_raw]);
}

fn create_dmabuf_buffer(
    fd: u32,
    dmabuf_id: u32,
    params_id: u32,
    buffer_id: u32,
    dma_fd: u32,
    width: u32,
    height: u32,
    stride: u32,
) {
    let mut buf = Vec::new();
    encode_header(dmabuf_id, 1, 12, &mut buf);
    buf.extend_from_slice(&params_id.to_ne_bytes());

    encode_header(params_id, 1, 28, &mut buf);
    buf.extend_from_slice(&0u32.to_ne_bytes()); // plane_idx
    buf.extend_from_slice(&0u32.to_ne_bytes()); // offset
    buf.extend_from_slice(&stride.to_ne_bytes());
    buf.extend_from_slice(&0u32.to_ne_bytes()); // modifier_hi: linear
    buf.extend_from_slice(&0u32.to_ne_bytes()); // modifier_lo: linear

    encode_header(params_id, 3, 28, &mut buf);
    buf.extend_from_slice(&buffer_id.to_ne_bytes());
    buf.extend_from_slice(&(width as i32).to_ne_bytes());
    buf.extend_from_slice(&(height as i32).to_ne_bytes());
    buf.extend_from_slice(&DRM_FORMAT_ARGB8888.to_ne_bytes());
    buf.extend_from_slice(&0u32.to_ne_bytes()); // flags

    encode_header(params_id, 0, 8, &mut buf);
    send_request_with_fds(fd, &buf, &[dma_fd]);
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

fn request_frame(fd: u32, surface_id: u32, callback_id: u32) {
    let mut buf = Vec::new();
    encode_header(surface_id, 3, 12, &mut buf);
    buf.extend_from_slice(&callback_id.to_ne_bytes());
    send_request(fd, &buf);
}

/// Send `wp_presentation.feedback(surface, callback)` (opcode 1).
///
/// The compositor will reply on `feedback_id` with either
/// `wp_presentation_feedback.presented` (opcode 1) or `discarded`
/// (opcode 2) once the next commit on `surface_id` is settled.
fn request_presentation_feedback(fd: u32, presentation_id: u32, surface_id: u32, feedback_id: u32) {
    let mut buf = Vec::new();
    encode_header(presentation_id, 1, 16, &mut buf);
    buf.extend_from_slice(&surface_id.to_ne_bytes());
    buf.extend_from_slice(&feedback_id.to_ne_bytes());
    send_request(fd, &buf);
}

fn commit_surface(fd: u32, surface_id: u32) {
    let mut buf = Vec::new();
    encode_header(surface_id, 6, 8, &mut buf);
    send_request(fd, &buf);
}

fn close_toplevel_objects(fd: u32) {
    destroy_object(fd, TOPLEVEL_ID);
    destroy_object(fd, TOP_XDG_SURFACE_ID);
    destroy_object(fd, TOP_SURFACE_ID);
    destroy_object(fd, POINTER_ID);
    destroy_object(fd, KEYBOARD_ID);
}

fn close_popup_objects(fd: u32) {
    destroy_object(fd, POPUP_ID);
    destroy_object(fd, POPUP_XDG_SURFACE_ID);
    destroy_object(fd, POPUP_SURFACE_ID);
    destroy_object(fd, POSITIONER_ID);
}

fn destroy_object(fd: u32, object_id: u32) {
    let mut buf = Vec::new();
    encode_header(object_id, 0, 8, &mut buf);
    send_request(fd, &buf);
}

fn alloc_callback_id(next_callback_id: &mut u32) -> u32 {
    let id = *next_callback_id;
    *next_callback_id = next_callback_id.saturating_add(1);
    id
}

fn send_string_request(fd: u32, object_id: u32, opcode: u16, value: &str) {
    let mut buf = Vec::new();
    let len = value.len() as u32 + 1;
    let mut bytes = value.as_bytes().to_vec();
    bytes.push(0);
    while bytes.len() % 4 != 0 {
        bytes.push(0);
    }
    let size = 8 + 4 + bytes.len() as u16;
    encode_header(object_id, opcode, size, &mut buf);
    buf.extend_from_slice(&len.to_ne_bytes());
    buf.extend_from_slice(&bytes);
    send_request(fd, &buf);
}

fn send_request(fd: u32, buf: &[u8]) {
    if let Err(e) = vfs_write(fd, buf) {
        stem::warn!("hello: write failed: {:?}", e);
    }
}

fn send_request_with_fds(fd: u32, buf: &[u8], fds: &[u32]) {
    if fds.is_empty() {
        send_request(fd, buf);
        return;
    }
    if let Err(e) = sendmsg(fd, buf, fds) {
        stem::warn!("hello: sendmsg failed: {:?}", e);
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

fn read_i32(buf: &[u8], offset: usize) -> i32 {
    i32::from_ne_bytes(buf[offset..offset + 4].try_into().unwrap())
}

fn read_wayland_string(buf: &[u8], offset: usize) -> Option<(&[u8], usize)> {
    if offset + 4 > buf.len() {
        return None;
    }
    let len = read_u32(buf, offset) as usize;
    let start = offset + 4;
    let end = start + len;
    if end > buf.len() || len == 0 {
        return None;
    }
    let padded = (len + 3) & !3;
    let str_end = if buf[end - 1] == 0 { end - 1 } else { end };
    Some((&buf[start..str_end], 4 + padded))
}

fn wl_fixed_to_i32(value: i32) -> i32 {
    value / 256
}

// ── wl_region helpers ─────────────────────────────────────────────────────────

/// Send `wl_compositor.create_region(new_id)` — opcode 1.
fn create_region(fd: u32, compositor_id: u32, new_id: u32) {
    let mut buf = Vec::new();
    encode_header(compositor_id, 1, 12, &mut buf);
    buf.extend_from_slice(&new_id.to_ne_bytes());
    send_request(fd, &buf);
}

/// Send `wl_region.add(x, y, width, height)` — opcode 1.
fn region_add(fd: u32, region_id: u32, x: i32, y: i32, width: i32, height: i32) {
    let mut buf = Vec::new();
    encode_header(region_id, 1, 24, &mut buf);
    buf.extend_from_slice(&x.to_ne_bytes());
    buf.extend_from_slice(&y.to_ne_bytes());
    buf.extend_from_slice(&width.to_ne_bytes());
    buf.extend_from_slice(&height.to_ne_bytes());
    send_request(fd, &buf);
}

/// Send `wl_surface.set_opaque_region(region_id)` — opcode 4.
/// Pass `region_id = 0` to clear (null region).
fn set_opaque_region(fd: u32, surface_id: u32, region_id: u32) {
    let mut buf = Vec::new();
    encode_header(surface_id, 4, 12, &mut buf);
    buf.extend_from_slice(&region_id.to_ne_bytes());
    send_request(fd, &buf);
}

/// Send `wl_surface.set_input_region(region_id)` — opcode 5.
/// Pass `region_id = 0` to clear (null region, entire surface receives input).
fn set_input_region(fd: u32, surface_id: u32, region_id: u32) {
    let mut buf = Vec::new();
    encode_header(surface_id, 5, 12, &mut buf);
    buf.extend_from_slice(&region_id.to_ne_bytes());
    send_request(fd, &buf);
}

/// Send `wl_region.destroy()` — opcode 0 (destructor).
fn region_destroy(fd: u32, region_id: u32) {
    let mut buf = Vec::new();
    encode_header(region_id, 0, 8, &mut buf);
    send_request(fd, &buf);
}

#[cfg(test)]
mod tests {
    use petals::{AlignItems, Color, Declaration, Description, FlexDirection, Rule, Selector, UiTree, AvailableSpace, Size};

    use super::*;

    // ── intersect_clip ──────────────────────────────────────────────────────

    #[test]
    fn intersect_clip_full_overlap() {
        let result = intersect_clip(10, 10, 80, 80, (0, 0, 100, 100));
        assert_eq!(result, Some((10, 10, 80, 80)));
    }

    #[test]
    fn intersect_clip_right_overflow_is_clamped() {
        // node right edge (150) extends past clip right (100)
        let result = intersect_clip(50, 0, 100, 50, (0, 0, 100, 100));
        assert_eq!(result, Some((50, 0, 50, 50)));
    }

    #[test]
    fn intersect_clip_no_overlap_returns_none() {
        let result = intersect_clip(200, 0, 50, 50, (0, 0, 100, 100));
        assert!(result.is_none());
    }

    #[test]
    fn intersect_clip_touching_edge_returns_none() {
        // rect starts exactly at clip right edge → zero width → None
        let result = intersect_clip(100, 0, 50, 50, (0, 0, 100, 100));
        assert!(result.is_none());
    }

    // ── point_in_clip ───────────────────────────────────────────────────────

    #[test]
    fn point_in_clip_inside() {
        assert!(point_in_clip(50, 50, Some((0, 0, 100, 100))));
    }

    #[test]
    fn point_in_clip_outside_right() {
        assert!(!point_in_clip(100, 50, Some((0, 0, 100, 100))));
    }

    #[test]
    fn point_in_clip_none_always_true() {
        assert!(point_in_clip(999, 999, None));
    }

    // ── paint_tree clipping ─────────────────────────────────────────────────

    /// Helper: build a tree containing a single wide pressable node.
    fn wide_button_tree(node_width: f32, container_width: f32) -> UiTree {
        let mut tree = UiTree::new().unwrap();
        let root = tree.root();
        let btn = tree.pressable("Wide").unwrap();
        tree.add_child(root, btn).unwrap();

        let rules = alloc::vec![
            Rule::new(
                Selector::has(Description::Container),
                alloc::vec![
                    Declaration::FlexDirection(FlexDirection::Row),
                    Declaration::AlignItems(AlignItems::Start),
                ],
            ),
            Rule::new(
                Selector::has(Description::Pressable),
                alloc::vec![
                    Declaration::BackgroundColor(Color::rgb(0xFF, 0, 0)),
                    Declaration::Width(node_width),
                    Declaration::Height(40.0),
                ],
            ),
        ];
        tree.restyle(&rules).unwrap();
        tree.compute_layout(Size {
            width: AvailableSpace::Definite(container_width),
            height: AvailableSpace::Definite(100.0),
        })
        .unwrap();
        tree
    }

    #[test]
    fn paint_tree_with_clip_does_not_draw_outside_clip_rect() {
        // Buffer is 200 px wide; the clip rect covers only the first 100 px.
        // The button node is 150 px wide, so without clipping it would fill
        // pixels 0..150.  With clipping it must stay within 0..100.
        let buf_w: u32 = 200;
        let buf_h: u32 = 100;
        let mut pixels = alloc::vec![0u32; (buf_w * buf_h) as usize];

        let tree = wide_button_tree(150.0, 200.0);
        paint_tree(
            &tree,
            &mut pixels,
            buf_w,
            buf_h,
            0,
            0,
            Some((0, 0, 100, 100)), // clip: left half of the buffer
            None,
            0xFF_FF_FF_FF,
        );

        // All pixels from x=100 onward must remain untouched (zero).
        for row in 0..buf_h {
            for col in 100..buf_w {
                let idx = (row * buf_w + col) as usize;
                assert_eq!(
                    pixels[idx], 0,
                    "pixel at ({col}, {row}) should not have been painted outside the clip rect"
                );
            }
        }
        // At least one pixel inside the clip should have been painted.
        let painted = pixels[..100].iter().any(|&p| p != 0);
        assert!(painted, "expected some pixels to be painted inside the clip rect");
    }

    #[test]
    fn paint_tree_without_clip_draws_full_node() {
        // Same setup but no clip: the 150 px button should fill pixels 0..150.
        let buf_w: u32 = 200;
        let buf_h: u32 = 100;
        let mut pixels = alloc::vec![0u32; (buf_w * buf_h) as usize];

        let tree = wide_button_tree(150.0, 200.0);
        paint_tree(&tree, &mut pixels, buf_w, buf_h, 0, 0, None, None, 0xFF_FF_FF_FF);

        // Pixels in the range 0..150 on the first painted row should be non-zero.
        let first_row_painted = (0..150u32).any(|col| pixels[col as usize] != 0);
        assert!(first_row_painted, "expected pixels inside the node to be painted");
    }
}
