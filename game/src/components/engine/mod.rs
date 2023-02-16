use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Rotation engine builder
mod rotation;
pub use rotation::RotationEngineCreateInfoBuilder as RotationEngineBuilder;

/// Engines that can move forward/backwards but not rotate
mod linear;
pub use linear::LinearEngineBuilder;
pub use linear::MainEngineBuilder;
pub use linear::SwayEngineBuilder;

#[derive(Component)]
pub struct Engine {
    max_force: ExternalForce,
    throttle: f32,
    min_throttle: f32,
    max_throttle: f32,
}

impl Engine {
    pub fn new(max_force: ExternalForce) -> Self {
        Self {
            max_force,
            throttle: 0.0,
            min_throttle: -1.0,
            max_throttle: 1.0,
        }
    }

    pub fn new_rotation_engine(max_torque: f32) -> Self {
        let max_force = ExternalForce {
            force: Vec2::ZERO,
            torque: max_torque,
        };
        Self::new(max_force)
    }

    pub fn new_linear_engine(max_force: Vec2) -> Self {
        let max_force = ExternalForce {
            force: max_force,
            torque: 0.0,
        };

        Self::new(max_force)
    }

    /// Returns force applied by engine
    pub fn force(&self) -> ExternalForce {
        ExternalForce {
            force: self.max_force.force * self.throttle,
            torque: self.max_force.torque * self.throttle,
        }
    }

    /// Sets throttle
    pub fn set_throttle(&mut self, throttle: f32) {
        self.throttle = (throttle).clamp(self.min_throttle, self.max_throttle);
    }
}
