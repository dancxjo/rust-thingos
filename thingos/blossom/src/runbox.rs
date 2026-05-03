//! RunBox overlay state and Petals structure.
//!
//! The compositor owns the actual surface, but this module keeps the model and
//! input reducer side-effect free so it can be tested without Bloom.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use abi::hid::{Key, Mods};
use petals::{
    AlignItems, Color, Declaration, Description, FlexDirection, JustifyContent, NodeId, Rule,
    Selector, TaffyError, UiTree,
};

use crate::layer_shell::{
    LayerKeyboardInteractivity, LayerShellLayer, LayerSurfaceConfig, compute_layer_placement,
};

pub const RUNBOX_WIDTH: u32 = 420;
pub const RUNBOX_HEIGHT: u32 = 124;
pub const RUNBOX_HISTORY_LIMIT: usize = 50;
const RUNBOX_VERTICAL_OFFSET: i32 = -40;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunState {
    pub input: String,
    pub cursor: usize,
    pub history: Vec<String>,
    pub visible: bool,
    history_index: Option<usize>,
}

impl RunState {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            cursor: 0,
            history: Vec::new(),
            visible: false,
            history_index: None,
        }
    }

    pub fn open(&mut self) {
        self.visible = true;
        self.history_index = None;
        self.cursor = self.input.len();
    }

    pub fn close(&mut self) {
        self.visible = false;
        self.input.clear();
        self.cursor = 0;
        self.history_index = None;
    }

    pub fn toggle(&mut self) -> bool {
        if self.visible {
            self.close();
            false
        } else {
            self.open();
            true
        }
    }

    pub fn insert(&mut self, ch: char) {
        if !ch.is_ascii_graphic() && ch != ' ' {
            return;
        }
        self.input.insert(self.cursor, ch);
        self.cursor += ch.len_utf8();
        self.history_index = None;
    }

    pub fn delete(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.cursor -= 1;
        self.input.remove(self.cursor);
        self.history_index = None;
    }

    pub fn move_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    pub fn move_right(&mut self) {
        self.cursor = self.cursor.saturating_add(1).min(self.input.len());
    }

    pub fn history_prev(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let next = match self.history_index {
            Some(index) => index.saturating_sub(1),
            None => self.history.len().saturating_sub(1),
        };
        self.history_index = Some(next);
        self.input = self.history[next].clone();
        self.cursor = self.input.len();
    }

    pub fn history_next(&mut self) {
        let Some(index) = self.history_index else {
            return;
        };
        if index + 1 >= self.history.len() {
            self.history_index = None;
            self.input.clear();
        } else {
            let next = index + 1;
            self.history_index = Some(next);
            self.input = self.history[next].clone();
        }
        self.cursor = self.input.len();
    }

    pub fn submit(&mut self) -> Option<RunAction> {
        let command = self.input.trim().to_string();
        self.close();
        if command.is_empty() {
            return None;
        }
        if self.history.last() != Some(&command) {
            self.history.push(command.clone());
            if self.history.len() > RUNBOX_HISTORY_LIMIT {
                self.history.remove(0);
            }
        }
        Some(RunAction::Execute(command))
    }

    pub fn handle_key(&mut self, key: Key, mods: Mods) -> RunBoxEvent {
        match key {
            Key::Escape => {
                self.close();
                RunBoxEvent::Closed
            }
            Key::Enter => match self.submit() {
                Some(action) => RunBoxEvent::Submit(action),
                None => RunBoxEvent::Closed,
            },
            Key::Backspace => {
                self.delete();
                RunBoxEvent::Updated
            }
            Key::Left => {
                self.move_left();
                RunBoxEvent::Updated
            }
            Key::Right => {
                self.move_right();
                RunBoxEvent::Updated
            }
            Key::Up => {
                self.history_prev();
                RunBoxEvent::Updated
            }
            Key::Down => {
                self.history_next();
                RunBoxEvent::Updated
            }
            Key::Tab => RunBoxEvent::Updated,
            _ => {
                if let Some(ch) = key_to_char(key, mods) {
                    self.insert(ch);
                    RunBoxEvent::Updated
                } else {
                    RunBoxEvent::Consumed
                }
            }
        }
    }
}

