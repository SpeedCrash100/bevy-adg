use bevy::prelude::*;

use crate::{
    components::common::{Despawn, DespawnOn},
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
        SystemSet::on_exit(GameState::InGame).with_system(despawn_entities_on_exit)
    }
}

fn despawn_entities_on_exit(mut commands: Commands, q_entities: Query<(Entity, &DespawnOn)>) {
    for (entity, mark) in q_entities.iter() {
        if !mark.contains(DespawnOn::ON_EXIT_GAME) {
            continue;
        }

        commands.entity(entity).insert(Despawn::Recursive);
    }
}
