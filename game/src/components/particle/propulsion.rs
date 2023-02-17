use bevy::prelude::*;

use crate::{components::common::TimeToLiveBundle, math::RotateAroundZ, random::Deviate};

use super::{
    ParticleBundle, ParticleBundleBuilder, ParticleColor, ParticleGenerator,
    ParticleGeneratorBundle, ParticleSize, ParticleVelocity,
};

pub struct PropulsionParticleBuilder {
    variance: f64,
}

impl PropulsionParticleBuilder {
    pub fn new(variance: f64) -> Self {
        Self { variance }
    }
}

impl ParticleBundleBuilder for PropulsionParticleBuilder {
    fn build(&self) -> super::ParticleBundle {
        let mut rng = rand::thread_rng();

        let angle = 0.0_f32.deviate(&mut rng, self.variance);
        let end_velocity = Vec2::NEG_X * 100.0;
        let start_velocity = end_velocity.rotate_z(angle);

        ParticleBundle {
            size: ParticleSize::new(1.0, 5.0),
            color: ParticleColor::new(Color::hsl(185.0, 1.0, 0.8), Color::hsl(205.0, 1.0, 0.5)),
            velocity: ParticleVelocity::new(start_velocity, end_velocity),
            time_to_live: TimeToLiveBundle::new(0.3),
        }
    }
}

#[derive(Bundle)]
pub struct PropulsionParticleGeneratorBundle {
    base_bundle: ParticleGeneratorBundle,
}

impl PropulsionParticleGeneratorBundle {
    pub fn new(variance: f64, rate: f64, deviation: f64, transform: Transform) -> Self {
        let builder = PropulsionParticleBuilder::new(variance);
        let generator = ParticleGenerator::new(builder);

        Self {
            base_bundle: ParticleGeneratorBundle::new(generator, rate, deviation, transform),
        }
    }
}