impl Default for RunState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunAction {
    Execute(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunBoxEvent {
    Consumed,
    Updated,
    Closed,
    Submit(RunAction),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaunchResult {
    Started { pid: u64 },
    NotFound,
    Failed,
}

pub trait Launcher {
    fn run(&self, command: &str) -> LaunchResult;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RunBoxNodes {
    pub root: NodeId,
    pub label: NodeId,
    pub text_field: NodeId,
    pub submit_button: NodeId,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RunBoxView;

impl RunBoxView {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, tree: &mut UiTree, parent: NodeId) -> Result<RunBoxNodes, TaffyError> {
        let root = tree.add_node(&[
            Description::Perspective,
            Description::RunBox,
            Description::Container,
            Description::Focusable,
            Description::AcceptsText,
            Description::SubmitsActionRequest,
        ])?;
        let label = tree.text("Run")?;
        if let Some(node) = tree.node_mut(label) {
            node.descriptions.push(Description::RunLabel);
        }
        let text_field = tree.add_node(&[
            Description::TextField,
            Description::Container,
            Description::Focusable,
            Description::AcceptsText,
        ])?;
        let submit_button = tree.add_node(&[
            Description::SubmitButton,
            Description::Pressable,
            Description::SubmitsActionRequest,
        ])?;

        tree.add_child(parent, root)?;
        tree.add_child(root, label)?;
        tree.add_child(root, text_field)?;
        tree.add_child(root, submit_button)?;

        Ok(RunBoxNodes { root, label, text_field, submit_button })
    }

    pub fn build_tree(&self) -> Result<(UiTree, RunBoxNodes), TaffyError> {
        let mut tree = UiTree::new()?;
        let root = tree.root();
        let nodes = self.render(&mut tree, root)?;
        tree.restyle(&self.rules())?;
        Ok((tree, nodes))
    }

    pub fn rules(&self) -> Vec<Rule<Description>> {
        alloc::vec![
            Rule::new(
                Selector::has(Description::RunBox),
                alloc::vec![
                    Declaration::Width(RUNBOX_WIDTH as f32),
                    Declaration::Height(RUNBOX_HEIGHT as f32),
                    Declaration::Padding(16.0),
                    Declaration::Gap(8.0),
                    Declaration::FlexDirection(FlexDirection::Column),
                    Declaration::AlignItems(AlignItems::Stretch),
                    Declaration::JustifyContent(JustifyContent::Start),
                    Declaration::BackgroundColor(Color::rgba(24, 28, 38, 245)),
                    Declaration::BorderWidth(1.0),
                ],
            ),
            Rule::new(
                Selector::has(Description::RunLabel),
                alloc::vec![
                    Declaration::Height(18.0),
                    Declaration::FontSize(14.0),
                    Declaration::Color(Color::rgb(0x8f, 0x98, 0xaa)),
                ],
            ),
            Rule::new(
                Selector::has(Description::TextField),
                alloc::vec![
                    Declaration::Height(42.0),
                    Declaration::Padding(10.0),
                    Declaration::BackgroundColor(Color::rgb(0x11, 0x13, 0x18)),
                    Declaration::BorderWidth(1.0),
                    Declaration::Color(Color::rgb(0xe6, 0xea, 0xf0)),
                ],
            ),
            Rule::new(
                Selector::has(Description::TextField).and(Selector::state(petals::State::Focus)),
                alloc::vec![Declaration::OutlineColor(Color::rgb(0xd8, 0xa6, 0x57))],
            ),
            Rule::new(
                Selector::has(Description::SubmitButton),
                alloc::vec![
                    Declaration::Width(72.0),
                    Declaration::Height(32.0),
                    Declaration::Padding(8.0),
                    Declaration::AlignItems(AlignItems::End),
                    Declaration::BackgroundColor(Color::rgb(0xd8, 0xa6, 0x57)),
                    Declaration::Color(Color::rgb(0x11, 0x13, 0x18)),
                ],
            ),
        ]
    }
}

pub fn runbox_layer_config() -> LayerSurfaceConfig {
    let mut cfg = LayerSurfaceConfig::new(LayerShellLayer::Overlay);
    cfg.anchor = 0;
    cfg.exclusive_zone = -1;
    cfg.keyboard_interactivity = LayerKeyboardInteractivity::Exclusive;
    cfg.size = (RUNBOX_WIDTH, RUNBOX_HEIGHT);
    cfg.margin_top = RUNBOX_VERTICAL_OFFSET;
    cfg
}

pub fn runbox_rect(output_w: u32, output_h: u32) -> crate::LayerSurfacePlacement {
    compute_layer_placement(output_w, output_h, &runbox_layer_config())
}

fn key_to_char(key: Key, mods: Mods) -> Option<char> {
    let shifted = mods.has_shift();
    match key {
        Key::A => Some(if shifted { 'A' } else { 'a' }),
        Key::B => Some(if shifted { 'B' } else { 'b' }),
        Key::C => Some(if shifted { 'C' } else { 'c' }),
        Key::D => Some(if shifted { 'D' } else { 'd' }),
        Key::E => Some(if shifted { 'E' } else { 'e' }),
        Key::F => Some(if shifted { 'F' } else { 'f' }),
        Key::G => Some(if shifted { 'G' } else { 'g' }),
        Key::H => Some(if shifted { 'H' } else { 'h' }),
        Key::I => Some(if shifted { 'I' } else { 'i' }),
        Key::J => Some(if shifted { 'J' } else { 'j' }),
        Key::K => Some(if shifted { 'K' } else { 'k' }),
        Key::L => Some(if shifted { 'L' } else { 'l' }),
        Key::M => Some(if shifted { 'M' } else { 'm' }),
        Key::N => Some(if shifted { 'N' } else { 'n' }),
        Key::O => Some(if shifted { 'O' } else { 'o' }),
        Key::P => Some(if shifted { 'P' } else { 'p' }),
        Key::Q => Some(if shifted { 'Q' } else { 'q' }),
        Key::R => Some(if shifted { 'R' } else { 'r' }),
        Key::S => Some(if shifted { 'S' } else { 's' }),
        Key::T => Some(if shifted { 'T' } else { 't' }),
        Key::U => Some(if shifted { 'U' } else { 'u' }),
        Key::V => Some(if shifted { 'V' } else { 'v' }),
        Key::W => Some(if shifted { 'W' } else { 'w' }),
        Key::X => Some(if shifted { 'X' } else { 'x' }),
        Key::Y => Some(if shifted { 'Y' } else { 'y' }),
        Key::Z => Some(if shifted { 'Z' } else { 'z' }),
        Key::Num1 => Some(if shifted { '!' } else { '1' }),
        Key::Num2 => Some(if shifted { '@' } else { '2' }),
        Key::Num3 => Some(if shifted { '#' } else { '3' }),
        Key::Num4 => Some(if shifted { '$' } else { '4' }),
        Key::Num5 => Some(if shifted { '%' } else { '5' }),
        Key::Num6 => Some(if shifted { '^' } else { '6' }),
        Key::Num7 => Some(if shifted { '&' } else { '7' }),
        Key::Num8 => Some(if shifted { '*' } else { '8' }),
        Key::Num9 => Some(if shifted { '(' } else { '9' }),
        Key::Num0 => Some(if shifted { ')' } else { '0' }),
        Key::Space => Some(' '),
        Key::Minus => Some(if shifted { '_' } else { '-' }),
        Key::Equal => Some(if shifted { '+' } else { '=' }),
        Key::LeftBracket => Some(if shifted { '{' } else { '[' }),
        Key::RightBracket => Some(if shifted { '}' } else { ']' }),
        Key::Backslash => Some(if shifted { '|' } else { '\\' }),
        Key::Semicolon => Some(if shifted { ':' } else { ';' }),
        Key::Quote => Some(if shifted { '"' } else { '\'' }),
        Key::Grave => Some(if shifted { '~' } else { '`' }),
        Key::Comma => Some(if shifted { '<' } else { ',' }),
        Key::Period => Some(if shifted { '>' } else { '.' }),
        Key::Slash => Some(if shifted { '?' } else { '/' }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edits_text_and_cursor() {
        let mut state = RunState::new();
        state.open();
        state.handle_key(Key::E, Mods(0));
        state.handle_key(Key::C, Mods(0));
        state.handle_key(Key::H, Mods(0));
        state.handle_key(Key::O, Mods(0));
        state.handle_key(Key::Left, Mods(0));
        state.handle_key(Key::Backspace, Mods(0));
        assert_eq!(state.input, "eco");
        assert_eq!(state.cursor, 2);
    }

    #[test]
    fn submit_records_history_and_closes() {
        let mut state = RunState::new();
        state.open();
        for key in [Key::E, Key::C, Key::H, Key::O] {
            state.handle_key(key, Mods(0));
        }
        let event = state.handle_key(Key::Enter, Mods(0));
        assert_eq!(event, RunBoxEvent::Submit(RunAction::Execute("echo".to_string())));
        assert!(!state.visible);
        assert_eq!(state.history, alloc::vec!["echo".to_string()]);
    }

    #[test]
    fn history_cycles_backward_and_forward() {
        let mut state = RunState::new();
        state.history.push("clock".to_string());
        state.history.push("calc".to_string());
        state.open();
        state.handle_key(Key::Up, Mods(0));
        assert_eq!(state.input, "calc");
        state.handle_key(Key::Up, Mods(0));
        assert_eq!(state.input, "clock");
        state.handle_key(Key::Down, Mods(0));
        assert_eq!(state.input, "calc");
        state.handle_key(Key::Down, Mods(0));
        assert_eq!(state.input, "");
    }

    #[test]
    fn runbox_uses_overlay_layer_exclusive_keyboard() {
        let cfg = runbox_layer_config();
        assert_eq!(cfg.layer, LayerShellLayer::Overlay);
        assert_eq!(cfg.anchor, 0);
        assert_eq!(cfg.exclusive_zone, -1);
        assert_eq!(cfg.keyboard_interactivity, LayerKeyboardInteractivity::Exclusive);

        let rect = runbox_rect(800, 600);
        assert_eq!(rect.x, 190);
        assert_eq!(rect.y, 198);
        assert_eq!(rect.width, RUNBOX_WIDTH);
        assert_eq!(rect.height, RUNBOX_HEIGHT);
    }

    #[test]
    fn builds_petals_tree_with_expected_descriptions() {
        let view = RunBoxView::new();
        let (tree, nodes) = view.build_tree().unwrap();
        assert!(tree.node(nodes.root).unwrap().descriptions.contains(&Description::RunBox));
        assert!(
            tree.node(nodes.text_field).unwrap().descriptions.contains(&Description::TextField)
        );
        assert!(
            tree.node(nodes.submit_button).unwrap().descriptions.contains(&Description::Pressable)
        );
    }
}
