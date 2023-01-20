use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use bevy_rapier2d::prelude::*;

use super::prelude::*;

pub struct Builder {
    params: PhysicObjectParams,
    radius: f32,
    draw_mode: DrawMode,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            draw_mode: DrawMode::Fill(bevy_prototype_lyon::prelude::FillMode::color(Color::BLACK)),
            params: Default::default(),
            radius: 1.0,
        }
    }
}

impl Builder {
    pub fn radius(self, radius: f32) -> Self {
        Self { radius, ..self }
    }
}

impl PhysicObjectBuilder for Builder {
    type OutputType = PhysicObjectBundle;

    fn params(self, params: PhysicObjectParams) -> Self {
        Self { params, ..self }
    }

    fn build_object(&self) -> Self::OutputType {
        let collider = Collider::ball(self.radius);

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
        let shape = shapes::Circle {
            radius: self.radius,
            center: Vec2::ZERO,
        };

        GeometryBuilder::build_as(&shape, self.draw_mode, Transform::IDENTITY)
    }

    fn draw_mode(self, draw_mode: DrawMode) -> Self {
        Self { draw_mode, ..self }
    }
}

impl ObjectBuilder<PhysicObjectBundle, ShapeBundle> for Builder {}
