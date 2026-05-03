use crate::{Description, NodeId, TaffyError, UiTree};

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

    tree.add_child(tree.root(), frame)?;
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

    Ok((
        tree,
        WindowChromeNodes {
            frame,
            titlebar,
            title,
            minimize,
            maximize,
            close,
            content,
            resize_edges: [north, east, south, west],
        },
    ))
}
