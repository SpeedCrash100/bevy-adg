use bevy::prelude::*;

/// Contains components used to controls engines or other things of the ship
pub mod control;

/// Simple triangle ship
mod simple;
pub use simple::ShipCreateInfoBuilder as SimpleShipBuilder;

/// Ship mark component
#[derive(Component)]
pub struct Ship;
