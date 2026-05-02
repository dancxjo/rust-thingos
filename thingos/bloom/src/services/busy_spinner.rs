//! `BusySpinnerService` — animates Bloom's built-in fallback cursor.

use core::time::Duration;

use crate::loop_types::{BloomService, Interest, LoopAction, LoopEvent};
use crate::world::BloomWorld;

const BUSY_SPINNER_TIMER: u64 = 1;
const BUSY_SPINNER_FRAME_MS: u64 = 80;
const BUSY_SPINNER_FRAMES: u32 = 12;

pub struct BusySpinnerService {
    frame: u32,
    running: bool,
}

impl BusySpinnerService {
    pub fn new() -> Self {
        Self { frame: 0, running: true }
    }

    fn arm_frame_timer() -> LoopAction {
        LoopAction::ArmTimer {
            delay: Duration::from_millis(BUSY_SPINNER_FRAME_MS),
            id: BUSY_SPINNER_TIMER,
        }
    }
}

impl BloomService for BusySpinnerService {
    fn name(&self) -> &'static str {
        "busy-spinner"
    }

    fn on_added(&mut self) -> LoopAction {
        stem::info!("bloom: busy spinner serviceLoop started");
        Self::arm_frame_timer()
    }

    fn interests(&self) -> &[Interest] {
        &[]
    }

    fn dispatch(&mut self, event: LoopEvent, world: &mut BloomWorld) -> LoopAction {
        if !self.running || !matches!(event, LoopEvent::Timer(BUSY_SPINNER_TIMER)) {
            return LoopAction::None;
        }

        self.frame = (self.frame + 1) % BUSY_SPINNER_FRAMES;
        if world.tick_busy_spinner(self.frame) {
            LoopAction::RequestRepaintAndArmTimer {
                delay: Duration::from_millis(BUSY_SPINNER_FRAME_MS),
                id: BUSY_SPINNER_TIMER,
            }
        } else {
            self.running = false;
            LoopAction::None
        }
    }
}
