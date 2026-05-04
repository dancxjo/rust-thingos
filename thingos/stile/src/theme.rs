use alloc::borrow::Cow;

use crate::paint::{
    PaintCommand, PaintList, ThemeControl, ThemeIcon, ThemeRect, WindowChromeRequest,
};
use crate::values::Color;

pub const DEFAULT_THEME_NAME: &str = "NocturneIris";

#[derive(Clone, Copy)]
pub struct Theme {
    pub name: &'static str,
    pub renderer: ThemeRenderer,
    /// The default wallpaper image path for this theme. Blossom reads this
    /// via the petals desktop module to choose what to display at first paint.
    pub wallpaper_path: Option<&'static str>,
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
    pub hover_tint: u32,
    pub edge_light: u32,
    pub edge_dark: u32,
    pub content_edge: u32,
    pub grid_line: u32,
    pub contact_shadow: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeRenderer {
    SolarisWarm,
    ObsidianFacet,
    AuroraGlass,
    NocturneIris,
}

#[derive(Clone, Copy)]
pub struct WindowStateTokens {
    pub border: u32,
    pub title_top: u32,
    pub title_bottom: u32,
}

#[derive(Clone, Copy)]
pub struct SolarisWarmPalette {
    pub bg_primary: Color,
    pub bg_secondary: Color,
    pub surface: Color,
    pub accent: Color,
    pub accent_soft: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub border: Color,
    pub shadow: Color,
}

pub const SOLARIS_WARM_PALETTE: SolarisWarmPalette = SolarisWarmPalette {
    bg_primary: Color::rgb(0x1E, 0x1A, 0x16),
    bg_secondary: Color::rgb(0x2A, 0x24, 0x1F),
    surface: Color::rgb(0x3A, 0x32, 0x2B),
    accent: Color::rgb(0xE3, 0xA8, 0x57),
    accent_soft: Color::rgb(0xF2, 0xC0, 0x78),
    text_primary: Color::rgb(0xF5, 0xE6, 0xD3),
    text_secondary: Color::rgb(0xC9, 0xB8, 0xA2),
    text_muted: Color::rgb(0x8A, 0x7A, 0x66),
    border: Color::rgb(0x4A, 0x40, 0x36),
    shadow: Color::rgba(0, 0, 0, 140),
};

pub const SOLARIS_WARM: Theme = Theme {
    name: "SolarisWarm",
    renderer: ThemeRenderer::SolarisWarm,
    wallpaper_path: Some("/public/wallpapers/flower.bmp"),
    active: WindowStateTokens {
        border: color_argb(SOLARIS_WARM_PALETTE.border),
        title_top: color_argb(SOLARIS_WARM_PALETTE.bg_secondary),
        title_bottom: color_argb(SOLARIS_WARM_PALETTE.bg_primary),
    },
    inactive: WindowStateTokens {
        border: color_argb(SOLARIS_WARM_PALETTE.border),
        title_top: color_argb(SOLARIS_WARM_PALETTE.surface),
        title_bottom: color_argb(SOLARIS_WARM_PALETTE.bg_secondary),
    },
    titlebar_height: 44,
    frame_thickness: 4,
    corner_radius: 0,
    chrome_text: color_argb(SOLARIS_WARM_PALETTE.text_primary),
    chrome_text_inactive: color_argb(SOLARIS_WARM_PALETTE.text_muted),
    control_icon: color_argb(SOLARIS_WARM_PALETTE.accent_soft),
    control_icon_inactive: color_argb(SOLARIS_WARM_PALETTE.text_muted),
    close_icon: color_argb(SOLARIS_WARM_PALETTE.accent),
    title_rule_active: color_argb(SOLARIS_WARM_PALETTE.accent),
    title_rule_inactive: color_argb(SOLARIS_WARM_PALETTE.border),
    body_top: color_argb(SOLARIS_WARM_PALETTE.bg_primary),
    button_top: color_argb(SOLARIS_WARM_PALETTE.surface),
    outer_stroke: color_argb(SOLARIS_WARM_PALETTE.border),
    inner_stroke: color_argb(SOLARIS_WARM_PALETTE.surface),
    inner_stroke_inactive: color_argb(SOLARIS_WARM_PALETTE.border),
    frame_fill: color_argb(SOLARIS_WARM_PALETTE.surface),
    frame_fill_inactive: color_argb(SOLARIS_WARM_PALETTE.bg_secondary),
    frame_fill_bottom: color_argb(SOLARIS_WARM_PALETTE.bg_secondary),
    frame_fill_bottom_inactive: color_argb(SOLARIS_WARM_PALETTE.bg_primary),
    frame_bevel_light: color_argb(SOLARIS_WARM_PALETTE.accent_soft),
    frame_bevel_light_inactive: color_argb(SOLARIS_WARM_PALETTE.text_muted),
    frame_bevel_shadow: color_argb(SOLARIS_WARM_PALETTE.bg_primary),
    title_sheen: color_argb(SOLARIS_WARM_PALETTE.accent_soft),
    facet: color_argb(SOLARIS_WARM_PALETTE.surface),
    facet_inactive: color_argb(SOLARIS_WARM_PALETTE.bg_secondary),
    focus_accent: color_argb(SOLARIS_WARM_PALETTE.accent),
    hover_tint: color_with_alpha(SOLARIS_WARM_PALETTE.accent, 38),
    edge_light: color_argb(SOLARIS_WARM_PALETTE.accent_soft),
    edge_dark: color_argb(SOLARIS_WARM_PALETTE.bg_primary),
    content_edge: color_argb(SOLARIS_WARM_PALETTE.border),
    grid_line: color_with_alpha(SOLARIS_WARM_PALETTE.border, 31),
    contact_shadow: color_argb(SOLARIS_WARM_PALETTE.shadow),
};

pub const OBSIDIAN_BLOOM: Theme = Theme {
    name: "Obsidian Bloom",
    renderer: ThemeRenderer::ObsidianFacet,
    wallpaper_path: Some("/public/wallpapers/leather.bmp"),
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
    titlebar_height: 44,
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
    hover_tint: 0x266EA8FF,
    edge_light: 0xFF8AD7FF,
    edge_dark: 0xFF080B12,
    content_edge: 0xFF1B2638,
    grid_line: 0x221F3148,
    contact_shadow: 0x72000000,
};

pub const AURORA_GLASS: Theme = Theme {
    name: "Aurora Glass",
    renderer: ThemeRenderer::AuroraGlass,
    wallpaper_path: Some("/public/wallpapers/clouds.bmp"),
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
    titlebar_height: 44,
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
    hover_tint: 0x26C6F7FF,
    edge_light: 0xC47DE7FF,
    edge_dark: 0x99020407,
    content_edge: 0x55406B87,
    grid_line: 0x1F7DE7FF,
    contact_shadow: 0x8A000000,
};

pub const NOCTURNE_IRIS: Theme = Theme {
    name: DEFAULT_THEME_NAME,
    renderer: ThemeRenderer::NocturneIris,
    wallpaper_path: "/public/wallpapers/flower.png",
    active: WindowStateTokens {
        border: 0x996F55D9,
        title_top: 0x702E2158,
        title_bottom: 0x24141020,
    },
    inactive: WindowStateTokens {
        border: 0x553B346F,
        title_top: 0x341D1730,
        title_bottom: 0x140B0E14,
    },
    titlebar_height: 44,
    frame_thickness: 4,
    corner_radius: 8,
    chrome_text: 0xFFEDE9FF,
    chrome_text_inactive: 0xCCA9A3C9,
    control_icon: 0xFFE8C36A,
    control_icon_inactive: 0xB8C9A44F,
    close_icon: 0xFFFFD785,
    title_rule_active: 0x667C5CFF,
    title_rule_inactive: 0x334B3FA3,
    body_top: 0xA6141020,
    button_top: 0x187C5CFF,
    outer_stroke: 0xA06F55D9,
    inner_stroke: 0x40B9A7FF,
    inner_stroke_inactive: 0x244B3FA3,
    frame_fill: 0xA6141020,
    frame_fill_inactive: 0x80110E1C,
    frame_fill_bottom: 0x900B0E14,
    frame_fill_bottom_inactive: 0x600B0E14,
    frame_bevel_light: 0x66B9A7FF,
    frame_bevel_light_inactive: 0x334B3FA3,
    frame_bevel_shadow: 0x66000000,
    title_sheen: 0x557C5CFF,
    facet: 0x303C2A78,
    facet_inactive: 0x18140F24,
    focus_accent: 0x997C5CFF,
    hover_tint: 0x2E7C5CFF,
    edge_light: 0xCCB9A7FF,
    edge_dark: 0xB00B0E14,
    content_edge: 0x24B9A7FF,
    grid_line: 0x00000000,
    contact_shadow: 0x407C5CFF,
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
            ThemeRenderer::SolarisWarm => {
                push_solaris_warm_window(self, request, out);
                return;
            }
            ThemeRenderer::NocturneIris => {
                push_nocturne_iris_window(self, request, out);
                return;
            }
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
    NOCTURNE_IRIS
}

pub static AVAILABLE_THEMES: [Theme; 4] =
    [SOLARIS_WARM, NOCTURNE_IRIS, AURORA_GLASS, OBSIDIAN_BLOOM];

pub fn available_themes() -> &'static [Theme] {
    &AVAILABLE_THEMES
}

