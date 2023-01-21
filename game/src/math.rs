use bevy::prelude::*;

pub trait RotateAroundZ {
    /// Rotate vector around Z axis
    fn rotate_z(self, angle: f32) -> Self;
}

impl RotateAroundZ for Vec2 {
    fn rotate_z(self, angle: f32) -> Self {
        Quat::from_rotation_z(angle)
            .mul_vec3(self.extend(0.0))
            .truncate()
    }
}
