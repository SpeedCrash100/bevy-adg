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
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicObjectPlugin)
        .add_plugin(plugins::physics::PhysicsPlugin)
        .add_plugin(plugins::living::LivingPlugin)
        .add_plugin(plugins::asteroid::AsteroidsPlugin)
        .add_plugin(plugins::ship::ShipPlugin)
        .add_plugin(plugins::player::PlayerPlugin)
        .run();
}
