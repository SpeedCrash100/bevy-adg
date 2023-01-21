use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::{entity::EntityBuilder, math::RotateAroundZ};

/// The ship's target point to head to
#[derive(Component, Clone)]
pub struct ShipTargetViewPoint(Vec2);

impl ShipTargetViewPoint {
    pub fn to_vec(self) -> Vec2 {
        self.0
    }
}

impl From<Vec2> for ShipTargetViewPoint {
    fn from(vec: Vec2) -> Self {
        Self(vec)
    }
}

#[derive(Builder)]
pub struct RotationControlCreateInfo {
    #[builder(default = "1.0")]
    p: f32,
    #[builder(default = "0.0001")]
    i: f32,
    #[builder(default = "100.0")]
    d: f32,
}
/// Rexport builder with good name
pub use RotationControlCreateInfoBuilder as RotationControlBuilder;

impl EntityBuilder for RotationControlCreateInfoBuilder {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        let create_info = self.build().unwrap();

        commands
            .insert(RotationControl::new(create_info))
            .insert(ShipTargetViewPoint::from(Vec2::Y * 100.0))
    }
}

#[derive(Component)]
pub struct RotationControl {
    rotation_pid: pid::Pid<f32>,
}

impl RotationControl {
    pub fn new(info: RotationControlCreateInfo) -> Self {
        let mut pid = pid::Pid::new(0.0, 1.0);
        pid.p(info.p, 1.0).i(info.i, 1.0).d(info.d, 1.0);

        Self { rotation_pid: pid }
    }

    pub fn new_control(&mut self, target_point: Vec2, position: Vec2, angle: f32) -> f32 {
        let target_vector = target_point - position;
        let real_vector = Vec2::X.rotate_z(angle);
        let angle_diff = target_vector.angle_between(real_vector);

        self.rotation_pid.next_control_output(angle_diff).output
    }
}
