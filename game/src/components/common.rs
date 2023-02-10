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

/// Special despawn timer used to despawn entities after specified time
#[derive(Component, Clone, Copy)]
pub struct TimeToLive(f32);

impl TimeToLive {
    fn new(time: f32) -> Self {
        Self(time)
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    pub fn decrease(&mut self, time: f32) {
        self.0 -= time;
    }

    pub fn finished(&self) -> bool {
        self.0 < 0.0
    }
}

/// Max value for [TimeToLive]
#[derive(Component, Clone, Copy)]
pub struct MaxTimeToLive(f32);

impl MaxTimeToLive {
    fn new(time: f32) -> Self {
        Self(time)
    }

    pub fn max(&self) -> f32 {
        self.0
    }
}

#[derive(Bundle)]
pub struct TimeToLiveBundle {
    time: TimeToLive,
    max: MaxTimeToLive,
}

impl TimeToLiveBundle {
    pub fn new(time: f32) -> Self {
        Self {
            time: TimeToLive::new(time),
            max: MaxTimeToLive::new(time),
        }
    }
}

bitflags! {
    #[derive(Component)]
    pub struct DespawnOn: u8 {
        /// Despawn when entity is way too far
        const OUT_OF_RANGE = 0x01;

        /// Despawn entity when exiting from InGame state
        const ON_EXIT_GAME = 0x02;

        /// Despawn entity when time to live elapsed
        const TIME_OF_LIVE = 0x04;
    }
}

/// Mark that entity should be resetted when game ends
#[derive(Component, Clone)]
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

    Effects,
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
