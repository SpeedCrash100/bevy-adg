use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(no_gravity);
    }
}

fn no_gravity(mut physic_cfg: ResMut<RapierConfiguration>) {
    physic_cfg.gravity = [0.0, 0.0].into();
}