pub fn find_theme_by_name(name: &str) -> Option<Theme> {
    let trimmed = name.trim();
    if matches_solaris_warm(trimmed) {
        Some(SOLARIS_WARM)
    } else if matches_nocturne_iris(trimmed) {
        Some(NOCTURNE_IRIS)
    } else if matches_aurora_glass(trimmed) {
        Some(AURORA_GLASS)
    } else if matches_obsidian_bloom(trimmed) {
        Some(OBSIDIAN_BLOOM)
    } else {
        None
    }
}

pub fn theme_by_name(name: &str) -> Theme {
    find_theme_by_name(name).unwrap_or(NOCTURNE_IRIS)
}

fn matches_solaris_warm(trimmed: &str) -> bool {
    trimmed.eq_ignore_ascii_case("solariswarm")
        || trimmed.eq_ignore_ascii_case("solaris warm")
        || trimmed.eq_ignore_ascii_case("solaris-warm")
        || trimmed.eq_ignore_ascii_case("solaris_warm")
        || trimmed.eq_ignore_ascii_case("solarized warm")
        || trimmed.eq_ignore_ascii_case("solarized-warm")
}

fn matches_aurora_glass(trimmed: &str) -> bool {
    trimmed.eq_ignore_ascii_case("aurora glass")
        || trimmed.eq_ignore_ascii_case("aurora-glass")
        || trimmed.eq_ignore_ascii_case("aurora_glass")
        || trimmed.eq_ignore_ascii_case("glass")
}

