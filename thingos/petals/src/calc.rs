use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::str::FromStr;

use abi::hid::Key;
use taffy::TaffyError;

use crate::{
    AlignItems, AttrValue, Color, Declaration, Description, FlexDirection, FontWeight,
    JustifyContent, NodeId, Rule, Selector, State, UiTheme, UiTree, default_theme,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalcMode {
    Basic,
    Scientific,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CalcError {
    DivideByZero,
    MalformedExpression,
    Unsupported,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Func {
    Sin,
    Cos,
    Tan,
    Sqrt,
    Pow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalcInput {
    Digit(u8),
    DecimalPoint,
    Operator(Op),
    Equals,
    Clear,
    Backspace,
    MemoryAdd,
    MemoryRecall,
    MemoryClear,
    Function(Func),
    ToggleMode,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CalcState {
    pub expression: String,
    pub display: String,
    pub cursor: usize,
    pub memory: f64,
    pub mode: CalcMode,
    pub error: Option<CalcError>,
}

impl Default for CalcState {
    fn default() -> Self {
        Self {
            expression: String::new(),
            display: "0".to_string(),
            cursor: 0,
            memory: 0.0,
            mode: CalcMode::Basic,
            error: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalcKey {
    pub label: &'static str,
    pub announces: &'static str,
    pub category: &'static str,
    pub input: CalcInput,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalcKeyNode {
    pub node: NodeId,
    pub label: NodeId,
    pub key: CalcKey,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalcNodes {
    pub root: NodeId,
    pub display_panel: NodeId,
    pub expression_line: NodeId,
    pub result_line: NodeId,
    pub keypad_grid: NodeId,
    pub key_rows: Vec<NodeId>,
    pub keys: Vec<CalcKeyNode>,
    pub mode_toggle: NodeId,
    pub mode_label: NodeId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Calculator;

impl Calculator {
    pub const fn new() -> Self {
        Self
    }

    pub fn reduce(&self, state: &mut CalcState, input: CalcInput) {
        reduce(state, input);
    }

    pub fn key_for_keyboard(&self, key: Key, shift: bool) -> Option<CalcInput> {
        key_to_input(key, shift)
    }

    pub fn narration(&self, input: CalcInput, state: &CalcState) -> String {
        let action = match input {
            CalcInput::Digit(value) => digit_name(value),
            CalcInput::DecimalPoint => "Decimal point",
            CalcInput::Operator(Op::Add) => "Plus",
            CalcInput::Operator(Op::Sub) => "Minus",
            CalcInput::Operator(Op::Mul) => "Multiply",
            CalcInput::Operator(Op::Div) => "Divide",
            CalcInput::Equals => "Equals",
            CalcInput::Clear => "Clear",
            CalcInput::Backspace => "Backspace",
            CalcInput::MemoryAdd => "Memory add",
            CalcInput::MemoryRecall => "Memory recall",
            CalcInput::MemoryClear => "Memory clear",
            CalcInput::Function(Func::Sin) => "Sine",
            CalcInput::Function(Func::Cos) => "Cosine",
            CalcInput::Function(Func::Tan) => "Tangent",
            CalcInput::Function(Func::Sqrt) => "Square root",
            CalcInput::Function(Func::Pow) => "Power",
            CalcInput::ToggleMode => "Mode",
        };
        let expr = if state.expression.is_empty() { "empty" } else { state.expression.as_str() };
        format!("{} pressed. Expression: {}. Result: {}", action, expr, state.display)
    }

    pub fn build_tree(&self, state: &CalcState) -> Result<(UiTree, CalcNodes), TaffyError> {
        self.build_tree_for_theme(state, default_theme())
    }

    pub fn build_tree_for_theme(
        &self,
        state: &CalcState,
        theme: UiTheme,
    ) -> Result<(UiTree, CalcNodes), TaffyError> {
        let mut tree = UiTree::new()?;
        let root = tree.root();
        let nodes = self.render(&mut tree, root, state)?;
        tree.restyle(&self.rules_for_theme(theme))?;
        Ok((tree, nodes))
    }

    pub fn render(
        &self,
        tree: &mut UiTree,
        parent: NodeId,
        state: &CalcState,
    ) -> Result<CalcNodes, TaffyError> {
        let root = tree.add_node(&[
            Description::Perspective,
            Description::Calculator,
            Description::Container,
            Description::Focusable,
            Description::ExpressesValue,
            Description::AcceptsInputSequence,
        ])?;
        let display_panel = tree.add_node(&[Description::DisplayPanel, Description::Container])?;
        let expression_line = tree.text(display_expression(state))?;
        let result_line = tree.text(&state.display)?;
        let keypad_grid = tree.add_node(&[Description::KeypadGrid, Description::Container])?;
        let mode_toggle = tree.pressable(match state.mode {
            CalcMode::Basic => "Basic",
            CalcMode::Scientific => "Scientific",
        })?;
        let mode_label = tree.text(match state.mode {
            CalcMode::Basic => "Basic",
            CalcMode::Scientific => "Scientific",
        })?;

        for (node, desc) in [
            (expression_line, Description::ExpressionLine),
            (result_line, Description::ResultLine),
            (mode_toggle, Description::ModeToggle),
        ] {
            if let Some(item) = tree.node_mut(node) {
                item.descriptions.push(desc);
            }
        }

        tree.add_child(parent, root)?;
        tree.add_child(root, display_panel)?;
        tree.add_child(display_panel, expression_line)?;
        tree.add_child(display_panel, result_line)?;
        tree.add_child(root, keypad_grid)?;

        let mut key_rows = Vec::new();
        let mut keys = Vec::new();
        for row in self.keys_for_mode(state.mode).chunks(4) {
            let row_node = tree.add_node(&[Description::KeypadRow, Description::Container])?;
            tree.add_child(keypad_grid, row_node)?;
            key_rows.push(row_node);
            for key in row {
                let node = tree.pressable(key.label)?;
                let label = tree.text(key.label)?;
                if let Some(item) = tree.node_mut(node) {
                    item.descriptions.push(Description::CalcKey);
                    item.attrs.insert("announces".into(), AttrValue::Str(key.announces.into()));
                    item.attrs.insert("category".into(), AttrValue::Str(key.category.into()));
                    push_key_descriptions(item, key.input);
                }
                tree.add_child(node, label)?;
                tree.add_child(row_node, node)?;
                keys.push(CalcKeyNode { node, label, key: *key });
            }
        }

        tree.add_child(root, mode_toggle)?;
        tree.add_child(mode_toggle, mode_label)?;

        Ok(CalcNodes {
            root,
            display_panel,
            expression_line,
            result_line,
            keypad_grid,
            key_rows,
            keys,
            mode_toggle,
            mode_label,
        })
    }

    pub fn keys_for_mode(&self, _mode: CalcMode) -> &'static [CalcKey] {
        &BASIC_KEYS
    }

    pub fn rules(&self) -> Vec<Rule<Description>> {
        self.rules_for_theme(default_theme())
    }

    pub fn rules_for_theme(&self, theme: UiTheme) -> Vec<Rule<Description>> {
        alloc::vec![
            Rule::new(
                Selector::has(Description::Calculator),
                alloc::vec![
                    Declaration::FlexDirection(FlexDirection::Column),
                    Declaration::AlignItems(AlignItems::Stretch),
                    Declaration::JustifyContent(JustifyContent::Start),
                    Declaration::Gap(8.0),
                    Declaration::Padding(12.0),
                    Declaration::BackgroundColor(color_from_argb(theme.body_top)),
                ],
            ),
            Rule::new(
                Selector::has(Description::DisplayPanel),
                alloc::vec![
                    Declaration::FlexDirection(FlexDirection::Column),
                    Declaration::AlignItems(AlignItems::End),
                    Declaration::JustifyContent(JustifyContent::Center),
                    Declaration::Padding(16.0),
                    Declaration::Gap(6.0),
                    Declaration::Height(112.0),
                    Declaration::BackgroundColor(color_from_argb(theme.inactive.title_bottom)),
                ],
            ),
            Rule::new(
                Selector::has(Description::ExpressionLine),
                alloc::vec![
                    Declaration::Width(320.0),
                    Declaration::Height(22.0),
                    Declaration::FontSize(18.0),
                    Declaration::Color(color_from_argb(theme.chrome_text_inactive)),
                ],
            ),
            Rule::new(
                Selector::has(Description::ResultLine),
                alloc::vec![
                    Declaration::Width(320.0),
                    Declaration::Height(56.0),
                    Declaration::FontSize(48.0),
                    Declaration::FontWeight(FontWeight::Normal),
                    Declaration::Color(color_from_argb(theme.chrome_text)),
                ],
            ),
            Rule::new(
                Selector::has(Description::KeypadGrid),
                alloc::vec![
                    Declaration::FlexDirection(FlexDirection::Column),
                    Declaration::AlignItems(AlignItems::Stretch),
                    Declaration::JustifyContent(JustifyContent::Start),
                    Declaration::Gap(8.0),
                ],
            ),
            Rule::new(
                Selector::has(Description::KeypadRow),
                alloc::vec![
                    Declaration::FlexDirection(FlexDirection::Row),
                    Declaration::AlignItems(AlignItems::Stretch),
                    Declaration::JustifyContent(JustifyContent::Start),
                    Declaration::Gap(8.0),
                    Declaration::Height(72.0),
                ],
            ),
            Rule::new(
                Selector::has(Description::CalcKey),
                alloc::vec![
                    Declaration::Width(72.0),
                    Declaration::Height(72.0),
                    Declaration::Padding(0.0),
                    Declaration::FlexDirection(FlexDirection::Row),
                    Declaration::AlignItems(AlignItems::Center),
                    Declaration::JustifyContent(JustifyContent::Center),
                    Declaration::BackgroundColor(color_from_argb(theme.button_top)),
                    Declaration::BorderWidth(1.0),
                ],
            ),
            Rule::new(
                Selector::has(Description::CalcKey).and(Selector::state(State::Active)),
                alloc::vec![Declaration::BackgroundColor(color_from_argb(theme.frame_bevel_light))],
            ),
            Rule::new(
                Selector::has(Description::OperatorKey),
                alloc::vec![Declaration::BackgroundColor(color_from_argb(theme.focus_accent))],
            ),
            Rule::new(
                Selector::has(Description::EqualsKey),
                alloc::vec![Declaration::BackgroundColor(color_from_argb(theme.focus_accent))],
            ),
            Rule::new(
                Selector::has(Description::DangerKey),
                alloc::vec![Declaration::BackgroundColor(color_from_argb(theme.close_icon))],
            ),
            Rule::new(
                Selector::has(Description::Textual),
                alloc::vec![
                    Declaration::FontSize(22.0),
                    Declaration::Width(64.0),
                    Declaration::Height(28.0),
                    Declaration::Color(color_from_argb(theme.chrome_text)),
                ],
            ),
            Rule::new(
                Selector::has(Description::OperatorKey),
                alloc::vec![Declaration::Color(Color::rgb(0x11, 0x13, 0x18))],
            ),
            Rule::new(
                Selector::has(Description::ModeToggle),
                alloc::vec![
                    Declaration::Height(32.0),
                    Declaration::Width(144.0),
                    Declaration::Padding(6.0),
                    Declaration::FlexDirection(FlexDirection::Row),
                    Declaration::AlignItems(AlignItems::Center),
                    Declaration::JustifyContent(JustifyContent::Center),
                    Declaration::BackgroundColor(color_from_argb(theme.inactive.title_bottom)),
                ],
            ),
        ]
    }
}

const fn color_from_argb(argb: u32) -> Color {
    Color::rgba(
        ((argb >> 16) & 0xFF) as u8,
        ((argb >> 8) & 0xFF) as u8,
        (argb & 0xFF) as u8,
        ((argb >> 24) & 0xFF) as u8,
    )
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

pub fn reduce(state: &mut CalcState, input: CalcInput) {
    match input {
        CalcInput::Digit(digit) if digit <= 9 => {
            clear_error(state);
            state.expression.insert(state.cursor, char::from(b'0' + digit));
            state.cursor += 1;
            refresh_display(state);
        }
        CalcInput::DecimalPoint => {
            clear_error(state);
            if !current_number_has_decimal(state) {
                if needs_number_prefix(state) {
                    state.expression.insert(state.cursor, '0');
                    state.cursor += 1;
                }
                state.expression.insert(state.cursor, '.');
                state.cursor += 1;
                refresh_display(state);
            }
        }
        CalcInput::Operator(op) => {
            clear_error(state);
            push_operator(state, op);
            refresh_display(state);
        }
        CalcInput::Equals => match evaluate_expression(&state.expression) {
            Ok(value) => {
                state.display = format_number(value);
                state.expression = state.display.clone();
                state.cursor = state.expression.len();
                state.error = None;
            }
            Err(err) => set_error(state, err),
        },
        CalcInput::Clear => *state = CalcState::default(),
        CalcInput::Backspace => {
            clear_error(state);
            backspace(state);
            refresh_display(state);
        }
        CalcInput::MemoryAdd => {
            if let Ok(value) = f64::from_str(&state.display) {
                state.memory += value;
            }
        }
        CalcInput::MemoryRecall => {
            clear_error(state);
            let text = format_number(state.memory);
            state.expression.insert_str(state.cursor, &text);
            state.cursor += text.len();
            refresh_display(state);
        }
        CalcInput::MemoryClear => state.memory = 0.0,
        CalcInput::ToggleMode => {
            state.mode = match state.mode {
                CalcMode::Basic => CalcMode::Scientific,
                CalcMode::Scientific => CalcMode::Basic,
            };
        }
        CalcInput::Function(_) => set_error(state, CalcError::Unsupported),
        CalcInput::Digit(_) => {}
    }
}

pub fn key_to_input(key: Key, shift: bool) -> Option<CalcInput> {
    Some(match key {
        Key::Num0 => CalcInput::Digit(0),
        Key::Num1 => CalcInput::Digit(1),
        Key::Num2 => CalcInput::Digit(2),
        Key::Num3 => CalcInput::Digit(3),
        Key::Num4 => CalcInput::Digit(4),
        Key::Num5 => CalcInput::Digit(5),
        Key::Num6 => CalcInput::Digit(6),
        Key::Num7 => CalcInput::Digit(7),
        Key::Num8 if shift => CalcInput::Operator(Op::Mul),
        Key::Num8 => CalcInput::Digit(8),
        Key::Num9 => CalcInput::Digit(9),
        Key::Period => CalcInput::DecimalPoint,
        Key::Enter | Key::Equal if !shift => CalcInput::Equals,
        Key::Equal => CalcInput::Operator(Op::Add),
        Key::Minus => CalcInput::Operator(Op::Sub),
        Key::Slash => CalcInput::Operator(Op::Div),
        Key::Backspace | Key::Delete => CalcInput::Backspace,
        Key::Escape => CalcInput::Clear,
        Key::M => CalcInput::MemoryRecall,
        Key::C => CalcInput::Clear,
        _ => return None,
    })
}

pub fn evaluate_expression(expression: &str) -> Result<f64, CalcError> {
    let tokens = tokenize(expression)?;
    if tokens.is_empty() {
        return Ok(0.0);
    }
    let mut parser = Parser { tokens, pos: 0 };
    let value = parser.parse_expression()?;
    if parser.pos != parser.tokens.len() {
        return Err(CalcError::MalformedExpression);
    }
    Ok(value)
}

const BASIC_KEYS: [CalcKey; 20] = [
    key("7", "Seven", "digit", CalcInput::Digit(7)),
    key("8", "Eight", "digit", CalcInput::Digit(8)),
    key("9", "Nine", "digit", CalcInput::Digit(9)),
    key("/", "Divide", "operator", CalcInput::Operator(Op::Div)),
    key("4", "Four", "digit", CalcInput::Digit(4)),
    key("5", "Five", "digit", CalcInput::Digit(5)),
    key("6", "Six", "digit", CalcInput::Digit(6)),
    key("*", "Multiply", "operator", CalcInput::Operator(Op::Mul)),
    key("1", "One", "digit", CalcInput::Digit(1)),
    key("2", "Two", "digit", CalcInput::Digit(2)),
    key("3", "Three", "digit", CalcInput::Digit(3)),
    key("-", "Minus", "operator", CalcInput::Operator(Op::Sub)),
    key("0", "Zero", "digit", CalcInput::Digit(0)),
    key(".", "Decimal point", "decimal", CalcInput::DecimalPoint),
    key("=", "Equals", "equals", CalcInput::Equals),
    key("+", "Plus", "operator", CalcInput::Operator(Op::Add)),
    key("C", "Clear", "danger", CalcInput::Clear),
    key("<", "Backspace", "editing", CalcInput::Backspace),
    key("M+", "Memory add", "memory", CalcInput::MemoryAdd),
    key("MR", "Memory recall", "memory", CalcInput::MemoryRecall),
];

const fn key(
    label: &'static str,
    announces: &'static str,
    category: &'static str,
    input: CalcInput,
) -> CalcKey {
    CalcKey { label, announces, category, input }
}

fn push_key_descriptions(node: &mut crate::Node, input: CalcInput) {
    match input {
        CalcInput::Digit(_) => node.descriptions.push(Description::DigitKey),
        CalcInput::DecimalPoint => node.descriptions.push(Description::DecimalKey),
        CalcInput::Operator(_) => node.descriptions.push(Description::OperatorKey),
        CalcInput::Equals => node.descriptions.push(Description::EqualsKey),
        CalcInput::Clear | CalcInput::Backspace => node.descriptions.push(Description::DangerKey),
        CalcInput::MemoryAdd | CalcInput::MemoryRecall | CalcInput::MemoryClear => {
            node.descriptions.push(Description::MemoryKey)
        }
        _ => {}
    }
}

fn clear_error(state: &mut CalcState) {
    if state.error.is_some() {
        state.expression.clear();
        state.cursor = 0;
        state.error = None;
        state.display = "0".to_string();
    }
}

fn set_error(state: &mut CalcState, err: CalcError) {
    state.error = Some(err);
    state.display = match err {
        CalcError::DivideByZero => "Divide by zero",
        CalcError::MalformedExpression => "Error",
        CalcError::Unsupported => "Unsupported",
    }
    .to_string();
}

fn refresh_display(state: &mut CalcState) {
    if state.expression.trim().is_empty() {
        state.display = "0".to_string();
        return;
    }
    match evaluate_expression(&state.expression) {
        Ok(value) => state.display = format_number(value),
        Err(_) => state.display = trailing_number(&state.expression).unwrap_or("0").to_string(),
    }
}

fn push_operator(state: &mut CalcState, op: Op) {
    if state.expression.trim().is_empty() {
        if op == Op::Sub {
            state.expression.push('-');
            state.cursor = state.expression.len();
        }
        return;
    }
    trim_trailing_spaces(&mut state.expression);
    while state.expression.ends_with_operator() {
        state.expression.pop();
        trim_trailing_spaces(&mut state.expression);
    }
    state.expression.push(' ');
    state.expression.push(op_char(op));
    state.expression.push(' ');
    state.cursor = state.expression.len();
}

fn backspace(state: &mut CalcState) {
    if state.cursor == 0 || state.expression.is_empty() {
        return;
    }
    if state.expression[..state.cursor].ends_with(' ') {
        while state.cursor > 0 && state.expression.as_bytes()[state.cursor - 1] == b' ' {
            state.expression.remove(state.cursor - 1);
            state.cursor -= 1;
        }
        if state.cursor > 0 {
            state.expression.remove(state.cursor - 1);
            state.cursor -= 1;
        }
        while state.cursor > 0 && state.expression.as_bytes()[state.cursor - 1] == b' ' {
            state.expression.remove(state.cursor - 1);
            state.cursor -= 1;
        }
    } else {
        state.expression.remove(state.cursor - 1);
        state.cursor -= 1;
    }
}

fn needs_number_prefix(state: &CalcState) -> bool {
    state.cursor == 0 || state.expression[..state.cursor].ends_with(' ')
}

fn current_number_has_decimal(state: &CalcState) -> bool {
    let start = state.expression[..state.cursor].rfind(' ').map(|index| index + 1).unwrap_or(0);
    state.expression[start..state.cursor].contains('.')
}

fn display_expression(state: &CalcState) -> &str {
    if state.expression.is_empty() { " " } else { state.expression.as_str() }
}

fn op_char(op: Op) -> char {
    match op {
        Op::Add => '+',
        Op::Sub => '-',
        Op::Mul => '*',
        Op::Div => '/',
    }
}

fn digit_name(value: u8) -> &'static str {
    match value {
        0 => "Zero",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        _ => "Digit",
    }
}

fn format_number(value: f64) -> String {
    if !value.is_finite() {
        return "Error".to_string();
    }
    if value > i64::MIN as f64 && value < i64::MAX as f64 {
        let integer = value as i64;
        let diff = value - integer as f64;
        let magnitude = if diff < 0.0 { -diff } else { diff };
        if magnitude < 0.000_000_001 {
            return format!("{}", integer);
        }
    }
    let text = format!("{:.8}", value);
    text.trim_end_matches('0').trim_end_matches('.').to_string()
}

fn trim_trailing_spaces(value: &mut String) {
    while value.ends_with(' ') {
        value.pop();
    }
}

trait ExpressionExt {
    fn ends_with_operator(&self) -> bool;
}

impl ExpressionExt for String {
    fn ends_with_operator(&self) -> bool {
        self.as_str().ends_with_operator()
    }
}

impl ExpressionExt for str {
    fn ends_with_operator(&self) -> bool {
        self.ends_with('+') || self.ends_with('-') || self.ends_with('*') || self.ends_with('/')
    }
}

fn trailing_number(expression: &str) -> Option<&str> {
    expression.split_whitespace().last()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    Number(f64),
    Op(Op),
}

fn tokenize(expression: &str) -> Result<Vec<Token>, CalcError> {
    let mut tokens = Vec::new();
    for item in expression.split_whitespace() {
        let token = match item {
            "+" => Token::Op(Op::Add),
            "-" => Token::Op(Op::Sub),
            "*" => Token::Op(Op::Mul),
            "/" => Token::Op(Op::Div),
            _ => {
                let value = f64::from_str(item).map_err(|_| CalcError::MalformedExpression)?;
                Token::Number(value)
            }
        };
        tokens.push(token);
    }
    Ok(tokens)
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn parse_expression(&mut self) -> Result<f64, CalcError> {
        let mut value = self.parse_term()?;
        while let Some(op @ (Op::Add | Op::Sub)) = self.peek_op() {
            self.pos += 1;
            let rhs = self.parse_term()?;
            value = match op {
                Op::Add => value + rhs,
                Op::Sub => value - rhs,
                _ => value,
            };
        }
        Ok(value)
    }

    fn parse_term(&mut self) -> Result<f64, CalcError> {
        let mut value = self.parse_number()?;
        while let Some(op @ (Op::Mul | Op::Div)) = self.peek_op() {
            self.pos += 1;
            let rhs = self.parse_number()?;
            value = match op {
                Op::Mul => value * rhs,
                Op::Div if rhs == 0.0 => return Err(CalcError::DivideByZero),
                Op::Div => value / rhs,
                _ => value,
            };
        }
        Ok(value)
    }

    fn parse_number(&mut self) -> Result<f64, CalcError> {
        match self.tokens.get(self.pos) {
            Some(Token::Number(value)) => {
                self.pos += 1;
                Ok(*value)
            }
            _ => Err(CalcError::MalformedExpression),
        }
    }

    fn peek_op(&self) -> Option<Op> {
        match self.tokens.get(self.pos) {
            Some(Token::Op(op)) => Some(*op),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use taffy::prelude::{AvailableSpace, Size};

    use super::*;
    use crate::ResolvedStyle;

    #[test]
    fn reducer_supports_basic_precedence_and_equals() {
        let calc = Calculator::new();
        let mut state = CalcState::default();
        for input in [
            CalcInput::Digit(1),
            CalcInput::Digit(2),
            CalcInput::Operator(Op::Add),
            CalcInput::Digit(7),
            CalcInput::Operator(Op::Mul),
            CalcInput::Digit(3),
        ] {
            calc.reduce(&mut state, input);
        }

        assert_eq!(state.expression, "12 + 7 * 3");
        assert_eq!(state.display, "33");
        calc.reduce(&mut state, CalcInput::Equals);
        assert_eq!(state.expression, "33");
        assert_eq!(state.display, "33");
    }

    #[test]
    fn backspace_reduces_the_model_without_style_state() {
        let calc = Calculator::new();
        let mut state = CalcState::default();
        for input in [
            CalcInput::Digit(4),
            CalcInput::Digit(2),
            CalcInput::Operator(Op::Div),
            CalcInput::Digit(2),
            CalcInput::Backspace,
            CalcInput::Digit(7),
        ] {
            calc.reduce(&mut state, input);
        }

        assert_eq!(state.expression, "42 / 7");
        assert_eq!(state.display, "6");
    }

    #[test]
    fn maps_keyboard_to_calculator_inputs() {
        assert_eq!(key_to_input(Key::Num8, false), Some(CalcInput::Digit(8)));
        assert_eq!(key_to_input(Key::Num8, true), Some(CalcInput::Operator(Op::Mul)));
        assert_eq!(key_to_input(Key::Equal, true), Some(CalcInput::Operator(Op::Add)));
        assert_eq!(key_to_input(Key::Backspace, false), Some(CalcInput::Backspace));
    }

    #[test]
    fn builds_petals_tree_with_descriptions_and_key_grid() {
        let calc = Calculator::new();
        let mut state = CalcState::default();
        calc.reduce(&mut state, CalcInput::Digit(7));
        let (mut tree, nodes) = calc.build_tree(&state).unwrap();

        tree.apply_style(
            tree.root(),
            ResolvedStyle {
                width: Some(360.0),
                height: Some(560.0),
                flex_direction: Some(FlexDirection::Column),
                justify_content: Some(JustifyContent::Start),
                align_items: Some(AlignItems::Stretch),
                ..ResolvedStyle::default()
            },
        )
        .unwrap();
        tree.compute_layout(Size {
            width: AvailableSpace::Definite(360.0),
            height: AvailableSpace::Definite(560.0),
        })
        .unwrap();

        assert_eq!(nodes.keys.len(), 20);
        assert!(tree.node(nodes.root).unwrap().descriptions.contains(&Description::Calculator));
        assert!(
            tree.node(nodes.keys[0].node).unwrap().descriptions.contains(&Description::Pressable)
        );
        assert_eq!(tree.global_layout_box(nodes.keys[0].node).unwrap().width, 72.0);
    }

    #[test]
    fn calculator_surfaces_use_stile_theme_colors() {
        let calc = Calculator::new();
        let theme = crate::theme_by_name("leather.bmp");
        let (tree, nodes) = calc.build_tree_for_theme(&CalcState::default(), theme).unwrap();

        assert_eq!(
            tree.node(nodes.root).unwrap().style.background_color,
            Some(color_from_argb(theme.body_top))
        );
        assert_eq!(
            tree.node(nodes.keys[0].node).unwrap().style.background_color,
            Some(color_from_argb(theme.button_top))
        );
    }

    #[test]
    fn narration_reports_input_expression_and_result() {
        let calc = Calculator::new();
        let mut state = CalcState::default();
        calc.reduce(&mut state, CalcInput::Digit(7));
        assert_eq!(
            calc.narration(CalcInput::Digit(7), &state),
            "Seven pressed. Expression: 7. Result: 7"
        );
    }
}
