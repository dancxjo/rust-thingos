#![no_std]
#![no_main]
extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::convert::TryInto;

use libdl::{RTLD_NOW, dlerror, dlopen_str, dlsym_bytes};
use stem::abi::syscall::{PollHandle, poll_flags};
use stem::info;
use stem::syscall::socket::{connect, sendmsg, socket};
use stem::syscall::socket_domain::AF_UNIX;
use stem::syscall::socket_type::SOCK_STREAM;
use stem::syscall::{
    exit, get_tid, memfd_create, set_priority, sleep_ms, vfs_close, vfs_poll, vfs_read, vfs_write,
    vm_map,
};

const REGISTRY_ID: u32 = 2;
const COMPOSITOR_ID: u32 = 3;
const SHM_ID: u32 = 4;
const WM_BASE_ID: u32 = 5;

const SURFACE_ID: u32 = 10;
const XDG_SURFACE_ID: u32 = 11;
const TOPLEVEL_ID: u32 = 12;

const PISTIL_PATH: &str = "/lib/libpistil.so";
const DRAW_DSEG7_TEXT_SYMBOL: &[u8] = b"pistil_draw_dseg7_text";
const DRAW_TEXT_SYMBOL: &[u8] = b"pistil_draw_text";
const DSEG7_FONT_PATH: &str = "/share/fonts/DSEG7Classic-Regular.ttf";
const SERIAL_TICK_INTERVAL_NS: u64 = 37_000_000_000;
const TZ_REFRESH_INTERVAL_NS: u64 = 60_000_000_000;
const CLOCK_PRIORITY_LOW: usize = 1;
const IDLE_SLEEP_MS: u64 = 250;
const CLOCK_GOLD: u32 = 0xFFFFB900;
const CLOCK_GOLD_GHOST: u32 = 0x12FFB900;
const CLOCK_GOLD_GLOW: u32 = 0x30FFB900;
const CLOCK_GREEN: u32 = 0xFF06D6A0;

type DrawTextFn = extern "C" fn(*const u8, *mut u32, u32, u32, u32, i32, i32, f32, u32) -> i32;

struct TextRenderer {
    _handle: *mut core::ffi::c_void,
    draw_dseg7_text: DrawTextFn,
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
    stride: u32,
}

struct PendingSurface {
    serial: Option<u32>,
    width: u32,
    height: u32,
    dirty: bool,
    configured: bool,
}

#[derive(Debug, Clone, Copy)]
struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

