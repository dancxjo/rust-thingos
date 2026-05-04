use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};

use kernel::{BootRuntime, BootRuntimeBase};
use spin::Mutex;

use crate::framebuffer::Framebuffer;
use crate::runtime::ArchRuntime as _;

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
const WRAP_CLEAR_ROW_DIVISOR: u32 = 2;

pub static CONSOLE: Mutex<Option<FbConsole>> = Mutex::new(None);
pub static FB_CONSOLE_DISABLED: AtomicBool = AtomicBool::new(false);

/// Lock to ensure only one CPU flushes the serial ring buffer at a time.
/// This prevents chunk-level reordering of logs from different CPUs without
/// blocking producers or IRQ handlers.
static SERIAL_FLUSH_ACTIVE: AtomicBool = AtomicBool::new(false);

/// Lock to ensure only one CPU flushes the framebuffer ring buffer at a time.
/// Rendering uses the console lock, but draining happens first; this keeps
/// dequeued chunks from being rendered out of order by competing CPUs.
static FB_FLUSH_ACTIVE: AtomicBool = AtomicBool::new(false);

// ---------------------------------------------------------------------------
// Deferred output ring buffer
//
// `put_char()` and `put_buf()` push bytes here instead of rendering glyphs
// immediately.  The actual framebuffer rendering happens in
// `flush_deferred()`, which is called from the timer tick (IRQ context) with
// `try_lock` so it never blocks an interrupt.
// ---------------------------------------------------------------------------
const DEFERRED_CAP: usize = 8192;
const SERIAL_DEFERRED_CAP: usize = 65536; // 64 KiB — sized for heavy multi-CPU logging

struct RingSlot {
    seq: AtomicUsize,
    byte: AtomicU8,
}

impl RingSlot {
    const fn new() -> Self {
        Self { seq: AtomicUsize::new(0), byte: AtomicU8::new(0) }
    }
}

struct DeferredRing<const CAP: usize> {
    slots: [RingSlot; CAP],
    write: AtomicUsize,
    read: AtomicUsize,
}

impl<const CAP: usize> DeferredRing<CAP> {
    const fn new() -> Self {
        Self {
            slots: [const { RingSlot::new() }; CAP],
            write: AtomicUsize::new(0),
            read: AtomicUsize::new(0),
        }
    }

    #[inline]
    fn publish_drop_floor(&self, floor: usize) {
        let mut current = self.read.load(Ordering::Acquire);
        while current < floor {
            match self.read.compare_exchange_weak(
                current,
                floor,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(actual) => current = actual,
            }
        }
    }

    fn push(&self, b: u8) {
        let seq = self.write.fetch_add(1, Ordering::AcqRel);
        if seq >= CAP {
            self.publish_drop_floor(seq - CAP + 1);
        }
        let slot = &self.slots[seq % CAP];
        slot.byte.store(b, Ordering::Relaxed);
        slot.seq.store(seq.wrapping_add(1), Ordering::Release);
    }

    fn push_slice(&self, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        let start = self.write.fetch_add(data.len(), Ordering::AcqRel);
        let end = start + data.len();
        if end > CAP {
            self.publish_drop_floor(end - CAP);
        }

        let keep_from = data.len().saturating_sub(CAP);
        for (offset, &b) in data.iter().enumerate().skip(keep_from) {
            let seq = start + offset;
            let slot = &self.slots[seq % CAP];
            slot.byte.store(b, Ordering::Relaxed);
            slot.seq.store(seq.wrapping_add(1), Ordering::Release);
        }
    }

    /// Drain up to `out.len()` committed bytes. Returns count drained.
    fn drain(&self, out: &mut [u8]) -> usize {
        let mut n = 0;
        while n < out.len() {
            let write = self.write.load(Ordering::Acquire);
            let mut read = self.read.load(Ordering::Acquire);
            if read >= write {
                break;
            }

            if write - read > CAP {
                self.publish_drop_floor(write - CAP);
                continue;
            }

            let slot = &self.slots[read % CAP];
            if slot.seq.load(Ordering::Acquire) != read.wrapping_add(1) {
                break;
            }
            let byte = slot.byte.load(Ordering::Relaxed);
            if slot.seq.load(Ordering::Acquire) != read.wrapping_add(1) {
                continue;
            }

            match self.read.compare_exchange_weak(
                read,
                read + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    out[n] = byte;
                    n += 1;
                }
                Err(actual) => read = actual,
            }
            let _ = read;
        }
        n
    }

    fn is_empty(&self) -> bool {
        self.read.load(Ordering::Acquire) >= self.write.load(Ordering::Acquire)
    }
}

