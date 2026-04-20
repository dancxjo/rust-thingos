use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicBool, Ordering};
use kernel::{BootRuntime, BootRuntimeBase};
use spin::Mutex;

use crate::framebuffer::Framebuffer;

const CELL_H: u32 = 16;
const CELL_W: u32 = 8;
const CURSOR_W: u32 = 2;
const CURSOR_BLINK_INTERVAL: u32 = 250;
const TAB_WIDTH: usize = 4;
const CSI_PARAM_CAP: usize = 8;
const DEFAULT_FG: u32 = 0x00FF_FFFF;
const DEFAULT_BG: u32 = 0x0000_0000;
const BRIGHT_FG_OFFSET: u32 = 0x0040_4040;
const BRIGHT_BG_OFFSET: u32 = 0x0020_2020;
const GLYPH_PRELOAD_START: u32 = 0x20;
const GLYPH_PRELOAD_END: u32 = 0xFF;
const GLYPH_PRELOAD_START_U8: u8 = GLYPH_PRELOAD_START as u8;
const GLYPH_PRELOAD_END_U8: u8 = GLYPH_PRELOAD_END as u8;
const ACTIVATION_BANNER: &[u8] = b"\x1b[0mThing-OS kernel terminal (F12)\n";

pub static CONSOLE: Mutex<Option<FbConsole>> = Mutex::new(None);
pub static CONSOLE_DISABLED: AtomicBool = AtomicBool::new(false);

const GLYPH_TABLE_LEN: usize = 256;

#[derive(Clone, Copy)]
struct Glyph {
    width: u8,
    bytes: [u8; 32],
    len: u8,
}

#[derive(Clone)]
enum AnsiState {
    Normal,
    Esc,
    Csi { params: [u16; CSI_PARAM_CAP], len: usize, cur: Option<u16>, private: bool },
}

pub struct FbConsole {
    fb: Framebuffer,
    glyphs: [Glyph; GLYPH_TABLE_LEN],
    /// Lazily-populated cache for glyphs above U+00FF.
    extended_glyphs: BTreeMap<u32, Glyph>,
    /// Raw unifont hex data for on-demand glyph lookup.
    unifont_data: &'static [u8],
    cursor_x: u32,
    cursor_y: u32,
    fg: u32,
    bg: u32,
    ansi: AnsiState,
    active: bool,
    /// Whether the visual cursor bar is currently painted on the framebuffer.
    cursor_drawn: bool,
    /// Tick counter for cursor blink; toggles every CURSOR_BLINK_INTERVAL ticks.
    blink_tick: u32,
    /// Whether the cursor should be shown (DECTCEM). Toggled by CSI ?25h / ?25l.
    cursor_visible: bool,
    /// Accumulation buffer for multi-byte UTF-8 sequences.
    utf8_buf: [u8; 4],
    /// Number of bytes accumulated so far.
    utf8_len: u8,
    /// Total bytes expected for the current UTF-8 character.
    utf8_expected: u8,
}

unsafe impl Send for FbConsole {}

impl FbConsole {
    pub fn new(fb: Framebuffer) -> Self {
        let (glyphs, unifont_data) = load_unifont_ascii();
        Self {
            fb,
            glyphs,
            extended_glyphs: BTreeMap::new(),
            unifont_data,
            cursor_x: 0,
            cursor_y: 0,
            fg: DEFAULT_FG,
            bg: DEFAULT_BG,
            ansi: AnsiState::Normal,
            active: false,
            cursor_drawn: false,
            blink_tick: 0,
            cursor_visible: true,
            utf8_buf: [0; 4],
            utf8_len: 0,
            utf8_expected: 0,
        }
    }

    fn ansi_color(code: u16, is_bg: bool) -> u32 {
        let idx = if is_bg { code.saturating_sub(40) } else { code.saturating_sub(30) };
        match idx {
            0 => 0x0000_0000,
            1 => 0x00AA_0000,
            2 => 0x0000_AA00,
            3 => 0x00AA_AA00,
            4 => 0x0000_00AA,
            5 => 0x00AA_00AA,
            6 => 0x0000_AAAA,
            7 => 0x00C0_C0C0,
            _ => {
                if is_bg {
                    DEFAULT_BG
                } else {
                    DEFAULT_FG
                }
            }
        }
    }

