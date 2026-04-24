#![no_std]
#![no_main]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt::Write;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use abi::display_driver_protocol::{BindPayload, FB_INFO_PAYLOAD_SIZE, FbInfoPayload};
use abi::syscall::vfs_flags::O_RDONLY;
use spin::Mutex;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_stat, vfs_watch_path};
use stem::{error, info};

/// A single glyph from the Unifont font.
struct Glyph {
    width: u32,
    bitmap: Vec<u8>,
}

struct Font {
    glyphs: BTreeMap<u32, Glyph>,
}

impl Font {
    fn load(path: &str) -> Result<Self, String> {
        let fd = vfs_open(path, O_RDONLY)
            .map_err(|e| alloc::format!("failed to open font file: {:?}", e))?;
        let stat = vfs_stat(fd).map_err(|e| alloc::format!("failed to stat font file: {:?}", e))?;
        let size = stat.size;

        let mut data = Vec::with_capacity(size as usize);
        data.resize(size as usize, 0);
        let n = vfs_read(fd, &mut data)
            .map_err(|e| alloc::format!("failed to read font file: {:?}", e))?;
        data.truncate(n);
        let _ = vfs_close(fd);

        let content = String::from_utf8_lossy(&data);
        let mut glyphs = BTreeMap::new();

        for line in content.lines() {
            if let Some((code_str, bitmap_str)) = line.split_once(':') {
                if let Ok(code) = u32::from_str_radix(code_str, 16) {
                    let mut bitmap = Vec::new();
                    for i in 0..(bitmap_str.len() / 2) {
                        if let Ok(byte) = u8::from_str_radix(&bitmap_str[i * 2..i * 2 + 2], 16) {
                            bitmap.push(byte);
                        }
                    }

                    let width = if bitmap_str.len() <= 32 { 8 } else { 16 };
                    glyphs.insert(code, Glyph { width, bitmap });
                }
            }
        }

        info!("Terminal: Loaded {} glyphs from {}", glyphs.len(), path);
        Ok(Font { glyphs })
    }

    fn get_glyph(&self, c: char) -> Option<&Glyph> {
        self.glyphs.get(&(c as u32))
    }
}

// SAFETY: Font contains only plain data (integer code points and Vec<u8> bitmaps)
// with no interior mutability or raw pointers.  All fields are trivially Send + Sync:
// BTreeMap<u32, Glyph> is Send + Sync because u32 and Glyph (u32 + Vec<u8>) are
// Send + Sync.  Font is immutable after construction, so no data-race hazard exists.
unsafe impl Send for Font {}
unsafe impl Sync for Font {}

#[derive(Debug, Clone, PartialEq)]
enum AnsiState {
    Normal,
    Esc,
    Csi { params: Vec<u32>, current_num: Option<u32>, private: bool },
}

// ─── Cell model ──────────────────────────────────────────────────────────────

/// A single cell in the terminal character grid.
#[derive(Clone, Copy)]
struct Cell {
    ch: char,
    fg: u32,
    bg: u32,
    /// True when the glyph occupies two 8-px column units (16-px wide glyph).
    wide: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Cell { ch: ' ', fg: 0xFFFFFFFF, bg: 0xFF000000, wide: false }
    }
}

/// Pure terminal model — tracks cell state, cursor and ANSI parser state.
///
/// This struct deliberately has **no** framebuffer pointer.  The parser thread
/// calls [`TermModel::putc`] / [`TermModel::write_str`] to update the grid;
/// the renderer thread snapshots dirty rows and blits them independently.
struct TermModel {
    cols: u32,
    rows: u32,
    cells: Vec<Cell>,
    /// Per-row dirty flag: set whenever a cell in that row changes.
    dirty_rows: Vec<bool>,
    cursor_col: u32,
    cursor_row: u32,
    cursor_visible: bool,
    current_fg: u32,
    current_bg: u32,
    ansi_state: AnsiState,
}