fn matches_nocturne_iris(trimmed: &str) -> bool {
    trimmed.eq_ignore_ascii_case("nocturneiris")
        || trimmed.eq_ignore_ascii_case("nocturne iris")
        || trimmed.eq_ignore_ascii_case("nocturne-iris")
        || trimmed.eq_ignore_ascii_case("nocturne_iris")
        || trimmed.eq_ignore_ascii_case("nocturne_iris.stile")
        || trimmed.eq_ignore_ascii_case("flower.png")
        || trimmed.eq_ignore_ascii_case("nocturne_iris.bmp")
        || trimmed.eq_ignore_ascii_case("iris")
}

fn matches_obsidian_bloom(trimmed: &str) -> bool {
    trimmed.eq_ignore_ascii_case("obsidian bloom")
        || trimmed.eq_ignore_ascii_case("obsidian-bloom")
        || trimmed.eq_ignore_ascii_case("obsidian_bloom")
        || trimmed.eq_ignore_ascii_case("leather.bmp")
}

fn push_nocturne_iris_window<'a>(
    theme: Theme,
    request: WindowChromeRequest<'a>,
    out: &mut PaintList<'a>,
) {
    let rect = request.visual_rect;
    let frame = request.frame.max(1);
    let titlebar_height = request.titlebar_height.min(theme.titlebar_height as i32);
    let active = request.state.active;
    let state = if active { theme.active } else { theme.inactive };

    out.commands.push(PaintCommand::Shadow {
        rect,
        offset_x: 0,
        offset_y: 8,
        blur_radius: 32,
        color: if active { theme.contact_shadow } else { 0x26000000 },
    });
    out.commands.push(PaintCommand::PushClip { rect });
    out.commands.push(PaintCommand::VerticalGradient {
        rect,
        top: if active { theme.frame_fill } else { theme.frame_fill_inactive },
        bottom: if active { theme.frame_fill_bottom } else { theme.frame_fill_bottom_inactive },
    });

    if titlebar_height > 0 {
        let titlebar = ThemeRect::new(rect.x, rect.y, rect.w, titlebar_height);
        out.commands.push(PaintCommand::VerticalGradient {
            rect: titlebar,
            top: state.title_top,
            bottom: state.title_bottom,
        });
        if rect.w > frame * 2 {
            out.commands.push(PaintCommand::HorizontalGradient {
                rect: ThemeRect::new(
                    rect.x + frame,
                    rect.y + titlebar_height - 1,
                    rect.w - frame * 2,
                    1,
                ),
                left: color_with_alpha(Color::rgb(0x7C, 0x5C, 0xFF), 0),
                center: if active { theme.title_rule_active } else { theme.title_rule_inactive },
                right: color_with_alpha(Color::rgb(0x7C, 0x5C, 0xFF), 0),
            });
        }
        if active && rect.w > frame * 2 {
            out.commands.push(PaintCommand::HorizontalGradient {
                rect: ThemeRect::new(rect.x + frame, rect.y + 1, rect.w - frame * 2, 2),
                left: 0x007C5CFF,
                center: theme.title_sheen,
                right: 0x007C5CFF,
            });
        }
    }

    if rect.w > frame * 2 && rect.h > titlebar_height + frame {
        let body = ThemeRect::new(
            rect.x + frame,
            rect.y + titlebar_height,
            rect.w - frame * 2,
            rect.h - titlebar_height - frame,
        );
        out.commands.push(PaintCommand::VerticalGradient {
            rect: body,
            top: 0x121E1538,
            bottom: 0x080B0E14,
        });
    }

    out.commands.push(PaintCommand::StrokeRect {
        rect,
        thickness: 1,
        color: if active { theme.outer_stroke } else { theme.inactive.border },
    });
    out.commands.push(PaintCommand::StrokeRect {
        rect: ThemeRect::new(rect.x + 1, rect.y + 1, rect.w - 2, rect.h - 2),
        thickness: 1,
        color: if active { theme.inner_stroke } else { theme.inner_stroke_inactive },
    });

    if request.titlebar_height > 0 {
        push_titlebar_grip(theme, request, out);
        push_nocturne_controls(theme, request, out);
        if let Some(title) = request.title {
            push_title(theme, request, title, out);
        }
    }

    out.commands.push(PaintCommand::PopClip);
}

