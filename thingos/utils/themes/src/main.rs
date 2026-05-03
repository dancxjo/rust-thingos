#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::convert::TryInto;

use abi::hid::Key;
use libdl::{RTLD_NOW, dlerror, dlopen_str, dlsym_bytes};
use petals::{Theme, available_themes, default_theme, find_theme_by_name, theme_by_name};
use stem::abi::syscall::{PollHandle, poll_flags, vfs_flags};
use stem::info;
use stem::syscall::socket::{connect, sendmsg, socket};
use stem::syscall::socket_domain::AF_UNIX;
use stem::syscall::socket_type::SOCK_STREAM;
use stem::syscall::{
    argv_get, exit, memfd_create, sleep_ms, vfs_close, vfs_mkdir, vfs_open, vfs_poll, vfs_read,
    vfs_write, vm_map,
};

const REGISTRY_ID: u32 = 2;
const COMPOSITOR_ID: u32 = 3;
const SHM_ID: u32 = 4;
const WM_BASE_ID: u32 = 5;
const SEAT_ID: u32 = 6;
const POINTER_ID: u32 = 7;
const KEYBOARD_ID: u32 = 8;

const SURFACE_ID: u32 = 10;
const XDG_SURFACE_ID: u32 = 11;
const TOPLEVEL_ID: u32 = 12;

const THEME_PATH: &str = "/session/desktop/theme";
const PISTIL_PATH: &str = "/lib/libpistil.so";
const DRAW_TEXT_SYMBOL: &[u8] = b"pistil_draw_text";
const IDLE_SLEEP_MS: u64 = 16;

type DrawTextFn = extern "C" fn(*const u8, *mut u32, u32, u32, u32, i32, i32, f32, u32) -> i32;

struct TextRenderer {
    _handle: *mut core::ffi::c_void,
    draw_text: DrawTextFn,
}

#[derive(Clone, Copy)]
struct BufferState {
    _pool_id: u32,
    buffer_id: u32,
    _handle: u32,
    ptr: *mut u8,
    width: u32,
    height: u32,
    _stride: u32,
}

struct PendingSurface {
    serial: Option<u32>,
    width: u32,
    height: u32,
    dirty: bool,
    configured: bool,
}

#[derive(Default)]
struct PointerState {
    x: i32,
    y: i32,
    pressed_index: Option<usize>,
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    if args.len() > 1 {
        run_cli(&args);
    }

    run_gui()
}

fn run_cli(args: &[String]) -> ! {
    match args.get(1).map(|arg| arg.as_str()) {
        Some("--list") | Some("-l") => {
            print_available_themes();
            exit(0);
        }
        Some("--current") | Some("-c") => {
            print(&alloc::format!("{}\n", current_theme().name));
            exit(0);
        }
        Some("--set") if args.len() >= 3 => set_theme_cli(&args[2]),
        Some("--help") | Some("-h") => {
            print("usage: themes [--list|--current|--set <theme>|<theme>]\n");
            exit(0);
        }
        Some(name) => set_theme_cli(name),
        None => exit(0),
    }
}

fn set_theme_cli(name: &str) -> ! {
    let Some(theme) = find_theme_by_name(name) else {
        print_err(&alloc::format!("themes: unknown theme '{}'\n", name));
        print_available_themes();
        exit(1);
    };

    match write_theme_name(theme.name) {
        Ok(()) => {
            print(&alloc::format!("Selected theme {}\n", theme.name));
            exit(0);
        }
        Err(msg) => {
            print_err(&alloc::format!("themes: {}\n", msg));
            exit(1);
        }
    }
}

fn print_available_themes() {
    let current = current_theme();
    print("Available themes:\n");
    for theme in available_themes() {
        let marker = if theme.name == current.name { "*" } else { " " };
        print(&alloc::format!("{} {}\n", marker, theme.name));
    }
}

