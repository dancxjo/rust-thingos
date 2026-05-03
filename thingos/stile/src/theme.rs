use crate::paint::{
    PaintCommand, PaintList, ThemeControl, ThemeIcon, ThemeRect, WindowChromeRequest,
};

pub const DEFAULT_THEME_NAME: &str = "Obsidian Bloom";

#[derive(Clone, Copy)]
pub struct Theme {
    pub name: &'static str,
    pub renderer: ThemeRenderer,
    pub active: WindowStateTokens,
    pub inactive: WindowStateTokens,
    pub titlebar_height: u32,
    pub frame_thickness: u32,
    pub corner_radius: u32,
    pub chrome_text: u32,
    pub chrome_text_inactive: u32,
    pub control_icon: u32,
    pub control_icon_inactive: u32,
    pub close_icon: u32,
    pub title_rule_active: u32,
    pub title_rule_inactive: u32,
    pub body_top: u32,
    pub button_top: u32,
    pub outer_stroke: u32,
    pub inner_stroke: u32,
    pub inner_stroke_inactive: u32,
    pub frame_fill: u32,
    pub frame_fill_inactive: u32,
    pub frame_fill_bottom: u32,
    pub frame_fill_bottom_inactive: u32,
    pub frame_bevel_light: u32,
    pub frame_bevel_light_inactive: u32,
    pub frame_bevel_shadow: u32,
    pub title_sheen: u32,
    pub facet: u32,
    pub facet_inactive: u32,
    pub focus_accent: u32,
    pub edge_light: u32,
    pub edge_dark: u32,
    pub content_edge: u32,
    pub grid_line: u32,
    pub contact_shadow: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeRenderer {
    ObsidianFacet,
    AuroraGlass,
}

#[derive(Clone, Copy)]
pub struct WindowStateTokens {
    pub border: u32,
    pub title_top: u32,
    pub title_bottom: u32,
}

pub const OBSIDIAN_BLOOM: Theme = Theme {
    name: DEFAULT_THEME_NAME,
    renderer: ThemeRenderer::ObsidianFacet,
    active: WindowStateTokens {
        border: 0xFF11131D,
        title_top: 0xFF2A3147,
        title_bottom: 0xFF111620,
    },
    inactive: WindowStateTokens {
        border: 0xFF0C1018,
        title_top: 0xFF181D2A,
        title_bottom: 0xFF0C1018,
    },
    titlebar_height: 28,
    frame_thickness: 4,
    corner_radius: 0,
    chrome_text: 0xFFE9EEF7,
    chrome_text_inactive: 0xFF7C879B,
    control_icon: 0xFFA8D7FF,
    control_icon_inactive: 0xFF667389,
    close_icon: 0xFFFF9EBA,
    title_rule_active: 0xFF31405D,
    title_rule_inactive: 0xFF1A2232,
    body_top: 0xFF090C12,
    button_top: 0xFF26334B,
    outer_stroke: 0xFF070A10,
    inner_stroke: 0xFF334360,
    inner_stroke_inactive: 0xFF1A2232,
    frame_fill: 0xFF171D2A,
    frame_fill_inactive: 0xFF101620,
    frame_fill_bottom: 0xFF0B0F16,
    frame_fill_bottom_inactive: 0xFF080B10,
    frame_bevel_light: 0xFF6EA8FF,
    frame_bevel_light_inactive: 0xFF2B3952,
    frame_bevel_shadow: 0xFF04060A,
    title_sheen: 0xFF526B9A,
    facet: 0xFF22304A,
    facet_inactive: 0xFF151E2E,
    focus_accent: 0xFFFFD166,
    edge_light: 0xFF8AD7FF,
    edge_dark: 0xFF080B12,
    content_edge: 0xFF1B2638,
    grid_line: 0x221F3148,
    contact_shadow: 0x72000000,
};

pub const AURORA_GLASS: Theme = Theme {
    name: "Aurora Glass",
    renderer: ThemeRenderer::AuroraGlass,
    active: WindowStateTokens {
        border: 0x8A174A5A,
        title_top: 0x70406B87,
        title_bottom: 0x30102734,
    },
    inactive: WindowStateTokens {
        border: 0x54203644,
        title_top: 0x30203344,
        title_bottom: 0x16060C12,
    },
    titlebar_height: 28,
    frame_thickness: 4,
    corner_radius: 0,
    chrome_text: 0xFFF0FCFF,
    chrome_text_inactive: 0xB89BB7C4,
    control_icon: 0xFFC6F7FF,
    control_icon_inactive: 0xA7749BA8,
    close_icon: 0xFFFFB6C8,
    title_rule_active: 0x6696E6D8,
    title_rule_inactive: 0x33406B87,
    body_top: 0xE6060C12,
    button_top: 0x4D42A7B8,
    outer_stroke: 0x7087DDEA,
    inner_stroke: 0x66B5FFF4,
    inner_stroke_inactive: 0x2B4C7880,
    frame_fill: 0x30153B46,
    frame_fill_inactive: 0x18070F14,
    frame_fill_bottom: 0x2204090D,
    frame_fill_bottom_inactive: 0x10020407,
    frame_bevel_light: 0x88C6F7FF,
    frame_bevel_light_inactive: 0x4D557B85,
    frame_bevel_shadow: 0x66000000,
    title_sheen: 0x66A7FFE7,
    facet: 0x403B6B76,
    facet_inactive: 0x22102530,
    focus_accent: 0xFFFFD166,
    edge_light: 0xC47DE7FF,
    edge_dark: 0x99020407,
    content_edge: 0x55406B87,
    grid_line: 0x1F7DE7FF,
    contact_shadow: 0x8A000000,
};

impl Theme {
    pub fn render_window_chrome<'a>(
        self,
        request: WindowChromeRequest<'a>,
        out: &mut PaintList<'a>,
    ) {
        if request.frame <= 0 || request.visual_rect.w <= 0 || request.visual_rect.h <= 0 {
            return;
        }

        match self.renderer {
            ThemeRenderer::ObsidianFacet => push_facet_frame(self, request, out),
            ThemeRenderer::AuroraGlass => push_glass_frame(self, request, out),
        }

        if request.titlebar_height > 0 {
            push_controls(self, request, out);
            if let Some(title) = request.title {
                push_title(self, request, title, out);
            }
        }
    }
}

