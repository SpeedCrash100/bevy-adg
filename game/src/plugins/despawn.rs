use bevy::prelude::*;

use crate::{
    components::{
        common::{Despawn, DespawnOn, TimeToLive},
        player::Player,
    },
    math::Position,
};

const DESPAWN_RANGE: f32 = 3000.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(despawn_on_out_of_range)
            .add_system(update_time_to_live)
            .add_system(despawn_on_time_to_live);
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
        if !mark.contains(DespawnOn::OUT_OF_RANGE) {
            continue;
        }

        let position = transform.position();
        let range = (position - player_position).length();

        if DESPAWN_RANGE <= range {
            commands.entity(entity).insert(Despawn::Recursive);
        }
    }
}

fn update_time_to_live(mut q_entities: Query<&mut TimeToLive>, time: Res<Time>) {
    for mut tol in q_entities.iter_mut() {
        tol.decrease(time.delta_seconds())
    }
}

fn despawn_on_time_to_live(
    mut commands: Commands,
    q_entities: Query<(&TimeToLive, &DespawnOn, Entity)>,
) {
    for (tol, despawn_on, entity) in q_entities.iter() {
        if tol.finished() && despawn_on.contains(DespawnOn::TIME_OF_LIVE) {
            commands.entity(entity).insert(Despawn::Recursive);
        }
    }
}