fn run_gui() -> ! {
    let fd = connect_wayland();
    let text_renderer = load_text_renderer();
    let mut selected_index = theme_index(current_theme());
    let mut active_theme = available_themes()[selected_index];

    send_get_registry(fd, REGISTRY_ID);
    read_initial_globals(fd);

    bind_global(fd, 1, "wl_compositor", 4, COMPOSITOR_ID);
    bind_global(fd, 2, "wl_shm", 1, SHM_ID);
    bind_global(fd, 3, "xdg_wm_base", 1, WM_BASE_ID);
    bind_global(fd, 4, "wl_seat", 5, SEAT_ID);
    seat_get_pointer(fd, SEAT_ID, POINTER_ID);
    seat_get_keyboard(fd, SEAT_ID, KEYBOARD_ID);

    create_surface(fd, COMPOSITOR_ID, SURFACE_ID);
    get_xdg_surface(fd, WM_BASE_ID, XDG_SURFACE_ID, SURFACE_ID);
    get_toplevel(fd, XDG_SURFACE_ID, TOPLEVEL_ID);
    set_toplevel_title(fd, TOPLEVEL_ID, "Themes");
    set_toplevel_app_id(fd, TOPLEVEL_ID, "thingos.themes");
    commit_surface(fd, SURFACE_ID);

    let mut pending =
        PendingSurface { serial: None, width: 460, height: 360, dirty: false, configured: false };
    let mut buffer: Option<BufferState> = None;
    let mut next_callback_id = 1000u32;
    let mut pending_frame_callbacks: Vec<u32> = Vec::new();
    let mut pointer = PointerState::default();
    let mut rx = Vec::new();
    let mut last_rendered_index = usize::MAX;

    info!("Ready");

    loop {
        let changed = read_events(
            fd,
            &mut rx,
            &mut pending,
            &mut pending_frame_callbacks,
            &mut pointer,
            &mut selected_index,
        );
        let current = current_theme();
        if current.name != active_theme.name {
            active_theme = current;
            selected_index = theme_index(active_theme);
        }

        if pending.configured && (pending.dirty || changed || selected_index != last_rendered_index)
        {
            last_rendered_index = selected_index;
            let buf = ensure_buffer(
                fd,
                SHM_ID,
                &mut buffer,
                SURFACE_ID + 100,
                pending.width,
                pending.height,
            );
            render_themes(
                buf,
                active_theme,
                selected_index,
                pointer.pressed_index,
                text_renderer.as_ref(),
            );
            if pending.dirty {
                ack_configure(fd, XDG_SURFACE_ID, pending.serial.unwrap_or(0));
                pending.dirty = false;
            }
            attach_buffer(fd, SURFACE_ID, buf.buffer_id);
            damage_surface(fd, SURFACE_ID, 0, 0, pending.width, pending.height);
            let cb_id = alloc_callback_id(&mut next_callback_id);
            request_frame(fd, SURFACE_ID, cb_id);
            pending_frame_callbacks.push(cb_id);
            commit_surface(fd, SURFACE_ID);
        }

        sleep_ms(IDLE_SLEEP_MS);
    }
}

