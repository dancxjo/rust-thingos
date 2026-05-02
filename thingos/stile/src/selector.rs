use alloc::boxed::Box;

use crate::values::{State, StateSet, StyleProperty};

pub trait StylableSurface<D> {
    fn descriptions(&self) -> &[D];
    fn states(&self) -> StateSet;

    fn supports_property(&self, _property: StyleProperty) -> bool {
        true
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Selector<D> {
    Has(D),
    And(Box<Selector<D>>, Box<Selector<D>>),
    State(State),
}

impl<D> Selector<D> {
    pub fn has(description: D) -> Self {
        Self::Has(description)
    }

    pub fn and(self, other: Selector<D>) -> Self {
        Self::And(Box::new(self), Box::new(other))
    }

    pub fn state(state: State) -> Self {
        Self::State(state)
    }
}

impl<D: PartialEq> Selector<D> {
    pub fn matches<E>(&self, element: &E) -> bool
    where
        E: StylableSurface<D>,
    {
        match self {
            Selector::Has(description) => element.descriptions().iter().any(|d| d == description),
            Selector::And(left, right) => left.matches(element) && right.matches(element),
            Selector::State(state) => element.states().contains_state(*state),
        }
    }
}
