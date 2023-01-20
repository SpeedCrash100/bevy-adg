use bevy::prelude::*;
use entity::EntityBuildDirector;
use physic_objects::prelude::*;

#[macro_use]
extern crate derive_builder;

mod components;
mod entity;
mod plugins;

use components::asteroid::*;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicObjectPlugin)
        .add_plugin(plugins::physics::PhysicsPlugin)
        .add_startup_system(create_camera)
        .add_startup_system(spawn_asteroid)
        .run();
}

pub fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_asteroid(mut commands: Commands) {
    let mut asteroid_builder = AsteroidBuilder::default();
    asteroid_builder.position(Vec2::ZERO);
    commands.build_entity(&asteroid_builder);
}
