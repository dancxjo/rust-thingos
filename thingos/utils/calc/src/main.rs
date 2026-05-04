#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::convert::TryInto;

use abi::hid::Key;
use libdl::{RTLD_NOW, dlerror, dlopen_str, dlsym_bytes};
use petals::{
    AlignItems, AvailableSpace, CalcInput, CalcKeyNode, CalcState, Calculator, Color,
    FlexDirection, JustifyContent, ResolvedStyle, Size, State, Theme, default_theme, theme_by_name,
};
use stem::application::{
    AppAction, Application, ApplicationContext, ServiceLooper, run_application,
};
use stem::info;
use stem::service_loop::ServiceEvent;
use stem::syscall::socket::{connect, sendmsg, socket};
use stem::syscall::socket_domain::AF_UNIX;
use stem::syscall::socket_type::SOCK_STREAM;
use stem::syscall::{memfd_create, sleep_ms, vfs_close, vfs_read, vfs_write, vm_map};
use stem::time::Duration;
use stem::wait_set::WaitToken;

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

const PISTIL_PATH: &str = "/lib/libpistil.so";
const DRAW_TEXT_SYMBOL: &[u8] = b"pistil_draw_text";
const THEME_PATH: &str = "/session/desktop/theme";
const IDLE_SLEEP_MS: u64 = 16;
const HOLD_DELAY_NS: u64 = 400_000_000;
const HOLD_REPEAT_NS: u64 = 50_000_000;

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
struct KeyboardState {
    shift: bool,
}

#[derive(Default)]
struct PointerState {
    x: i32,
    y: i32,
    pressed_key: Option<usize>,
    pressed_at_ns: u64,
    last_repeat_ns: u64,
    repeated: bool,
}

#[stem::main]
fn main(_arg: usize) -> ! {
    run_application::<CalcApp>()
}

struct CalcApp {
    fd: u32,
    wayland_token: WaitToken,
    text_renderer: Option<TextRenderer>,
    calc: Calculator,
    state: CalcState,
    theme_name: String,
    theme: Theme,
    pending: PendingSurface,
    buffer: Option<BufferState>,
    next_callback_id: u32,
    pending_frame_callbacks: Vec<u32>,
    keyboard: KeyboardState,
    pointer: PointerState,
    rx: Vec<u8>,
    last_rendered_expression: String,
    last_rendered_display: String,
    last_pressed_key: Option<usize>,
}

impl Application for CalcApp {
    const NAME: &'static str = "calc";

    fn init(
        ctx: &mut ApplicationContext,
        looper: &mut ServiceLooper,
    ) -> Result<Self, stem::errors::Errno> {
        let fd = connect_wayland();
        let text_renderer = load_text_renderer();
        let calc = Calculator::new();
        let state = CalcState::default();
        let theme_name = read_theme_name();
        let theme = theme_by_name(&theme_name);
        let wayland_token = looper.add_fd_readable(fd)?;

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
        set_toplevel_title(fd, TOPLEVEL_ID, "Calculator");
        set_toplevel_app_id(fd, TOPLEVEL_ID, "thingos.calc");
        commit_surface(fd, SURFACE_ID);

        ctx.register_window("calculator toplevel", move || close_toplevel(fd));
        ctx.register_cleanup("wayland fd", move || {
            let _ = vfs_close(fd);
        });

        Ok(Self {
            fd,
            wayland_token,
            text_renderer,
            calc,
            state,
            theme_name,
            theme,
            pending: PendingSurface {
                serial: None,
                width: 360,
                height: 580,
                dirty: false,
                configured: false,
            },
            buffer: None,
            next_callback_id: 1000,
            pending_frame_callbacks: Vec::new(),
            keyboard: KeyboardState::default(),
            pointer: PointerState::default(),
            rx: Vec::new(),
            last_rendered_expression: String::new(),
            last_rendered_display: String::new(),
            last_pressed_key: None,
        })
    }

