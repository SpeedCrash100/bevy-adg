use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use bevy_rapier2d::prelude::*;

use super::prelude::*;

pub struct Builder {
    params: PhysicObjectParams,
    points: Vec<Vec2>,
    draw_mode: DrawMode,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            draw_mode: DrawMode::Fill(bevy_prototype_lyon::prelude::FillMode::color(Color::BLACK)),
            params: Default::default(),
            points: vec![Vec2::X, Vec2::Y, Vec2::NEG_Y, Vec2::NEG_X],
        }
    }
}

impl Builder {
    pub fn points(self, points: Vec<Vec2>) -> Self {
        Self { points, ..self }
    }
}

impl PhysicObjectBuilder for Builder {
    type OutputType = PhysicObjectBundle;

    fn params(self, params: PhysicObjectParams) -> Self {
        Self { params, ..self }
    }

    fn build_object(&self) -> Self::OutputType {
        // Building array for generating triangles:
        let iter_without_last = self.points[..(self.points.len() - 1)].iter();
        let iter_without_first = self.points[1..].iter();
        let pair_points = iter_without_last.zip(iter_without_first);

        let mut compound = Vec::with_capacity(pair_points.len());
        for (first, second) in pair_points {
            let shape = Collider::triangle(Vec2::ZERO, *first, *second);
            compound.push((Vec2::ZERO, 0.0, shape));
        }

        // Add last shape to create last triangle
        let shape = Collider::triangle(
            Vec2::ZERO,
            *self.points.last().unwrap(),
            *self.points.first().unwrap(),
        );
        compound.push((Vec2::ZERO, 0.0, shape));

        let collider = Collider::compound(compound);

        PhysicObjectBundle {
            params: self.params.clone(),
            collider,
            force: ExternalForce::default(),
            velocity: Velocity::default(),
            read_mass_properties: ReadMassProperties::default(),
        }
    }
}

impl DrawableObjectBuilder for Builder {
    type OutputType = ShapeBundle;

    fn build_primitive(&self) -> Self::OutputType {
        let shape = shapes::Polygon {
            points: self.points.clone(),
            closed: true,
        };

        GeometryBuilder::build_as(&shape, self.draw_mode, Transform::IDENTITY)
    }

    fn draw_mode(self, draw_mode: DrawMode) -> Self {
        Self { draw_mode, ..self }
    }
}

impl ObjectBuilder<PhysicObjectBundle, ShapeBundle> for Builder {}
