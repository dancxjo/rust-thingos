//! Application launcher overlay state and Petals structure.

use alloc::string::String;
use alloc::vec::Vec;

use petals::{ApplicationEntry, ApplicationLauncher, ApplicationLauncherNodes, ResolvedStyle};

use crate::layer_shell::{
    LayerKeyboardInteractivity, LayerShellLayer, LayerSurfaceConfig, compute_layer_placement,
};

pub const LAUNCHER_WIDTH: u32 = 600;
pub const LAUNCHER_HEIGHT: u32 = 430;
const LAUNCHER_VERTICAL_OFFSET: i32 = 12;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LauncherState {
    pub entries: Vec<ApplicationEntry>,
    pub visible: bool,
    pub pressed_index: Option<usize>,
    pub status: String,
}

impl LauncherState {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            visible: false,
            pressed_index: None,
            status: String::from("No applications found"),
        }
    }

    pub fn open(&mut self, entries: Vec<ApplicationEntry>) {
        self.entries = entries;
        self.visible = true;
        self.pressed_index = None;
        self.status = if self.entries.is_empty() {
            String::from("No applications found")
        } else {
            alloc::format!("{} applications", self.entries.len())
        };
    }

    pub fn close(&mut self) {
        self.visible = false;
        self.pressed_index = None;
    }

    pub fn toggle(&mut self, entries: Vec<ApplicationEntry>) -> bool {
        if self.visible {
            self.close();
            false
        } else {
            self.open(entries);
            true
        }
    }

    pub fn view(&self) -> ApplicationLauncher {
        ApplicationLauncher::new(self.entries.clone()).with_status(&self.status)
    }

    pub fn press_at(&mut self, output_w: u32, output_h: u32, x: i32, y: i32) -> bool {
        if !self.visible {
            return false;
        }
        let Some(index) = self.hit_test(output_w, output_h, x, y) else {
            self.pressed_index = None;
            return false;
        };
        self.pressed_index = Some(index);
        true
    }

    pub fn release_at(&mut self, output_w: u32, output_h: u32, x: i32, y: i32) -> Option<String> {
        if !self.visible {
            return None;
        }
        let pressed = self.pressed_index.take()?;
        let released = self.hit_test(output_w, output_h, x, y)?;
        if pressed != released {
            return None;
        }
        self.entries.get(released).map(|entry| entry.path.clone())
    }

    pub fn hit_test(&self, output_w: u32, output_h: u32, x: i32, y: i32) -> Option<usize> {
        let placement = launcher_rect(output_w, output_h);
        if x < placement.x
            || y < placement.y
            || x >= placement.x + placement.width as i32
            || y >= placement.y + placement.height as i32
        {
            return None;
        }

        let (mut tree, nodes) = self.view().build_tree().ok()?;
        apply_root_size(&mut tree, LAUNCHER_WIDTH, LAUNCHER_HEIGHT).ok()?;
        tree.compute_layout(petals::Size {
            width: petals::AvailableSpace::Definite(LAUNCHER_WIDTH as f32),
            height: petals::AvailableSpace::Definite(LAUNCHER_HEIGHT as f32),
        })
        .ok()?;

        let local_x = x - placement.x;
        let local_y = y - placement.y;
        for tile in nodes.tiles {
            let b = tree.global_layout_box(tile.node).ok()?;
            if local_x >= b.x as i32
                && local_y >= b.y as i32
                && local_x < (b.x + b.width) as i32
                && local_y < (b.y + b.height) as i32
            {
                return Some(tile.entry_index);
            }
        }
        None
    }
}

impl Default for LauncherState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn launcher_layer_config() -> LayerSurfaceConfig {
    let mut cfg = LayerSurfaceConfig::new(LayerShellLayer::Overlay);
    cfg.anchor = 0;
    cfg.exclusive_zone = -1;
    cfg.keyboard_interactivity = LayerKeyboardInteractivity::Exclusive;
    cfg.size = (LAUNCHER_WIDTH, LAUNCHER_HEIGHT);
    cfg.margin_top = LAUNCHER_VERTICAL_OFFSET;
    cfg
}

pub fn launcher_rect(output_w: u32, output_h: u32) -> crate::LayerSurfacePlacement {
    compute_layer_placement(output_w, output_h, &launcher_layer_config())
}

pub fn launcher_tree(
    state: &LauncherState,
) -> Result<(petals::UiTree, ApplicationLauncherNodes), petals::TaffyError> {
    let view = state.view();
    let mut tree = petals::UiTree::new()?;
    let root = tree.root();
    let nodes = view.render(&mut tree, root)?;
    if let Some(pressed_index) = state.pressed_index {
        if let Some(tile) = nodes.tiles.iter().find(|tile| tile.entry_index == pressed_index) {
            tree.set_state(tile.node, petals::State::Active, true);
        }
    }
    tree.restyle(&view.rules())?;
    apply_root_size(&mut tree, LAUNCHER_WIDTH, LAUNCHER_HEIGHT)?;
    tree.compute_layout(petals::Size {
        width: petals::AvailableSpace::Definite(LAUNCHER_WIDTH as f32),
        height: petals::AvailableSpace::Definite(LAUNCHER_HEIGHT as f32),
    })?;
    Ok((tree, nodes))
}

fn apply_root_size(
    tree: &mut petals::UiTree,
    width: u32,
    height: u32,
) -> Result<(), petals::TaffyError> {
    tree.apply_style(
        tree.root(),
        ResolvedStyle {
            width: Some(width as f32),
            height: Some(height as f32),
            flex_direction: Some(petals::FlexDirection::Column),
            justify_content: Some(petals::JustifyContent::Start),
            align_items: Some(petals::AlignItems::Stretch),
            ..ResolvedStyle::default()
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entries() -> Vec<ApplicationEntry> {
        alloc::vec![
            ApplicationEntry {
                name: String::from("Calculator"),
                path: String::from("/applications/calc"),
                glyph: String::from("calculator"),
            },
            ApplicationEntry {
                name: String::from("Clock"),
                path: String::from("/applications/clock"),
                glyph: String::from("list-todo"),
            },
        ]
    }

    #[test]
    fn launcher_hit_tests_pressable_tiles() {
        let mut state = LauncherState::new();
        state.open(entries());
        assert_eq!(state.hit_test(800, 600, 130, 156), Some(0));
    }

    #[test]
    fn press_release_returns_application_path() {
        let mut state = LauncherState::new();
        state.open(entries());
        assert!(state.press_at(800, 600, 130, 156));
        assert_eq!(state.release_at(800, 600, 130, 156), Some(String::from("/applications/calc")));
    }
}