#[stem::main]
fn main(_arg: usize) -> ! {
    lower_clock_priority();

    let fd = connect_wayland();
    let text_renderer = load_text_renderer();

    send_get_registry(fd, REGISTRY_ID);
    read_initial_globals(fd);

    bind_global(fd, 1, "wl_compositor", 4, COMPOSITOR_ID);
    bind_global(fd, 2, "wl_shm", 1, SHM_ID);
    bind_global(fd, 3, "xdg_wm_base", 1, WM_BASE_ID);

    create_surface(fd, COMPOSITOR_ID, SURFACE_ID);
    get_xdg_surface(fd, WM_BASE_ID, XDG_SURFACE_ID, SURFACE_ID);
    get_toplevel(fd, XDG_SURFACE_ID, TOPLEVEL_ID);
    set_toplevel_title(fd, TOPLEVEL_ID, "Clock");
    set_toplevel_app_id(fd, TOPLEVEL_ID, "thingos.clock");
    commit_surface(fd, SURFACE_ID);

    let mut pending =
        PendingSurface { serial: None, width: 520, height: 220, dirty: false, configured: false };
    let mut buffer: Option<BufferState> = None;
    let mut next_callback_id = 1000u32;
    let mut pending_frame_callbacks: Vec<u32> = Vec::new();
    let mut tz_offset = get_tz_offset();
    let mut last_tz_refresh_ns = stem::time::monotonic_ns();
    let mut last_serial_tick_ns = 0u64;
    let mut last_time = String::new();
    let mut last_date = String::new();
    let mut last_am_pm = String::new();

    loop {
        read_events(fd, &mut pending, &mut pending_frame_callbacks);

        let now_ns = stem::time::monotonic_ns();
        if now_ns.saturating_sub(last_tz_refresh_ns) >= TZ_REFRESH_INTERVAL_NS {
            tz_offset = get_tz_offset();
            last_tz_refresh_ns = now_ns;
        }

        let realtime = local_datetime(tz_offset);
        let (time_text, date_text, am_pm_text) = match realtime {
            Some((dt, _, _)) => {
                let hour12 = if dt.hour == 0 {
                    12
                } else if dt.hour > 12 {
                    dt.hour - 12
                } else {
                    dt.hour
                };
                let am_pm = if dt.hour < 12 { "AM" } else { "PM" };
                (
                    format!("{:02}:{:02}", hour12, dt.minute),
                    format!("{:04}-{:02}-{:02} UTC{:+}", dt.year, dt.month, dt.day, tz_offset),
                    String::from(am_pm),
                )
            }
            None => ("00:00".into(), format!("WAITING FOR RTC UTC{:+}", tz_offset), "--".into()),
        };

        if now_ns.saturating_sub(last_serial_tick_ns) >= SERIAL_TICK_INTERVAL_NS {
            match realtime {
                Some((dt, unix_secs, nanos)) => {
                    info!(
                        "clock: tick local={:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC{:+} system_unix={}.{:09}",
                        dt.year,
                        dt.month,
                        dt.day,
                        dt.hour,
                        dt.minute,
                        dt.second,
                        tz_offset,
                        unix_secs,
                        nanos
                    );
                }
                None => {
                    info!("clock: waiting for system clock anchor");
                }
            }
            last_serial_tick_ns = now_ns;
        }

        let time_changed =
            time_text != last_time || date_text != last_date || am_pm_text != last_am_pm;
        if time_changed {
            last_time = time_text;
            last_date = date_text;
            last_am_pm = am_pm_text;
        }

        let should_render = pending.configured && (pending.dirty || time_changed);
        if should_render {
            let buf = ensure_buffer(
                fd,
                SHM_ID,
                &mut buffer,
                SURFACE_ID + 100,
                pending.width,
                pending.height,
            );
            render_clock(buf, &last_time, &last_date, &last_am_pm, text_renderer.as_ref());
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

fn lower_clock_priority() {
    match get_tid().and_then(|tid| set_priority(tid, CLOCK_PRIORITY_LOW).map(|_| tid)) {
        Ok(tid) => info!("clock: running at low scheduler priority tid={}", tid),
        Err(e) => stem::warn!("clock: failed to lower scheduler priority: {:?}", e),
    }
}

fn connect_wayland() -> u32 {
    loop {
        let fd = match socket(AF_UNIX, SOCK_STREAM, 0) {
            Ok(fd) => fd,
            Err(e) => {
                stem::error!("clock: socket(AF_UNIX) failed: {:?}", e);
                sleep_ms(250);
                continue;
            }
        };

        match connect(fd, "/run/wayland-0") {
            Ok(()) => {
                info!("clock: connected to /run/wayland-0");
                return fd;
            }
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

fn read_events(fd: u32, pending: &mut PendingSurface, pending_frame_callbacks: &mut Vec<u32>) {
    let mut pollfd = [PollHandle { handle: fd as i32, events: poll_flags::POLLIN, revents: 0 }];
    if !matches!(vfs_poll(&mut pollfd, 0), Ok(n) if n > 0)
        || (pollfd[0].revents & poll_flags::POLLIN) == 0
    {
        return;
    }

    let mut in_buf = [0u8; 4096];
    let len = match vfs_read(fd, &mut in_buf) {
        Ok(n) if n > 0 => n,
        _ => return,
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
            (XDG_SURFACE_ID, 0) if payload.len() >= 4 => {
                pending.serial = Some(read_u32(payload, 0));
                pending.dirty = true;
                pending.configured = true;
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
            }
            (TOPLEVEL_ID, 1) => {
                info!("clock: compositor requested close; exiting");
                stem::syscall::vfs_write(1, b"clock: explicit exit(0) call\n").ok();
                exit(0);
            }
            (_, 0) if pending_frame_callbacks.iter().any(|&id| id == object_id) => {
                pending_frame_callbacks.retain(|&id| id != object_id);
            }
            _ => {}
        }
        offset += size as usize;
    }
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
    let fd_buf = memfd_create("clock.buffer", size as usize).expect("create memfd");

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

    let out =
        BufferState { _pool_id: pool_id, buffer_id, _handle: fd_buf, ptr, width, height, stride };
    *current = Some(out);
    out
}

fn render_clock(
    buffer: BufferState,
    time_text: &str,
    date_text: &str,
    am_pm: &str,
    text_renderer: Option<&TextRenderer>,
) {
    unsafe {
        let pixels = core::slice::from_raw_parts_mut(
            buffer.ptr as *mut u32,
            (buffer.width * buffer.height) as usize,
        );
        pixels.fill(0);

        let px_size = (buffer.width as f32 / 8.8).clamp(42.0, 50.0);
        let estimated_w = (time_text.len() as f32 * px_size * 0.55) as i32;
        let text_x = ((buffer.width as i32 - estimated_w) / 2).max(12);
        let body_top = 36i32;
        let body_h = buffer.height.saturating_sub(body_top as u32).saturating_sub(10);
        let text_y = body_top + ((body_h as f32 * 0.58) as i32);

        draw_dseg7_text(
            text_renderer,
            pixels,
            buffer.width,
            buffer.height,
            text_x,
            text_y,
            px_size,
            "88:88",
            CLOCK_GOLD_GHOST,
        );
        for (dx, dy, color) in [
            (-1, 0, CLOCK_GOLD_GLOW),
            (1, 0, CLOCK_GOLD_GLOW),
            (0, -1, CLOCK_GOLD_GLOW),
            (0, 1, CLOCK_GOLD_GLOW),
        ] {
            draw_dseg7_text(
                text_renderer,
                pixels,
                buffer.width,
                buffer.height,
                text_x + dx,
                text_y + dy,
                px_size,
                time_text,
                color,
            );
        }
        draw_dseg7_text(
            text_renderer,
            pixels,
            buffer.width,
            buffer.height,
            text_x,
            text_y,
            px_size,
            time_text,
            CLOCK_GOLD,
        );

        let am_pm_x = text_x + estimated_w + 12;
        draw_generic_text(
            text_renderer,
            pixels,
            buffer.width,
            buffer.height,
            am_pm_x,
            text_y,
            20.0,
            am_pm,
            CLOCK_GREEN,
        );

        let dot_y = body_top + 9;
        fill_rect(pixels, buffer.width, buffer.height, 22, dot_y, 4, 4, CLOCK_GOLD);
        fill_rect(
            pixels,
            buffer.width,
            buffer.height,
            buffer.width as i32 - 26,
            dot_y,
            4,
            4,
            CLOCK_GOLD,
        );
        draw_generic_text(
            text_renderer,
            pixels,
            buffer.width,
            buffer.height,
            18,
            body_top + 26,
            20.0,
            "ALARM",
            0xFF586E75,
        );
        draw_generic_text(
            text_renderer,
            pixels,
            buffer.width,
            buffer.height,
            16,
            buffer.height as i32 - 18,
            20.0,
            date_text,
            0xD1586E75,
        );
    }
}

fn fill_rect(
    pixels: &mut [u32],
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    rect_w: u32,
    rect_h: u32,
    color: u32,
) {
    let x0 = x.max(0) as u32;
    let y0 = y.max(0) as u32;
    let x1 = (x + rect_w as i32).max(0).min(width as i32) as u32;
    let y1 = (y + rect_h as i32).max(0).min(height as i32) as u32;
    for py in y0..y1 {
        let row = py as usize * width as usize;
        for px in x0..x1 {
            pixels[row + px as usize] = color;
        }
    }
}

fn draw_dseg7_text(
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
        stem::warn!("clock: text too long for pistil text call");
        return;
    }
    text_c[..bytes.len()].copy_from_slice(bytes);

    let rc = (renderer.draw_dseg7_text)(
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
        stem::warn!("clock: pistil_draw_dseg7_text failed: {}", rc);
    }
}

fn draw_generic_text(
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
        stem::warn!("clock: text too long for pistil text call");
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
        stem::warn!("clock: pistil_draw_text failed: {}", rc);
    }
}

fn load_text_renderer() -> Option<TextRenderer> {
    let handle = dlopen_str(PISTIL_PATH, RTLD_NOW);
    if handle.is_null() {
        log_dlerror("clock: failed to load /lib/libpistil.so");
        return None;
    }

    let sym_dseg7 = dlsym_bytes(handle, DRAW_DSEG7_TEXT_SYMBOL);
    if sym_dseg7.is_null() {
        log_dlerror("clock: failed to resolve pistil_draw_dseg7_text");
        return None;
    }

    let sym_text = dlsym_bytes(handle, DRAW_TEXT_SYMBOL);
    if sym_text.is_null() {
        log_dlerror("clock: failed to resolve pistil_draw_text");
        return None;
    }

    let draw_dseg7_text: DrawTextFn = unsafe { core::mem::transmute(sym_dseg7) };
    let draw_text: DrawTextFn = unsafe { core::mem::transmute(sym_text) };
    info!("clock: pistil DSEG7 text renderer loaded with {}", DSEG7_FONT_PATH);
    info!("clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf");
    Some(TextRenderer { _handle: handle, draw_dseg7_text, draw_text })
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

fn local_datetime(tz_offset_hours: i32) -> Option<(DateTime, u64, u32)> {
    let spec = stem::time::clock_now(stem::time::ClockId::Realtime)?;
    let unix_secs = spec.secs;
    let local_secs = if tz_offset_hours >= 0 {
        unix_secs.saturating_add(tz_offset_hours as u64 * 3600)
    } else {
        unix_secs.saturating_sub(tz_offset_hours.abs() as u64 * 3600)
    };
    Some((unix_to_datetime(local_secs), unix_secs, spec.nanos))
}

fn get_tz_offset() -> i32 {
    let mut buf = [0u8; 1024];
    let Ok(fd) =
        stem::syscall::vfs::vfs_open("/etc/locale.conf", stem::syscall::vfs_flags::O_RDONLY)
    else {
        return 0;
    };

    let offset = match vfs_read(fd, &mut buf) {
        Ok(n) => core::str::from_utf8(&buf[..n]).ok().and_then(parse_tz_offset).unwrap_or(0),
        Err(_) => 0,
    };
    let _ = vfs_close(fd);
    offset
}

fn parse_tz_offset(content: &str) -> Option<i32> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(value) = line.strip_prefix("TZ_OFFSET=") {
            return value.trim().parse::<i32>().ok();
        }
        if let Some(value) = line.strip_prefix("TZ=") {
            return value.trim().parse::<i32>().ok();
        }
    }
    None
}

fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

fn days_in_month(month: u8, year: u16) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

fn unix_to_datetime(seconds: u64) -> DateTime {
    let mut remaining = seconds;

    let second = (remaining % 60) as u8;
    remaining /= 60;
    let minute = (remaining % 60) as u8;
    remaining /= 60;
    let hour = (remaining % 24) as u8;
    remaining /= 24;

    let mut year = 1970u16;
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining < days_in_year as u64 {
            break;
        }
        remaining -= days_in_year as u64;
        year += 1;
    }

    let mut month = 1u8;
    loop {
        let dim = days_in_month(month, year);
        if remaining < dim as u64 {
            break;
        }
        remaining -= dim as u64;
        month += 1;
    }

    DateTime { year, month, day: (remaining + 1) as u8, hour, minute, second }
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
        stem::warn!("clock: write failed: {:?}", e);
    }
}

fn send_request_with_fds(fd: u32, buf: &[u8], fds: &[u32]) {
    if fds.is_empty() {
        send_request(fd, buf);
        return;
    }
    if let Err(e) = sendmsg(fd, buf, fds) {
        stem::warn!("clock: sendmsg failed: {:?}", e);
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