impl TermModel {
    fn new(cols: u32, rows: u32) -> Self {
        let n = (cols * rows) as usize;
        Self {
            cols,
            rows,
            cells: alloc::vec![Cell::default(); n],
            dirty_rows: alloc::vec![false; rows as usize],
            cursor_col: 0,
            cursor_row: 0,
            cursor_visible: true,
            current_fg: 0xFFFFFFFF,
            current_bg: 0xFF000000,
            ansi_state: AnsiState::Normal,
        }
    }

    fn cell_idx(&self, col: u32, row: u32) -> usize {
        (row * self.cols + col) as usize
    }

    fn set_cell(&mut self, col: u32, row: u32, ch: char, fg: u32, bg: u32, wide: bool) {
        if col < self.cols && row < self.rows {
            let idx = self.cell_idx(col, row);
            self.cells[idx] = Cell { ch, fg, bg, wide };
            self.dirty_rows[row as usize] = true;
        }
    }

    fn clear_all(&mut self, bg: u32) {
        let fg = self.current_fg;
        for cell in &mut self.cells {
            *cell = Cell { ch: ' ', fg, bg, wide: false };
        }
        for d in &mut self.dirty_rows {
            *d = true;
        }
    }

    fn scroll(&mut self) {
        let cols = self.cols as usize;
        let rows = self.rows as usize;
        // Shift rows 1..rows-1 up by one using copy_within (Cell: Copy).
        self.cells.copy_within(cols..cols * rows, 0);
        // Blank the last row.
        let bg = self.current_bg;
        let fg = self.current_fg;
        for col in 0..cols {
            self.cells[(rows - 1) * cols + col] = Cell { ch: ' ', fg, bg, wide: false };
        }
        // Entire buffer has changed.
        for d in &mut self.dirty_rows {
            *d = true;
        }
        if self.cursor_row > 0 {
            self.cursor_row -= 1;
        }
    }

    fn ansi_color_to_u32(code: u32, is_bg: bool) -> u32 {
        let base = if is_bg { 40 } else { 30 };
        match code - base {
            0 => 0xFF000000,
            1 => 0xFFFF0000,
            2 => 0xFF00FF00,
            3 => 0xFFFFFF00,
            4 => 0xFF0000FF,
            5 => 0xFFFF00FF,
            6 => 0xFF00FFFF,
            7 => 0xFFFFFFFF,
            _ => {
                if is_bg {
                    0xFF000000
                } else {
                    0xFFFFFFFF
                }
            }
        }
    }

