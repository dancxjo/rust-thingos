//! Description-based styling for Thing-OS UI surfaces.
//!
//! `stile` deliberately has no dependency on `petals`. It matches generic
//! description/state surfaces and produces resolved values that a structure
//! crate can map into layout or rendering backends.

#![no_std]

extern crate alloc;

pub mod cascade;
pub mod rule;
pub mod selector;
pub mod values;

pub use cascade::{compute, compute_for};
pub use rule::{Declaration, Rule};
pub use selector::{Selector, StylableSurface};
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
