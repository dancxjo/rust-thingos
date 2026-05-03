use stile::{
    PaintCommand, PaintList, Theme, ThemeControl, ThemeControlRect, ThemeRect, ThemeState,
    WindowChromeRequest,
};

use crate::Rect;
use crate::wm::{ChromeButton, SurfaceChrome, chrome_button_rects, surface_visual_rect};

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

pub type WindowPaintPlan<'a> = PaintList<'a>;
pub type WindowPaintCommand<'a> = PaintCommand<'a>;

pub fn window_chrome_plan<'a>(
    rect: Rect,
    chrome: SurfaceChrome,
    theme: Theme,
    state: ChromeVisualState,
    title: Option<&'a str>,
) -> WindowPaintPlan<'a> {
    let mut plan = PaintList::default();
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
    let controls = chrome_button_rects(rect, chrome).map(theme_control_rects).unwrap_or_default();

    theme.render_window_chrome(
        WindowChromeRequest {
            visual_rect: to_theme_rect(visual_rect),
            content_rect: to_theme_rect(rect),
            frame,
            titlebar_height,
            controls,
            state: ThemeState {
                active: state.active,
                hovered: state.hovered,
                primary_button_down: state.primary_button_down,
                pointer_x: state.pointer_x,
                pointer_y: state.pointer_y,
                shaded: state.shaded,
                fullscreen: state.fullscreen,
            },
            title,
        },
        &mut plan,
    );

    plan
}

fn theme_control_rects(buttons: [(ChromeButton, Rect); 3]) -> [Option<ThemeControlRect>; 3] {
    [
        Some(ThemeControlRect {
            control: to_theme_control(buttons[0].0),
            rect: to_theme_rect(buttons[0].1),
        }),
        Some(ThemeControlRect {
            control: to_theme_control(buttons[1].0),
            rect: to_theme_rect(buttons[1].1),
        }),
        Some(ThemeControlRect {
            control: to_theme_control(buttons[2].0),
            rect: to_theme_rect(buttons[2].1),
        }),
    ]
}

fn to_theme_control(button: ChromeButton) -> ThemeControl {
    match button {
        ChromeButton::Minimize | ChromeButton::Shade => ThemeControl::Shade,
        ChromeButton::Fullscreen | ChromeButton::Maximize => ThemeControl::Fullscreen,
        ChromeButton::Close => ThemeControl::Close,
    }
}

fn to_theme_rect(rect: Rect) -> ThemeRect {
    ThemeRect { x: rect.x, y: rect.y, w: rect.w, h: rect.h }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chrome_plan_contains_frame_title_and_control_pads() {
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
            plan.commands.iter().any(|cmd| matches!(cmd, PaintCommand::VerticalGradient { .. }))
        );
        assert!(plan.commands.iter().any(|cmd| matches!(cmd, PaintCommand::Shadow { .. })));
        assert!(plan.commands.iter().any(|cmd| matches!(cmd, PaintCommand::Text { .. })));
        assert!(plan.commands.iter().any(|cmd| matches!(cmd, PaintCommand::FillRect { .. })));
    }
}
