use bevy::prelude::*;

use crate::{
    components::{
        common::{Despawn, DespawnOn},
        player::Player,
    },
    math::Position,
};

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(despawn_on_out_of_range);
    }
}

fn despawn_on_out_of_range(
    mut commands: Commands,
    q_entities: Query<(&Transform, &DespawnOn, Entity), Changed<Transform>>,
    q_player: Query<&Transform, With<Player>>,
) {
    if q_entities.is_empty() {
        return;
    }

    let Ok(player_transform) = q_player.get_single() else { return; };
    let player_position = player_transform.position();

    for (transform, mark, entity) in q_entities.iter() {
        let DespawnOn::OutOfRange(max_range) = mark;

        let position = transform.position();
        let range = (position - player_position).length();

        if *max_range <= range {
            commands.entity(entity).insert(Despawn::Recursive);
        }
    }
}
