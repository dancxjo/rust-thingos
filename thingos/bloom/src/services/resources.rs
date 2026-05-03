//! Deferred visual resource retry service.

use core::time::Duration;

use crate::loop_types::{BloomService, Interest, LoopAction, LoopEvent};
use crate::services::wallpaper::wallpaper_target_or_default;
use crate::world::BloomWorld;

const RESOURCE_RETRY_TIMER: u64 = 1;
const RESOURCE_RETRY_MS: u64 = 1500;

pub struct ResourceRetryService {
    wallpaper_config_path: Option<&'static str>,
    load_cursor: bool,
    ready_logged: bool,
    all_settled_logged: bool,
}

impl ResourceRetryService {
    pub fn new(wallpaper_config_path: Option<&'static str>, load_cursor: bool) -> Self {
        Self { wallpaper_config_path, load_cursor, ready_logged: false, all_settled_logged: false }
    }

    fn arm_retry_timer() -> LoopAction {
        LoopAction::ArmTimer {
            delay: Duration::from_millis(RESOURCE_RETRY_MS),
            id: RESOURCE_RETRY_TIMER,
        }
    }

    fn arm_retry_timer_after(delay_ms: u64) -> LoopAction {
        LoopAction::ArmTimer { delay: Duration::from_millis(delay_ms), id: RESOURCE_RETRY_TIMER }
    }

    fn arm_retry_timer_and_repaint() -> LoopAction {
        LoopAction::RequestRepaintAndArmTimer {
            delay: Duration::from_millis(RESOURCE_RETRY_MS),
            id: RESOURCE_RETRY_TIMER,
        }
    }
}

impl BloomService for ResourceRetryService {
    fn name(&self) -> &'static str {
        "resources"
    }

    fn on_added(&mut self) -> LoopAction {
        Self::arm_retry_timer()
    }

    fn interests(&self) -> &[Interest] {
        &[]
    }

    fn dispatch(&mut self, event: LoopEvent, world: &mut BloomWorld) -> LoopAction {
        if !matches!(event, LoopEvent::Timer(RESOURCE_RETRY_TIMER)) {
            return LoopAction::None;
        }

        let wallpaper_path = self.wallpaper_config_path.map(wallpaper_target_or_default);
        let defer_font_checks = self.load_cursor && world.visuals.cursor_is_fallback();
        let had_fallback_cursor = world.visuals.cursor_is_fallback();
        let mut status = world.visuals.retry_deferred_resources(
            &world.display,
            wallpaper_path.as_deref(),
            self.load_cursor,
            !defer_font_checks,
        );
        if had_fallback_cursor && !world.visuals.cursor_is_fallback() {
            status.improved |= world.replay_deferred_cursor_motion();
        }

        if status.improved {
            world.damage.mark_full(world.primary.width, world.primary.height);
        }

        let tracks_wallpaper_or_cursor = self.wallpaper_config_path.is_some() || self.load_cursor;
        let wallpaper_and_cursor_settled =
            tracks_wallpaper_or_cursor && !status.wallpaper_pending && !status.cursor_pending;
        if wallpaper_and_cursor_settled && !self.all_settled_logged {
            stem::info!("Wallpaper and cursor are ready");
            self.all_settled_logged = true;
        }
        if !status.pending
            && self.load_cursor
            && world.visuals.cursor_is_fallback()
            && world.visuals.pending_cursor_ready()
        {
            world.arm_busy_spinner_handoff();
        }

        if status.pending {
            if status.improved {
                Self::arm_retry_timer_and_repaint()
            } else {
                Self::arm_retry_timer()
            }
        } else if defer_font_checks {
            if !self.ready_logged {
                stem::info!("Deferred visual resources are ready");
                self.ready_logged = true;
            }
            if status.improved {
                LoopAction::RequestRepaintAndArmTimer {
                    delay: Duration::from_millis(RESOURCE_RETRY_MS),
                    id: RESOURCE_RETRY_TIMER,
                }
            } else {
                Self::arm_retry_timer_after(RESOURCE_RETRY_MS)
            }
        } else if status.improved {
            if !self.ready_logged {
                stem::info!("Deferred visual resources are ready");
                self.ready_logged = true;
            }
            LoopAction::RequestRepaint
        } else {
            if !self.ready_logged {
                stem::info!("Deferred visual resources are ready");
                self.ready_logged = true;
            }
            LoopAction::None
        }
    }
}
