#![no_std]
#![no_main]
use core::default::Default;
extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

use libdl::{RTLD_NOW, dlerror, dlopen_str, dlsym_bytes};
use stem::dmabuf::DmaBuf;
use stem::info;
use stem::syscall::socket::{connect, sendmsg, socket};
use stem::syscall::socket_domain::AF_UNIX;
use stem::syscall::socket_type::SOCK_STREAM;
use stem::syscall::{sleep_ms, vfs_close, vfs_read, vfs_write};

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

const DRM_FORMAT_ARGB8888: u32 = 0x3432_5241; // "AR24"

const PISTIL_PATH: &str = "/lib/libpistil.so";
const DRAW_TEXT_SYMBOL: &[u8] = b"pistil_draw_text";
const DEFAULT_FONT_PATH: &str = "/share/fonts/Inter-Regular.ttf";

type DrawTextFn = extern "C" fn(*const u8, *mut u32, u32, u32, u32, i32, i32, f32, u32) -> i32;

static POINTER_MOTION_LOGS: AtomicU32 = AtomicU32::new(0);

struct TextRenderer {
    _handle: *mut core::ffi::c_void,
    draw_text: DrawTextFn,
}

#[derive(Clone, Copy)]
struct BufferState {
    pool_id: u32,
    buffer_id: u32,
    backing: DmaBuf,
    width: u32,
    height: u32,
    stride: u32,
}

#[derive(Clone, Copy, Default)]
struct InitialGlobals {
    dmabuf_name: Option<u32>,
}

struct PendingSurface {
    serial: Option<u32>,
    width: u32,
    height: u32,
    dirty: bool,
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let fd = connect_wayland();
    let text_renderer = load_text_renderer();

    send_get_registry(fd, REGISTRY_ID);
    let globals = read_initial_globals(fd);

    bind_global(fd, 1, "wl_compositor", 4, COMPOSITOR_ID);
    bind_global(fd, 2, "wl_shm", 1, SHM_ID);
    bind_global(fd, 3, "xdg_wm_base", 1, WM_BASE_ID);
    bind_global(fd, 4, "wl_seat", 5, SEAT_ID);
    let dmabuf_id = if let Some(name) = globals.dmabuf_name {
        bind_global(fd, name, "zwp_linux_dmabuf_v1", 3, DMABUF_ID);
        info!("wayland_hello: using zwp_linux_dmabuf_v1 buffers");
        Some(DMABUF_ID)
    } else {
        info!("wayland_hello: zwp_linux_dmabuf_v1 unavailable; using wl_shm buffers");
        None
    };
    seat_get_pointer(fd, SEAT_ID, POINTER_ID);
    seat_get_keyboard(fd, SEAT_ID, KEYBOARD_ID);

    create_surface(fd, COMPOSITOR_ID, TOP_SURFACE_ID);
    get_xdg_surface(fd, WM_BASE_ID, TOP_XDG_SURFACE_ID, TOP_SURFACE_ID);
    get_toplevel(fd, TOP_XDG_SURFACE_ID, TOPLEVEL_ID);
    set_toplevel_title(fd, TOPLEVEL_ID, "Thing-OS Wayland Lab");
    set_toplevel_app_id(fd, TOPLEVEL_ID, "thingos.wayland_hello");
    commit_surface(fd, TOP_SURFACE_ID);

    let mut top_pending = PendingSurface { serial: None, width: 480, height: 320, dirty: false };
    let mut popup_pending = PendingSurface { serial: None, width: 160, height: 96, dirty: false };
    let mut top_buffer: Option<BufferState> = None;
    let mut popup_buffer: Option<BufferState> = None;
    let mut popup_created = false;
    let mut next_callback_id = 1000u32;
    let mut pending_frame_callbacks: Vec<u32> = Vec::new();