fn read_events(
    fd: u32,
    rx: &mut Vec<u8>,
    pending: &mut PendingSurface,
    pending_frame_callbacks: &mut Vec<u32>,
    pointer: &mut PointerState,
    selected_index: &mut usize,
) -> bool {
    let mut pollfd = [PollHandle { handle: fd as i32, events: poll_flags::POLLIN, revents: 0 }];
    if !matches!(vfs_poll(&mut pollfd, 0), Ok(n) if n > 0)
        || (pollfd[0].revents & poll_flags::POLLIN) == 0
    {
        return false;
    }

    let mut changed = false;
    let mut in_buf = [0u8; 4096];
    let len = match vfs_read(fd, &mut in_buf) {
        Ok(n) if n > 0 => n,
        _ => return false,
    };
    rx.extend_from_slice(&in_buf[..len]);
    if rx.len() > 16 * 1024 {
        rx.clear();
        return true;
    }

    let mut offset = 0usize;
    while offset + 8 <= rx.len() {
        let (object_id, opcode, size) = decode_header(&rx[offset..]);
        if size < 8 {
            rx.clear();
            return true;
        }
        if offset + size as usize > rx.len() {
            break;
        }
        let payload = &rx[offset + 8..offset + size as usize];
        match (object_id, opcode) {
            (WM_BASE_ID, 0) if payload.len() >= 4 => {
                send_pong(fd, WM_BASE_ID, read_u32(payload, 0));
            }
            (XDG_SURFACE_ID, 0) if payload.len() >= 4 => {
                pending.serial = Some(read_u32(payload, 0));
                pending.dirty = true;
                pending.configured = true;
                changed = true;
            }
            (TOPLEVEL_ID, 0) if payload.len() >= 12 => {
                let width = read_i32(payload, 0);
                let height = read_i32(payload, 4);
                if width > 0 {
                    pending.width = width as u32;
                }
                if height > 0 {
                    pending.height = height as u32;
                }
                changed = true;
            }
            (TOPLEVEL_ID, 1) => {
                info!("Closed");
                exit(0);
            }
            (POINTER_ID, 2) if payload.len() >= 12 => {
                pointer.x = wl_fixed_to_i32(read_i32(payload, 4));
                pointer.y = wl_fixed_to_i32(read_i32(payload, 8));
            }
            (POINTER_ID, 3) if payload.len() >= 16 => {
                let button = read_u32(payload, 8);
                let pressed = read_u32(payload, 12) == 1;
                if button == 0x110 {
                    if pressed {
                        pointer.pressed_index = hit_test_theme(pointer.x, pointer.y);
                    } else if let Some(index) = pointer.pressed_index.take() {
                        if hit_test_theme(pointer.x, pointer.y) == Some(index) {
                            apply_theme_index(index);
                            *selected_index = index;
                        }
                    }
                    changed = true;
                }
            }
            (KEYBOARD_ID, 3) if payload.len() >= 16 => {
                if read_u32(payload, 12) == 1 {
                    match wayland_key_to_hid(read_u32(payload, 8)) {
                        Key::Up => {
                            *selected_index = selected_index.saturating_sub(1);
                            changed = true;
                        }
                        Key::Down | Key::Tab => {
                            let max = available_themes().len().saturating_sub(1);
                            *selected_index = (*selected_index + 1).min(max);
                            changed = true;
                        }
                        Key::Enter | Key::Space => {
                            apply_theme_index(*selected_index);
                            changed = true;
                        }
                        _ => {}
                    }
                }
            }
            (_, 0) if pending_frame_callbacks.iter().any(|&id| id == object_id) => {
                pending_frame_callbacks.retain(|&id| id != object_id);
            }
            _ => {}
        }
        offset += size as usize;
    }
    if offset > 0 {
        rx.drain(0..offset);
    }
    changed
}

fn apply_theme_index(index: usize) {
    if let Some(theme) = available_themes().get(index) {
        if write_theme_name(theme.name).is_ok() {
            info!("Selected theme {}", theme.name);
        } else {
            stem::warn!("Could not write theme selection");
        }
    }
}

fn hit_test_theme(x: i32, y: i32) -> Option<usize> {
    let item_x = 24;
    let item_w = 412;
    let item_h = 70;
    for index in 0..available_themes().len() {
        let item_y = 82 + index as i32 * 82;
        if x >= item_x && x < item_x + item_w && y >= item_y && y < item_y + item_h {
            return Some(index);
        }
    }
    None
}

fn render_themes(
    buffer: BufferState,
    active_theme: Theme,
    selected_index: usize,
    pressed_index: Option<usize>,
    text_renderer: Option<&TextRenderer>,
) {
    unsafe {
        let pixels = core::slice::from_raw_parts_mut(
            buffer.ptr as *mut u32,
            (buffer.width * buffer.height) as usize,
        );
        pixels.fill(active_theme.body_top);
        fill_rect(pixels, buffer.width, buffer.height, 0, 0, buffer.width as i32, 54, 0xEE10151C);
        draw_text(
            text_renderer,
            pixels,
            buffer.width,
            buffer.height,
            24,
            35,
            24.0,
            "Themes",
            active_theme.chrome_text,
        );
        draw_text(
            text_renderer,
            pixels,
            buffer.width,
            buffer.height,
            24,
            62,
            12.0,
            "Select a theme",
            active_theme.chrome_text_inactive,
        );

        for (index, theme) in available_themes().iter().copied().enumerate() {
            draw_theme_row(
                pixels,
                buffer.width,
                buffer.height,
                text_renderer,
                theme,
                index,
                index == selected_index,
                pressed_index == Some(index),
            );
        }
    }
}

