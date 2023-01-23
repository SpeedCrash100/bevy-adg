use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_prototype_lyon::prelude::{DrawMode, FillMode};
use bevy_rapier2d::prelude::{Ccd, ColliderMassProperties, RigidBody};
use physic_objects::prelude::*;

use super::{Projectile, ProjectileEntityBuilder};
use crate::{
    components::{
        common::DespawnOn,
        health::{CollisionDamageBundle, Health},
    },
    entity::EntityBuilder,
};

#[derive(Builder)]
pub struct BulletCreateInfo {
    #[builder(default = "2.0")]
    radius: f32,
    #[builder(default = "10.0")]
    density: f32,
}

pub use BulletCreateInfoBuilder as BulletBuilder;

impl EntityBuilder for BulletBuilder {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        let info = self.build().unwrap();

        let physic_object = CircleBuilder::default()
            .radius(info.radius)
            .params(PhysicObjectParams {
                body: RigidBody::Dynamic,
                mass_properties: ColliderMassProperties::Density(info.density),
            })
            .draw_mode(DrawMode::Fill(FillMode::color(Color::BLACK)))
            .build();

        commands
            .insert(Projectile)
            .insert(physic_object)
            .insert(Ccd::enabled())
            .insert(DespawnOn::OutOfRange(4000.0))
            .insert(Health::new(1.0))
            .insert(CollisionDamageBundle::new())
    }
}

impl ProjectileEntityBuilder for BulletBuilder {}
