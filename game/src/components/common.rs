use bevy::prelude::*;

/// Mark that entity is active
#[derive(Component)]
pub struct Active;

/// Marks entity to be despawned in living system
#[derive(Component)]
pub enum Despawn {
    Normal,
    Recursive,
}

/// Mark entity to despawn if something happens
#[derive(Component, Clone)]
pub enum DespawnOn {
    /// If an entity is at a distance greate than the specified value, then entity must despawn
    ///
    /// # Warning
    /// Entities without [Transform](bevy::prelude::Component) component will be ignored
    OutOfRange(f32),
}
