use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use taffy::TaffyError;

use crate::{
    AlignItems, Color, Declaration, Description, FlexDirection, FontWeight, JustifyContent, NodeId,
    Rule, Selector, UiTree,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Clock {
    pub show_date: bool,
    pub format_12h: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClockState {
    pub hours: u8,
    pub minutes: u8,
    pub is_pm: bool,
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClockNodes {
    pub root: NodeId,
    pub face: NodeId,
    pub time: NodeId,
    pub am_pm: NodeId,
    pub date: Option<NodeId>,
}

impl Clock {
    pub const fn new() -> Self {
        Self { show_date: true, format_12h: true }
    }

    pub fn update_from_parts(
        &self,
        year: u16,
        month: u8,
        day: u8,
        hour_24: u8,
        minute: u8,
    ) -> ClockState {
        let is_pm = hour_24 >= 12;
        let hours = if self.format_12h {
            match hour_24 {
                0 => 12,
                13..=23 => hour_24 - 12,
                _ => hour_24,
            }
        } else {
            hour_24
        };

        ClockState { hours, minutes: minute, is_pm, day, month, year }
    }

    pub fn time_text(&self, state: &ClockState) -> String {
        if self.format_12h {
            format!("{:>2}:{:02}", state.hours, state.minutes)
        } else {
            format!("{:02}:{:02}", state.hours, state.minutes)
        }
    }

    pub fn am_pm_text(&self, state: &ClockState) -> &'static str {
        if state.is_pm { "PM" } else { "AM" }
    }

    pub fn date_text(&self, state: &ClockState) -> String {
        format!("{} {}, {}", month_name(state.month), state.day, state.year)
    }

    pub fn render(
        &self,
        tree: &mut UiTree,
        parent: NodeId,
        state: &ClockState,
    ) -> Result<ClockNodes, TaffyError> {
        let root =
            tree.add_node(&[Description::Perspective, Description::Clock, Description::Container])?;
        let face = tree.add_node(&[Description::ClockFace, Description::Container])?;
        let time = tree.text(&self.time_text(state))?;
        let am_pm = tree.text(self.am_pm_text(state))?;

        if let Some(node) = tree.node_mut(time) {
            node.descriptions.push(Description::ClockTime);
        }
        if let Some(node) = tree.node_mut(am_pm) {
            node.descriptions.push(Description::ClockAmPm);
        }

        tree.add_child(parent, root)?;
        tree.add_child(root, face)?;
        tree.add_child(face, time)?;
        tree.add_child(face, am_pm)?;

        let date = if self.show_date {
            let date = tree.text(&self.date_text(state))?;
            if let Some(node) = tree.node_mut(date) {
                node.descriptions.push(Description::ClockDate);
            }
            tree.add_child(root, date)?;
            Some(date)
        } else {
            None
        };

        Ok(ClockNodes { root, face, time, am_pm, date })
    }

    pub fn build_tree(&self, state: &ClockState) -> Result<(UiTree, ClockNodes), TaffyError> {
        let mut tree = UiTree::new()?;
        let root = tree.root();
        let nodes = self.render(&mut tree, root, state)?;
        tree.restyle(&self.rules())?;
        Ok((tree, nodes))
    }

    pub fn rules(&self) -> Vec<Rule<Description>> {
        let mut rules = alloc::vec![
            Rule::new(
                Selector::has(Description::Clock),
                alloc::vec![
                    Declaration::FlexDirection(FlexDirection::Column),
                    Declaration::AlignItems(AlignItems::Center),
                    Declaration::JustifyContent(JustifyContent::Center),
                    Declaration::Gap(4.0),
                    Declaration::Padding(12.0),
                ],
            ),
            Rule::new(
                Selector::has(Description::ClockFace),
                alloc::vec![
                    Declaration::FlexDirection(FlexDirection::Row),
                    Declaration::AlignItems(AlignItems::Center),
                    Declaration::JustifyContent(JustifyContent::Center),
                    Declaration::Gap(6.0),
                    Declaration::Height(40.0),
                ],
            ),
            Rule::new(
                Selector::has(Description::ClockTime),
                alloc::vec![
                    Declaration::Width(if self.format_12h { 92.0 } else { 104.0 }),
                    Declaration::Height(40.0),
                    Declaration::FontSize(32.0),
                    Declaration::FontWeight(FontWeight::Normal),
                    Declaration::Color(Color::rgb(242, 239, 232)),
                ],
            ),
            Rule::new(
                Selector::has(Description::ClockAmPm),
                alloc::vec![
                    Declaration::Width(if self.format_12h { 24.0 } else { 0.0 }),
                    Declaration::Height(18.0),
                    Declaration::FontSize(12.0),
                    Declaration::Color(Color::rgb(176, 184, 182)),
                ],
            ),
        ];

        if self.show_date {
            rules.push(Rule::new(
                Selector::has(Description::ClockDate),
                alloc::vec![
                    Declaration::Width(136.0),
                    Declaration::Height(20.0),
                    Declaration::FontSize(14.0),
                    Declaration::Color(Color::rgb(142, 152, 149)),
                ],
            ));
        }

        rules
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}

pub const fn month_name(month: u8) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "---",
    }
}

#[cfg(test)]
mod tests {
    use taffy::prelude::{AvailableSpace, Size};

    use super::*;
    use crate::ResolvedStyle;

    #[test]
    fn formats_noon_midnight_and_pm() {
        let clock = Clock::new();

        assert_eq!(clock.update_from_parts(2026, 5, 2, 0, 4).hours, 12);
        assert_eq!(clock.update_from_parts(2026, 5, 2, 12, 4).hours, 12);

        let evening = clock.update_from_parts(2026, 5, 2, 21, 41);
        assert_eq!(evening.hours, 9);
        assert_eq!(evening.minutes, 41);
        assert!(evening.is_pm);
        assert_eq!(clock.time_text(&evening), " 9:41");
        assert_eq!(clock.am_pm_text(&evening), "PM");
        assert_eq!(clock.date_text(&evening), "May 2, 2026");
    }

    #[test]
    fn supports_24_hour_time() {
        let clock = Clock { show_date: true, format_12h: false };
        let state = clock.update_from_parts(2026, 5, 2, 21, 41);

        assert_eq!(state.hours, 21);
        assert!(state.is_pm);
        assert_eq!(clock.time_text(&state), "21:41");
    }

    #[test]
    fn renders_centered_stacked_tree() {
        let clock = Clock::new();
        let state = clock.update_from_parts(2026, 5, 2, 21, 41);
        let (mut tree, nodes) = clock.build_tree(&state).unwrap();

        tree.apply_style(
            tree.root(),
            ResolvedStyle {
                width: Some(320.0),
                height: Some(200.0),
                flex_direction: Some(FlexDirection::Column),
                justify_content: Some(JustifyContent::Center),
                align_items: Some(AlignItems::Center),
                ..ResolvedStyle::default()
            },
        )
        .unwrap();
        tree.compute_layout(Size {
            width: AvailableSpace::Definite(320.0),
            height: AvailableSpace::Definite(200.0),
        })
        .unwrap();

        let clock_box = tree.global_layout_box(nodes.root).unwrap();
        let time_box = tree.global_layout_box(nodes.time).unwrap();
        let date_box = tree.global_layout_box(nodes.date.unwrap()).unwrap();

        assert!(clock_box.x > 60.0);
        assert!(clock_box.y > 40.0);
        assert!(time_box.y < date_box.y);
        assert_eq!(tree.node(nodes.time).unwrap().attrs.get("text").is_some(), true);
    }
}
