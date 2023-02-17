use bevy::prelude::*;

use crate::components::movement::Axis;

const THROTTLE_SENSITIVITY: f32 = 0.05;

/// Trait for describe trigger conditions for effects
pub trait EffectTrigger: Component {
    /// Checks throttle and returns true if effect must be activated or false otherwise
    fn check(throttle: f32) -> bool;

    /// Returns axis used in ShipEngineController to check
    fn axis() -> Axis;
}

#[derive(Component)]
pub struct ForwardEngineEffect;
impl EffectTrigger for ForwardEngineEffect {
    fn axis() -> Axis {
        Axis::Main
    }

    fn check(throttle: f32) -> bool {
        THROTTLE_SENSITIVITY < throttle
    }
}

#[derive(Component)]
pub struct BackwardEngineEffect;
impl EffectTrigger for BackwardEngineEffect {
    fn axis() -> Axis {
        Axis::Main
    }

    fn check(throttle: f32) -> bool {
        throttle < -THROTTLE_SENSITIVITY
    }
}

#[derive(Component)]
pub struct SwayLeftEngineEffect;
impl EffectTrigger for SwayLeftEngineEffect {
    fn axis() -> Axis {
        Axis::Sway
    }

    fn check(throttle: f32) -> bool {
        THROTTLE_SENSITIVITY < throttle
    }
}

#[derive(Component)]
pub struct SwayRightEngineEffect;
impl EffectTrigger for SwayRightEngineEffect {
    fn axis() -> Axis {
        Axis::Sway
    }

    fn check(throttle: f32) -> bool {
        throttle < -THROTTLE_SENSITIVITY
    }
}

#[derive(Component)]
pub struct RotateLeftEngineEffect;
impl EffectTrigger for RotateLeftEngineEffect {
    fn axis() -> Axis {
        Axis::Rotation
    }

    fn check(throttle: f32) -> bool {
        throttle < -THROTTLE_SENSITIVITY
    }
}

#[derive(Component)]
pub struct RotateRightEngineEffect;
impl EffectTrigger for RotateRightEngineEffect {
    fn axis() -> Axis {
        Axis::Rotation
    }

    fn check(throttle: f32) -> bool {
        THROTTLE_SENSITIVITY < throttle
    }
}