    loop {
        let mut in_buf = [0u8; 4096];
        let len = match vfs_read(fd, &mut in_buf) {
            Ok(n) if n > 0 => n,
            _ => {
                sleep_ms(16);
                continue;
            }
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
                    send_pong(fd, WM_BASE_ID, read_u32(payload, 0));
                }
                (TOP_XDG_SURFACE_ID, 0) if payload.len() >= 4 => {
                    top_pending.serial = Some(read_u32(payload, 0));
                    top_pending.dirty = true;
                }
                (TOPLEVEL_ID, 0) if payload.len() >= 12 => {
                    let width = read_i32(payload, 0);
                    let height = read_i32(payload, 4);
                    if width > 0 {
                        top_pending.width = width as u32;
                    }
                    if height > 0 {
                        top_pending.height = height as u32;
                    }
                }
                (TOPLEVEL_ID, 1) => {
                    info!("wayland_hello: compositor requested close; idling");
                    idle_forever();
                }
                (POINTER_ID, 0) if payload.len() >= 16 => {
                    info!(
                        "wayland_hello: pointer enter surface={} x={} y={}",
                        read_u32(payload, 4),
                        wl_fixed_to_i32(read_i32(payload, 8)),
                        wl_fixed_to_i32(read_i32(payload, 12))
                    );
                }
                (POINTER_ID, 1) if payload.len() >= 8 => {
                    info!("wayland_hello: pointer leave surface={}", read_u32(payload, 4));
                }
                (POINTER_ID, 2) if payload.len() >= 12 => {
                    if POINTER_MOTION_LOGS.fetch_add(1, Ordering::Relaxed) < 4 {
                        info!(
                            "wayland_hello: pointer motion x={} y={}",
                            wl_fixed_to_i32(read_i32(payload, 4)),
                            wl_fixed_to_i32(read_i32(payload, 8))
                        );
                    }
                }
                (POINTER_ID, 3) if payload.len() >= 16 => {
                    info!(
                        "wayland_hello: pointer button button={} state={}",
                        read_u32(payload, 8),
                        read_u32(payload, 12)
                    );
                }
                (KEYBOARD_ID, 1) if payload.len() >= 8 => {
                    info!("wayland_hello: keyboard enter surface={}", read_u32(payload, 4));
                }
                (KEYBOARD_ID, 2) if payload.len() >= 8 => {
                    info!("wayland_hello: keyboard leave surface={}", read_u32(payload, 4));
                }
                (KEYBOARD_ID, 3) if payload.len() >= 16 => {
                    info!(
                        "wayland_hello: keyboard key key={} state={}",
                        read_u32(payload, 8),
                        read_u32(payload, 12)
                    );
                }
                (KEYBOARD_ID, 4) if payload.len() >= 20 => {
                    info!("wayland_hello: keyboard modifiers depressed={}", read_u32(payload, 4));
                }
                (POPUP_XDG_SURFACE_ID, 0) if payload.len() >= 4 => {
                    popup_pending.serial = Some(read_u32(payload, 0));
                    popup_pending.dirty = true;
                }
                (POPUP_ID, 0) if payload.len() >= 16 => {
                    let width = read_i32(payload, 8);
                    let height = read_i32(payload, 12);
                    if width > 0 {
                        popup_pending.width = width as u32;
                    }
                    if height > 0 {
                        popup_pending.height = height as u32;
                    }
                }
                (POPUP_ID, 1) => {
                    popup_created = false;
                    popup_pending.serial = None;
                }
                (_, 0) if pending_frame_callbacks.iter().any(|&id| id == object_id) => {
                    let callback_data = if payload.len() >= 4 { read_u32(payload, 0) } else { 0 };
                    info!(
                        "wayland_hello: frame callback done object={} data={}",
                        object_id, callback_data
                    );
                    pending_frame_callbacks.retain(|&id| id != object_id);
                }
                _ => {}
            }
            offset += size as usize;
        }

        if top_pending.dirty {
            let title = "Thing-OS Wayland";
            let buffer = ensure_buffer(
                fd,
                SHM_ID,
                dmabuf_id,
                &mut top_buffer,
                TOP_SURFACE_ID + 100,
                top_pending.width,
                top_pending.height,
            );
            render_window(buffer, title, text_renderer.as_ref());
            ack_configure(fd, TOP_XDG_SURFACE_ID, top_pending.serial.unwrap_or(0));
            attach_buffer(fd, TOP_SURFACE_ID, buffer.buffer_id);
            damage_surface(fd, TOP_SURFACE_ID, 0, 0, top_pending.width, top_pending.height);
            let cb_id = alloc_callback_id(&mut next_callback_id);
            request_frame(fd, TOP_SURFACE_ID, cb_id);
            pending_frame_callbacks.push(cb_id);
            commit_surface(fd, TOP_SURFACE_ID);
            top_pending.dirty = false;

            if !popup_created {
                popup_created = true;
                create_surface(fd, COMPOSITOR_ID, POPUP_SURFACE_ID);
                get_xdg_surface(fd, WM_BASE_ID, POPUP_XDG_SURFACE_ID, POPUP_SURFACE_ID);
                create_positioner(fd, WM_BASE_ID, POSITIONER_ID);
                positioner_set_size(fd, POSITIONER_ID, 160, 96);
                positioner_set_anchor_rect(fd, POSITIONER_ID, 24, 24, 100, 24);
                positioner_set_offset(fd, POSITIONER_ID, 0, 6);
                get_popup(fd, TOP_XDG_SURFACE_ID, POPUP_XDG_SURFACE_ID, POPUP_ID, POSITIONER_ID);
                commit_surface(fd, POPUP_SURFACE_ID);
            }
        }

        if popup_created && popup_pending.dirty {
            let buffer = ensure_buffer(
                fd,
                SHM_ID,
                dmabuf_id,
                &mut popup_buffer,
                POPUP_SURFACE_ID + 100,
                popup_pending.width,
                popup_pending.height,
            );
            render_popup(buffer, "Fresh pixels", text_renderer.as_ref());
            ack_configure(fd, POPUP_XDG_SURFACE_ID, popup_pending.serial.unwrap_or(0));
            attach_buffer(fd, POPUP_SURFACE_ID, buffer.buffer_id);
            damage_surface(fd, POPUP_SURFACE_ID, 0, 0, popup_pending.width, popup_pending.height);
            let cb_id = alloc_callback_id(&mut next_callback_id);
            request_frame(fd, POPUP_SURFACE_ID, cb_id);
            pending_frame_callbacks.push(cb_id);
            commit_surface(fd, POPUP_SURFACE_ID);
            popup_pending.dirty = false;
        }
    }
}

