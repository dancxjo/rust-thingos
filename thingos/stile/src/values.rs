use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum State {
    Hover,
    Focus,
    Active,
    Disabled,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct StateSet(u32);

impl StateSet {
    pub const HOVER: Self = Self(1 << 0);
    pub const FOCUS: Self = Self(1 << 1);
    pub const ACTIVE: Self = Self(1 << 2);
    pub const DISABLED: Self = Self(1 << 3);

    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn bits(self) -> u32 {
        self.0
    }

    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub const fn contains_state(self, state: State) -> bool {
        self.contains(Self::from_state(state))
    }

    pub fn insert(&mut self, state: State) {
        self.0 |= Self::from_state(state).0;
    }

    pub fn remove(&mut self, state: State) {
        self.0 &= !Self::from_state(state).0;
    }

    pub const fn from_state(state: State) -> Self {
        match state {
            State::Hover => Self::HOVER,
            State::Focus => Self::FOCUS,
            State::Active => Self::ACTIVE,
            State::Disabled => Self::DISABLED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StyleProperty {
    BackgroundColor,
    BorderWidth,
    Padding,
    Margin,
    Gap,
    Width,
    Height,
    MinWidth,
    MinHeight,
    MaxWidth,
    MaxHeight,
    FlexBasis,
    FlexGrow,
    FlexShrink,
    FlexDirection,
    JustifyContent,
    AlignItems,
    Color,
    OutlineColor,
    OutlineWidth,
    FontFamily,
    FontSize,
    FontWeight,
    Overflow,
}

/// How a container handles content that extends beyond its bounds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Overflow {
    /// Content is clipped to the container's bounds (default).
    #[default]
    Clip,
    /// Flex items are allowed to wrap onto the next line.
    Wrap,
    /// Overflow content is reachable via scrolling.
    Scroll,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Stylable {
    pub properties: Vec<StyleProperty>,
    pub states: Vec<State>,
    pub controls: Vec<Control>,
}

impl Stylable {
    pub fn all_layout() -> Self {
        Self {
            properties: alloc::vec![
                StyleProperty::Width,
                StyleProperty::Height,
                StyleProperty::Padding,
                StyleProperty::Margin,
                StyleProperty::Gap,
                StyleProperty::BorderWidth,
                StyleProperty::MinWidth,
                StyleProperty::MinHeight,
                StyleProperty::MaxWidth,
                StyleProperty::MaxHeight,
                StyleProperty::FlexBasis,
                StyleProperty::FlexGrow,
                StyleProperty::FlexShrink,
                StyleProperty::FlexDirection,
                StyleProperty::JustifyContent,
                StyleProperty::AlignItems,
                StyleProperty::Overflow,
            ],
            states: Vec::new(),
            controls: Vec::new(),
        }
    }

    pub fn supports(&self, property: StyleProperty) -> bool {
        self.properties.is_empty() || self.properties.contains(&property)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Control {
    Range { name: String, min: f32, max: f32, step: f32, default: f32 },
    Toggle { name: String, default: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JustifyContent {
    Start,
    Center,
    End,
    SpaceBetween,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignItems {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontWeight {
    Normal,
    Bold,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResolvedStyle {
    pub background_color: Option<Color>,
    pub color: Option<Color>,
    pub border_width: Option<f32>,
    pub padding: Option<f32>,
    pub margin: Option<f32>,
    pub gap: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub min_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
    pub flex_basis: Option<f32>,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub flex_direction: Option<FlexDirection>,
    pub justify_content: Option<JustifyContent>,
    pub align_items: Option<AlignItems>,
    pub outline_color: Option<Color>,
    pub outline_width: Option<f32>,
    pub font_family: Option<String>,
    pub font_size: Option<f32>,
    pub font_weight: Option<FontWeight>,
    pub overflow: Option<Overflow>,
}
