#![no_std]
#![no_main]
extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::convert::TryInto;

use libdl::{RTLD_NOW, dlerror, dlopen_str, dlsym_bytes};
use petals::{
    AlignItems, AvailableSpace, Clock, ClockState, Color, FlexDirection, JustifyContent,
    ResolvedStyle, Size, Theme, default_theme, theme_by_name,
};
use stem::application::{
    AppAction, Application, ApplicationContext, ServiceLooper, run_application,
};
use stem::info;
use stem::service_loop::ServiceEvent;
use stem::syscall::socket::{connect, sendmsg, socket};
use stem::syscall::socket_domain::AF_UNIX;
use stem::syscall::socket_type::SOCK_STREAM;
use stem::syscall::{
    get_tid, memfd_create, set_priority, sleep_ms, vfs_close, vfs_read, vfs_write, vm_map,
};
use stem::time::Duration;
use stem::wait_set::WaitToken;

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
const DSEG7_FONT_PATH: &str = "/public/fonts/DSEG7Classic-Regular.ttf";
const THEME_PATH: &str = "/session/desktop/theme";
const SERIAL_TICK_INTERVAL_NS: u64 = 37_000_000_000;
const TZ_REFRESH_INTERVAL_NS: u64 = 60_000_000_000;
const CLOCK_PRIORITY_LOW: usize = 1;
const IDLE_SLEEP_MS: u64 = 250;
type DrawTextFn = extern "C" fn(*const u8, *mut u32, u32, u32, u32, i32, i32, f32, u32) -> i32;

struct TextRenderer {
    _handle: *mut core::ffi::c_void,
    _draw_dseg7_text: DrawTextFn,
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
    run_application::<ClockApp>()
}

struct ClockApp {
    fd: u32,
    wayland_token: WaitToken,
    text_renderer: Option<TextRenderer>,
    clock: Clock,
    theme_name: String,
    theme: Theme,
    pending: PendingSurface,
    buffer: Option<BufferState>,
    next_callback_id: u32,
    pending_frame_callbacks: Vec<u32>,
    tz_offset: i32,
    last_tz_refresh_ns: u64,
    last_serial_tick_ns: u64,
    last_state: Option<ClockState>,
    last_waiting_text: String,
    petal_render_logged: bool,
}

impl Application for ClockApp {
    const NAME: &'static str = "clock";

    fn init(
        ctx: &mut ApplicationContext,
        looper: &mut ServiceLooper,
    ) -> Result<Self, stem::errors::Errno> {
        lower_clock_priority();

        let fd = connect_wayland();
        let text_renderer = load_text_renderer();
        let clock = Clock::new();
        let theme_name = read_theme_name();
        let theme = theme_by_name(&theme_name);
        let wayland_token = looper.add_fd_readable(fd)?;

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

        ctx.register_window("clock toplevel", move || close_toplevel(fd));
        ctx.register_cleanup("wayland fd", move || {
            let _ = vfs_close(fd);
        });

        Ok(Self {
            fd,
            wayland_token,
            text_renderer,
            clock,
            theme_name,
            theme,
            pending: PendingSurface {
                serial: None,
                width: 520,
                height: 220,
                dirty: false,
                configured: false,
            },
            buffer: None,
            next_callback_id: 1000,
            pending_frame_callbacks: Vec::new(),
            tz_offset: get_tz_offset(),
            last_tz_refresh_ns: stem::time::monotonic_ns(),
            last_serial_tick_ns: 0,
            last_state: None,
            last_waiting_text: String::new(),
            petal_render_logged: false,
        })
    }

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(IDLE_SLEEP_MS))
    }

    fn handle_event(
        &mut self,
        _ctx: &mut ApplicationContext,
        event: ServiceEvent<'_>,
    ) -> AppAction {
        match event {
            ServiceEvent::Ready { token, event }
                if token == self.wayland_token && event.is_readable() =>
            {
                let mut quit = false;
                if read_events(
                    self.fd,
                    &mut self.pending,
                    &mut self.pending_frame_callbacks,
                    &mut quit,
                ) {
                    self.tick_and_render();
                }
                if quit {
                    return AppAction::Quit;
                }
            }
            ServiceEvent::Ready { token, event }
                if token == self.wayland_token && (event.is_hangup() || event.is_error()) =>
            {
                return AppAction::Quit;
            }
            ServiceEvent::Timeout => self.tick_and_render(),
            ServiceEvent::Message { .. } | ServiceEvent::Ready { .. } => {}
            ServiceEvent::InboxClosed => return AppAction::Quit,
        }
        AppAction::Continue
    }
}

