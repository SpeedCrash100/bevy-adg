use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::{ColliderMassProperties, RigidBody};
use physic_objects::prelude::*;

use crate::{entity::EntityBuilder, math::RotateAroundZ};

use super::Ship;

const SHIP_HEIGHT: f32 = 40.0;

#[derive(Builder)]
pub struct ShipCreateInfo {
    #[builder(default = "Vec2::ZERO")]
    position: Vec2,
}

impl EntityBuilder for ShipCreateInfoBuilder {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        let create_info = self.build().unwrap();

        // This is a triangle ship. We will create a radius vector and will rotate it by 120 degrets to get points of the triangle
        let radius = SHIP_HEIGHT / (1.0 + 60.0_f32.to_radians().cos());
        let mut points = [Vec2::ZERO; 3];
        for i in 0..3 {
            let angle = 120.0 * i as f32;
            let vector = Vec2::X * radius;
            points[i] = vector.rotate_z(angle.to_radians());
        }

        let physic_object = TriangleBuilder::default()
            .params(PhysicObjectParams {
                body: RigidBody::Dynamic,
                mass_properties: ColliderMassProperties::Density(10.0),
            })
            .draw_mode(DrawMode::Fill(FillMode::color(Color::BLUE)))
            .points(points)
            .build();

        commands
            .insert(Ship)
            .insert(physic_object)
            .insert(TransformBundle::from(Transform::from_translation(
                create_info.position.extend(0.0),
            )))
    }
}
