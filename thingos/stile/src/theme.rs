pub const DEFAULT_THEME_NAME: &str = "Facet Frame";

#[derive(Clone, Copy)]
pub struct UiTheme {
    pub name: &'static str,
    pub chrome_style: ChromeStyle,
    pub active: WindowStateTheme,
    pub inactive: WindowStateTheme,
    pub titlebar_height: u32,
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
pub enum ChromeStyle {
    Facet,
    Ghost,
}

#[derive(Clone, Copy)]
pub struct WindowStateTheme {
    pub border: u32,
    pub title_top: u32,
    pub title_bottom: u32,
}

pub const FACET_FRAME: UiTheme = UiTheme {
    name: DEFAULT_THEME_NAME,
    chrome_style: ChromeStyle::Facet,
    active: WindowStateTheme {
        border: 0xFF0F0C18,
        title_top: 0xFF2C2140,
        title_bottom: 0xFF14101D,
    },
    inactive: WindowStateTheme {
        border: 0xFF0F0C18,
        title_top: 0xFF1C1628,
        title_bottom: 0xFF100D17,
    },
    titlebar_height: 28,
    corner_radius: 0,
    chrome_text: 0xFFE6E1FF,
    chrome_text_inactive: 0xFF6E6599,
    control_icon: 0xFFB8A8FF,
    control_icon_inactive: 0xFF6E6599,
    close_icon: 0xFFB8A8FF,
    title_rule_active: 0xFF2A1F3A,
    title_rule_inactive: 0xFF1B1426,
    body_top: 0xFF0B0A10,
    button_top: 0xFF2A1F3A,
    outer_stroke: 0xFF0F0C18,
    inner_stroke: 0xFF2A1F3A,
    inner_stroke_inactive: 0xFF1B1426,
    frame_fill: 0xFF1A1424,
    frame_fill_inactive: 0xFF14101C,
    frame_fill_bottom: 0xFF0F0C18,
    frame_fill_bottom_inactive: 0xFF0B0910,
    frame_bevel_light: 0xFF6D53D9,
    frame_bevel_light_inactive: 0xFF2D2440,
    frame_bevel_shadow: 0xFF08060C,
    title_sheen: 0xFF4A3865,
    facet: 0xFF231A33,
    facet_inactive: 0xFF1A1426,
    focus_accent: 0xFFF2C94C,
    edge_light: 0xFF7C5CFF,
    edge_dark: 0xFF120E18,
    content_edge: 0xFF221A30,
    grid_line: 0x26120E18,
    contact_shadow: 0x66000000,
};

pub const GHOST_FLOWER: UiTheme = UiTheme {
    name: "Ghost Flower",
    chrome_style: ChromeStyle::Ghost,
    active: WindowStateTheme {
        border: 0x8A2A1742,
        title_top: 0x5A42246B,
        title_bottom: 0x24150C27,
    },
    inactive: WindowStateTheme {
        border: 0x54211434,
        title_top: 0x32150C27,
        title_bottom: 0x16030108,
    },
    titlebar_height: 28,
    corner_radius: 0,
    chrome_text: 0xFFF2E8FF,
    chrome_text_inactive: 0xB89584B8,
    control_icon: 0xFFE2C8FF,
    control_icon_inactive: 0xA77C6899,
    close_icon: 0xFFFFC6D7,
    title_rule_active: 0x667957A8,
    title_rule_inactive: 0x33281740,
    body_top: 0xE60A0612,
    button_top: 0x4D8457B0,
    outer_stroke: 0x707957A8,
    inner_stroke: 0x55C7A2FF,
    inner_stroke_inactive: 0x2B5C407A,
    frame_fill: 0x30211434,
    frame_fill_inactive: 0x180A0612,
    frame_fill_bottom: 0x22030108,
    frame_fill_bottom_inactive: 0x10020107,
    frame_bevel_light: 0x80C7A2FF,
    frame_bevel_light_inactive: 0x4D5E417C,
    frame_bevel_shadow: 0x66000000,
    title_sheen: 0x668457B0,
    facet: 0x403B2460,
    facet_inactive: 0x22150C27,
    focus_accent: 0xFFD8A657,
    edge_light: 0xB88457B0,
    edge_dark: 0x99020107,
    content_edge: 0x5542246B,
    grid_line: 0x1F8457B0,
    contact_shadow: 0x8A000000,
};

pub const SOLAR_WARM_SPINE: UiTheme = UiTheme {
    name: "Solar Warm Spine",
    chrome_style: ChromeStyle::Facet,
    active: WindowStateTheme {
        border: 0xFF2A1810,
        title_top: 0xFF5A3C21,
        title_bottom: 0xFF24130D,
    },
    inactive: WindowStateTheme {
        border: 0xFF241710,
        title_top: 0xFF33261A,
        title_bottom: 0xFF1B120D,
    },
    titlebar_height: 28,
    corner_radius: 0,
    chrome_text: 0xFFFFF1C8,
    chrome_text_inactive: 0xFFB59C70,
    control_icon: 0xFFFFD45C,
    control_icon_inactive: 0xFF92784D,
    close_icon: 0xFFFFAA9B,
    title_rule_active: 0xFF7A4A1A,
    title_rule_inactive: 0xFF402817,
    body_top: 0xFF17120C,
    button_top: 0xFF6C431B,
    outer_stroke: 0xFF1A0D08,
    inner_stroke: 0xFF7B5B22,
    inner_stroke_inactive: 0xFF3A2B18,
    frame_fill: 0xFF3B2514,
    frame_fill_inactive: 0xFF24180F,
    frame_fill_bottom: 0xFF1B0F09,
    frame_fill_bottom_inactive: 0xFF120C08,
    frame_bevel_light: 0xFFE1C042,
    frame_bevel_light_inactive: 0xFF6A5B32,
    frame_bevel_shadow: 0xFF0B0705,
    title_sheen: 0xFFA76020,
    facet: 0xFF4A2C16,
    facet_inactive: 0xFF2F2014,
    focus_accent: 0xFFE1C042,
    edge_light: 0xFFC77619,
    edge_dark: 0xFF120905,
    content_edge: 0xFF3D2817,
    grid_line: 0x244D4E2E,
    contact_shadow: 0x66000000,
};

pub const LEATHER_GRAIN: UiTheme = UiTheme {
    name: "Leather Grain",
    chrome_style: ChromeStyle::Facet,
    active: WindowStateTheme {
        border: 0xFF171318,
        title_top: 0xFF342C35,
        title_bottom: 0xFF1F1A20,
    },
    inactive: WindowStateTheme {
        border: 0xFF171318,
        title_top: 0xFF272128,
        title_bottom: 0xFF1C171D,
    },
    titlebar_height: 28,
    corner_radius: 0,
    chrome_text: 0xFFF0E6EE,
    chrome_text_inactive: 0xFF9D8F9A,
    control_icon: 0xFFD8C3D2,
    control_icon_inactive: 0xFF8D7F89,
    close_icon: 0xFFFFB9C0,
    title_rule_active: 0xFF4B3E49,
    title_rule_inactive: 0xFF2B242C,
    body_top: 0xFF171318,
    button_top: 0xFF3E343F,
    outer_stroke: 0xFF120F13,
    inner_stroke: 0xFF4B3E49,
    inner_stroke_inactive: 0xFF2B242C,
    frame_fill: 0xFF2B242C,
    frame_fill_inactive: 0xFF221C23,
    frame_fill_bottom: 0xFF1A151B,
    frame_fill_bottom_inactive: 0xFF171318,
    frame_bevel_light: 0xFF6E5F6A,
    frame_bevel_light_inactive: 0xFF3A313A,
    frame_bevel_shadow: 0xFF0C090D,
    title_sheen: 0xFF4E414E,
    facet: 0xFF342C35,
    facet_inactive: 0xFF241E25,
    focus_accent: 0xFFC79A70,
    edge_light: 0xFF8F6E5A,
    edge_dark: 0xFF120F13,
    content_edge: 0xFF342C35,
    grid_line: 0x242B242C,
    contact_shadow: 0x70000000,
};

pub const LINEN_LIGHT: UiTheme = UiTheme {
    name: "Linen Light",
    chrome_style: ChromeStyle::Facet,
    active: WindowStateTheme {
        border: 0xFFB8AA9A,
        title_top: 0xFFFFFBF4,
        title_bottom: 0xFFE9DED2,
    },
    inactive: WindowStateTheme {
        border: 0xFFCFC4B8,
        title_top: 0xFFF8F2EA,
        title_bottom: 0xFFE9E0D7,
    },
    titlebar_height: 28,
    corner_radius: 0,
    chrome_text: 0xFF2A2520,
    chrome_text_inactive: 0xFF7B7066,
    control_icon: 0xFF3E352D,
    control_icon_inactive: 0xFF8C8176,
    close_icon: 0xFF8E2F2B,
    title_rule_active: 0xFFD3C3B1,
    title_rule_inactive: 0xFFE2D8CD,
    body_top: 0xFFFBF7F1,
    button_top: 0xFFE8DCCF,
    outer_stroke: 0xFFB8AA9A,
    inner_stroke: 0xFFFFFFFF,
    inner_stroke_inactive: 0xFFF3E9DF,
    frame_fill: 0xFFF7F1E9,
    frame_fill_inactive: 0xFFF0E8DF,
    frame_fill_bottom: 0xFFE7DDD2,
    frame_fill_bottom_inactive: 0xFFE1D7CC,
    frame_bevel_light: 0xFFFFFFFF,
    frame_bevel_light_inactive: 0xFFF8F2EA,
    frame_bevel_shadow: 0xFFB8AA9A,
    title_sheen: 0xFFFFFFFF,
    facet: 0xFFEFE5DA,
    facet_inactive: 0xFFE8DED3,
    focus_accent: 0xFF7D5B2E,
    edge_light: 0xFFD6BE9E,
    edge_dark: 0xFFA99A8A,
    content_edge: 0xFFD7CCC0,
    grid_line: 0x225E554C,
    contact_shadow: 0x33000000,
};

pub fn default_theme() -> UiTheme {
    FACET_FRAME
}

pub fn theme_by_name(name: &str) -> UiTheme {
    let trimmed = name.trim();
    if trimmed.eq_ignore_ascii_case("facet frame")
        || trimmed.eq_ignore_ascii_case("facet-frame")
        || trimmed.eq_ignore_ascii_case("facet_frame")
        || trimmed.eq_ignore_ascii_case("bloom chrome")
        || trimmed.eq_ignore_ascii_case("bloom-chrome")
        || trimmed.eq_ignore_ascii_case("bloom_chrome")
    {
        FACET_FRAME
    } else if trimmed.eq_ignore_ascii_case("ghost flower")
        || trimmed.eq_ignore_ascii_case("ghost-flower")
        || trimmed.eq_ignore_ascii_case("ghost_flower")
        || trimmed.eq_ignore_ascii_case("flower.png")
        || trimmed.eq_ignore_ascii_case("/public/wallpapers/flower.png")
    {
        GHOST_FLOWER
    } else if trimmed.eq_ignore_ascii_case("solar warm spine")
        || trimmed.eq_ignore_ascii_case("solar-warm-spine")
        || trimmed.eq_ignore_ascii_case("solar_warm_spine")
        || trimmed.eq_ignore_ascii_case("flat warm")
        || trimmed.eq_ignore_ascii_case("flat-warm")
        || trimmed.eq_ignore_ascii_case("flat_warm")
        || trimmed.eq_ignore_ascii_case("solarized warm")
        || trimmed.eq_ignore_ascii_case("solarized-warm")
        || trimmed.eq_ignore_ascii_case("solarized_warm")
        || trimmed.eq_ignore_ascii_case("flower.bmp")
        || trimmed.eq_ignore_ascii_case("/public/wallpapers/flower.bmp")
    {
        SOLAR_WARM_SPINE
    } else if trimmed.eq_ignore_ascii_case("leather grain")
        || trimmed.eq_ignore_ascii_case("leather-grain")
        || trimmed.eq_ignore_ascii_case("leather_grain")
        || trimmed.eq_ignore_ascii_case("leather.bmp")
        || trimmed.eq_ignore_ascii_case("/public/wallpapers/leather.bmp")
    {
        LEATHER_GRAIN
    } else if trimmed.eq_ignore_ascii_case("linen light")
        || trimmed.eq_ignore_ascii_case("linen-light")
        || trimmed.eq_ignore_ascii_case("linen_light")
        || trimmed.eq_ignore_ascii_case("linen.bmp")
        || trimmed.eq_ignore_ascii_case("/public/wallpapers/linen.bmp")
    {
        LINEN_LIGHT
    } else {
        default_theme()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_wallpaper_theme_aliases_resolve_to_matching_chrome() {
        assert_eq!(theme_by_name("flower.png").name, GHOST_FLOWER.name);
        assert_eq!(theme_by_name("flower.png").chrome_style, ChromeStyle::Ghost);
        assert_eq!(theme_by_name("flower.bmp").name, SOLAR_WARM_SPINE.name);
        assert_eq!(theme_by_name("leather.bmp").name, LEATHER_GRAIN.name);
        assert_eq!(theme_by_name("linen.bmp").name, LINEN_LIGHT.name);
    }
}
