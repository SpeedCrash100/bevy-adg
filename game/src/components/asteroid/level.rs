use bevy::prelude::*;

#[derive(Component)]
pub struct AsteroidSizeLevel(i32);

impl AsteroidSizeLevel {
    pub fn new(level: i32) -> Self {
        debug_assert!(0 < level);
        Self(level)
    }

    pub fn level(&self) -> i32 {
        self.0
    }

    pub fn typical_radius(&self) -> f32 {
        (2.0 as f32).powi(self.level() + 3)
    }
}
