use bevy::prelude::*;

#[derive(StageLabel)]
pub enum LivingStages {
    /// Health processing runs after [Update] Stage
    HealthProcessing,
    /// On this stage you can expect that [Dead] mark inserted. Run after [HealthProcessing]
    DeadProcessing,

    /// Despawn entity if it has [Despawn](crate::components::common::Despawn) mark
    DespawnProcessing,
}
