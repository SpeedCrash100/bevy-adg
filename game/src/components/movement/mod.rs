use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(u8)]
pub enum Axis {
    Main,
    Sway,
    Rotation,
}

/// Trait for components that specifiy movement axis
pub trait MovementAxis: Component {
    fn axis() -> Axis;
}

#[derive(Component)]
pub struct MainAxis;
impl MovementAxis for MainAxis {
    fn axis() -> Axis {
        Axis::Main
    }
}

#[derive(Component)]
pub struct SwayAxis;
impl MovementAxis for SwayAxis {
    fn axis() -> Axis {
        Axis::Sway
    }
}

#[derive(Component)]
pub struct RotationAxis;
impl MovementAxis for RotationAxis {
    fn axis() -> Axis {
        Axis::Rotation
    }
}
