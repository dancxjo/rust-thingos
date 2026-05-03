//! Description-based styling for Thing-OS UI surfaces.
//!
//! `stile` deliberately has no dependency on `petals`. It matches generic
//! description/state surfaces and produces resolved values that a structure
//! crate can map into layout or rendering backends.

#![no_std]

extern crate alloc;

pub mod cascade;
pub mod paint;
pub mod rule;
pub mod selector;
pub mod theme;
pub mod values;
pub mod wasi;

pub use cascade::{compute, compute_for};
pub use paint::{
    CompiledTheme, PaintCommand, PaintList, ThemeControl, ThemeControlRect, ThemeIcon, ThemePart,
    ThemeRect, ThemeRuntimeError, ThemeState, WasiThemeRuntime, WindowChromeRequest,
};
pub use rule::{Declaration, Rule};
pub use selector::{Selector, StylableSurface};
pub use theme::{
    AURORA_GLASS, DEFAULT_THEME_NAME, OBSIDIAN_BLOOM, SOLARIS_WARM, SOLARIS_WARM_PALETTE,
    SolarisWarmPalette, Theme, ThemeRenderer, WindowStateTokens, available_themes, default_theme,
    find_theme_by_name, theme_by_name,
};
pub use values::{
    AlignItems, Color, Control, FlexDirection, FontWeight, JustifyContent, ResolvedStyle, State,
    StateSet, Stylable, StyleProperty,
};
pub use wasi::{STILE_WASI_RENDER_CHROME, STILE_WASI_VERSION, WasmTheme, WasmThemeHost};

#[macro_export]
macro_rules! rule {
    ($selector:expr => [ $($decl:expr),* $(,)? ]) => {
        $crate::Rule::new($selector, alloc::vec![$($decl),*])
    };
}
