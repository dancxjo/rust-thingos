use alloc::string::String;
use alloc::vec::Vec;

use stile::{
    AlignItems as StileAlignItems, FlexDirection as StileFlexDirection, FontWeight,
    JustifyContent as StileJustifyContent, Overflow as StileOverflow, ResolvedStyle, Rule,
};
use taffy::prelude::*;
pub use taffy::prelude::{AvailableSpace, Size};

use crate::description::Description;
use crate::node::{AttrValue, Node, NodeId};
use crate::style::{State, Stylable, StyleProperty};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LayoutBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub struct UiTree {
    nodes: Vec<Node>,
    root: NodeId,
    taffy: TaffyTree<LayoutContext>,
}

#[derive(Clone, Debug, PartialEq)]
enum LayoutContext {
    Text(TextMeasure),
}

#[derive(Clone, Debug, PartialEq)]
struct TextMeasure {
    text: String,
    font_size: f32,
    font_weight: FontWeight,
}

impl UiTree {
    pub fn new() -> Result<Self, taffy::TaffyError> {
        let mut taffy = TaffyTree::new();
        let root_layout = taffy.new_leaf(Style::default())?;
        let mut root = Node::new(0, root_layout).with_description(Description::Container);
        root.stylable = Stylable::all_layout();
        Ok(Self { nodes: alloc::vec![root], root: 0, taffy })
    }

