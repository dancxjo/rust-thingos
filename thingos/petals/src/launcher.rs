use alloc::string::{String, ToString};
use alloc::vec::Vec;

use taffy::TaffyError;

use crate::{
    AlignItems, AttrValue, Color, Declaration, Description, FlexDirection, FontWeight,
    JustifyContent, NodeId, PetalsEvent, PetalsService, Rule, Selector, ServiceAction, UiTree,
};
use stile::typography::{
    SCALE_CAPTION, SCALE_H2, SCALE_H3, SCALE_LABEL, SPACE_LG, SPACE_MD, SPACE_SM,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApplicationEntry {
    pub name: String,
    pub path: String,
    pub glyph: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ApplicationTileNode {
    pub node: NodeId,
    pub icon: NodeId,
    pub label: NodeId,
    pub path: NodeId,
    pub entry_index: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApplicationLauncherNodes {
    pub root: NodeId,
    pub title: NodeId,
    pub grid: NodeId,
    pub rows: Vec<NodeId>,
    pub tiles: Vec<ApplicationTileNode>,
    pub status: NodeId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApplicationLauncher {
    pub entries: Vec<ApplicationEntry>,
    pub status: String,
}

impl ApplicationLauncher {
    pub fn new(entries: Vec<ApplicationEntry>) -> Self {
        let status = if entries.is_empty() {
            String::from("No applications found")
        } else {
            alloc::format!("{} applications", entries.len())
        };
        Self { entries, status }
    }

    pub fn with_status(mut self, status: &str) -> Self {
        self.status = status.to_string();
        self
    }

    pub fn build_tree(&self) -> Result<(UiTree, ApplicationLauncherNodes), TaffyError> {
        let mut tree = UiTree::new()?;
        let root = tree.root();
        let nodes = self.render(&mut tree, root)?;
        tree.restyle(&self.rules())?;
        Ok((tree, nodes))
    }

    pub fn render(
        &self,
        tree: &mut UiTree,
        parent: NodeId,
    ) -> Result<ApplicationLauncherNodes, TaffyError> {
        let root = tree.add_node(&[
            Description::Perspective,
            Description::ApplicationLauncher,
            Description::Container,
            Description::Focusable,
        ])?;
        let title = tree.text("Applications")?;
        let grid = tree.add_node(&[Description::ApplicationGrid, Description::Container])?;
        let status = tree.text(&self.status)?;

        if let Some(node) = tree.node_mut(title) {
            node.descriptions.push(Description::ApplicationTitle);
        }
        if let Some(node) = tree.node_mut(status) {
            node.descriptions.push(Description::ApplicationStatus);
        }

        tree.add_child(parent, root)?;
        tree.add_child(root, title)?;
        tree.add_child(root, grid)?;

        let mut rows = Vec::new();
        let mut tiles = Vec::new();
        let mut row_node: Option<NodeId> = None;
        for (entry_index, entry) in self.entries.iter().enumerate() {
            if entry_index % 4 == 0 {
                let row = tree.add_node(&[Description::ApplicationRow, Description::Container])?;
                tree.add_child(grid, row)?;
                rows.push(row);
                row_node = Some(row);
            }
            let node = tree.pressable(&entry.name)?;
            let icon = tree.logogram(&entry.glyph)?;
            let label = tree.text(&entry.name)?;
            let path = tree.text(&entry.path)?;

            if let Some(item) = tree.node_mut(node) {
                item.descriptions.push(Description::ApplicationTile);
                item.attrs.insert(String::from("app_name"), AttrValue::Str(entry.name.clone()));
                item.attrs.insert(String::from("app_path"), AttrValue::Str(entry.path.clone()));
                item.attrs
                    .insert(String::from("entry_index"), AttrValue::Number(entry_index as f64));
            }
            if let Some(item) = tree.node_mut(label) {
                item.descriptions.push(Description::ApplicationName);
            }
            if let Some(item) = tree.node_mut(path) {
                item.descriptions.push(Description::ApplicationPath);
            }

            tree.add_child(node, icon)?;
            tree.add_child(node, label)?;
            tree.add_child(node, path)?;
            tree.add_child(row_node.unwrap_or(grid), node)?;
            tiles.push(ApplicationTileNode { node, icon, label, path, entry_index });
        }

        tree.add_child(root, status)?;
        Ok(ApplicationLauncherNodes { root, title, grid, rows, tiles, status })
    }

    pub fn rules(&self) -> Vec<Rule<Description>> {
        alloc::vec![
            Rule::new(
                Selector::has(Description::ApplicationLauncher),
                alloc::vec![
                    Declaration::FlexDirection(FlexDirection::Column),
                    Declaration::AlignItems(AlignItems::Stretch),
                    Declaration::JustifyContent(JustifyContent::Start),
                    Declaration::Gap(SPACE_MD),
                    Declaration::Padding(SPACE_LG),
                    Declaration::BackgroundColor(Color::rgb(0x16, 0x18, 0x1d)),
                ],
            ),
            Rule::new(
                Selector::has(Description::ApplicationTitle),
                alloc::vec![
                    Declaration::Height(24.0),
                    Declaration::FontSize(SCALE_H2),
                    Declaration::FontWeight(FontWeight::Bold),
                    Declaration::Color(Color::rgb(0xf1, 0xf4, 0xf8)),
                ],
            ),
            Rule::new(
                Selector::has(Description::ApplicationGrid),
                alloc::vec![
                    Declaration::FlexDirection(FlexDirection::Column),
                    Declaration::AlignItems(AlignItems::Start),
                    Declaration::JustifyContent(JustifyContent::Start),
                    Declaration::Gap(SPACE_SM),
                ],
            ),
            Rule::new(
                Selector::has(Description::ApplicationRow),
                alloc::vec![
                    Declaration::FlexDirection(FlexDirection::Row),
                    Declaration::AlignItems(AlignItems::Start),
                    Declaration::JustifyContent(JustifyContent::Start),
                    Declaration::Gap(SPACE_SM),
                ],
            ),
            Rule::new(
                Selector::has(Description::ApplicationTile),
                alloc::vec![
                    Declaration::FlexDirection(FlexDirection::Column),
                    Declaration::AlignItems(AlignItems::Center),
                    Declaration::JustifyContent(JustifyContent::Center),
                    Declaration::Gap(SPACE_SM),
                    Declaration::Padding(SPACE_SM),
                    Declaration::Width(128.0),
                    Declaration::Height(96.0),
                    Declaration::BackgroundColor(Color::rgb(0x24, 0x29, 0x33)),
                    Declaration::BorderWidth(1.0),
                ],
            ),
            // Hover: subtle tint -- lowest interactive priority
            Rule::new(
                Selector::has(Description::ApplicationTile)
                    .and(Selector::state(crate::State::Hover)),
                alloc::vec![Declaration::BackgroundColor(Color::rgb(0x2A, 0x30, 0x3C))],
            ),
            // Focus: visible outline ring for keyboard navigation -- medium priority
            Rule::new(
                Selector::has(Description::ApplicationTile)
                    .and(Selector::state(crate::State::Focus)),
                alloc::vec![
                    Declaration::OutlineColor(Color::rgb(0x8A, 0xD7, 0xFF)),
                    Declaration::OutlineWidth(2.0),
                ],
            ),
            // Active: dominant fill -- highest interactive priority (placed last to win cascade)
            Rule::new(
                Selector::has(Description::ApplicationTile)
                    .and(Selector::state(crate::State::Active)),
                alloc::vec![Declaration::BackgroundColor(Color::rgb(0x38, 0x46, 0x54))],
            ),
            Rule::new(
                Selector::has(Description::Logogram),
                alloc::vec![
                    Declaration::Width(34.0),
                    Declaration::Height(34.0),
                    Declaration::Color(Color::rgb(0x8f, 0xd1, 0xff)),
                ],
            ),
            Rule::new(
                Selector::has(Description::ApplicationName),
                alloc::vec![
                    Declaration::Width(108.0),
                    Declaration::Height(20.0),
                    Declaration::FontSize(SCALE_H3),
                    Declaration::FontWeight(FontWeight::Bold),
                    Declaration::Color(Color::rgb(0xf1, 0xf4, 0xf8)),
                ],
            ),
            Rule::new(
                Selector::has(Description::ApplicationPath),
                alloc::vec![
                    Declaration::Width(108.0),
                    Declaration::Height(SPACE_LG),
                    Declaration::FontSize(SCALE_CAPTION),
                    Declaration::Color(Color::rgb(0x9d, 0xa8, 0xb7)),
                ],
            ),
            Rule::new(
                Selector::has(Description::ApplicationStatus),
                alloc::vec![
                    Declaration::Width(360.0),
                    Declaration::Height(SPACE_LG),
                    Declaration::FontSize(SCALE_LABEL),
                    Declaration::Color(Color::rgb(0xad, 0xb7, 0xc7)),
                ],
            ),
        ]
    }
}

pub fn glyph_for_application(name: &str) -> &'static str {
    match name {
        "calc" | "calculator" => "calculator",
        "clock" | "date" => "list-todo",
        "themes" => "layers",
        "leaf" | "sh" => "monitor",
        "httpsd" | "netd" | "ping" | "nslookup" => "blocks",
        _ => "box",
    }
}

pub fn display_name_from_path(path: &str) -> String {
    let name = path.rsplit('/').next().unwrap_or(path);
    let mut out = String::new();
    let mut capitalize = true;
    for ch in name.chars() {
        if ch == '_' || ch == '-' {
            out.push(' ');
            capitalize = true;
        } else if capitalize {
            for upper in ch.to_uppercase() {
                out.push(upper);
            }
            capitalize = false;
        } else {
            out.push(ch);
        }
    }
    out
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ApplicationLauncherService;

impl PetalsService for ApplicationLauncherService {
    fn dispatch(&mut self, tree: &mut UiTree, event: PetalsEvent) -> ServiceAction {
        match event {
            PetalsEvent::SetState { node, state, enabled } => {
                tree.set_state(node, state, enabled);
                ServiceAction::Continue
            }
            PetalsEvent::Shutdown => ServiceAction::Shutdown,
            PetalsEvent::Restyle | PetalsEvent::Layout => ServiceAction::Continue,
        }
    }
}

#[cfg(test)]
mod tests {
    use taffy::prelude::{AvailableSpace, Size};

    use super::*;
    use crate::ResolvedStyle;

    #[test]
    fn builds_launcher_tiles_from_application_entries() {
        let launcher = ApplicationLauncher::new(alloc::vec![
            ApplicationEntry {
                name: String::from("Calculator"),
                path: String::from("/applications/calc"),
                glyph: String::from("calculator"),
            },
            ApplicationEntry {
                name: String::from("Clock"),
                path: String::from("/applications/clock"),
                glyph: String::from("clock"),
            },
        ]);
        let (mut tree, nodes) = launcher.build_tree().unwrap();

        tree.apply_style(
            tree.root(),
            ResolvedStyle {
                width: Some(480.0),
                height: Some(320.0),
                flex_direction: Some(FlexDirection::Column),
                justify_content: Some(JustifyContent::Start),
                align_items: Some(AlignItems::Stretch),
                ..ResolvedStyle::default()
            },
        )
        .unwrap();
        tree.compute_layout(Size {
            width: AvailableSpace::Definite(480.0),
            height: AvailableSpace::Definite(320.0),
        })
        .unwrap();

        assert_eq!(nodes.tiles.len(), 2);
        assert!(
            tree.node(nodes.tiles[0].node).unwrap().descriptions.contains(&Description::Pressable)
        );
        assert!(
            tree.node(nodes.tiles[0].icon).unwrap().descriptions.contains(&Description::Logogram)
        );
        assert_eq!(tree.global_layout_box(nodes.tiles[0].node).unwrap().width, 128.0);
    }

    #[test]
    fn section_title_uses_application_title_description_and_is_larger_than_tile_labels() {
        use stile::typography::{SCALE_H2, SCALE_H3};
        let launcher = ApplicationLauncher::new(alloc::vec![ApplicationEntry {
            name: String::from("Clock"),
            path: String::from("/applications/clock"),
            glyph: String::from("clock"),
        }]);
        let (tree, nodes) = launcher.build_tree().unwrap();

        // The top-level "Applications" header must carry ApplicationTitle, not ApplicationName.
        assert!(
            tree.node(nodes.title).unwrap().descriptions.contains(&Description::ApplicationTitle)
        );
        assert!(
            !tree.node(nodes.title).unwrap().descriptions.contains(&Description::ApplicationName)
        );

        // Section header must render at SCALE_H2; tile label must render at SCALE_H3.
        let title_size = tree.node(nodes.title).unwrap().style.font_size;
        let label_size = tree.node(nodes.tiles[0].label).unwrap().style.font_size;
        assert_eq!(title_size, Some(SCALE_H2));
        assert_eq!(label_size, Some(SCALE_H3));
        assert!(title_size.unwrap() > label_size.unwrap());
    }

    #[test]
    fn formats_names_and_chooses_stable_glyphs() {
        assert_eq!(display_name_from_path("/applications/wayland_hello"), "Wayland Hello");
        assert_eq!(glyph_for_application("calc"), "calculator");
        assert_eq!(glyph_for_application("unknown"), "box");
    }
}
