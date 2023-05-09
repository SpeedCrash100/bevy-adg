use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use bevy_rapier2d::prelude::*;

use super::prelude::*;

pub struct Builder {
    params: PhysicObjectParams,
    points: [Vec2; 3],
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            params: Default::default(),
            points: Default::default(),
        }
    }
}

impl Builder {
    pub fn points(self, points: [Vec2; 3]) -> Self {
        Self { points, ..self }
    }
}

impl PhysicObjectBuilder for Builder {
    type OutputType = PhysicObjectBundle;

    fn params(self, params: PhysicObjectParams) -> Self {
        Self { params, ..self }
    }

    fn build_object(&self) -> Self::OutputType {
        let area = Area::triangle(&self.points);
        let collider = Collider::triangle(self.points[0], self.points[1], self.points[2]);

        PhysicObjectBundle {
            params: self.params.clone(),
            collider,
            area,
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
            points: Vec::from(self.points),
            closed: true,
        };

        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        }
    }
}

impl ObjectBuilder<PhysicObjectBundle, ShapeBundle> for Builder {}
