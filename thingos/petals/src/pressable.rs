//! Device-agnostic Pressable primitive: unified press / hold / release model.
//!
//! Normalizes touch, mouse, and keyboard input into a single human-centered
//! interaction model: press -> hold -> release (or abandon). Emits
//! [`PressEvent::Press`], [`PressEvent::LongPress`], [`PressEvent::Repeat`],
//! and [`PressEvent::Release`].
//!
//! The caller is responsible for driving the timing by calling
//! [`Pressable::update`] with a monotonic nanosecond timestamp on each frame
//! or timer tick. Input events are fed via [`Pressable::process`].
//!
//! # Hit region contract
//!
//! The hit region of a [`Pressable`] node in a [`crate::UiTree`] is its
//! **computed layout bounds** — the same rectangle that is painted on screen.
//! There is no separate "invisible" extension of the touch target; what you see
//! is exactly what you can press.
//!
//! ## Full-row list items
//!
//! For list-style layouts where each row should be activatable across its full
//! width, the row pressable must expand to fill available space.  Use
//! [`crate::UiTree::list_row_pressable`] which sets `flex_grow: 1.0` on the
//! node; pair it with a column container that has `align_items: Stretch` so
//! the node also fills the container's cross-axis.
//!
//! ## Nested pressables
//!
//! When a [`Pressable`] node contains child nodes (including other
//! [`Pressable`] nodes), input events are routed by the **innermost** matching
//! hit region first (deepest first in the render tree).  A child pressable
//! captures events that land inside its bounds; events outside its bounds but
//! inside the parent's bounds are handled by the parent.  Each [`Pressable`]
//! state machine is independent — the parent's state is not affected by events
//! consumed by a child.
//!
//! # Example
//!
//! ```
//! use petals::pressable::{InputEvent, PressConfig, PressEvent, Pressable, Vec2};
//!
//! let mut p = Pressable::new(PressConfig::default());
//!
//! let events = p.process(InputEvent::Down { id: 0, position: Vec2::new(10.0, 20.0) }, 0);
//! assert!(events.contains(&PressEvent::Press));
//!
//! let events = p.process(InputEvent::Up { id: 0 }, 100_000_000);
//! assert!(events.contains(&PressEvent::Release { cancelled: false }));
//! ```

use alloc::vec::Vec;

use abi::hid::Key;

/// Re-export of [`abi::hid::Key`] for use as keyboard codes in input events.
pub type KeyCode = Key;

/// A 2-D floating-point position used by pointer input events.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Squared Euclidean distance from `other`.
    pub fn distance_sq(self, other: Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

/// Normalised input event fed into a [`Pressable`] state machine.
///
/// Pointer events carry a stable `id` that identifies the finger or button so
/// the caller can maintain independent [`Pressable`] instances for concurrent
/// contacts.  Keyboard events carry the logical [`KeyCode`]; only [`Key::Enter`]
/// and [`Key::Space`] are treated as press/release; all other keys are ignored.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputEvent {
    /// Pointer or button contact began at `position`.
    Down { id: u32, position: Vec2 },
    /// Pointer moved to `position` while held.
    Move { id: u32, position: Vec2 },
    /// Pointer or button was released normally.
    Up { id: u32 },
    /// Pointer contact was cancelled by the system (e.g. grabbed by another surface).
    Cancel { id: u32 },
    /// Keyboard key pressed.
    KeyDown { key: KeyCode },
    /// Keyboard key released.
    KeyUp { key: KeyCode },
}

/// Configuration for a [`Pressable`] instance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PressConfig {
    /// Milliseconds of continuous hold before a [`PressEvent::LongPress`] fires.
    pub long_press_ms: u64,
    /// Milliseconds after the initial press before the first [`PressEvent::Repeat`] fires.
    pub repeat_delay_ms: u64,
    /// Milliseconds between subsequent [`PressEvent::Repeat`] events.
    pub repeat_rate_ms: u64,
    /// Pixels of pointer travel that cancel the press (when `cancel_on_leave` is `true`).
    pub move_tolerance_px: f32,
    /// Cancel the press when the pointer moves beyond [`move_tolerance_px`] from
    /// the origin position at which the press began.
    pub cancel_on_leave: bool,
}

impl Default for PressConfig {
    fn default() -> Self {
        Self {
            long_press_ms: 500,
            repeat_delay_ms: 400,
            repeat_rate_ms: 50,
            move_tolerance_px: 8.0,
            cancel_on_leave: true,
        }
    }
}

/// Observable state snapshot of a [`Pressable`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PressState {
    /// Whether a press is currently active.
    pub is_pressed: bool,
    /// Nanosecond timestamp when the current press began, or `None` if idle.
    pub press_started_at: Option<u64>,
    /// Whether a [`PressEvent::LongPress`] has already been emitted for the
    /// current press.
    pub is_long_press: bool,
}

