use alloc::vec::Vec;

use stile::typography::{SCALE_BODY, SPACE_MD, SPACE_SM};

use crate::{
    AlignItems, Color, Declaration, Description, FlexDirection, FontWeight, JustifyContent, NodeId,
    PetalsEvent, PetalsService, Rule, Selector, ServiceAction, TaffyError, Theme, UiTree,
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
        Rule::new(
            Selector::has(Description::Titlebar),
            alloc::vec![
                Declaration::FlexDirection(FlexDirection::Row),
                Declaration::AlignItems(AlignItems::Center),
                Declaration::JustifyContent(JustifyContent::End),
                Declaration::Gap(SPACE_SM),
                Declaration::Padding(SPACE_MD),
                Declaration::Height(44.0),
                Declaration::BackgroundColor(color_from_argb(theme.active.title_top)),
            ],
        ),
        Rule::new(
            Selector::has(Description::Title),
            alloc::vec![
                Declaration::MinWidth(160.0),
                Declaration::FontSize(SCALE_BODY),
                Declaration::FontWeight(FontWeight::Bold),
                Declaration::Color(color_from_argb(theme.chrome_text)),
            ],
        ),
        Rule::new(
            Selector::has(Description::ChromeButton),
            alloc::vec![
                Declaration::Width(44.0),
                Declaration::Height(36.0),
                Declaration::BackgroundColor(color_from_argb(theme.button_top)),
                Declaration::Color(color_from_argb(theme.control_icon)),
            ],
        ),
        Rule::new(
            Selector::has(Description::WindowContent),
            alloc::vec![
                Declaration::MinHeight(66.0),
                Declaration::BackgroundColor(color_from_argb(theme.body_top)),
            ],
        ),
        Rule::new(Selector::has(Description::ResizeEdge), alloc::vec![Declaration::Height(0.0)],),
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
