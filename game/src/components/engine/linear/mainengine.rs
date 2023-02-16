use bevy::{ecs::system::EntityCommands, prelude::*};

use super::LinearEngineBuilder;
use crate::{components::movement::MainAxis, entity::EntityBuilder};

#[derive(Builder)]
pub struct MainEngineCreateInfo {
    force: f32,
}

pub use MainEngineCreateInfoBuilder as MainEngineBuilder;

impl EntityBuilder for MainEngineCreateInfoBuilder {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        let info = self.build().unwrap();

        let mut builder = LinearEngineBuilder::default();
        builder.force(info.force).direction(Vec2::X);

        EntityBuilder::build(&builder, commands).insert(MainAxis)
    }
}
