pub const DEFAULT_THEME_NAME: &str = "Solarized Warm";

#[derive(Clone, Copy)]
pub struct UiTheme {
    pub name: &'static str,
    pub active: WindowStateTheme,
    pub inactive: WindowStateTheme,
    pub titlebar_height: u32,
    pub visual_border: u32,
    pub corner_radius: u32,
    pub focus_border: u32,
    pub inner_highlight: u32,
    pub chrome_text: u32,
    pub chrome_text_inactive: u32,
    pub control_icon: u32,
    pub control_icon_inactive: u32,
    pub close_icon: u32,
    pub title_rule_active: u32,
    pub title_rule_inactive: u32,
}

#[derive(Clone, Copy)]
pub struct WindowStateTheme {
    pub border: u32,
    pub title_top: u32,
    pub title_mid: u32,
    pub title_bottom: u32,
}

pub const SOLARIZED_WARM: UiTheme = UiTheme {
    name: DEFAULT_THEME_NAME,
    active: WindowStateTheme {
        border: 0xFFB58900,
        title_top: 0xFFF3CC58,
        title_mid: 0xFFE5B83F,
        title_bottom: 0xFFD49A20,
    },
    inactive: WindowStateTheme {
        border: 0xFFC9A94E,
        title_top: 0xFFF0D88A,
        title_mid: 0xFFE4CB72,
        title_bottom: 0xFFD6B95A,
    },
    titlebar_height: 30,
    visual_border: 1,
    corner_radius: 12,
    focus_border: 0xA6CB4B16,
    inner_highlight: 0x59FFFFFF,
    chrome_text: 0xFF3B2A0A,
    chrome_text_inactive: 0xFF6B5A3A,
    control_icon: 0xFF3B2A0A,
    control_icon_inactive: 0xFF6B5A3A,
    close_icon: 0xFF3B2A0A,
    title_rule_active: 0xB38C6500,
    title_rule_inactive: 0x8098780A,
};

pub fn default_theme() -> UiTheme {
    SOLARIZED_WARM
}

pub fn theme_by_name(name: &str) -> UiTheme {
    let trimmed = name.trim();
    if trimmed.eq_ignore_ascii_case("solarized warm")
        || trimmed.eq_ignore_ascii_case("solarized-warm")
        || trimmed.eq_ignore_ascii_case("solarized_warm")
    {
        SOLARIZED_WARM
    } else {
        default_theme()
    }
}