fn draw_theme_row(
    pixels: &mut [u32],
    width: u32,
    height: u32,
    text_renderer: Option<&TextRenderer>,
    theme: Theme,
    index: usize,
    selected: bool,
    pressed: bool,
) {
    let x = 24;
    let y = 82 + index as i32 * 82;
    let w = 412;
    let h = 70;
    let bg =
        if selected { theme.frame_fill } else { soften_argb(theme.frame_fill, 0xFF000000, 64) };
    fill_rect(pixels, width, height, x, y, w, h, bg);
    stroke_rect(
        pixels,
        width,
        height,
        x,
        y,
        w,
        h,
        if selected { theme.focus_accent } else { theme.inner_stroke },
        if selected { 2 } else { 1 },
    );
    if pressed {
        fill_rect(pixels, width, height, x + 2, y + 2, w - 4, h - 4, 0x22000000);
    }

    fill_rect(pixels, width, height, x + 16, y + 16, 38, 38, theme.active.title_top);
    fill_rect(pixels, width, height, x + 58, y + 16, 38, 38, theme.focus_accent);
    fill_rect(pixels, width, height, x + 100, y + 16, 38, 38, theme.body_top);

    let label_color = if selected { theme.chrome_text } else { theme.chrome_text_inactive };
    draw_text(text_renderer, pixels, width, height, x + 154, y + 35, 18.0, theme.name, label_color);
    if selected {
        draw_text(
            text_renderer,
            pixels,
            width,
            height,
            x + 154,
            y + 55,
            11.0,
            "Active",
            theme.focus_accent,
        );
    }
}

fn current_theme() -> Theme {
    theme_by_name(&read_theme_name())
}

fn theme_index(theme: Theme) -> usize {
    available_themes().iter().position(|candidate| candidate.name == theme.name).unwrap_or(0)
}

fn read_theme_name() -> String {
    let Ok(fd) = vfs_open(THEME_PATH, vfs_flags::O_RDONLY) else {
        return String::from(default_theme().name);
    };
    let mut buf = [0u8; 128];
    let name = match vfs_read(fd, &mut buf) {
        Ok(n) if n > 0 => match core::str::from_utf8(&buf[..n]) {
            Ok(text) => String::from(text.trim()),
            Err(_) => String::from(default_theme().name),
        },
        _ => String::from(default_theme().name),
    };
    let _ = vfs_close(fd);
    if name.is_empty() { String::from(default_theme().name) } else { name }
}

fn write_theme_name(name: &str) -> Result<(), &'static str> {
    let _ = vfs_mkdir("/session");
    let _ = vfs_mkdir("/session/desktop");
    let fd = vfs_open(THEME_PATH, vfs_flags::O_WRONLY | vfs_flags::O_CREAT | vfs_flags::O_TRUNC)
        .map_err(|_| "could not open /session/desktop/theme")?;
    let mut line = name.to_string();
    line.push('\n');
    let ok = vfs_write(fd, line.as_bytes()).is_ok();
    let _ = vfs_close(fd);
    if ok { Ok(()) } else { Err("could not write /session/desktop/theme") }
}

fn get_args() -> Vec<String> {
    let mut len = 0;
    if let Ok(l) = argv_get(&mut []) {
        len = l;
    }
    if len == 0 {
        return Vec::new();
    }
    let mut buf = alloc::vec![0u8; len];
    if argv_get(&mut buf).is_err() {
        return Vec::new();
    }
    stem::utils::parse_argv(&buf)
        .into_iter()
        .filter_map(|arg| core::str::from_utf8(arg).ok().map(String::from))
        .collect()
}

fn print(args: &str) {
    let _ = vfs_write(1, args.as_bytes());
}

fn print_err(args: &str) {
    let _ = vfs_write(2, args.as_bytes());
}