    /// Process one character through the ANSI parser and update the cell grid.
    fn putc(&mut self, c: char, font: &Font) {
        match self.ansi_state.clone() {
            AnsiState::Normal => {
                if c == '\x1B' {
                    self.ansi_state = AnsiState::Esc;
                    return;
                }
                if c == '\n' {
                    self.cursor_col = 0;
                    self.cursor_row += 1;
                    if self.cursor_row >= self.rows {
                        self.scroll();
                    }
                    return;
                }
                if c == '\r' {
                    self.cursor_col = 0;
                    return;
                }
                if c == '\x08' {
                    if self.cursor_col >= 1 {
                        self.cursor_col -= 1;
                    }
                    return;
                }
                if c == '\t' {
                    for _ in 0..4 {
                        self.putc(' ', font);
                    }
                    return;
                }
            }
            AnsiState::Esc => {
                if c == '[' {
                    self.ansi_state =
                        AnsiState::Csi { params: Vec::new(), current_num: None, private: false };
                } else {
                    self.ansi_state = AnsiState::Normal;
                }
                return;
            }
            AnsiState::Csi { mut params, mut current_num, mut private } => {
                if c == '?' && params.is_empty() && current_num.is_none() {
                    private = true;
                    self.ansi_state = AnsiState::Csi { params, current_num, private };
                    return;
                }
                if c.is_ascii_digit() {
                    let digit = c.to_digit(10).unwrap();
                    current_num = Some(current_num.unwrap_or(0) * 10 + digit);
                    self.ansi_state = AnsiState::Csi { params, current_num, private };
                    return;
                } else if c == ';' {
                    params.push(current_num.unwrap_or(0));
                    self.ansi_state = AnsiState::Csi { params, current_num: None, private };
                    return;
                } else if private && (c == 'h' || c == 'l') {
                    params.push(current_num.unwrap_or(0));
                    let show = c == 'h';
                    for &p in &params {
                        if p == 25 {
                            self.cursor_visible = show;
                        }
                    }
                    self.ansi_state = AnsiState::Normal;
                    return;
                } else if c == 'm' {
                    params.push(current_num.unwrap_or(0));
                    for &p in &params {
                        if p == 0 {
                            self.current_fg = 0xFFFFFFFF;
                            self.current_bg = 0xFF000000;
                        } else if (30..=37).contains(&p) {
                            self.current_fg = Self::ansi_color_to_u32(p, false);
                        } else if (40..=47).contains(&p) {
                            self.current_bg = Self::ansi_color_to_u32(p, true);
                        } else if (90..=97).contains(&p) {
                            self.current_fg = Self::ansi_color_to_u32(p - 60, false) | 0xFF888888;
                        }
                    }
                    self.ansi_state = AnsiState::Normal;
                    return;
                } else if c == 'J' {
                    let bg = self.current_bg;
                    self.clear_all(bg);
                    self.ansi_state = AnsiState::Normal;
                    return;
                } else if c == 'H' {
                    params.push(current_num.unwrap_or(0));
                    let row = params.first().copied().unwrap_or(1).max(1) - 1;
                    let col = params.get(1).copied().unwrap_or(1).max(1) - 1;
                    self.cursor_row = row.min(self.rows - 1);
                    self.cursor_col = col.min(self.cols - 1);
                    self.ansi_state = AnsiState::Normal;
                    return;
                } else if c == 'K' {
                    params.push(current_num.unwrap_or(0));
                    let mode = params.first().copied().unwrap_or(0);
                    let row = self.cursor_row;
                    let fg = self.current_fg;
                    let bg = self.current_bg;
                    match mode {
                        0 => {
                            for col in self.cursor_col..self.cols {
                                self.set_cell(col, row, ' ', fg, bg, false);
                            }
                        }
                        1 => {
                            for col in 0..=self.cursor_col.min(self.cols - 1) {
                                self.set_cell(col, row, ' ', fg, bg, false);
                            }
                        }
                        2 => {
                            for col in 0..self.cols {
                                self.set_cell(col, row, ' ', fg, bg, false);
                            }
                        }
                        _ => {}
                    }
                    self.ansi_state = AnsiState::Normal;
                    return;
                } else if c == 'C' {
                    let n = current_num.unwrap_or(1).max(1);
                    self.cursor_col = (self.cursor_col + n).min(self.cols - 1);
                    self.ansi_state = AnsiState::Normal;
                    return;
                } else if c == 'D' {
                    let n = current_num.unwrap_or(1).max(1);
                    self.cursor_col = self.cursor_col.saturating_sub(n);
                    self.ansi_state = AnsiState::Normal;
                    return;
                } else if c == 'A' {
                    let n = current_num.unwrap_or(1).max(1);
                    self.cursor_row = self.cursor_row.saturating_sub(n);
                    self.ansi_state = AnsiState::Normal;
                    return;
                } else if c == 'B' {
                    let n = current_num.unwrap_or(1).max(1);
                    self.cursor_row = (self.cursor_row + n).min(self.rows - 1);
                    self.ansi_state = AnsiState::Normal;
                    return;
                } else {
                    self.ansi_state = AnsiState::Normal;
                    return;
                }
            }
        }

        // Printable character: look up glyph width (in 8-px column units).
        let glyph_px_width = font
            .get_glyph(c)
            .map(|g| g.width)
            .or_else(|| font.get_glyph('?').map(|g| g.width))
            .unwrap_or(8);
        let col_units = glyph_px_width / 8; // 1 or 2
        let wide = col_units >= 2;

        // Wrap to next line if we'd overflow.
        if self.cursor_col + col_units > self.cols {
            self.putc('\n', font);
        }

        let fg = self.current_fg;
        let bg = self.current_bg;
        self.set_cell(self.cursor_col, self.cursor_row, c, fg, bg, wide);
        // For wide glyphs, fill the continuation column with a blank sentinel
        // so the renderer can skip it.
        if wide && self.cursor_col + 1 < self.cols {
            let next_col = self.cursor_col + 1;
            let row = self.cursor_row;
            self.set_cell(next_col, row, '\0', fg, bg, false);
        }
        self.cursor_col += col_units;
    }

