use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::entity::EntityBuilder;

use super::projectile::bullet::BulletBuilder;
use super::projectile::ProjectileCreator;
use super::Weapon;

#[derive(Component)]
pub struct MachineGun;

#[derive(Builder)]
#[builder(name = "MachineGunBuilder")]
pub struct MachineGunCreateInfo {
    #[builder(default = "1.0")]
    firerate: f32,
    #[builder(default = "Vec2::ZERO")]
    position: Vec2,
}

impl EntityBuilder for MachineGunBuilder {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        let info = self.build().unwrap();

        let mut bullet_gen = BulletBuilder::default();
        bullet_gen.radius(2.0).density(50.0);

        commands
            .insert(MachineGun)
            .insert(Weapon::new(info.firerate, 500.0, 0.05))
            .insert(ProjectileCreator::new(bullet_gen))
            .insert(TransformBundle::from(Transform::from_translation(
                info.position.extend(0.0),
            )))
    }
}
