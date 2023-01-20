use bevy::prelude::Vec2;
use rand::{prelude::Distribution, Rng};
use statrs::distribution::Normal;

// Allow value to deviate from current
pub trait Deviate {
    /// Deviate value from current.
    ///  
    /// # Warning
    /// `deviation` must be more than 0.0
    fn deviate<R: Rng>(self, rng: &mut R, deviation: f64) -> Self;
}

impl Deviate for f32 {
    fn deviate<R: Rng>(self, rng: &mut R, deviation: f64) -> Self {
        let distribution = Normal::new(0.0, deviation).unwrap();
        let offset = distribution.sample(rng) as f32;
        self + offset
    }
}

impl Deviate for Vec2 {
    fn deviate<R: Rng>(self, rng: &mut R, deviation: f64) -> Self {
        Self {
            x: self.x.deviate(rng, deviation),
            y: self.y.deviate(rng, deviation),
        }
    }
}