    fn brighten_rgb(base: u32, offset: u32) -> u32 {
        let br = (base >> 16) & 0xFF;
        let bg = (base >> 8) & 0xFF;
        let bb = base & 0xFF;
        let or = (offset >> 16) & 0xFF;
        let og = (offset >> 8) & 0xFF;
        let ob = offset & 0xFF;
        let nr = (br + or).min(0xFF);
        let ng = (bg + og).min(0xFF);
        let nb = (bb + ob).min(0xFF);
        (nr << 16) | (ng << 8) | nb
    }

    fn reset_style(&mut self) {
        self.fg = DEFAULT_FG;
        self.bg = DEFAULT_BG;
    }

    fn clear_to_bg(&mut self) {
        self.fb.clear(self.bg);
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.cursor_drawn = false;
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        if self.fb.bpp != 4 || x >= self.fb.width || y >= self.fb.height {
            return;
        }
        let pitch_px = (self.fb.pitch / 4) as usize;
        let off = (y as usize).saturating_mul(pitch_px).saturating_add(x as usize);
        unsafe { *self.fb.addr.add(off) = color };
    }

    fn draw_glyph(&mut self, g: &Glyph, x: u32, y: u32) {
        if g.len == 16 {
            for row in 0..16u32 {
                let bits = g.bytes[row as usize];
                for col in 0..8u32 {
                    let on = (bits & (0x80 >> col)) != 0;
                    self.set_pixel(x + col, y + row, if on { self.fg } else { self.bg });
                }
            }
        } else {
            for row in 0..16u32 {
                let b1 = g.bytes[(row as usize) * 2];
                let b2 = g.bytes[(row as usize) * 2 + 1];
                for col in 0..8u32 {
                    let on1 = (b1 & (0x80 >> col)) != 0;
                    let on2 = (b2 & (0x80 >> col)) != 0;
                    self.set_pixel(x + col, y + row, if on1 { self.fg } else { self.bg });
                    self.set_pixel(x + 8 + col, y + row, if on2 { self.fg } else { self.bg });
                }
            }
        }
    }

    fn scroll(&mut self) {
        if self.fb.bpp != 4 || self.fb.height <= CELL_H {
            return;
        }
        let pitch_px = (self.fb.pitch / 4) as usize;
        let row_px = (CELL_H as usize).saturating_mul(pitch_px);
        let total_px = (self.fb.height as usize).saturating_mul(pitch_px);
        if row_px >= total_px {
            self.clear_to_bg();
            return;
        }
        unsafe {
            core::ptr::copy(self.fb.addr.add(row_px), self.fb.addr, total_px - row_px);
            let tail = core::slice::from_raw_parts_mut(self.fb.addr.add(total_px - row_px), row_px);
            tail.fill(self.bg);
        }
        self.cursor_y = self.cursor_y.saturating_sub(CELL_H);
        // The scroll shifted framebuffer contents; any painted cursor is gone.
        self.cursor_drawn = false;
    }

    fn newline(&mut self) {
        self.erase_cursor();
        self.cursor_x = 0;
        self.cursor_y = self.cursor_y.saturating_add(CELL_H);
        if self.cursor_y.saturating_add(CELL_H) > self.fb.height {
            self.scroll();
        }
        self.draw_cursor();
    }

    fn put_visible_char(&mut self, ch: char) {
        self.erase_cursor();
        // Reset blink counter so cursor stays visible right after typing.
        self.blink_tick = 0;
        let code = ch as u32;
        let glyph = if (code as usize) < GLYPH_TABLE_LEN {
            self.glyphs[code as usize]
        } else if let Some(&g) = self.extended_glyphs.get(&code) {
            g
        } else {
            let g = lookup_unifont_glyph(self.unifont_data, code)
                .unwrap_or(self.glyphs[b'?' as usize]);
            self.extended_glyphs.insert(code, g);
            g
        };
        let width = glyph.width as u32;
        if self.cursor_x.saturating_add(width) > self.fb.width {
            self.newline();
        }
        self.draw_glyph(&glyph, self.cursor_x, self.cursor_y);
        self.cursor_x = self.cursor_x.saturating_add(width);
        self.draw_cursor();
    }

    /// Paint a thin vertical bar at the current cursor position.
    fn draw_cursor(&mut self) {
        if self.cursor_drawn || !self.cursor_visible {
            return;
        }
        let x = self.cursor_x;
        let y = self.cursor_y;
        for row in 0..CELL_H {
            for col in 0..CURSOR_W {
                self.set_pixel(x + col, y + row, self.fg);
            }
        }
        self.cursor_drawn = true;
    }

    /// Erase the cursor bar by painting background colour over it.
    fn erase_cursor(&mut self) {
        if !self.cursor_drawn {
            return;
        }
        let x = self.cursor_x;
        let y = self.cursor_y;
        for row in 0..CELL_H {
            for col in 0..CURSOR_W {
                self.set_pixel(x + col, y + row, self.bg);
            }
        }
        self.cursor_drawn = false;
    }

