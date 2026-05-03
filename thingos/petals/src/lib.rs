//! Structure, state, and Taffy-backed layout for description-first UI.

#![no_std]

extern crate alloc;

pub mod calc;
pub mod chrome;
pub mod clock;
pub mod description;
pub mod launcher;
pub mod layout;
pub mod logogram;
pub mod node;
pub mod pressable;
pub mod service_loop;
pub mod showcase;
pub mod style;

pub use calc::{
    CalcError, CalcInput, CalcKey, CalcKeyNode, CalcMode, CalcNodes, CalcState, Calculator,
    CalculatorService, Func, Op,
};
pub use chrome::{
    WindowChromeNodes, WindowChromeService, render_window_chrome, window_chrome_rules,
    window_chrome_rules_for_theme, window_chrome_tree,
};
pub use clock::{Clock, ClockNodes, ClockService, ClockState};
pub use description::Description;
pub use launcher::{
    ApplicationEntry, ApplicationLauncher, ApplicationLauncherNodes, ApplicationLauncherService,
    ApplicationTileNode, display_name_from_path, glyph_for_application,
};
pub use layout::{AvailableSpace, LayoutBox, Size, UiTree, apply_style_to_taffy};
pub use logogram::{Logogram, node_glyph, node_icon_path};
pub use node::{AttrValue, Attrs, Node, NodeId};
pub use pressable::{InputEvent, KeyCode, PressConfig, PressEvent, PressState, Pressable, Vec2};
pub use service_loop::{PetalsEvent, PetalsService, ServiceAction};
pub use showcase::{PetalsShowcase, ShowcaseComponent, ShowcaseServices, StileStateDemoNodes};
pub use stile::{
    AlignItems, Color, Control, Declaration, FlexDirection, FontWeight, JustifyContent,
    ResolvedStyle, Rule, Selector, State, StateSet, Stylable, StyleProperty, Theme,
    WindowStateTokens, available_themes, default_theme, find_theme_by_name, theme_by_name,
};
pub use taffy::TaffyError;
