use spin::Mutex;
use crate::{FramebufferInfo, PixelFormat};

const ICON_SIDE: usize = 80;

/// Total boot tasks matching the 16 sequential `push()` calls in `kernel::start`.
const TOTAL_TASKS: usize = 16;
/// Grid columns; TOTAL_TASKS / GRID_COLS gives the number of rows.
const GRID_COLS: usize = 4;
/// Number of milestone dots displayed below the progress bar.
const NUM_DOTS: usize = 8;

include!("boot_progress_icons.rs");


#[derive(Clone, Copy)]
pub enum BootPhase {
    Framebuffer,
    Memory,
    Allocator,
    Devices,
    Compute,
    Vfs,
    Pci,
    Cpu,
    Modules,
    Init,
    Scheduler,
}

/// Phase for each of the 16 sequential `push()` calls (in boot order).
/// Used to pre-populate the grid with pending icons and to redraw cells on
/// state transitions without requiring the caller to pass the phase twice.
const TASK_PHASES: [BootPhase; TOTAL_TASKS] = [
    BootPhase::Framebuffer, // 00 – Framebuffer Initialized
    BootPhase::Memory,      // 01 – Memory Map OK
    BootPhase::Allocator,   // 02 – Global Allocator
    BootPhase::Devices,     // 03 – Display Registry
    BootPhase::Compute,     // 04 – SIMD Ready
    BootPhase::Compute,     // 05 – SIMD Ready (entropy)
    BootPhase::Cpu,         // 06 – Tasking Initialized
    BootPhase::Vfs,         // 07 – VFS Root Ready
    BootPhase::Pci,         // 08 – PCI Bus Scanned
    BootPhase::Devices,     // 09 – Legacy Devices
    BootPhase::Cpu,         // 10 – BSP Timer OK
    BootPhase::Cpu,         // 11 – SMP Bring-up
    BootPhase::Modules,     // 12 – Boot Info OK
    BootPhase::Modules,     // 13 – Modules Scanned
    BootPhase::Init,        // 14 – Spawning Sprout
    BootPhase::Scheduler,   // 15 – Entering Scheduler
];

/// Milestone dot[i] lights up (green) when `pushed_count >= DOT_THRESHOLDS[i]`.
const DOT_THRESHOLDS: [usize; NUM_DOTS] = [1, 3, 5, 7, 9, 11, 13, 16];

/// Short phase labels rendered below each milestone dot when a font is available.
const DOT_LABELS: [&str; NUM_DOTS] = [
    "FB", "MEM", "CPU", "BUS", "DEV", "CLK", "MOD", "INIT",
];

// ── Colour palette ────────────────────────────────────────────────────────────
const PANEL_BG: u32 = 0x111111;
const PENDING_FG: u32 = 0x2A2A2A;   // very dim – task not yet reached
const ACTIVE_FG: u32 = 0x00AADD;    // cyan  – task just pushed (in progress)
const COMPLETE_FG: u32 = 0x55AACC;  // light cyan – task finished
const CHECK_COLOR: u32 = 0x00CC55;  // green corner marker for complete cells
const TEXT_FG: u32 = 0xCCCCCC;      // milestone label text
const LABEL_FG: u32 = 0x666666;     // dot phase labels
const BAR_BG: u32 = 0x222222;       // unfilled bar
const BAR_FG: u32 = 0x00AA44;       // filled bar (green)
const DOT_PENDING_COLOR: u32 = 0x2D2D2D;
const DOT_COMPLETE_COLOR: u32 = 0x00AA44;
const DOT_ACTIVE_COLOR: u32 = 0x00AADD;

const BAR_H: usize = 6;
const DOT_SIZE: usize = 8;
const MSG_H: usize = 16;

// ── Layout ────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
struct Layout {
    panel_x: usize,
    panel_y: usize,
    panel_side: usize,
    icon_px: usize,
    gap_px: usize,
    /// Y position of the current-milestone text row.
    msg_y: usize,
    /// Y position of the thin progress bar.
    bar_y: usize,
    /// Y position of the phase milestone dots.
    dot_y: usize,
    /// Y position of the short dot-phase labels (drawn when font available).
    dot_label_y: usize,
}