    fn write_str(&mut self, s: &str, font: &Font) {
        for c in s.chars() {
            self.putc(c, font);
        }
    }
}

// ─── Renderer ─────────────────────────────────────────────────────────────────

/// Newtype wrapper that makes a raw framebuffer pointer `Send + Sync`.
///
/// # Safety
/// The following invariants must be maintained:
/// * The pointer must remain valid (non-dangling, correctly aligned) for
///   the entire lifetime of any `TermRenderer` that holds this value.
/// * Only the renderer thread ever accesses the pointed-to memory region.
///   The main/parser thread relinquishes all access to `fb_ptr` immediately
///   after constructing `FbPtr` and moving it into the renderer closure.
/// * No other concurrent access to the framebuffer region occurs from any
///   other thread — the kernel device mapping is exclusive to this process.
struct FbPtr(*mut u32);
// SAFETY: Exclusive ownership is enforced by convention (see above).
unsafe impl Send for FbPtr {}
unsafe impl Sync for FbPtr {}

/// Framebuffer blitter.  Lives exclusively in the renderer thread.
struct TermRenderer {
    fb: FbPtr,
    fb_width: u32,
    fb_height: u32,
    fb_stride: u32,
    font: Arc<Font>,
}

impl TermRenderer {
    fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x < self.fb_width && y < self.fb_height {
            unsafe {
                let off = y * (self.fb_stride / 4) + x;
                *self.fb.0.add(off as usize) = color;
            }
        }
    }

    fn draw_glyph(&mut self, bitmap: &[u8], x: u32, y: u32, fg: u32, bg: u32) {
        if bitmap.len() == 16 {
            for row in 0..16usize {
                let bits = bitmap[row];
                for col in 0..8u32 {
                    let color = if (bits & (0x80 >> col)) != 0 { fg } else { bg };
                    self.set_pixel(x + col, y + row as u32, color);
                }
            }
        } else if bitmap.len() == 32 {
            for row in 0..16usize {
                let b1 = bitmap[row * 2];
                let b2 = bitmap[row * 2 + 1];
                for col in 0..8u32 {
                    let c1 = if (b1 & (0x80 >> col)) != 0 { fg } else { bg };
                    self.set_pixel(x + col, y + row as u32, c1);
                    let c2 = if (b2 & (0x80 >> col)) != 0 { fg } else { bg };
                    self.set_pixel(x + col + 8, y + row as u32, c2);
                }
            }
        }
    }

    /// Blit a single cell at the given column / row position.
    fn blit_cell(&mut self, col: u32, row: u32, cell: &Cell) {
        // Skip continuation columns of wide glyphs (ch == '\0').
        if cell.ch == '\0' {
            return;
        }
        let x = col * 8;
        let y = row * 16;
        // Clone glyph data before calling &mut self methods to avoid a borrow
        // conflict: `self.font` cannot be borrowed immutably while we also
        // need a mutable borrow of `self` for `draw_glyph` / `set_pixel`.
        let glyph_data: Option<Vec<u8>> = self
            .font
            .get_glyph(cell.ch)
            .or_else(|| self.font.get_glyph('?'))
            .map(|g| g.bitmap.clone());
        if let Some(bitmap) = glyph_data {
            self.draw_glyph(&bitmap, x, y, cell.fg, cell.bg);
        } else {
            // Unknown glyph: fill with background colour.
            for py in 0..16u32 {
                for px in 0..8u32 {
                    self.set_pixel(x + px, y + py, cell.bg);
                }
            }
        }
    }

    fn draw_cursor(&mut self, col: u32, row: u32, fg: u32) {
        let x = col * 8;
        let y = row * 16;
        for r in 0..16u32 {
            for c in 0..2u32 {
                self.set_pixel(x + c, y + r, fg);
            }
        }
    }

    fn erase_cursor(&mut self, col: u32, row: u32, bg: u32) {
        let x = col * 8;
        let y = row * 16;
        for r in 0..16u32 {
            for c in 0..2u32 {
                self.set_pixel(x + c, y + r, bg);
            }
        }
    }
}