pub fn default_theme() -> Theme {
    OBSIDIAN_BLOOM
}

pub fn theme_by_name(name: &str) -> Theme {
    let trimmed = name.trim();
    if trimmed.eq_ignore_ascii_case("aurora glass")
        || trimmed.eq_ignore_ascii_case("aurora-glass")
        || trimmed.eq_ignore_ascii_case("aurora_glass")
        || trimmed.eq_ignore_ascii_case("glass")
    {
        AURORA_GLASS
    } else if trimmed.eq_ignore_ascii_case("obsidian bloom")
        || trimmed.eq_ignore_ascii_case("obsidian-bloom")
        || trimmed.eq_ignore_ascii_case("obsidian_bloom")
    {
        OBSIDIAN_BLOOM
    } else {
        OBSIDIAN_BLOOM
    }
}

fn push_facet_frame<'a>(theme: Theme, request: WindowChromeRequest<'a>, out: &mut PaintList<'a>) {
    let rect = request.visual_rect;
    let frame = request.frame;
    let titlebar_height = request.titlebar_height;
    let active = request.state.active;
    let fill_top = if active { theme.frame_fill } else { theme.frame_fill_inactive };
    let fill_bottom =
        if active { theme.frame_fill_bottom } else { theme.frame_fill_bottom_inactive };
    let inner = if active { theme.inner_stroke } else { theme.inner_stroke_inactive };
    let facet = if active { theme.facet } else { theme.facet_inactive };
    let state = if active { theme.active } else { theme.inactive };

    if titlebar_height > 0 {
        out.commands.push(PaintCommand::VerticalGradient {
            rect: ThemeRect { h: titlebar_height, ..rect },
            top: state.title_top,
            bottom: state.title_bottom,
        });
        if active && titlebar_height > 6 && rect.w > frame * 2 {
            out.commands.push(PaintCommand::HorizontalGradient {
                rect: ThemeRect { x: rect.x + frame, y: rect.y + 2, w: rect.w - frame * 2, h: 2 },
                left: state.title_top,
                center: theme.title_sheen,
                right: state.title_top,
            });
        }
    } else {
        out.commands.push(PaintCommand::VerticalGradient {
            rect: ThemeRect { h: frame, ..rect },
            top: fill_top,
            bottom: fill_bottom,
        });
    }

    out.commands.push(PaintCommand::VerticalGradient {
        rect: ThemeRect { w: frame, ..rect },
        top: fill_top,
        bottom: fill_bottom,
    });
    out.commands.push(PaintCommand::VerticalGradient {
        rect: ThemeRect { x: rect.x + rect.w - frame, w: frame, ..rect },
        top: fill_top,
        bottom: fill_bottom,
    });
    out.commands.push(PaintCommand::HorizontalGradient {
        rect: ThemeRect { y: rect.y + rect.h - frame, h: frame, ..rect },
        left: fill_bottom,
        center: fill_top,
        right: fill_bottom,
    });
    out.commands.push(PaintCommand::StrokeRect { rect, thickness: 2, color: theme.outer_stroke });

    push_frame_bevel(out, rect, frame, active, theme);
    push_title_separator(out, rect, frame, titlebar_height, active, theme);
    push_content_edges(out, rect, frame, titlebar_height, inner);
    push_corner_facets(out, rect, facet);
    if active {
        push_top_glow_strip(out, rect.x + 2, rect.y, rect.w.saturating_sub(4), theme);
        out.commands.push(PaintCommand::FillRect {
            rect: ThemeRect::new(rect.x + rect.w - 4, rect.y + 2, 2, 2),
            color: theme.focus_accent,
        });
        out.commands.push(PaintCommand::FillRect {
            rect: ThemeRect::new(rect.x + frame, rect.y + rect.h - frame - 2, 2, 2),
            color: theme.focus_accent,
        });
    }
}