fn connect_wayland() -> u32 {
    loop {
        let fd = match socket(AF_UNIX, SOCK_STREAM, 0) {
            Ok(fd) => fd,
            Err(e) => {
                stem::error!("wayland_hello: socket(AF_UNIX) failed: {:?}", e);
                sleep_ms(250);
                continue;
            }
        };

        match connect(fd, "/run/wayland-0") {
            Ok(()) => {
                info!("wayland_hello: connected to /run/wayland-0");
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
                            if version_off + 4 <= payload.len()
                                && iface == b"zwp_linux_dmabuf_v1"
                                && read_u32(payload, version_off) >= 3
                            {
                                globals.dmabuf_name = Some(name);
                            }
                        }
                    }
                    offset += size as usize;
                }
                if offset > 0 {
                    rx.drain(..offset);
                }
                if globals.dmabuf_name.is_some() {
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
) -> BufferState {
    if let Some(buf) = current {
        if buf.width == width && buf.height == height {
            return *buf;
        }
    }

    let stride = width * 4;
    let size = stride * height;
    let backing = DmaBuf::create("wl.buffer", size as usize).expect("create wl buffer");

    let pool_id = base_id;
    let buffer_id = base_id + 1;
    if let Some(dmabuf) = dmabuf_id {
        create_dmabuf_buffer(fd, dmabuf, pool_id, buffer_id, backing.fd(), width, height, stride);
    } else {
        create_pool(fd, shm_id, pool_id, backing.fd(), size);
        create_buffer(fd, pool_id, buffer_id, width, height, stride);
    }

    let out = BufferState { pool_id, buffer_id, backing, width, height, stride };
    *current = Some(out);
    out
}

fn render_window(buffer: BufferState, title: &str, text_renderer: Option<&TextRenderer>) {
    unsafe {
        let pixels = core::slice::from_raw_parts_mut(
            buffer.backing.as_mut_ptr() as *mut u32,
            (buffer.width * buffer.height) as usize,
        );
        paint_vertical_gradient(pixels, buffer.width, buffer.height, 0xFFFFF9EC, 0xFFFDF1D2);
        draw_text(
            text_renderer,
            pixels,
            buffer.width,
            buffer.height,
            10,
            23,
            13.0,
            title,
            0xFF3F3A2F,
        );
        draw_text(
            text_renderer,
            pixels,
            buffer.width,
            buffer.height,
            10,
            48,
            13.0,
            "Resize the frame; the buffer follows.",
            0xFF3F3A2F,
        );
        draw_text(
            text_renderer,
            pixels,
            buffer.width,
            buffer.height,
            10,
            72,
            13.0,
            "Compositor round-trip: shm, xdg, paint.",
            0xFF3F3A2F,
        );
    }
}

fn render_popup(buffer: BufferState, label: &str, text_renderer: Option<&TextRenderer>) {
    unsafe {
        let pixels = core::slice::from_raw_parts_mut(
            buffer.backing.as_mut_ptr() as *mut u32,
            (buffer.width * buffer.height) as usize,
        );
        for y in 0..buffer.height as usize {
            for x in 0..buffer.width as usize {
                let border = x < 2
                    || y < 2
                    || x + 2 >= buffer.width as usize
                    || y + 2 >= buffer.height as usize;
                pixels[y * buffer.width as usize + x] =
                    if border { 0xFFB58900 } else { 0xEEFEF6E3 };
            }
        }
        draw_text(
            text_renderer,
            pixels,
            buffer.width,
            buffer.height,
            14,
            34,
            18.0,
            label,
            0xFF3F3A2F,
        );
    }
}

fn paint_vertical_gradient(pixels: &mut [u32], width: u32, height: u32, top: u32, bottom: u32) {
    let denom = height.saturating_sub(1).max(1);
    for y in 0..height {
        let color = lerp_argb(top, bottom, y.saturating_mul(255) / denom);
        let row = y as usize * width as usize;
        for x in 0..width as usize {
            pixels[row + x] = color;
        }
    }
}

fn lerp_argb(a: u32, b: u32, t: u32) -> u32 {
    let inv = 255u32.saturating_sub(t.min(255));
    let aa = ((a >> 24) & 0xFF) * inv + ((b >> 24) & 0xFF) * t;
    let ar = ((a >> 16) & 0xFF) * inv + ((b >> 16) & 0xFF) * t;
    let ag = ((a >> 8) & 0xFF) * inv + ((b >> 8) & 0xFF) * t;
    let ab = (a & 0xFF) * inv + (b & 0xFF) * t;
    ((aa / 255) << 24) | ((ar / 255) << 16) | ((ag / 255) << 8) | (ab / 255)
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
        stem::warn!("wayland_hello: text too long for pistil text call");
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
        stem::warn!("wayland_hello: pistil_draw_text failed: {}", rc);
    }
}

