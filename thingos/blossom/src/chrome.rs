use alloc::vec::Vec;

use stile::{ChromeStyle, UiTheme};

use crate::Rect;
use crate::wm::{ChromeButton, SurfaceChrome, chrome_button_rects, surface_visual_rect};

pub const WINDOW_ICON_SHADE_PATH: &str = "/public/icons/lucide/chevron-up.svg";
pub const WINDOW_ICON_UNSHADE_PATH: &str = "/public/icons/lucide/chevron-down.svg";
pub const WINDOW_ICON_FULLSCREEN_PATH: &str = "/public/icons/lucide/fullscreen.svg";
pub const WINDOW_ICON_RESTORE_PATH: &str = "/public/icons/lucide/minimize-2.svg";
pub const WINDOW_ICON_CLOSE_PATH: &str = "/public/icons/lucide/x.svg";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChromeDrawCommand<'a> {
    FillRect { rect: Rect, color: u32 },
    VerticalGradient { rect: Rect, top: u32, bottom: u32 },
    HorizontalGradient { rect: Rect, left: u32, center: u32, right: u32 },
    StrokeRect { rect: Rect, thickness: i32, color: u32 },
    Text { x: i32, y: i32, px_size_bits: u32, text: &'a str, color: u32 },
    ControlGlyph { rect: Rect, button: ChromeButton, icon_path: Option<&'static str>, color: u32 },
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChromeDrawPlan<'a> {
    pub commands: Vec<ChromeDrawCommand<'a>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChromeVisualState {
    pub active: bool,
    pub hovered: bool,
    pub primary_button_down: bool,
    pub pointer_x: i32,
    pub pointer_y: i32,
    pub shaded: bool,
    pub fullscreen: bool,
}

pub fn window_chrome_plan<'a>(
    rect: Rect,
    chrome: SurfaceChrome,
    theme: UiTheme,
    state: ChromeVisualState,
    title: Option<&'a str>,
) -> ChromeDrawPlan<'a> {
    let mut plan = ChromeDrawPlan { commands: Vec::new() };
    if chrome.is_empty() || rect.w <= 0 || rect.h <= 0 {
        return plan;
    }

    let visual_rect = surface_visual_rect(rect, chrome);
    let frame = chrome.frame_thickness.max(0).min(visual_rect.w / 2).min(visual_rect.h / 2);
    if frame <= 0 {
        return plan;
    }

    let titlebar_height =
        chrome.titlebar_height.max(0).min(theme.titlebar_height as i32).min(visual_rect.h);
    match theme.chrome_style {
        ChromeStyle::Facet => {
            push_facet_frame(&mut plan, visual_rect, frame, titlebar_height, state.active, theme)
        }
        ChromeStyle::Ghost => push_ghost_frame(
            &mut plan,
            visual_rect,
            frame,
            titlebar_height,
            state.active,
            state.hovered,
            theme,
        ),
    }

    if titlebar_height > 0 {
        push_chrome_buttons(&mut plan, rect, chrome, theme, state);
        if let Some(title) = title {
            push_title(
                &mut plan,
                visual_rect,
                rect,
                chrome,
                frame,
                titlebar_height,
                theme,
                state.active,
                title,
            );
        }
    }

    plan
}

