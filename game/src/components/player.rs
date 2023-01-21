use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::entity::EntityBuilder;

/// Player components mark
#[derive(Component)]
pub struct Player;

/// Allows to insert Player component in entity constucted by another builder
pub struct PlayerDecorator<B: EntityBuilder> {
    builder: B,
}

impl<B: EntityBuilder> PlayerDecorator<B> {
    pub fn new(builder: B) -> Self {
        Self { builder }
    }
}

impl<B: EntityBuilder> EntityBuilder for PlayerDecorator<B> {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        self.builder.build(commands).insert(Player)
    }
}
