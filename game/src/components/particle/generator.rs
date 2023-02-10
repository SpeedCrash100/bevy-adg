use bevy::prelude::*;
use rand::Rng;

use crate::components::common::Resettable;

use super::{ParticleBundle, ParticleBundleBuilder};

#[derive(Component)]
pub struct ParticleGenerator {
    builder: Box<dyn ParticleBundleBuilder>,
}

/// Count of particles must be generated in 1 tick
#[derive(Component)]
pub struct ParticleGeneratorRate(f64);

impl ParticleGeneratorRate {
    pub fn set(&mut self, value: f64) {
        self.0 = value;
    }

    pub fn get(&self) -> f64 {
        self.0
    }

    /// Generate count of particles with mean equals rate
    pub fn particles_count<R: Rng>(&self, rng: &mut R) -> usize {
        let base_count = self.0.floor() as usize;

        let fract = self.0.fract();
        if rng.gen_bool(fract) {
            return base_count + 1;
        }

        base_count
    }
}

impl From<f64> for ParticleGeneratorRate {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl Into<f64> for ParticleGeneratorRate {
    fn into(self) -> f64 {
        self.0
    }
}

/// Deviation from center of spawn position
#[derive(Component)]
pub struct ParticleGeneratorDeviation(f64);

impl ParticleGeneratorDeviation {
    pub fn set(&mut self, value: f64) {
        self.0 = value;
    }

    pub fn get(&self) -> f64 {
        self.0
    }
}

impl From<f64> for ParticleGeneratorDeviation {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl Into<f64> for ParticleGeneratorDeviation {
    fn into(self) -> f64 {
        self.0
    }
}

impl ParticleGenerator {
    pub fn new<T: ParticleBundleBuilder + 'static>(builder: T) -> Self {
        Self {
            builder: Box::new(builder),
        }
    }

    pub fn particle(&self) -> ParticleBundle {
        self.builder.build()
    }
}

#[derive(Bundle)]
pub struct ParticleGeneratorBundle {
    generator: ParticleGenerator,
    rate: ParticleGeneratorRate,
    deviation: ParticleGeneratorDeviation,
    transform: TransformBundle,
    resettable: Resettable,
}

impl ParticleGeneratorBundle {
    pub fn new(
        generator: ParticleGenerator,
        rate: impl Into<ParticleGeneratorRate>,
        deviation: impl Into<ParticleGeneratorDeviation>,
        transform: Transform,
    ) -> Self {
        Self {
            generator,
            rate: rate.into(),
            deviation: deviation.into(),
            transform: TransformBundle::from(transform),
            resettable: Resettable,
        }
    }
}
