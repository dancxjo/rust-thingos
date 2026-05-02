use alloc::vec::Vec;

use crate::rule::Rule;
use crate::selector::StylableSurface;
use crate::values::ResolvedStyle;

pub fn compute_for<D, E>(element: &E, rules: &[Rule<D>]) -> ResolvedStyle
where
    D: PartialEq,
    E: StylableSurface<D>,
{
    let mut resolved = ResolvedStyle::default();

    for rule in rules {
        if !rule.selector.matches(element) {
            continue;
        }

        for declaration in &rule.declarations {
            if element.supports_property(declaration.property()) {
                declaration.apply_to(&mut resolved);
            }
        }
    }

    resolved
}

pub fn compute<D, E>(elements: &[E], rules: &[Rule<D>]) -> Vec<ResolvedStyle>
where
    D: PartialEq,
    E: StylableSurface<D>,
{
    elements.iter().map(|element| compute_for(element, rules)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::{Color, State, StateSet, StyleProperty};
    use crate::{Declaration, Selector};

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Description {
        Pressable,
        Textual,
    }

    struct Element {
        descriptions: &'static [Description],
        states: StateSet,
    }

    impl StylableSurface<Description> for Element {
        fn descriptions(&self) -> &[Description] {
            self.descriptions
        }

        fn states(&self) -> StateSet {
            self.states
        }

        fn supports_property(&self, property: StyleProperty) -> bool {
            matches!(property, StyleProperty::BackgroundColor | StyleProperty::FontSize)
        }
    }

    #[test]
    fn later_matching_rules_override_earlier_values() {
        let element =
            Element { descriptions: &[Description::Pressable], states: StateSet::empty() };
        let rules = alloc::vec![
            Rule::new(
                Selector::has(Description::Pressable),
                alloc::vec![Declaration::BackgroundColor(Color::rgb(1, 2, 3))],
            ),
            Rule::new(
                Selector::has(Description::Pressable),
                alloc::vec![Declaration::BackgroundColor(Color::rgb(4, 5, 6))],
            ),
        ];

        assert_eq!(compute_for(&element, &rules).background_color, Some(Color::rgb(4, 5, 6)));
    }

    #[test]
    fn state_selectors_only_match_active_states() {
        let mut states = StateSet::empty();
        states.insert(State::Focus);
        let element = Element { descriptions: &[Description::Textual], states };
        let rules = alloc::vec![Rule::new(
            Selector::has(Description::Textual).and(Selector::state(State::Focus)),
            alloc::vec![Declaration::FontSize(18.0)],
        )];

        assert_eq!(compute_for(&element, &rules).font_size, Some(18.0));
    }
}
