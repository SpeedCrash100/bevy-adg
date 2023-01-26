use bevy::{
    ecs::system::{Commands, EntityCommands},
    prelude::{Bundle, ChildBuilder, Entity},
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

/// Allows to inject component to entity builded by another builder
pub struct ComponentInjectorBuilder<B: EntityBuilder, I: Bundle + Clone> {
    builder: B,
    injectee: I,
}

impl<B: EntityBuilder, I: Bundle + Clone> ComponentInjectorBuilder<B, I> {
    pub fn new(builder: B, injectee: I) -> Self {
        Self { builder, injectee }
    }
}

impl<B: EntityBuilder, I: Bundle + Clone> EntityBuilder for ComponentInjectorBuilder<B, I> {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        self.builder.build(commands).insert(self.injectee.clone())
    }
}
