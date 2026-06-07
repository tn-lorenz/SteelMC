//! Vanilla-shaped mob AI foundations.
#![expect(
    dead_code,
    reason = "pathfinding controls are foundation code consumed by upcoming goals and navigation"
)]

pub mod control;
pub mod goal;
pub mod navigation;
pub mod node;
pub mod path;
pub mod pathfinder;
pub mod walk;
