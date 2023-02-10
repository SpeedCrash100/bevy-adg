use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::common::{Active, Despawn, Layer, PositionBundle, Reset};
use crate::components::engine::{Engine, RotationEngine};
use crate::components::health::{Dead, Health, MaxHealth, Regenerate};
use crate::components::particle::fire::FireGenerator;
use crate::components::player::Player;
use crate::components::ship::control::rotation::{RotationControl, ShipTargetViewPoint};
use crate::components::ship::Ship;
use crate::math::{Angle, Position, RotateAroundZ};
use crate::stages::LivingStages;
use crate::states::GameState;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(Self::ship_update())
            .add_system_to_stage(LivingStages::DeadProcessing, ship_dead_handler);
    }
}

impl ShipPlugin {
    fn ship_update() -> SystemSet {
        SystemSet::on_update(GameState::InGame)
            .with_system(ship_rotate_to_target)
            .with_system(ship_engine_process)
            .with_system(engine_reset)
            .with_system(ship_reset)
            .with_system(fire_effect_enable)
            .with_system(fire_effect_disable)
    }
}

fn ship_rotate_to_target(
    mut q_ships: Query<
        (
            &Transform,
            &ShipTargetViewPoint,
            &mut RotationControl,
            &Children,
        ),
        With<Ship>,
    >,
    mut q_engines: Query<&mut Engine, With<RotationEngine>>,
) {
    for (transform, target_point, mut rotation_control, children) in q_ships.iter_mut() {
        let ship_pos = transform.position();
        let ship_angle = transform.angle();

        let new_control =
            rotation_control.new_control(target_point.clone().to_vec(), ship_pos, ship_angle);

        let Some(rotation_engine) = children.iter().find(|el| q_engines.get(**el).is_ok()) else {
            continue;
        };

        let mut engine = q_engines.get_mut(*rotation_engine).unwrap();
        engine.set_throttle(new_control);
    }
}

fn ship_engine_process(
    mut q_ships: Query<&Transform, With<Ship>>,
    mut q_ship_engines: Query<(&Engine, &mut ExternalForce, &Parent), Without<Ship>>,
) {
    for (engine, mut force, parent) in q_ship_engines.iter_mut() {
        let Ok(parent_transform) = q_ships.get_mut(parent.get()) else {
            continue;
        };

        *force = engine.force().rotate_z(parent_transform.angle());
    }
}

/// This handler must ignore Player
fn ship_dead_handler(
    mut commands: Commands,
    q_ships: Query<Entity, (With<Dead>, With<Ship>, Without<Player>)>,
) {
    for entity in q_ships.iter() {
        commands.entity(entity).insert(Despawn::Recursive);
    }
}

fn engine_reset(mut commands: Commands, mut q_engines: Query<(Entity, &mut Engine), With<Reset>>) {
    for (entity, mut engine) in q_engines.iter_mut() {
        engine.set_throttle(0.0);
        commands.entity(entity).remove::<Reset>();
    }
}

fn ship_reset(mut commands: Commands, q_ships: Query<Entity, (With<Ship>, With<Reset>)>) {
    for entity in q_ships.iter() {
        commands
            .entity(entity)
            .insert(Regenerate::OneTimeToFull)
            .insert(PositionBundle::new(Vec2::ZERO, Layer::Main))
            .insert(Velocity::zero())
            .remove::<Reset>();
    }
}

fn fire_effect_enable(
    mut commands: Commands,
    q_ships: Query<(&Health, &MaxHealth, &Children), (With<Ship>, Changed<Health>)>,
    q_effects: Query<Entity, With<FireGenerator>>,
) {
    for (health, max_health, children) in q_ships.iter() {
        let relative_health = health.health() / max_health.max_health();
        if relative_health < 0.5 {
            let fire_effects = children.iter().filter(|el| q_effects.contains(**el));
            for e in fire_effects {
                commands.entity(*e).insert(Active);
            }
        }
    }
}

fn fire_effect_disable(
    mut commands: Commands,
    q_ships: Query<(&Health, &MaxHealth, &Children), (With<Ship>, Changed<Health>)>,
    q_effects: Query<Entity, With<FireGenerator>>,
) {
    for (health, max_health, children) in q_ships.iter() {
        let relative_health = health.health() / max_health.max_health();
        if 0.5 < relative_health {
            let fire_effects = children.iter().filter(|el| q_effects.contains(**el));
            for e in fire_effects {
                commands.entity(*e).remove::<Active>();
            }
        }
    }
}
