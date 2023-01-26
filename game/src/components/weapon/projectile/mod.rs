use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier2d::prelude::Velocity;

use crate::{
    components::common::{Layer, PositionBundle},
    entity::EntityBuilder,
};

/// Simple bullet implementation. It deals damage by collision
pub mod bullet;

/// Mark for all projectiles
#[derive(Component)]
pub struct Projectile;

/// All projectile creators must have this trait implemented
///
/// # Warning
/// Bullet projectile builder must create a static projectile without translation, rotation, velocity.
/// These components will be added by weapon itself on fire.
///
/// # Warning
/// Add [Projectile] component so weapon system can handle projectiles
pub trait ProjectileEntityBuilder: EntityBuilder + Send + Sync {}

#[derive(Builder)]
#[builder(name = "ProjectileDecorator")]
pub struct ProjectileCreateInfo {
    position: Vec2,
    velocity: Vec2,
}

impl EntityBuilder for ProjectileDecorator {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        let info = self.build().unwrap();
        commands
            .insert(Velocity::linear(info.velocity))
            .insert(PositionBundle::new(info.position, Layer::Main))
    }
}

#[derive(Component)]
pub struct ProjectileCreator {
    builder: Box<dyn ProjectileEntityBuilder>,
    decorator: ProjectileDecorator,
}

impl ProjectileCreator {
    pub fn new(projectile_builder: impl ProjectileEntityBuilder + 'static) -> Self {
        Self {
            builder: Box::new(projectile_builder),
            decorator: ProjectileDecorator::default(),
        }
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.decorator.position(position);
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.decorator.velocity(velocity);
    }
}

impl EntityBuilder for ProjectileCreator {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        // build base projectile
        let projectile = EntityBuilder::build(self.builder.as_ref(), commands);
        // Apply transformation and velocity
        EntityBuilder::build(&self.decorator, projectile)
    }
}
