use bevy::prelude::*;

use crate::{math::RotateAroundZ, random::Deviate};

use super::{
    ParticleBundle, ParticleBundleBuilder, ParticleColor, ParticleGenerator,
    ParticleGeneratorBundle, ParticleSize, ParticleVelocity,
};

/// Mark for fire generators
#[derive(Component)]
pub struct FireGenerator;

pub struct FireParticleBuilder {
    variance: f64,
}

impl FireParticleBuilder {
    pub fn new() -> Self {
        Self { variance: 0.2 }
    }
}

impl ParticleBundleBuilder for FireParticleBuilder {
    fn build(&self) -> ParticleBundle {
        let mut rng = rand::thread_rng();

        let angle = 0.0_f32.deviate(&mut rng, self.variance);
        let end_velocity = Vec2::NEG_X * 50.0;
        let start_velocity = end_velocity.rotate_z(angle);

        ParticleBundle {
            size: ParticleSize::new(1.0, 15.0),
            color: ParticleColor::new(Color::hsl(59.0, 1.0, 0.8), Color::hsl(30.0, 1.0, 0.5)),
            velocity: ParticleVelocity::new(start_velocity, end_velocity),
        }
    }
}

#[derive(Bundle)]
pub struct FireGeneratorBundle {
    base_bundle: ParticleGeneratorBundle,
    mark: FireGenerator,
}

impl FireGeneratorBundle {
    pub fn new(rate: f64, deviation: f64, transform: Transform) -> Self {
        let builder = FireParticleBuilder::new();
        let generator = ParticleGenerator::new(builder, rate, deviation);

        Self {
            base_bundle: ParticleGeneratorBundle::new(generator, transform),
            mark: FireGenerator,
        }
    }
}
