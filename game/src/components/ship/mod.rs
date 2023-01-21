use bevy::prelude::*;

/// Simple triangle ship
mod simple;
pub use simple::ShipCreateInfoBuilder as SimpleShipBuilder;

/// Ship mark component
#[derive(Component)]
pub struct Ship;
