//! Debug overlay drawing for petals UI trees.
//!
//! Provides [`draw_layout_debug`], which renders colored bounding-box borders
//! and tint fills over every node in a [`UiTree`].  Intended for developer
//! tooling only; has zero runtime cost when the call site is disabled.
//!
//! ## Visual encoding
//!
//! | Visual                  | Meaning                                          |
//! |-------------------------|--------------------------------------------------|
//! | Depth-coded border      | Layout bounds for every node                     |
//! | Blue tint fill          | Clip/container regions                           |
//! | Yellow/amber border     | Hit-testable regions (Pressable, ResizeEdge)     |
//! | Nesting level → color   | Helps distinguish sibling vs parent/child groups |

use alloc::vec::Vec;

use crate::description::Description;
use crate::node::NodeId;
use crate::UiTree;

/// Per-depth border colors (ARGB).  The palette cycles for trees deeper than 8
/// levels.
pub const DEPTH_COLORS: [u32; 8] = [
    0xCC4488FF, // depth 0 – blue
    0xCC00DDAA, // depth 1 – cyan-green
    0xCCFFCC00, // depth 2 – amber
    0xCCFF8800, // depth 3 – orange
    0xCCFF4488, // depth 4 – pink
    0xCCAA44FF, // depth 5 – violet
    0xCC44FFB8, // depth 6 – mint
    0xCCFF4444, // depth 7 – red
];

/// Bright border for hit-testable (Pressable / ResizeEdge) nodes.
pub const HIT_REGION_COLOR: u32 = 0xEEFFEE00;

/// Faint tint fill for container / clip nodes.
const CLIP_FILL_COLOR: u32 = 0x220066CC;

/// Draw visual debug information for every node in `tree`.
///
/// Colored bounding-box borders are drawn directly into `dst` (BGRA8888,
/// `stride` pixels wide, `height` pixels tall).  Color is depth-coded;
/// `Pressable` and `ResizeEdge` nodes receive a distinct yellow highlight;
/// container / clip nodes receive a faint blue fill.
pub fn draw_layout_debug(tree: &UiTree, dst: &mut [u32], stride: u32, height: u32) {
    let nodes = tree.nodes();
    if nodes.is_empty() {
        return;
    }

    // ── Build a depth table indexed by NodeId (BFS from root). ────────────────
    let mut depths: Vec<u32> = alloc::vec![0u32; nodes.len()];
    let root = tree.root();
    let mut queue: Vec<(NodeId, u32)> = alloc::vec![(root, 0)];
    while !queue.is_empty() {
        let (id, d) = queue.remove(0);
        if let Some(slot) = depths.get_mut(id as usize) {
            *slot = d;
        }
        if let Some(node) = tree.node(id) {
            for &child in &node.children {
                queue.push((child, d + 1));
            }
        }
    }

    // ── Draw each node. ───────────────────────────────────────────────────────
    for node in nodes {
        let Ok(b) = tree.global_layout_box(node.id) else {
            continue;
        };
        if b.width < 1.0 || b.height < 1.0 {
            continue;
        }

        let x = b.x as i32;
        let y = b.y as i32;
        let w = b.width.max(0.0) as u32;
        let h = b.height.max(0.0) as u32;

        let is_hit = node.descriptions.contains(&Description::Pressable)
            || node.descriptions.contains(&Description::ResizeEdge);

        let is_container = node.descriptions.contains(&Description::Container)
            || node.descriptions.contains(&Description::WindowChrome)
            || node.descriptions.contains(&Description::WindowContent)
            || node.descriptions.contains(&Description::ApplicationGrid)
            || node.descriptions.contains(&Description::ApplicationRow)
            || node.descriptions.contains(&Description::Titlebar);

        // Clip/container nodes get a faint background tint to mark the region.
        if is_container {
            fill_rect_alpha(dst, stride, height, x, y, w, h, CLIP_FILL_COLOR);
        }

        // Border color: hit regions are always amber; others use depth coding.
        let depth = depths.get(node.id as usize).copied().unwrap_or(0) as usize;
        let border_color =
            if is_hit { HIT_REGION_COLOR } else { DEPTH_COLORS[depth % DEPTH_COLORS.len()] };

        draw_border(dst, stride, height, x, y, w, h, border_color);
    }
}

// ── Pixel helpers ─────────────────────────────────────────────────────────────

