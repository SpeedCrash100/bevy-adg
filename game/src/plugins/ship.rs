use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::common::{Active, Despawn, Layer, PositionBundle, Reset};
use crate::components::engine::Engine;
use crate::components::health::{Dead, Health, MaxHealth, Regenerate};
use crate::components::movement::{Axis, MainAxis, MovementAxis, RotationAxis, SwayAxis};
use crate::components::particle::fire::FireGenerator;
use crate::components::particle::{ParticleGeneratorDeviation, ParticleGeneratorRate};
use crate::components::player::Player;
use crate::components::ship::control::effects::*;
use crate::components::ship::control::rotation::{RotationControl, ShipTargetViewPoint};
use crate::components::ship::control::ShipEngineController;
use crate::components::ship::Ship;
use crate::math::{Angle, Position, RotateAroundZ};
use crate::stages::LivingStages;
use crate::states::GameState;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                ship_rotate_to_target,
                ship_engine_process,
                engine_reset,
                ship_reset,
                fire_effect_enable,
                fire_effect_progress,
                fire_effect_disable,
            )
                .in_set(OnUpdate(GameState::InGame)),
        )
        .add_systems(
            (
                ship_engine_controller::<MainAxis>,
                ship_engine_controller::<SwayAxis>,
                ship_engine_controller::<RotationAxis>,
                ship_engine_controller_reset,
                ship_engine_controller_update_velocity,
                ship_engine_controller_effect::<ForwardEngineEffect>,
                ship_engine_controller_effect::<BackwardEngineEffect>,
                ship_engine_controller_effect::<SwayLeftEngineEffect>,
                ship_engine_controller_effect::<SwayRightEngineEffect>,
                ship_engine_controller_effect::<RotateLeftEngineEffect>,
                ship_engine_controller_effect::<RotateRightEngineEffect>,
            )
                .in_set(OnUpdate(GameState::InGame)),
        )
        .add_system(ship_dead_handler.in_set(LivingStages::DeadProcessing));
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
    mut q_controller: Query<&mut ShipEngineController>,
) {
    for (transform, target_point, mut rotation_control, children) in q_ships.iter_mut() {
        let ship_pos = transform.position();
        let ship_angle = transform.angle();

        let new_control =
            rotation_control.new_control(target_point.clone().to_vec(), ship_pos, ship_angle);

        let Some(rotation_engine) = children.iter().find(|el| q_controller.get(**el).is_ok()) else {
            continue;
        };

        let mut controller = q_controller.get_mut(*rotation_engine).unwrap();
        controller.set_throttle(Axis::Rotation, new_control);
    }
}

fn ship_engine_process(
    mut q_ships: Query<&GlobalTransform, With<ShipEngineController>>,
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

fn fire_effect_progress(
    q_ships: Query<(&Health, &MaxHealth, &Children), (With<Ship>, Changed<Health>)>,
    mut q_effects: Query<
        (&mut ParticleGeneratorRate, &mut ParticleGeneratorDeviation),
        (With<FireGenerator>, Without<Ship>),
    >,
) {
    for (health, max_health, children) in q_ships.iter() {
        let relative_health = health.health() / max_health.max_health();

        let effect_modifier = 1.0 - (relative_health / 0.5) as f64;

        if relative_health < 0.5 {
            for entity in children {
                let Ok((mut rate, mut deviation)) = q_effects.get_mut(*entity) else {
                    continue;
                };

                rate.set(effect_modifier * 5.0);
                deviation.set(effect_modifier * 5.0);
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

fn ship_engine_controller<A: MovementAxis>(
    q_controllers: Query<(&ShipEngineController, &Children)>,
    mut q_engines: Query<&mut Engine, With<A>>,
) {
    for (controller, children) in q_controllers.iter() {
        let throttle = controller.throttle(A::axis());

        for child in children.iter() {
            if !q_engines.contains(*child) {
                continue;
            }

            let mut engine = q_engines.get_mut(*child).unwrap();
            engine.set_throttle(throttle);
        }
    }
}

fn ship_engine_controller_update_velocity(
    mut q_controllers: Query<(&Parent, &mut Velocity), With<ShipEngineController>>,
    q_velocities: Query<&Velocity, Without<ShipEngineController>>,
) {
    for (parent, mut velocity) in q_controllers.iter_mut() {
        *velocity = q_velocities.get(parent.get()).cloned().unwrap_or_default();
    }
}

fn ship_engine_controller_reset(
    mut commands: Commands,
    mut q_controllers: Query<(Entity, &mut ShipEngineController), With<Reset>>,
) {
    for (entity, mut controller) in q_controllers.iter_mut() {
        controller.reset();

        commands.entity(entity).remove::<Reset>();
    }
}

fn ship_engine_controller_effect<E: EffectTrigger>(
    mut commands: Commands,
    q_controllers: Query<(&ShipEngineController, &Children), Changed<ShipEngineController>>,
    q_effects: Query<Entity, With<E>>,
) {
    for (controller, children) in q_controllers.iter() {
        let effects = children.iter().filter(|c| q_effects.contains(**c));

        let throttle = controller.throttle(E::axis());

        if E::check(throttle) {
            effects.for_each(|e| {
                commands.entity(*e).insert(Active);
            })
        } else {
            effects.for_each(|e| {
                commands.entity(*e).remove::<Active>();
            })
        }
    }
}
