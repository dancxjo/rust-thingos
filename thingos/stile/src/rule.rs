use alloc::string::String;
use alloc::vec::Vec;

use crate::selector::Selector;
use crate::values::{
    AlignItems, Color, FlexDirection, FontWeight, JustifyContent, Overflow, ResolvedStyle,
    StyleProperty,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Declaration {
    BackgroundColor(Color),
    Color(Color),
    BorderWidth(f32),
    Padding(f32),
    Margin(f32),
    Gap(f32),
    Width(f32),
    Height(f32),
    MinWidth(f32),
    MinHeight(f32),
    MaxWidth(f32),
    MaxHeight(f32),
    FlexBasis(f32),
    FlexGrow(f32),
    FlexShrink(f32),
    FlexDirection(FlexDirection),
    JustifyContent(JustifyContent),
    AlignItems(AlignItems),
    OutlineColor(Color),
    OutlineWidth(f32),
    FontFamily(String),
    FontSize(f32),
    FontWeight(FontWeight),
    Overflow(Overflow),
}

impl Declaration {
    pub const fn property(&self) -> StyleProperty {
        match self {
            Declaration::BackgroundColor(_) => StyleProperty::BackgroundColor,
            Declaration::Color(_) => StyleProperty::Color,
            Declaration::BorderWidth(_) => StyleProperty::BorderWidth,
            Declaration::Padding(_) => StyleProperty::Padding,
            Declaration::Margin(_) => StyleProperty::Margin,
            Declaration::Gap(_) => StyleProperty::Gap,
            Declaration::Width(_) => StyleProperty::Width,
            Declaration::Height(_) => StyleProperty::Height,
            Declaration::MinWidth(_) => StyleProperty::MinWidth,
            Declaration::MinHeight(_) => StyleProperty::MinHeight,
            Declaration::MaxWidth(_) => StyleProperty::MaxWidth,
            Declaration::MaxHeight(_) => StyleProperty::MaxHeight,
            Declaration::FlexBasis(_) => StyleProperty::FlexBasis,
            Declaration::FlexGrow(_) => StyleProperty::FlexGrow,
            Declaration::FlexShrink(_) => StyleProperty::FlexShrink,
            Declaration::FlexDirection(_) => StyleProperty::FlexDirection,
            Declaration::JustifyContent(_) => StyleProperty::JustifyContent,
            Declaration::AlignItems(_) => StyleProperty::AlignItems,
            Declaration::OutlineColor(_) => StyleProperty::OutlineColor,
            Declaration::OutlineWidth(_) => StyleProperty::OutlineWidth,
            Declaration::FontFamily(_) => StyleProperty::FontFamily,
            Declaration::FontSize(_) => StyleProperty::FontSize,
            Declaration::FontWeight(_) => StyleProperty::FontWeight,
            Declaration::Overflow(_) => StyleProperty::Overflow,
        }
    }

    pub fn apply_to(&self, style: &mut ResolvedStyle) {
        match self {
            Declaration::BackgroundColor(value) => style.background_color = Some(*value),
            Declaration::Color(value) => style.color = Some(*value),
            Declaration::BorderWidth(value) => style.border_width = Some(*value),
            Declaration::Padding(value) => style.padding = Some(*value),
            Declaration::Margin(value) => style.margin = Some(*value),
            Declaration::Gap(value) => style.gap = Some(*value),
            Declaration::Width(value) => style.width = Some(*value),
            Declaration::Height(value) => style.height = Some(*value),
            Declaration::MinWidth(value) => style.min_width = Some(*value),
            Declaration::MinHeight(value) => style.min_height = Some(*value),
            Declaration::MaxWidth(value) => style.max_width = Some(*value),
            Declaration::MaxHeight(value) => style.max_height = Some(*value),
            Declaration::FlexBasis(value) => style.flex_basis = Some(*value),
            Declaration::FlexGrow(value) => style.flex_grow = Some(*value),
            Declaration::FlexShrink(value) => style.flex_shrink = Some(*value),
            Declaration::FlexDirection(value) => style.flex_direction = Some(*value),
            Declaration::JustifyContent(value) => style.justify_content = Some(*value),
            Declaration::AlignItems(value) => style.align_items = Some(*value),
            Declaration::OutlineColor(value) => style.outline_color = Some(*value),
            Declaration::OutlineWidth(value) => style.outline_width = Some(*value),
            Declaration::FontFamily(value) => style.font_family = Some(value.clone()),
            Declaration::FontSize(value) => style.font_size = Some(*value),
            Declaration::FontWeight(value) => style.font_weight = Some(*value),
            Declaration::Overflow(value) => style.overflow = Some(*value),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rule<D> {
    pub selector: Selector<D>,
    pub declarations: Vec<Declaration>,
}

impl<D> Rule<D> {
    pub fn new(selector: Selector<D>, declarations: Vec<Declaration>) -> Self {
        Self { selector, declarations }
    }
}
