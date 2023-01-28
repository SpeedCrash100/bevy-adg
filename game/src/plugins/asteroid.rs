use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use physic_objects::PhysicObjectBundle;
use rand::Rng;

use crate::{
    components::{
        asteroid::{Asteroid, AsteroidBuilder, AsteroidSizeLevel},
        common::{Despawn, DespawnOn},
        health::Dead,
        player::Player,
    },
    entity::{ComponentInjectorBuilder, EntityBuildDirector},
    math::Position,
    random::Deviate,
    stages::LivingStages,
    states::GameState,
};

/// Minimal range from screen border to spawn asteroid
const MIN_SPAWN_RANGE: f32 = 200.0;
/// Maximum range from screen border to spawn asteroid
const MAX_SPAWN_RANGE: f32 = 1200.0;

/// Target Asteroid count in world
#[derive(Resource)]
pub struct AsteroidCount(usize);

impl Default for AsteroidCount {
    fn default() -> Self {
        Self(100)
    }
}

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsteroidCount>()
            .add_system_set(Self::spawn_asteroid_systems())
            .add_system_set(Self::despawn_asteroids_on_exit())
            .add_system_set_to_stage(LivingStages::DeadProcessing, Self::dead_handling());
    }
}

impl AsteroidsPlugin {
    fn spawn_asteroid_systems() -> SystemSet {
        SystemSet::on_update(GameState::InGame).with_system(asteroids_spawn_system)
    }

    fn dead_handling() -> SystemSet {
        SystemSet::new().with_system(asteroid_dead)
    }

    fn despawn_asteroids_on_exit() -> SystemSet {
        SystemSet::on_exit(GameState::InGame).with_system(destroy_asteroids)
    }
}

/// Spawn asteroids
fn asteroids_spawn_system(
    mut commands: Commands,
    window: Res<Windows>,
    asteroids_count: Res<AsteroidCount>,
    asteroids: Query<(), With<Asteroid>>,
    player: Query<&Transform, With<Player>>,
) {
    let window = window.get_primary().unwrap();

    let radius_w = window.width() / 2.0;
    let radius_h = window.width() / 2.0;
    let radius_diagonal = (radius_w.powi(2) + radius_h.powi(2)).sqrt();

    let asteroid_spawned = asteroids.iter().count();
    let mut rng = rand::thread_rng();

    let player_position = player.single().position();

    if asteroid_spawned < asteroids_count.0 {
        let center_position = player_position;

        let range_from_border = rng.gen_range(MIN_SPAWN_RANGE..MAX_SPAWN_RANGE);
        let angle: f32 = rng.gen_range(0.0..(2.0 * PI));
        let position = Quat::from_rotation_z(angle).mul_vec3(Vec3::Y).truncate()
            * (radius_diagonal + range_from_border);

        let size_level = rng.gen_range(1..5);

        let mut builder = AsteroidBuilder::default();
        let created_entity = commands.build_entity(
            builder
                .position(position + center_position)
                .size_level(size_level)
                .base_velocity(Vec2::ZERO),
        );

        commands
            .entity(created_entity)
            .insert(DespawnOn::OutOfRange(1.5 * MAX_SPAWN_RANGE));
    }
}

fn asteroid_dead(
    mut commands: Commands,
    q_deads: Query<
        (&AsteroidSizeLevel, &Transform, &Velocity, Entity),
        (With<Asteroid>, With<Dead>),
    >,
) {
    if q_deads.is_empty() {
        return;
    }

    let mut rng = rand::thread_rng();

    for (size, transform, parent_velocity, entity) in q_deads.iter() {
        commands
            .entity(entity)
            .remove::<PhysicObjectBundle>()
            .insert(Despawn::Normal);

        // Do not create zero sized asteroids
        if size.level() <= 1 {
            continue;
        }

        let shard_count = rng.gen_range(2..=4);

        let mut velocity_angle = 0.0;
        let velocity_angle_step = 2.0 * PI / shard_count as f32;
        let velocity_angle_deviation = 2.0 * PI / shard_count as f32 / 3.0; // 3.0 - is 3 sigma rule for normal distribution

        for _ in 0..shard_count {
            //
            let mutated_velocity_angle =
                velocity_angle.deviate(&mut rng, velocity_angle_deviation as f64);
            let velocity = Quat::from_rotation_z(mutated_velocity_angle)
                .mul_vec3(Vec3::Y)
                .truncate()
                * 50.0;
            let position = transform.translation.truncate();
            let size_level = size.level() - 1;

            let mut builder = AsteroidBuilder::default();
            builder
                .position(position + velocity.normalize() * size.typical_radius())
                .size_level(size_level)
                .base_velocity(parent_velocity.linvel);

            let builder = ComponentInjectorBuilder::new(
                builder,
                DespawnOn::OutOfRange(1.5 * MAX_SPAWN_RANGE),
            );

            commands.build_entity(&builder);

            //
            velocity_angle += velocity_angle_step;
        }
    }
}

fn destroy_asteroids(mut commands: Commands, q_asteroids: Query<Entity, With<Asteroid>>) {
    for entity in q_asteroids.iter() {
        commands.entity(entity).insert(Despawn::Normal);
    }
}