    pub fn root(&self) -> NodeId {
        self.root
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id as usize)
    }

    pub fn node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id as usize)
    }

    pub fn add_node(&mut self, descriptions: &[Description]) -> Result<NodeId, taffy::TaffyError> {
        let layout = self.taffy.new_leaf(Style::default())?;
        let id = self.nodes.len() as NodeId;
        let mut node = Node::new(id, layout);
        node.descriptions.extend_from_slice(descriptions);
        node.stylable = default_stylable_for(descriptions);
        self.nodes.push(node);
        Ok(id)
    }

    pub fn pressable(&mut self, label: &str) -> Result<NodeId, taffy::TaffyError> {
        let id = self.add_node(&[
            Description::Pressable,
            Description::Focusable,
            Description::Container,
        ])?;
        if let Some(node) = self.node_mut(id) {
            node.attrs.insert(alloc::string::String::from("label"), AttrValue::Str(label.into()));
            node.stylable.states =
                alloc::vec![State::Hover, State::Focus, State::Active, State::Disabled,];
        }
        Ok(id)
    }

    /// Create a full-width list-row [`Pressable`] node.
    ///
    /// Equivalent to [`pressable`][Self::pressable] but also tags the node
    /// with [`Description::ListRow`].
    ///
    /// To achieve a full-width hit region, add `align_items: Stretch` to the
    /// parent container.  In a `flex-direction: column` list, the cross-axis
    /// is horizontal, so `Stretch` causes each `ListRow` child to fill the
    /// full container width.  The [`Description::ListRow`] tag lets callers
    /// write targeted CSS-like rules (e.g. minimum height, padding) and is
    /// also recognised by the debug overlay as a hit region marker.
    ///
    /// # Hit region contract
    ///
    /// The hit region of a `ListRow` is its computed layout bounds — the same
    /// rectangle that is painted.  No invisible extension is added beyond the
    /// visible area.  When the parent container uses `align_items: Stretch`,
    /// those bounds span the full container width, fulfilling the full-row
    /// activation requirement.
    pub fn list_row_pressable(&mut self, label: &str) -> Result<NodeId, taffy::TaffyError> {
        let id = self.pressable(label)?;
        if let Some(node) = self.node_mut(id) {
            node.descriptions.push(Description::ListRow);
        }
        Ok(id)
    }

    pub fn text(&mut self, text: &str) -> Result<NodeId, taffy::TaffyError> {
        let id = self.add_node(&[Description::Textual])?;
        if let Some(node) = self.node_mut(id) {
            node.attrs.insert(alloc::string::String::from("text"), AttrValue::Str(text.into()));
        }
        Ok(id)
    }

    pub fn add_child(&mut self, parent: NodeId, child: NodeId) -> Result<(), taffy::TaffyError> {
        let parent_layout = self.nodes[parent as usize].layout;
        let child_layout = self.nodes[child as usize].layout;
        self.nodes[parent as usize].children.push(child);
        self.taffy.add_child(parent_layout, child_layout)
    }

    pub fn apply_style(
        &mut self,
        id: NodeId,
        style: ResolvedStyle,
    ) -> Result<(), taffy::TaffyError> {
        let layout = self.nodes[id as usize].layout;
        let taffy_style = apply_style_to_taffy(&style);
        self.nodes[id as usize].style = style;
        self.taffy.set_style(layout, taffy_style)
    }

    pub fn apply_styles(
        &mut self,
        styles: &[(NodeId, ResolvedStyle)],
    ) -> Result<(), taffy::TaffyError> {
        for (id, style) in styles {
            self.apply_style(*id, style.clone())?;
        }
        Ok(())
    }

    pub fn restyle(&mut self, rules: &[Rule<Description>]) -> Result<(), taffy::TaffyError> {
        for id in 0..self.nodes.len() {
            let style = stile::compute_for(&self.nodes[id], rules);
            self.apply_style(id as NodeId, style)?;
        }
        Ok(())
    }

    pub fn compute_layout(
        &mut self,
        available_space: Size<AvailableSpace>,
    ) -> Result<(), taffy::TaffyError> {
        self.sync_layout_contexts()?;
        let root_layout = self.nodes[self.root as usize].layout;
        self.taffy.compute_layout_with_measure(
            root_layout,
            available_space,
            |known_dimensions, available_space, _node_id, context, _style| {
                measure_layout_context(known_dimensions, available_space, context)
            },
        )
    }

    pub fn compute_content_fit_layout(
        &mut self,
        max_width: f32,
        max_height: f32,
    ) -> Result<Size<f32>, taffy::TaffyError> {
        self.compute_layout(Size::MAX_CONTENT)?;
        let preferred = self.layout_box(self.root)?;
        let width = if preferred.width > max_width { max_width } else { preferred.width };
        let height = if preferred.height > max_height { max_height } else { preferred.height };

        let mut root_style = self.nodes[self.root as usize].style.clone();
        root_style.width = Some(width);
        root_style.height = Some(height);
        self.apply_style(self.root, root_style)?;
        self.compute_layout(Size {
            width: AvailableSpace::Definite(width),
            height: AvailableSpace::Definite(height),
        })?;
        Ok(Size { width, height })
    }

    pub fn layout_box(&self, id: NodeId) -> Result<LayoutBox, taffy::TaffyError> {
        let layout = self.taffy.layout(self.nodes[id as usize].layout)?;
        Ok(LayoutBox {
            x: layout.location.x,
            y: layout.location.y,
            width: layout.size.width,
            height: layout.size.height,
        })
    }

    pub fn global_layout_box(&self, id: NodeId) -> Result<LayoutBox, taffy::TaffyError> {
        let mut out = self.layout_box(id)?;
        let mut current = id;
        while current != self.root {
            let Some(parent) = self.parent_of(current) else {
                break;
            };
            let parent_box = self.layout_box(parent)?;
            out.x += parent_box.x;
            out.y += parent_box.y;
            current = parent;
        }
        Ok(out)
    }

    pub fn set_state(&mut self, id: NodeId, state: State, enabled: bool) {
        if let Some(node) = self.node_mut(id) {
            if enabled {
                node.states.insert(state);
            } else {
                node.states.remove(state);
            }
        }
    }

    /// Returns the declared overflow mode for the given node.
    ///
    /// When no explicit overflow is set, containers default to [`StileOverflow::Clip`] so
    /// that the renderer always has a well-defined policy.  Non-container nodes return
    /// `None` to indicate that they carry no overflow intent of their own.
    pub fn overflow_intent(&self, id: NodeId) -> Option<StileOverflow> {
        let node = self.nodes.get(id as usize)?;
        let is_container = node.descriptions.iter().any(|d| {
            matches!(
                d,
                Description::Container
                    | Description::ScrollContainer
                    | Description::WindowChrome
                    | Description::WindowContent
                    | Description::ApplicationGrid
                    | Description::ApplicationRow
            )
        });
        if !is_container {
            return None;
        }
        Some(node.style.overflow.unwrap_or(StileOverflow::Clip))
    }

    /// Creates a scroll container node: a [`Description::Container`] +
    /// [`Description::ScrollContainer`] node with [`Overflow::Scroll`] pre-applied.
    pub fn scroll_container(&mut self) -> Result<NodeId, taffy::TaffyError> {
        let id = self.add_node(&[Description::Container, Description::ScrollContainer])?;
        if let Some(node) = self.node_mut(id) {
            node.style.overflow = Some(StileOverflow::Scroll);
        }
        Ok(id)
    }

    fn parent_of(&self, id: NodeId) -> Option<NodeId> {
        self.nodes
            .iter()
            .find(|node| node.children.iter().any(|child| *child == id))
            .map(|node| node.id)
    }

    fn sync_layout_contexts(&mut self) -> Result<(), taffy::TaffyError> {
        for node in &self.nodes {
            self.taffy.set_node_context(node.layout, layout_context_for(node))?;
        }
        Ok(())
    }
}

