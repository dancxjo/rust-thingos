#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub const CELL_WIDTH: u32 = 8;
pub const CELL_HEIGHT: u32 = 16;
pub const DEFAULT_FG: u32 = 0xFFFF_FFFF;
pub const DEFAULT_BG: u32 = 0xFF00_0000;

#[derive(Clone)]
pub struct Glyph {
    pub width: u32,
    pub bitmap: Vec<u8>,
}

pub struct Font {
    glyphs: BTreeMap<u32, Glyph>,
}

impl Font {
    pub fn from_unifont_hex(content: &str) -> Self {
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
        Self { glyphs }
    }

    pub fn glyph_count(&self) -> usize {
        self.glyphs.len()
    }

    pub fn get_glyph(&self, c: char) -> Option<&Glyph> {
        self.glyphs.get(&(c as u32))
    }

    pub fn glyph_width(&self, c: char) -> u32 {
        self.get_glyph(c)
            .or_else(|| self.get_glyph('?'))
            .map(|g| g.width)
            .unwrap_or(CELL_WIDTH)
    }
}

// SAFETY: Font is immutable after construction and contains only owned maps,
// integers, and byte vectors.
unsafe impl Send for Font {}
unsafe impl Sync for Font {}

#[derive(Debug, Clone, PartialEq)]
pub enum AnsiState {
    Normal,
    Esc,
    Csi { params: Vec<u32>, current_num: Option<u32>, private: bool },
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub ch: char,
    pub fg: u32,
    pub bg: u32,
    pub wide: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Cell { ch: ' ', fg: DEFAULT_FG, bg: DEFAULT_BG, wide: false }
    }
}

pub struct TermModel {
    pub cols: u32,
    pub rows: u32,
    pub cells: Vec<Cell>,
    pub dirty_rows: Vec<bool>,
    pub cursor_col: u32,
    pub cursor_row: u32,
    pub cursor_visible: bool,
    pub current_fg: u32,
    pub current_bg: u32,
    ansi_state: AnsiState,
}

impl TermModel {
    pub fn new(cols: u32, rows: u32) -> Self {
        let cols = cols.max(1);
        let rows = rows.max(1);
        let n = (cols * rows) as usize;
        Self {
            cols,
            rows,
            cells: alloc::vec![Cell::default(); n],
            dirty_rows: alloc::vec![false; rows as usize],
            cursor_col: 0,
            cursor_row: 0,
            cursor_visible: true,
            current_fg: DEFAULT_FG,
            current_bg: DEFAULT_BG,
            ansi_state: AnsiState::Normal,
        }
    }

    pub fn resize(&mut self, cols: u32, rows: u32) {
        let cols = cols.max(1);
        let rows = rows.max(1);
        if cols == self.cols && rows == self.rows {
            return;
        }
        *self = Self::new(cols, rows);
    }

    pub fn mark_all_dirty(&mut self) {
        for dirty in &mut self.dirty_rows {
            *dirty = true;
        }
    }

    pub fn cell_idx(&self, col: u32, row: u32) -> usize {
        (row * self.cols + col) as usize
    }

    pub fn set_cell(&mut self, col: u32, row: u32, ch: char, fg: u32, bg: u32, wide: bool) {
        if col < self.cols && row < self.rows {
            let idx = self.cell_idx(col, row);
            self.cells[idx] = Cell { ch, fg, bg, wide };
            self.dirty_rows[row as usize] = true;
        }
    }

    pub fn clear_all(&mut self, bg: u32) {
        let fg = self.current_fg;
        for cell in &mut self.cells {
            *cell = Cell { ch: ' ', fg, bg, wide: false };
        }
        for dirty in &mut self.dirty_rows {
            *dirty = true;
        }
        self.cursor_col = 0;
        self.cursor_row = 0;
    }

    fn scroll(&mut self) {
        let cols = self.cols as usize;
        let rows = self.rows as usize;
        self.cells.copy_within(cols..cols * rows, 0);
        let bg = self.current_bg;
        let fg = self.current_fg;
        for col in 0..cols {
            self.cells[(rows - 1) * cols + col] = Cell { ch: ' ', fg, bg, wide: false };
        }
        for dirty in &mut self.dirty_rows {
            *dirty = true;
        }
        if self.cursor_row > 0 {
            self.cursor_row -= 1;
        }
    }