// ─── Shared state ─────────────────────────────────────────────────────────────

/// State shared between the parser (main) thread and the renderer thread.
struct SharedState {
    /// Terminal grid and ANSI parser state.  The parser thread is the sole
    /// writer; the renderer thread snapshots a subset while holding the lock.
    model: Mutex<TermModel>,
    /// Commit flag: 0 = nothing pending, 1 = renderer should blit.
    ///
    /// The parser sets this to 1 after writing to `model`; the renderer
    /// atomically clears it before processing so subsequent writes from the
    /// parser are not missed.
    commit: AtomicU32,
    /// Set by the main thread; read by the renderer to decide whether to
    /// send a `MSG_PRESENT` to the display driver.
    has_focus: AtomicBool,
    display_req_write: u32,
}

// SAFETY: `model` is protected by `Mutex`, `commit` and `has_focus` are atomic
// types.  `display_req_write` is a plain `u32` that is written exactly once
// during `SharedState` construction (before `Arc::new`) and is thereafter
// read-only, so no data race can occur on it.
unsafe impl Send for SharedState {}
unsafe impl Sync for SharedState {}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn get_active_ui() -> String {
    if let Ok(fd) = vfs_open("/session/active_ui", O_RDONLY) {
        if let Ok(stat) = vfs_stat(fd) {
            let size = stat.size;
            let mut buf = Vec::with_capacity(size as usize);
            buf.resize(size as usize, 0);
            if let Ok(n) = vfs_read(fd, &mut buf) {
                buf.truncate(n);
                let _ = vfs_close(fd);
                return String::from_utf8_lossy(&buf).trim().to_string();
            }
        }
        let _ = vfs_close(fd);
    }
    "terminal".to_string()
}

fn send_present(display_req_write: u32) {
    let mut present_header = [0u8; abi::display_driver_protocol::HEADER_SIZE
        + abi::display_driver_protocol::PRESENT_HEADER_WIRE_SIZE];
    let mut payload = [0u8; abi::display_driver_protocol::PRESENT_HEADER_WIRE_SIZE];
    abi::display_driver_protocol::encode_present_header_le(0, &mut payload);
    if let Some(total) = abi::display_driver_protocol::encode_message(
        &mut present_header,
        abi::display_driver_protocol::MSG_PRESENT,
        &payload,
    ) {
        let _ = stem::syscall::port_send_all(display_req_write, &present_header[..total]);
    }
}

// ─── Entry point ──────────────────────────────────────────────────────────────

