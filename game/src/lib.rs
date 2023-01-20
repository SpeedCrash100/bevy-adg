use bevy::prelude::*;
use physic_objects::prelude::*;

#[macro_use]
extern crate derive_builder;

mod components;
mod entity;
mod plugins;
mod random;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicObjectPlugin)
        .add_plugin(plugins::physics::PhysicsPlugin)
        .add_plugin(plugins::asteroid::AsteroidsPlugin)
        .add_startup_system(create_camera)
        .run();
}

pub fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
