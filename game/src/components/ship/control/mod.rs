use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::prelude::ExternalForce;

use crate::components::{common::Resettable, movement::Axis};

pub mod rotation;

#[derive(Component)]
pub struct ShipEngineController {
    throttles: HashMap<Axis, f32>,
}

impl ShipEngineController {
    fn new() -> Self {
        let throttles = HashMap::new();

        Self { throttles }
    }

    pub fn throttle(&self, throttle: Axis) -> f32 {
        self.throttles.get(&throttle).cloned().unwrap_or_default()
    }

    pub fn set_throttle(&mut self, throttle: Axis, value: f32) {
        self.throttles.insert(throttle, value);
    }

    pub fn throttle_up(&mut self, throttle: Axis, value: f32) {
        if !self.throttles.contains_key(&throttle) {
            self.set_throttle(throttle, 0.0);
        }

        *self.throttles.get_mut(&throttle).unwrap() += value;
    }

    pub fn throttle_down(&mut self, throttle: Axis, value: f32) {
        if !self.throttles.contains_key(&throttle) {
            self.set_throttle(throttle, 0.0);
        }

        *self.throttles.get_mut(&throttle).unwrap() -= value;
    }

    pub fn reset(&mut self) {
        self.throttles.clear();
    }
}

#[derive(Bundle)]
pub struct ShipEngineControllerBundle {
    controller: ShipEngineController,
    transform: TransformBundle,
    force: ExternalForce,
    resettable: Resettable,
}

impl ShipEngineControllerBundle {
    pub fn new() -> Self {
        Self {
            controller: ShipEngineController::new(),
            transform: TransformBundle::from(Transform::default()),
            force: ExternalForce::default(),
            resettable: Resettable,
        }
    }
}
