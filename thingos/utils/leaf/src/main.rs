#![cfg_attr(target_os = "thingos", no_std)]
#![cfg_attr(target_os = "thingos", no_main)]

#[cfg(not(target_os = "thingos"))]
fn main() {
    host_app::run();
}

#[cfg(not(target_os = "thingos"))]
mod host_app {
    use std::num::NonZeroU32;
    use std::sync::Arc;

    use terminal_core::{CELL_HEIGHT, CELL_WIDTH, Cell, Font, TermModel};
    use winit::dpi::LogicalSize;
    use winit::event::{ElementState, Event, WindowEvent};
    use winit::event_loop::{ControlFlow, EventLoop};
    use winit::keyboard::{Key, NamedKey};
    use winit::window::Window;

    const HOST_WIDTH: u32 = 880;
    const HOST_HEIGHT: u32 = 520;

    #[allow(deprecated)]
    pub fn run() {
        let backend = std::env::var("WINIT_UNIX_BACKEND").unwrap_or_else(|_| "auto".to_string());
        eprintln!("leaf: host window starting (WINIT_UNIX_BACKEND={backend})");
        eprintln!("leaf: set WINIT_UNIX_BACKEND=wayland to force a Wayland window");

        let font = load_font();
        let mut model = TermModel::new(HOST_WIDTH / CELL_WIDTH, HOST_HEIGHT / CELL_HEIGHT);
        model.write_str("\x1b[32mThing-OS Leaf\x1b[0m\n", &font);
        model.write_str("host Wayland preview\n", &font);
        model.write_str("type here; Enter starts a new prompt\n\nleaf$ ", &font);
        model.mark_all_dirty();

        let event_loop = EventLoop::new().expect("create event loop");
        event_loop.set_control_flow(ControlFlow::Wait);
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Leaf")
                        .with_inner_size(LogicalSize::new(HOST_WIDTH as f64, HOST_HEIGHT as f64)),
                )
                .expect("create Leaf host window"),
        );

        let context = softbuffer::Context::new(window.clone()).expect("create softbuffer context");
        let mut surface =
            softbuffer::Surface::new(&context, window.clone()).expect("create softbuffer surface");

        let mut resized = true;
        let mut redraw = true;

        event_loop
            .run(move |event, elwt| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(_) => {
                        resized = true;
                        redraw = true;
                        window.request_redraw();
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == ElementState::Pressed {
                            if handle_key(&mut model, &font, &event.logical_key) {
                                redraw = true;
                                window.request_redraw();
                            }
                            if let Some(text) = event.text {
                                let mut wrote_text = false;
                                for ch in text.chars() {
                                    if !ch.is_control() {
                                        model.putc(ch, &font);
                                        wrote_text = true;
                                    }
                                }
                                if wrote_text {
                                    redraw = true;
                                    window.request_redraw();
                                }
                            }
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        let size = window.inner_size();
                        if size.width == 0 || size.height == 0 {
                            return;
                        }
                        if resized {
                            let width = NonZeroU32::new(size.width.max(1)).unwrap();
                            let height = NonZeroU32::new(size.height.max(1)).unwrap();
                            surface.resize(width, height).expect("resize softbuffer surface");
                            let cols = (size.width / CELL_WIDTH).max(1);
                            let rows = (size.height / CELL_HEIGHT).max(1);
                            if cols != model.cols || rows != model.rows {
                                model.resize(cols, rows);
                                model.write_str("\x1b[32mThing-OS Leaf\x1b[0m\n", &font);
                                model.write_str("host Wayland preview\n\nleaf$ ", &font);
                                model.mark_all_dirty();
                            }
                            resized = false;
                            redraw = true;
                        }
                        if redraw {
                            paint(&mut surface, &mut model, &font, size.width, size.height);
                            redraw = false;
                        }
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    if redraw {
                        window.request_redraw();
                    }
                }
                _ => {}
            })
            .expect("run event loop");
    }

    fn load_font() -> Font {
        let candidates = [
            "assets/fonts/unifont.hex",
            "../../assets/fonts/unifont.hex",
            "/usr/share/unifont/unifont.hex",
        ];
        for path in candidates {
            if let Ok(text) = std::fs::read_to_string(path) {
                let font = Font::from_unifont_hex(&text);
                eprintln!("leaf: loaded {} glyphs from {path}", font.glyph_count());
                return font;
            }
        }
        eprintln!("leaf: unifont.hex not found; using empty fallback font");
        Font::from_unifont_hex("003F:00000000000000000000000000000000\n")
    }

    fn handle_key(model: &mut TermModel, font: &Font, key: &Key) -> bool {
        match key {
            Key::Named(NamedKey::Enter) => {
                model.write_str("\nleaf$ ", font);
                true
            }
            Key::Named(NamedKey::Backspace) => {
                model.putc('\x08', font);
                model.putc(' ', font);
                model.putc('\x08', font);
                true
            }
            Key::Named(NamedKey::Tab) => {
                model.putc('\t', font);
                true
            }
            _ => false,
        }
    }

    fn paint(
        surface: &mut softbuffer::Surface<Arc<Window>, Arc<Window>>,
        model: &mut TermModel,
        font: &Font,
        width: u32,
        height: u32,
    ) {
        let mut buffer = surface.buffer_mut().expect("get softbuffer buffer");
        buffer.fill(0xFF00_0000);
        for row in 0..model.rows {
            for col in 0..model.cols {
                let idx = model.cell_idx(col, row);
                draw_cell(&mut buffer, width, height, col, row, model.cells[idx], font);
            }
        }
        if model.cursor_visible {
            fill_rect(
                &mut buffer,
                width,
                height,
                model.cursor_col * CELL_WIDTH,
                model.cursor_row * CELL_HEIGHT,
                2,
                CELL_HEIGHT,
                model.current_fg,
            );
        }
        buffer.present().expect("present Leaf host window");
    }

    fn draw_cell(
        pixels: &mut [u32],
        width: u32,
        height: u32,
        col: u32,
        row: u32,
        cell: Cell,
        font: &Font,
    ) {
        if cell.ch == '\0' {
            return;
        }
        let x = col * CELL_WIDTH;
        let y = row * CELL_HEIGHT;
        fill_rect(pixels, width, height, x, y, CELL_WIDTH, CELL_HEIGHT, cell.bg);
        let Some(glyph) = font.get_glyph(cell.ch).or_else(|| font.get_glyph('?')) else {
            return;
        };
        if glyph.bitmap.len() == 16 {
            for py in 0..16usize {
                let bits = glyph.bitmap[py];
                for px in 0..8u32 {
                    if (bits & (0x80 >> px)) != 0 {
                        put_pixel(pixels, width, height, x + px, y + py as u32, cell.fg);
                    }
                }
            }
        } else if glyph.bitmap.len() == 32 {
            for py in 0..16usize {
                let b1 = glyph.bitmap[py * 2];
                let b2 = glyph.bitmap[py * 2 + 1];
                for px in 0..8u32 {
                    if (b1 & (0x80 >> px)) != 0 {
                        put_pixel(pixels, width, height, x + px, y + py as u32, cell.fg);
                    }
                    if (b2 & (0x80 >> px)) != 0 {
                        put_pixel(pixels, width, height, x + px + 8, y + py as u32, cell.fg);
                    }
                }
            }
        }
    }

    fn fill_rect(
        pixels: &mut [u32],
        width: u32,
        height: u32,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        color: u32,
    ) {
        for py in y..(y + h).min(height) {
            for px in x..(x + w).min(width) {
                pixels[py as usize * width as usize + px as usize] = color;
            }
        }
    }

    fn put_pixel(pixels: &mut [u32], width: u32, height: u32, x: u32, y: u32, color: u32) {
        if x < width && y < height {
            pixels[y as usize * width as usize + x as usize] = color;
        }
    }
}

