use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(no_gravity)
            .add_system(external_force_children_sum);
    }
}

fn no_gravity(mut physic_cfg: ResMut<RapierConfiguration>) {
    physic_cfg.gravity = [0.0, 0.0].into();
}

/// Sum all forces in children elements and apply result to parent
fn external_force_children_sum(
    mut q_parent: Query<(&mut ExternalForce, &Children), With<RigidBody>>,
    q_childs: Query<&ExternalForce, Without<RigidBody>>,
) {
    for (mut parent_force, childrens) in q_parent.iter_mut() {
        *parent_force = ExternalForce::default(); // Reset

        for child in childrens.iter() {
            let Ok(force) = q_childs.get(*child) else {
                continue;
            };

            parent_force.force += force.force;
            parent_force.torque += force.torque;
        }
    }
}
