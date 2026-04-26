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

/// Lock to ensure only one CPU flushes the serial ring buffer at a time.
/// This prevents chunk-level reordering of logs from different CPUs.
static SERIAL_FLUSH_LOCK: Mutex<()> = Mutex::new(());

// ---------------------------------------------------------------------------
// Deferred output ring buffer
//
// `put_char()` and `put_buf()` push bytes here instead of rendering glyphs
// immediately.  The actual framebuffer rendering happens in
// `flush_deferred()`, which is called from the timer tick (IRQ context) with
// `try_lock` so it never blocks an interrupt.
// ---------------------------------------------------------------------------
const DEFERRED_CAP: usize = 8192;

struct DeferredRing {
    buf: [u8; DEFERRED_CAP],
    head: usize,
    len: usize,
}

impl DeferredRing {
    const fn new() -> Self {
        Self { buf: [0; DEFERRED_CAP], head: 0, len: 0 }
    }

    fn push(&mut self, b: u8) {
        let tail = (self.head + self.len) % DEFERRED_CAP;
        self.buf[tail] = b;
        if self.len < DEFERRED_CAP {
            self.len += 1;
        } else {
            // Overwrite oldest byte.
            self.head = (self.head + 1) % DEFERRED_CAP;
        }
    }

    fn push_slice(&mut self, data: &[u8]) {
        for &b in data {
            self.push(b);
        }
    }

    /// Drain up to `out.len()` bytes. Returns count drained.
    fn drain(&mut self, out: &mut [u8]) -> usize {
        let n = self.len.min(out.len());
        for i in 0..n {
            out[i] = self.buf[self.head];
            self.head = (self.head + 1) % DEFERRED_CAP;
        }
        self.len -= n;
        n
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Prepend bytes at the *front* of the ring so they are drained first.
    ///
    /// This is used when bytes were drained for rendering but could not be
    /// rendered (because the console lock was busy).  Putting them back at the
    /// front preserves the original output order: the re-enqueued bytes will be
    /// consumed before any bytes that were pushed while the lock was contended.
    ///
    /// If there is not enough free space for all of `data`, the oldest (leading)
    /// bytes are silently discarded.
    fn push_front_slice(&mut self, data: &[u8]) {
        let space = DEFERRED_CAP - self.len;
        let n = data.len().min(space);
        if n == 0 {
            return;
        }
        // If we must truncate, skip the leading (earliest-enqueued) bytes in
        // `data` so we keep the bytes that are closest to the current ring head.
        let skip = data.len() - n;
        self.head = (self.head + DEFERRED_CAP - n) % DEFERRED_CAP;
        for (i, &b) in data[skip..].iter().enumerate() {
            self.buf[(self.head + i) % DEFERRED_CAP] = b;
        }
        self.len += n;
    }
}

static DEFERRED: Mutex<DeferredRing> = Mutex::new(DeferredRing::new());
static SERIAL_DEFERRED: Mutex<DeferredRing> = Mutex::new(DeferredRing::new());

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
            let g =
                lookup_unifont_glyph(self.unifont_data, code).unwrap_or(self.glyphs[b'?' as usize]);
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
    DEFERRED.lock().push(c);
}

pub fn serial_put_char(c: u8) {
    if CONSOLE_DISABLED.load(Ordering::Relaxed) {
        return;
    }
    SERIAL_DEFERRED.lock().push(c);
}

/// Enqueue a byte slice for deferred framebuffer rendering.
pub fn put_buf(buf: &[u8]) {
    if CONSOLE_DISABLED.load(Ordering::Relaxed) {
        return;
    }
    DEFERRED.lock().push_slice(buf);
}

