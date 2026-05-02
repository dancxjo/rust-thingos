use alloc::vec::Vec;

use stile::{
    AlignItems as StileAlignItems, FlexDirection as StileFlexDirection,
    JustifyContent as StileJustifyContent, ResolvedStyle, Rule,
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
    taffy: TaffyTree<()>,
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
        let root_layout = self.nodes[self.root as usize].layout;
        self.taffy.compute_layout(root_layout, available_space)
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

    fn parent_of(&self, id: NodeId) -> Option<NodeId> {
        self.nodes
            .iter()
            .find(|node| node.children.iter().any(|child| *child == id))
            .map(|node| node.id)
    }
}

pub fn apply_style_to_taffy(style: &ResolvedStyle) -> Style {
    let length = |value: f32| LengthPercentage::from_length(value);
    let length_auto = |value: f32| LengthPercentageAuto::from_length(value);

    Style {
        display: Display::Flex,
        size: Size {
            width: style.width.map(Dimension::from_length).unwrap_or(Dimension::AUTO),
            height: style.height.map(Dimension::from_length).unwrap_or(Dimension::AUTO),
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
    if descriptions.contains(&Description::Textual) {
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
    use stile::{AlignItems, Color, Declaration, FlexDirection, JustifyContent, Selector};

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
                    Declaration::Width(120.0),
                    Declaration::Height(40.0),
                ],
            ),
            Rule::new(
                Selector::has(Description::Textual),
                alloc::vec![
                    Declaration::FontSize(14.0),
                    Declaration::Width(64.0),
                    Declaration::Height(16.0)
                ],
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
}
