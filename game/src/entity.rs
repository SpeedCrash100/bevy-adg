use bevy::{
    ecs::system::{Commands, EntityCommands},
    prelude::ChildBuilder,
};

/// Builds complex entity using provided commands
pub trait EntityBuilder {
    fn build(&self, commands: &mut EntityCommands);
}

/// EntityBuildDirector provides empty entity and commands for building using EntityBuilder
pub trait EntityBuildDirector {
    fn build_entity<B: EntityBuilder>(&mut self, builder: &B);
}

impl<'w, 's> EntityBuildDirector for Commands<'w, 's> {
    fn build_entity<B: EntityBuilder>(&mut self, builder: &B) {
        let mut commands = self.spawn(());
        builder.build(&mut commands);
    }
}

impl<'w, 's, 'a> EntityBuildDirector for ChildBuilder<'w, 's, 'a> {
    fn build_entity<B: EntityBuilder>(&mut self, builder: &B) {
        let mut commands = self.spawn(());
        builder.build(&mut commands);
    }
}