pub fn serial_put_buf(buf: &[u8]) {
    if CONSOLE_DISABLED.load(Ordering::Relaxed) {
        return;
    }
    SERIAL_DEFERRED.lock().push_slice(buf);
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

/// Drain the deferred ring buffer and render to the framebuffer console.
///
/// Called from the timer tick (and optionally from idle or explicit flush
/// points).  Uses `try_lock` on both the ring buffer and the console so it
/// never blocks an IRQ handler.
///
/// Processes up to `FLUSH_BATCH` bytes per call so a burst of log output
/// doesn't monopolise the timer ISR.  Remaining bytes will be rendered on
/// the next tick or during idle-time flushing.
const FLUSH_BATCH: usize = 64;

/// Larger batch used by `flush_deferred_idle()` — called from the
/// scheduler idle loop where there is no urgency to return quickly.
const FLUSH_BATCH_IDLE: usize = 2048;

pub fn flush_deferred() {
    if CONSOLE_DISABLED.load(Ordering::Relaxed) {
        return;
    }

    let mut local = [0u8; FLUSH_BATCH];

    // Drain from the ring buffer (very short lock).
    let n = {
        let mut ring = match DEFERRED.try_lock() {
            Some(r) => r,
            None => return,
        };
        if ring.is_empty() {
            return;
        }
        ring.drain(&mut local)
    };

    if n == 0 {
        return;
    }

    // Render under the CONSOLE lock.
    if let Some(ref mut guard) = CONSOLE.try_lock() {
        if let Some(ref mut console) = **guard {
            for &b in &local[..n] {
                console.put_char(b);
            }
        }
    } else {
        // Console is busy — re-enqueue bytes at the *front* of the ring so
        // they are rendered before any bytes that arrived in the meantime,
        // preserving output order.
        if let Some(mut ring) = DEFERRED.try_lock() {
            ring.push_front_slice(&local[..n]);
        }
        // If we can't re-lock the ring either, the bytes are lost — acceptable
        // for a debug console under extreme contention.
    }
}

/// Idle-time flush — drains a larger batch from the deferred ring buffer.
///
/// Called from the scheduler idle loop (when the CPU has no runnable tasks)
/// so that console rendering happens without delaying real work.  Still
/// uses `try_lock` so it never blocks an IRQ handler.
pub fn flush_deferred_idle() {
    if CONSOLE_DISABLED.load(Ordering::Relaxed) {
        return;
    }

    let mut local = [0u8; FLUSH_BATCH_IDLE];

    let n = {
        let mut ring = match DEFERRED.try_lock() {
            Some(r) => r,
            None => return,
        };
        if ring.is_empty() {
            return;
        }
        ring.drain(&mut local)
    };

    if n == 0 {
        return;
    }

    if let Some(ref mut guard) = CONSOLE.try_lock() {
        if let Some(ref mut console) = **guard {
            for &b in &local[..n] {
                console.put_char(b);
            }
        }
    } else {
        if let Some(mut ring) = DEFERRED.try_lock() {
            ring.push_front_slice(&local[..n]);
        }
    }
}

/// Synchronous flush — drains the *entire* deferred ring buffer and renders
/// immediately.  Intended for panic paths where we need output NOW.
/// Callers should disable IRQs before calling if needed.
pub fn flush_sync() {
    let mut local = [0u8; FLUSH_BATCH];
    loop {
        let n = DEFERRED.lock().drain(&mut local);
        if n == 0 {
            break;
        }
        if let Some(ref mut console) = *CONSOLE.lock() {
            for &b in &local[..n] {
                console.put_char(b);
            }
        }
    }
}

pub fn serial_flush_deferred() {
    if CONSOLE_DISABLED.load(Ordering::Relaxed) {
        return;
    }

    // Ensure only one CPU flushes at a time to prevent reordering.
    let _flush_guard = match SERIAL_FLUSH_LOCK.try_lock() {
        Some(g) => g,
        None => return,
    };

    let mut local = [0u8; 1024];
    loop {
        let n = {
            let mut ring = match SERIAL_DEFERRED.try_lock() {
                Some(r) => r,
                None => return,
            };
            if ring.is_empty() {
                return;
            }
            ring.drain(&mut local)
        };

        if n > 0 {
            // Drain directly to serial hardware — batch write to avoid interleaving.
            // Holding SERIAL_FLUSH_LOCK ensures that even if serial_putbuf_sync
            // drops the hardware lock, no other CPU can start a new flush and
            // overtake us.
            crate::RUNTIME.serial_putbuf_sync(&local[..n]);
        } else {
            break;
        }
    }
}

pub fn serial_flush_deferred_idle() {
    if CONSOLE_DISABLED.load(Ordering::Relaxed) {
        return;
    }

    // Ensure only one CPU flushes at a time to prevent reordering.
    let _flush_guard = match SERIAL_FLUSH_LOCK.try_lock() {
        Some(g) => g,
        None => return,
    };

    let mut local = [0u8; 2048];
    loop {
        let n = {
            let mut ring = match SERIAL_DEFERRED.try_lock() {
                Some(r) => r,
                None => return,
            };
            if ring.is_empty() {
                return;
            }
            ring.drain(&mut local)
        };

        if n > 0 {
            crate::RUNTIME.serial_putbuf_sync(&local[..n]);
        } else {
            break;
        }
    }
}

pub fn serial_flush_sync() {
    let mut local = [0u8; FLUSH_BATCH];
    loop {
        let n = SERIAL_DEFERRED.lock().drain(&mut local);
        if n == 0 {
            break;
        }
        crate::RUNTIME.serial_putbuf_sync(&local[..n]);
    }
}

pub unsafe fn force_unlock() {
    unsafe {
        CONSOLE.force_unlock();
        DEFERRED.force_unlock();
        SERIAL_DEFERRED.force_unlock();
    }
}

#[cfg(test)]
mod tests {
    use super::{DeferredRing, DEFERRED_CAP};

    fn drain_all(ring: &mut DeferredRing, out: &mut [u8]) -> usize {
        ring.drain(out)
    }

    #[test]
    fn push_front_slice_preserves_order() {
        let mut ring = DeferredRing::new();
        // Simulate: "line1\n" was drained but could not be rendered.
        // Meanwhile "line2\n" was pushed.
        ring.push_slice(b"line2\n");
        // Re-enqueue "line1\n" at the front.
        ring.push_front_slice(b"line1\n");
        // Drain should give line1 before line2.
        let mut out = [0u8; 12];
        let n = drain_all(&mut ring, &mut out);
        assert_eq!(&out[..n], b"line1\nline2\n");
    }

    #[test]
    fn push_front_slice_wraps_correctly() {
        // Fill the ring almost full so the head wraps around the array boundary.
        let mut ring = DeferredRing::new();
        // Advance head to near the end of the buffer.
        let filler = [b'x'; DEFERRED_CAP - 4];
        ring.push_slice(&filler);
        let mut discard = [0u8; DEFERRED_CAP];
        ring.drain(&mut discard[..DEFERRED_CAP - 4]);
        // Ring is now empty but head is at DEFERRED_CAP - 4.
        ring.push_slice(b"new ");
        ring.push_front_slice(b"old ");
        let mut out = [0u8; 8];
        let n = drain_all(&mut ring, &mut out);
        assert_eq!(&out[..n], b"old new ");
    }

    #[test]
    fn push_front_slice_truncates_to_free_space() {
        // Fill all but 3 bytes.
        let mut ring = DeferredRing::new();
        let filler = [b'y'; DEFERRED_CAP - 3];
        ring.push_slice(&filler);
        // Try to prepend 6 bytes — only 3 should fit (oldest 3 are dropped).
        ring.push_front_slice(b"abcdef");
        // The ring should be full.
        assert_eq!(ring.len, DEFERRED_CAP);
        // Drain first 3 bytes — should be "def" (last 3 of "abcdef").
        let mut head = [0u8; 3];
        ring.drain(&mut head);
        assert_eq!(&head, b"def");
    }

    #[test]
    fn push_front_slice_empty_data_is_noop() {
        let mut ring = DeferredRing::new();
        ring.push_slice(b"hello");
        ring.push_front_slice(b"");
        let mut out = [0u8; 5];
        let n = drain_all(&mut ring, &mut out);
        assert_eq!(&out[..n], b"hello");
    }

    #[test]
    fn push_front_slice_full_ring_drops_all() {
        let mut ring = DeferredRing::new();
        let filler = [b'z'; DEFERRED_CAP];
        ring.push_slice(&filler);
        // Ring is full — push_front_slice must not corrupt it.
        ring.push_front_slice(b"ignored");
        assert_eq!(ring.len, DEFERRED_CAP);
        let mut out = [0u8; DEFERRED_CAP];
        let n = drain_all(&mut ring, &mut out);
        assert!(out[..n].iter().all(|&b| b == b'z'));
    }
}
