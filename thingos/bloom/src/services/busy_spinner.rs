//! `BusySpinnerService` — animates Bloom's built-in fallback cursor.

use core::time::Duration;

use crate::loop_types::{BloomService, Interest, LoopAction, LoopEvent};
use crate::world::BloomWorld;

const BUSY_SPINNER_TIMER: u64 = 1;
const BUSY_SPINNER_FRAME_MS: u64 = 80;
const BUSY_SPINNER_FRAMES: u32 = 12;
const BUSY_SPINNER_FADE_FRAMES: u32 = 6;

enum BusySpinnerPhase {
    Running,
    Fading { frame: u32 },
}

pub struct BusySpinnerService {
    frame: u32,
    phase: BusySpinnerPhase,
}

impl BusySpinnerService {
    pub fn new() -> Self {
        Self { frame: 0, phase: BusySpinnerPhase::Running }
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
        stem::debug!("Busy spinner service loop started");
        Self::arm_frame_timer()
    }

    fn interests(&self) -> &[Interest] {
        &[]
    }

    fn dispatch(&mut self, event: LoopEvent, world: &mut BloomWorld) -> LoopAction {
        if !matches!(event, LoopEvent::Timer(BUSY_SPINNER_TIMER)) {
            return LoopAction::None;
        }

        self.frame = (self.frame + 1) % BUSY_SPINNER_FRAMES;
        if matches!(self.phase, BusySpinnerPhase::Running) && world.busy_spinner_can_fade() {
            stem::debug!("Busy spinner fading out");
            self.phase = BusySpinnerPhase::Fading { frame: 0 };
        }

        let opacity = match &self.phase {
            BusySpinnerPhase::Running => 255,
            BusySpinnerPhase::Fading { frame } => {
                let next = frame.saturating_add(1).min(BUSY_SPINNER_FADE_FRAMES);
                let remaining = BUSY_SPINNER_FADE_FRAMES.saturating_sub(next);
                ((remaining * 255) / BUSY_SPINNER_FADE_FRAMES) as u8
            }
        };

        let ticked = world.tick_busy_spinner_with_opacity(self.frame, opacity);
        match &mut self.phase {
            BusySpinnerPhase::Running => {
                if ticked {
                    LoopAction::RequestRepaintAndArmTimer {
                        delay: Duration::from_millis(BUSY_SPINNER_FRAME_MS),
                        id: BUSY_SPINNER_TIMER,
                    }
                } else {
                    LoopAction::None
                }
            }
            BusySpinnerPhase::Fading { ref mut frame } => {
                *frame = frame.saturating_add(1);
                if *frame >= BUSY_SPINNER_FADE_FRAMES {
                    let _ = world.finish_busy_spinner_handoff();
                    LoopAction::RequestRepaint
                } else if ticked {
                    LoopAction::RequestRepaintAndArmTimer {
                        delay: Duration::from_millis(BUSY_SPINNER_FRAME_MS),
                        id: BUSY_SPINNER_TIMER,
                    }
                } else {
                    LoopAction::None
                }
            }
        }
    }
}