struct FlushGuard(&'static AtomicBool);

impl FlushGuard {
    fn try_enter(flag: &'static AtomicBool) -> Option<Self> {
        flag.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .ok()
            .map(|_| Self(flag))
    }
}

impl Drop for FlushGuard {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}

static DEFERRED: DeferredRing<DEFERRED_CAP> = DeferredRing::new();
static SERIAL_DEFERRED: DeferredRing<SERIAL_DEFERRED_CAP> = DeferredRing::new();

fn disarm_serial_tx_if_idle() {
    crate::RUNTIME.arch.disarm_serial_tx_irq();
    if !SERIAL_DEFERRED.is_empty() {
        crate::RUNTIME.arch.arm_serial_tx_irq();
    }
}

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
            2 => 0x00FF_CC00, // Golden
            3 => 0x00AA_AA00,
            4 => 0x0000_00AA,
            5 => 0x00AA_00AA,
            6 => 0x00AA_88FF, // Lavender
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

    fn text_rows(&self) -> u32 {
        (self.fb.height / CELL_H).max(1)
    }

    fn cursor_row(&self) -> u32 {
        (self.cursor_y / CELL_H).min(self.text_rows().saturating_sub(1))
    }

    fn fill_pixel_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: u32) {
        if self.fb.bpp != 4
            || x >= self.fb.width
            || y >= self.fb.height
            || width == 0
            || height == 0
        {
            return;
        }

        let pitch_px = (self.fb.pitch / 4) as usize;
        if pitch_px == 0 {
            return;
        }

        let pitch_limit = (pitch_px as u32).saturating_sub(x);
        let start_col = x as usize;
        let cols = width.min(self.fb.width - x).min(pitch_limit) as usize;
        if cols == 0 {
            return;
        }
        let start_row = y as usize;
        let end_row = y.saturating_add(height).min(self.fb.height) as usize;
        for row in start_row..end_row {
            let start = row.saturating_mul(pitch_px).saturating_add(start_col);
            unsafe {
                let pixels = core::slice::from_raw_parts_mut(self.fb.addr.add(start), cols);
                pixels.fill(color);
            }
        }
    }

    fn clear_pixel_band(&mut self, y: u32, height: u32) {
        self.fill_pixel_rect(0, y, self.fb.width, height, self.bg);
    }

    fn clear_text_rows(&mut self, start: u32, count: u32) {
        let rows = self.text_rows();
        if start >= rows || count == 0 {
            return;
        }
        let count = count.min(rows - start);
        self.clear_pixel_band(start.saturating_mul(CELL_H), count.saturating_mul(CELL_H));
    }

    fn clear_text_row(&mut self, row: u32) {
        self.clear_text_rows(row, 1);
    }

    fn draw_bits_row(&mut self, bits: u16, bit_width: u32, x: u32, y: u32) {
        if self.fb.bpp != 4 || x >= self.fb.width || y >= self.fb.height || bit_width == 0 {
            return;
        }

        let pitch_px = (self.fb.pitch / 4) as usize;
        if pitch_px == 0 {
            return;
        }

        let pitch_limit = (pitch_px as u32).saturating_sub(x);
        let cols = bit_width.min(self.fb.width - x).min(pitch_limit) as usize;
        if cols == 0 {
            return;
        }
        let start = (y as usize).saturating_mul(pitch_px).saturating_add(x as usize);
        unsafe {
            let pixels = core::slice::from_raw_parts_mut(self.fb.addr.add(start), cols);
            for (col, pixel) in pixels.iter_mut().enumerate() {
                let mask = 1u16 << (bit_width - 1 - col as u32);
                *pixel = if (bits & mask) != 0 { self.fg } else { self.bg };
            }
        }
    }

    fn draw_glyph(&mut self, g: &Glyph, x: u32, y: u32) {
        if g.len == 16 {
            for row in 0..16u32 {
                self.draw_bits_row(g.bytes[row as usize] as u16, 8, x, y + row);
            }
        } else {
            for row in 0..16u32 {
                let b1 = g.bytes[(row as usize) * 2];
                let b2 = g.bytes[(row as usize) * 2 + 1];
                let bits = ((b1 as u16) << 8) | b2 as u16;
                self.draw_bits_row(bits, 16, x, y + row);
            }
        }
    }

    fn newline(&mut self) {
        self.erase_cursor();
        self.cursor_x = 0;
        let row = self.cursor_row();
        let rows = self.text_rows();
        if row.saturating_add(1) >= rows {
            // Avoid a full-framebuffer scroll on every bottom-row newline.
            self.cursor_y = 0;
            let clear_rows = (rows / WRAP_CLEAR_ROW_DIVISOR).max(1);
            self.clear_text_rows(0, clear_rows);
        } else {
            let next_row = row + 1;
            self.cursor_y = next_row.saturating_mul(CELL_H);
            self.clear_text_row(next_row);
        }
        self.cursor_drawn = false;
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
        self.fill_pixel_rect(self.cursor_x, self.cursor_y, CURSOR_W, CELL_H, self.fg);
        self.cursor_drawn = true;
    }

    /// Erase the cursor bar by painting background colour over it.
    fn erase_cursor(&mut self) {
        if !self.cursor_drawn {
            return;
        }
        self.fill_pixel_rect(self.cursor_x, self.cursor_y, CURSOR_W, CELL_H, self.bg);
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
    if FB_CONSOLE_DISABLED.load(Ordering::Relaxed) {
        return;
    }
    let state = crate::RUNTIME.irq_disable();
    if let Some(ref mut console) = *CONSOLE.lock() {
        console.activate();
    }
    crate::RUNTIME.irq_restore(state);
}

