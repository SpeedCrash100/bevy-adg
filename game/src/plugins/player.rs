use bevy::prelude::*;

use crate::components::common::Active;
use crate::components::engine::{Engine, MainEngine, SwayEngine};
use crate::components::player::{Player, PlayerDecorator};
use crate::components::ship::control::rotation::ShipTargetViewPoint;
use crate::components::ship::SimpleShipBuilder;
use crate::components::weapon::Weapon;
use crate::entity::EntityBuildDirector;

#[derive(Component)]
pub struct MainCamera;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_camera)
            .add_startup_system(create_player_ship)
            .add_system(camera_follow_player)
            .add_system_set(
                SystemSet::new()
                    .label("Player control handling")
                    .with_system(handle_mouse_controls)
                    .with_system(throttle_forward)
                    .with_system(throttle_backward)
                    .with_system(sway_left)
                    .with_system(sway_right)
                    .with_system(fire_main),
            );
    }
}

pub fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}

fn create_player_ship(mut commands: Commands) {
    let ship_builder = SimpleShipBuilder::default();
    let player_ship_builder = PlayerDecorator::new(ship_builder);
    commands.build_entity(&player_ship_builder);
}

fn camera_follow_player(
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    q_player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let player_transform = q_player.single();
    let mut camera_transform = q_camera.single_mut();

    camera_transform.translation = player_transform.translation.truncate().extend(999.9);
}

fn handle_mouse_controls(
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_player: Query<&mut ShipTargetViewPoint, With<Player>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let mut player_target = q_player.single_mut();

    let wnd = wnds.get_primary().unwrap();

    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        *player_target = world_pos.into();
    }
}

fn throttle_forward(
    key_state: Res<Input<KeyCode>>,
    q_player: Query<&Children, With<Player>>,
    mut q_ship_engines: Query<&mut Engine, With<MainEngine>>,
) {
    let children = q_player.single();

    let engine_option = children
        .iter()
        .find(|e| q_ship_engines.contains(**e))
        .and_then(|e| Some(q_ship_engines.get_mut(*e).unwrap()));

    let Some(mut engine) = engine_option else {
        warn!("Player doesn't have engine");
        return;
    };

    if key_state.just_pressed(KeyCode::W) || key_state.just_pressed(KeyCode::Up) {
        engine.throttle_up(1.0);
    } else if key_state.just_released(KeyCode::W) || key_state.just_released(KeyCode::Up) {
        engine.throttle_down(1.0);
    }
}

fn throttle_backward(
    key_state: Res<Input<KeyCode>>,
    q_player: Query<&Children, With<Player>>,
    mut q_ship_engines: Query<&mut Engine, With<MainEngine>>,
) {
    let children = q_player.single();

    let engine_option = children
        .iter()
        .find(|e| q_ship_engines.contains(**e))
        .and_then(|e| Some(q_ship_engines.get_mut(*e).unwrap()));

    let Some(mut engine) = engine_option else {
        warn!("Player doesn't have engine");
        return;
    };

    if key_state.just_pressed(KeyCode::S) || key_state.just_pressed(KeyCode::Down) {
        engine.throttle_down(1.0);
    } else if key_state.just_released(KeyCode::S) || key_state.just_released(KeyCode::Down) {
        engine.throttle_up(1.0);
    }
}

fn sway_right(
    key_state: Res<Input<KeyCode>>,
    q_player: Query<&Children, With<Player>>,
    mut q_ship_engines: Query<&mut Engine, With<SwayEngine>>,
) {
    let children = q_player.single();

    let engine_option = children
        .iter()
        .find(|e| q_ship_engines.contains(**e))
        .and_then(|e| Some(q_ship_engines.get_mut(*e).unwrap()));

    let Some(mut engine) = engine_option else {
        warn!("Player doesn't have engine");
        return;
    };

    if key_state.just_pressed(KeyCode::D) || key_state.just_pressed(KeyCode::Right) {
        engine.throttle_up(1.0);
    } else if key_state.just_released(KeyCode::D) || key_state.just_released(KeyCode::Right) {
        engine.throttle_down(1.0);
    }
}

fn sway_left(
    key_state: Res<Input<KeyCode>>,
    q_player: Query<&Children, With<Player>>,
    mut q_ship_engines: Query<&mut Engine, With<SwayEngine>>,
) {
    let children = q_player.single();

    let engine_option = children
        .iter()
        .find(|e| q_ship_engines.contains(**e))
        .and_then(|e| Some(q_ship_engines.get_mut(*e).unwrap()));

    let Some(mut engine) = engine_option else {
        warn!("Player doesn't have engine");
        return;
    };

    if key_state.just_pressed(KeyCode::A) || key_state.just_pressed(KeyCode::Left) {
        engine.throttle_down(1.0);
    } else if key_state.just_released(KeyCode::A) || key_state.just_released(KeyCode::Left) {
        engine.throttle_up(1.0);
    }
}

fn fire_main(
    key_state: Res<Input<MouseButton>>,
    q_player: Query<&Children, With<Player>>,
    q_weapon: Query<Entity, With<Weapon>>,
    mut commands: Commands,
) {
    let children = q_player.single();

    let weapon_option = children
        .iter()
        .find(|e| q_weapon.contains(**e))
        .and_then(|e| Some(q_weapon.get(*e).unwrap()));

    let Some(weapon) = weapon_option else {
        warn!("Player doesn't have main weapon");
        return;
    };

    if key_state.just_pressed(MouseButton::Left) {
        commands.entity(weapon).insert(Active);
    } else if key_state.just_released(MouseButton::Left) {
        commands.entity(weapon).remove::<Active>();
    }
}
