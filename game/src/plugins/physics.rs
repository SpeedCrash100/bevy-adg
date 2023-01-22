use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::health::{CollisionDamage, Health};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(no_gravity)
            .add_system(external_force_children_sum)
            .add_system(damage_collided_entities);
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

fn damage_collided_entities(
    mut collision_events: EventReader<ContactForceEvent>,
    mut q_entities: Query<&mut Health, With<CollisionDamage>>,
) {
    for e in collision_events.iter() {
        let first = e.collider1;
        let second = e.collider2;

        let Ok(entities) = q_entities.get_many_mut([first, second]) else {
                continue;
            };

        let mut entities = Vec::from(entities);

        let mut first_hp = entities.swap_remove(0);
        let mut second_hp = entities.swap_remove(0);

        let impulse = e.total_force_magnitude / 1_000_000.0;

        first_hp.damage(impulse);
        second_hp.damage(impulse);
    }
}
