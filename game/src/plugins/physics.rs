use std::collections::HashMap;

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
            .add_startup_system(no_gravity);

        for system_set in Self::get_update_system_sets() {
            app.add_system_set(system_set);
        }
    }
}

impl PhysicsPlugin {
    fn get_update_system_sets() -> Vec<SystemSet> {
        vec![
            Self::update_systems(),
            Self::on_pause(),
            Self::on_continue(),
            Self::on_enter_in_game(),
            Self::on_exit_from_game(),
        ]
    }

    fn update_systems() -> SystemSet {
        SystemSet::on_update(GameState::InGame)
            .with_system(
                external_force_to_update
                    .pipe(external_forces_sum)
                    .pipe(external_forces_apply),
            )
            .with_system(damage_collided_entities)
    }

    fn on_pause() -> SystemSet {
        SystemSet::on_enter(GameState::Pause).with_system(pause_physic)
    }

    fn on_continue() -> SystemSet {
        SystemSet::on_exit(GameState::Pause).with_system(unpause_physic)
    }

    fn on_enter_in_game() -> SystemSet {
        SystemSet::on_enter(GameState::InGame).with_system(unpause_physic)
    }

    fn on_exit_from_game() -> SystemSet {
        SystemSet::on_exit(GameState::InGame).with_system(pause_physic)
    }
}

fn no_gravity(mut physic_cfg: ResMut<RapierConfiguration>) {
    physic_cfg.gravity = [0.0, 0.0].into();
}

fn external_force_to_update(
    q_changed_childs: Query<&Parent, Changed<ExternalForce>>,
    q_highest: Query<(&ExternalForce, Entity), (Without<Parent>, With<Children>)>,
    q_childs: Query<(&ExternalForce, &Parent)>,
) -> Vec<Entity> {
    let mut elements_to_update = Vec::new();

    for parent in q_changed_childs.iter() {
        let mut highest = parent.get();
        elements_to_update.push(highest);

        while !q_highest.contains(highest) {
            match q_childs.get(highest) {
                Ok((_, new_parent)) => {
                    highest = new_parent.get();
                    elements_to_update.push(highest);
                }
                Err(_) => break,
            };
        }
    }

    elements_to_update
}

fn external_forces_sum(
    In(entities): In<Vec<Entity>>,
    q_parent: Query<&Children>,
    q_force: Query<&ExternalForce>,
) -> HashMap<Entity, ExternalForce> {
    let mut new_forces_map = HashMap::new();

    for entity in entities {
        let children = q_parent.get(entity).unwrap();
        let mut new_force = ExternalForce::default();

        for child in children.iter() {
            let Ok(force) = q_force.get(*child) else {
                            continue;
                        };

            new_force.force += force.force;
            new_force.torque += force.torque;
        }

        new_forces_map.insert(entity, new_force);
    }

    new_forces_map
}

fn external_forces_apply(
    In(forces_map): In<HashMap<Entity, ExternalForce>>,
    mut q_force: Query<&mut ExternalForce>,
) {
    for (entity, new_force) in forces_map.iter() {
        let mut force = q_force.get_mut(*entity).unwrap();
        *force = *new_force;
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
