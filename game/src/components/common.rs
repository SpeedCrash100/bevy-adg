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

#[derive(Component, Clone, Copy)]
#[repr(i32)]
pub enum Layer {
    // Backgrounds layers
    BackgroundLow,
    BackgroundMiddle,
    BackgroundHigh,

    Main,
}

impl From<Layer> for f32 {
    fn from(value: Layer) -> Self {
        value as i32 as f32
    }
}

#[derive(Bundle)]
pub struct PositionBundle {
    transform: TransformBundle,
    layer: Layer,
}

impl PositionBundle {
    pub fn new(position: Vec2, layer: Layer) -> Self {
        Self {
            transform: TransformBundle::from(Transform::from_translation(
                position.extend(layer.into()),
            )),
            layer,
        }
    }
}