fn layout_context_for(node: &Node) -> Option<LayoutContext> {
    if !(node.descriptions.contains(&Description::Textual)
        || node.descriptions.contains(&Description::Title)
        || node.descriptions.contains(&Description::Logogram)
        || node.descriptions.contains(&Description::Pressable))
    {
        return None;
    }

    let text = text_like_attr(node)?;
    Some(LayoutContext::Text(TextMeasure {
        text: String::from(text),
        font_size: node.style.font_size.unwrap_or(14.0),
        font_weight: node.style.font_weight.unwrap_or(FontWeight::Normal),
    }))
}

fn text_like_attr(node: &Node) -> Option<&str> {
    match node
        .attrs
        .get("text")
        .or_else(|| node.attrs.get("label"))
        .or_else(|| node.attrs.get("glyph"))
    {
        Some(AttrValue::Str(value)) => Some(value.as_str()),
        _ => None,
    }
}

fn measure_layout_context(
    known_dimensions: Size<Option<f32>>,
    _available_space: Size<AvailableSpace>,
    context: Option<&mut LayoutContext>,
) -> Size<f32> {
    if let Size { width: Some(width), height: Some(height) } = known_dimensions {
        return Size { width, height };
    }

    match context {
        Some(LayoutContext::Text(text)) => measure_text(known_dimensions, text),
        None => Size::ZERO,
    }
}

fn measure_text(known_dimensions: Size<Option<f32>>, text: &TextMeasure) -> Size<f32> {
    let px_size = text.font_size.max(1.0);
    let weight_factor = match text.font_weight {
        FontWeight::Normal => 0.62,
        FontWeight::Bold => 0.66,
    };
    let advance = (px_size * weight_factor).max(1.0);
    let natural_width = text.text.chars().count() as f32 * advance;
    let natural_height = px_size * 1.2;

    Size {
        width: known_dimensions.width.unwrap_or(natural_width),
        height: known_dimensions.height.unwrap_or(natural_height),
    }
}

pub fn apply_style_to_taffy(style: &ResolvedStyle) -> Style {
    let length = |value: f32| LengthPercentage::from_length(value);
    let length_auto = |value: f32| LengthPercentageAuto::from_length(value);

    let flex_wrap = match style.overflow {
        Some(StileOverflow::Wrap) => FlexWrap::Wrap,
        _ => FlexWrap::NoWrap,
    };

    Style {
        display: Display::Flex,
        size: Size {
            width: style.width.map(Dimension::from_length).unwrap_or(Dimension::AUTO),
            height: style.height.map(Dimension::from_length).unwrap_or(Dimension::AUTO),
        },
        min_size: Size {
            width: style.min_width.map(Dimension::from_length).unwrap_or(Dimension::AUTO),
            height: style.min_height.map(Dimension::from_length).unwrap_or(Dimension::AUTO),
        },
        max_size: Size {
            width: style.max_width.map(Dimension::from_length).unwrap_or(Dimension::AUTO),
            height: style.max_height.map(Dimension::from_length).unwrap_or(Dimension::AUTO),
        },
        margin: rect_all_auto(length_auto(style.margin.unwrap_or(0.0))),
        padding: rect_all(length(style.padding.unwrap_or(0.0))),
        border: rect_all(length(style.border_width.unwrap_or(0.0))),
        gap: Size {
            width: length(style.gap.unwrap_or(0.0)),
            height: length(style.gap.unwrap_or(0.0)),
        },
        flex_direction: map_flex_direction(
            style.flex_direction.unwrap_or(StileFlexDirection::Column),
        ),
        flex_wrap,
        flex_basis: style.flex_basis.map(Dimension::from_length).unwrap_or(Dimension::AUTO),
        flex_grow: style.flex_grow.unwrap_or(0.0),
        flex_shrink: style.flex_shrink.unwrap_or(1.0),
        justify_content: style.justify_content.map(map_justify_content),
        align_items: style.align_items.map(map_align_items),
        ..Style::default()
    }
}