// ── State ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
struct BootProgressState {
    fb: FramebufferInfo,
    layout: Layout,
    /// How many times `push()` has been called (= index of the next task to push).
    next_index: usize,
    unifont_data: Option<&'static [u8]>,
    /// Stores the phase that was passed to each push() call so cells can be
    /// correctly redrawn (e.g. Active → Complete) later.
    pushed_phases: [BootPhase; TOTAL_TASKS],
}

static BOOT_PROGRESS: Mutex<Option<BootProgressState>> = Mutex::new(None);

// ── Public API ────────────────────────────────────────────────────────────────

pub fn init(fb: FramebufferInfo) {
    if fb.width == 0 || fb.height == 0 || fb.pitch < 4 {
        return;
    }

    let w = fb.width as usize;
    let h = fb.height as usize;
    let min_dim = w.min(h);

    // Panel occupies ~half of the narrower screen dimension.
    let panel_side_raw = min_dim / 2;
    // icon_px = panel_side / 5, then gap fills the rest across 5 gap slots.
    let icon_px = (panel_side_raw / 5).clamp(32, 80);
    let gap_px = ((panel_side_raw).saturating_sub(4 * icon_px)) / 5;
    let gap_px = gap_px.max(2);
    // Recompute exact panel_side from the clamped values.
    let panel_side = 4 * icon_px + 5 * gap_px;

    // Vertical space needed below the icon grid.
    let below_h = MSG_H + 4 + BAR_H + 4 + DOT_SIZE + 3 + 10;
    let total_h = panel_side + below_h + 10;

    let panel_x = w.saturating_sub(panel_side) / 2;
    let panel_y = h.saturating_sub(total_h) / 2;

    let msg_y = panel_y + panel_side + 6;
    let bar_y = msg_y + MSG_H + 4;
    let dot_y = bar_y + BAR_H + 4;
    let dot_label_y = dot_y + DOT_SIZE + 3;

    let layout = Layout { panel_x, panel_y, panel_side, icon_px, gap_px, msg_y, bar_y, dot_y, dot_label_y };

    let mut state = BootProgressState {
        fb,
        layout,
        next_index: 0,
        unifont_data: None,
        pushed_phases: [BootPhase::Framebuffer; TOTAL_TASKS],
    };
    state.draw_initial_panel();
    *BOOT_PROGRESS.lock() = Some(state);
}

pub fn set_unifont_data(data: &'static [u8]) {
    let mut guard = BOOT_PROGRESS.lock();
    if let Some(state) = guard.as_mut() {
        state.unifont_data = Some(data);
    }
}

/// Record that a boot task has completed.
///
/// - Transitions the **previous** active cell to the *complete* visual state.
/// - Draws the **current** cell in the *active* state.
/// - Advances the progress bar and milestone dots.
/// - Emits a kernel INFO log line so the milestone is visible on the serial
///   console and can be captured by BDD tests.
pub fn push(phase: BootPhase, message: &str) {
    // ── Milestone text → serial / kernel log ────────────────────────────────
    // Safe to call even before logging::init() — the logger silently no-ops
    // when the global logger has not yet been installed.
    crate::kinfo!("boot_progress: milestone=\"{}\"", message);

    // ── Visual update (lock held) ────────────────────────────────────────────
    {
        let mut guard = BOOT_PROGRESS.lock();
        let Some(state) = guard.as_mut() else { return };

        let idx = state.next_index;

        // Store the caller's phase for use when this cell is later redrawn as Complete.
        if idx < TOTAL_TASKS {
            state.pushed_phases[idx] = phase;
        }

        // Transition the previous active cell to Complete.
        if idx > 0 && idx - 1 < TOTAL_TASKS {
            let prev_phase = state.pushed_phases[idx - 1];
            state.redraw_cell(idx - 1, prev_phase, false, true);
        }

        // Draw the current cell as Active.
        if idx < TOTAL_TASKS {
            state.redraw_cell(idx, phase, true, false);
        }

        // Progress bar and dots count pushed tasks (the active task counts as
        // in-progress, so the fill reflects idx completed + 1 active).
        let pushed = (idx + 1).min(TOTAL_TASKS);
        state.draw_progress_bar(pushed);
        state.draw_phase_dots(pushed);
        state.update_message(message);

        state.next_index += 1;
        // Lock dropped here.
    }
}