#[stem::main]
fn main(arg: usize) -> ! {
    info!("Terminal: Starting...");

    let boot_fd = arg as u32;
    let mut display_req_write = 0u32;
    let mut display_resp_read = 0u32;
    let mut fb_id = 0u32;

    if boot_fd != 0 {
        use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
        let req = VmMapReq {
            addr_hint: 0,
            len: 4096,
            prot: VmProt::READ | VmProt::USER,
            flags: VmMapFlags::empty(),
            backing: VmBacking::File { thing: boot_fd, offset: 0 },
        };
        if let Ok(resp) = stem::syscall::vm_map(&req) {
            let ptr = resp.addr as *const u32;
            let slice = unsafe { core::slice::from_raw_parts(ptr, 1024) };
            info!(
                "Terminal: slice[0]=0x{:08x} [1]=0x{:x} [2]=0x{:x} [4]=0x{:x}",
                slice[0], slice[1], slice[2], slice[4]
            );
            if slice[0] == 0xB100AA01 {
                display_req_write = slice[1];
                display_resp_read = slice[2];
                fb_id = slice[4];
                info!(
                    "Terminal: Bootstrapped via memfd: req={}, resp={}, fb_id={}",
                    display_req_write, display_resp_read, fb_id
                );
            } else {
                error!(
                    "Terminal: Bootstrap magic mismatch! expected 0xB100AA01, got 0x{:08x}",
                    slice[0]
                );
            }
        } else {
            error!("Terminal: Failed to map bootstrap memfd");
        }
    } else {
        info!("Terminal: No bootstrap FD provided (arg was 0)");
    }

    if fb_id == 0 {
        info!("Terminal: No bootstrap FB, trying /dev/fb0 fallback...");
        match vfs_open("/dev/fb0", O_RDONLY) {
            Ok(fd) => {
                fb_id = fd;
                info!("Terminal: Using /dev/fb0 as fb_id={}", fb_id);
            }
            Err(e) => {
                error!("Terminal: Failed to open /dev/fb0: {:?}", e);
                stem::syscall::exit(1);
            }
        }
    }

    let font = match Font::load("/share/fonts/unifont.hex") {
        Ok(f) => f,
        Err(e) => {
            error!("Terminal: Failed to load font: {}", e);
            stem::syscall::exit(1);
        }
    };
    let font = Arc::new(font);

    let fb_info = match read_fb_info() {
        Some(info) => info,
        None => {
            error!("Terminal: Failed to read fb info from /dev/fb0");
            stem::syscall::exit(1);
        }
    };

    info!("Terminal: Display {}x{}, stride={}", fb_info.width, fb_info.height, fb_info.stride);

    let fb_ptr = {
        use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
        let req = VmMapReq {
            addr_hint: 0,
            len: (fb_info.stride as usize) * (fb_info.height as usize),
            prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
            flags: VmMapFlags::empty(),
            backing: VmBacking::File { thing: fb_id, offset: 0 },
        };
        match stem::syscall::vm_map(&req) {
            Ok(resp) => resp.addr as *mut u32,
            Err(e) => {
                error!("Terminal: Failed to map FB bytespace {}: {:?}", fb_id, e);
                stem::syscall::exit(1);
            }
        }
    };

    // Compute cell grid dimensions from framebuffer geometry.
    let cols = fb_info.width / 8;
    let rows = fb_info.height / 16;

    // Build the shared state and perform initial render.
    let mut model = TermModel::new(cols, rows);
    model.write_str("Thing-OS Terminal v1.0\n", &font);
    model.write_str("Unicode test: こんにち世界! 🚀\n", &font);
    // Mark all rows dirty so the first renderer pass blits the welcome text.
    for d in &mut model.dirty_rows {
        *d = true;
    }

    let shared = Arc::new(SharedState {
        model: Mutex::new(model),
        commit: AtomicU32::new(1), // start with a pending commit so first frame is rendered
        has_focus: AtomicBool::new(if display_req_write == 0 {
            true
        } else {
            get_active_ui() == "terminal"
        }),
        display_req_write,
    });

    // ── Connect to display driver ───────────────────────────────────────────
    let bind_payload = BindPayload {
        fb_fd: fb_id,
        _pad: 0,
        width: fb_info.width,
        height: fb_info.height,
        stride: fb_info.stride,
        format: fb_info.format,
    };
    if display_req_write != 0 {
        let mut header_buf = [0u8; abi::display_driver_protocol::HEADER_SIZE
            + abi::display_driver_protocol::BIND_PAYLOAD_WIRE_SIZE];
        let mut payload_buf = [0u8; abi::display_driver_protocol::BIND_PAYLOAD_WIRE_SIZE];
        abi::display_driver_protocol::encode_bind_payload_le(&bind_payload, &mut payload_buf);
        if let Some(total) = abi::display_driver_protocol::encode_message(
            &mut header_buf,
            abi::display_driver_protocol::MSG_BIND,
            &payload_buf,
        ) {
            let _ = stem::syscall::port_send_all(display_req_write, &header_buf[..total]);
            let req_fd = stem::syscall::vfs::vfs_handle_from_port(display_req_write)
                .unwrap_or(display_req_write);
            let _ = stem::syscall::socket::sendmsg(req_fd, &[], &[bind_payload.fb_fd]);
        }
    }

    // ── Renderer thread ────────────────────────────────────────────────────
    // The renderer owns the framebuffer pointer; the parser (main thread) owns
    // no framebuffer access after this point.
    {
        let shared_r = Arc::clone(&shared);
        let font_r = Arc::clone(&font);
        // Wrap the raw pointer before moving into the closure so the closure
        // captures `FbPtr` (Send) rather than `*mut u32` (not Send).
        let fb = FbPtr(fb_ptr);

        let _ = stem::thread::spawn_task_detached(move || {
            renderer_loop(
                shared_r,
                font_r,
                fb,
                fb_info.width,
                fb_info.height,
                fb_info.stride,
            );
        });
    }

    info!("Terminal: Renderer thread spawned; entering parser loop.");

    // ── Focus watcher ───────────────────────────────────────────────────────
    let focus_watch =
        vfs_watch_path("/session/active_ui", abi::vfs_watch::mask::MODIFY, 0).unwrap_or(0);

    let mut frame_count = 0u64;

    // ── Parser / main loop ──────────────────────────────────────────────────
    // This thread updates the terminal model and signals the renderer via the
    // commit flag.  It never touches the framebuffer directly.
    loop {
        if frame_count % 60 == 0 {
            info!("Terminal: Liveness check - frame {}", frame_count);
            {
                let mut m = shared.model.lock();
                m.write_str(".", &font);
                if frame_count % (60 * 40) == 0 {
                    let mut status = String::new();
                    let _ = write!(
                        status,
                        "\n[Terminal Liveness] Frame {} - Focus: {}\n",
                        frame_count,
                        shared.has_focus.load(Ordering::Relaxed)
                    );
                    m.write_str(&status, &font);
                }
            }
            // Signal the renderer that new content is ready.
            shared.commit.store(1, Ordering::Release);
            let _ = stem::syscall::futex_wake(&shared.commit, 1);
        }

        // Check for focus change.
        if focus_watch != 0 {
            let mut fds = [abi::syscall::PollHandle {
                handle: focus_watch as i32,
                events: abi::syscall::poll_flags::POLLIN as u16,
                revents: 0,
            }];
            if let Ok(n) = stem::syscall::vfs::vfs_poll(&mut fds, 0) {
                if n > 0 {
                    let mut dummy = [0u8; 1024];
                    let _ = vfs_read(focus_watch, &mut dummy);

                    let new_focus = get_active_ui() == "terminal";
                    let old_focus = shared.has_focus.swap(new_focus, Ordering::Release);
                    if new_focus != old_focus {
                        if new_focus {
                            info!("Terminal: Gained focus!");
                        } else {
                            info!("Terminal: Lost focus. Blanking screen.");
                            let mut m = shared.model.lock();
                            m.clear_all(0xFF000000);
                        }
                        // Notify renderer.
                        shared.commit.store(1, Ordering::Release);
                        let _ = stem::syscall::futex_wake(&shared.commit, 1);
                    }
                }
            }
        }

        frame_count += 1;
        stem::sleep(core::time::Duration::from_millis(16)); // ~60 fps
    }
}