/// Semantic events emitted by a [`Pressable`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PressEvent {
    /// Fired immediately when the press begins.
    Press,
    /// Fired once when the press has been held for [`PressConfig::long_press_ms`].
    LongPress,
    /// Fired periodically while the press is held, starting after
    /// [`PressConfig::repeat_delay_ms`] and repeating every
    /// [`PressConfig::repeat_rate_ms`].
    Repeat,
    /// Fired when the press ends, either normally or cancelled.
    Release {
        /// `true` when the release was forced (pointer left bounds, system
        /// cancel, or the pointer ID does not match the tracked contact).
        cancelled: bool,
    },
}

/// Which kind of input is currently being tracked.
#[derive(Clone, Copy, Debug, PartialEq)]
enum ActiveSource {
    Pointer { id: u32, origin: Vec2 },
    Key { key: KeyCode },
}

/// Device-agnostic state machine that turns raw input into semantic press events.
///
/// Create one instance per interactive element (or per pointer contact for
/// multi-touch). Feed input via [`process`][Pressable::process] and advance
/// timers via [`update`][Pressable::update] on each frame tick.
pub struct Pressable {
    config: PressConfig,
    state: PressState,
    source: Option<ActiveSource>,
    /// `true` once a [`PressEvent::LongPress`] has been emitted this press.
    long_press_fired: bool,
    /// Nanosecond timestamp of the last [`PressEvent::Repeat`] event, or
    /// `None` if no repeat has fired yet.
    last_repeat_ns: Option<u64>,
}

impl Pressable {
    pub fn new(config: PressConfig) -> Self {
        Self {
            config,
            state: PressState::default(),
            source: None,
            long_press_fired: false,
            last_repeat_ns: None,
        }
    }

    /// Current observable state.
    pub fn state(&self) -> PressState {
        self.state
    }

    /// Feed an input event at nanosecond timestamp `now_ns`.
    ///
    /// Returns any [`PressEvent`]s synchronously generated by this event.
    /// Call [`update`][Pressable::update] separately to generate timer-driven
    /// events (LongPress, Repeat).
    pub fn process(&mut self, event: InputEvent, now_ns: u64) -> Vec<PressEvent> {
        match event {
            InputEvent::Down { id, position } => self.handle_down_pointer(id, position, now_ns),
            InputEvent::Move { id, position } => self.handle_move(id, position),
            InputEvent::Up { id } => self.handle_up_pointer(id),
            InputEvent::Cancel { id } => self.handle_cancel_pointer(id),
            InputEvent::KeyDown { key } => self.handle_key_down(key, now_ns),
            InputEvent::KeyUp { key } => self.handle_key_up(key),
        }
    }

    /// Advance timers to `now_ns` and return any time-driven events (LongPress,
    /// Repeat). Should be called every frame or on a regular timer tick.
    pub fn update(&mut self, now_ns: u64) -> Vec<PressEvent> {
        if !self.state.is_pressed {
            return Vec::new();
        }
        let started = match self.state.press_started_at {
            Some(t) => t,
            None => return Vec::new(),
        };

        let mut events = Vec::new();
        let elapsed_ns = now_ns.saturating_sub(started);
        let long_press_ns = self.config.long_press_ms * 1_000_000;
        let repeat_delay_ns = self.config.repeat_delay_ms * 1_000_000;
        let repeat_rate_ns = self.config.repeat_rate_ms * 1_000_000;

        // Emit LongPress once when the threshold is crossed.
        if !self.long_press_fired && elapsed_ns >= long_press_ns {
            self.long_press_fired = true;
            self.state.is_long_press = true;
            events.push(PressEvent::LongPress);
        }

        // Emit Repeat: first after repeat_delay_ns from start, then every
        // repeat_rate_ns.
        if elapsed_ns >= repeat_delay_ns {
            match self.last_repeat_ns {
                None => {
                    self.last_repeat_ns = Some(now_ns);
                    events.push(PressEvent::Repeat);
                }
                Some(last) => {
                    if now_ns.saturating_sub(last) >= repeat_rate_ns {
                        self.last_repeat_ns = Some(now_ns);
                        events.push(PressEvent::Repeat);
                    }
                }
            }
        }

        events
    }

    // --- private helpers ---

    fn begin_press(&mut self, source: ActiveSource, now_ns: u64) -> Vec<PressEvent> {
        self.source = Some(source);
        self.state =
            PressState { is_pressed: true, press_started_at: Some(now_ns), is_long_press: false };
        self.long_press_fired = false;
        self.last_repeat_ns = None;
        alloc::vec![PressEvent::Press]
    }

