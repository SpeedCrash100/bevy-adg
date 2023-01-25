use bevy::prelude::*;

use crate::{
    components::{
        common::Despawn,
        health::{Dead, Health, MaxHealth, Regenerate},
    },
    stages::LivingStages,
};

#[derive(SystemLabel)]
pub struct DeadMarkInserterSystemSet;

#[derive(SystemLabel)]
pub struct RegenerateSystemSet;

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

        app.add_system_set_to_stage(
            LivingStages::HealthProcessing,
            SystemSet::new()
                .label(RegenerateSystemSet)
                .with_system(regenerate_one_time),
        );

        app.add_system_set_to_stage(
            LivingStages::HealthProcessing,
            SystemSet::new()
                .label(DeadMarkInserterSystemSet)
                .after(RegenerateSystemSet)
                .with_system(dead_mark_inserter),
        );

        app.add_system_to_stage(LivingStages::DespawnProcessing, despawn_entities);
    }
}

fn regenerate_one_time(
    mut commands: Commands,
    mut q_entity: Query<(&mut Health, &MaxHealth, &Regenerate, Entity)>,
) {
    for (mut hp, max_hp, regen_enum, entity) in q_entity.iter_mut() {
        let Regenerate::OneTimeToFull = regen_enum;

        *hp = Health::new(max_hp.max_health());
        commands.entity(entity).remove::<Regenerate>();
        commands.entity(entity).remove::<Dead>();
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