fn default_stylable_for(descriptions: &[Description]) -> Stylable {
    let mut stylable = Stylable::all_layout();
    if descriptions.contains(&Description::Pressable) {
        stylable.properties.extend_from_slice(&[
            StyleProperty::BackgroundColor,
            StyleProperty::OutlineColor,
            StyleProperty::OutlineWidth,
        ]);
    }
    if descriptions.iter().any(|description| {
        matches!(
            description,
            Description::Calculator
                | Description::DisplayPanel
                | Description::KeypadGrid
                | Description::KeypadRow
                | Description::ModeToggle
                | Description::RunBox
                | Description::TextField
                | Description::ApplicationLauncher
                | Description::ApplicationGrid
                | Description::ApplicationRow
                | Description::ApplicationTile
                | Description::Showcase
                | Description::ShowcasePanel
                | Description::ShowcaseHeader
                | Description::ShowcaseSwatch
                | Description::ShowcaseStateStrip
                | Description::WindowChrome
                | Description::WindowFrame
                | Description::Titlebar
                | Description::WindowContent
                | Description::ResizeEdge
                | Description::ScrollContainer
        )
    }) {
        stylable.properties.push(StyleProperty::BackgroundColor);
    }
    if descriptions.contains(&Description::Textual)
        || descriptions.contains(&Description::Logogram)
        || descriptions.contains(&Description::Title)
        || descriptions.contains(&Description::ApplicationTitle)
    {
        stylable.properties.extend_from_slice(&[
            StyleProperty::Color,
            StyleProperty::FontFamily,
            StyleProperty::FontSize,
            StyleProperty::FontWeight,
        ]);
    }
    stylable
}

fn rect_all(value: LengthPercentage) -> Rect<LengthPercentage> {
    Rect { left: value, right: value, top: value, bottom: value }
}

fn rect_all_auto(value: LengthPercentageAuto) -> Rect<LengthPercentageAuto> {
    Rect { left: value, right: value, top: value, bottom: value }
}

fn map_flex_direction(value: StileFlexDirection) -> FlexDirection {
    match value {
        StileFlexDirection::Row => FlexDirection::Row,
        StileFlexDirection::Column => FlexDirection::Column,
    }
}

fn map_justify_content(value: StileJustifyContent) -> JustifyContent {
    match value {
        StileJustifyContent::Start => JustifyContent::Start,
        StileJustifyContent::Center => JustifyContent::Center,
        StileJustifyContent::End => JustifyContent::End,
        StileJustifyContent::SpaceBetween => JustifyContent::SpaceBetween,
    }
}

fn map_align_items(value: StileAlignItems) -> AlignItems {
    match value {
        StileAlignItems::Start => AlignItems::Start,
        StileAlignItems::Center => AlignItems::Center,
        StileAlignItems::End => AlignItems::End,
        StileAlignItems::Stretch => AlignItems::Stretch,
    }
}

#[cfg(test)]
mod tests {
    use stile::{AlignItems, Color, Declaration, FlexDirection, JustifyContent, Overflow, Selector};

    use super::*;

    #[test]
    fn style_resolves_and_layout_computes() {
        let mut tree = UiTree::new().unwrap();
        let button = tree.pressable("Delete").unwrap();
        let label = tree.text("Delete").unwrap();
        tree.add_child(tree.root(), button).unwrap();
        tree.add_child(button, label).unwrap();

        let rules = alloc::vec![
            Rule::new(
                Selector::has(Description::Container),
                alloc::vec![
                    Declaration::FlexDirection(FlexDirection::Column),
                    Declaration::AlignItems(AlignItems::Start),
                ],
            ),
            Rule::new(
                Selector::has(Description::Pressable),
                alloc::vec![
                    Declaration::BackgroundColor(Color::rgb(0xb5, 0x89, 0x00)),
                    Declaration::Padding(8.0),
                    Declaration::BorderWidth(2.0),
                    Declaration::FlexDirection(FlexDirection::Row),
                    Declaration::JustifyContent(JustifyContent::Center),
                    Declaration::MinWidth(120.0),
                    Declaration::MinHeight(40.0),
                ],
            ),
            Rule::new(
                Selector::has(Description::Textual),
                alloc::vec![Declaration::FontSize(14.0)],
            ),
        ];

        tree.restyle(&rules).unwrap();
        tree.compute_layout(Size::MAX_CONTENT).unwrap();

        let root_box = tree.layout_box(tree.root()).unwrap();
        let button_box = tree.layout_box(button).unwrap();
        assert!(root_box.width >= button_box.width);
        assert_eq!(button_box.width, 120.0);
        assert_eq!(button_box.height, 40.0);
    }