fn push_facet_frame(
    plan: &mut ChromeDrawPlan<'_>,
    rect: Rect,
    frame: i32,
    titlebar_height: i32,
    active: bool,
    theme: UiTheme,
) {
    let fill_top = if active { theme.frame_fill } else { theme.frame_fill_inactive };
    let fill_bottom =
        if active { theme.frame_fill_bottom } else { theme.frame_fill_bottom_inactive };
    let inner = if active { theme.inner_stroke } else { theme.inner_stroke_inactive };
    let facet = if active { theme.facet } else { theme.facet_inactive };
    let state = if active { theme.active } else { theme.inactive };

    if titlebar_height > 0 {
        plan.commands.push(ChromeDrawCommand::VerticalGradient {
            rect: Rect { h: titlebar_height, ..rect },
            top: state.title_top,
            bottom: state.title_bottom,
        });
        if active && titlebar_height > 6 && rect.w > frame * 2 {
            plan.commands.push(ChromeDrawCommand::HorizontalGradient {
                rect: Rect { x: rect.x + frame, y: rect.y + 2, w: rect.w - frame * 2, h: 2 },
                left: state.title_top,
                center: theme.title_sheen,
                right: state.title_top,
            });
        }
    } else {
        plan.commands.push(ChromeDrawCommand::VerticalGradient {
            rect: Rect { h: frame, ..rect },
            top: fill_top,
            bottom: fill_bottom,
        });
    }

    plan.commands.push(ChromeDrawCommand::VerticalGradient {
        rect: Rect { w: frame, ..rect },
        top: fill_top,
        bottom: fill_bottom,
    });
    plan.commands.push(ChromeDrawCommand::VerticalGradient {
        rect: Rect { x: rect.x + rect.w - frame, w: frame, ..rect },
        top: fill_top,
        bottom: fill_bottom,
    });
    plan.commands.push(ChromeDrawCommand::HorizontalGradient {
        rect: Rect { y: rect.y + rect.h - frame, h: frame, ..rect },
        left: fill_bottom,
        center: fill_top,
        right: fill_bottom,
    });
    plan.commands.push(ChromeDrawCommand::StrokeRect {
        rect,
        thickness: 2,
        color: theme.outer_stroke,
    });

    push_frame_bevel(plan, rect, frame, active, theme);
    push_title_separator(plan, rect, frame, titlebar_height, active, theme);
    push_content_edges(plan, rect, frame, titlebar_height, inner);
    push_corner_facets(plan, rect, facet);
    if active {
        push_top_glow_strip(plan, rect.x + 2, rect.y, rect.w.saturating_sub(4), theme);
        plan.commands.push(ChromeDrawCommand::FillRect {
            rect: Rect::new(rect.x + rect.w - 4, rect.y + 2, 2, 2),
            color: theme.focus_accent,
        });
        plan.commands.push(ChromeDrawCommand::FillRect {
            rect: Rect::new(rect.x + frame, rect.y + rect.h - frame - 2, 2, 2),
            color: theme.focus_accent,
        });
    }
}

fn push_ghost_frame(
    plan: &mut ChromeDrawPlan<'_>,
    rect: Rect,
    frame: i32,
    titlebar_height: i32,
    active: bool,
    hovered: bool,
    theme: UiTheme,
) {
    let state = if active { theme.active } else { theme.inactive };
    if titlebar_height > 0 && (active || hovered) {
        plan.commands.push(ChromeDrawCommand::VerticalGradient {
            rect: Rect { h: titlebar_height, ..rect },
            top: state.title_top,
            bottom: state.title_bottom,
        });
        plan.commands.push(ChromeDrawCommand::FillRect {
            rect: Rect::new(rect.x, rect.y + titlebar_height - 1, rect.w, 1),
            color: if active { theme.title_rule_active } else { theme.title_rule_inactive },
        });
    }
    if active {
        push_top_glow_strip(plan, rect.x + 2, rect.y, rect.w.saturating_sub(4), theme);
    }
    if hovered {
        let inner = if active { theme.inner_stroke } else { theme.inner_stroke_inactive };
        plan.commands.push(ChromeDrawCommand::StrokeRect {
            rect,
            thickness: 1,
            color: theme.outer_stroke,
        });
        push_content_edges(plan, rect, frame, titlebar_height, inner);
    }
}

fn push_frame_bevel(
    plan: &mut ChromeDrawPlan<'_>,
    rect: Rect,
    frame: i32,
    active: bool,
    theme: UiTheme,
) {
    if rect.w < 6 || rect.h < 6 || frame <= 0 {
        return;
    }
    let light = if active { theme.frame_bevel_light } else { theme.frame_bevel_light_inactive };
    let shadow = theme.frame_bevel_shadow;
    let mid = if active { theme.edge_light } else { theme.inner_stroke_inactive };
    let inner_x = rect.x + frame.min(rect.w / 2);
    let inner_y = rect.y + frame.min(rect.h / 2);
    let inner_w = rect.w - frame * 2;
    let inner_h = rect.h - frame * 2;

    push_fill(plan, rect.x + 2, rect.y + 2, rect.w - 4, 1, light);
    push_fill(plan, rect.x + 2, rect.y + 2, 1, rect.h - 4, mid);
    push_fill(plan, rect.x + 2, rect.y + rect.h - 3, rect.w - 4, 1, shadow);
    push_fill(plan, rect.x + rect.w - 3, rect.y + 2, 1, rect.h - 4, shadow);
    if inner_w > 4 && inner_h > 4 {
        push_fill(plan, inner_x, inner_y, inner_w, 1, shadow);
        push_fill(plan, inner_x, inner_y, 1, inner_h, shadow);
        push_fill(plan, inner_x, inner_y + inner_h - 1, inner_w, 1, light);
        push_fill(plan, inner_x + inner_w - 1, inner_y, 1, inner_h, light);
    }
}

