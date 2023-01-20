use bevy::prelude::*;

/// Generates asteroid
mod generate;

/// Build asteroid
mod builder;
pub use builder::AsteroidCreateInfoBuilder as AsteroidBuilder;

/// Asteroid levels descriptors
mod level;
pub use level::AsteroidSizeLevel;

/// Asteroid mark
#[derive(Component)]
pub struct Asteroid;
