use bevy::prelude::*;

use crate::{
    components::common::{Despawn, DespawnOnExitGame, Reset, Resettable},
    states::GameState,
};

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        for set in Self::get_update_system_sets() {
            app.add_system_set(set);
        }
    }
}

impl InGamePlugin {
    fn get_update_system_sets() -> Vec<SystemSet> {
        vec![Self::on_exit()]
    }

    fn on_exit() -> SystemSet {
        SystemSet::on_exit(GameState::InGame)
            .with_system(despawn_entities_on_exit)
            .with_system(reset_entities_on_exit)
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
