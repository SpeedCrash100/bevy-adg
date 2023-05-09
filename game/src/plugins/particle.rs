use std::collections::HashMap;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use bevy_rapier2d::prelude::*;

use crate::{
    components::{
        common::{Active, Layer, MaxTimeToLive, Reset, TimeToLive},
        particle::{
            ParticleColor, ParticleGenerator, ParticleGeneratorDeviation, ParticleGeneratorRate,
            ParticleSize, ParticleVelocity,
        },
    },
    math::{Angle, RotateAroundZ},
    random::Deviate,
    states::GameState,
};

#[derive(Resource)]
struct ParticleMesh(Handle<Mesh>);

impl FromWorld for ParticleMesh {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh_handle = meshes.add(Mesh::from(shape::Circle::new(0.5)));

        Self(mesh_handle)
    }
}

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ParticleMesh>()
            .add_system(particle_generator_hierarchical_spawn.in_set(OnUpdate(GameState::InGame)))
            .add_systems(
                (
                    particle_color_update,
                    particle_size_update,
                    particle_velocity_update,
                )
                    .in_set(OnUpdate(GameState::InGame)),
            )
            .add_system(particles_gen_reset);
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
    particle_mesh: Res<ParticleMesh>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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

        let material = materials.add(ColorMaterial::from(particle_bundle.color.start()));

        // Generate particles
        let count = rate.particles_count(&mut rng);

        for _ in 0..count {
            // Deviate position
            let current_particle_position = particle_position.deviate(&mut rng, deviation.get());

            let mesh = MaterialMesh2dBundle {
                mesh: particle_mesh.0.clone().into(),
                transform: Transform::from_translation(
                    current_particle_position.extend(Layer::Effects.into()),
                )
                .with_scale(scale),
                material: material.clone(),
                ..default()
            };

            commands
                .spawn(mesh)
                .insert(RigidBody::Dynamic)
                .insert(Velocity::linear(particle_bundle.velocity.start()))
                .insert(particle_bundle.clone());
        }
    }
}

fn particle_color_update(
    q_particles: Query<(
        &ParticleColor,
        &TimeToLive,
        &MaxTimeToLive,
        &Handle<ColorMaterial>,
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut materials_new_color = HashMap::new();

    for (particle_color, tol, max_tol, handle) in q_particles.iter() {
        let factor = 1.0 - tol.value() / max_tol.max();
        let new_color = particle_color.lerp(factor);

        materials_new_color.insert(handle, new_color);
    }

    for (handle, new_color) in materials_new_color {
        let Some(material) = materials.get_mut(handle) else {
            continue;
        };

        material.color = new_color;
    }
}

fn particle_size_update(
    mut q_particles: Query<(&ParticleSize, &TimeToLive, &MaxTimeToLive, &mut Transform)>,
) {
    q_particles.for_each_mut(|(particle_size, tol, max_tol, mut transform)| {
        let factor = 1.0 - tol.value() / max_tol.max();
        let new_size = particle_size.lerp(factor);

        *transform = transform.with_scale(Vec2::splat(new_size).extend(1.0));
    });
}

fn particle_velocity_update(
    mut q_particles: Query<(
        &ParticleVelocity,
        &TimeToLive,
        &MaxTimeToLive,
        &mut Velocity,
    )>,
) {
    q_particles.for_each_mut(|(particle_velocity, tol, max_tol, mut velocity)| {
        let factor = 1.0 - tol.value() / max_tol.max();
        let new_velocity = particle_velocity.lerp(factor);

        velocity.linvel = new_velocity;
    });
}

fn particles_gen_reset(
    mut commands: Commands,
    q_generators: Query<Entity, (With<ParticleGenerator>, With<Reset>)>,
) {
    for entity in q_generators.iter() {
        commands.entity(entity).remove::<Active>().remove::<Reset>();
    }
}
