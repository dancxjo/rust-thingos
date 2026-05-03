use petals::{
    AlignItems, AvailableSpace, Declaration, Description, FlexDirection, Rule, Selector, Size,
    UiTree,
};

use crate::Rect;

const CASCADE_START_X: i32 = 40;
const CASCADE_START_Y: i32 = 48;
const CASCADE_STEP_X: i32 = 36;
const CASCADE_STEP_Y: i32 = 32;
const CASCADE_SLOTS: i32 = 8;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SurfaceChrome {
    pub titlebar_height: i32,
    pub frame_thickness: i32,
}

impl SurfaceChrome {
    pub fn is_empty(self) -> bool {
        self.titlebar_height <= 0 && self.frame_thickness <= 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HitTarget {
    Client { surface_id: u32 },
    TitleBar { surface_id: u32 },
    ChromeButton { surface_id: u32, button: ChromeButton },
    Frame { surface_id: u32, edge: ResizeEdge },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChromeButton {
    Minimize,
    Shade,
    Maximize,
    Fullscreen,
    Close,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizeEdge {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CursorKind {
    #[default]
    Default,
    Move,
    ResizeNorth,
    ResizeSouth,
    ResizeEast,
    ResizeWest,
    ResizeNorthEast,
    ResizeNorthWest,
    ResizeSouthEast,
    ResizeSouthWest,
}

impl CursorKind {
    pub fn for_resize_edge(edge: ResizeEdge) -> Self {
        match edge {
            ResizeEdge::North => Self::ResizeNorth,
            ResizeEdge::South => Self::ResizeSouth,
            ResizeEdge::East => Self::ResizeEast,
            ResizeEdge::West => Self::ResizeWest,
            ResizeEdge::NorthEast => Self::ResizeNorthEast,
            ResizeEdge::NorthWest => Self::ResizeNorthWest,
            ResizeEdge::SouthEast => Self::ResizeSouthEast,
            ResizeEdge::SouthWest => Self::ResizeSouthWest,
        }
    }
}

pub struct SurfaceMove {
    pub old_rect: Rect,
    pub new_rect: Rect,
    pub changed: bool,
}

pub struct SurfaceResize {
    pub old_rect: Rect,
    pub new_rect: Rect,
    pub changed: bool,
}

pub struct SurfaceToggle {
    pub old_rect: Rect,
    pub new_rect: Rect,
    pub changed: bool,
    pub active: bool,
}

pub fn rect_overlaps(a: Rect, b: Rect) -> bool {
    if a.x >= b.x + b.w || b.x >= a.x + a.w {
        return false;
    }
    if a.y >= b.y + b.h || b.y >= a.y + a.h {
        return false;
    }
    true
}

pub fn rect_center_dist_sq(a: Rect, b: Rect) -> i64 {
    let cx_a = (a.x + a.w / 2) as i64;
    let cy_a = (a.y + a.h / 2) as i64;
    let cx_b = (b.x + b.w / 2) as i64;
    let cy_b = (b.y + b.h / 2) as i64;
    let dx = cx_a - cx_b;
    let dy = cy_a - cy_b;
    dx * dx + dy * dy
}

pub fn airy_window_rect(
    screen_w: i32,
    screen_h: i32,
    existing_windows: &[Rect],
    width: i32,
    height: i32,
) -> Rect {
    let padding_x = 40;
    let padding_y = 60;

    if existing_windows.is_empty() {
        return Rect { x: padding_x, y: padding_y, w: width, h: height };
    }

    let mut candidates = alloc::vec::Vec::new();

    candidates.push((padding_x, padding_y));
    candidates.push(((screen_w - width - padding_x).max(0), padding_y));
    candidates.push((padding_x, (screen_h - height - padding_x).max(0)));
    candidates
        .push(((screen_w - width - padding_x).max(0), (screen_h - height - padding_x).max(0)));

    candidates.push(((screen_w - width).max(0) / 2, (screen_h - height + padding_y).max(0) / 2));

    candidates.push(((screen_w - width).max(0) / 2, padding_y));
    candidates.push(((screen_w - width).max(0) / 2, (screen_h - height - padding_x).max(0)));
    candidates.push((padding_x, (screen_h - height).max(0) / 2));
    candidates.push(((screen_w - width - padding_x).max(0), (screen_h - height).max(0) / 2));

    let mut best_pos = candidates[0];
    let mut max_min_dist_sq = -1i64;

    for (cx, cy) in candidates {
        let cand = Rect { x: cx, y: cy, w: width, h: height };
        let mut overlaps = false;
        let mut min_dist_sq = core::i64::MAX;

        for w in existing_windows {
            if rect_overlaps(cand, *w) {
                overlaps = true;
                break;
            }

            let d_sq = rect_center_dist_sq(cand, *w);
            if d_sq < min_dist_sq {
                min_dist_sq = d_sq;
            }
        }

        if overlaps {
            continue;
        }

        if min_dist_sq > max_min_dist_sq {
            max_min_dist_sq = min_dist_sq;
            best_pos = (cx, cy);
        }
    }

    if max_min_dist_sq >= 0 {
        Rect { x: best_pos.0, y: best_pos.1, w: width, h: height }
    } else {
        let slot = (existing_windows.len() as i32) % CASCADE_SLOTS;
        Rect {
            x: CASCADE_START_X + (slot * CASCADE_STEP_X),
            y: CASCADE_START_Y + (slot * CASCADE_STEP_Y),
            w: width,
            h: height,
        }
    }
}

pub fn cascaded_window_rect(existing_standalone_windows: u32, width: i32, height: i32) -> Rect {
    let slot = (existing_standalone_windows as i32) % CASCADE_SLOTS;
    Rect {
        x: CASCADE_START_X + (slot * CASCADE_STEP_X),
        y: CASCADE_START_Y + (slot * CASCADE_STEP_Y),
        w: width,
        h: height,
    }
}

pub fn surface_visual_rect(rect: Rect, chrome: SurfaceChrome) -> Rect {
    if chrome.is_empty() {
        return rect;
    }
    let t = chrome.frame_thickness.max(0);
    let titlebar_height = chrome.titlebar_height.max(0);
    let x0 = (rect.x - t).max(0);
    let y0 = (rect.y - titlebar_height).max(0);
    let x1 = rect.x + rect.w + t;
    let y1 = rect.y + rect.h + t;
    Rect { x: x0, y: y0, w: (x1 - x0).max(0), h: (y1 - y0).max(0) }
}

pub fn resize_edge_at(rect: Rect, chrome: SurfaceChrome, x: i32, y: i32) -> Option<ResizeEdge> {
    let t = chrome.frame_thickness.max(0);
    if chrome.is_empty() || t <= 0 {
        return None;
    }

    let visual = surface_visual_rect(rect, chrome);
    let vx0 = visual.x;
    let vy0 = visual.y;
    let vx1 = visual.x + visual.w;
    let vy1 = visual.y + visual.h;
    if x < vx0 || x >= vx1 || y < vy0 || y >= vy1 {
        return None;
    }
    if x >= rect.x && x < rect.x + rect.w && y >= rect.y && y < rect.y + rect.h {
        return None;
    }

    let north = y < visual.y + t;
    let south = y >= rect.y + rect.h;
    let west = x < rect.x;
    let east = x >= rect.x + rect.w;

    match (north, south, west, east) {
        (true, _, true, _) => Some(ResizeEdge::NorthWest),
        (true, _, _, true) => Some(ResizeEdge::NorthEast),
        (_, true, true, _) => Some(ResizeEdge::SouthWest),
        (_, true, _, true) => Some(ResizeEdge::SouthEast),
        (true, _, _, _) => Some(ResizeEdge::North),
        (_, true, _, _) => Some(ResizeEdge::South),
        (_, _, true, _) => Some(ResizeEdge::West),
        (_, _, _, true) => Some(ResizeEdge::East),
        _ => None,
    }
}

pub fn chrome_button_rects(rect: Rect, chrome: SurfaceChrome) -> Option<[(ChromeButton, Rect); 3]> {
    if chrome.titlebar_height <= 0 || rect.w <= 0 || rect.h <= 0 {
        return None;
    }

    let visual = surface_visual_rect(rect, chrome);
    let frame = chrome.frame_thickness.max(0).min(visual.w / 2).min(visual.h / 2);
    let titlebar_height = chrome.titlebar_height.max(0).min(visual.h);
    let button_height = 44.min(titlebar_height);
    let button_width = 44;
    let spacing = 6;
    let right_inset = frame + 8;
    if button_height < 24 || button_width < 24 {
        return None;
    }

    let right = visual.x + visual.w - right_inset;
    let total_w = button_width * 3 + spacing * 2;
    if total_w + right_inset > visual.w {
        return None;
    }

    let left = right - total_w;
    let mut tree = UiTree::new().ok()?;
    let root = tree.root();
    if let Some(root) = tree.node_mut(root) {
        root.descriptions.push(Description::Titlebar);
    }
    let shade = tree.add_node(&[Description::Pressable, Description::ChromeButton]).ok()?;
    let fullscreen = tree.add_node(&[Description::Pressable, Description::ChromeButton]).ok()?;
    let close = tree.add_node(&[Description::Pressable, Description::ChromeButton]).ok()?;
    tree.add_child(tree.root(), shade).ok()?;
    tree.add_child(tree.root(), fullscreen).ok()?;
    tree.add_child(tree.root(), close).ok()?;

    let rules = alloc::vec![
        Rule::new(
            Selector::has(Description::Container),
            alloc::vec![
                Declaration::Width(total_w as f32),
                Declaration::Height(titlebar_height as f32),
                Declaration::FlexDirection(FlexDirection::Row),
                Declaration::AlignItems(AlignItems::Center),
                Declaration::Gap(spacing as f32),
            ],
        ),
        Rule::new(
            Selector::has(Description::ChromeButton),
            alloc::vec![
                Declaration::Width(button_width as f32),
                Declaration::Height(button_height as f32),
            ],
        ),
    ];

    tree.restyle(&rules).ok()?;
    tree.compute_layout(Size {
        width: AvailableSpace::Definite(total_w as f32),
        height: AvailableSpace::Definite(titlebar_height as f32),
    })
    .ok()?;

    let shade_box = tree.layout_box(shade).ok()?;
    let fullscreen_box = tree.layout_box(fullscreen).ok()?;
    let close_box = tree.layout_box(close).ok()?;

    let mk_rect = |b: petals::LayoutBox| Rect {
        x: left + b.x as i32,
        y: visual.y + b.y as i32,
        w: b.width as i32,
        h: b.height as i32,
    };

    Some([
        (ChromeButton::Shade, mk_rect(shade_box)),
        (ChromeButton::Fullscreen, mk_rect(fullscreen_box)),
        (ChromeButton::Close, mk_rect(close_box)),
    ])
}

pub fn chrome_button_at(rect: Rect, chrome: SurfaceChrome, x: i32, y: i32) -> Option<ChromeButton> {
    let rects = chrome_button_rects(rect, chrome)?;
    for (button, bounds) in rects {
        let x0 = bounds.x;
        let y0 = bounds.y;
        let x1 = bounds.x + bounds.w;
        let y1 = bounds.y + bounds.h;
        if x >= x0 && x < x1 && y >= y0 && y < y1 {
            return Some(button);
        }
    }
    None
}

pub fn translate_surface_damage(local: Rect, src_w: i32, src_h: i32, dest: Rect) -> Option<Rect> {
    if src_w <= 0 || src_h <= 0 || dest.w <= 0 || dest.h <= 0 || local.w <= 0 || local.h <= 0 {
        return None;
    }

    let x0 = local.x.min(src_w);
    let y0 = local.y.min(src_h);
    let x1 = (local.x + local.w).min(src_w);
    let y1 = (local.y + local.h).min(src_h);
    if x1 <= x0 || y1 <= y0 {
        return None;
    }

    let dx0 = dest.x + ((x0 as i64 * dest.w as i64) / src_w as i64) as i32;
    let dy0 = dest.y + ((y0 as i64 * dest.h as i64) / src_h as i64) as i32;
    let dx1 = dest.x + (((x1 as i64 * dest.w as i64) + src_w as i64 - 1) / src_w as i64) as i32;
    let dy1 = dest.y + (((y1 as i64 * dest.h as i64) + src_h as i64 - 1) / src_h as i64) as i32;

    if dx1 <= dx0 || dy1 <= dy0 {
        None
    } else {
        Some(Rect { x: dx0, y: dy0, w: dx1 - dx0, h: dy1 - dy0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chrome_geometry_reserves_space_around_client_content() {
        let content = Rect { x: 44, y: 88, w: 320, h: 200 };
        let chrome = SurfaceChrome { titlebar_height: 44, frame_thickness: 4 };

        assert_eq!(surface_visual_rect(content, chrome), Rect { x: 40, y: 44, w: 328, h: 248 });

        let buttons = chrome_button_rects(content, chrome).expect("chrome buttons");
        assert!(buttons.iter().all(|(_, rect)| rect.y >= 44 && rect.y < content.y));
        assert!(buttons.iter().all(|(_, rect)| rect.w >= 44 && rect.h >= 44));
        assert_eq!(
            chrome_button_at(content, chrome, buttons[2].1.x + 1, buttons[2].1.y + 1),
            Some(ChromeButton::Close)
        );

        assert_eq!(resize_edge_at(content, chrome, 200, 100), None);
        assert_eq!(resize_edge_at(content, chrome, 42, 120), Some(ResizeEdge::West));
        assert_eq!(resize_edge_at(content, chrome, 200, 290), Some(ResizeEdge::South));
    }
}
