use std::ops::{Add, AddAssign};

use bevy::prelude::*;

/// Calulated area of figure
#[derive(Component, Default)]
pub struct Area(f32);

impl Area {
    pub fn new(area: f32) -> Self {
        Self(area)
    }

    pub fn area(&self) -> f32 {
        self.0
    }

    pub(crate) fn triangle(points: &[Vec2; 3]) -> Self {
        let side_1 = points[1] - points[0];
        let side_2 = points[2] - points[0];
        let angle = side_1.angle_between(side_2);

        let area = 0.5 * side_1.length() * side_2.length() * angle.cos();

        Self(area)
    }

    pub(crate) fn circle(radius: f32) -> Self {
        use std::f32::consts::PI;

        Self(PI * radius.powi(2))
    }
}

impl Add for Area {
    type Output = Area;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Area {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}