fn push_glass_frame<'a>(theme: Theme, request: WindowChromeRequest<'a>, out: &mut PaintList<'a>) {
    let rect = request.visual_rect;
    let frame = request.frame;
    let titlebar_height = request.titlebar_height;
    let active = request.state.active;
    let state = if active { theme.active } else { theme.inactive };
    if titlebar_height > 0 && (active || request.state.hovered) {
        out.commands.push(PaintCommand::VerticalGradient {
            rect: ThemeRect { h: titlebar_height, ..rect },
            top: state.title_top,
            bottom: state.title_bottom,
        });
        out.commands.push(PaintCommand::FillRect {
            rect: ThemeRect::new(rect.x, rect.y + titlebar_height - 1, rect.w, 1),
            color: if active { theme.title_rule_active } else { theme.title_rule_inactive },
        });
    }
    if active {
        push_top_glow_strip(out, rect.x + 2, rect.y, rect.w.saturating_sub(4), theme);
    }
    if request.state.hovered {
        let inner = if active { theme.inner_stroke } else { theme.inner_stroke_inactive };
        out.commands.push(PaintCommand::StrokeRect {
            rect,
            thickness: 1,
            color: theme.outer_stroke,
        });
        push_content_edges(out, rect, frame, titlebar_height, inner);
    }
}

fn push_frame_bevel(
    out: &mut PaintList<'_>,
    rect: ThemeRect,
    frame: i32,
    active: bool,
    theme: Theme,
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

    push_fill(out, rect.x + 2, rect.y + 2, rect.w - 4, 1, light);
    push_fill(out, rect.x + 2, rect.y + 2, 1, rect.h - 4, mid);
    push_fill(out, rect.x + 2, rect.y + rect.h - 3, rect.w - 4, 1, shadow);
    push_fill(out, rect.x + rect.w - 3, rect.y + 2, 1, rect.h - 4, shadow);
    if inner_w > 4 && inner_h > 4 {
        push_fill(out, inner_x, inner_y, inner_w, 1, shadow);
        push_fill(out, inner_x, inner_y, 1, inner_h, shadow);
        push_fill(out, inner_x, inner_y + inner_h - 1, inner_w, 1, light);
        push_fill(out, inner_x + inner_w - 1, inner_y, 1, inner_h, light);
    }
}

fn push_title_separator(
    out: &mut PaintList<'_>,
    rect: ThemeRect,
    frame: i32,
    titlebar_height: i32,
    active: bool,
    theme: Theme,
) {
    if titlebar_height <= 0 || titlebar_height >= rect.h || rect.w <= frame * 2 {
        return;
    }
    let sep_y = rect.y + titlebar_height - 1;
    push_fill(
        out,
        rect.x + frame,
        sep_y - 1,
        rect.w - frame * 2,
        1,
        if active { theme.frame_bevel_light } else { theme.frame_bevel_light_inactive },
    );
    push_fill(
        out,
        rect.x + frame,
        sep_y,
        rect.w - frame * 2,
        1,
        if active { theme.title_rule_active } else { theme.title_rule_inactive },
    );
}

fn push_content_edges(
    out: &mut PaintList<'_>,
    rect: ThemeRect,
    frame: i32,
    titlebar_height: i32,
    color: u32,
) {
    let body_y = rect.y + titlebar_height;
    let body_h = rect.h - titlebar_height - frame;
    if body_h <= 0 || rect.w <= frame * 2 {
        return;
    }
    push_fill(out, rect.x + frame - 1, body_y, 1, body_h, color);
    push_fill(out, rect.x + rect.w - frame, body_y, 1, body_h, color);
    push_fill(out, rect.x + frame, rect.y + rect.h - frame, rect.w - frame * 2, 1, color);
}

fn push_corner_facets(out: &mut PaintList<'_>, rect: ThemeRect, color: u32) {
    if rect.w < 16 || rect.h < 16 {
        return;
    }
    let size = 8;
    let inset = 2;
    for row in 0..size {
        push_fill(out, rect.x + inset, rect.y + inset + row, size - row, 1, color);
        push_fill(
            out,
            rect.x + rect.w - inset - size + row,
            rect.y + rect.h - inset - size + row,
            size - row,
            1,
            color,
        );
    }
}

