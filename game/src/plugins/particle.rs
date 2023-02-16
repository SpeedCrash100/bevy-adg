use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::{
        common::{Active, DespawnOn, Layer, MaxTimeToLive, Reset, TimeToLive},
        particle::{
            ParticleColor, ParticleGenerator, ParticleGeneratorDeviation, ParticleGeneratorRate,
            ParticleSize, ParticleVelocity,
        },
    },
    math::{Angle, RotateAroundZ},
    random::Deviate,
    states::GameState,
};

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(Self::particle_spawn())
            .add_system_set(Self::particle_update())
            .add_system_set(Self::particle_reset());
    }
}

impl ParticlePlugin {
    fn particle_spawn() -> SystemSet {
        SystemSet::on_update(GameState::InGame).with_system(particle_generator_hierarchical_spawn)
    }

    fn particle_update() -> SystemSet {
        SystemSet::on_update(GameState::InGame)
            .with_system(particle_color_update)
            .with_system(particle_size_update)
            .with_system(particle_velocity_update)
    }

    fn particle_reset() -> SystemSet {
        SystemSet::new().with_system(particles_gen_reset)
    }
}

/// For particles generators in moving objects
fn particle_generator_hierarchical_spawn(
    mut commands: Commands,
    q_generators: Query<
        (
            &ParticleGenerator,
            &ParticleGeneratorRate,
            &ParticleGeneratorDeviation,
            &GlobalTransform,
            &Parent,
        ),
        With<Active>,
    >,
    q_velocity: Query<&Velocity>,
) {
    let mut rng = rand::thread_rng();

    for (generator, rate, deviation, transform, parent) in q_generators.iter() {
        let mut particle_bundle = generator.particle();

        // Rotate velocity vectors and apply parent velocity
        let angle = transform.angle();
        let parent_velocity = q_velocity.get(parent.get()).cloned().unwrap_or_default();
        let start_velocity = particle_bundle.velocity.start().rotate_z(angle);
        let end_velocity = particle_bundle.velocity.end().rotate_z(angle);
        particle_bundle.velocity = ParticleVelocity::new(
            start_velocity + parent_velocity.linvel,
            end_velocity + parent_velocity.linvel,
        );

        // Prepare base position and scale
        let particle_position = transform.translation().truncate();
        let scale = Vec2::splat(particle_bundle.size.start()).extend(1.0);

        // Shape of particles is circle with diameter equals 1
        let shape = shapes::Circle {
            center: Vec2::ZERO,
            radius: 1.0 / 2.0,
        };

        let draw_mode = DrawMode::Fill(bevy_prototype_lyon::prelude::FillMode::color(
            particle_bundle.color.start(),
        ));

        // Generate particles
        let count = rate.particles_count(&mut rng);

        for _ in 0..count {
            // Deviate position
            let current_particle_position = particle_position.deviate(&mut rng, deviation.get());

            let shape = GeometryBuilder::build_as(
                &shape,
                draw_mode,
                Transform::from_translation(
                    current_particle_position.extend(Layer::Effects.into()),
                )
                .with_scale(scale),
            );

            commands
                .spawn(shape)
                .insert(DespawnOn::TIME_OF_LIVE)
                .insert(RigidBody::Dynamic)
                .insert(Velocity::linear(particle_bundle.velocity.start()))
                .insert(particle_bundle);
        }
    }
}

fn particle_color_update(
    mut q_particles: Query<(&ParticleColor, &TimeToLive, &MaxTimeToLive, &mut DrawMode)>,
) {
    for (particle_color, tol, max_tol, mut draw_mode) in q_particles.iter_mut() {
        let factor = 1.0 - tol.value() / max_tol.max();
        let new_color = particle_color.lerp(factor);

        if let DrawMode::Fill(ref mut fill_mode) = *draw_mode {
            fill_mode.color = new_color;
        }
    }
}

fn particle_size_update(
    mut q_particles: Query<(&ParticleSize, &TimeToLive, &MaxTimeToLive, &mut Transform)>,
) {
    for (particle_size, tol, max_tol, mut transform) in q_particles.iter_mut() {
        let factor = 1.0 - tol.value() / max_tol.max();
        let new_size = particle_size.lerp(factor);

        *transform = transform.with_scale(Vec2::splat(new_size).extend(1.0));
    }
}

fn particle_velocity_update(
    mut q_particles: Query<(
        &ParticleVelocity,
        &TimeToLive,
        &MaxTimeToLive,
        &mut Velocity,
    )>,
) {
    for (particle_velocity, tol, max_tol, mut velocity) in q_particles.iter_mut() {
        let factor = 1.0 - tol.value() / max_tol.max();
        let new_velocity = particle_velocity.lerp(factor);

        velocity.linvel = new_velocity;
    }
}

fn particles_gen_reset(
    mut commands: Commands,
    q_generators: Query<Entity, (With<ParticleGenerator>, With<Reset>)>,
) {
    for entity in q_generators.iter() {
        commands.entity(entity).remove::<Active>().remove::<Reset>();
    }
}
