//! Projectile entity implementations.

mod ender_pearl;
mod firework_rocket;
mod fishing_hook;
mod thrown_egg;

pub use ender_pearl::EnderPearlEntity;
pub use firework_rocket::FireworkRocketEntity;
pub use fishing_hook::{FishingHookEntity, FishingHookState};
pub use thrown_egg::ThrownEggEntity;