impl ClockApp {
    fn tick_and_render(&mut self) {
        let now_ns = stem::time::monotonic_ns();
        if now_ns.saturating_sub(self.last_tz_refresh_ns) >= TZ_REFRESH_INTERVAL_NS {
            self.tz_offset = get_tz_offset();
            self.last_tz_refresh_ns = now_ns;
        }

        let realtime = local_datetime(self.tz_offset);
        let (clock_state, waiting_text) = match realtime {
            Some((dt, _, _)) => (
                Some(self.clock.update_from_parts(dt.year, dt.month, dt.day, dt.hour, dt.minute)),
                String::new(),
            ),
            None => (None, format!("Waiting for RTC UTC{:+}", self.tz_offset)),
        };

        if now_ns.saturating_sub(self.last_serial_tick_ns) >= SERIAL_TICK_INTERVAL_NS {
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
                        self.tz_offset,
                        unix_secs,
                        nanos
                    );
                }
                None => {
                    info!("clock: waiting for system clock anchor");
                }
            }
            self.last_serial_tick_ns = now_ns;
        }

        let time_changed = clock_state != self.last_state || waiting_text != self.last_waiting_text;
        let theme_changed = refresh_theme(&mut self.theme_name, &mut self.theme);
        if time_changed {
            self.last_state = clock_state;
            self.last_waiting_text = waiting_text;
        }

        let should_render =
            self.pending.configured && (self.pending.dirty || time_changed || theme_changed);
        if should_render {
            let buf = ensure_buffer(
                self.fd,
                SHM_ID,
                &mut self.buffer,
                SURFACE_ID + 100,
                self.pending.width,
                self.pending.height,
            );
            let petal_rendered = render_clock(
                buf,
                &self.clock,
                self.last_state.as_ref(),
                &self.last_waiting_text,
                self.theme,
                self.text_renderer.as_ref(),
            );
            if petal_rendered && !self.petal_render_logged {
                info!("clock: petal perspective rendering date+time");
                self.petal_render_logged = true;
            }
            if self.pending.dirty {
                ack_configure(self.fd, XDG_SURFACE_ID, self.pending.serial.unwrap_or(0));
                self.pending.dirty = false;
            }
            attach_buffer(self.fd, SURFACE_ID, buf.buffer_id);
            damage_surface(self.fd, SURFACE_ID, 0, 0, self.pending.width, self.pending.height);
            let cb_id = alloc_callback_id(&mut self.next_callback_id);
            request_frame(self.fd, SURFACE_ID, cb_id);
            self.pending_frame_callbacks.push(cb_id);
            commit_surface(self.fd, SURFACE_ID);
        }
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

fn read_events(
    fd: u32,
    pending: &mut PendingSurface,
    pending_frame_callbacks: &mut Vec<u32>,
    quit: &mut bool,
) -> bool {
    let mut changed = false;
    let mut in_buf = [0u8; 4096];
    let len = match vfs_read(fd, &mut in_buf) {
        Ok(n) if n > 0 => n,
        _ => return false,
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
                info!("clock: compositor requested close; exiting");
                stem::syscall::vfs_write(1, b"clock: explicit exit(0) call\n").ok();
                *quit = true;
                changed = true;
            }
            (_, 0) if pending_frame_callbacks.iter().any(|&id| id == object_id) => {
                pending_frame_callbacks.retain(|&id| id != object_id);
            }
            _ => {}
        }
        offset += size as usize;
    }
    changed
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

fn render_clock(
    buffer: BufferState,
    clock: &Clock,
    state: Option<&ClockState>,
    waiting_text: &str,
    theme: Theme,
    text_renderer: Option<&TextRenderer>,
) -> bool {
    unsafe {
        let pixels = core::slice::from_raw_parts_mut(
            buffer.ptr as *mut u32,
            (buffer.width * buffer.height) as usize,
        );
        pixels.fill(0);

        let Some(state) = state else {
            draw_generic_text(
                text_renderer,
                pixels,
                buffer.width,
                buffer.height,
                18,
                buffer.height as i32 / 2,
                18.0,
                waiting_text,
                theme.chrome_text_inactive,
            );
            return false;
        };

        if render_clock_petal(buffer, pixels, clock, state, theme, text_renderer).is_ok() {
            return true;
        }

        let time_text = clock.time_text(state);
        let date_text = clock.date_text(state);
        let px_size = 32.0;
        let estimated_w = (time_text.len() as f32 * px_size * 0.58) as i32;
        let text_x = ((buffer.width as i32 - estimated_w) / 2).max(12);
        let text_y = (buffer.height as i32 / 2).saturating_sub(4);

        draw_generic_text(
            text_renderer,
            pixels,
            buffer.width,
            buffer.height,
            text_x,
            text_y,
            px_size,
            &time_text,
            0xFFF2EFE8,
        );
        draw_generic_text(
            text_renderer,
            pixels,
            buffer.width,
            buffer.height,
            text_x + estimated_w + 10,
            text_y,
            12.0,
            clock.am_pm_text(state),
            0xFFB0B8B6,
        );
        draw_generic_text(
            text_renderer,
            pixels,
            buffer.width,
            buffer.height,
            text_x,
            text_y + 26,
            14.0,
            &date_text,
            0xFF8E9895,
        );
    }
    false
}

