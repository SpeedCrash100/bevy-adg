use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::entity::EntityBuilder;

#[derive(Component)]
pub struct ButtonColorsConfig {
    pub standart_color: Color,
    pub hovered_color: Color,
    pub pressed_color: Color,
}

/// Button create information
///
/// # Warning
/// Fields `text`, `style`, `font` is required
#[derive(Builder)]
#[builder(name = "ButtonBuilder")]
pub struct ButtonCreateInfo {
    text: String,
    style: Style,
    font: Handle<Font>,

    #[builder(default = "32.0")]
    font_size: f32,

    #[builder(default = "Color::rgb(1.0, 1.0, 1.0)")]
    text_color: Color,

    #[builder(default = "Color::rgb(0.3, 0.3, 0.3)")]
    standart_color: Color,

    #[builder(default = "Color::rgb(0.5, 0.5, 0.5)")]
    hovered_color: Color,

    #[builder(default = "Color::rgb(0.7, 0.7, 0.7)")]
    pressed_color: Color,
}

impl EntityBuilder for ButtonBuilder {
    fn build<'w, 's, 'a, 'c>(
        &self,
        commands: &'c mut EntityCommands<'w, 's, 'a>,
    ) -> &'c mut EntityCommands<'w, 's, 'a> {
        let info = self.build().unwrap();

        commands
            .insert(ButtonBundle {
                style: info.style,
                background_color: info.standart_color.clone().into(),
                ..Default::default()
            })
            .insert(ButtonColorsConfig {
                standart_color: info.standart_color,
                hovered_color: info.hovered_color,
                pressed_color: info.pressed_color,
            })
            .with_children(|cs| {
                cs.spawn(
                    TextBundle::from_section(
                        info.text,
                        TextStyle {
                            font: info.font,
                            font_size: info.font_size,
                            color: info.text_color,
                        },
                    )
                    .with_style(Style {
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    }),
                );
            })
    }
}
