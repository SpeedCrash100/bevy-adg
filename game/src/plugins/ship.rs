use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::engine::{Engine, RotationEngine};
use crate::components::ship::control::rotation::{RotationControl, ShipTargetViewPoint};
use crate::components::ship::Ship;
use crate::math::{Angle, Position, RotateAroundZ};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ship_rotate_to_target)
            .add_system(ship_engine_process);
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