pub fn disable() {
    FB_CONSOLE_DISABLED.store(true, Ordering::Relaxed);
}

pub fn is_disabled() -> bool {
    FB_CONSOLE_DISABLED.load(Ordering::Relaxed)
}

pub fn put_char(c: u8) {
    if FB_CONSOLE_DISABLED.load(Ordering::Relaxed) {
        return;
    }
    DEFERRED.push(c);
}

pub fn serial_put_char(c: u8) {
    SERIAL_DEFERRED.push(c);
    // Kick-start: arm the TX interrupt so the serial IRQ handler drains the
    // ring asynchronously.  Producers deliberately do not touch UART readiness
    // or write FIFO bursts; timer and idle flush paths cover platforms without
    // TX interrupts.
    crate::RUNTIME.arch.arm_serial_tx_irq();
}

/// Enqueue a byte slice for deferred framebuffer rendering.
pub fn put_buf(buf: &[u8]) {
    if FB_CONSOLE_DISABLED.load(Ordering::Relaxed) {
        return;
    }
    DEFERRED.push_slice(buf);
}

pub fn serial_put_buf(buf: &[u8]) {
    SERIAL_DEFERRED.push_slice(buf);
    // Kick-start TX interrupt for async drain without making the producer
    // perform UART I/O on the hot path.
    crate::RUNTIME.arch.arm_serial_tx_irq();
}