    #[test]
    fn text_intrinsic_size_expands_parent_past_minimum() {
        let mut tree = UiTree::new().unwrap();
        let button = tree.pressable("Launch very long application").unwrap();
        let label = tree.text("Launch very long application").unwrap();
        tree.add_child(tree.root(), button).unwrap();
        tree.add_child(button, label).unwrap();

        let rules = alloc::vec![
            Rule::new(
                Selector::has(Description::Container),
                alloc::vec![
                    Declaration::FlexDirection(FlexDirection::Column),
                    Declaration::AlignItems(AlignItems::Start),
                ],
            ),
            Rule::new(
                Selector::has(Description::Pressable),
                alloc::vec![
                    Declaration::Padding(8.0),
                    Declaration::FlexDirection(FlexDirection::Row),
                    Declaration::MinWidth(40.0),
                    Declaration::MinHeight(24.0),
                ],
            ),
            Rule::new(
                Selector::has(Description::Textual),
                alloc::vec![Declaration::FontSize(14.0)],
            ),
        ];

        tree.restyle(&rules).unwrap();
        tree.compute_layout(Size::MAX_CONTENT).unwrap();

        let label_box = tree.layout_box(label).unwrap();
        let button_box = tree.layout_box(button).unwrap();
        assert!(label_box.width > 40.0);
        assert!(button_box.width > label_box.width);
        assert!(button_box.width > 40.0);
    }

    #[test]
    fn content_fit_layout_prefers_content_then_clamps_to_bounds() {
        let mut tree = UiTree::new().unwrap();
        let label = tree.text("A very wide label").unwrap();
        tree.add_child(tree.root(), label).unwrap();

        let rules = alloc::vec![
            Rule::new(
                Selector::has(Description::Container),
                alloc::vec![Declaration::AlignItems(AlignItems::Start)],
            ),
            Rule::new(
                Selector::has(Description::Textual),
                alloc::vec![Declaration::FontSize(20.0)],
            ),
        ];

        tree.restyle(&rules).unwrap();
        let fitted = tree.compute_content_fit_layout(80.0, 200.0).unwrap();
        assert_eq!(fitted.width, 80.0);
        assert_eq!(tree.layout_box(tree.root()).unwrap().width, 80.0);
    }

    #[test]
    fn container_defaults_to_clip_overflow_when_no_policy_set() {
        let tree = UiTree::new().unwrap();
        // Root node is a Container; it should report Clip when no overflow is declared.
        assert_eq!(tree.overflow_intent(tree.root()), Some(Overflow::Clip));
    }

    #[test]
    fn container_reports_declared_overflow_policy() {
        let mut tree = UiTree::new().unwrap();
        let rules = alloc::vec![Rule::new(
            Selector::has(Description::Container),
            alloc::vec![Declaration::Overflow(Overflow::Scroll)],
        )];
        tree.restyle(&rules).unwrap();
        assert_eq!(tree.overflow_intent(tree.root()), Some(Overflow::Scroll));
    }

    #[test]
    fn non_container_node_has_no_overflow_intent() {
        let mut tree = UiTree::new().unwrap();
        let label = tree.text("hello").unwrap();
        tree.add_child(tree.root(), label).unwrap();
        // Textual nodes carry no overflow intent.
        assert_eq!(tree.overflow_intent(label), None);
    }

    #[test]
    fn scroll_container_factory_creates_scroll_node() {
        let mut tree = UiTree::new().unwrap();
        let sc = tree.scroll_container().unwrap();
        // The factory node must report Scroll.
        assert_eq!(tree.overflow_intent(sc), Some(Overflow::Scroll));
        // It must carry the ScrollContainer description.
        let node = tree.node(sc).unwrap();
        assert!(node.descriptions.contains(&Description::ScrollContainer));
    }