// ─── Renderer loop ────────────────────────────────────────────────────────────

/// The renderer runs in its own kernel-scheduled thread.
///
/// It sleeps via `futex_wait` until the parser sets `shared.commit` to 1, then
/// snapshots only the dirty rows from the model (holding the lock as briefly as
/// possible), blits them to the framebuffer, draws the cursor, and sends a
/// `MSG_PRESENT` to the display driver.
///
/// Because rendering is decoupled from parsing, heavy glyph blits cannot block
/// the parser from processing further input.
fn renderer_loop(
    shared: Arc<SharedState>,
    font: Arc<Font>,
    fb: FbPtr,
    fb_width: u32,
    fb_height: u32,
    fb_stride: u32,
) -> ! {
    let mut renderer = TermRenderer { fb, fb_width, fb_height, fb_stride, font };

    // Cursor tracking: remember where the cursor was last rendered so we can
    // erase it before drawing it at the new position.
    let mut last_cursor_col = 0u32;
    let mut last_cursor_row = 0u32;
    let mut last_cursor_visible = false;

    loop {
        // Sleep until the parser signals a commit (value transitions from 0
        // to 1).  `futex_wait` returns immediately if the value is already
        // non-zero, so we never miss a wake-up.
        let _ = stem::syscall::futex_wait(&shared.commit, 0, 0);

        // Atomically consume the commit flag before locking the model so that
        // any writes the parser makes *while* we render are not lost.
        // On success (commit was 1, now set to 0): fall through to render.
        // On failure (commit is already 0, e.g. spurious futex wakeup): yield
        // and loop back to futex_wait.
        if shared.commit.compare_exchange(1, 0, Ordering::AcqRel, Ordering::Relaxed).is_err() {
            stem::thread::yield_now();
            continue;
        }

        // ── Snapshot dirty rows under the model lock ────────────────────
        // Hold the lock only long enough to copy cell data for dirty rows and
        // clear the dirty flags atomically.  This prevents the parser from
        // writing new cells and re-setting a dirty flag that we then clear
        // without having captured the cell.
        let (dirty_snapshot, cursor_col, cursor_row, cursor_visible, cursor_fg) = {
            let mut m = shared.model.lock();

            let cols = m.cols;
            let rows = m.rows;
            let cursor_col = m.cursor_col;
            let cursor_row = m.cursor_row;
            let cursor_visible = m.cursor_visible;
            let cursor_fg = m.current_fg;

            // Copy cells for every dirty row and clear the flag in one pass.
            let mut snapshot: Vec<(u32, Vec<Cell>)> = Vec::new();
            for row in 0..rows {
                if m.dirty_rows[row as usize] {
                    let start = (row * cols) as usize;
                    let end = start + cols as usize;
                    snapshot.push((row, m.cells[start..end].to_vec()));
                    m.dirty_rows[row as usize] = false;
                }
            }

            (snapshot, cursor_col, cursor_row, cursor_visible, cursor_fg)
        };

        // ── Blit dirty rows (no lock held) ─────────────────────────────
        for (row, cells) in &dirty_snapshot {
            for col in 0..cells.len() as u32 {
                renderer.blit_cell(col, *row, &cells[col as usize]);
            }
        }

        // ── Cursor management ──────────────────────────────────────────
        // Erase cursor from its previous position (re-blit the underlying cell).
        if last_cursor_visible {
            let row = last_cursor_row;
            let col = last_cursor_col;
            // Re-read the cell under the old cursor from the model.
            let cell_bg = {
                let m = shared.model.lock();
                if col < m.cols && row < m.rows {
                    let idx = (row * m.cols + col) as usize;
                    Some(m.cells[idx])
                } else {
                    None
                }
            };
            if let Some(cell) = cell_bg {
                renderer.erase_cursor(col, row, cell.bg);
                renderer.blit_cell(col, row, &cell);
            }
        }

        // Draw cursor at the new position.
        if cursor_visible {
            renderer.draw_cursor(cursor_col, cursor_row, cursor_fg);
        }
        last_cursor_col = cursor_col;
        last_cursor_row = cursor_row;
        last_cursor_visible = cursor_visible;

        // ── Send present to display driver ─────────────────────────────
        if shared.has_focus.load(Ordering::Acquire) && shared.display_req_write != 0 {
            send_present(shared.display_req_write);
        }
    }
}

fn read_fb_info() -> Option<FbInfoPayload> {
    let fd = vfs_open("/dev/fb0", O_RDONLY).ok()?;
    let mut payload = FbInfoPayload {
        device_handle: 0,
        width: 0,
        height: 0,
        stride: 0,
        bpp: 0,
        format: 0,
        _reserved: 0,
    };
    let buf = unsafe {
        core::slice::from_raw_parts_mut(&mut payload as *mut _ as *mut u8, FB_INFO_PAYLOAD_SIZE)
    };
    let n = stem::syscall::vfs::vfs_read(fd, buf).ok()?;
    let _ = stem::syscall::vfs::vfs_close(fd);
    if n < FB_INFO_PAYLOAD_SIZE || payload.width == 0 || payload.height == 0 || payload.stride == 0
    {
        stem::error!(
            "Terminal: FB info mismatch: n={}, expected={}, w={}, h={}, s={}",
            n,
            FB_INFO_PAYLOAD_SIZE,
            payload.width,
            payload.height,
            payload.stride
        );
        return None;
    }
    Some(payload)
}
