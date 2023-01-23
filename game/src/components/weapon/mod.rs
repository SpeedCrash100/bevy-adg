pub use bevy::prelude::*;

/// Projectiles for weapons
pub mod projectile;

/// Basic bullet launcher
pub mod machinegun;
pub use machinegun::{MachineGun, MachineGunBuilder};

/// Basic weapon component with firerate checking
#[derive(Component)]
pub struct Weapon {
    /// Firerate in shots per second
    firerate: f32,
    /// Setted automaticaly, represent minimal time to make next shoot
    time_to_shot: f32,
    /// Projectile velocity on fire
    velocity: f32,
    /// Angle deviation
    accuracy: f32,
}

impl Weapon {
    pub fn new(firerate: f32, velocity: f32, accuracy: f32) -> Self {
        Self {
            time_to_shot: 0.0,
            firerate,
            velocity,
            accuracy,
        }
    }

    pub fn update(&mut self, time: f32) {
        if self.time_to_shot > 0.0 {
            self.time_to_shot -= time;
        }
    }

    /// Returns true if gun should fire
    pub fn fire(&mut self) -> bool {
        if self.time_to_shot <= 0.0 {
            self.time_to_shot = 1.0 / self.firerate;
            return true;
        }
        false
    }

    pub fn accuracy(&self) -> f32 {
        self.accuracy
    }

    pub fn velocity(&self) -> f32 {
        self.velocity
    }
}
