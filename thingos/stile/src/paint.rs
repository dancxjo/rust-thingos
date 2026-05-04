//! Theme paint-list and programmable theme contracts.
//!
//! Built-in themes render through the same request/response shapes exposed to
//! future WASI theme modules: the host owns geometry, state, focus, routing,
//! and composition; a theme module may only return validated paint commands.

use alloc::borrow::Cow;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ThemeRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl ThemeRect {
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    /// Returns the intersection of `self` and `other`, or `None` if they do
    /// not overlap.  Both zero-area rectangles and non-overlapping pairs
    /// produce `None`.
    pub fn intersect(self, other: ThemeRect) -> Option<ThemeRect> {
        let x0 = self.x.max(other.x);
        let y0 = self.y.max(other.y);
        let x1 = self.x.saturating_add(self.w).min(other.x.saturating_add(other.w));
        let y1 = self.y.saturating_add(self.h).min(other.y.saturating_add(other.h));
        if x1 <= x0 || y1 <= y0 { None } else { Some(ThemeRect::new(x0, y0, x1 - x0, y1 - y0)) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemePart {
    WindowFrame,
    WindowTitlebar,
    WindowTitle,
    WindowControlShade,
    WindowControlFullscreen,
    WindowControlClose,
    WindowResizeEdge,
    Pressable,
    TextField,
    FocusRing,
    Selection,
    Caret,
    RunBox,
    StatusBar,
    StatusBarItem,
    LauncherTile,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeIcon {
    Shade,
    Unshade,
    Fullscreen,
    Restore,
    Close,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeControl {
    Shade,
    Fullscreen,
    Close,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThemeControlRect {
    pub control: ThemeControl,
    pub rect: ThemeRect,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThemeState {
    pub active: bool,
    pub hovered: bool,
    pub primary_button_down: bool,
    pub pointer_x: i32,
    pub pointer_y: i32,
    pub shaded: bool,
    pub fullscreen: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowChromeRequest<'a> {
    pub visual_rect: ThemeRect,
    pub content_rect: ThemeRect,
    pub frame: i32,
    pub titlebar_height: i32,
    pub controls: [Option<ThemeControlRect>; 3],
    pub state: ThemeState,
    pub title: Option<&'a str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PaintCommand<'a> {
    FillRect { rect: ThemeRect, color: u32 },
    VerticalGradient { rect: ThemeRect, top: u32, bottom: u32 },
    HorizontalGradient { rect: ThemeRect, left: u32, center: u32, right: u32 },
    StrokeRect { rect: ThemeRect, thickness: i32, color: u32 },
    Shadow { rect: ThemeRect, offset_x: i32, offset_y: i32, blur_radius: i32, color: u32 },
    PushClip { rect: ThemeRect },
    PopClip,
    Text { x: i32, y: i32, px_size_bits: u32, text: Cow<'a, str>, color: u32 },
    Icon { rect: ThemeRect, icon: ThemeIcon, color: u32 },
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PaintList<'a> {
    pub commands: Vec<PaintCommand<'a>>,
}

impl<'a> PaintList<'a> {
    /// Push a clip region onto the clip stack.  All subsequent paint commands
    /// will be confined to the intersection of this rect and any outer clip.
    /// Must be paired with a corresponding [`Self::pop_clip`] call.
    pub fn push_clip(&mut self, rect: ThemeRect) {
        self.commands.push(PaintCommand::PushClip { rect });
    }

    /// Restore the clip state that was active before the matching
    /// [`Self::push_clip`] call.
    pub fn pop_clip(&mut self) {
        self.commands.push(PaintCommand::PopClip);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompiledTheme<'a> {
    pub module: &'a [u8],
    pub entrypoint: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeRuntimeError {
    Unavailable,
    InvalidModule,
    MemoryLimitExceeded,
    ExecutionFailed,
    SerializationFailed,
    InvalidResponse,
}

pub trait WasiThemeRuntime {
    fn render_window_chrome<'a>(
        &mut self,
        module: CompiledTheme<'_>,
        request: WindowChromeRequest<'a>,
        out: &mut PaintList<'a>,
    ) -> Result<(), ThemeRuntimeError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── ThemeRect::intersect ──────────────────────────────────────────────

    #[test]
    fn intersect_overlapping_rects_returns_overlap() {
        let a = ThemeRect::new(0, 0, 100, 100);
        let b = ThemeRect::new(50, 50, 100, 100);
        assert_eq!(a.intersect(b), Some(ThemeRect::new(50, 50, 50, 50)));
    }

    #[test]
    fn intersect_non_overlapping_rects_returns_none() {
        let a = ThemeRect::new(0, 0, 50, 50);
        let b = ThemeRect::new(100, 100, 50, 50);
        assert_eq!(a.intersect(b), None);
    }

    #[test]
    fn intersect_touching_edge_returns_none() {
        // Rects that share only an edge have zero area overlap.
        let a = ThemeRect::new(0, 0, 50, 50);
        let b = ThemeRect::new(50, 0, 50, 50);
        assert_eq!(a.intersect(b), None);
    }

    #[test]
    fn intersect_contained_rect_returns_inner() {
        let outer = ThemeRect::new(0, 0, 200, 200);
        let inner = ThemeRect::new(40, 40, 60, 60);
        assert_eq!(outer.intersect(inner), Some(inner));
    }

    // ── PaintList::push_clip / pop_clip ───────────────────────────────────

    #[test]
    fn push_clip_appends_push_clip_command() {
        let mut list = PaintList::default();
        let rect = ThemeRect::new(10, 20, 80, 60);
        list.push_clip(rect);
        assert_eq!(list.commands, alloc::vec![PaintCommand::PushClip { rect }]);
    }

    #[test]
    fn pop_clip_appends_pop_clip_command() {
        let mut list = PaintList::default();
        list.pop_clip();
        assert_eq!(list.commands, alloc::vec![PaintCommand::PopClip]);
    }

    #[test]
    fn push_and_pop_clip_produce_matching_pair() {
        let mut list = PaintList::default();
        let rect = ThemeRect::new(0, 0, 100, 100);
        list.push_clip(rect);
        list.pop_clip();
        assert_eq!(
            list.commands,
            alloc::vec![PaintCommand::PushClip { rect }, PaintCommand::PopClip]
        );
    }

    // ── Simulated clip-stack execution ────────────────────────────────────
    //
    // These tests simulate the clip-stack logic used by Bloom's paint
    // executor so we can verify correctness at the stile level without
    // depending on the rendering binary.

    /// Execute the clip stack from a PaintList and return the effective clip
    /// rect that was active at every FillRect command, in order.
    fn effective_clips_at_fill_rects(list: &PaintList<'_>) -> alloc::vec::Vec<Option<ThemeRect>> {
        let mut stack: alloc::vec::Vec<Option<ThemeRect>> = alloc::vec::Vec::new();
        let mut clip: Option<ThemeRect> = None;
        let mut clips = alloc::vec::Vec::new();
        for cmd in &list.commands {
            match cmd {
                PaintCommand::PushClip { rect } => {
                    stack.push(clip);
                    clip = match clip {
                        Some(current) => current.intersect(*rect),
                        None => {
                            if rect.w > 0 && rect.h > 0 { Some(*rect) } else { None }
                        }
                    };
                }
                PaintCommand::PopClip => {
                    clip = stack.pop().unwrap_or(None);
                }
                PaintCommand::FillRect { .. } => {
                    clips.push(clip);
                }
                _ => {}
            }
        }
        clips
    }

    #[test]
    fn nested_clips_intersect() {
        // Outer clip: (0,0)→(100,100); inner clip: (50,50)→(150,150).
        // The effective clip inside the inner scope must be the intersection:
        // (50,50)→(100,100).
        let outer = ThemeRect::new(0, 0, 100, 100);
        let inner = ThemeRect::new(50, 50, 100, 100);
        let mut list = PaintList::default();
        list.push_clip(outer);
        list.push_clip(inner);
        list.commands.push(PaintCommand::FillRect { rect: ThemeRect::new(0, 0, 200, 200), color: 0 });
        list.pop_clip();
        list.pop_clip();

        let clips = effective_clips_at_fill_rects(&list);
        assert_eq!(clips, alloc::vec![Some(ThemeRect::new(50, 50, 50, 50))]);
    }

    #[test]
    fn outer_clip_is_restored_after_pop() {
        let outer = ThemeRect::new(0, 0, 100, 100);
        let inner = ThemeRect::new(50, 50, 100, 100);
        let mut list = PaintList::default();
        // FillRect before entering the inner scope — uses outer clip.
        list.commands.push(PaintCommand::FillRect { rect: ThemeRect::new(0, 0, 10, 10), color: 0 });
        list.push_clip(outer);
        list.push_clip(inner);
        // FillRect inside inner scope — uses intersection.
        list.commands.push(PaintCommand::FillRect { rect: ThemeRect::new(0, 0, 10, 10), color: 1 });
        list.pop_clip();
        // FillRect after inner pop — uses outer clip again.
        list.commands.push(PaintCommand::FillRect { rect: ThemeRect::new(0, 0, 10, 10), color: 2 });
        list.pop_clip();

        let clips = effective_clips_at_fill_rects(&list);
        assert_eq!(
            clips,
            alloc::vec![
                None,                                        // before any PushClip
                Some(ThemeRect::new(50, 50, 50, 50)),       // inside inner scope (intersection)
                Some(outer),                                 // outer scope restored after PopClip
            ]
        );
    }

    #[test]
    fn sibling_clip_scopes_are_independent() {
        // Two sibling PushClip/PopClip blocks must not bleed into each other.
        // After the first block is closed, the second block starts fresh from
        // the same parent clip state.
        let parent = ThemeRect::new(0, 0, 200, 200);
        let sibling_a = ThemeRect::new(0, 0, 80, 80);
        let sibling_b = ThemeRect::new(100, 100, 80, 80);
        let mut list = PaintList::default();
        list.push_clip(parent);

        // First sibling block.
        list.push_clip(sibling_a);
        list.commands.push(PaintCommand::FillRect { rect: ThemeRect::new(0, 0, 10, 10), color: 0xa });
        list.pop_clip();

        // Second sibling block — must see parent, not sibling_a.
        list.push_clip(sibling_b);
        list.commands.push(PaintCommand::FillRect { rect: ThemeRect::new(0, 0, 10, 10), color: 0xb });
        list.pop_clip();

        list.pop_clip();

        let clips = effective_clips_at_fill_rects(&list);
        // sibling_a ∩ parent = sibling_a; sibling_b ∩ parent = sibling_b.
        assert_eq!(
            clips,
            alloc::vec![Some(sibling_a), Some(sibling_b)],
            "sibling clip scopes leaked state into each other"
        );
    }
}