fn push_solaris_warm_window<'a>(
    theme: Theme,
    request: WindowChromeRequest<'a>,
    out: &mut PaintList<'a>,
) {
    let rect = request.visual_rect;
    let frame = request.frame.max(1);
    let titlebar_height = request.titlebar_height.min(theme.titlebar_height as i32);
    let active = request.state.active;
    let palette = SOLARIS_WARM_PALETTE;
    let primary = color_argb(palette.bg_primary);
    let secondary = color_argb(palette.bg_secondary);
    let surface = color_argb(palette.surface);
    let border = color_argb(palette.border);
    let accent = color_argb(palette.accent);
    let text =
        if active { color_argb(palette.text_primary) } else { color_argb(palette.text_muted) };
    let title_top =
        if active { secondary } else { soften_argb(surface, color_argb(palette.text_muted), 36) };
    let title_bottom = if active { primary } else { soften_argb(secondary, primary, 132) };
    let surface_top = if active { surface } else { soften_argb(surface, primary, 116) };
    let surface_bottom = if active { secondary } else { primary };

    out.commands.push(PaintCommand::Shadow {
        rect,
        offset_x: 0,
        offset_y: 6,
        blur_radius: 12,
        color: color_argb(palette.shadow),
    });
    out.commands.push(PaintCommand::PushClip { rect });
    out.commands.push(PaintCommand::VerticalGradient {
        rect,
        top: surface_top,
        bottom: surface_bottom,
    });
    out.commands.push(PaintCommand::StrokeRect { rect, thickness: 1, color: border });

    if rect.w > 2 && rect.h > 2 {
        out.commands.push(PaintCommand::FillRect {
            rect: ThemeRect::new(rect.x + 1, rect.y + 1, rect.w - 2, 1),
            color: color_with_alpha(palette.accent_soft, if active { 62 } else { 24 }),
        });
    }

    if titlebar_height > 0 {
        let titlebar = ThemeRect::new(rect.x + 1, rect.y + 1, rect.w - 2, titlebar_height - 1);
        out.commands.push(PaintCommand::VerticalGradient {
            rect: titlebar,
            top: title_top,
            bottom: title_bottom,
        });
        out.commands.push(PaintCommand::FillRect {
            rect: ThemeRect::new(rect.x + 1, rect.y + titlebar_height, rect.w - 2, 1),
            color: soften_argb(primary, border, if active { 120 } else { 68 }),
        });
        if active && rect.w > frame * 2 {
            let underline = ThemeRect::new(
                rect.x + frame + 8,
                rect.y + titlebar_height - 2,
                rect.w - frame * 2 - 16,
                2,
            );
            out.commands.push(PaintCommand::Shadow {
                rect: underline,
                offset_x: 0,
                offset_y: 0,
                blur_radius: 7,
                color: color_with_alpha(palette.accent, 92),
            });
            out.commands.push(PaintCommand::HorizontalGradient {
                rect: underline,
                left: color_with_alpha(palette.accent, 0),
                center: accent,
                right: color_with_alpha(palette.accent, 0),
            });
        }
    }

    push_solaris_content_edges(out, rect, frame, titlebar_height, theme);
    if request.titlebar_height > 0 {
        push_titlebar_grip(theme, request, out);
        push_solaris_control_pads(theme, request, out);
        if let Some(title) = request.title {
            let buttons_w = request
                .controls
                .iter()
                .flatten()
                .next()
                .map(|button| request.visual_rect.x + request.visual_rect.w - button.rect.x)
                .unwrap_or(0);
            let max_chars = ((request.visual_rect.w - request.frame * 2 - 28 - buttons_w).max(10)
                as u32)
                .saturating_div(9)
                .max(1) as usize;
            out.commands.push(PaintCommand::Text {
                x: rect.x + frame + 10,
                y: rect.y + 22,
                px_size_bits: 15.0f32.to_bits(),
                text: Cow::Borrowed(title_prefix(title, max_chars)),
                color: text,
            });
        }
    }
    out.commands.push(PaintCommand::PopClip);

    if rect.w > 8 && rect.h > 8 {
        out.commands.push(PaintCommand::StrokeRect {
            rect: ThemeRect::new(rect.x + 1, rect.y + 1, rect.w - 2, rect.h - 2),
            thickness: 1,
            color: color_with_alpha(palette.accent_soft, if active { 48 } else { 18 }),
        });
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
    // Always draw a minimal border so the window edge is visible on any
    // background.  Hovered or active windows get the full outer_stroke; others
    // get a dimmed version so the frame remains legible without dominating.
    let border_color = if request.state.hovered {
        theme.outer_stroke
    } else {
        let base_a = (theme.outer_stroke >> 24) & 0xFF;
        let dim_a = (base_a * 2 / 3).max(40);
        (dim_a << 24) | (theme.outer_stroke & 0x00FFFFFF)
    };
    out.commands.push(PaintCommand::StrokeRect { rect, thickness: 1, color: border_color });
    if request.state.hovered {
        let inner = if active { theme.inner_stroke } else { theme.inner_stroke_inactive };
        push_content_edges(out, rect, frame, titlebar_height, inner);
    }
}

fn push_solaris_content_edges(
    out: &mut PaintList<'_>,
    rect: ThemeRect,
    frame: i32,
    titlebar_height: i32,
    theme: Theme,
) {
    if rect.w <= frame * 2 || rect.h <= titlebar_height + frame {
        return;
    }
    let inner_x = rect.x + frame;
    let inner_y = rect.y + titlebar_height;
    let inner_w = rect.w - frame * 2;
    let inner_h = rect.h - titlebar_height - frame;
    // Clear the content area so the client surface shows through unobstructed.
    // The body overlay provides the dark background when no client is present.
    out.commands.push(PaintCommand::FillRect {
        rect: ThemeRect::new(inner_x, inner_y, inner_w, inner_h),
        color: 0x00000000,
    });
    out.commands.push(PaintCommand::StrokeRect {
        rect: ThemeRect::new(inner_x - 1, inner_y, inner_w + 2, inner_h + 1),
        thickness: 1,
        color: theme.content_edge,
    });
}

fn push_solaris_control_pads<'a>(
    theme: Theme,
    request: WindowChromeRequest<'a>,
    out: &mut PaintList<'a>,
) {
    let palette = SOLARIS_WARM_PALETTE;
    for control in request.controls.iter().flatten() {
        let hovered = contains(control.rect, request.state.pointer_x, request.state.pointer_y);
        let pressed = hovered && request.state.primary_button_down;
        let pad = centered_square(control.rect, if hovered { 30 } else { 26 });
        let base = match control.control {
            ThemeControl::Close => {
                soften_argb(theme.close_icon, color_argb(palette.accent_soft), 42)
            }
            ThemeControl::Shade | ThemeControl::Fullscreen => {
                soften_argb(color_argb(palette.surface), color_argb(palette.text_secondary), 88)
            }
        };
        let top = if pressed {
            soften_argb(base, color_argb(palette.bg_primary), 146)
        } else if hovered {
            soften_argb(base, color_argb(palette.accent_soft), 72)
        } else if request.state.active {
            base
        } else {
            soften_argb(base, color_argb(palette.bg_primary), 116)
        };
        let bottom = if pressed {
            soften_argb(base, color_argb(palette.bg_primary), 208)
        } else {
            soften_argb(top, color_argb(palette.bg_primary), 92)
        };
        if hovered {
            push_solaris_disc(
                out,
                centered_square(control.rect, 38),
                color_with_alpha(palette.accent, if pressed { 72 } else { 36 }),
            );
        }
        push_solaris_disc_gradient(out, pad, top, bottom);
        if pressed {
            push_solaris_disc(
                out,
                centered_square(pad, 12),
                color_with_alpha(palette.bg_primary, 96),
            );
        } else {
            push_solaris_disc(
                out,
                centered_square(pad, 8),
                color_with_alpha(palette.text_primary, 38),
            );
        }
        let icon = control_icon(control.control, request.state.shaded, request.state.fullscreen);
        let icon_color = if pressed {
            color_argb(palette.bg_primary)
        } else if matches!(control.control, ThemeControl::Close) {
            color_argb(palette.bg_primary)
        } else {
            color_argb(palette.text_primary)
        };
        out.commands.push(PaintCommand::Icon { rect: control.rect, icon, color: icon_color });
    }
}

