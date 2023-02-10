use std::ops::{Add, Mul, Sub};

use bevy::prelude::*;

/// Color with math operations
#[derive(Clone, Copy)]
pub struct MathColor(pub Color);

impl Add for MathColor {
    type Output = MathColor;

    fn add(self, rhs: Self) -> Self::Output {
        let Color::Hsla {
            hue: lhs_hue,
            saturation: lhs_saturation,
            lightness: lhs_lightness,
            alpha,
        } = self.0.as_hsla() else {
            unreachable!()
        };

        let Color::Hsla {
            hue: rhs_hue,
            saturation: rhs_saturation,
            lightness: rhs_lightness,
            alpha: _,
        } = rhs.0.as_hsla() else {
            unreachable!()
        };

        let result = Color::Hsla {
            hue: lhs_hue + rhs_hue,
            saturation: lhs_saturation + rhs_saturation,
            lightness: lhs_lightness + rhs_lightness,
            alpha,
        };

        MathColor(result)
    }
}

impl Sub for MathColor {
    type Output = MathColor;
    fn sub(self, rhs: Self) -> Self::Output {
        let Color::Hsla {
            hue: lhs_hue,
            saturation: lhs_saturation,
            lightness: lhs_lightness,
            alpha,
        } = self.0.as_hsla() else {
            unreachable!()
        };

        let Color::Hsla {
            hue: rhs_hue,
            saturation: rhs_saturation,
            lightness: rhs_lightness,
            alpha: _,
        } = rhs.0.as_hsla() else {
            unreachable!()
        };

        let result = Color::Hsla {
            hue: lhs_hue - rhs_hue,
            saturation: lhs_saturation - rhs_saturation,
            lightness: lhs_lightness - rhs_lightness,
            alpha,
        };

        MathColor(result)
    }
}

impl Mul<f32> for MathColor {
    type Output = MathColor;
    fn mul(self, rhs: f32) -> Self::Output {
        let Color::Hsla {
            hue: lhs_hue,
            saturation: lhs_saturation,
            lightness: lhs_lightness,
            alpha,
        } = self.0.as_hsla() else {
            unreachable!()
        };

        let result = Color::Hsla {
            hue: lhs_hue * rhs,
            saturation: lhs_saturation * rhs,
            lightness: lhs_lightness * rhs,
            alpha,
        };

        MathColor(result)
    }
}

impl Into<Color> for MathColor {
    fn into(self) -> Color {
        self.0
    }
}
