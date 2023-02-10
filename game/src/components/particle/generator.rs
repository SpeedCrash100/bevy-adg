use bevy::prelude::*;
use rand::Rng;

use super::{ParticleBundle, ParticleBundleBuilder};

#[derive(Component)]
pub struct ParticleGenerator {
    builder: Box<dyn ParticleBundleBuilder>,
    rate: f64,
    deviation: f64,
}

impl ParticleGenerator {
    pub fn new<T: ParticleBundleBuilder + 'static>(builder: T, rate: f64, deviation: f64) -> Self {
        Self {
            builder: Box::new(builder),
            rate,
            deviation,
        }
    }

    pub fn particle(&self) -> ParticleBundle {
        self.builder.build()
    }

    /// Returns count of particles to be generated
    pub fn particles_count<R: Rng>(&self, rng: &mut R) -> usize {
        let base_count = self.rate as usize;
        let fract = self.rate.fract();

        // Generate extra particle sometimes

        let prob = rng.gen_range(0.0..1.0);
        if prob < fract {
            return base_count + 1;
        }

        base_count
    }

    pub fn deviation(&self) -> f64 {
        self.deviation
    }
}

#[derive(Bundle)]
pub struct ParticleGeneratorBundle {
    generator: ParticleGenerator,
    transform: TransformBundle,
}

impl ParticleGeneratorBundle {
    pub fn new(generator: ParticleGenerator, transform: Transform) -> Self {
        Self {
            generator,
            transform: TransformBundle::from(transform),
        }
    }
}
