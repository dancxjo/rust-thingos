use spin::Mutex;

use crate::{FramebufferInfo, PixelFormat};

const GLYPH_SIDE: usize = 8;
const PANEL_BG: u32 = 0x00_10_10_10;
const PANEL_BORDER: u32 = 0x00_D0_D0_D0;
const GLYPH_COLOR: u32 = 0x00_F2_F2_F2;

static BOOT_PROGRESS: Mutex<Option<BootProgressState>> = Mutex::new(None);

const ICON_FRAMEBUFFER: &[u8] =
    include_bytes!("../assets/boot_progress/framebuffer.txt");
const ICON_MEMORY: &[u8] = include_bytes!("../assets/boot_progress/memory.txt");
const ICON_ALLOCATOR: &[u8] = include_bytes!("../assets/boot_progress/allocator.txt");
const ICON_DEVICES: &[u8] = include_bytes!("../assets/boot_progress/devices.txt");
const ICON_COMPUTE: &[u8] = include_bytes!("../assets/boot_progress/compute.txt");
const ICON_VFS: &[u8] = include_bytes!("../assets/boot_progress/vfs.txt");
const ICON_PCI: &[u8] = include_bytes!("../assets/boot_progress/pci.txt");
const ICON_CPU: &[u8] = include_bytes!("../assets/boot_progress/cpu.txt");
const ICON_MODULES: &[u8] = include_bytes!("../assets/boot_progress/modules.txt");
const ICON_INIT: &[u8] = include_bytes!("../assets/boot_progress/init.txt");
const ICON_SCHEDULER: &[u8] = include_bytes!("../assets/boot_progress/scheduler.txt");

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

#[derive(Clone, Copy)]
struct Layout {
    panel_x: usize,
    panel_y: usize,
    panel_side: usize,
    margin: usize,
    cell_px: usize,
    gap_px: usize,
    rows: usize,
    cols: usize,
}

#[derive(Clone, Copy)]
struct BootProgressState {
    fb: FramebufferInfo,
    layout: Layout,
    next_index: usize,
}

pub fn init(fb: FramebufferInfo) {
    if fb.width == 0 || fb.height == 0 || fb.pitch < 4 || !is_supported_format(fb) {
        return;
    }

    let Some(layout) = Layout::new(fb.width as usize, fb.height as usize) else {
        return;
    };

    let mut state = BootProgressState { fb, layout, next_index: 0 };
    state.draw_panel();
    *BOOT_PROGRESS.lock() = Some(state);
}

pub fn push(phase: BootPhase) {
    let mut guard = BOOT_PROGRESS.lock();
    let Some(state) = guard.as_mut() else {
        return;
    };

    let capacity = state.layout.rows.saturating_mul(state.layout.cols);
    if state.next_index >= capacity {
        return;
    }

    let (col, row) = grid_position(state.next_index, state.layout.rows);
    state.draw_icon(col, row, icon_for_phase(phase));
    state.next_index = state.next_index.saturating_add(1);
}

fn icon_for_phase(phase: BootPhase) -> &'static [u8] {
    match phase {
        BootPhase::Framebuffer => ICON_FRAMEBUFFER,
        BootPhase::Memory => ICON_MEMORY,
        BootPhase::Allocator => ICON_ALLOCATOR,
        BootPhase::Devices => ICON_DEVICES,
        BootPhase::Compute => ICON_COMPUTE,
        BootPhase::Vfs => ICON_VFS,
        BootPhase::Pci => ICON_PCI,
        BootPhase::Cpu => ICON_CPU,
        BootPhase::Modules => ICON_MODULES,
        BootPhase::Init => ICON_INIT,
        BootPhase::Scheduler => ICON_SCHEDULER,
    }
}

