use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Rotation engine builder
mod rotation;
pub use rotation::RotationEngine;
pub use rotation::RotationEngineCreateInfoBuilder as RotationEngineBuilder;

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

    pub fn into_oneway_mode(self) -> Self {
        Self {
            throttle: 0.0,
            min_throttle: 0.0,
            ..self
        }
    }

    pub fn into_twoway_mode(self) -> Self {
        Self {
            throttle: 0.0,
            min_throttle: -1.0,
            ..self
        }
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

    /// Get throttle
    pub fn throttle(&self) -> f32 {
        self.throttle
    }

    /// Changes throttle by delta
    pub fn throttle_delta(&mut self, delta: f32) {
        self.set_throttle((self.throttle() + delta).clamp(self.min_throttle, self.max_throttle));
    }

    /// Adds throttle on engine. Negative values is ignored
    pub fn throttle_up(&mut self, value: f32) {
        if 0.0 < value {
            self.throttle_delta(value)
        }
    }

    /// Decrease throttle on engine. Negative values is ignored
    pub fn throttle_down(&mut self, value: f32) {
        if 0.0 < value {
            self.throttle_delta(-value)
        }
    }
}
