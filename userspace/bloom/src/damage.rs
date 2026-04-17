use abi::display_protocol::Rect;
use alloc::vec::Vec;

pub struct DamageTracker {
    dirty: bool,
    regions: Vec<Rect>,
}

impl DamageTracker {
    pub fn new() -> Self {
        Self {
            dirty: true,
            regions: Vec::new(),
        }
    }

    pub fn mark_rect(&mut self, rect: Rect) {
        self.dirty = true;
        self.regions.push(rect);
    }

    pub fn mark_full(&mut self, width: u32, height: u32) {
        self.mark_rect(Rect {
            x: 0,
            y: 0,
            w: width,
            h: height,
        });
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn take(&mut self) -> Vec<Rect> {
        self.dirty = false;
        if self.regions.is_empty() {
            return Vec::new();
        }
        let mut out = Vec::new();
        core::mem::swap(&mut out, &mut self.regions);
        out
    }
}
