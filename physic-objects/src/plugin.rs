use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;

pub struct PhysicObjectPlugin;

impl Plugin for PhysicObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ShapePlugin);
    }
}
