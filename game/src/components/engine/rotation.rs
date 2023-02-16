use bevy::ecs::system::EntityCommands;
use bevy_rapier2d::prelude::ExternalForce;

use crate::{components::movement::RotationAxis, entity::EntityBuilder};

use super::Engine;

#[derive(Builder)]
pub struct RotationEngineCreateInfo {
    torque: f32,
}

impl EntityBuilder for RotationEngineCreateInfoBuilder {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        let create_info = self.build().unwrap();

        commands
            .insert(Engine::new_rotation_engine(create_info.torque))
            .insert(ExternalForce::default())
            .insert(RotationAxis)
    }
}
