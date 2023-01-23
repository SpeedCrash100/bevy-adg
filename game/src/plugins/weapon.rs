use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::{
    components::{
        common::{Active, Despawn},
        health::Dead,
        weapon::{
            projectile::{Projectile, ProjectileCreator},
            Weapon,
        },
    },
    entity::EntityBuildDirector,
    math::{Angle, Position, RotateAroundZ},
    random::Deviate,
    stages::LivingStages,
};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(weapon_update)
            .add_system(fire_weapon)
            .add_system_to_stage(LivingStages::DeadProcessing, despawn_dead_projectiles);
    }
}

fn weapon_update(mut q_weapons: Query<&mut Weapon>, time: Res<Time>) {
    for mut weapon in q_weapons.iter_mut() {
        weapon.update(time.delta_seconds());
    }
}

fn fire_weapon(
    mut commands: Commands,
    q_parents: Query<&Velocity>,
    mut q_weapons: Query<
        (
            &mut Weapon,
            &mut ProjectileCreator,
            &GlobalTransform,
            &Parent,
        ),
        With<Active>,
    >,
) {
    for (mut weapon, mut creator, transform, parent) in q_weapons.iter_mut() {
        if weapon.fire() {
            let mut rng = rand::thread_rng();

            let base_angle = transform.angle();
            let angle = base_angle.deviate(&mut rng, weapon.accuracy() as f64);

            let velocity = Vec2::X.rotate_z(angle) * weapon.velocity();
            let position = transform.position();

            let parent_velocity_option = q_parents.get(parent.get()).ok();
            let parent_velocity = match parent_velocity_option {
                Some(vel) => vel.linvel,
                _ => Vec2::ZERO,
            };

            creator.set_position(position);
            creator.set_velocity(velocity + parent_velocity);

            commands.build_entity(&*creator);
        }
    }
}

fn despawn_dead_projectiles(
    mut commands: Commands,
    q_projectiles: Query<Entity, (With<Dead>, With<Projectile>)>,
) {
    for entity in q_projectiles.iter() {
        commands.entity(entity).insert(Despawn::Normal);
    }
}
