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

pub use cascade::{compute, compute_for};
pub use paint::{
    CompiledTheme, PaintCommand, PaintList, ThemeControl, ThemeControlRect, ThemeIcon, ThemePart,
    ThemeRect, ThemeRuntimeError, ThemeState, WasiThemeRuntime, WindowChromeRequest,
};
pub use rule::{Declaration, Rule};
pub use selector::{Selector, StylableSurface};
pub use theme::{
    AURORA_GLASS, DEFAULT_THEME_NAME, OBSIDIAN_BLOOM, Theme, ThemeRenderer, WindowStateTokens,
    default_theme, theme_by_name,
};
pub use values::{
    AlignItems, Color, Control, FlexDirection, FontWeight, JustifyContent, ResolvedStyle, State,
    StateSet, Stylable, StyleProperty,
};

#[macro_export]
macro_rules! rule {
    ($selector:expr => [ $($decl:expr),* $(,)? ]) => {
        $crate::Rule::new($selector, alloc::vec![$($decl),*])
    };
}