fn load_text_renderer() -> Option<TextRenderer> {
    let handle = dlopen_str(PISTIL_PATH, RTLD_NOW);
    if handle.is_null() {
        log_dlerror("wayland_hello: failed to load /lib/libpistil.so");
        return None;
    }

    let sym = dlsym_bytes(handle, DRAW_TEXT_SYMBOL);
    if sym.is_null() {
        log_dlerror("wayland_hello: failed to resolve pistil_draw_text");
        return None;
    }

    let draw_text: DrawTextFn = unsafe { core::mem::transmute(sym) };
    info!("wayland_hello: pistil text renderer loaded with default {}", DEFAULT_FONT_PATH);
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

fn panic_forever(msg: &str) -> ! {
    stem::error!("wayland_hello: {}", msg);
    idle_forever()
}

fn idle_forever() -> ! {
    loop {
        sleep_ms(1000);
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

fn commit_surface(fd: u32, surface_id: u32) {
    let mut buf = Vec::new();
    encode_header(surface_id, 6, 8, &mut buf);
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
        stem::warn!("wayland_hello: write failed: {:?}", e);
    }
}

fn send_request_with_fds(fd: u32, buf: &[u8], fds: &[u32]) {
    if fds.is_empty() {
        send_request(fd, buf);
        return;
    }
    if let Err(e) = sendmsg(fd, buf, fds) {
        stem::warn!("wayland_hello: sendmsg failed: {:?}", e);
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
