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
pub mod style;

pub use calc::{
    CalcError, CalcInput, CalcKey, CalcKeyNode, CalcMode, CalcNodes, CalcState, Calculator, Func,
    Op,
};
pub use chrome::{WindowChromeNodes, window_chrome_tree};
pub use clock::{Clock, ClockNodes, ClockState};
pub use description::Description;
pub use launcher::{
    ApplicationEntry, ApplicationLauncher, ApplicationLauncherNodes, ApplicationTileNode,
    display_name_from_path, glyph_for_application,
};
pub use layout::{AvailableSpace, LayoutBox, Size, UiTree, apply_style_to_taffy};
pub use logogram::{Logogram, node_glyph, node_icon_path};
pub use node::{AttrValue, Attrs, Node, NodeId};
pub use pressable::{InputEvent, KeyCode, PressConfig, PressEvent, PressState, Pressable, Vec2};
pub use service_loop::{PetalsEvent, PetalsService, ServiceAction};
pub use stile::{
    AlignItems, ChromeStyle, Color, Control, Declaration, FlexDirection, FontWeight,
    JustifyContent, ResolvedStyle, Rule, Selector, State, StateSet, Stylable, StyleProperty,
    UiTheme, WindowStateTheme, default_theme, theme_by_name,
};
pub use taffy::TaffyError;
