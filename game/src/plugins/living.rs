use bevy::prelude::*;

use crate::{
    components::{
        common::Despawn,
        health::{
            Dead, Health, Immortality, MaxHealth, RegenerateOneTimeToFull, TimedImmortality,
            TimedImmortalityBundle,
        },
    },
    stages::LivingStages,
};

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone)]
pub struct DeadMarkInserterSystemSet;

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone)]
pub struct RegenerateSystemSet;

pub struct LivingPlugin;

impl Plugin for LivingPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            (
                LivingStages::HealthProcessing,
                LivingStages::DeadProcessing,
                LivingStages::DespawnProcessing,
            )
                .chain(),
        );

        app.configure_sets((RegenerateSystemSet, DeadMarkInserterSystemSet).chain());

        app.add_system(
            regenerate_one_time
                .in_set(LivingStages::HealthProcessing)
                .in_set(RegenerateSystemSet),
        );

        app.add_systems(
            (dead_mark_inserter, timed_immortality_update)
                .in_set(LivingStages::HealthProcessing)
                .in_set(DeadMarkInserterSystemSet),
        );

        app.add_system(despawn_entities.in_set(LivingStages::DespawnProcessing));
    }
}

fn regenerate_one_time(
    mut commands: Commands,
    mut q_entity: Query<(&mut Health, &MaxHealth, Entity), With<RegenerateOneTimeToFull>>,
) {
    for (mut hp, max_hp, entity) in q_entity.iter_mut() {
        *hp = Health::new(max_hp.max_health());
        commands.entity(entity).remove::<RegenerateOneTimeToFull>();
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

fn timed_immortality_update(
    mut commands: Commands,
    mut q_entities: Query<(&mut TimedImmortality, Entity), With<Immortality>>,
    time: Res<Time>,
) {
    for (mut timer, entity) in q_entities.iter_mut() {
        timer.tick(time.delta());

        if timer.finished() {
            commands.entity(entity).remove::<TimedImmortalityBundle>();
        }
    }
}
