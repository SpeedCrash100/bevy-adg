use bevy::prelude::*;

use crate::components::player::PlayerDecorator;
use crate::components::ship::SimpleShipBuilder;
use crate::entity::EntityBuildDirector;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_player_ship);
    }
}

fn create_player_ship(mut commands: Commands) {
    let ship_builder = SimpleShipBuilder::default();
    let player_ship_builder = PlayerDecorator::new(ship_builder);
    commands.build_entity(&player_ship_builder);
}
