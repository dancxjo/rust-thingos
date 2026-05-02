use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use stile::{ResolvedStyle, StateSet, Stylable, StylableSurface, StyleProperty};

use crate::description::Description;

pub type NodeId = u32;
pub type Attrs = BTreeMap<String, AttrValue>;

#[derive(Clone, Debug, PartialEq)]
pub enum AttrValue {
    Str(String),
    Number(f64),
    Bool(bool),
}

#[derive(Clone, Debug)]
pub struct Node {
    pub id: NodeId,
    pub descriptions: Vec<Description>,
    pub states: StateSet,
    pub attrs: Attrs,
    pub stylable: Stylable,
    pub children: Vec<NodeId>,
    pub style: ResolvedStyle,
    pub layout: taffy::prelude::NodeId,
}

impl Node {
    pub fn new(id: NodeId, layout: taffy::prelude::NodeId) -> Self {
        Self {
            id,
            descriptions: Vec::new(),
            states: StateSet::empty(),
            attrs: Attrs::new(),
            stylable: Stylable::default(),
            children: Vec::new(),
            style: ResolvedStyle::default(),
            layout,
        }
    }

    pub fn with_description(mut self, description: Description) -> Self {
        self.descriptions.push(description);
        self
    }

    pub fn with_attr(mut self, key: &str, value: AttrValue) -> Self {
        self.attrs.insert(String::from(key), value);
        self
    }

    pub fn supports(&self, property: StyleProperty) -> bool {
        self.stylable.supports(property)
    }
}

impl StylableSurface<Description> for Node {
    fn descriptions(&self) -> &[Description] {
        &self.descriptions
    }

    fn states(&self) -> StateSet {
        self.states
    }

    fn supports_property(&self, property: StyleProperty) -> bool {
        self.supports(property)
    }
}