    fn ready(&mut self, _ctx: &mut ApplicationContext) {
        info!("Ready");
    }

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(IDLE_SLEEP_MS))
    }

    fn handle_event(
        &mut self,
        _ctx: &mut ApplicationContext,
        event: ServiceEvent<'_>,
    ) -> AppAction {
        let mut changed = false;
        match event {
            ServiceEvent::Ready { token, event }
                if token == self.wayland_token && event.is_readable() =>
            {
                let mut quit = false;
                changed = read_events(
                    self.fd,
                    &mut self.rx,
                    &mut self.pending,
                    &mut self.pending_frame_callbacks,
                    &mut self.keyboard,
                    &mut self.pointer,
                    &self.calc,
                    &mut self.state,
                    &mut quit,
                );
                if quit {
                    return AppAction::Quit;
                }
            }
            ServiceEvent::Ready { token, event }
                if token == self.wayland_token && (event.is_hangup() || event.is_error()) =>
            {
                return AppAction::Quit;
            }
            ServiceEvent::Timeout => {}
            ServiceEvent::Message { kind, .. }
                if kind.0 == stem::kinds::KIND_ID_THINGOS_UI_THEME_CHANGED =>
            {
                changed = true;
            }
            ServiceEvent::Message { .. } | ServiceEvent::Ready { .. } => {}
            ServiceEvent::InboxClosed => return AppAction::Quit,
        }
        self.tick(changed);
        AppAction::Continue
    }
}

