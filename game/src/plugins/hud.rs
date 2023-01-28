use bevy::prelude::*;

use crate::{
    components::{
        health::{Health, MaxHealth},
        player::Player,
        ui::{button::ButtonColorsConfig, progressbar::*},
    },
    entity::{ComponentInjectorBuilder, EntityBuildDirector, EntityBuilder},
};

#[derive(Component)]
pub struct Root;

#[derive(Component, Clone)]
pub struct PlayerHP;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_root_element)
            .add_system_to_stage(CoreStage::PostUpdate, progress_bar_update)
            .add_system_set(Self::button_effects())
            .add_system(update_player_hp);
    }
}

impl HudPlugin {
    fn create_player_hp_builder() -> impl EntityBuilder {
        let mut progress_bar_builder = ProgressBarBuilder::default();
        progress_bar_builder
            .min(0.0)
            .max(100.0)
            .style(Style {
                size: Size::new(Val::Percent(10.0), Val::Percent(2.5)),
                position: UiRect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                ..Default::default()
            })
            .color_front(Color::RED)
            .color_back(Color::WHITE);

        ComponentInjectorBuilder::new(progress_bar_builder, PlayerHP)
    }

    fn button_effects() -> SystemSet {
        SystemSet::new().with_system(button_effects)
    }
}

fn progress_bar_update(
    q_progress_bars: Query<
        (&Value, &MinValue, &MaxValue, &Children),
        (
            With<ProgressBar>,
            Or<(Changed<Value>, Changed<MinValue>, Changed<MaxValue>)>,
        ),
    >,
    mut q_progress_fronts: Query<&mut Style, With<ProgressBarFront>>,
) {
    for (val, min, max, children) in q_progress_bars.iter() {
        let style = children
            .iter()
            .find(|child| q_progress_fronts.contains(**child))
            .and_then(|child| Some(q_progress_fronts.get_mut(*child).unwrap()));

        let Some(mut style) = style else {
            continue;
        };

        let mut fill_value = 1.0;

        if min.0 < max.0 {
            fill_value = val.0.clamp(min.0, max.0) - min.0;
            fill_value /= max.0 - min.0;
        }

        style.size.width = Val::Percent(fill_value * 100.0);
    }
}

fn create_root_element(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                ..default()
            },
            ..default()
        })
        .with_children(|cs| {
            let hp_builder = HudPlugin::create_player_hp_builder();
            cs.build_entity(&hp_builder);
        });
}

fn update_player_hp(
    mut q_progress_bars: Query<(&mut Value, &mut MaxValue), (With<ProgressBar>, Without<Player>)>,
    q_player: Query<(&Health, &MaxHealth), With<Player>>,
) {
    let (player_hp, player_max_hp) = q_player.single();

    for (mut value, mut max_hp) in q_progress_bars.iter_mut() {
        *max_hp = MaxValue(player_max_hp.max_health());
        *value = Value(player_hp.health());
    }
}

fn button_effects(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonColorsConfig),
        With<Button>,
    >,
) {
    for (interaction, mut color, config) in button_query.iter_mut() {
        match interaction {
            Interaction::Clicked => *color = config.pressed_color.into(),
            Interaction::Hovered => *color = config.hovered_color.into(),
            Interaction::None => *color = config.standart_color.into(),
        }
    }
}
