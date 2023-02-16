use bevy::{ecs::system::EntityCommands, prelude::Vec2};
use bevy_rapier2d::prelude::ExternalForce;

use super::Engine;
use crate::{components::common::Resettable, entity::EntityBuilder};

pub mod mainengine;
pub use mainengine::MainEngineBuilder;

mod swayengine;
pub use swayengine::SwayEngineBuilder;

#[derive(Builder)]
pub struct LinearEngineCreateInfo {
    force: f32,
    direction: Vec2,
}

pub use LinearEngineCreateInfoBuilder as LinearEngineBuilder;

impl EntityBuilder for LinearEngineCreateInfoBuilder {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        let create_info = self.build().unwrap();

        let force = create_info.direction.normalize() * create_info.force;

        commands
            .insert(Engine::new_linear_engine(force))
            .insert(ExternalForce::default())
            .insert(Resettable)
    }
}