impl CalcApp {
    fn tick(&mut self, changed: bool) {
        let hold_changed = update_hold_repeat(&mut self.pointer, &self.calc, &mut self.state);
        let theme_changed = refresh_theme(&mut self.theme_name, &mut self.theme);

        let state_changed = self.state.expression != self.last_rendered_expression
            || self.state.display != self.last_rendered_display
            || self.pointer.pressed_key != self.last_pressed_key
            || changed
            || hold_changed
            || theme_changed;

        if self.pending.configured && (self.pending.dirty || state_changed) {
            self.last_rendered_expression = self.state.expression.clone();
            self.last_rendered_display = self.state.display.clone();
            self.last_pressed_key = self.pointer.pressed_key;

            let buf = ensure_buffer(
                self.fd,
                SHM_ID,
                &mut self.buffer,
                SURFACE_ID + 100,
                self.pending.width,
                self.pending.height,
            );
            render_calc(
                buf,
                &self.calc,
                &self.state,
                self.pointer.pressed_key,
                self.theme,
                self.text_renderer.as_ref(),
            );
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

fn read_events(
    fd: u32,
    rx: &mut Vec<u8>,
    pending: &mut PendingSurface,
    pending_frame_callbacks: &mut Vec<u32>,
    keyboard: &mut KeyboardState,
    pointer: &mut PointerState,
    calc: &Calculator,
    state: &mut CalcState,
    quit: &mut bool,
) -> bool {
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
                send_pong(fd, WM_BASE_ID, read_u32(payload, 0))
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
                vfs_write(1, b"calc: explicit exit(0) call\n").ok();
                *quit = true;
                changed = true;
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
                        pointer.pressed_key = hit_test_key(calc, state, pointer.x, pointer.y);
                        pointer.pressed_at_ns = stem::time::monotonic_ns();
                        pointer.last_repeat_ns = pointer.pressed_at_ns;
                        pointer.repeated = false;
                    } else if let Some(index) = pointer.pressed_key.take() {
                        if !pointer.repeated
                            && hit_test_key(calc, state, pointer.x, pointer.y) == Some(index)
                        {
                            let input = calc.keys_for_mode(state.mode)[index].input;
                            calc.reduce(state, input);
                            stem::debug!("{}", calc.narration(input, state));
                        }
                        pointer.pressed_at_ns = 0;
                        pointer.last_repeat_ns = 0;
                        pointer.repeated = false;
                    }
                    changed = true;
                }
            }
            (KEYBOARD_ID, 3) if payload.len() >= 16 => {
                let key = wayland_key_to_hid(read_u32(payload, 8));
                let pressed = read_u32(payload, 12) == 1;
                if pressed {
                    if let Some(input) = calc.key_for_keyboard(key, keyboard.shift) {
                        calc.reduce(state, input);
                        stem::debug!("{}", calc.narration(input, state));
                        changed = true;
                    }
                }
            }
            (KEYBOARD_ID, 4) if payload.len() >= 20 => {
                let depressed = read_u32(payload, 4);
                keyboard.shift = depressed & 1 != 0;
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

fn update_hold_repeat(
    pointer: &mut PointerState,
    calc: &Calculator,
    state: &mut CalcState,
) -> bool {
    let Some(index) = pointer.pressed_key else {
        return false;
    };
    let input = calc.keys_for_mode(state.mode)[index].input;
    if !matches!(input, CalcInput::Backspace | CalcInput::Clear) {
        return false;
    }

    let now = stem::time::monotonic_ns();
    let elapsed = now.saturating_sub(pointer.pressed_at_ns);
    if elapsed < HOLD_DELAY_NS {
        return false;
    }

    if !pointer.repeated || now.saturating_sub(pointer.last_repeat_ns) >= HOLD_REPEAT_NS {
        calc.reduce(state, input);
        stem::debug!("{}", calc.narration(input, state));
        pointer.last_repeat_ns = now;
        pointer.repeated = true;
        return true;
    }

    false
}

fn hit_test_key(calc: &Calculator, state: &CalcState, x: i32, y: i32) -> Option<usize> {
    let (mut tree, nodes) = calc.build_tree(state).ok()?;
    tree.apply_style(
        tree.root(),
        ResolvedStyle {
            width: Some(360.0),
            height: Some(580.0),
            flex_direction: Some(FlexDirection::Column),
            justify_content: Some(JustifyContent::Start),
            align_items: Some(AlignItems::Stretch),
            ..ResolvedStyle::default()
        },
    )
    .ok()?;
    tree.compute_layout(Size {
        width: AvailableSpace::Definite(360.0),
        height: AvailableSpace::Definite(580.0),
    })
    .ok()?;

    for (index, key) in nodes.keys.iter().enumerate() {
        let b = tree.global_layout_box(key.node).ok()?;
        if x >= b.x as i32
            && y >= b.y as i32
            && x < (b.x + b.width) as i32
            && y < (b.y + b.height) as i32
        {
            return Some(index);
        }
    }
    None
}

fn render_calc(
    buffer: BufferState,
    calc: &Calculator,
    state: &CalcState,
    pressed_key: Option<usize>,
    theme: Theme,
    text_renderer: Option<&TextRenderer>,
) {
    unsafe {
        let pixels = core::slice::from_raw_parts_mut(
            buffer.ptr as *mut u32,
            (buffer.width * buffer.height) as usize,
        );
        pixels.fill(theme.body_top);
        let _ = render_calc_petal(buffer, pixels, calc, state, pressed_key, theme, text_renderer);
    }
}

fn render_calc_petal(
    buffer: BufferState,
    pixels: &mut [u32],
    calc: &Calculator,
    state: &CalcState,
    pressed_key: Option<usize>,
    theme: Theme,
    text_renderer: Option<&TextRenderer>,
) -> Result<(), ()> {
    let (mut tree, nodes) = calc.build_tree_for_theme(state, theme).map_err(|_| ())?;
    if let Some(index) = pressed_key {
        if let Some(key) = nodes.keys.get(index) {
            tree.set_state(key.node, State::Active, true);
            tree.restyle(&calc.rules_for_theme(theme)).map_err(|_| ())?;
        }
    }
    tree.apply_style(
        tree.root(),
        ResolvedStyle {
            width: Some(buffer.width as f32),
            height: Some(buffer.height as f32),
            flex_direction: Some(FlexDirection::Column),
            justify_content: Some(JustifyContent::Start),
            align_items: Some(AlignItems::Stretch),
            ..ResolvedStyle::default()
        },
    )
    .map_err(|_| ())?;
    tree.compute_layout(Size {
        width: AvailableSpace::Definite(buffer.width as f32),
        height: AvailableSpace::Definite(buffer.height as f32),
    })
    .map_err(|_| ())?;

    fill_node(pixels, buffer.width, buffer.height, &tree, nodes.display_panel)?;
    draw_text_node(
        text_renderer,
        pixels,
        buffer.width,
        buffer.height,
        &tree,
        nodes.expression_line,
    )?;
    draw_text_node(text_renderer, pixels, buffer.width, buffer.height, &tree, nodes.result_line)?;

    for key in nodes.keys.iter() {
        draw_key(
            text_renderer,
            pixels,
            buffer.width,
            buffer.height,
            &tree,
            key,
            matches!(key.key.input, CalcInput::Operator(_) | CalcInput::Equals),
            theme,
        )?;
    }
    fill_node(pixels, buffer.width, buffer.height, &tree, nodes.mode_toggle)?;
    draw_text_node(text_renderer, pixels, buffer.width, buffer.height, &tree, nodes.mode_label)?;

    Ok(())
}

fn draw_key(
    text_renderer: Option<&TextRenderer>,
    pixels: &mut [u32],
    width: u32,
    height: u32,
    tree: &petals::UiTree,
    key: &CalcKeyNode,
    dark_text: bool,
    theme: Theme,
) -> Result<(), ()> {
    fill_node(pixels, width, height, tree, key.node)?;
    let label = tree.node(key.label).ok_or(())?;
    let label_box = tree.global_layout_box(key.label).map_err(|_| ())?;
    let key_box = tree.global_layout_box(key.node).map_err(|_| ())?;
    let text = match label.attrs.get("text") {
        Some(petals::AttrValue::Str(text)) => text.as_str(),
        _ => key.key.label,
    };
    let color = if dark_text {
        theme.body_top
    } else {
        argb(label.style.color.unwrap_or(Color::rgb(0xe6, 0xea, 0xf0)))
    };
    let x = (key_box.x + (key_box.width - label_box.width) / 2.0).max(0.0) as i32;
    let y = (key_box.y + (key_box.height + label.style.font_size.unwrap_or(22.0)) / 2.0 - 4.0)
        .max(0.0) as i32;
    draw_generic_text(
        text_renderer,
        pixels,
        width,
        height,
        x,
        y,
        label.style.font_size.unwrap_or(22.0),
        text,
        color,
    );
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
    stem::info!("calc: applying theme {}", next_theme.name);
    *current_name = next_name;
    *theme = next_theme;
    true
}

fn fill_node(
    pixels: &mut [u32],
    width: u32,
    height: u32,
    tree: &petals::UiTree,
    node_id: u32,
) -> Result<(), ()> {
    let node = tree.node(node_id).ok_or(())?;
    let color = node.style.background_color.unwrap_or(Color::rgb(0x22, 0x26, 0x34));
    let b = tree.global_layout_box(node_id).map_err(|_| ())?;
    fill_rect(
        pixels,
        width,
        height,
        b.x as i32,
        b.y as i32,
        b.width as i32,
        b.height as i32,
        argb(color),
    );
    Ok(())
}

fn draw_text_node(
    text_renderer: Option<&TextRenderer>,
    pixels: &mut [u32],
    width: u32,
    height: u32,
    tree: &petals::UiTree,
    node_id: u32,
) -> Result<(), ()> {
    let node = tree.node(node_id).ok_or(())?;
    let text = match node.attrs.get("text") {
        Some(petals::AttrValue::Str(text)) => text.as_str(),
        _ => "",
    };
    let b = tree.global_layout_box(node_id).map_err(|_| ())?;
    let font_size = node.style.font_size.unwrap_or(18.0);
    let estimated_w = text.len() as f32 * font_size * 0.55;
    let x = (b.x + b.width - estimated_w).max(b.x) as i32;
    let y = (b.y + font_size).max(0.0) as i32;
    draw_generic_text(
        text_renderer,
        pixels,
        width,
        height,
        x,
        y,
        font_size,
        text,
        argb(node.style.color.unwrap_or(Color::rgb(0xe6, 0xea, 0xf0))),
    );
    Ok(())
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
    let fd_buf = memfd_create("calc.buffer", size as usize).expect("create memfd");

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

fn close_toplevel(fd: u32) {
    destroy_object(fd, TOPLEVEL_ID);
    destroy_object(fd, XDG_SURFACE_ID);
    destroy_object(fd, SURFACE_ID);
    destroy_object(fd, POINTER_ID);
    destroy_object(fd, KEYBOARD_ID);
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
        2 => Key::Num1,
        3 => Key::Num2,
        4 => Key::Num3,
        5 => Key::Num4,
        6 => Key::Num5,
        7 => Key::Num6,
        8 => Key::Num7,
        9 => Key::Num8,
        10 => Key::Num9,
        11 => Key::Num0,
        12 => Key::Minus,
        13 => Key::Equal,
        14 => Key::Backspace,
        28 => Key::Enter,
        46 => Key::C,
        50 => Key::M,
        52 => Key::Period,
        53 => Key::Slash,
        57 => Key::Space,
        _ => Key::Unknown,
    }
}