/// Mark the last active cell as complete and fill the progress bar to 100 %.
/// Call this once after the final `push()` (i.e. just before entering the
/// scheduler loop) so the display shows a fully completed grid.
pub fn finish() {
    let mut guard = BOOT_PROGRESS.lock();
    let Some(state) = guard.as_mut() else { return };

    let last = state.next_index.saturating_sub(1);
    if last < TOTAL_TASKS {
        let phase = state.pushed_phases[last];
        state.redraw_cell(last, phase, false, true);
    }
    state.draw_progress_bar(TOTAL_TASKS);
    state.draw_phase_dots(TOTAL_TASKS);
    state.update_message("BOOT COMPLETE");
}

// ── Helper: icon bitmap lookup ────────────────────────────────────────────────

fn icon_bits_for_phase(phase: BootPhase) -> &'static [[u128; 2]; 80] {
    match phase {
        BootPhase::Framebuffer => &ICON_FRAMEBUFFER,
        BootPhase::Memory => &ICON_MEMORY,
        BootPhase::Allocator => &ICON_ALLOCATOR,
        BootPhase::Devices => &ICON_DEVICES,
        BootPhase::Compute => &ICON_COMPUTE,
        BootPhase::Vfs => &ICON_VFS,
        BootPhase::Pci => &ICON_PCI,
        BootPhase::Cpu => &ICON_CPU,
        BootPhase::Modules => &ICON_MODULES,
        BootPhase::Init => &ICON_INIT,
        BootPhase::Scheduler => &ICON_SCHEDULER,
    }
}

// ── BootProgressState impls ───────────────────────────────────────────────────

impl BootProgressState {
    /// Draw the full initial panel: background + all 16 cells as Pending + bar + dots.
    fn draw_initial_panel(&mut self) {
        self.fill_rect(self.layout.panel_x, self.layout.panel_y, self.layout.panel_side, self.layout.panel_side, PANEL_BG);
        for i in 0..TOTAL_TASKS {
            self.redraw_cell(i, TASK_PHASES[i], false, false);
        }
        self.draw_progress_bar(0);
        self.draw_phase_dots(0);
        self.update_message("BOOTING...");
    }

    /// Pixel coordinate of the top-left corner of cell `idx`.
    fn cell_xy(&self, idx: usize) -> (usize, usize) {
        let col = idx % GRID_COLS;
        let row = idx / GRID_COLS;
        let stride = self.layout.icon_px + self.layout.gap_px;
        let x = self.layout.panel_x + self.layout.gap_px + col * stride;
        let y = self.layout.panel_y + self.layout.gap_px + row * stride;
        (x, y)
    }

    /// Redraw a single grid cell in the requested state.
    ///
    /// * `active`   – cyan; currently executing
    /// * `complete` – light cyan with a small green corner marker; done
    /// * neither    – pending; very dim gray
    fn redraw_cell(&mut self, idx: usize, phase: BootPhase, active: bool, complete: bool) {
        if idx >= TOTAL_TASKS { return; }
        let (x, y) = self.cell_xy(idx);
        let size = self.layout.icon_px;

        // Erase cell background.
        self.fill_rect(x, y, size, size, PANEL_BG);

        // Old-timey bezel: Sunken look (dark top/left, light bottom/right)
        let shadow = 0x080808;
        let highlight = 0x222222;
        for i in 0..size {
            self.put_pixel(x + i, y, shadow);
            self.put_pixel(x, y + i, shadow);
            self.put_pixel(x + i, y + size - 1, highlight);
            self.put_pixel(x + size - 1, y + i, highlight);
        }

        let color = if active {
            ACTIVE_FG
        } else if complete {
            COMPLETE_FG
        } else {
            PENDING_FG
        };
        
        // Draw icon slightly inset within the bezel
        self.blit_bit_icon_colored(x + 2, y + 2, size.saturating_sub(4), icon_bits_for_phase(phase), color);

        // Small green square in the top-right corner marks completion.
        if complete {
            let marker = (size / 6).clamp(3, 8);
            self.fill_rect(x + size - marker - 2, y + 2, marker, marker, CHECK_COLOR);
        }
    }

