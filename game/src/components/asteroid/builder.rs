use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{DrawMode, FillMode, StrokeMode};
use bevy_rapier2d::prelude::*;
use physic_objects::prelude::*;

use super::generate::generate_asteroid_vectors;
use super::level::AsteroidSizeLevel;
use super::Asteroid;
use crate::entity::EntityBuilder;

#[derive(Builder)]
#[allow(dead_code)]
pub struct AsteroidCreateInfo {
    #[builder(default = "Vec2::ZERO")]
    position: Vec2,
    #[builder(default = "3")]
    size_level: i32,
}

impl EntityBuilder for AsteroidCreateInfoBuilder {
    fn build(&self, commands: &mut bevy::ecs::system::EntityCommands) {
        let create_info = self.build().unwrap();
        let asteroid_structure = generate_asteroid_vectors();

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

        commands.insert(physic_object).insert(Velocity {
            angvel: 1.0,
            linvel: Vec2::ZERO,
        });

        let asteroid_level = AsteroidSizeLevel::new(create_info.size_level);
        let scale = asteroid_level.typical_radius();

        let transform = Transform::from_scale(Vec3::splat(scale))
            .with_translation(create_info.position.extend(0.0));

        commands
            .insert(Asteroid)
            .insert(asteroid_level)
            .insert(TransformBundle::from_transform(transform));
    }
}
