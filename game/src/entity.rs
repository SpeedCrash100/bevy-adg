use bevy::{
    ecs::system::{Commands, EntityCommands},
    prelude::{ChildBuilder, Entity},
};

/// Builds complex entity using provided commands
pub trait EntityBuilder {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a>;
}

/// EntityBuildDirector provides empty entity and commands for building using EntityBuilder
pub trait EntityBuildDirector {
    fn build_entity<B: EntityBuilder>(&mut self, builder: &B) -> Entity;
}

impl<'w, 's> EntityBuildDirector for Commands<'w, 's> {
    fn build_entity<B: EntityBuilder>(&mut self, builder: &B) -> Entity {
        let mut commands = self.spawn(());
        builder.build(&mut commands).id()
    }
}

impl<'w, 's, 'a> EntityBuildDirector for ChildBuilder<'w, 's, 'a> {
    fn build_entity<B: EntityBuilder>(&mut self, builder: &B) -> Entity {
        let mut commands = self.spawn(());
        builder.build(&mut commands).id()
    }
}
