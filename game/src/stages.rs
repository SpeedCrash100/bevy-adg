use bevy::prelude::*;

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone)]
#[system_set(base)]
pub enum LivingStages {
    /// Health processing runs after [Update] Stage
    HealthProcessing,
    /// On this stage you can expect that [Dead] mark inserted. Run after [HealthProcessing]
    DeadProcessing,

    /// Despawn entity if it has [Despawn](crate::components::common::Despawn) mark
    DespawnProcessing,
}

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone)]
#[system_set(base)]
pub struct UiUpdate;