    /// Draw (or redraw) the thin horizontal progress bar.
    ///
    /// `pushed_count` is the number of tasks that have been pushed so far
    /// (including the currently active one).
    fn draw_progress_bar(&mut self, pushed_count: usize) {
        let x = self.layout.panel_x;
        let y = self.layout.bar_y;
        let total_w = self.layout.panel_side;

        let filled = if total_w > 0 {
            (pushed_count * total_w) / TOTAL_TASKS
        } else {
            0
        };

        self.fill_rect(x, y, total_w, BAR_H, BAR_BG);
        if filled > 0 {
            self.fill_rect(x, y, filled, BAR_H, BAR_FG);
        }
    }

    /// Draw (or redraw) the row of 8 milestone dots below the bar.
    ///
    /// A dot is green when its threshold has been met, cyan when the previous
    /// threshold has been met but this one hasn't yet, otherwise dark gray.
    fn draw_phase_dots(&mut self, pushed_count: usize) {
        let spacing = self.layout.panel_side / NUM_DOTS;

        for i in 0..NUM_DOTS {
            let dot_x = self.layout.panel_x + i * spacing + spacing / 2 - DOT_SIZE / 2;
            let dot_y = self.layout.dot_y;

            let color = if pushed_count >= DOT_THRESHOLDS[i] {
                DOT_COMPLETE_COLOR
            } else if i == 0 || pushed_count >= DOT_THRESHOLDS[i - 1] {
                DOT_ACTIVE_COLOR
            } else {
                DOT_PENDING_COLOR
            };

            self.fill_rect(dot_x, dot_y, DOT_SIZE, DOT_SIZE, color);

            // Short phase label below the dot (only when unifont is loaded).
            if self.unifont_data.is_some() {
                let label = DOT_LABELS[i];
                let lx = self.layout.panel_x + i * spacing;
                self.draw_string(lx, self.layout.dot_label_y, label, LABEL_FG);
            }
        }
    }

    /// Replace the milestone text row with `msg`, centred within the panel.
    fn update_message(&mut self, msg: &str) {
        // Clear 18 pixels to ensure no artifacts remain from 16-pixel Unifont.
        self.fill_rect(self.layout.panel_x, self.layout.msg_y, self.layout.panel_side, 18, 0);
        if self.unifont_data.is_some() {
            let char_w = 8;
            let text_w = msg.len() * char_w;
            let text_x = if text_w < self.layout.panel_side {
                self.layout.panel_x + (self.layout.panel_side - text_w) / 2
            } else {
                self.layout.panel_x
            };
            self.draw_string(text_x, self.layout.msg_y, msg, TEXT_FG);
        }
    }

    fn blit_bit_icon_colored(&mut self, x: usize, y: usize, size: usize, bits: &[[u128; 2]; ICON_SIDE], color: u32) {
        for dy in 0..size {
            let row = (dy * ICON_SIDE) / size;
            let row_hi = bits[row][0];
            let row_lo = bits[row][1];
            for dx in 0..size {
                let col = (dx * ICON_SIDE) / size;
                let hi = (row_hi >> (127 - col)) & 1;
                let lo = (row_lo >> (127 - col)) & 1;
                let alpha_idx = (hi << 1) | lo;
                if alpha_idx == 0 {
                    continue;
                }

                let mut pixel_color = if alpha_idx == 3 {
                    color
                } else {
                    self.blend(PANEL_BG, color, alpha_idx as u32)
                };

                // Old-timey scanline effect: dim every second screen row
                if (y + dy) % 2 == 1 {
                    pixel_color = self.dim(pixel_color, 210); // slightly less aggressive dimming
                }
                
                self.put_pixel(x + dx, y + dy, pixel_color);
            }
        }
    }

