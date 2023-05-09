use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::health::{CollisionDamage, Health, Immortality, TimedImmortalityBundle},
    states::GameState,
};

const IMMORTALITY_AFTER_COLLIDE_TIME: f32 = 0.1;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(no_gravity);

        // Update forces
        app.add_system(
            external_force_to_update
                .pipe(external_forces_sum)
                .pipe(external_forces_apply)
                .in_set(OnUpdate(GameState::InGame)),
        );
        // Damage
        app.add_system(damage_collided_entities.in_set(OnUpdate(GameState::InGame)));

        // Pausing
        app.add_systems((
            pause_physic.in_schedule(OnExit(GameState::InGame)),
            unpause_physic.in_schedule(OnEnter(GameState::InGame)),
        ));
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
    mut commands: Commands,
    mut collision_events: EventReader<ContactForceEvent>,
    mut q_entities: Query<(&mut Health, Entity), (With<CollisionDamage>, Without<Immortality>)>,
) {
    for e in collision_events.iter() {
        let first = e.collider1;
        let second = e.collider2;

        let Ok(entities) = q_entities.get_many_mut([first, second]) else {
                continue;
            };

        let mut entities = Vec::from(entities);

        let (mut first_hp, first_entity) = entities.swap_remove(0);
        let (mut second_hp, second_entity) = entities.swap_remove(0);

        let impulse = e.total_force_magnitude / 100_000.0;

        first_hp.damage(impulse);
        second_hp.damage(impulse);

        commands
            .entity(first_entity)
            .insert(TimedImmortalityBundle::new(IMMORTALITY_AFTER_COLLIDE_TIME));

        commands
            .entity(second_entity)
            .insert(TimedImmortalityBundle::new(IMMORTALITY_AFTER_COLLIDE_TIME));
    }
}

fn pause_physic(mut config: ResMut<RapierConfiguration>) {
    config.physics_pipeline_active = false;
}

fn unpause_physic(mut config: ResMut<RapierConfiguration>) {
    config.physics_pipeline_active = true;
}
