use spin::Mutex;
use crate::{FramebufferInfo, PixelFormat};

const ICON_SIDE: usize = 32;

const ICON_FRAMEBUFFER: [u32; 32] = [
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x07ffffe0, 0x0fffffe0,
    0x0c000020, 0x0c000020, 0x0c000020, 0x0c000020, 0x0c000020, 0x0c000020, 0x0c000020, 0x0c000020,
    0x0c000020, 0x0c000020, 0x0c000020, 0x0c000020, 0x0ffe7fe0, 0x07ffffe0, 0x0003c000, 0x0003c000,
    0x001ff000, 0x001ff800, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
];
const ICON_MEMORY: [u32; 32] = [
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x0ffffff0, 0x0c23c430, 0x0c23c430, 0x0c23c420, 0x0c23c430,
    0x0c23c430, 0x0c23c430, 0x0ffffff0, 0x08000010, 0x098ee670, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
];
const ICON_ALLOCATOR: [u32; 32] = [
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x000e7000, 0x00000000, 0x001e7000, 0x001e7000, 0x001e7000, 0x01de7380, 0x00000000, 0x03dff3c0,
    0x03dff3c0, 0x03dff3c0, 0x03dff3c0, 0x00000000, 0x03fe7fc0, 0x03fe7fc0, 0x03fe7fc0, 0x03fe7fc0,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
];
const ICON_DEVICES: [u32; 32] = [
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000400, 0x00000c00, 0x00000c00,
    0x00001700, 0x00003300, 0x0001bdc0, 0x0003ed00, 0x0007ef00, 0x000ff000, 0x001ff800, 0x003ffc00,
    0x003ffc00, 0x003ff000, 0x003ff000, 0x003fe000, 0x003f8000, 0x00bb8000, 0x01c00000, 0x03c00000,
    0x01800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
];
const ICON_COMPUTE: [u32; 32] = [
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x0003f000, 0x0007e000, 0x0007e000,
    0x0007e000, 0x0007e000, 0x000fc000, 0x001f8000, 0x001f8000, 0x001f8000, 0x001ffc00, 0x003ff800,
    0x003ff800, 0x003ff000, 0x0001e000, 0x0001e000, 0x00038000, 0x00038000, 0x00070000, 0x00060000,
    0x00040000, 0x00080000, 0x00080000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
];
const ICON_VFS: [u32; 32] = [
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00060000, 0x0007e000, 0x0007e000,
    0x0007e000, 0x0007e000, 0x00010000, 0x00010000, 0x00010000, 0x00010100, 0x00010100, 0x00010e00,
    0x00010fc0, 0x00010fc0, 0x00010fc0, 0x00010000, 0x0001ff00, 0x00010000, 0x00010000, 0x06070300,
    0x07c7e3e0, 0x07c7e3e0, 0x07c7e3e0, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
];
const ICON_PCI: [u32; 32] = [
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x0c000000, 0x00000000, 0x00ffffe0, 0x00000000, 0x00000000, 0x00ffffe0, 0x00ffffe0, 0x00000000,
    0x00000000, 0x00ffffe0, 0x00ffffe0, 0x00000000, 0x00ffffe0, 0x000d35c0, 0x000d35c0, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
];
const ICON_CPU: [u32; 32] = [
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x002a5400, 0x002a5400, 0x00000000, 0x00000000,
    0x00ffff00, 0x00c00300, 0x0c9ff920, 0x00affd00, 0x0cbffd20, 0x00bffd00, 0x0cbffd20, 0x00bffd00,
    0x00bffd00, 0x0cbffd20, 0x00bffd00, 0x0cbffd20, 0x00bffd00, 0x0c9ff920, 0x00c00300, 0x00ffff00,
    0x00000000, 0x00000000, 0x002a5400, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
];
const ICON_MODULES: [u32; 32] = [
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00018000, 0x0007e000, 0x0007f000,
    0x0021fc00, 0x00f0ff00, 0x01fe3f80, 0x027f8c40, 0x033fc0c0, 0x03e7e3c0, 0x03e3c3c0, 0x03fc33c0,
    0x03fe73c0, 0x03fe7fc0, 0x03fe7fc0, 0x03fe7fc0, 0x03fe7fc0, 0x03fe7fc0, 0x00fe7f00, 0x003e7c00,
    0x000e7000, 0x00066000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
];
const ICON_INIT: [u32; 32] = [
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x000000e0, 0x00000fe0, 0x00001fc0,
    0x00007fc0, 0x0000ffc0, 0x0001e7c0, 0x0007c380, 0x0007c380, 0x0efe7000, 0x01ffff00, 0x03dffe00,
    0x03dffc00, 0x003ff800, 0x003ff000, 0x003fe000, 0x009fa000, 0x010e6000, 0x0301e000, 0x0321e000,
    0x03e1c000, 0x03c18000, 0x04000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
];
const ICON_SCHEDULER: [u32; 32] = [
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x0007e000, 0x001ff800, 0x00781e00, 0x00e18f00,
    0x01d18380, 0x039811c0, 0x030180c0, 0x06c18360, 0x06418260, 0x0c018020, 0x0c018020, 0x0d8181a0,
    0x0d8181a0, 0x0c004020, 0x0c003020, 0x06400260, 0x070000e0, 0x030000c0, 0x039811c0, 0x01d18380,
    0x00e18f00, 0x00781e00, 0x001ff800, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
];

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

