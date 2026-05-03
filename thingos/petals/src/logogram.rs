use alloc::format;
use alloc::string::{String, ToString};

use taffy::TaffyError;

use crate::{AttrValue, Description, NodeId, UiTree};

pub const LUCIDE_ROOT: &str = "/public/icons/lucide";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Logogram {
    glyph: String,
}

impl Logogram {
    pub fn new(glyph: &str) -> Self {
        Self { glyph: glyph.to_string() }
    }

    pub fn glyph(&self) -> &str {
        &self.glyph
    }

    pub fn icon_path(&self) -> String {
        lucide_icon_path(&self.glyph)
    }

    pub fn render(&self, tree: &mut UiTree, parent: NodeId) -> Result<NodeId, TaffyError> {
        let node = tree.logogram(&self.glyph)?;
        tree.add_child(parent, node)?;
        Ok(node)
    }
}

pub fn lucide_icon_path(glyph: &str) -> String {
    format!("{}/{}.svg", LUCIDE_ROOT, glyph)
}

pub fn node_glyph(tree: &UiTree, node: NodeId) -> Option<&str> {
    let node = tree.node(node)?;
    match node.attrs.get("glyph") {
        Some(AttrValue::Str(value)) => Some(value.as_str()),
        _ => None,
    }
}

pub fn node_icon_path(tree: &UiTree, node: NodeId) -> Option<&str> {
    let node = tree.node(node)?;
    match node.attrs.get("icon_path") {
        Some(AttrValue::Str(value)) => Some(value.as_str()),
        _ => None,
    }
}

impl UiTree {
    pub fn logogram(&mut self, glyph: &str) -> Result<NodeId, TaffyError> {
        let id = self.add_node(&[Description::Logogram])?;
        if let Some(node) = self.node_mut(id) {
            node.attrs.insert(String::from("glyph"), AttrValue::Str(glyph.into()));
            node.attrs.insert(String::from("icon_path"), AttrValue::Str(lucide_icon_path(glyph)));
        }
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logogram_records_named_lucide_glyph() {
        let mut tree = UiTree::new().unwrap();
        let root = tree.root();
        let node = Logogram::new("calculator").render(&mut tree, root).unwrap();

        assert!(tree.node(node).unwrap().descriptions.contains(&Description::Logogram));
        assert_eq!(node_glyph(&tree, node), Some("calculator"));
        assert_eq!(node_icon_path(&tree, node), Some("/public/icons/lucide/calculator.svg"));
    }
}