    fn ansi_color_to_u32(code: u32, is_bg: bool) -> u32 {
        let base = if is_bg { 40 } else { 30 };
        match code - base {
            0 => 0xFF00_0000,
            1 => 0xFFFF_0000,
            2 => 0xFFFF_CC00,
            3 => 0xFFFF_FF00,
            4 => 0xFF00_00FF,
            5 => 0xFFFF_00FF,
            6 => 0xFFAA_88FF,
            7 => 0xFFFF_FFFF,
            _ => {
                if is_bg {
                    DEFAULT_BG
                } else {
                    DEFAULT_FG
                }
            }
        }
    }

    pub fn putc(&mut self, c: char, font: &Font) {
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
                    current_num = Some(current_num.unwrap_or(0).saturating_mul(10) + digit);
                    self.ansi_state = AnsiState::Csi { params, current_num, private };
                    return;
                }
                if c == ';' {
                    params.push(current_num.unwrap_or(0));
                    self.ansi_state = AnsiState::Csi { params, current_num: None, private };
                    return;
                }
                if private && (c == 'h' || c == 'l') {
                    params.push(current_num.unwrap_or(0));
                    let show = c == 'h';
                    for &p in &params {
                        if p == 25 {
                            self.cursor_visible = show;
                        }
                    }
                    self.ansi_state = AnsiState::Normal;
                    return;
                }
                if c == 'm' {
                    params.push(current_num.unwrap_or(0));
                    if params.is_empty() {
                        params.push(0);
                    }
                    for &p in &params {
                        if p == 0 {
                            self.current_fg = DEFAULT_FG;
                            self.current_bg = DEFAULT_BG;
                        } else if (30..=37).contains(&p) {
                            self.current_fg = Self::ansi_color_to_u32(p, false);
                        } else if (40..=47).contains(&p) {
                            self.current_bg = Self::ansi_color_to_u32(p, true);
                        } else if (90..=97).contains(&p) {
                            self.current_fg = Self::ansi_color_to_u32(p - 60, false) | 0xFF88_8888;
                        }
                    }
                    self.ansi_state = AnsiState::Normal;
                    return;
                }
                if c == 'J' {
                    let bg = self.current_bg;
                    self.clear_all(bg);
                    self.ansi_state = AnsiState::Normal;
                    return;
                }
                if c == 'H' || c == 'f' {
                    params.push(current_num.unwrap_or(0));
                    let row = params.first().copied().unwrap_or(1).max(1) - 1;
                    let col = params.get(1).copied().unwrap_or(1).max(1) - 1;
                    self.cursor_row = row.min(self.rows - 1);
                    self.cursor_col = col.min(self.cols - 1);
                    self.ansi_state = AnsiState::Normal;
                    return;
                }
                if c == 'K' {
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
                }
                match c {
                    'C' => {
                        let n = current_num.unwrap_or(1).max(1);
                        self.cursor_col = (self.cursor_col + n).min(self.cols - 1);
                    }
                    'D' => {
                        let n = current_num.unwrap_or(1).max(1);
                        self.cursor_col = self.cursor_col.saturating_sub(n);
                    }
                    'A' => {
                        let n = current_num.unwrap_or(1).max(1);
                        self.cursor_row = self.cursor_row.saturating_sub(n);
                    }
                    'B' => {
                        let n = current_num.unwrap_or(1).max(1);
                        self.cursor_row = (self.cursor_row + n).min(self.rows - 1);
                    }
                    _ => {}
                }
                self.ansi_state = AnsiState::Normal;
                return;
            }
        }

        if c < ' ' {
            return;
        }

        let glyph_px_width = font.glyph_width(c);
        let col_units = (glyph_px_width / CELL_WIDTH).max(1);
        let wide = col_units >= 2;

        if self.cursor_col + col_units > self.cols {
            self.putc('\n', font);
        }

        let fg = self.current_fg;
        let bg = self.current_bg;
        self.set_cell(self.cursor_col, self.cursor_row, c, fg, bg, wide);
        if wide && self.cursor_col + 1 < self.cols {
            self.set_cell(self.cursor_col + 1, self.cursor_row, '\0', fg, bg, false);
        }
        self.cursor_col += col_units;
        if self.cursor_col >= self.cols {
            self.putc('\n', font);
        }
    }

    pub fn write_str(&mut self, s: &str, font: &Font) {
        for c in s.chars() {
            self.putc(c, font);
        }
    }

    pub fn write_bytes_lossy(&mut self, bytes: &[u8], font: &Font) {
        let text = String::from_utf8_lossy(bytes);
        self.write_str(&text, font);
    }
}

