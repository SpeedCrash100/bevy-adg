use bevy::{
    ecs::system::{Commands, EntityCommands},
    prelude::{BuildChildren, Bundle, ChildBuilder, Entity},
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

/// EntityChildBuildDirector use provided entity to construct child entity
pub trait EntityChildBuildDirector {
    type OutCommands;

    fn build_child_entity<'c, B: EntityBuilder>(
        &'c mut self,
        builder: &B,
    ) -> &'c mut Self::OutCommands;
}

impl<'w, 's, 'a> EntityChildBuildDirector for EntityCommands<'w, 's, 'a> {
    type OutCommands = EntityCommands<'w, 's, 'a>;

    fn build_child_entity<'c, B: EntityBuilder>(
        &'c mut self,
        builder: &B,
    ) -> &'c mut Self::OutCommands {
        self.with_children(|cs| {
            cs.build_entity(builder);
        })
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

/// Allows to concatenate 2 builders in one
///
/// # Note
/// Left builder applied before a right one
pub struct BuilderConcatenator<BLeft: EntityBuilder, BRight: EntityBuilder> {
    builder_left: BLeft,
    builder_right: BRight,
}

impl<BLeft: EntityBuilder, BRight: EntityBuilder> BuilderConcatenator<BLeft, BRight> {
    pub fn new(builder_left: BLeft, builder_right: BRight) -> Self {
        Self {
            builder_left,
            builder_right,
        }
    }
}

impl<BLeft: EntityBuilder, BRight: EntityBuilder> EntityBuilder
    for BuilderConcatenator<BLeft, BRight>
{
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        let commands = self.builder_left.build(commands);
        self.builder_right.build(commands)
    }
}