fn connect_wayland() -> u32 {
    loop {
        let fd = match socket(AF_UNIX, SOCK_STREAM, 0) {
            Ok(fd) => fd,
            Err(e) => {
                stem::error!("Socket failed: {:?}", e);
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

fn read_initial_globals(fd: u32) {
    let mut buf = [0u8; 512];
    let _ = vfs_read(fd, &mut buf);
}

fn ensure_buffer(
    fd: u32,
    shm_id: u32,
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
    let fd_buf = memfd_create("themes.buffer", size as usize).expect("create memfd");

    use abi::vm::{VmBacking, VmMapReq, VmProt};
    let req = VmMapReq {
        addr_hint: 0,
        len: size as usize,
        prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
        flags: abi::vm::VmMapFlags::empty(),
        backing: VmBacking::File { thing: fd_buf, offset: 0 },
    };
    let resp = vm_map(&req).expect("map memfd");
    let ptr = resp.addr as *mut u8;

    let pool_id = base_id;
    let buffer_id = base_id + 1;
    create_pool(fd, shm_id, pool_id, fd_buf, size);
    create_buffer(fd, pool_id, buffer_id, width, height, stride);

    let out = BufferState {
        _pool_id: pool_id,
        buffer_id,
        _handle: fd_buf,
        ptr,
        width,
        height,
        _stride: stride,
    };
    *current = Some(out);
    out
}

fn fill_rect(
    pixels: &mut [u32],
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: u32,
) {
    let x0 = x.max(0) as u32;
    let y0 = y.max(0) as u32;
    let x1 = (x + w).min(width as i32).max(0) as u32;
    let y1 = (y + h).min(height as i32).max(0) as u32;
    for yy in y0..y1 {
        let row = (yy * width) as usize;
        for xx in x0..x1 {
            pixels[row + xx as usize] = color;
        }
    }
}

fn stroke_rect(
    pixels: &mut [u32],
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: u32,
    thickness: i32,
) {
    for i in 0..thickness {
        fill_rect(pixels, width, height, x + i, y + i, w - i * 2, 1, color);
        fill_rect(pixels, width, height, x + i, y + h - 1 - i, w - i * 2, 1, color);
        fill_rect(pixels, width, height, x + i, y + i, 1, h - i * 2, color);
        fill_rect(pixels, width, height, x + w - 1 - i, y + i, 1, h - i * 2, color);
    }
}

fn soften_argb(a: u32, b: u32, amount: u32) -> u32 {
    let amount = amount.min(255);
    let inv = 255 - amount;
    let aa = (a >> 24) & 0xFF;
    let ar = (a >> 16) & 0xFF;
    let ag = (a >> 8) & 0xFF;
    let ab = a & 0xFF;
    let ba = (b >> 24) & 0xFF;
    let br = (b >> 16) & 0xFF;
    let bg = (b >> 8) & 0xFF;
    let bb = b & 0xFF;
    ((aa * inv + ba * amount) / 255) << 24
        | ((ar * inv + br * amount) / 255) << 16
        | ((ag * inv + bg * amount) / 255) << 8
        | ((ab * inv + bb * amount) / 255)
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
        stem::warn!("Text too long for renderer");
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
        stem::warn!("Text render failed: {}", rc);
    }
}

fn load_text_renderer() -> Option<TextRenderer> {
    let handle = dlopen_str(PISTIL_PATH, RTLD_NOW);
    if handle.is_null() {
        log_dlerror("Could not load /lib/libpistil.so");
        return None;
    }

    let sym_text = dlsym_bytes(handle, DRAW_TEXT_SYMBOL);
    if sym_text.is_null() {
        log_dlerror("Could not resolve pistil_draw_text");
        return None;
    }

    let draw_text: DrawTextFn = unsafe { core::mem::transmute(sym_text) };
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
        stem::warn!("Write failed: {:?}", e);
    }
}

fn send_request_with_fds(fd: u32, buf: &[u8], fds: &[u32]) {
    if fds.is_empty() {
        send_request(fd, buf);
        return;
    }
    if let Err(e) = sendmsg(fd, buf, fds) {
        stem::warn!("sendmsg failed: {:?}", e);
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

fn wl_fixed_to_i32(value: i32) -> i32 {
    value / 256
}

fn wayland_key_to_hid(key: u32) -> Key {
    match key {
        1 => Key::Escape,
        15 => Key::Tab,
        28 => Key::Enter,
        57 => Key::Space,
        103 => Key::Up,
        108 => Key::Down,
        _ => Key::Unknown,
    }
}