impl Layout {
    fn new(width: usize, height: usize) -> Option<Self> {
        let min_dim = width.min(height);
        if min_dim < 48 {
            return None;
        }

        let scale = if min_dim >= 900 {
            4
        } else if min_dim >= 600 {
            3
        } else {
            2
        };
        let margin = scale * 2;
        let gap_px = scale;
        let cell_px = GLYPH_SIDE * scale;
        let target_side = (min_dim / 3).clamp(64, 192);

        let usable = target_side.saturating_sub(margin * 2);
        let rows = ((usable + gap_px) / (cell_px + gap_px)).max(1);
        let cols = rows;
        let panel_side = (margin * 2)
            .saturating_add(rows * cell_px)
            .saturating_add(rows.saturating_sub(1) * gap_px);

        Some(Self {
            panel_x: width.saturating_sub(panel_side) / 2,
            panel_y: height.saturating_sub(panel_side) / 2,
            panel_side,
            margin,
            cell_px,
            gap_px,
            rows,
            cols,
        })
    }
}

impl BootProgressState {
    fn draw_panel(&mut self) {
        self.fill_rect(
            self.layout.panel_x,
            self.layout.panel_y,
            self.layout.panel_side,
            self.layout.panel_side,
            PANEL_BG,
        );
        self.stroke_rect(
            self.layout.panel_x,
            self.layout.panel_y,
            self.layout.panel_side,
            self.layout.panel_side,
            PANEL_BORDER,
        );
    }

    fn draw_icon(&mut self, col: usize, row: usize, icon: &[u8]) {
        let x = self.layout.panel_x
            + self.layout.margin
            + col.saturating_mul(self.layout.cell_px + self.layout.gap_px);
        let y = self.layout.panel_y
            + self.layout.margin
            + row.saturating_mul(self.layout.cell_px + self.layout.gap_px);
        let scale = self.layout.cell_px / GLYPH_SIDE;

        for gy in 0..GLYPH_SIDE {
            for gx in 0..GLYPH_SIDE {
                let idx = gy * (GLYPH_SIDE + 1) + gx;
                if icon.get(idx).copied() != Some(b'#') {
                    continue;
                }
                self.fill_rect(x + gx * scale, y + gy * scale, scale, scale, GLYPH_COLOR);
            }
        }
    }

    fn stroke_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        if w == 0 || h == 0 {
            return;
        }
        self.fill_rect(x, y, w, 1, color);
        self.fill_rect(x, y + h.saturating_sub(1), w, 1, color);
        self.fill_rect(x, y, 1, h, color);
        self.fill_rect(x + w.saturating_sub(1), y, 1, h, color);
    }

    fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        if w == 0 || h == 0 {
            return;
        }

        let pitch_px = (self.fb.pitch as usize) / 4;
        let width = self.fb.width as usize;
        let height = self.fb.height as usize;
        let x_end = x.saturating_add(w).min(width);
        let y_end = y.saturating_add(h).min(height);
        if x >= x_end || y >= y_end || pitch_px == 0 {
            return;
        }

        let ptr = self.fb.addr as *mut u32;
        if ptr.is_null() {
            return;
        }

        unsafe {
            for py in y..y_end {
                let row = core::slice::from_raw_parts_mut(ptr.add(py * pitch_px), pitch_px);
                for px in x..x_end {
                    row[px] = color;
                }
            }
        }
    }
}

fn is_supported_format(fb: FramebufferInfo) -> bool {
    matches!(fb.format, PixelFormat::Bgra8888 | PixelFormat::Bgrx8888) && fb.bpp >= 24
}

fn grid_position(index: usize, rows: usize) -> (usize, usize) {
    if rows == 0 {
        return (0, 0);
    }
    (index / rows, index % rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_position_fills_top_to_bottom_then_left_to_right() {
        assert_eq!(grid_position(0, 3), (0, 0));
        assert_eq!(grid_position(1, 3), (0, 1));
        assert_eq!(grid_position(2, 3), (0, 2));
        assert_eq!(grid_position(3, 3), (1, 0));
        assert_eq!(grid_position(4, 3), (1, 1));
    }

    #[test]
    fn layout_centers_square_panel() {
        let layout = Layout::new(1024, 768).expect("layout");
        assert_eq!(layout.panel_x + layout.panel_side / 2, 512);
        assert_eq!(layout.panel_y + layout.panel_side / 2, 384);
        assert!(layout.rows >= 1);
        assert_eq!(layout.rows, layout.cols);
    }
}