    fn apply_csi(&mut self, final_byte: u8, params: &[u16], private: bool) {
        // DEC Private Mode: CSI ? <n> h/l
        if private {
            match final_byte {
                b'h' | b'l' => {
                    let show = final_byte == b'h';
                    for &p in params {
                        if p == 25 {
                            // DECTCEM — cursor visibility
                            if show {
                                self.cursor_visible = true;
                                self.draw_cursor();
                            } else {
                                self.erase_cursor();
                                self.cursor_visible = false;
                            }
                        }
                    }
                }
                _ => {}
            }
            return;
        }
        match final_byte {
            b'm' => {
                let mut had_any = false;
                for &p in params {
                    had_any = true;
                    if p == 0 {
                        self.reset_style();
                    } else if (30..=37).contains(&p) {
                        self.fg = Self::ansi_color(p, false);
                    } else if (40..=47).contains(&p) {
                        self.bg = Self::ansi_color(p, true);
                    } else if (90..=97).contains(&p) {
                        // ANSI bright foreground: add a small RGB brightness offset.
                        self.fg =
                            Self::brighten_rgb(Self::ansi_color(p - 60, false), BRIGHT_FG_OFFSET);
                    } else if (100..=107).contains(&p) {
                        // ANSI bright background: add a smaller RGB brightness offset.
                        self.bg =
                            Self::brighten_rgb(Self::ansi_color(p - 60, true), BRIGHT_BG_OFFSET);
                    }
                }
                if !had_any {
                    self.reset_style();
                }
            }
            b'J' => {
                self.erase_cursor();
                self.clear_to_bg();
                self.draw_cursor();
            }
            b'H' | b'f' => {
                self.erase_cursor();
                let row = params.first().copied().unwrap_or(1).max(1) as u32;
                let col = params.get(1).copied().unwrap_or(1).max(1) as u32;
                self.cursor_y =
                    (row - 1).saturating_mul(CELL_H).min(self.fb.height.saturating_sub(1));
                self.cursor_x =
                    (col - 1).saturating_mul(CELL_W).min(self.fb.width.saturating_sub(1));
                self.draw_cursor();
            }
            _ => {}
        }
    }