/// Called from the timer IRQ to blink the cursor.
pub fn blink_cursor() {
    if FB_CONSOLE_DISABLED.load(Ordering::Relaxed) {
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
/// points).  Producers reserve ring slots atomically; rendering uses
/// `try_lock` on the console so it never blocks an IRQ handler.
///
/// Processes up to `FLUSH_BATCH` bytes per call so a burst of log output
/// doesn't monopolise the timer ISR.  Remaining bytes will be rendered on
/// the next tick or during idle-time flushing.
const FLUSH_BATCH: usize = 64;

/// Larger batch used by `flush_deferred_idle()` — called from the
/// scheduler idle loop where there is no urgency to return quickly.
const FLUSH_BATCH_IDLE: usize = 2048;

pub fn flush_deferred() {
    if FB_CONSOLE_DISABLED.load(Ordering::Relaxed) {
        return;
    }

    let _flush_guard = match FlushGuard::try_enter(&FB_FLUSH_ACTIVE) {
        Some(g) => g,
        None => return,
    };

    let mut local = [0u8; FLUSH_BATCH];

    // Try the framebuffer console first. If it is busy, leave bytes queued.
    if let Some(ref mut guard) = CONSOLE.try_lock() {
        if let Some(ref mut console) = **guard {
            let n = DEFERRED.drain(&mut local);
            if n == 0 {
                return;
            }
            for &b in &local[..n] {
                console.put_char(b);
            }
        }
    }
}

/// Idle-time flush — drains a larger batch from the deferred ring buffer.
///
/// Called from the scheduler idle loop (when the CPU has no runnable tasks)
/// so that console rendering happens without delaying real work.  Still
/// uses `try_lock` so it never blocks an IRQ handler.
pub fn flush_deferred_idle() {
    if FB_CONSOLE_DISABLED.load(Ordering::Relaxed) {
        return;
    }

    let _flush_guard = match FlushGuard::try_enter(&FB_FLUSH_ACTIVE) {
        Some(g) => g,
        None => return,
    };

    let mut local = [0u8; FLUSH_BATCH_IDLE];

    if let Some(ref mut guard) = CONSOLE.try_lock() {
        if let Some(ref mut console) = **guard {
            let n = DEFERRED.drain(&mut local);
            if n == 0 {
                return;
            }
            for &b in &local[..n] {
                console.put_char(b);
            }
        }
    }
}

/// Synchronous flush — drains the *entire* deferred ring buffer and renders
/// immediately.  Intended for panic paths where we need output NOW.
/// Callers should disable IRQs before calling if needed.
pub fn flush_sync() {
    let mut local = [0u8; FLUSH_BATCH];
    loop {
        let n = DEFERRED.drain(&mut local);
        if n == 0 {
            break;
        }
        if let Some(ref mut guard) = CONSOLE.try_lock() {
            let Some(ref mut console) = **guard else {
                break;
            };
            for &b in &local[..n] {
                console.put_char(b);
            }
        } else {
            break;
        }
    }
}

/// Non-blocking serial flush — drains one FIFO burst if the UART is ready.
///
/// Called from the timer tick.  Never spin-waits for TX-ready; if the UART
/// FIFO is busy it simply returns and tries again on the next tick (or the
/// serial IRQ handler will pick it up when THRE fires).
pub fn serial_flush_deferred() {
    // Ensure only one CPU flushes at a time to prevent reordering.
    let _flush_guard = match FlushGuard::try_enter(&SERIAL_FLUSH_ACTIVE) {
        Some(g) => g,
        None => return,
    };

    // If the UART TX register isn't ready, don't stall — bail out.
    if !crate::RUNTIME.arch.serial_tx_ready() {
        return;
    }

    let mut local = [0u8; 16]; // One FIFO burst
    if SERIAL_DEFERRED.is_empty() {
        // Nothing to send — disarm the TX interrupt to avoid spurious IRQs.
        disarm_serial_tx_if_idle();
        return;
    }
    let n = SERIAL_DEFERRED.drain(&mut local);

    if n > 0 {
        crate::RUNTIME.arch.write_serial_fifo_burst(&local[..n]);
    }
}

/// Drain one FIFO burst from the serial ring buffer, called from the
/// serial IRQ handler (vector 0x24) when a THRE interrupt fires.
///
/// Producers reserve ring slots atomically, so the IRQ handler never blocks
/// on the ring. If the ring is empty after draining, disarms the TX interrupt
/// to stop spurious IRQs until new data is pushed.
pub fn serial_drain_irq() {
    // Share the same drain gate as timer/idle flushes.  Without this, two CPUs
    // can dequeue different FIFO-sized chunks and write them to the UART in the
    // opposite order.
    let _flush_guard = match FlushGuard::try_enter(&SERIAL_FLUSH_ACTIVE) {
        Some(g) => g,
        None => return,
    };

    // Check TX-ready first (should be set since THRE fired, but be safe).
    if !crate::RUNTIME.arch.serial_tx_ready() {
        return;
    }

    let mut local = [0u8; 16]; // One FIFO burst
    let n = SERIAL_DEFERRED.drain(&mut local);

    if n > 0 {
        crate::RUNTIME.arch.write_serial_fifo_burst(&local[..n]);
    }

    if SERIAL_DEFERRED.is_empty() {
        disarm_serial_tx_if_idle();
    }
}

pub fn serial_flush_deferred_idle() {
    // Ensure only one CPU flushes at a time to prevent reordering.
    let _flush_guard = match FlushGuard::try_enter(&SERIAL_FLUSH_ACTIVE) {
        Some(g) => g,
        None => return,
    };

    // Drain multiple FIFO bursts — idle time is free, but still never
    // spin-wait.  If the UART isn't ready we bail immediately.
    let mut local = [0u8; 16];
    for _ in 0..128 {
        if !crate::RUNTIME.arch.serial_tx_ready() {
            return;
        }

        if SERIAL_DEFERRED.is_empty() {
            disarm_serial_tx_if_idle();
            return;
        }
        let n = SERIAL_DEFERRED.drain(&mut local);

        if n > 0 {
            crate::RUNTIME.arch.write_serial_fifo_burst(&local[..n]);
        } else {
            break;
        }
    }
}

pub fn serial_flush_sync() {
    let mut local = [0u8; FLUSH_BATCH];
    loop {
        let n = SERIAL_DEFERRED.drain(&mut local);
        if n == 0 {
            break;
        }
        crate::RUNTIME.serial_putbuf_sync(&local[..n]);
    }
}

pub unsafe fn force_unlock() {
    unsafe {
        CONSOLE.force_unlock();
    }
    SERIAL_FLUSH_ACTIVE.store(false, Ordering::Release);
    FB_FLUSH_ACTIVE.store(false, Ordering::Release);
}

#[cfg(test)]
mod tests {
    use super::{DEFERRED_CAP, DeferredRing};
    type TestRing = DeferredRing<DEFERRED_CAP>;

    fn drain_all(ring: &TestRing, out: &mut [u8]) -> usize {
        ring.drain(out)
    }

    #[test]
    fn push_slice_preserves_contiguous_order() {
        let ring = TestRing::new();
        ring.push_slice(b"line1\n");
        ring.push_slice(b"line2\n");
        let mut out = [0u8; 12];
        let n = drain_all(&ring, &mut out);
        assert_eq!(&out[..n], b"line1\nline2\n");
    }

    #[test]
    fn drain_wraps_correctly() {
        let ring = TestRing::new();
        let filler = [b'x'; DEFERRED_CAP - 4];
        ring.push_slice(&filler);
        let mut discard = [0u8; DEFERRED_CAP - 4];
        ring.drain(&mut discard);
        ring.push_slice(b"old new ");
        let mut out = [0u8; 8];
        let n = drain_all(&ring, &mut out);
        assert_eq!(&out[..n], b"old new ");
    }

    #[test]
    fn overwrite_discards_oldest_bytes() {
        let ring = TestRing::new();
        let filler = [b'y'; DEFERRED_CAP];
        ring.push_slice(&filler);
        ring.push_slice(b"abcdef");
        let mut out = [0u8; DEFERRED_CAP];
        let n = drain_all(&ring, &mut out);
        assert_eq!(n, DEFERRED_CAP);
        assert_eq!(&out[n - 6..n], b"abcdef");
    }

    #[test]
    fn empty_data_is_noop() {
        let ring = TestRing::new();
        ring.push_slice(b"hello");
        ring.push_slice(b"");
        let mut out = [0u8; 5];
        let n = drain_all(&ring, &mut out);
        assert_eq!(&out[..n], b"hello");
    }

    #[test]
    fn oversized_slice_keeps_tail() {
        let ring = TestRing::new();
        let oversized = [b'z'; DEFERRED_CAP + 8];
        ring.push_slice(&oversized);
        let mut out = [0u8; DEFERRED_CAP];
        let n = drain_all(&ring, &mut out);
        assert_eq!(n, DEFERRED_CAP);
        assert!(out[..n].iter().all(|&b| b == b'z'));
    }
}