fn push_title_separator(
    plan: &mut ChromeDrawPlan<'_>,
    rect: Rect,
    frame: i32,
    titlebar_height: i32,
    active: bool,
    theme: UiTheme,
) {
    if titlebar_height <= 0 || titlebar_height >= rect.h || rect.w <= frame * 2 {
        return;
    }
    let sep_y = rect.y + titlebar_height - 1;
    push_fill(
        plan,
        rect.x + frame,
        sep_y - 1,
        rect.w - frame * 2,
        1,
        if active { theme.frame_bevel_light } else { theme.frame_bevel_light_inactive },
    );
    push_fill(
        plan,
        rect.x + frame,
        sep_y,
        rect.w - frame * 2,
        1,
        if active { theme.title_rule_active } else { theme.title_rule_inactive },
    );
}

fn push_content_edges(
    plan: &mut ChromeDrawPlan<'_>,
    rect: Rect,
    frame: i32,
    titlebar_height: i32,
    color: u32,
) {
    let body_y = rect.y + titlebar_height;
    let body_h = rect.h - titlebar_height - frame;
    if body_h <= 0 || rect.w <= frame * 2 {
        return;
    }
    push_fill(plan, rect.x + frame - 1, body_y, 1, body_h, color);
    push_fill(plan, rect.x + rect.w - frame, body_y, 1, body_h, color);
    push_fill(plan, rect.x + frame, rect.y + rect.h - frame, rect.w - frame * 2, 1, color);
}

fn push_corner_facets(plan: &mut ChromeDrawPlan<'_>, rect: Rect, color: u32) {
    if rect.w < 16 || rect.h < 16 {
        return;
    }
    let size = 8;
    let inset = 2;
    for row in 0..size {
        push_fill(plan, rect.x + inset, rect.y + inset + row, size - row, 1, color);
        push_fill(
            plan,
            rect.x + rect.w - inset - size + row,
            rect.y + rect.h - inset - size + row,
            size - row,
            1,
            color,
        );
    }
}

fn push_top_glow_strip(plan: &mut ChromeDrawPlan<'_>, x: i32, y: i32, w: i32, theme: UiTheme) {
    if w <= 0 {
        return;
    }
    for sx in 0..w {
        let t = if w <= 1 { 0 } else { (sx as u32).saturating_mul(255) / (w as u32 - 1) };
        let color = if t < 85 {
            lerp_argb(0x007C5CFF, theme.edge_light, t.saturating_mul(3))
        } else if t < 170 {
            lerp_argb(theme.edge_light, theme.control_icon, (t - 85).saturating_mul(3))
        } else {
            lerp_argb(theme.control_icon, 0x00B8A8FF, (t - 170).saturating_mul(3))
        };
        push_fill(plan, x + sx, y, 1, 2, color);
    }
}

fn push_chrome_buttons(
    plan: &mut ChromeDrawPlan<'_>,
    rect: Rect,
    chrome: SurfaceChrome,
    theme: UiTheme,
    state: ChromeVisualState,
) {
    let Some(buttons) = chrome_button_rects(rect, chrome) else {
        return;
    };
    let icon_color = if state.active { theme.control_icon } else { theme.control_icon_inactive };
    for (button, rect) in buttons {
        let hovered = contains(rect, state.pointer_x, state.pointer_y);
        let pressed = hovered && state.primary_button_down;
        let color = if pressed {
            0xFF0B0A10
        } else if matches!(button, ChromeButton::Close) {
            theme.close_icon
        } else {
            icon_color
        };
        if hovered {
            plan.commands.push(ChromeDrawCommand::FillRect {
                rect: centered_square(rect, 16),
                color: if pressed { theme.edge_light } else { theme.button_top },
            });
        }
        plan.commands.push(ChromeDrawCommand::ControlGlyph {
            rect,
            button,
            icon_path: icon_path(button, state.shaded, state.fullscreen),
            color,
        });
    }
}

