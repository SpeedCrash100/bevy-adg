use bevy::prelude::*;
use bevy_prototype_lyon::prelude::DrawMode;
use bevy_rapier2d::prelude::*;

/// Traingle builders
pub mod triangle;

/// Circle objects builder
pub mod circle;

/// Triangles fan builder
pub mod trianglefan;

/// Plugin to enable drawing
mod plugin;

pub mod prelude {
    pub use super::DrawableObjectBuilder;
    pub use super::ObjectBuilder;
    pub use super::PhysicObjectBuilder;
    pub use super::PhysicObjectBundle;
    pub use super::PhysicObjectParams;

    pub use super::plugin::PhysicObjectPlugin;

    pub use super::circle::Builder as CircleBuilder;
    pub use super::triangle::Builder as TriangleBuilder;
    pub use super::trianglefan::Builder as TriangleFanBuilder;
}

/// Base physic object bundle. Without drawing capabilities
#[derive(Default, Bundle)]
pub struct PhysicObjectBundle {
    params: PhysicObjectParams,
    collider: Collider,

    force: ExternalForce,
    velocity: Velocity,
    read_mass_properties: ReadMassProperties,
}

/// General parameters for physic objects
#[derive(Bundle, Clone)]
pub struct PhysicObjectParams {
    pub body: RigidBody,
    pub mass_properties: ColliderMassProperties,
}

impl Default for PhysicObjectParams {
    fn default() -> Self {
        Self {
            body: RigidBody::Dynamic,
            mass_properties: ColliderMassProperties::Density(1.0),
        }
    }
}

/// Builds physic object
///
/// # Warning
/// Created bundles should be inserted _before_ any transformation, setting physic values(velocity, force, etc) otherwise they may be overwritten
pub trait PhysicObjectBuilder: Default {
    type OutputType: Bundle;

    /// Sets physic object parameters for body
    fn params(self, params: PhysicObjectParams) -> Self;

    /// Builds physic object bundle only
    fn build_object(&self) -> Self::OutputType;
}

/// Builds drawable object
///
/// # Warning
/// Created bundles should be inserted _before_ any transformation, setting physic values(velocity, force, etc) otherwise they may be overwritten
pub trait DrawableObjectBuilder: Default {
    type OutputType: Bundle;

    fn draw_mode(self, draw_mode: DrawMode) -> Self;

    /// Builds drawable primitive bundle only
    fn build_primitive(&self) -> Self::OutputType;
}

pub trait ObjectBuilder<PO: Bundle, DO: Bundle>:
    PhysicObjectBuilder<OutputType = PO> + DrawableObjectBuilder<OutputType = DO>
{
    fn build(&self) -> (PO, DO) {
        (self.build_object(), self.build_primitive())
    }
}
