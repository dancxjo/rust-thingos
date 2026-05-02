//! Structure, state, and Taffy-backed layout for description-first UI.

#![no_std]

extern crate alloc;

pub mod description;
pub mod layout;
pub mod node;
pub mod service_loop;
pub mod style;

pub use description::Description;
pub use layout::{AvailableSpace, LayoutBox, Size, UiTree, apply_style_to_taffy};
pub use node::{AttrValue, Attrs, Node, NodeId};
pub use service_loop::{PetalsEvent, PetalsService, ServiceAction};
pub use stile::{
    AlignItems, Color, Control, Declaration, FlexDirection, FontWeight, JustifyContent,
    ResolvedStyle, Rule, Selector, State, StateSet, Stylable, StyleProperty,
};
