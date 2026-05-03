//! Theme paint-list and programmable theme contracts.
//!
//! Built-in themes render through the same request/response shapes exposed to
//! future WASI theme modules: the host owns geometry, state, focus, routing,
//! and composition; a theme module may only return validated paint commands.

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaintCommand<'a> {
    FillRect { rect: ThemeRect, color: u32 },
    VerticalGradient { rect: ThemeRect, top: u32, bottom: u32 },
    HorizontalGradient { rect: ThemeRect, left: u32, center: u32, right: u32 },
    StrokeRect { rect: ThemeRect, thickness: i32, color: u32 },
    Text { x: i32, y: i32, px_size_bits: u32, text: &'a str, color: u32 },
    Icon { rect: ThemeRect, icon: ThemeIcon, color: u32 },
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PaintList<'a> {
    pub commands: Vec<PaintCommand<'a>>,
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
