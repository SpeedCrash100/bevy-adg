use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::health::{CollisionDamage, Health},
    states::GameState,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(no_gravity)
            .add_system_set(Self::update_systems())
            .add_system_set(Self::on_pause())
            .add_system_set(Self::on_continue());
    }
}

impl PhysicsPlugin {
    fn update_systems() -> SystemSet {
        SystemSet::on_update(GameState::InGame)
            .with_system(external_force_children_sum)
            .with_system(damage_collided_entities)
    }

    fn on_pause() -> SystemSet {
        SystemSet::on_enter(GameState::Pause).with_system(pause_physic)
    }

    fn on_continue() -> SystemSet {
        SystemSet::on_exit(GameState::Pause).with_system(unpause_physic)
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

        let impulse = e.total_force_magnitude / 100_000.0;

        first_hp.damage(impulse);
        second_hp.damage(impulse);
    }
}

fn pause_physic(mut config: ResMut<RapierConfiguration>) {
    config.physics_pipeline_active = false;
}

fn unpause_physic(mut config: ResMut<RapierConfiguration>) {
    config.physics_pipeline_active = true;
}
