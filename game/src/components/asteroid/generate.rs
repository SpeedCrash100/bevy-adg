use std::f64::consts::PI;

use bevy::prelude::*;
use rand::distributions::Distribution;
use rand::Rng;
use statrs::distribution::Normal;

const ASTEROID_IRREGULARITY: f64 = 1.0; // (0, 1]
const ASTEROID_SPIKEYNES: f64 = 0.5; // (0, 1]

/// Generates normalized polygon points that should be used to build collider of asteroid
pub fn generate_asteroid_vectors() -> Vec<Vec2> {
    let mut rng = rand::thread_rng();

    let edges_count = rng.gen_range(8..15);
    let mut out_points = Vec::with_capacity(edges_count);

    let mut current_angle = 0.0;
    let step = 2.0 * PI / (edges_count as f64);

    let angle_allow_disance = step / 2.0;
    let angle_sigma = ASTEROID_IRREGULARITY * angle_allow_disance / 3.0; // 3.0 -- Rule of 3 sigma
    let angle_distribution = Normal::new(0.0, angle_sigma).unwrap();

    let length_allow_distance = 0.9;
    let length_distribution = Normal::new(1.0, ASTEROID_SPIKEYNES / 3.0).unwrap();

    for _ in 0..edges_count {
        let angle = angle_distribution.sample(&mut rng) + current_angle;
        let angle = angle.clamp(
            current_angle - angle_allow_disance,
            current_angle + angle_allow_disance,
        );

        let length = length_distribution.sample(&mut rng);
        let length = length.clamp(1.0 - length_allow_distance, 1.0 + length_allow_distance);

        let vector = Quat::from_rotation_z(angle as f32)
            .mul_vec3(Vec3::Y)
            .truncate()
            * (length as f32);

        out_points.push(vector);

        current_angle += step;
    }

    out_points
}