    /// Linear blend between bg and fg based on 2-bit alpha index (1=33%, 2=66%).
    fn blend(&self, bg: u32, fg: u32, alpha_idx: u32) -> u32 {
        let (f_a, b_a) = match alpha_idx {
            1 => (85, 170),
            2 => (170, 85),
            _ => (255, 0),
        };

        let rb = (((fg & 0x00FF00FF) * f_a + (bg & 0x00FF00FF) * b_a) >> 8) & 0x00FF00FF;
        let g = (((fg & 0x0000FF00) * f_a + (bg & 0x0000FF00) * b_a) >> 8) & 0x0000FF00;
        rb | g
    }

    /// Reduce brightness of a color by a factor (0-255).
    fn dim(&self, color: u32, factor: u32) -> u32 {
        let rb = (((color & 0x00FF00FF) * factor) >> 8) & 0x00FF00FF;
        let g = (((color & 0x0000FF00) * factor) >> 8) & 0x0000FF00;
        rb | g
    }

    fn draw_char(&mut self, x: usize, y: usize, c: char, color: u32) {
        let Some(data) = self.unifont_data else { return };
        if let Some(glyph) = lookup_unifont_glyph(data, c) {
            let width = if glyph.len() == 16 { 8 } else { 16 };
            for row in 0..16 {
                let row_data = glyph[row];
                for col in 0..width {
                    if (row_data >> (width - 1 - col)) & 1 != 0 {
                        self.put_pixel(x + col, y + row, color);
                    }
                }
            }
        }
    }

    fn draw_string(&mut self, x: usize, y: usize, s: &str, color: u32) {
        let mut cur_x = x;
        for c in s.chars() {
            self.draw_char(cur_x, y, c, color);
            cur_x += 8;
        }
    }

    fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.fb.width as usize || y >= self.fb.height as usize {
            return;
        }
        let addr = self.fb.addr + (y * self.fb.pitch as usize + x * 4) as u64;
        let final_color = match self.fb.format {
            PixelFormat::Bgra8888 | PixelFormat::Bgrx8888 => color,
            _ => color,
        };
        unsafe {
            *(addr as *mut u32) = final_color;
        }
    }

    fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for dy in 0..h {
            for dx in 0..w {
                self.put_pixel(x + dx, y + dy, color);
            }
        }
    }
}

fn lookup_unifont_glyph(data: &[u8], c: char) -> Option<[u16; 32]> {
    let target_code = c as u32;
    let mut offset = 0;
    while offset < data.len() {
        // Find next line
        let mut end = offset;
        while end < data.len() && data[end] != b'\n' {
            end += 1;
        }
        let line = &data[offset..end];
        offset = end + 1;

        if line.is_empty() { continue; }

        // Parse CODE:BITMAP
        let mut parts = line.split(|&b| b == b':');
        let code_hex = parts.next()?;
        let bitmap_hex = parts.next()?;

        let code = parse_hex(code_hex)?;
        if code == target_code {
            let mut glyph = [0u16; 32];
            let is_wide = bitmap_hex.len() == 64;
            let rows = 16;
            let step = if is_wide { 4 } else { 2 };

            for i in 0..rows {
                let start = i * step;
                glyph[i] = parse_hex(&bitmap_hex[start..start + step])? as u16;
            }
            return Some(glyph);
        }

        if code > target_code { break; }
    }
    None
}

fn parse_hex(hex: &[u8]) -> Option<u32> {
    let mut res = 0u32;
    for &b in hex {
        res <<= 4;
        res |= match b {
            b'0'..=b'9' => (b - b'0') as u32,
            b'a'..=b'f' => (b - b'a' + 10) as u32,
            b'A'..=b'F' => (b - b'A' + 10) as u32,
            _ => return None,
        };
    }
    Some(res)
}
