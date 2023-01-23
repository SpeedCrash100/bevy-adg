use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::{ColliderMassProperties, RigidBody};
use physic_objects::prelude::*;

use crate::components::engine::{MainEngineBuilder, RotationEngineBuilder, SwayEngineBuilder};
use crate::components::weapon::machinegun::MachineGunBuilder;
use crate::entity::{EntityBuildDirector, EntityBuilder};
use crate::math::RotateAroundZ;

use super::control::rotation::RotationControlBuilder;
use super::Ship;

/// Radius vector used to create points for ship
const SHIP_RADIUS: f32 = 30.0;
/// Radius vector will be rotated by this angles to create points
const SHIP_ANGLES: [f32; 3] = [0.0, 135.0, -135.0];

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

        let mut points = [Vec2::ZERO; 3];
        for i in 0..SHIP_ANGLES.len() {
            let angle = SHIP_ANGLES[i];
            let vector = Vec2::X * SHIP_RADIUS;
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

        let commands = commands
            .insert(Ship)
            .insert(physic_object)
            .insert(TransformBundle::from(Transform::from_translation(
                create_info.position.extend(0.0),
            )))
            .with_children(|cb| {
                cb.build_entity(RotationEngineBuilder::default().torque(50_000_000.0_f32));
            })
            .with_children(|cb| {
                cb.build_entity(MainEngineBuilder::default().force(1_000_000.0));
            })
            .with_children(|cb| {
                cb.build_entity(SwayEngineBuilder::default().force(1_000_000.0));
            })
            .with_children(|cb| {
                cb.build_entity(
                    MachineGunBuilder::default()
                        .firerate(4.0)
                        .position(Vec2::X * 33.0),
                );
            });

        EntityBuilder::build(&RotationControlBuilder::default(), commands)
    }
}
