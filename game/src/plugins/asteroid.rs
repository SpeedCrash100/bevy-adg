use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use rand::Rng;

use crate::{
    components::asteroid::{Asteroid, AsteroidBuilder},
    entity::EntityBuildDirector,
    random::Deviate,
};

/// Minimal range from screen border to spawn asteroid
const MIN_SPAWN_RANGE: f32 = 200.0;
/// Maximum range from screen border to spawn asteroid
const MAX_SPAWN_RANGE: f32 = 1200.0;
/// Deviation
const VELOCITY_DEVIATION: f64 = 5.0;
/// Deviation
const ANGULAR_VELOCITY_DEVIATION: f64 = 1.0;

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
            .add_system(asteroids_spawn_system);
    }
}

/// Spawn asteroids
fn asteroids_spawn_system(
    mut commands: Commands,
    window: Res<Windows>,
    asteroids_count: Res<AsteroidCount>,
    asteroids: Query<(), With<Asteroid>>,
) {
    let window = window.get_primary().unwrap();

    let radius_w = window.width() / 2.0;
    let radius_h = window.width() / 2.0;
    let radius_diagonal = (radius_w.powi(2) + radius_h.powi(2)).sqrt();

    let asteroid_spawned = asteroids.iter().count();
    let mut rng = rand::thread_rng();

    if asteroid_spawned < asteroids_count.0 {
        let center_position = Vec2::ZERO;

        let range_from_border = rng.gen_range(MIN_SPAWN_RANGE..MAX_SPAWN_RANGE);
        let angle: f32 = rng.gen_range(0.0..(2.0 * PI));
        let position = Quat::from_rotation_z(angle).mul_vec3(Vec3::Y).truncate()
            * (radius_diagonal + range_from_border);

        let velocity = Vec2::ZERO.deviate(&mut rng, VELOCITY_DEVIATION);

        let size_level = rng.gen_range(1..5);

        let mut builder = AsteroidBuilder::default();
        let created_entity = commands.build_entity(
            builder
                .position(position + center_position)
                .size_level(size_level),
        );

        commands.entity(created_entity).insert(Velocity {
            angvel: (0.0 as f32).deviate(&mut rng, ANGULAR_VELOCITY_DEVIATION),
            linvel: velocity,
        });
    }
}
