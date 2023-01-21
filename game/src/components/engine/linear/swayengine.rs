use bevy::{ecs::system::EntityCommands, prelude::*};

use super::LinearEngineBuilder;
use crate::entity::EntityBuilder;

/// Mark that engine can be used for make things move Sway right/left
#[derive(Component)]
pub struct SwayEngine;

#[derive(Builder)]
pub struct SwayEngineCreateInfo {
    force: f32,
}

pub use SwayEngineCreateInfoBuilder as SwayEngineBuilder;

impl EntityBuilder for SwayEngineCreateInfoBuilder {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        let info = self.build().unwrap();

        let mut builder = LinearEngineBuilder::default();
        builder.force(info.force).direction(Vec2::NEG_Y);

        EntityBuilder::build(&builder, commands).insert(SwayEngine)
    }
}