fn push_solaris_disc(out: &mut PaintList<'_>, rect: ThemeRect, color: u32) {
    if rect.w <= 0 || rect.h <= 0 {
        return;
    }
    let radius = rect.w.min(rect.h) / 2;
    let cx = rect.x + rect.w / 2;
    let cy = rect.y + rect.h / 2;
    for row in -radius..=radius {
        let y = cy + row;
        let dy = row.abs();
        let span = radius - ((dy * dy + radius / 2) / radius.max(1));
        if span > 0 {
            push_fill(out, cx - span, y, span * 2, 1, color);
        }
    }
}

fn push_solaris_disc_gradient(out: &mut PaintList<'_>, rect: ThemeRect, top: u32, bottom: u32) {
    if rect.w <= 0 || rect.h <= 0 {
        return;
    }
    let radius = rect.w.min(rect.h) / 2;
    let cx = rect.x + rect.w / 2;
    let cy = rect.y + rect.h / 2;
    for row in -radius..=radius {
        let y = cy + row;
        let dy = row.abs();
        let span = radius - ((dy * dy + radius / 2) / radius.max(1));
        if span > 0 {
            let t = ((row + radius) as u32).saturating_mul(255) / (radius * 2).max(1) as u32;
            push_fill(out, cx - span, y, span * 2, 1, lerp_argb(top, bottom, t));
        }
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
    push_titlebar_grip(theme, request, out);
    for control in request.controls.iter().flatten() {
        let hovered = contains(control.rect, request.state.pointer_x, request.state.pointer_y);
        let pressed = hovered && request.state.primary_button_down;
        let icon = control_icon(control.control, request.state.shaded, request.state.fullscreen);
        let color = if pressed {
            theme.body_top
        } else if hovered {
            if matches!(control.control, ThemeControl::Close) {
                theme.close_icon
            } else {
                theme.chrome_text
            }
        } else if matches!(control.control, ThemeControl::Close) {
            theme.close_icon
        } else {
            icon_color
        };
        let pad = centered_square(control.rect, if hovered { 38 } else { 34 });
        out.commands.push(PaintCommand::FillRect {
            rect: pad,
            color: if pressed {
                theme.edge_light
            } else if hovered {
                soften_argb(theme.button_top, theme.edge_light, 72)
            } else if request.state.active {
                theme.button_top
            } else {
                theme.facet_inactive
            },
        });
        out.commands.push(PaintCommand::StrokeRect {
            rect: pad,
            thickness: if hovered { 2 } else { 1 },
            color: if pressed || hovered {
                theme.edge_light
            } else if request.state.active {
                theme.inner_stroke
            } else {
                theme.inner_stroke_inactive
            },
        });
        out.commands.push(PaintCommand::Icon { rect: control.rect, icon, color });
    }
}

fn push_nocturne_controls<'a>(
    theme: Theme,
    request: WindowChromeRequest<'a>,
    out: &mut PaintList<'a>,
) {
    for control in request.controls.iter().flatten() {
        let hovered = contains(control.rect, request.state.pointer_x, request.state.pointer_y);
        let pressed = hovered && request.state.primary_button_down;
        let halo = centered_square(control.rect, if hovered { 34 } else { 28 });
        let icon = control_icon(control.control, request.state.shaded, request.state.fullscreen);
        let base = if matches!(control.control, ThemeControl::Close) {
            theme.close_icon
        } else if request.state.active {
            theme.control_icon
        } else {
            theme.control_icon_inactive
        };
        let icon_color = if pressed {
            theme.chrome_text
        } else if hovered {
            if matches!(control.control, ThemeControl::Close) {
                theme.close_icon
            } else {
                theme.edge_light
            }
        } else {
            base
        };
        if hovered {
            out.commands.push(PaintCommand::Shadow {
                rect: halo,
                offset_x: 0,
                offset_y: 0,
                blur_radius: 12,
                color: if matches!(control.control, ThemeControl::Close) {
                    0x4DE8C36A
                } else {
                    0x4D7C5CFF
                },
            });
            out.commands.push(PaintCommand::FillRect {
                rect: halo,
                color: if pressed { 0x267C5CFF } else { theme.hover_tint },
            });
            out.commands.push(PaintCommand::StrokeRect {
                rect: halo,
                thickness: 1,
                color: if matches!(control.control, ThemeControl::Close) {
                    0x66E8C36A
                } else {
                    theme.inner_stroke
                },
            });
            out.commands.push(PaintCommand::FillRect {
                rect: centered_square(control.rect, 8),
                color: if pressed { 0x40E8C36A } else { 0x267C5CFF },
            });
        }
        out.commands.push(PaintCommand::Icon { rect: control.rect, icon, color: icon_color });
    }
}

