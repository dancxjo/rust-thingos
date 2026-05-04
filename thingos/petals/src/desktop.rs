//! Desktop layout and theme-derived wallpaper selection.
//!
//! The `Desktop` node is the root of the desktop scene tree.  Its child
//! `Wallpaper` node carries a `"path"` attr whose value is the image file the
//! compositor should display as the desktop background.  The path is driven
//! entirely by the active `Theme`, making the wallpaper a first-class part of
//! the theme alongside chrome colours and widget styling.
//!
//! # Usage
//!
//! ```
//! // Resolve the wallpaper path for the active theme:
//! let path = petals::wallpaper_path_for_theme(petals::default_theme());
//!
//! // Or build a full desktop UiTree for layout-aware rendering:
//! let (tree, nodes) = petals::desktop_tree_for_theme(petals::default_theme()).unwrap();
//! ```

use alloc::vec::Vec;

use stile::Theme;
use stile::values::{AlignItems, Color, FlexDirection, JustifyContent};
use taffy::TaffyError;

use crate::description::Description;
use crate::layout::UiTree;
use crate::node::{AttrValue, NodeId};
use crate::{Declaration, Rule};

/// Node handles for the desktop scene tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DesktopNodes {
    /// Root `Desktop` container node.
    pub root: NodeId,
    /// Full-screen `Wallpaper` node.  Its `"path"` attr holds the image path.
    pub wallpaper: NodeId,
}

const fn color_from_argb(argb: u32) -> Color {
    Color::rgba(
        ((argb >> 16) & 0xFF) as u8,
        ((argb >> 8) & 0xFF) as u8,
        (argb & 0xFF) as u8,
        ((argb >> 24) & 0xFF) as u8,
    )
}

const FALLBACK_WALLPAPER_PATH: &str = "/public/wallpapers/flower.bmp";

/// Return the wallpaper image path for `theme`.
///
/// This is the canonical source of truth for the per-theme default wallpaper.
/// Callers that need only the path and not the full `UiTree` should use this
/// function in preference to constructing a tree and reading an attr.
pub fn wallpaper_path_for_theme(theme: Theme) -> &'static str {
    theme.wallpaper_path.unwrap_or(FALLBACK_WALLPAPER_PATH)
}

/// Return styling rules for the desktop scene expressed against `theme`.
///
/// The rules define layout and colour for `Desktop` and `Wallpaper` nodes.
/// They are applied via `stile::compute_for` exactly like chrome/calc rules.
pub fn desktop_rules_for_theme(theme: Theme) -> Vec<Rule<Description>> {
    let bg = color_from_argb(theme.body_top);
    alloc::vec![
        Rule::new(
            stile::Selector::has(Description::Desktop),
            alloc::vec![
                Declaration::FlexDirection(FlexDirection::Column),
                Declaration::AlignItems(AlignItems::Stretch),
                Declaration::JustifyContent(JustifyContent::Start),
                Declaration::BackgroundColor(bg),
            ],
        ),
        Rule::new(
            stile::Selector::has(Description::Wallpaper),
            alloc::vec![
                Declaration::FlexDirection(FlexDirection::Column),
                Declaration::AlignItems(AlignItems::Stretch),
                Declaration::JustifyContent(JustifyContent::Start),
                Declaration::BackgroundColor(bg),
            ],
        ),
    ]
}

/// Build a `UiTree` containing a `Desktop` root node with a single `Wallpaper`
/// child.  The `Wallpaper` node carries a `"path"` attr set to
/// `theme.wallpaper_path`.
///
/// This tree is the petals representation of the desktop background.  The
/// compositor (blossom) inspects the `"path"` attr to determine which image
/// file to decode and display.
pub fn desktop_tree_for_theme(theme: Theme) -> Result<(UiTree, DesktopNodes), TaffyError> {
    let mut tree = UiTree::new()?;
    let root_id = tree.root();

    let desktop = tree.add_node(&[Description::Desktop, Description::Container])?;
    let wallpaper = tree.add_node(&[Description::Wallpaper])?;

    if let Some(node) = tree.node_mut(wallpaper) {
        node.attrs.insert(
            alloc::string::String::from("path"),
            AttrValue::Str(alloc::string::String::from(wallpaper_path_for_theme(theme))),
        );
    }

    tree.add_child(root_id, desktop)?;
    tree.add_child(desktop, wallpaper)?;

    Ok((tree, DesktopNodes { root: desktop, wallpaper }))
}

#[cfg(test)]
mod tests {
    use stile::{AURORA_GLASS, OBSIDIAN_BLOOM, SOLARIS_WARM, available_themes};

    use super::*;
    use crate::node::AttrValue;

    #[test]
    fn wallpaper_path_for_theme_returns_per_theme_path() {
        assert_eq!(wallpaper_path_for_theme(SOLARIS_WARM), "/public/wallpapers/flower.bmp");
        assert_eq!(wallpaper_path_for_theme(OBSIDIAN_BLOOM), "/public/wallpapers/flower.png");
        assert_eq!(wallpaper_path_for_theme(AURORA_GLASS), "/public/wallpapers/clouds.bmp");
    }

    #[test]
    fn desktop_tree_wallpaper_attr_matches_theme() {
        for theme in available_themes() {
            let (tree, nodes) = desktop_tree_for_theme(*theme).expect("tree build failed");
            let wallpaper_node = tree.node(nodes.wallpaper).expect("wallpaper node missing");
            let path_attr = wallpaper_node.attrs.get("path").expect("missing path attr");
            assert_eq!(
                *path_attr,
                AttrValue::Str(alloc::string::String::from(wallpaper_path_for_theme(*theme))),
                "wrong path attr for theme {}",
                theme.name
            );
        }
    }

    #[test]
    fn desktop_tree_structure() {
        let (tree, nodes) = desktop_tree_for_theme(SOLARIS_WARM).unwrap();
        let desktop = tree.node(nodes.root).unwrap();
        assert!(desktop.descriptions.contains(&Description::Desktop));
        assert!(desktop.children.contains(&nodes.wallpaper));
        let wallpaper = tree.node(nodes.wallpaper).unwrap();
        assert!(wallpaper.descriptions.contains(&Description::Wallpaper));
    }

    #[test]
    fn desktop_rules_cover_desktop_and_wallpaper() {
        let rules = desktop_rules_for_theme(SOLARIS_WARM);
        assert_eq!(rules.len(), 2);
    }
}
