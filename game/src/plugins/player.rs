use bevy::prelude::*;

use crate::components::camera::MainCamera;
use crate::components::common::{Active, Resettable};

use crate::components::movement::Axis;
use crate::components::player::{Player, PlayerDecorator};
use crate::components::ship::control::rotation::ShipTargetViewPoint;
use crate::components::ship::control::ShipEngineController;
use crate::components::ship::SimpleShipBuilder;
use crate::components::weapon::Weapon;
use crate::entity::{ComponentInjectorBuilder, EntityBuildDirector};

use crate::states::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_player_ship)
            .add_system_set(Self::player_controls());
    }
}

impl PlayerPlugin {
    fn player_controls() -> SystemSet {
        SystemSet::on_update(GameState::InGame)
            .with_system(handle_mouse_controls)
            .with_system(throttle_forward)
            .with_system(throttle_backward)
            .with_system(sway_left)
            .with_system(sway_right)
            .with_system(fire_main)
    }
}

fn create_player_ship(mut commands: Commands) {
    let ship_builder = SimpleShipBuilder::new(Vec2::ZERO);
    let ship_builder = PlayerDecorator::new(ship_builder);
    let ship_builder = ComponentInjectorBuilder::new(ship_builder, Resettable);

    commands.build_entity(&ship_builder);
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
    mut q_controller: Query<&mut ShipEngineController>,
) {
    let children = q_player.single();

    let controller_option = children
        .iter()
        .find(|e| q_controller.contains(**e))
        .and_then(|e| Some(q_controller.get_mut(*e).unwrap()));

    let Some(mut controller) = controller_option else {
        warn!("Player doesn't have engines controller");
        return;
    };

    if key_state.just_pressed(KeyCode::W) || key_state.just_pressed(KeyCode::Up) {
        controller.throttle_up(Axis::Main, 1.0);
    } else if key_state.just_released(KeyCode::W) || key_state.just_released(KeyCode::Up) {
        controller.throttle_down(Axis::Main, 1.0);
    }
}

fn throttle_backward(
    key_state: Res<Input<KeyCode>>,
    q_player: Query<&Children, With<Player>>,
    mut q_controller: Query<&mut ShipEngineController>,
) {
    let children = q_player.single();

    let controller_option = children
        .iter()
        .find(|e| q_controller.contains(**e))
        .and_then(|e| Some(q_controller.get_mut(*e).unwrap()));

    let Some(mut controller) = controller_option else {
        warn!("Player doesn't have engines controller");
        return;
    };

    if key_state.just_pressed(KeyCode::S) || key_state.just_pressed(KeyCode::Down) {
        controller.throttle_down(Axis::Main, 1.0);
    } else if key_state.just_released(KeyCode::S) || key_state.just_released(KeyCode::Down) {
        controller.throttle_up(Axis::Main, 1.0);
    }
}

fn sway_right(
    key_state: Res<Input<KeyCode>>,
    q_player: Query<&Children, With<Player>>,
    mut q_controller: Query<&mut ShipEngineController>,
) {
    let children = q_player.single();

    let controller_option = children
        .iter()
        .find(|e| q_controller.contains(**e))
        .and_then(|e| Some(q_controller.get_mut(*e).unwrap()));

    let Some(mut controller) = controller_option else {
        warn!("Player doesn't have engines controller");
        return;
    };

    if key_state.just_pressed(KeyCode::D) || key_state.just_pressed(KeyCode::Right) {
        controller.throttle_up(Axis::Sway, 1.0);
    } else if key_state.just_released(KeyCode::D) || key_state.just_released(KeyCode::Right) {
        controller.throttle_down(Axis::Sway, 1.0);
    }
}

fn sway_left(
    key_state: Res<Input<KeyCode>>,
    q_player: Query<&Children, With<Player>>,
    mut q_controller: Query<&mut ShipEngineController>,
) {
    let children = q_player.single();

    let controller_option = children
        .iter()
        .find(|e| q_controller.contains(**e))
        .and_then(|e| Some(q_controller.get_mut(*e).unwrap()));

    let Some(mut controller) = controller_option else {
        warn!("Player doesn't have engines controller");
        return;
    };
    if key_state.just_pressed(KeyCode::A) || key_state.just_pressed(KeyCode::Left) {
        controller.throttle_down(Axis::Sway, 1.0);
    } else if key_state.just_released(KeyCode::A) || key_state.just_released(KeyCode::Left) {
        controller.throttle_up(Axis::Sway, 1.0);
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