/// Alpha-blend a single ARGB `color` pixel over the destination pixel at
/// `(px, py)` in `dst` (BGRA8888 layout, `stride` pixels wide).
#[inline(always)]
fn blend_pixel(dst: &mut [u32], stride: u32, height: u32, px: i32, py: i32, color: u32) {
    if px < 0 || py < 0 {
        return;
    }
    let px = px as u32;
    let py = py as u32;
    if px >= stride || py >= height {
        return;
    }
    let idx = (py * stride + px) as usize;
    if idx >= dst.len() {
        return;
    }
    let src_a = (color >> 24) & 0xFF;
    if src_a == 0 {
        return;
    }
    if src_a == 255 {
        dst[idx] = color;
        return;
    }
    let inv = 255 - src_a;
    let dst_px = dst[idx];
    let r = ((color >> 16 & 0xFF) * src_a + (dst_px >> 16 & 0xFF) * inv) / 255;
    let g = ((color >> 8 & 0xFF) * src_a + (dst_px >> 8 & 0xFF) * inv) / 255;
    let b = ((color & 0xFF) * src_a + (dst_px & 0xFF) * inv) / 255;
    dst[idx] = (255 << 24) | (r << 16) | (g << 8) | b;
}

/// Alpha-blend a filled rectangle with ARGB `color`.
fn fill_rect_alpha(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    color: u32,
) {
    let src_a = (color >> 24) & 0xFF;
    if src_a == 0 || w == 0 || h == 0 {
        return;
    }
    let x0 = x.max(0) as u32;
    let y0 = y.max(0) as u32;
    let x1 = (x + w as i32).max(0) as u32;
    let y1 = (y + h as i32).max(0) as u32;
    let x1 = x1.min(stride);
    let y1 = y1.min(height);
    if x1 <= x0 || y1 <= y0 {
        return;
    }
    let inv = 255 - src_a;
    let src_r = (color >> 16) & 0xFF;
    let src_g = (color >> 8) & 0xFF;
    let src_b = color & 0xFF;
    for py in y0..y1 {
        let row = (py * stride) as usize;
        for px in x0..x1 {
            let idx = row + px as usize;
            if idx >= dst.len() {
                break;
            }
            let dst_px = dst[idx];
            let r = (src_r * src_a + (dst_px >> 16 & 0xFF) * inv) / 255;
            let g = (src_g * src_a + (dst_px >> 8 & 0xFF) * inv) / 255;
            let b = (src_b * src_a + (dst_px & 0xFF) * inv) / 255;
            dst[idx] = (255 << 24) | (r << 16) | (g << 8) | b;
        }
    }
}

/// Draw a 1-pixel border rectangle with ARGB `color` (alpha-blended).
fn draw_border(
    dst: &mut [u32],
    stride: u32,
    height: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    color: u32,
) {
    if w == 0 || h == 0 {
        return;
    }
    let x1 = x + w as i32 - 1;
    let y1 = y + h as i32 - 1;

    // Top and bottom rows.
    for px in x..=x1 {
        blend_pixel(dst, stride, height, px, y, color);
        blend_pixel(dst, stride, height, px, y1, color);
    }
    // Left and right columns (excluding corners already drawn above).
    for py in (y + 1)..y1 {
        blend_pixel(dst, stride, height, x, py, color);
        blend_pixel(dst, stride, height, x1, py, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::description::Description;
    use crate::layout::UiTree;
    use stile::{Declaration, Rule, Selector};

    #[test]
    fn draw_layout_debug_does_not_panic_on_empty_tree() {
        let tree = UiTree::new().unwrap();
        let mut pixels = alloc::vec![0u32; 100 * 100];
        draw_layout_debug(&tree, &mut pixels, 100, 100);
    }

    #[test]
    fn draw_layout_debug_marks_pressable_nodes() {
        let mut tree = UiTree::new().unwrap();
        let btn = tree.pressable("OK").unwrap();
        tree.add_child(tree.root(), btn).unwrap();

        let rules = alloc::vec![
            Rule::new(
                Selector::has(Description::Container),
                alloc::vec![
                    Declaration::Width(80.0),
                    Declaration::Height(40.0),
                    Declaration::FlexDirection(stile::FlexDirection::Row),
                ],
            ),
            Rule::new(
                Selector::has(Description::Pressable),
                alloc::vec![Declaration::Width(60.0), Declaration::Height(30.0)],
            ),
        ];
        tree.restyle(&rules).unwrap();
        tree.compute_layout(crate::layout::Size {
            width: crate::layout::AvailableSpace::Definite(100.0),
            height: crate::layout::AvailableSpace::Definite(100.0),
        })
        .unwrap();

        let mut pixels = alloc::vec![0u32; 100 * 100];
        draw_layout_debug(&tree, &mut pixels, 100, 100);
        // At least some pixels should have been painted (non-zero).
        assert!(pixels.iter().any(|&p| p != 0));
    }
}
