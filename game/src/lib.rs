use bevy::prelude::*;
use physic_objects::prelude::*;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicObjectPlugin)
        .run();
}