fn push_top_glow_strip(out: &mut PaintList<'_>, x: i32, y: i32, w: i32, theme: Theme) {
    if w <= 0 {
        return;
    }
    for sx in 0..w {
        let t = if w <= 1 { 0 } else { (sx as u32).saturating_mul(255) / (w as u32 - 1) };
        let color = if t < 85 {
            lerp_argb(0x008AD7FF, theme.edge_light, t.saturating_mul(3))
        } else if t < 170 {
            lerp_argb(theme.edge_light, theme.control_icon, (t - 85).saturating_mul(3))
        } else {
            lerp_argb(theme.control_icon, 0x00FFD166, (t - 170).saturating_mul(3))
        };
        push_fill(out, x + sx, y, 1, 2, color);
    }
}

fn push_controls<'a>(theme: Theme, request: WindowChromeRequest<'a>, out: &mut PaintList<'a>) {
    let icon_color =
        if request.state.active { theme.control_icon } else { theme.control_icon_inactive };
    for control in request.controls.iter().flatten() {
        let hovered = contains(control.rect, request.state.pointer_x, request.state.pointer_y);
        let pressed = hovered && request.state.primary_button_down;
        let icon = control_icon(control.control, request.state.shaded, request.state.fullscreen);
        let color = if pressed {
            theme.body_top
        } else if matches!(control.control, ThemeControl::Close) {
            theme.close_icon
        } else {
            icon_color
        };
        if hovered {
            out.commands.push(PaintCommand::FillRect {
                rect: centered_square(control.rect, 16),
                color: if pressed { theme.edge_light } else { theme.button_top },
            });
        }
        out.commands.push(PaintCommand::Icon { rect: control.rect, icon, color });
    }
}

fn push_title<'a>(
    theme: Theme,
    request: WindowChromeRequest<'a>,
    title: &'a str,
    out: &mut PaintList<'a>,
) {
    let buttons_w = request
        .controls
        .iter()
        .flatten()
        .next()
        .map(|button| request.visual_rect.x + request.visual_rect.w - button.rect.x)
        .unwrap_or(0);
    let max_chars = ((request.visual_rect.w - request.frame * 2 - 24 - buttons_w).max(10) as u32)
        .saturating_div(10)
        .max(1) as usize;
    out.commands.push(PaintCommand::Text {
        x: request.visual_rect.x + request.frame + 8,
        y: request.visual_rect.y + (request.titlebar_height + 16) / 2,
        px_size_bits: 16.0f32.to_bits(),
        text: title_prefix(title, max_chars),
        color: if request.state.active { theme.chrome_text } else { theme.chrome_text_inactive },
    });
}

fn push_fill(out: &mut PaintList<'_>, x: i32, y: i32, w: i32, h: i32, color: u32) {
    if w > 0 && h > 0 {
        out.commands.push(PaintCommand::FillRect { rect: ThemeRect::new(x, y, w, h), color });
    }
}

fn centered_square(rect: ThemeRect, size: i32) -> ThemeRect {
    let size = size.min(rect.w).min(rect.h);
    ThemeRect::new(rect.x + (rect.w - size) / 2, rect.y + (rect.h - size) / 2, size, size)
}

fn contains(rect: ThemeRect, x: i32, y: i32) -> bool {
    x >= rect.x && x < rect.x + rect.w && y >= rect.y && y < rect.y + rect.h
}

fn control_icon(control: ThemeControl, shaded: bool, fullscreen: bool) -> ThemeIcon {
    match control {
        ThemeControl::Shade => {
            if shaded {
                ThemeIcon::Unshade
            } else {
                ThemeIcon::Shade
            }
        }
        ThemeControl::Fullscreen => {
            if fullscreen {
                ThemeIcon::Restore
            } else {
                ThemeIcon::Fullscreen
            }
        }
        ThemeControl::Close => ThemeIcon::Close,
    }
}

fn title_prefix(title: &str, max_chars: usize) -> &str {
    if max_chars == 0 {
        return "";
    }
    let mut end = 0;
    let mut count = 0;
    for (idx, ch) in title.char_indices() {
        if count >= max_chars {
            break;
        }
        end = idx + ch.len_utf8();
        count += 1;
    }
    if count < title.chars().count() { &title[..end] } else { title }
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
    fn bundled_theme_names_resolve_to_two_theme_system() {
        assert_eq!(default_theme().name, OBSIDIAN_BLOOM.name);
        assert_eq!(theme_by_name("aurora-glass").name, AURORA_GLASS.name);
        assert_eq!(theme_by_name("leather.bmp").name, OBSIDIAN_BLOOM.name);
    }
}
