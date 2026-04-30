pub const DEFAULT_THEME_NAME: &str = "Solarized Warm";

#[derive(Clone, Copy)]
pub struct UiTheme {
    pub name: &'static str,
    pub active: WindowStateTheme,
    pub inactive: WindowStateTheme,
    pub titlebar_height: u32,
    pub visual_border: u32,
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
        title_top: 0xFFF6DFA0,
        title_mid: 0xFFE9C75A,
        title_bottom: 0xFFD8A92F,
    },
    inactive: WindowStateTheme {
        border: 0x7A586E75,
        title_top: 0xFFF2E6C7,
        title_mid: 0xFFE8D8AE,
        title_bottom: 0xFFD9C58F,
    },
    titlebar_height: 30,
    visual_border: 1,
    focus_border: 0xB8CB4B16,
    inner_highlight: 0x61FFFFFF,
    chrome_text: 0xFF3B2A0A,
    chrome_text_inactive: 0xFF657B83,
    control_icon: 0xFF3B2A0A,
    control_icon_inactive: 0xFF657B83,
    close_icon: 0xFF7A3C19,
    title_rule_active: 0xB88C6500,
    title_rule_inactive: 0x4D586E75,
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
