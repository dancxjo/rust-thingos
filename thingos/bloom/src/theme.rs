pub const DEFAULT_THEME_NAME: &str = "Facet Frame";

#[derive(Clone, Copy)]
pub struct UiTheme {
    pub name: &'static str,
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
    pub facet: u32,
    pub facet_inactive: u32,
    pub focus_accent: u32,
    pub edge_light: u32,
    pub edge_dark: u32,
    pub content_edge: u32,
    pub grid_line: u32,
    pub contact_shadow: u32,
}

#[derive(Clone, Copy)]
pub struct WindowStateTheme {
    pub border: u32,
    pub title_top: u32,
}

pub const FACET_FRAME: UiTheme = UiTheme {
    name: DEFAULT_THEME_NAME,
    active: WindowStateTheme { border: 0xFF0F0C18, title_top: 0xFF1A1424 },
    inactive: WindowStateTheme { border: 0xFF0F0C18, title_top: 0xFF14101C },
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
    facet: 0xFF231A33,
    facet_inactive: 0xFF1A1426,
    focus_accent: 0xFFF2C94C,
    edge_light: 0xFF7C5CFF,
    edge_dark: 0xFF120E18,
    content_edge: 0xFF221A30,
    grid_line: 0x26120E18,
    contact_shadow: 0x66000000,
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
        || trimmed.eq_ignore_ascii_case("flat warm")
        || trimmed.eq_ignore_ascii_case("flat-warm")
        || trimmed.eq_ignore_ascii_case("flat_warm")
        || trimmed.eq_ignore_ascii_case("solarized warm")
        || trimmed.eq_ignore_ascii_case("solarized-warm")
        || trimmed.eq_ignore_ascii_case("solarized_warm")
    {
        FACET_FRAME
    } else {
        default_theme()
    }
}
