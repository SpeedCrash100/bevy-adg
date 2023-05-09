use bevy::prelude::*;

use crate::{
    components::common::{Despawn, DespawnOnExitGame, Reset, Resettable},
    states::GameState,
};

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (despawn_entities_on_exit, reset_entities_on_exit)
                .in_schedule(OnExit(GameState::InGame)),
        );
    }
}

fn despawn_entities_on_exit(
    mut commands: Commands,
    q_entities: Query<Entity, With<DespawnOnExitGame>>,
) {
    for entity in q_entities.iter() {
        commands.entity(entity).insert(Despawn::Recursive);
    }
}

fn reset_entities_on_exit(mut commands: Commands, q_entities: Query<Entity, With<Resettable>>) {
    for entity in q_entities.iter() {
        commands.entity(entity).insert(Reset);
    }
}
