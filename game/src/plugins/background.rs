use bevy::prelude::*;
use bevy_parallax::{
    LayerData, LayerSpeed, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxPlugin,
    ParallaxResource,
};

use crate::{
    components::{camera::MainCamera, common::Layer, player::Player},
    math::Position,
};

#[derive(Resource, Default)]
pub struct PlayerPreviousPosition(Vec2);

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ParallaxPlugin)
            .init_resource::<PlayerPreviousPosition>()
            .insert_resource(ParallaxResource::new(vec![
                LayerData {
                    speed: LayerSpeed::Bidirectional(0.9, 0.9),
                    path: "images/back.png".to_string(),
                    tile_size: Vec2::new(256.0, 256.0),
                    cols: 1,
                    rows: 1,
                    scale: 1.0,
                    z: Layer::BackgroundLow.into(),
                    ..default()
                },
                LayerData {
                    speed: LayerSpeed::Bidirectional(0.8, 0.8),
                    path: "images/middle.png".to_string(),
                    tile_size: Vec2::new(256.0, 256.0),
                    cols: 1,
                    rows: 1,
                    scale: 1.0,
                    z: Layer::BackgroundMiddle.into(),
                    ..default()
                },
                LayerData {
                    speed: LayerSpeed::Bidirectional(0.5, 0.5),
                    path: "images/front.png".to_string(),
                    tile_size: Vec2::new(256.0, 256.0),
                    cols: 1,
                    rows: 1,
                    scale: 1.0,
                    z: Layer::BackgroundHigh.into(),
                    ..default()
                },
            ]))
            .add_startup_system(create_parallax_camera)
            .add_system(camera_follow_player);
    }
}

fn create_parallax_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(MainCamera)
        .insert(ParallaxCameraComponent);
}

fn camera_follow_player(
    q_player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut move_events_writer: EventWriter<ParallaxMoveEvent>,
    mut previous_position: ResMut<PlayerPreviousPosition>,
) {
    let player_position = q_player.single().position();
    let velocity = player_position - previous_position.0;
    previous_position.0 = player_position;

    move_events_writer.send(ParallaxMoveEvent {
        camera_move_speed: velocity,
    });
}