fn render_clock_petal(
    buffer: BufferState,
    pixels: &mut [u32],
    clock: &Clock,
    state: &ClockState,
    theme: Theme,
    text_renderer: Option<&TextRenderer>,
) -> Result<(), ()> {
    let (mut tree, nodes) = clock.build_tree_for_theme(state, theme).map_err(|_| ())?;
    tree.apply_style(
        tree.root(),
        ResolvedStyle {
            width: Some(buffer.width as f32),
            height: Some(buffer.height as f32),
            flex_direction: Some(FlexDirection::Column),
            justify_content: Some(JustifyContent::Center),
            align_items: Some(AlignItems::Center),
            ..ResolvedStyle::default()
        },
    )
    .map_err(|_| ())?;
    tree.compute_layout(Size {
        width: AvailableSpace::Definite(buffer.width as f32),
        height: AvailableSpace::Definite(buffer.height as f32),
    })
    .map_err(|_| ())?;

    let time = tree.node(nodes.time).ok_or(())?;
    let am_pm = tree.node(nodes.am_pm).ok_or(())?;
    let time_text = clock.time_text(state);
    let time_box = tree.global_layout_box(nodes.time).map_err(|_| ())?;
    let am_pm_box = tree.global_layout_box(nodes.am_pm).map_err(|_| ())?;

    draw_generic_text(
        text_renderer,
        pixels,
        buffer.width,
        buffer.height,
        time_box.x as i32,
        (time_box.y + time.style.font_size.unwrap_or(32.0)) as i32,
        time.style.font_size.unwrap_or(32.0),
        &time_text,
        argb(time.style.color.unwrap_or(Color::rgb(242, 239, 232))),
    );
    draw_generic_text(
        text_renderer,
        pixels,
        buffer.width,
        buffer.height,
        am_pm_box.x as i32,
        (am_pm_box.y + am_pm.style.font_size.unwrap_or(12.0)) as i32,
        am_pm.style.font_size.unwrap_or(12.0),
        clock.am_pm_text(state),
        argb(am_pm.style.color.unwrap_or(Color::rgb(176, 184, 182))),
    );

    if let Some(date_id) = nodes.date {
        let date = tree.node(date_id).ok_or(())?;
        let date_text = clock.date_text(state);
        let date_box = tree.global_layout_box(date_id).map_err(|_| ())?;
        draw_generic_text(
            text_renderer,
            pixels,
            buffer.width,
            buffer.height,
            date_box.x as i32,
            (date_box.y + date.style.font_size.unwrap_or(14.0)) as i32,
            date.style.font_size.unwrap_or(14.0),
            &date_text,
            argb(date.style.color.unwrap_or(Color::rgb(142, 152, 149))),
        );
    }

    Ok(())
}

fn read_theme_name() -> String {
    let Ok(fd) = stem::syscall::vfs::vfs_open(THEME_PATH, abi::syscall::vfs_flags::O_RDONLY) else {
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

fn refresh_theme(current_name: &mut String, theme: &mut Theme) -> bool {
    let next_name = read_theme_name();
    let next_theme = theme_by_name(&next_name);
    if next_theme.name == theme.name && next_name == *current_name {
        return false;
    }
    stem::info!("clock: applying theme {}", next_theme.name);
    *current_name = next_name;
    *theme = next_theme;
    true
}

fn argb(color: Color) -> u32 {
    ((color.a as u32) << 24) | ((color.r as u32) << 16) | ((color.g as u32) << 8) | color.b as u32
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
    info!("clock: pistil generic text renderer loaded with /public/fonts/Inter-Regular.ttf");
    Some(TextRenderer { _handle: handle, _draw_dseg7_text: draw_dseg7_text, draw_text })
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

fn close_toplevel(fd: u32) {
    destroy_object(fd, TOPLEVEL_ID);
    destroy_object(fd, XDG_SURFACE_ID);
    destroy_object(fd, SURFACE_ID);
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
