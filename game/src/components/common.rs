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

bitflags! {
    #[derive(Component)]
    pub struct DespawnOn: u8 {
        /// Despawn when entity is way too far
        const OUT_OF_RANGE = 0x01;

        /// Despawn entity when exiting from InGame state
        const ON_EXIT_GAME = 0x02;
    }
}

/// Mark that entity should be resetted when game ends
#[derive(Component)]
pub struct Resettable;

/// Entity is queued to reset
///
/// The system that performs reset must remove this flag when entity resetted
#[derive(Component)]
pub struct Reset;

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