fn push_titlebar_grip<'a>(theme: Theme, request: WindowChromeRequest<'a>, out: &mut PaintList<'a>) {
    if request.titlebar_height < 28 {
        return;
    }
    let Some(first_control) = request.controls.iter().flatten().next() else {
        return;
    };
    let title_clear_x = request.visual_rect.x + request.frame + 120;
    let grip_w = 34;
    let grip_x = (first_control.rect.x - grip_w - 16).max(title_clear_x);
    if grip_x + grip_w >= first_control.rect.x - 8 {
        return;
    }
    let center_y = request.visual_rect.y + request.titlebar_height / 2;
    let color = if request.state.active {
        soften_argb(theme.edge_light, theme.active.title_top, 80)
    } else {
        soften_argb(theme.inner_stroke_inactive, theme.inactive.title_top, 96)
    };
    for i in 0..5 {
        let x = grip_x + i * 7;
        out.commands
            .push(PaintCommand::FillRect { rect: ThemeRect::new(x, center_y - 5, 2, 10), color });
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
        text: Cow::Borrowed(title_prefix(title, max_chars)),
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

const fn color_argb(color: Color) -> u32 {
    ((color.a as u32) << 24) | ((color.r as u32) << 16) | ((color.g as u32) << 8) | color.b as u32
}

const fn color_with_alpha(color: Color, alpha: u8) -> u32 {
    ((alpha as u32) << 24) | ((color.r as u32) << 16) | ((color.g as u32) << 8) | color.b as u32
}

fn soften_argb(a: u32, b: u32, amount: u32) -> u32 {
    lerp_argb(a, b, amount)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paint::{ThemeControlRect, ThemeState};

    #[test]
    fn bundled_theme_names_resolve_to_theme_system() {
        assert_eq!(default_theme().name, NOCTURNE_IRIS.name);
        assert_eq!(available_themes().len(), 4);
        assert_eq!(theme_by_name("solaris-warm").name, SOLARIS_WARM.name);
        assert_eq!(theme_by_name("nocturne-iris").name, NOCTURNE_IRIS.name);
        assert_eq!(theme_by_name("nocturne_iris.stile").name, NOCTURNE_IRIS.name);
        assert_eq!(theme_by_name("iris").wallpaper_path, "/public/wallpapers/flower.png");
        assert_eq!(theme_by_name("aurora-glass").name, AURORA_GLASS.name);
        assert_eq!(theme_by_name("obsidian-bloom").name, OBSIDIAN_BLOOM.name);
        assert_eq!(find_theme_by_name("obsidian_bloom").unwrap().name, OBSIDIAN_BLOOM.name);
        assert!(find_theme_by_name("not-a-theme").is_none());
        assert_eq!(theme_by_name("leather.bmp").name, OBSIDIAN_BLOOM.name);
    }

    #[test]
    fn bundled_themes_have_correct_wallpaper_paths() {
        assert_eq!(SOLARIS_WARM.wallpaper_path, Some("/public/wallpapers/flower.bmp"));
        assert_eq!(OBSIDIAN_BLOOM.wallpaper_path, Some("/public/wallpapers/leather.bmp"));
        assert_eq!(AURORA_GLASS.wallpaper_path, Some("/public/wallpapers/clouds.bmp"));
        // All paths are non-empty and rooted under /public/
        for theme in available_themes() {
            let path = theme.wallpaper_path.expect("Bundled themes must define wallpaper_path");
            assert!(!path.is_empty(), "{} has empty wallpaper_path", theme.name);
            assert!(
                path.starts_with("/public/"),
                "{} wallpaper_path '{}' not under /public/",
                theme.name,
                path
            );
        }
    }

    #[test]
    fn bundled_themes_render_actionable_window_controls() {
        let controls = [
            Some(ThemeControlRect {
                control: ThemeControl::Shade,
                rect: ThemeRect::new(212, 0, 44, 44),
            }),
            Some(ThemeControlRect {
                control: ThemeControl::Fullscreen,
                rect: ThemeRect::new(262, 0, 44, 44),
            }),
            Some(ThemeControlRect {
                control: ThemeControl::Close,
                rect: ThemeRect::new(312, 0, 44, 44),
            }),
        ];

        for theme in available_themes() {
            assert!(theme.titlebar_height >= 44, "{} titlebar is too small", theme.name);

            let mut idle = PaintList::default();
            theme.render_window_chrome(
                WindowChromeRequest {
                    visual_rect: ThemeRect::new(0, 0, 368, 244),
                    content_rect: ThemeRect::new(4, 44, 360, 196),
                    frame: 4,
                    titlebar_height: 44,
                    controls,
                    state: ThemeState {
                        active: true,
                        hovered: true,
                        primary_button_down: false,
                        pointer_x: -1,
                        pointer_y: -1,
                        shaded: false,
                        fullscreen: false,
                    },
                    title: Some("Window"),
                },
                &mut idle,
            );
            assert_eq!(icon_count(&idle), 3, "{} did not render all control icons", theme.name);
            assert!(
                grip_count(&idle) >= 5,
                "{} did not render a titlebar drag affordance",
                theme.name
            );

            for control in controls.iter().flatten() {
                let mut hover = PaintList::default();
                theme.render_window_chrome(
                    WindowChromeRequest {
                        visual_rect: ThemeRect::new(0, 0, 368, 244),
                        content_rect: ThemeRect::new(4, 44, 360, 196),
                        frame: 4,
                        titlebar_height: 44,
                        controls,
                        state: ThemeState {
                            active: true,
                            hovered: true,
                            primary_button_down: false,
                            pointer_x: control.rect.x + control.rect.w / 2,
                            pointer_y: control.rect.y + control.rect.h / 2,
                            shaded: false,
                            fullscreen: false,
                        },
                        title: Some("Window"),
                    },
                    &mut hover,
                );
                assert!(
                    feedback_paint_count(&hover, control.rect) >= 3,
                    "{} did not render clear hover feedback for {:?}",
                    theme.name,
                    control.control
                );

                let mut pressed = PaintList::default();
                theme.render_window_chrome(
                    WindowChromeRequest {
                        visual_rect: ThemeRect::new(0, 0, 368, 244),
                        content_rect: ThemeRect::new(4, 44, 360, 196),
                        frame: 4,
                        titlebar_height: 44,
                        controls,
                        state: ThemeState {
                            active: true,
                            hovered: true,
                            primary_button_down: true,
                            pointer_x: control.rect.x + control.rect.w / 2,
                            pointer_y: control.rect.y + control.rect.h / 2,
                            shaded: false,
                            fullscreen: false,
                        },
                        title: Some("Window"),
                    },
                    &mut pressed,
                );
                assert_ne!(
                    hover.commands, pressed.commands,
                    "{} press feedback matches hover feedback for {:?}",
                    theme.name, control.control
                );
            }
        }
    }

    #[test]
    fn aurora_glass_always_has_visible_border() {
        // AuroraGlass must draw at least one StrokeRect for the window border
        // regardless of hover/active state so the window edge is always legible.
        let theme = theme_by_name("aurora-glass");
        let window = ThemeRect::new(0, 0, 368, 244);

        for (active, hovered) in [(false, false), (false, true), (true, false), (true, true)] {
            let mut plan = PaintList::default();
            theme.render_window_chrome(
                WindowChromeRequest {
                    visual_rect: window,
                    content_rect: ThemeRect::new(4, 44, 360, 196),
                    frame: 4,
                    titlebar_height: 44,
                    controls: [None, None, None],
                    state: ThemeState {
                        active,
                        hovered,
                        primary_button_down: false,
                        pointer_x: -1,
                        pointer_y: -1,
                        shaded: false,
                        fullscreen: false,
                    },
                    title: None,
                },
                &mut plan,
            );
            let border_count = plan.commands.iter().filter(|cmd| {
                matches!(cmd, PaintCommand::StrokeRect { rect, thickness: 1, .. } if *rect == window)
            }).count();
            assert!(
                border_count >= 1,
                "AuroraGlass (active={active}, hovered={hovered}) must draw at least one window border stroke"
            );
        }
    }

    #[test]
    fn solaris_warm_content_area_is_transparent() {
        // The content area of the SolarisWarm chrome overlay must be cleared to
        // transparent so the client surface shows through unobstructed.
        let theme = theme_by_name("solaris-warm");
        let mut plan = PaintList::default();
        theme.render_window_chrome(
            WindowChromeRequest {
                visual_rect: ThemeRect::new(0, 0, 368, 244),
                content_rect: ThemeRect::new(4, 44, 360, 196),
                frame: 4,
                titlebar_height: 44,
                controls: [None, None, None],
                state: ThemeState {
                    active: true,
                    hovered: false,
                    primary_button_down: false,
                    pointer_x: -1,
                    pointer_y: -1,
                    shaded: false,
                    fullscreen: false,
                },
                title: None,
            },
            &mut plan,
        );
        let has_transparent_fill = plan.commands.iter().any(|cmd| {
            matches!(cmd, PaintCommand::FillRect { rect, color: 0x00000000 }
                if rect.x == 4 && rect.y == 44 && rect.w == 360 && rect.h == 196)
        });
        assert!(
            has_transparent_fill,
            "SolarisWarm must punch a transparent fill over the content area for client legibility"
        );
    }

    fn icon_count(plan: &PaintList<'_>) -> usize {
        plan.commands.iter().filter(|command| matches!(command, PaintCommand::Icon { .. })).count()
    }

    fn feedback_paint_count(plan: &PaintList<'_>, control_rect: ThemeRect) -> usize {
        plan.commands
            .iter()
            .filter(|command| match command {
                PaintCommand::FillRect { rect, .. } | PaintCommand::StrokeRect { rect, .. } => {
                    rect_inside(*rect, control_rect)
                }
                _ => false,
            })
            .count()
    }

    fn grip_count(plan: &PaintList<'_>) -> usize {
        plan.commands
            .iter()
            .filter(|command| {
                matches!(
                    command,
                    PaintCommand::FillRect { rect, .. }
                        if rect.w == 2 && rect.h == 10 && (120..212).contains(&rect.x)
                )
            })
            .count()
    }

    fn rect_inside(inner: ThemeRect, outer: ThemeRect) -> bool {
        inner.x >= outer.x
            && inner.y >= outer.y
            && inner.x + inner.w <= outer.x + outer.w
            && inner.y + inner.h <= outer.y + outer.h
    }
}