    fn end_press(&mut self, cancelled: bool) -> Vec<PressEvent> {
        self.source = None;
        self.state = PressState::default();
        self.long_press_fired = false;
        self.last_repeat_ns = None;
        alloc::vec![PressEvent::Release { cancelled }]
    }

    fn handle_down_pointer(&mut self, id: u32, position: Vec2, now_ns: u64) -> Vec<PressEvent> {
        if self.state.is_pressed {
            // Already tracking a different contact; ignore.
            return Vec::new();
        }
        self.begin_press(ActiveSource::Pointer { id, origin: position }, now_ns)
    }

    fn handle_move(&mut self, id: u32, position: Vec2) -> Vec<PressEvent> {
        if !self.config.cancel_on_leave {
            return Vec::new();
        }
        let origin = match self.source {
            Some(ActiveSource::Pointer { id: tracked_id, origin }) if tracked_id == id => origin,
            _ => return Vec::new(),
        };
        let tolerance = self.config.move_tolerance_px;
        if position.distance_sq(origin) > tolerance * tolerance {
            self.end_press(true)
        } else {
            Vec::new()
        }
    }

    fn handle_up_pointer(&mut self, id: u32) -> Vec<PressEvent> {
        match self.source {
            Some(ActiveSource::Pointer { id: tracked_id, .. }) if tracked_id == id => {
                self.end_press(false)
            }
            _ => Vec::new(),
        }
    }

    fn handle_cancel_pointer(&mut self, id: u32) -> Vec<PressEvent> {
        match self.source {
            Some(ActiveSource::Pointer { id: tracked_id, .. }) if tracked_id == id => {
                self.end_press(true)
            }
            _ => Vec::new(),
        }
    }

    fn handle_key_down(&mut self, key: KeyCode, now_ns: u64) -> Vec<PressEvent> {
        if !is_press_key(key) {
            return Vec::new();
        }
        if self.state.is_pressed {
            // Key repeat from OS; ignore – we handle our own repeat timing.
            return Vec::new();
        }
        self.begin_press(ActiveSource::Key { key }, now_ns)
    }

    fn handle_key_up(&mut self, key: KeyCode) -> Vec<PressEvent> {
        if !is_press_key(key) {
            return Vec::new();
        }
        match self.source {
            Some(ActiveSource::Key { key: tracked_key }) if tracked_key == key => {
                self.end_press(false)
            }
            _ => Vec::new(),
        }
    }
}

