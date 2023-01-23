use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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

impl RotateAroundZ for ExternalForce {
    fn rotate_z(self, angle: f32) -> Self {
        Self {
            force: self.force.rotate_z(angle),
            ..self
        }
    }
}

pub trait Position {
    fn position(&self) -> Vec2;
}

pub trait Angle {
    fn angle(&self) -> f32;
}

impl Position for Transform {
    fn position(&self) -> Vec2 {
        self.translation.truncate()
    }
}

impl Angle for Transform {
    fn angle(&self) -> f32 {
        let dir = self.rotation.mul_vec3(Vec3::X).truncate();
        Vec2::X.angle_between(dir)
    }
}

impl Position for GlobalTransform {
    fn position(&self) -> Vec2 {
        self.translation().truncate()
    }
}

impl Angle for GlobalTransform {
    fn angle(&self) -> f32 {
        let (_, rotation, _) = self.to_scale_rotation_translation();
        rotation.angle()
    }
}

impl Angle for Quat {
    fn angle(&self) -> f32 {
        let dir = self.mul_vec3(Vec3::X).truncate();
        Vec2::X.angle_between(dir)
    }
}
