use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{DrawMode, FillMode, StrokeMode};
use bevy_rapier2d::prelude::*;
use physic_objects::prelude::*;

use super::generate::generate_asteroid_vectors;
use super::level::AsteroidSizeLevel;
use super::Asteroid;
use crate::components::common::{DespawnOnExitGame, DespawnOnOutOfRange, Layer, PositionBundle};
use crate::components::health::{CollisionDamageBundle, Health};
use crate::entity::EntityBuilder;
use crate::random::Deviate;

const ASTEROID_ANGULAR_SPEED_DEVIATION: f64 = 0.3;
const ASTEROID_LINEAR_SPEED_DEVIATION: f64 = 5.0;

#[derive(Builder)]
pub struct AsteroidCreateInfo {
    #[builder(default = "Vec2::ZERO")]
    position: Vec2,
    #[builder(default = "3")]
    size_level: i32,

    #[builder(default = "Vec2::ZERO")]
    base_velocity: Vec2,
}

impl EntityBuilder for AsteroidCreateInfoBuilder {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        let create_info = self.build().unwrap();
        let asteroid_structure = generate_asteroid_vectors();
        let asteroid_level = AsteroidSizeLevel::new(create_info.size_level);
        let scale = asteroid_level.typical_radius();

        let asteroid_structure: Vec<_> =
            asteroid_structure.into_iter().map(|v| v * scale).collect();

        let physic_object = TriangleFanBuilder::default()
            .params(PhysicObjectParams {
                body: RigidBody::Dynamic,
                mass_properties: ColliderMassProperties::Density(20.0),
            })
            .draw_mode(DrawMode::Outlined {
                fill_mode: FillMode::color(Color::ORANGE_RED),
                outline_mode: StrokeMode::new(Color::BLACK, 0.0),
            })
            .points(asteroid_structure)
            .build();

        let mut rng = rand::thread_rng();
        commands.insert(physic_object).insert(Velocity {
            angvel: 0.0_f32.deviate(&mut rng, ASTEROID_ANGULAR_SPEED_DEVIATION),
            linvel: create_info.base_velocity
                + Vec2::ZERO.deviate(&mut rng, ASTEROID_LINEAR_SPEED_DEVIATION),
        });

        let transform = PositionBundle::new(create_info.position, Layer::Main);

        commands
            .insert(Asteroid)
            .insert(Health::new(asteroid_level.max_health()))
            .insert(asteroid_level)
            .insert(transform)
            .insert(CollisionDamageBundle::new())
            .insert(DespawnOnOutOfRange)
            .insert(DespawnOnExitGame)
    }
}