/// Returns `true` for keys that act as a press (Enter and Space).
fn is_press_key(key: KeyCode) -> bool {
    matches!(key, Key::Enter | Key::Space)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MS: u64 = 1_000_000;

    fn make() -> Pressable {
        Pressable::new(PressConfig::default())
    }

    #[test]
    fn pointer_down_emits_press() {
        let mut p = make();
        let evs = p.process(InputEvent::Down { id: 0, position: Vec2::new(0.0, 0.0) }, 0);
        assert_eq!(evs, alloc::vec![PressEvent::Press]);
        assert!(p.state().is_pressed);
    }

    #[test]
    fn pointer_up_emits_release() {
        let mut p = make();
        p.process(InputEvent::Down { id: 0, position: Vec2::new(0.0, 0.0) }, 0);
        let evs = p.process(InputEvent::Up { id: 0 }, 100 * MS);
        assert_eq!(evs, alloc::vec![PressEvent::Release { cancelled: false }]);
        assert!(!p.state().is_pressed);
    }

    #[test]
    fn cancel_emits_release_cancelled() {
        let mut p = make();
        p.process(InputEvent::Down { id: 0, position: Vec2::new(0.0, 0.0) }, 0);
        let evs = p.process(InputEvent::Cancel { id: 0 }, 100 * MS);
        assert_eq!(evs, alloc::vec![PressEvent::Release { cancelled: true }]);
    }

    #[test]
    fn move_within_tolerance_does_not_cancel() {
        let mut p = make();
        p.process(InputEvent::Down { id: 0, position: Vec2::new(0.0, 0.0) }, 0);
        let evs = p.process(InputEvent::Move { id: 0, position: Vec2::new(4.0, 4.0) }, 10 * MS);
        assert!(evs.is_empty());
        assert!(p.state().is_pressed);
    }

    #[test]
    fn move_beyond_tolerance_cancels() {
        let mut p = make();
        p.process(InputEvent::Down { id: 0, position: Vec2::new(0.0, 0.0) }, 0);
        let evs = p.process(InputEvent::Move { id: 0, position: Vec2::new(20.0, 0.0) }, 10 * MS);
        assert_eq!(evs, alloc::vec![PressEvent::Release { cancelled: true }]);
        assert!(!p.state().is_pressed);
    }

    #[test]
    fn move_cancel_disabled_when_cancel_on_leave_false() {
        let cfg = PressConfig { cancel_on_leave: false, ..PressConfig::default() };
        let mut p = Pressable::new(cfg);
        p.process(InputEvent::Down { id: 0, position: Vec2::new(0.0, 0.0) }, 0);
        let evs = p.process(InputEvent::Move { id: 0, position: Vec2::new(100.0, 100.0) }, 10 * MS);
        assert!(evs.is_empty());
        assert!(p.state().is_pressed);
    }

    #[test]
    fn long_press_fires_after_threshold() {
        let mut p = make();
        p.process(InputEvent::Down { id: 0, position: Vec2::new(0.0, 0.0) }, 0);

        // Just before threshold – no LongPress.
        let evs = p.update(499 * MS);
        assert!(!evs.contains(&PressEvent::LongPress));

        // At threshold – LongPress fires.
        let evs = p.update(500 * MS);
        assert!(evs.contains(&PressEvent::LongPress));
        assert!(p.state().is_long_press);

        // Does not fire again.
        let evs = p.update(600 * MS);
        assert!(!evs.contains(&PressEvent::LongPress));
    }

    #[test]
    fn repeat_fires_after_delay_and_at_rate() {
        let mut p = make();
        p.process(InputEvent::Down { id: 0, position: Vec2::new(0.0, 0.0) }, 0);

        // Just before repeat_delay_ms (400ms) – no Repeat.
        let evs = p.update(399 * MS);
        assert!(!evs.contains(&PressEvent::Repeat));

        // At repeat_delay_ms – first Repeat.
        let evs = p.update(400 * MS);
        assert!(evs.contains(&PressEvent::Repeat));

        // Before next repeat interval (50ms) – no Repeat.
        let evs = p.update(430 * MS);
        assert!(!evs.contains(&PressEvent::Repeat));

        // At next interval (400 + 50 = 450ms) – another Repeat.
        let evs = p.update(450 * MS);
        assert!(evs.contains(&PressEvent::Repeat));
    }

    #[test]
    fn no_timer_events_when_idle() {
        let mut p = make();
        let evs = p.update(9999 * MS);
        assert!(evs.is_empty());
    }

    #[test]
    fn enter_key_acts_as_press_release() {
        let mut p = make();
        let evs = p.process(InputEvent::KeyDown { key: Key::Enter }, 0);
        assert_eq!(evs, alloc::vec![PressEvent::Press]);

        let evs = p.process(InputEvent::KeyUp { key: Key::Enter }, 100 * MS);
        assert_eq!(evs, alloc::vec![PressEvent::Release { cancelled: false }]);
    }

    #[test]
    fn space_key_acts_as_press_release() {
        let mut p = make();
        let evs = p.process(InputEvent::KeyDown { key: Key::Space }, 0);
        assert_eq!(evs, alloc::vec![PressEvent::Press]);

        let evs = p.process(InputEvent::KeyUp { key: Key::Space }, 50 * MS);
        assert_eq!(evs, alloc::vec![PressEvent::Release { cancelled: false }]);
    }

    #[test]
    fn non_press_keys_are_ignored() {
        let mut p = make();
        let evs = p.process(InputEvent::KeyDown { key: Key::A }, 0);
        assert!(evs.is_empty());
        assert!(!p.state().is_pressed);
    }

    #[test]
    fn mismatched_pointer_id_ignored() {
        let mut p = make();
        p.process(InputEvent::Down { id: 1, position: Vec2::new(0.0, 0.0) }, 0);
        // Up from a different pointer – should not release.
        let evs = p.process(InputEvent::Up { id: 2 }, 100 * MS);
        assert!(evs.is_empty());
        assert!(p.state().is_pressed);
    }

    #[test]
    fn second_down_while_pressed_is_ignored() {
        let mut p = make();
        p.process(InputEvent::Down { id: 0, position: Vec2::new(0.0, 0.0) }, 0);
        let evs = p.process(InputEvent::Down { id: 1, position: Vec2::new(5.0, 5.0) }, 10 * MS);
        assert!(evs.is_empty());
    }

    #[test]
    fn press_state_cleared_after_release() {
        let mut p = make();
        p.process(InputEvent::Down { id: 0, position: Vec2::new(0.0, 0.0) }, 0);
        p.process(InputEvent::Up { id: 0 }, 100 * MS);
        let s = p.state();
        assert!(!s.is_pressed);
        assert!(s.press_started_at.is_none());
        assert!(!s.is_long_press);
    }
}