const PANEL_BG: u32 = 0x111111;
const ICON_FG: u32 = 0x00AADD; // Cyan
const TEXT_FG: u32 = 0xFFFFFF; // White

#[derive(Clone, Copy)]
struct Layout {
    panel_x: usize,
    panel_y: usize,
    panel_side: usize,
    icon_px: usize,
    gap_px: usize,
    rows: usize,
    cols: usize,
}

#[derive(Clone, Copy)]
struct BootProgressState {
    fb: FramebufferInfo,
    layout: Layout,
    next_index: usize,
    unifont_data: Option<&'static [u8]>,
}

static BOOT_PROGRESS: Mutex<Option<BootProgressState>> = Mutex::new(None);

pub fn init(fb: FramebufferInfo) {
    if fb.width == 0 || fb.height == 0 || fb.pitch < 4 {
        return;
    }

    let min_dim = fb.width.min(fb.height) as usize;
    let target_side = (min_dim / 3).clamp(128, 320);
    let icon_px = (target_side / 5).clamp(16, 48);
    let gap_px = icon_px / 2;
    let rows = 3;
    let cols = 4;
    
    let layout = Layout {
        panel_x: (fb.width as usize - target_side) / 2,
        panel_y: (fb.height as usize - target_side) / 2 - 20,
        panel_side: target_side,
        icon_px,
        gap_px,
        rows,
        cols,
    };

    let mut state = BootProgressState { fb, layout, next_index: 0, unifont_data: None };
    state.draw_panel();
    *BOOT_PROGRESS.lock() = Some(state);
}

pub fn set_unifont_data(data: &'static [u8]) {
    let mut guard = BOOT_PROGRESS.lock();
    if let Some(state) = guard.as_mut() {
        state.unifont_data = Some(data);
    }
}

pub fn push(phase: BootPhase, message: &str) {
    let mut guard = BOOT_PROGRESS.lock();
    let Some(state) = guard.as_mut() else {
        return;
    };

    let (col, row) = (state.next_index % state.layout.cols, state.next_index / state.layout.cols);
    if row < state.layout.rows {
        let x = state.layout.panel_x + state.layout.gap_px + col * (state.layout.icon_px + state.layout.gap_px);
        let y = state.layout.panel_y + state.layout.gap_px + row * (state.layout.icon_px + state.layout.gap_px);
        
        state.blit_bit_icon(x, y, state.layout.icon_px, icon_bits_for_phase(phase));
        state.next_index += 1;
    }

    // Clear previous message area (approximate)
    state.fill_rect(
        state.layout.panel_x,
        state.layout.panel_y + state.layout.panel_side + 5,
        state.layout.panel_side,
        20,
        0, // Black background
    );

    // Draw new message centered below panel
    let text_x = state.layout.panel_x + (state.layout.panel_side.saturating_sub(message.len() * 8)) / 2;
    let text_y = state.layout.panel_y + state.layout.panel_side + 5;
    state.draw_string(text_x, text_y, message, TEXT_FG);
}

fn icon_bits_for_phase(phase: BootPhase) -> &'static [u32; 32] {
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

impl BootProgressState {
    fn draw_panel(&mut self) {
        self.fill_rect(
            self.layout.panel_x,
            self.layout.panel_y,
            self.layout.panel_side,
            self.layout.panel_side,
            PANEL_BG,
        );
    }

    fn blit_bit_icon(&mut self, x: usize, y: usize, size: usize, bits: &[u32; 32]) {
        let scale = size / 32;
        for row in 0..32 {
            let row_bits = bits[row];
            for col in 0..32 {
                if (row_bits >> (31 - col)) & 1 != 0 {
                    self.fill_rect(x + col * scale, y + row * scale, scale, scale, ICON_FG);
                }
            }
        }
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
                glyph[i] = parse_hex(&bitmap_hex[start..start+step])? as u16;
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