#[cfg(target_os = "thingos")]
mod thingos_app {
    extern crate alloc;

    use alloc::collections::BTreeMap;
    use alloc::vec::Vec;
    use core::convert::TryInto;

    use abi::syscall::{PollHandle, poll_flags};
    use abi::types::stdio_mode;
    use stem::syscall::socket::{connect, sendmsg, socket};
    use stem::syscall::socket_domain::AF_UNIX;
    use stem::syscall::socket_type::SOCK_STREAM;
    use stem::syscall::{
        exit, memfd_create, sleep_ms, spawn_process_ex, vfs_close, vfs_open, vfs_poll, vfs_read,
        vfs_stat, vfs_write, vm_map,
    };
    use terminal_core::{CELL_HEIGHT, CELL_WIDTH, Cell, Font, TermModel};

    const REGISTRY_ID: u32 = 2;
    const COMPOSITOR_ID: u32 = 3;
    const SHM_ID: u32 = 4;
    const WM_BASE_ID: u32 = 5;
    const SEAT_ID: u32 = 6;
    const KEYBOARD_ID: u32 = 7;

    const SURFACE_ID: u32 = 10;
    const XDG_SURFACE_ID: u32 = 11;
    const TOPLEVEL_ID: u32 = 12;

    const INITIAL_WIDTH: u32 = 760;
    const INITIAL_HEIGHT: u32 = 460;
    const SHELL_PATH: &str = "/bin/sh";
    const FONT_PATH: &str = "/public/fonts/unifont.hex";
    const CURSOR_BLINK_NS: u64 = 500_000_000;
    const MAX_WAYLAND_RX_BYTES: usize = 64 * 1024;

