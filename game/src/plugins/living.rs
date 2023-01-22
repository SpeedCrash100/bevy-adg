use bevy::prelude::*;

use crate::{
    components::{
        common::Despawn,
        health::{Dead, Health},
    },
    stages::LivingStages,
};

pub struct LivingPlugin;

impl Plugin for LivingPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::Update,
            LivingStages::HealthProcessing,
            SystemStage::parallel(),
        );

        app.add_stage_after(
            LivingStages::HealthProcessing,
            LivingStages::DeadProcessing,
            SystemStage::parallel(),
        );

        app.add_stage_after(
            LivingStages::DeadProcessing,
            LivingStages::DespawnProcessing,
            SystemStage::parallel(),
        );

        app.add_system_to_stage(LivingStages::HealthProcessing, dead_mark_inserter);
        app.add_system_to_stage(LivingStages::DespawnProcessing, despawn_entities);
    }
}

fn dead_mark_inserter(
    mut commands: Commands,
    q_entities: Query<(&Health, Entity), (Changed<Health>, Without<Dead>, Without<Despawn>)>,
) {
    for (health, entity) in q_entities.iter() {
        if !health.alive() {
            commands.entity(entity).insert(Dead);
        }
    }
}

fn despawn_entities(mut commands: Commands, q_entities: Query<(&Despawn, Entity)>) {
    for (mark, entity) in q_entities.iter() {
        match mark {
            Despawn::Normal => commands.entity(entity).despawn(),
            Despawn::Recursive => commands.entity(entity).despawn_recursive(),
        }
    }
}
