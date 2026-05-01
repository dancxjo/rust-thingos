pub const DEFAULT_THEME_NAME: &str = "Flat Warm";

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
}

#[derive(Clone, Copy)]
pub struct WindowStateTheme {
    pub border: u32,
    pub title_top: u32,
}

pub const SOLARIZED_WARM: UiTheme = UiTheme {
    name: DEFAULT_THEME_NAME,
    active: WindowStateTheme {
        border: 0xFFB8A060,
        title_top: 0xFFD6C07A,
    },
    inactive: WindowStateTheme {
        border: 0xFFB8A060,
        title_top: 0xFFD6C07A,
    },
    titlebar_height: 30,
    corner_radius: 0,
    chrome_text: 0xFF3B2A0A,
    chrome_text_inactive: 0xFF6B5A3A,
    control_icon: 0xFF3B2A0A,
    control_icon_inactive: 0xFF6B5A3A,
    close_icon: 0xFF3B2A0A,
    title_rule_active: 0xFF8C743A,
    title_rule_inactive: 0xFF8C743A,
    body_top: 0xFFF4E6BF,
    button_top: 0xFF806A35,
};

pub fn default_theme() -> UiTheme {
    SOLARIZED_WARM
}

pub fn theme_by_name(name: &str) -> UiTheme {
    let trimmed = name.trim();
    if trimmed.eq_ignore_ascii_case("flat warm")
        || trimmed.eq_ignore_ascii_case("flat-warm")
        || trimmed.eq_ignore_ascii_case("flat_warm")
        || trimmed.eq_ignore_ascii_case("solarized warm")
        || trimmed.eq_ignore_ascii_case("solarized-warm")
        || trimmed.eq_ignore_ascii_case("solarized_warm")
    {
        SOLARIZED_WARM
    } else {
        default_theme()
    }
}