    fn handle_byte(&mut self, b: u8) {
        // If we're accumulating a multi-byte UTF-8 sequence, try to continue.
        if self.utf8_expected > 0 {
            if b & 0xC0 == 0x80 {
                self.utf8_buf[self.utf8_len as usize] = b;
                self.utf8_len += 1;
                if self.utf8_len == self.utf8_expected {
                    let ch = decode_utf8_char(&self.utf8_buf[..self.utf8_len as usize]);
                    self.utf8_expected = 0;
                    self.utf8_len = 0;
                    self.put_visible_char(ch.unwrap_or('?'));
                }
                return;
            } else {
                // Invalid continuation — reset and process byte normally.
                self.utf8_expected = 0;
                self.utf8_len = 0;
            }
        }

        match &mut self.ansi {
            AnsiState::Normal => match b {
                0x1B => self.ansi = AnsiState::Esc,
                b'\n' => self.newline(),
                b'\r' => {
                    self.erase_cursor();
                    self.cursor_x = 0;
                    self.draw_cursor();
                }
                0x08 => {
                    self.erase_cursor();
                    self.cursor_x = self.cursor_x.saturating_sub(CELL_W);
                    self.draw_cursor();
                }
                b'\t' => {
                    for _ in 0..TAB_WIDTH {
                        self.put_visible_char(' ');
                    }
                }
                // UTF-8 lead bytes — start accumulation.
                0xC0..=0xDF => {
                    self.utf8_buf[0] = b;
                    self.utf8_len = 1;
                    self.utf8_expected = 2;
                }
                0xE0..=0xEF => {
                    self.utf8_buf[0] = b;
                    self.utf8_len = 1;
                    self.utf8_expected = 3;
                }
                0xF0..=0xF7 => {
                    self.utf8_buf[0] = b;
                    self.utf8_len = 1;
                    self.utf8_expected = 4;
                }
                GLYPH_PRELOAD_START_U8..=0x7E => self.put_visible_char(b as char),
                _ => {}
            },
            AnsiState::Esc => {
                if b == b'[' {
                    self.ansi = AnsiState::Csi {
                        params: [0; CSI_PARAM_CAP],
                        len: 0,
                        cur: None,
                        private: false,
                    };
                } else {
                    self.ansi = AnsiState::Normal;
                }
            }
            AnsiState::Csi { params, len, cur, private } => {
                if b == b'?' && *len == 0 && cur.is_none() {
                    *private = true;
                    return;
                }
                if b.is_ascii_digit() {
                    let d = (b - b'0') as u16;
                    // Saturate at `u16::MAX` to keep parser storage fixed-size.
                    *cur = Some(cur.unwrap_or(0).saturating_mul(10).saturating_add(d));
                    return;
                }
                if b == b';' {
                    if *len < params.len() {
                        params[*len] = cur.unwrap_or(0);
                        *len += 1;
                    }
                    *cur = None;
                    return;
                }
                if *len < params.len() {
                    params[*len] = cur.unwrap_or(0);
                    *len += 1;
                }
                let is_private = *private;
                let mut used = [0u16; CSI_PARAM_CAP];
                let copy_len = (*len).min(CSI_PARAM_CAP);
                used[..copy_len].copy_from_slice(&params[..copy_len]);
                self.apply_csi(b, &used[..copy_len], is_private);
                self.ansi = AnsiState::Normal;
            }
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.reset_style();
        self.clear_to_bg();
        // Reset style and print the terminal activation banner.
        for &b in ACTIVATION_BANNER {
            self.handle_byte(b);
        }
        self.draw_cursor();
    }

    /// Toggle cursor visibility. Call from the periodic timer tick.
    fn blink(&mut self) {
        if !self.active || !self.cursor_visible {
            return;
        }
        self.blink_tick = self.blink_tick.wrapping_add(1);
        if self.blink_tick % CURSOR_BLINK_INTERVAL == 0 {
            if self.cursor_drawn {
                self.erase_cursor();
            } else {
                self.draw_cursor();
            }
        }
    }

    pub fn put_char(&mut self, c: u8) {
        if !self.active {
            return;
        }
        self.handle_byte(c);
    }
}

/// Decode a complete UTF-8 byte sequence (2–4 bytes) into a `char`.
fn decode_utf8_char(buf: &[u8]) -> Option<char> {
    let cp = match buf.len() {
        2 => ((buf[0] as u32 & 0x1F) << 6) | (buf[1] as u32 & 0x3F),
        3 => {
            ((buf[0] as u32 & 0x0F) << 12) | ((buf[1] as u32 & 0x3F) << 6) | (buf[2] as u32 & 0x3F)
        }
        4 => {
            ((buf[0] as u32 & 0x07) << 18)
                | ((buf[1] as u32 & 0x3F) << 12)
                | ((buf[2] as u32 & 0x3F) << 6)
                | (buf[3] as u32 & 0x3F)
        }
        _ => return None,
    };
    char::from_u32(cp)
}

/// Parse one hexadecimal ASCII nibble into its numeric value.
fn parse_hex_nibble(v: u8) -> Option<u8> {
    match v {
        b'0'..=b'9' => Some(v - b'0'),
        b'a'..=b'f' => Some(v - b'a' + 10),
        b'A'..=b'F' => Some(v - b'A' + 10),
        _ => None,
    }
}

/// Parse an ASCII hexadecimal string into `u32`.
fn parse_hex_u32(bytes: &[u8]) -> Option<u32> {
    let mut v = 0u32;
    for &b in bytes {
        let n = parse_hex_nibble(b)? as u32;
        v = v.checked_mul(16)?.checked_add(n)?;
    }
    Some(v)
}

/// Parse two hexadecimal ASCII characters into one byte.
fn parse_hex_byte_pair(hi: u8, lo: u8) -> Option<u8> {
    let h = parse_hex_nibble(hi)?;
    let l = parse_hex_nibble(lo)?;
    Some((h << 4) | l)
}

/// Load ASCII-range glyphs from the boot module that contains `unifont.hex`.
///
/// Returns an empty map when the module is missing or invalid so the console
/// can degrade gracefully instead of failing boot.
fn fallback_glyph() -> Glyph {
    let mut bytes = [0u8; 32];
    // Minimal 8x16 placeholder frame.
    for row in 0..16usize {
        bytes[row] = if row == 0 || row == 15 { 0x7E } else { 0x42 };
    }
    Glyph { width: 8, bytes, len: 16 }
}

fn empty_glyph() -> Glyph {
    Glyph { width: 8, bytes: [0; 32], len: 16 }
}

/// Parse a unifont hex bitmap line (after the ':') into a Glyph.
fn parse_hex_glyph(hex: &[u8]) -> Option<Glyph> {
    if hex.len() != 32 && hex.len() != 64 {
        return None;
    }
    let mut bytes = [0u8; 32];
    let pairs = hex.len() / 2;
    for i in 0..pairs {
        bytes[i] = parse_hex_byte_pair(hex[i * 2], hex[i * 2 + 1])?;
    }
    Some(Glyph { width: if hex.len() == 32 { 8 } else { 16 }, bytes, len: pairs as u8 })
}

/// Search the raw unifont hex data for a specific codepoint and parse its glyph.
fn lookup_unifont_glyph(data: &[u8], target: u32) -> Option<Glyph> {
    let mut start = 0usize;
    while start < data.len() {
        let mut end = start;
        while end < data.len() && data[end] != b'\n' {
            end += 1;
        }
        let line = &data[start..end];
        start = end.saturating_add(1);
        if line.is_empty() {
            continue;
        }
        let mut sep = 0usize;
        while sep < line.len() && line[sep] != b':' {
            sep += 1;
        }
        if sep == 0 || sep + 1 >= line.len() {
            continue;
        }
        let Some(code) = parse_hex_u32(&line[..sep]) else {
            continue;
        };
        if code == target {
            return parse_hex_glyph(&line[sep + 1..]);
        }
        // The hex file is sorted; stop early if we've passed the target.
        if code > target {
            return None;
        }
    }
    None
}

fn load_unifont_ascii() -> ([Glyph; GLYPH_TABLE_LEN], &'static [u8]) {
    let mut out = [empty_glyph(); GLYPH_TABLE_LEN];
    let fallback = fallback_glyph();
    out[b'?' as usize] = fallback;

    let modules = crate::RUNTIME.modules();
    let unifont = modules
        .iter()
        .find(|m| m.name.ends_with("/unifont.hex") || m.name.ends_with("unifont.hex"));
    let Some(module) = unifont else {
        return (out, &[]);
    };
    let data = module.bytes;
    let mut start = 0usize;
    while start < data.len() {
        let mut end = start;
        while end < data.len() && data[end] != b'\n' {
            end += 1;
        }
        let line = &data[start..end];
        start = end.saturating_add(1);
        if line.is_empty() {
            continue;
        }
        let mut sep = 0usize;
        while sep < line.len() && line[sep] != b':' {
            sep += 1;
        }
        if sep == 0 || sep + 1 >= line.len() {
            continue;
        }
        let Some(code) = parse_hex_u32(&line[..sep]) else {
            continue;
        };
        // Restrict preloaded glyphs to the byte-oriented terminal render range.
        if !(GLYPH_PRELOAD_START..=GLYPH_PRELOAD_END).contains(&code) {
            continue;
        }
        if (code as usize) >= GLYPH_TABLE_LEN {
            continue;
        }
        if let Some(g) = parse_hex_glyph(&line[sep + 1..]) {
            out[code as usize] = g;
        }
    }
    (out, data)
}

pub fn init(fb: Framebuffer) {
    let state = crate::RUNTIME.irq_disable();
    *CONSOLE.lock() = Some(FbConsole::new(fb));
    crate::RUNTIME.irq_restore(state);
}

pub fn activate_onscreen_terminal() {
    if CONSOLE_DISABLED.load(Ordering::Relaxed) {
        return;
    }
    let state = crate::RUNTIME.irq_disable();
    if let Some(ref mut console) = *CONSOLE.lock() {
        console.activate();
    }
    crate::RUNTIME.irq_restore(state);
}

pub fn disable() {
    CONSOLE_DISABLED.store(true, Ordering::Relaxed);
}

pub fn is_disabled() -> bool {
    CONSOLE_DISABLED.load(Ordering::Relaxed)
}

pub fn put_char(c: u8) {
    if CONSOLE_DISABLED.load(Ordering::Relaxed) {
        return;
    }
    let state = crate::RUNTIME.irq_disable();
    if let Some(ref mut console) = *CONSOLE.lock() {
        console.put_char(c);
    }
    crate::RUNTIME.irq_restore(state);
}

/// Called from the timer IRQ to blink the cursor.
pub fn blink_cursor() {
    if CONSOLE_DISABLED.load(Ordering::Relaxed) {
        return;
    }
    // Use try_lock to avoid deadlock if the console is already held.
    if let Some(ref mut guard) = CONSOLE.try_lock() {
        if let Some(ref mut console) = **guard {
            console.blink();
        }
    }
}

pub unsafe fn force_unlock() {
    unsafe {
        CONSOLE.force_unlock();
    }
}
