//! Vanilla-shaped goal selector and movement goals.

mod random_pos;
mod random_stroll;
mod selector;
mod water_avoiding_random_stroll;

pub(crate) use selector::GoalSelector;
pub(crate) use water_avoiding_random_stroll::WaterAvoidingRandomStrollGoal;
