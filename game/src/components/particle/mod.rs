use bevy::prelude::*;

mod mathcolor;
use mathcolor::MathColor;

mod generator;
pub use generator::ParticleGenerator;
pub use generator::ParticleGeneratorBundle;
pub use generator::ParticleGeneratorDeviation;
pub use generator::ParticleGeneratorRate;

/// Fire particles
pub mod fire;

#[derive(Component, Clone, Copy)]
pub struct ParticleSize {
    start: f32,
    end: f32,
}

impl ParticleSize {
    pub fn new(start: f32, end: f32) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> f32 {
        self.start
    }
    /// Lerp for factor [0, 1]
    pub fn lerp(&self, factor: f32) -> f32 {
        self.start + factor * (self.end - self.start)
    }
}

#[derive(Component, Clone, Copy)]
pub struct ParticleColor {
    start: MathColor,
    end: MathColor,
}

impl ParticleColor {
    pub fn new(start: Color, end: Color) -> Self {
        Self {
            start: MathColor(start),
            end: MathColor(end),
        }
    }

    pub fn start(&self) -> Color {
        self.start.0
    }

    /// Lerp for factor [0, 1]
    pub fn lerp(&self, factor: f32) -> Color {
        (self.start + (self.end - self.start) * factor).into()
    }
}

#[derive(Component, Clone, Copy)]
pub struct ParticleVelocity {
    start: Vec2,
    end: Vec2,
}

impl ParticleVelocity {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> Vec2 {
        self.start
    }

    pub fn end(&self) -> Vec2 {
        self.end
    }

    /// Lerp for factor [0, 1]
    pub fn lerp(&self, factor: f32) -> Vec2 {
        self.start + (self.end - self.start) * factor
    }
}

/// Typical Particle components
/// Used to create particle in generators
#[derive(Bundle, Clone, Copy)]
pub struct ParticleBundle {
    pub size: ParticleSize,
    pub color: ParticleColor,
    pub velocity: ParticleVelocity,
}

/// Used to generate new ParticleBundle to spawn in generators
pub trait ParticleBundleBuilder: Send + Sync {
    fn build(&self) -> ParticleBundle;
}
