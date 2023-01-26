use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::entity::EntityBuilder;

#[derive(Component)]
pub struct ProgressBar;

#[derive(Component)]
pub struct MinValue(pub f32);

#[derive(Component)]
pub struct MaxValue(pub f32);

#[derive(Component)]
pub struct Value(pub f32);

/// used to mark back part of progress bar
#[derive(Component)]
pub struct ProgressBarBack;

/// used to mark front part of progress bar
#[derive(Component)]
pub struct ProgressBarFront;

#[derive(Builder)]
#[builder(name = "ProgressBarBuilder")]
pub struct ProgressBarCreateInfo {
    #[builder(default = "0.0")]
    min: f32,
    #[builder(default = "0.0")]
    max: f32,
    style: Style,
    color_back: Color,
    color_front: Color,
}

impl EntityBuilder for ProgressBarBuilder {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        let info = self.build().unwrap();

        commands
            .insert(ProgressBar)
            .insert(MinValue(info.min))
            .insert(MaxValue(info.max))
            .insert(Value(info.min))
            .insert(NodeBundle {
                style: info.style,
                background_color: info.color_back.into(),
                ..Default::default()
            })
            .insert(ProgressBarBack)
            .with_children(|cs| {
                cs.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(0.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    background_color: info.color_front.into(),
                    ..Default::default()
                })
                .insert(ProgressBarFront);
            })
    }
}