fn push_title<'a>(
    plan: &mut ChromeDrawPlan<'a>,
    visual_rect: Rect,
    content_rect: Rect,
    chrome: SurfaceChrome,
    frame: i32,
    titlebar_height: i32,
    theme: UiTheme,
    active: bool,
    title: &'a str,
) {
    let buttons_w = chrome_button_rects(content_rect, chrome)
        .map(|rects| visual_rect.x + visual_rect.w - rects[0].1.x)
        .unwrap_or(0);
    let max_chars = ((visual_rect.w - frame * 2 - 24 - buttons_w).max(10) as u32)
        .saturating_div(10)
        .max(1) as usize;
    plan.commands.push(ChromeDrawCommand::Text {
        x: visual_rect.x + frame + 8,
        y: visual_rect.y + (titlebar_height + 16) / 2,
        px_size_bits: 16.0f32.to_bits(),
        text: title_prefix(title, max_chars),
        color: if active { theme.chrome_text } else { theme.chrome_text_inactive },
    });
}

fn push_fill(plan: &mut ChromeDrawPlan<'_>, x: i32, y: i32, w: i32, h: i32, color: u32) {
    if w > 0 && h > 0 {
        plan.commands.push(ChromeDrawCommand::FillRect { rect: Rect::new(x, y, w, h), color });
    }
}

fn centered_square(rect: Rect, size: i32) -> Rect {
    let size = size.min(rect.w).min(rect.h);
    Rect::new(rect.x + (rect.w - size) / 2, rect.y + (rect.h - size) / 2, size, size)
}

fn contains(rect: Rect, x: i32, y: i32) -> bool {
    x >= rect.x && x < rect.x + rect.w && y >= rect.y && y < rect.y + rect.h
}

fn icon_path(button: ChromeButton, is_shaded: bool, is_fullscreen: bool) -> Option<&'static str> {
    match button {
        ChromeButton::Minimize => None,
        ChromeButton::Shade => {
            if is_shaded {
                Some(WINDOW_ICON_UNSHADE_PATH)
            } else {
                Some(WINDOW_ICON_SHADE_PATH)
            }
        }
        ChromeButton::Maximize => None,
        ChromeButton::Fullscreen => {
            if is_fullscreen {
                Some(WINDOW_ICON_RESTORE_PATH)
            } else {
                Some(WINDOW_ICON_FULLSCREEN_PATH)
            }
        }
        ChromeButton::Close => Some(WINDOW_ICON_CLOSE_PATH),
    }
}

fn title_prefix(title: &str, max_chars: usize) -> &str {
    if max_chars == 0 {
        return "";
    }
    match title.char_indices().nth(max_chars) {
        Some((idx, _)) => &title[..idx],
        None => title,
    }
}

fn lerp_argb(a: u32, b: u32, t: u32) -> u32 {
    let t = t.min(255);
    let inv = 255 - t;
    let aa = (a >> 24) & 0xFF;
    let ar = (a >> 16) & 0xFF;
    let ag = (a >> 8) & 0xFF;
    let ab = a & 0xFF;
    let ba = (b >> 24) & 0xFF;
    let br = (b >> 16) & 0xFF;
    let bg = (b >> 8) & 0xFF;
    let bb = b & 0xFF;
    let ca = (aa * inv + ba * t) / 255;
    let cr = (ar * inv + br * t) / 255;
    let cg = (ag * inv + bg * t) / 255;
    let cb = (ab * inv + bb * t) / 255;
    (ca << 24) | (cr << 16) | (cg << 8) | cb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chrome_plan_contains_frame_title_and_button_glyphs() {
        let plan = window_chrome_plan(
            Rect::new(0, 0, 320, 240),
            SurfaceChrome { titlebar_height: 28, frame_thickness: 4 },
            stile::default_theme(),
            ChromeVisualState {
                active: true,
                hovered: true,
                primary_button_down: false,
                pointer_x: -1,
                pointer_y: -1,
                shaded: false,
                fullscreen: false,
            },
            Some("Window"),
        );

        assert!(
            plan.commands
                .iter()
                .any(|cmd| matches!(cmd, ChromeDrawCommand::VerticalGradient { .. }))
        );
        assert!(plan.commands.iter().any(|cmd| matches!(cmd, ChromeDrawCommand::Text { .. })));
        assert!(
            plan.commands.iter().any(|cmd| matches!(cmd, ChromeDrawCommand::ControlGlyph { .. }))
        );
    }
}