    #[test]
    fn overflow_wrap_enables_flex_wrap_in_taffy_style() {
        let style = ResolvedStyle { overflow: Some(Overflow::Wrap), ..ResolvedStyle::default() };
        let taffy_style = apply_style_to_taffy(&style);
        assert_eq!(taffy_style.flex_wrap, FlexWrap::Wrap);
    }

    #[test]
    fn overflow_clip_and_scroll_do_not_enable_flex_wrap() {
        for overflow in [Overflow::Clip, Overflow::Scroll] {
            let style = ResolvedStyle { overflow: Some(overflow), ..ResolvedStyle::default() };
            let taffy_style = apply_style_to_taffy(&style);
            assert_eq!(taffy_style.flex_wrap, FlexWrap::NoWrap);
        }
    /// A `list_row_pressable` should expand to fill the full container width
    /// when the parent container uses `align_items: Stretch`.
    ///
    /// In a `flex-direction: column` parent, the cross axis is horizontal.
    /// `align_items: Stretch` on the parent makes each child fill the full
    /// container width — that is the intended mechanism for full-row hit areas.
    /// (`flex_grow` controls expansion along the main axis, which is vertical
    /// in a column layout, and does not affect width.)
    #[test]
    fn list_row_pressable_fills_container_width() {
        let container_width = 360.0_f32;
        let mut tree = UiTree::new().unwrap();
        let row = tree.list_row_pressable("Settings").unwrap();
        tree.add_child(tree.root(), row).unwrap();

        let rules = alloc::vec![Rule::new(
            Selector::has(Description::ListRow),
            alloc::vec![Declaration::Height(44.0)],
        )];
        tree.restyle(&rules).unwrap();

        // Parent container with explicit width and align_items: Stretch so
        // that list row children fill the full cross-axis (width).
        tree.apply_style(
            tree.root(),
            ResolvedStyle {
                width: Some(container_width),
                height: Some(200.0),
                flex_direction: Some(FlexDirection::Column),
                align_items: Some(AlignItems::Stretch),
                ..ResolvedStyle::default()
            },
        )
        .unwrap();

        tree.compute_layout(Size {
            width: AvailableSpace::Definite(container_width),
            height: AvailableSpace::Definite(200.0),
        })
        .unwrap();

        let row_box = tree.global_layout_box(row).unwrap();
        assert_eq!(
            row_box.width, container_width,
            "list row should span the full container width via parent align_items: Stretch"
        );
        assert_eq!(row_box.height, 44.0, "row height should respect the explicit Height rule");
        // Verify it has the correct descriptions.
        let node = tree.node(row).unwrap();
        assert!(node.descriptions.contains(&Description::Pressable));
        assert!(node.descriptions.contains(&Description::ListRow));
    }

    /// Explicit `Width` and `Height` rules on a `ListRow` node are respected
    /// even when the parent uses `align_items: Start` (no cross-axis stretch).
    #[test]
    fn list_row_pressable_explicit_dimensions_are_respected() {
        let container_width = 300.0_f32;
        let mut tree = UiTree::new().unwrap();
        let row = tree.list_row_pressable("Item").unwrap();
        tree.add_child(tree.root(), row).unwrap();

        let rules = alloc::vec![Rule::new(
            Selector::has(Description::ListRow),
            alloc::vec![
                Declaration::Width(120.0),
                Declaration::Height(44.0),
            ],
        )];
        tree.restyle(&rules).unwrap();
        tree.apply_style(
            tree.root(),
            ResolvedStyle {
                width: Some(container_width),
                height: Some(200.0),
                flex_direction: Some(FlexDirection::Column),
                align_items: Some(AlignItems::Start),
                ..ResolvedStyle::default()
            },
        )
        .unwrap();
        tree.compute_layout(Size {
            width: AvailableSpace::Definite(container_width),
            height: AvailableSpace::Definite(200.0),
        })
        .unwrap();

        let row_box = tree.global_layout_box(row).unwrap();
        assert_eq!(row_box.width, 120.0, "explicit width from rule should be respected");
        assert_eq!(row_box.height, 44.0, "explicit height from rule should be respected");
    }
}
