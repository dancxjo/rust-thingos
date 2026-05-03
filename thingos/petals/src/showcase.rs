use alloc::string::String;
use alloc::vec::Vec;

use taffy::TaffyError;

use crate::{
    AlignItems, ApplicationEntry, ApplicationLauncher, ApplicationLauncherService, AvailableSpace,
    CalcInput, CalcState, Calculator, CalculatorService, Clock, ClockService, ClockState, Color,
    Declaration, Description, FlexDirection, FontWeight, JustifyContent, NodeId, PetalsEvent,
    PetalsService, ResolvedStyle, Rule, Selector, ServiceAction, Size, State, Theme, UiTree,
    WindowChromeNodes, WindowChromeService, default_theme, render_window_chrome,
    window_chrome_rules_for_theme,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShowcaseComponent {
    Clock,
    Calculator,
    Launcher,
    WindowChrome,
    StileStates,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StileStateDemoNodes {
    pub root: NodeId,
    pub title: NodeId,
    pub strip: NodeId,
    pub normal: NodeId,
    pub active: NodeId,
    pub focus: NodeId,
    pub disabled: NodeId,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StileStateDemoService;

impl PetalsService for StileStateDemoService {
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ShowcaseServices {
    pub clock: ClockService,
    pub calculator: CalculatorService,
    pub launcher: ApplicationLauncherService,
    pub chrome: WindowChromeService,
    pub stile_states: StileStateDemoService,
}

impl ShowcaseServices {
    pub fn dispatch(
        &mut self,
        component: ShowcaseComponent,
        tree: &mut UiTree,
        event: PetalsEvent,
    ) -> ServiceAction {
        match component {
            ShowcaseComponent::Clock => self.clock.dispatch(tree, event),
            ShowcaseComponent::Calculator => self.calculator.dispatch(tree, event),
            ShowcaseComponent::Launcher => self.launcher.dispatch(tree, event),
            ShowcaseComponent::WindowChrome => self.chrome.dispatch(tree, event),
            ShowcaseComponent::StileStates => self.stile_states.dispatch(tree, event),
        }
    }
}

#[derive(Clone, Copy)]
pub struct PetalsShowcase {
    pub theme: Theme,
}

impl PetalsShowcase {
    pub const fn new(theme: Theme) -> Self {
        Self { theme }
    }

    pub fn clock_tree(&self) -> Result<(UiTree, crate::ClockNodes), TaffyError> {
        let clock = Clock::new();
        let state = ClockState { hours: 9, minutes: 41, is_pm: true, day: 2, month: 5, year: 2026 };
        clock.build_tree_for_theme(&state, self.theme)
    }

    pub fn calculator_tree(&self) -> Result<(UiTree, crate::CalcNodes), TaffyError> {
        let calc = Calculator::new();
        let mut state = CalcState::default();
        for input in [
            CalcInput::Digit(1),
            CalcInput::Digit(2),
            CalcInput::Operator(crate::Op::Add),
            CalcInput::Digit(7),
            CalcInput::Operator(crate::Op::Mul),
            CalcInput::Digit(3),
            CalcInput::Equals,
        ] {
            calc.reduce(&mut state, input);
        }
        calc.build_tree_for_theme(&state, self.theme)
    }

    pub fn launcher_tree(&self) -> Result<(UiTree, crate::ApplicationLauncherNodes), TaffyError> {
        ApplicationLauncher::new(alloc::vec![
            app("Clock", "/applications/clock", "clock"),
            app("Calculator", "/applications/calc", "calculator"),
            app("Leaf", "/applications/leaf", "monitor"),
            app("Wayland Hello", "/applications/wayland_hello", "sparkles"),
        ])
        .with_status("Petals + Stile components")
        .build_tree()
    }

    pub fn chrome_tree(&self) -> Result<(UiTree, WindowChromeNodes), TaffyError> {
        let mut tree = UiTree::new()?;
        let root = tree.root();
        let nodes = render_window_chrome(&mut tree, root, "Petals chrome")?;
        tree.restyle(&window_chrome_rules_for_theme(self.theme))?;
        Ok((tree, nodes))
    }

    pub fn stile_state_tree(&self) -> Result<(UiTree, StileStateDemoNodes), TaffyError> {
        let mut tree = UiTree::new()?;
        let root = tree.add_node(&[
            Description::Perspective,
            Description::Showcase,
            Description::ShowcasePanel,
            Description::Container,
        ])?;
        let title = tree.text("Stile states")?;
        let strip = tree.add_node(&[Description::ShowcaseStateStrip, Description::Container])?;
        let normal = tree.pressable("Normal")?;
        let active = tree.pressable("Active")?;
        let focus = tree.pressable("Focus")?;
        let disabled = tree.pressable("Disabled")?;

        if let Some(node) = tree.node_mut(title) {
            node.descriptions.push(Description::ShowcaseHeader);
        }
        tree.add_child(tree.root(), root)?;
        tree.add_child(root, title)?;
        tree.add_child(root, strip)?;
        for node in [normal, active, focus, disabled] {
            if let Some(item) = tree.node_mut(node) {
                item.descriptions.push(Description::ShowcaseSwatch);
            }
            tree.add_child(strip, node)?;
        }

        let mut service = StileStateDemoService;
        service.dispatch(
            &mut tree,
            PetalsEvent::SetState { node: active, state: State::Active, enabled: true },
        );
        service.dispatch(
            &mut tree,
            PetalsEvent::SetState { node: focus, state: State::Focus, enabled: true },
        );
        service.dispatch(
            &mut tree,
            PetalsEvent::SetState { node: disabled, state: State::Disabled, enabled: true },
        );
        tree.restyle(&self.stile_state_rules())?;
        Ok((tree, StileStateDemoNodes { root, title, strip, normal, active, focus, disabled }))
    }

    pub fn prepare_component(
        &self,
        tree: &mut UiTree,
        width: u32,
        height: u32,
        justify_content: JustifyContent,
        align_items: AlignItems,
    ) -> Result<(), TaffyError> {
        tree.apply_style(
            tree.root(),
            ResolvedStyle {
                width: Some(width as f32),
                height: Some(height as f32),
                flex_direction: Some(FlexDirection::Column),
                justify_content: Some(justify_content),
                align_items: Some(align_items),
                ..ResolvedStyle::default()
            },
        )?;
        tree.compute_layout(Size {
            width: AvailableSpace::Definite(width as f32),
            height: AvailableSpace::Definite(height as f32),
        })
    }

    pub fn stile_state_rules(&self) -> Vec<Rule<Description>> {
        let theme = self.theme;
        alloc::vec![
            Rule::new(
                Selector::has(Description::ShowcasePanel),
                alloc::vec![
                    Declaration::FlexDirection(FlexDirection::Column),
                    Declaration::AlignItems(AlignItems::Stretch),
                    Declaration::JustifyContent(JustifyContent::Start),
                    Declaration::Gap(10.0),
                    Declaration::Padding(14.0),
                    Declaration::BackgroundColor(color_from_argb(theme.frame_fill)),
                    Declaration::BorderWidth(1.0),
                ],
            ),
            Rule::new(
                Selector::has(Description::ShowcaseHeader),
                alloc::vec![
                    Declaration::Height(20.0),
                    Declaration::FontSize(15.0),
                    Declaration::FontWeight(FontWeight::Bold),
                    Declaration::Color(color_from_argb(theme.chrome_text)),
                ],
            ),
            Rule::new(
                Selector::has(Description::ShowcaseStateStrip),
                alloc::vec![
                    Declaration::FlexDirection(FlexDirection::Row),
                    Declaration::AlignItems(AlignItems::Center),
                    Declaration::JustifyContent(JustifyContent::Start),
                    Declaration::Gap(8.0),
                ],
            ),
            Rule::new(
                Selector::has(Description::ShowcaseSwatch),
                alloc::vec![
                    Declaration::Width(76.0),
                    Declaration::Height(36.0),
                    Declaration::Padding(8.0),
                    Declaration::BackgroundColor(color_from_argb(theme.button_top)),
                    Declaration::Color(color_from_argb(theme.chrome_text)),
                    Declaration::BorderWidth(1.0),
                ],
            ),
            Rule::new(
                Selector::has(Description::ShowcaseSwatch).and(Selector::state(State::Active)),
                alloc::vec![Declaration::BackgroundColor(color_from_argb(theme.focus_accent))],
            ),
            Rule::new(
                Selector::has(Description::ShowcaseSwatch).and(Selector::state(State::Focus)),
                alloc::vec![Declaration::OutlineColor(color_from_argb(theme.edge_light))],
            ),
            Rule::new(
                Selector::has(Description::ShowcaseSwatch).and(Selector::state(State::Disabled)),
                alloc::vec![
                    Declaration::BackgroundColor(color_from_argb(theme.frame_fill_inactive)),
                    Declaration::Color(color_from_argb(theme.chrome_text_inactive)),
                ],
            ),
        ]
    }
}

impl Default for PetalsShowcase {
    fn default() -> Self {
        Self::new(default_theme())
    }
}

fn app(name: &str, path: &str, glyph: &str) -> ApplicationEntry {
    ApplicationEntry {
        name: String::from(name),
        path: String::from(path),
        glyph: String::from(glyph),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_all_showcase_component_trees() {
        let showcase = PetalsShowcase::default();
        let (mut clock_tree, _) = showcase.clock_tree().unwrap();
        showcase
            .prepare_component(
                &mut clock_tree,
                220,
                120,
                JustifyContent::Center,
                AlignItems::Center,
            )
            .unwrap();

        let (mut calc_tree, calc_nodes) = showcase.calculator_tree().unwrap();
        showcase
            .prepare_component(&mut calc_tree, 240, 360, JustifyContent::Start, AlignItems::Stretch)
            .unwrap();
        assert_eq!(calc_nodes.keys.len(), 20);

        let (launcher_tree, launcher_nodes) = showcase.launcher_tree().unwrap();
        assert_eq!(launcher_nodes.tiles.len(), 4);
        assert!(launcher_tree.node(launcher_nodes.tiles[0].icon).is_some());

        let (_, chrome_nodes) = showcase.chrome_tree().unwrap();
        assert_eq!(chrome_nodes.resize_edges.len(), 4);

        let (state_tree, state_nodes) = showcase.stile_state_tree().unwrap();
        assert!(state_tree.node(state_nodes.active).unwrap().states.contains_state(State::Active));
    }
}