    #[derive(Clone, Copy)]
    struct BufferState {
        buffer_id: u32,
        _handle: u32,
        ptr: *mut u8,
        width: u32,
        height: u32,
    }

    struct PendingSurface {
        serial: Option<u32>,
        width: u32,
        height: u32,
        dirty: bool,
        configured: bool,
    }

    struct ShellPipes {
        stdin_write: u32,
        stdout_read: u32,
        stderr_read: u32,
        input_line: Vec<u8>,
    }

    #[derive(Default)]
    struct KeyboardState {
        shift: bool,
        ctrl: bool,
    }

    #[stem::main]
    fn main(_arg: usize) -> ! {
        stem::info!("leaf: starting");

        let font = load_font();
        let mut shell = spawn_shell();
        let fd = connect_wayland();

        send_get_registry(fd, REGISTRY_ID);
        read_initial_globals(fd);
        bind_global(fd, 1, "wl_compositor", 4, COMPOSITOR_ID);
        bind_global(fd, 2, "wl_shm", 1, SHM_ID);
        bind_global(fd, 3, "xdg_wm_base", 1, WM_BASE_ID);
        bind_global(fd, 4, "wl_seat", 5, SEAT_ID);
        seat_get_keyboard(fd, SEAT_ID, KEYBOARD_ID);

        create_surface(fd, COMPOSITOR_ID, SURFACE_ID);
        get_xdg_surface(fd, WM_BASE_ID, XDG_SURFACE_ID, SURFACE_ID);
        get_toplevel(fd, XDG_SURFACE_ID, TOPLEVEL_ID);
        set_toplevel_title(fd, TOPLEVEL_ID, "Leaf");
        set_toplevel_app_id(fd, TOPLEVEL_ID, "thingos.leaf");
        commit_surface(fd, SURFACE_ID);

        let mut pending = PendingSurface {
            serial: None,
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
            dirty: false,
            configured: false,
        };
        let mut model = TermModel::new(INITIAL_WIDTH / CELL_WIDTH, INITIAL_HEIGHT / CELL_HEIGHT);
        model.write_str("\x1b[32mThing-OS Leaf\x1b[0m\n", &font);
        model.mark_all_dirty();

        let mut buffer: Option<BufferState> = None;
        let mut next_callback_id = 1000u32;
        let mut pending_frame_callbacks: Vec<u32> = Vec::new();
        let mut wayland_rx: Vec<u8> = Vec::new();
        let mut keyboard = KeyboardState::default();
        let mut cursor_blink_on = true;
        let mut last_cursor_blink_ns = stem::monotonic_ns();

        loop {
            let mut needs_render = false;
            if drain_shell(&mut shell, &mut model, &font) {
                needs_render = true;
                cursor_blink_on = true;
                last_cursor_blink_ns = stem::monotonic_ns();
            }
            if read_wayland_events(
                fd,
                &mut wayland_rx,
                &mut pending,
                &mut pending_frame_callbacks,
                &mut keyboard,
                &mut shell,
                &mut model,
                &font,
            ) {
                needs_render = true;
                cursor_blink_on = true;
                last_cursor_blink_ns = stem::monotonic_ns();
            }

            let now_ns = stem::monotonic_ns();
            if model.cursor_visible
                && now_ns.saturating_sub(last_cursor_blink_ns) >= CURSOR_BLINK_NS
            {
                cursor_blink_on = !cursor_blink_on;
                last_cursor_blink_ns = now_ns;
                needs_render = true;
            }

            if pending.configured {
                let cols = (pending.width / CELL_WIDTH).max(1);
                let rows = (pending.height / CELL_HEIGHT).max(1);
                if cols != model.cols || rows != model.rows {
                    model.resize(cols, rows);
                    model.write_str("\x1b[32mThing-OS Leaf\x1b[0m\n", &font);
                    model.mark_all_dirty();
                    needs_render = true;
                }
            }

            let should_render = pending.configured && (pending.dirty || needs_render);
            if should_render {
                let buf = ensure_buffer(
                    fd,
                    SHM_ID,
                    &mut buffer,
                    SURFACE_ID + 100,
                    pending.width,
                    pending.height,
                );
                render_terminal(buf, &mut model, &font, cursor_blink_on);
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

            sleep_ms(16);
        }
    }

    fn load_font() -> Font {
        let fd = match vfs_open(FONT_PATH, abi::syscall::vfs_flags::O_RDONLY) {
            Ok(fd) => fd,
            Err(e) => {
                stem::error!("leaf: failed to open {}: {:?}", FONT_PATH, e);
                exit(1);
            }
        };
        let stat = match vfs_stat(fd) {
            Ok(stat) => stat,
            Err(e) => {
                stem::error!("leaf: failed to stat {}: {:?}", FONT_PATH, e);
                exit(1);
            }
        };
        let mut data = Vec::new();
        data.resize(stat.size as usize, 0);
        let n = match vfs_read(fd, &mut data) {
            Ok(n) => n,
            Err(e) => {
                stem::error!("leaf: failed to read {}: {:?}", FONT_PATH, e);
                exit(1);
            }
        };
        let _ = vfs_close(fd);
        data.truncate(n);
        let text = alloc::string::String::from_utf8_lossy(&data);
        let font = Font::from_unifont_hex(&text);
        stem::info!("leaf: loaded {} glyphs from {}", font.glyph_count(), FONT_PATH);
        font
    }

    fn spawn_shell() -> ShellPipes {
        let argv: [&[u8]; 1] = [SHELL_PATH.as_bytes()];
        match spawn_process_ex(
            SHELL_PATH,
            &argv,
            &BTreeMap::new(),
            stdio_mode::PIPE,
            stdio_mode::PIPE,
            stdio_mode::PIPE,
            0,
            &[],
        ) {
            Ok(resp) => {
                stem::info!("leaf: spawned shell pid={}", resp.child_pid);
                stem::info!("leaf: pipe-backed shell input uses local echo");
                ShellPipes {
                    stdin_write: resp.stdin_pipe as u32,
                    stdout_read: resp.stdout_pipe as u32,
                    stderr_read: resp.stderr_pipe as u32,
                    input_line: Vec::new(),
                }
            }
            Err(e) => {
                stem::error!("leaf: failed to spawn /bin/sh: {:?}", e);
                exit(1);
            }
        }
    }

    fn drain_shell(shell: &mut ShellPipes, model: &mut TermModel, font: &Font) -> bool {
        let mut changed = false;
        for fd in [shell.stdout_read, shell.stderr_read] {
            loop {
                let mut pollfd =
                    [PollHandle { handle: fd as i32, events: poll_flags::POLLIN, revents: 0 }];
                if !matches!(vfs_poll(&mut pollfd, 0), Ok(n) if n > 0)
                    || (pollfd[0].revents & poll_flags::POLLIN) == 0
                {
                    break;
                }
                let mut buf = [0u8; 512];
                match vfs_read(fd, &mut buf) {
                    Ok(n) if n > 0 => {
                        model.write_bytes_lossy(&buf[..n], font);
                        changed = true;
                    }
                    _ => break,
                }
            }
        }
        changed
    }

    fn connect_wayland() -> u32 {
        loop {
            let fd = match socket(AF_UNIX, SOCK_STREAM, 0) {
                Ok(fd) => fd,
                Err(e) => {
                    stem::error!("leaf: socket(AF_UNIX) failed: {:?}", e);
                    sleep_ms(250);
                    continue;
                }
            };
            match connect(fd, "/run/wayland-0") {
                Ok(()) => {
                    stem::info!("leaf: connected to /run/wayland-0");
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
        let mut buf = [0u8; 1024];
        let _ = vfs_read(fd, &mut buf);
    }

    fn read_wayland_events(
        fd: u32,
        rx: &mut Vec<u8>,
        pending: &mut PendingSurface,
        pending_frame_callbacks: &mut Vec<u32>,
        keyboard: &mut KeyboardState,
        shell: &mut ShellPipes,
        model: &mut TermModel,
        font: &Font,
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
        if rx.len() > MAX_WAYLAND_RX_BYTES {
            stem::warn!("leaf: dropping oversized Wayland receive buffer len={}", rx.len());
            rx.clear();
            return true;
        }

        let mut offset = 0usize;
        while offset + 8 <= rx.len() {
            let (object_id, opcode, size) = decode_header(&rx[offset..]);
            if size < 8 {
                stem::warn!("leaf: invalid Wayland message size {}", size);
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
                    stem::info!("leaf: compositor requested close; exiting");
                    exit(0);
                }
                (KEYBOARD_ID, 3) if payload.len() >= 16 => {
                    let key = read_u32(payload, 8);
                    let pressed = read_u32(payload, 12) == 1;
                    if pressed {
                        if let Some(bytes) = key_to_bytes(key, keyboard) {
                            if handle_shell_input(shell, model, font, bytes) {
                                changed = true;
                            }
                        }
                    }
                }
                (KEYBOARD_ID, 4) if payload.len() >= 20 => {
                    let depressed = read_u32(payload, 4);
                    keyboard.shift = depressed & 1 != 0;
                    keyboard.ctrl = depressed & (1 << 2) != 0;
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

    fn handle_shell_input(
        shell: &mut ShellPipes,
        model: &mut TermModel,
        font: &Font,
        bytes: &[u8],
    ) -> bool {
        if bytes.first().copied() == Some(0x1b) {
            return false;
        }

        let mut changed = false;
        for &byte in bytes {
            match byte {
                b'\r' | b'\n' => {
                    model.putc('\n', font);
                    shell.input_line.push(b'\n');
                    let _ = vfs_write(shell.stdin_write, &shell.input_line);
                    shell.input_line.clear();
                    changed = true;
                }
                0x08 | 0x7f => {
                    if shell.input_line.pop().is_some() {
                        model.putc('\x08', font);
                        model.putc(' ', font);
                        model.putc('\x08', font);
                        changed = true;
                    }
                }
                b'\t' => {
                    shell.input_line.push(byte);
                    model.putc('\t', font);
                    changed = true;
                }
                0x03 => {
                    shell.input_line.clear();
                    model.write_str("^C\n", font);
                    let _ = vfs_write(shell.stdin_write, &[byte]);
                    changed = true;
                }
                b if b >= 0x20 => {
                    shell.input_line.push(b);
                    model.write_bytes_lossy(&[b], font);
                    changed = true;
                }
                b => {
                    let _ = vfs_write(shell.stdin_write, &[b]);
                }
            }
        }
        changed
    }

    fn key_to_bytes(key: u32, keyboard: &KeyboardState) -> Option<&'static [u8]> {
        if keyboard.ctrl {
            return match key {
                30 => Some(b"\x01"), // A
                46 => Some(b"\x03"), // C
                32 => Some(b"\x04"), // D
                18 => Some(b"\x05"), // E
                37 => Some(b"\x0b"), // K
                22 => Some(b"\x15"), // U
                17 => Some(b"\x17"), // W
                _ => None,
            };
        }

        match key {
            2 => Some(if keyboard.shift { b"!" } else { b"1" }),
            3 => Some(if keyboard.shift { b"@" } else { b"2" }),
            4 => Some(if keyboard.shift { b"#" } else { b"3" }),
            5 => Some(if keyboard.shift { b"$" } else { b"4" }),
            6 => Some(if keyboard.shift { b"%" } else { b"5" }),
            7 => Some(if keyboard.shift { b"^" } else { b"6" }),
            8 => Some(if keyboard.shift { b"&" } else { b"7" }),
            9 => Some(if keyboard.shift { b"*" } else { b"8" }),
            10 => Some(if keyboard.shift { b"(" } else { b"9" }),
            11 => Some(if keyboard.shift { b")" } else { b"0" }),
            12 => Some(if keyboard.shift { b"_" } else { b"-" }),
            13 => Some(if keyboard.shift { b"+" } else { b"=" }),
            14 => Some(b"\x7f"),
            15 => Some(b"\t"),
            16 => Some(if keyboard.shift { b"Q" } else { b"q" }),
            17 => Some(if keyboard.shift { b"W" } else { b"w" }),
            18 => Some(if keyboard.shift { b"E" } else { b"e" }),
            19 => Some(if keyboard.shift { b"R" } else { b"r" }),
            20 => Some(if keyboard.shift { b"T" } else { b"t" }),
            21 => Some(if keyboard.shift { b"Y" } else { b"y" }),
            22 => Some(if keyboard.shift { b"U" } else { b"u" }),
            23 => Some(if keyboard.shift { b"I" } else { b"i" }),
            24 => Some(if keyboard.shift { b"O" } else { b"o" }),
            25 => Some(if keyboard.shift { b"P" } else { b"p" }),
            26 => Some(if keyboard.shift { b"{" } else { b"[" }),
            27 => Some(if keyboard.shift { b"}" } else { b"]" }),
            28 => Some(b"\n"),
            30 => Some(if keyboard.shift { b"A" } else { b"a" }),
            31 => Some(if keyboard.shift { b"S" } else { b"s" }),
            32 => Some(if keyboard.shift { b"D" } else { b"d" }),
            33 => Some(if keyboard.shift { b"F" } else { b"f" }),
            34 => Some(if keyboard.shift { b"G" } else { b"g" }),
            35 => Some(if keyboard.shift { b"H" } else { b"h" }),
            36 => Some(if keyboard.shift { b"J" } else { b"j" }),
            37 => Some(if keyboard.shift { b"K" } else { b"k" }),
            38 => Some(if keyboard.shift { b"L" } else { b"l" }),
            39 => Some(if keyboard.shift { b":" } else { b";" }),
            40 => Some(if keyboard.shift { b"\"" } else { b"'" }),
            41 => Some(if keyboard.shift { b"~" } else { b"`" }),
            43 => Some(if keyboard.shift { b"|" } else { b"\\" }),
            44 => Some(if keyboard.shift { b"Z" } else { b"z" }),
            45 => Some(if keyboard.shift { b"X" } else { b"x" }),
            46 => Some(if keyboard.shift { b"C" } else { b"c" }),
            47 => Some(if keyboard.shift { b"V" } else { b"v" }),
            48 => Some(if keyboard.shift { b"B" } else { b"b" }),
            49 => Some(if keyboard.shift { b"N" } else { b"n" }),
            50 => Some(if keyboard.shift { b"M" } else { b"m" }),
            51 => Some(if keyboard.shift { b"<" } else { b"," }),
            52 => Some(if keyboard.shift { b">" } else { b"." }),
            53 => Some(if keyboard.shift { b"?" } else { b"/" }),
            57 => Some(b" "),
            103 => Some(b"\x1b[A"),
            108 => Some(b"\x1b[B"),
            106 => Some(b"\x1b[C"),
            105 => Some(b"\x1b[D"),
            _ => None,
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
        let fd_buf = memfd_create("wayland-terminal.buffer", size as usize).expect("create memfd");
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
        let out = BufferState { buffer_id, _handle: fd_buf, ptr, width, height };
        *current = Some(out);
        out
    }

    fn render_terminal(
        buffer: BufferState,
        model: &mut TermModel,
        font: &Font,
        cursor_blink_on: bool,
    ) {
        unsafe {
            let pixels = core::slice::from_raw_parts_mut(
                buffer.ptr as *mut u32,
                (buffer.width * buffer.height) as usize,
            );
            pixels.fill(0xFF00_0000);
            for row in 0..model.rows {
                for col in 0..model.cols {
                    let idx = model.cell_idx(col, row);
                    draw_cell(
                        pixels,
                        buffer.width,
                        buffer.height,
                        col,
                        row,
                        model.cells[idx],
                        font,
                    );
                }
            }
            if model.cursor_visible && cursor_blink_on {
                draw_cursor(
                    pixels,
                    buffer.width,
                    buffer.height,
                    model.cursor_col,
                    model.cursor_row,
                    model.current_fg,
                );
            }
            for dirty in &mut model.dirty_rows {
                *dirty = false;
            }
        }
    }

    fn draw_cell(
        pixels: &mut [u32],
        width: u32,
        height: u32,
        col: u32,
        row: u32,
        cell: Cell,
        font: &Font,
    ) {
        if cell.ch == '\0' {
            return;
        }
        let x = col * CELL_WIDTH;
        let y = row * CELL_HEIGHT;
        let Some(glyph) = font.get_glyph(cell.ch).or_else(|| font.get_glyph('?')) else {
            fill_rect(pixels, width, height, x, y, CELL_WIDTH, CELL_HEIGHT, cell.bg);
            return;
        };
        if glyph.bitmap.len() == 16 {
            for py in 0..16usize {
                let bits = glyph.bitmap[py];
                for px in 0..8u32 {
                    let color = if (bits & (0x80 >> px)) != 0 { cell.fg } else { cell.bg };
                    put_pixel(pixels, width, height, x + px, y + py as u32, color);
                }
            }
        } else if glyph.bitmap.len() == 32 {
            for py in 0..16usize {
                let b1 = glyph.bitmap[py * 2];
                let b2 = glyph.bitmap[py * 2 + 1];
                for px in 0..8u32 {
                    let c1 = if (b1 & (0x80 >> px)) != 0 { cell.fg } else { cell.bg };
                    let c2 = if (b2 & (0x80 >> px)) != 0 { cell.fg } else { cell.bg };
                    put_pixel(pixels, width, height, x + px, y + py as u32, c1);
                    put_pixel(pixels, width, height, x + px + 8, y + py as u32, c2);
                }
            }
        }
    }

    fn draw_cursor(pixels: &mut [u32], width: u32, height: u32, col: u32, row: u32, color: u32) {
        let x = col * CELL_WIDTH;
        let y = row * CELL_HEIGHT;
        fill_rect(pixels, width, height, x, y, 2, CELL_HEIGHT, color);
    }

    fn fill_rect(
        pixels: &mut [u32],
        width: u32,
        height: u32,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        color: u32,
    ) {
        for py in y..(y + h).min(height) {
            for px in x..(x + w).min(width) {
                pixels[py as usize * width as usize + px as usize] = color;
            }
        }
    }

    fn put_pixel(pixels: &mut [u32], width: u32, height: u32, x: u32, y: u32, color: u32) {
        if x < width && y < height {
            pixels[y as usize * width as usize + x as usize] = color;
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

    fn seat_get_keyboard(fd: u32, seat_id: u32, new_id: u32) {
        let mut buf = Vec::new();
        encode_header(seat_id, 1, 12, &mut buf);
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
            stem::warn!("leaf: write failed: {:?}", e);
            if matches!(e, abi::errors::Errno::EBADF | abi::errors::Errno::EPIPE) {
                stem::info!("leaf: Wayland connection closed; exiting");
                exit(0);
            }
        }
    }

    fn send_request_with_fds(fd: u32, buf: &[u8], fds: &[u32]) {
        if fds.is_empty() {
            send_request(fd, buf);
            return;
        }
        if let Err(e) = sendmsg(fd, buf, fds) {
            stem::warn!("leaf: sendmsg failed: {:?}", e);
            if matches!(e, abi::errors::Errno::EBADF | abi::errors::Errno::EPIPE) {
                stem::info!("leaf: Wayland connection closed; exiting");
                exit(0);
            }
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
}
