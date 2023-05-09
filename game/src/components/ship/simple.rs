use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::{ColliderMassProperties, RigidBody};
use physic_objects::prelude::*;

use crate::components::common::{Layer, PositionBundle};
use crate::components::engine::{MainEngineBuilder, RotationEngineBuilder, SwayEngineBuilder};
use crate::components::health::{CollisionDamageBundle, HealthBundle};
use crate::components::particle::fire::FireGeneratorBundle;
use crate::components::particle::propulsion::PropulsionParticleGeneratorBundle;
use crate::components::weapon::machinegun::MachineGunBuilder;
use crate::entity::{BuilderConcatenator, EntityBuildDirector, EntityBuilder};
use crate::math::RotateAroundZ;

use super::control::effects::*;
use super::control::rotation::RotationControlBuilder;
use super::control::ShipEngineControllerBundle;
use super::Ship;

/// Radius vector used to create points for ship
const SHIP_RADIUS: f32 = 30.0;
/// Radius vector will be rotated by this angles to create points
const SHIP_ANGLES: [f32; 3] = [0.0, 135.0, -135.0];

pub struct ShipBuilder;

impl ShipBuilder {
    pub fn new(position: Vec2) -> impl EntityBuilder {
        let mut builder = ShipBaseBuilder::default();
        builder.position(position);

        let builder = BuilderConcatenator::new(builder, ShipEnginesBuilder);
        let builder = BuilderConcatenator::new(builder, ShipWeaponsBuilder);

        builder
    }
}

#[derive(Builder)]
#[builder(name = "ShipBaseBuilder")]
pub struct ShipBaseCreateInfo {
    #[builder(default = "Vec2::ZERO")]
    position: Vec2,
}

impl EntityBuilder for ShipBaseBuilder {
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
            .points(points)
            .build();

        let commands = commands
            .insert(Ship)
            .insert(physic_object)
            .insert(Fill {
                color: Color::BLUE,
                ..default()
            })
            .insert(HealthBundle::new(10000.0))
            .insert(CollisionDamageBundle::new())
            .insert(PositionBundle::new(create_info.position, Layer::Main))
            .with_children(|cb| {
                // Fire effects when damaged
                cb.spawn(FireGeneratorBundle::new(
                    2.5,
                    5.0,
                    Transform::from_translation(Vec3::ZERO),
                ));
            });

        EntityBuilder::build(&RotationControlBuilder::default(), commands)
    }
}

pub struct ShipEnginesBuilder;

impl EntityBuilder for ShipEnginesBuilder {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        let commands = commands.with_children(|cb| {
            cb.spawn(ShipEngineControllerBundle::new())
                .with_children(|cb| {
                    cb.build_entity(RotationEngineBuilder::default().torque(50_000_000.0_f32));
                })
                .with_children(|cb| {
                    cb.build_entity(MainEngineBuilder::default().force(1_000_000.0));
                })
                .with_children(|cb| {
                    cb.build_entity(SwayEngineBuilder::default().force(1_000_000.0));
                })
                // 1 Main engine effect
                .with_children(|cb| {
                    cb.spawn(PropulsionParticleGeneratorBundle::new(
                        1.0,
                        10.0,
                        1.0,
                        Transform::from_translation(Vec3::NEG_X * 22.0),
                    ))
                    .insert(ForwardEngineEffect);
                })
                // 2 Backward engine effects
                .with_children(|cb| {
                    cb.spawn(PropulsionParticleGeneratorBundle::new(
                        0.15,
                        10.0,
                        0.25,
                        Transform::from_translation(Vec3::Y * 12.5)
                            .with_rotation(Quat::from_rotation_z(180.0_f32.to_radians())),
                    ))
                    .insert(BackwardEngineEffect);
                })
                .with_children(|cb| {
                    cb.spawn(PropulsionParticleGeneratorBundle::new(
                        0.15,
                        10.0,
                        0.25,
                        Transform::from_translation(Vec3::NEG_Y * 12.5)
                            .with_rotation(Quat::from_rotation_z(180.0_f32.to_radians())),
                    ))
                    .insert(BackwardEngineEffect);
                })
                // Sway left
                .with_children(|cb| {
                    cb.spawn(PropulsionParticleGeneratorBundle::new(
                        0.25,
                        10.0,
                        0.25,
                        Transform::from_translation(Vec3::Y * 12.5)
                            .with_rotation(Quat::from_rotation_z(90.0_f32.to_degrees())),
                    ))
                    .insert(SwayLeftEngineEffect);
                })
                // Sway right
                .with_children(|cb| {
                    cb.spawn(PropulsionParticleGeneratorBundle::new(
                        0.25,
                        10.0,
                        0.25,
                        Transform::from_translation(Vec3::NEG_Y * 12.5)
                            .with_rotation(Quat::from_rotation_z(-90.0_f32.to_degrees())),
                    ))
                    .insert(SwayRightEngineEffect);
                })
                // Rotate left
                .with_children(|cb| {
                    cb.spawn(PropulsionParticleGeneratorBundle::new(
                        0.05,
                        10.0,
                        0.25,
                        Transform::from_translation(Vec3::X * 28.0)
                            .with_rotation(Quat::from_rotation_z(90.0_f32.to_degrees())),
                    ))
                    .insert(RotateLeftEngineEffect);
                })
                // Rotate right
                .with_children(|cb| {
                    cb.spawn(PropulsionParticleGeneratorBundle::new(
                        0.05,
                        10.0,
                        0.25,
                        Transform::from_translation(Vec3::X * 28.0)
                            .with_rotation(Quat::from_rotation_z(-90.0_f32.to_degrees())),
                    ))
                    .insert(RotateRightEngineEffect);
                });
        });

        EntityBuilder::build(&RotationControlBuilder::default(), commands)
    }
}

pub struct ShipWeaponsBuilder;

impl EntityBuilder for ShipWeaponsBuilder {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        commands.with_children(|cb| {
            cb.build_entity(
                MachineGunBuilder::default()
                    .firerate(4.0)
                    .position(Vec2::X * 33.0),
            );
        })
    }
}
