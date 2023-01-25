use bevy::prelude::{App, DefaultPlugins};
use physic_objects::prelude::*;

#[macro_use]
extern crate derive_builder;

mod components;
mod entity;
mod math;
mod plugins;
mod random;
mod stages;

pub fn run() {
    let mut app = App::new();

    // base compnents
    app.add_plugins(DefaultPlugins)
        .add_plugin(PhysicObjectPlugin)
        .add_plugin(plugins::physics::PhysicsPlugin)
        .add_plugin(plugins::living::LivingPlugin)
        .add_plugin(plugins::despawn::DespawnPlugin)
        .add_plugin(plugins::asteroid::AsteroidsPlugin)
        .add_plugin(plugins::ship::ShipPlugin)
        .add_plugin(plugins::player::PlayerPlugin)
        .add_plugin(plugins::weapon::WeaponPlugin);

    // Debug only components
    if cfg!(feature = "debug_hp") {
        app.add_plugin(plugins::healthdisplay::HealthDisplayPlugin);
    }

    // Start up
    app.run();
}
