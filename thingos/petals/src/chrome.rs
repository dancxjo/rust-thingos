use alloc::vec::Vec;

use stile::typography::{SCALE_BODY, SPACE_MD, SPACE_SM};

use crate::{
    AlignItems, Color, Declaration, Description, FlexDirection, FontWeight, JustifyContent, NodeId,
    PetalsEvent, PetalsService, Rule, Selector, ServiceAction, State, TaffyError, Theme, UiTree,
    default_theme,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowChromeNodes {
    pub frame: NodeId,
    pub titlebar: NodeId,
    pub title: NodeId,
    pub minimize: NodeId,
    pub maximize: NodeId,
    pub close: NodeId,
    pub content: NodeId,
    pub resize_edges: [NodeId; 4],
}

pub fn window_chrome_tree(title: &str) -> Result<(UiTree, WindowChromeNodes), TaffyError> {
    let mut tree = UiTree::new()?;
    let root = tree.root();
    let nodes = render_window_chrome(&mut tree, root, title)?;
    Ok((tree, nodes))
}

pub fn render_window_chrome(
    tree: &mut UiTree,
    parent: NodeId,
    title: &str,
) -> Result<WindowChromeNodes, TaffyError> {
    let frame = tree.add_node(&[Description::WindowChrome, Description::WindowFrame])?;
    let titlebar = tree.add_node(&[Description::Titlebar, Description::Container])?;
    let title = tree.text(title)?;
    let minimize = tree.pressable("Minimize")?;
    let maximize = tree.pressable("Maximize")?;
    let close = tree.pressable("Close")?;
    let content = tree.add_node(&[Description::WindowContent, Description::Container])?;
    let north = tree.add_node(&[Description::ResizeEdge])?;
    let east = tree.add_node(&[Description::ResizeEdge])?;
    let south = tree.add_node(&[Description::ResizeEdge])?;
    let west = tree.add_node(&[Description::ResizeEdge])?;

    for (node, description) in [
        (title, Description::Title),
        (minimize, Description::ChromeButton),
        (maximize, Description::ChromeButton),
        (close, Description::ChromeButton),
    ] {
        if let Some(node) = tree.node_mut(node) {
            node.descriptions.push(description);
        }
    }

    tree.add_child(parent, frame)?;
    tree.add_child(frame, titlebar)?;
    tree.add_child(frame, content)?;
    tree.add_child(titlebar, title)?;
    tree.add_child(titlebar, minimize)?;
    tree.add_child(titlebar, maximize)?;
    tree.add_child(titlebar, close)?;
    tree.add_child(frame, north)?;
    tree.add_child(frame, east)?;
    tree.add_child(frame, south)?;
    tree.add_child(frame, west)?;

    Ok(WindowChromeNodes {
        frame,
        titlebar,
        title,
        minimize,
        maximize,
        close,
        content,
        resize_edges: [north, east, south, west],
    })
}

pub fn window_chrome_rules() -> Vec<Rule<Description>> {
    window_chrome_rules_for_theme(default_theme())
}

pub fn window_chrome_rules_for_theme(theme: Theme) -> Vec<Rule<Description>> {
    alloc::vec![
        Rule::new(
            Selector::has(Description::WindowChrome),
            alloc::vec![
                Declaration::FlexDirection(FlexDirection::Column),
                Declaration::AlignItems(AlignItems::Stretch),
                Declaration::JustifyContent(JustifyContent::Start),
                Declaration::BackgroundColor(color_from_argb(theme.frame_fill)),
                Declaration::BorderWidth(1.0),
            ],
        ),
        // Titlebar: row layout with title on the left and buttons packed right.
        // JustifyContent::Start combined with FlexGrow on the Title node causes
        // the title to expand and push chrome buttons to the trailing edge.
        Rule::new(
            Selector::has(Description::Titlebar),
            alloc::vec![
                Declaration::FlexDirection(FlexDirection::Row),
                Declaration::AlignItems(AlignItems::Center),
                Declaration::JustifyContent(JustifyContent::Start),
                Declaration::Gap(SPACE_SM),
                Declaration::Padding(SPACE_MD),
                Declaration::Height(44.0),
                Declaration::BackgroundColor(color_from_argb(theme.active.title_top)),
            ],
        ),
        // Window title: chrome metadata, not content.  Normal weight and body
        // size keep it clearly distinct from content headings (H3/H2/H1 which
        // are bold and larger).  FlexGrow fills remaining titlebar space and
        // pushes the chrome buttons to the right.
        Rule::new(
            Selector::has(Description::Title),
            alloc::vec![
                Declaration::MinWidth(160.0),
                Declaration::FlexGrow(1.0),
                Declaration::FontSize(SCALE_BODY),
                Declaration::FontWeight(FontWeight::Normal),
                Declaration::Color(color_from_argb(theme.chrome_text)),
            ],
        ),
        // Chrome buttons: 44×44 px minimum hit target (WCAG 2.5.5 / platform
        // minimum for pointer, keyboard, and touch input).
        Rule::new(
            Selector::has(Description::ChromeButton),
            alloc::vec![
                Declaration::Width(44.0),
                Declaration::Height(44.0),
                Declaration::BackgroundColor(color_from_argb(theme.button_top)),
                Declaration::Color(color_from_argb(theme.control_icon)),
            ],
        ),
        // Hover: subtle tint so pointer position is reflected immediately.
        Rule::new(
            Selector::has(Description::ChromeButton).and(Selector::state(State::Hover)),
            alloc::vec![Declaration::BackgroundColor(color_from_argb(theme.hover_tint))],
        ),
        // Focus ring: visible keyboard-navigation affordance.
        Rule::new(
            Selector::has(Description::ChromeButton).and(Selector::state(State::Focus)),
            alloc::vec![
                Declaration::OutlineColor(color_from_argb(theme.edge_light)),
                Declaration::OutlineWidth(2.0),
            ],
        ),
        // Active/press: dominant fill confirms the action.
        Rule::new(
            Selector::has(Description::ChromeButton).and(Selector::state(State::Active)),
            alloc::vec![Declaration::BackgroundColor(color_from_argb(theme.focus_accent))],
        ),
        Rule::new(
            Selector::has(Description::WindowContent),
            alloc::vec![
                Declaration::MinHeight(66.0),
                Declaration::BackgroundColor(color_from_argb(theme.body_top)),
            ],
        ),
        Rule::new(Selector::has(Description::ResizeEdge), alloc::vec![Declaration::Height(0.0)]),
    ]
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WindowChromeService;

impl PetalsService for WindowChromeService {
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
    use stile::available_themes;

    /// Chrome buttons must expose at least 44×44 px hit targets across every
    /// bundled theme (WCAG 2.5.5 / platform minimum for pointer, keyboard, and
    /// touch input).
    #[test]
    fn chrome_button_rules_have_44px_hit_targets() {
        for theme in available_themes() {
            let rules = window_chrome_rules_for_theme(*theme);
            let button_rule = rules.iter().find(|r| {
                r.declarations.iter().any(|d| matches!(d, Declaration::Width(44.0)))
                    && r.declarations.iter().any(|d| matches!(d, Declaration::Height(44.0)))
                    && r.declarations.iter().any(|d| {
                        matches!(d, Declaration::BackgroundColor(_))
                    })
            });
            assert!(
                button_rule.is_some(),
                "Theme '{}' chrome button rule must declare 44×44 px dimensions",
                theme.name
            );
        }
    }

    /// The window title must grow to fill available space so chrome buttons are
    /// pushed to the trailing edge of the titlebar (title left, buttons right).
    #[test]
    fn chrome_title_has_flex_grow() {
        let rules = window_chrome_rules();
        let has_flex_grow = rules.iter().any(|r| {
            r.declarations.iter().any(|d| matches!(d, Declaration::FlexGrow(v) if *v >= 1.0))
        });
        assert!(has_flex_grow, "A chrome rule must include FlexGrow to push buttons right");
    }

    /// Window title must use normal (not bold) weight to differentiate it
    /// visually from content headings (ShowcaseHeader, H1-H3) which are bold.
    #[test]
    fn chrome_title_uses_normal_weight() {
        let rules = window_chrome_rules();
        let title_rule = rules.iter().find(|r| {
            r.declarations.iter().any(|d| matches!(d, Declaration::FlexGrow(_)))
                && r.declarations.iter().any(|d| matches!(d, Declaration::FontSize(_)))
        });
        assert!(title_rule.is_some(), "Expected a title rule with FlexGrow and FontSize");
        let is_normal = title_rule.unwrap().declarations.iter().any(|d| {
            matches!(d, Declaration::FontWeight(FontWeight::Normal))
        });
        assert!(
            is_normal,
            "Window title must use FontWeight::Normal to distinguish it from bold content headings"
        );
    }

    /// Chrome buttons must expose focus, hover, and active states so keyboard
    /// and pointer users see clear affordance feedback.  Each state must use a
    /// distinct color — hover gets the subtle `hover_tint`, active/press gets
    /// the emphatic `focus_accent`.
    #[test]
    fn chrome_button_rules_have_interactive_states() {
        let theme = default_theme();
        let rules = window_chrome_rules_for_theme(theme);

        let hover_color = color_from_argb(theme.hover_tint);
        let active_color = color_from_argb(theme.focus_accent);
        let focus_outline = color_from_argb(theme.edge_light);

        let has_focus = rules.iter().any(|r| {
            r.declarations.iter().any(|d| matches!(d, Declaration::OutlineWidth(_)))
                && r.declarations.iter().any(|d| {
                    matches!(d, Declaration::OutlineColor(c) if *c == focus_outline)
                })
        });
        let has_hover = rules.iter().any(|r| {
            r.declarations
                .iter()
                .any(|d| matches!(d, Declaration::BackgroundColor(c) if *c == hover_color))
        });
        let has_active = rules.iter().any(|r| {
            r.declarations
                .iter()
                .any(|d| matches!(d, Declaration::BackgroundColor(c) if *c == active_color))
        });

        assert!(has_focus, "Chrome buttons must have a focus state (visible outline ring)");
        assert!(has_hover, "Chrome buttons must have a hover state (hover_tint background)");
        assert!(
            has_active,
            "Chrome buttons must have an active/press state (focus_accent background)"
        );
        assert_ne!(
            hover_color, active_color,
            "Hover and active/press state backgrounds must be visually distinct colors"
        );
    }
}
