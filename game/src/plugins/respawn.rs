use bevy::{app::AppExit, prelude::*};

use crate::components::health::Dead;
use crate::components::player::Player;
use crate::components::ship::Ship;
use crate::components::ui::button::ButtonBuilder;
use crate::entity::{ComponentInjectorBuilder, EntityChildBuildDirector};
use crate::stages::LivingStages;
use crate::states::GameState;

const MENU_FONT: &str = "fonts/FiraMono-Medium.ttf";

/// Mark for respawn element
#[derive(Component)]
struct Respawn;

/// Mark for respawn button
#[derive(Component, Clone)]
struct RespawnButton;

/// Mark for exit button
#[derive(Component, Clone)]
struct ExitButton;

pub struct RespawnPlugin;

impl Plugin for RespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_player_death.in_base_set(LivingStages::DeadProcessing));

        app.add_systems((
            build_menu.in_schedule(OnEnter(GameState::Respawn)),
            despawn_menu.in_schedule(OnExit(GameState::Respawn)),
        ))
        .add_systems((on_respawn_pressed, on_exit_pressed).in_set(OnUpdate(GameState::Respawn)));
    }
}

fn button_style() -> Style {
    Style {
        size: Size::new(Val::Percent(30.0), Val::Percent(10.0)),
        margin: UiRect {
            left: Val::Auto,
            right: Val::Auto,
            bottom: Val::Px(20.0),
            ..Default::default()
        },
        justify_content: JustifyContent::Center,
        align_content: AlignContent::Center,
        ..Default::default()
    }
}

fn build_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(MENU_FONT);

    // Build respawn button
    let mut respawn_button_builder = ButtonBuilder::default();
    respawn_button_builder
        .text("Respawn".to_string())
        .font(font.clone())
        .style(button_style());

    let respawn_button_builder =
        ComponentInjectorBuilder::new(respawn_button_builder, RespawnButton);

    let mut exit_button_builder = ButtonBuilder::default();
    exit_button_builder
        .text("Exit".to_string())
        .font(font)
        .style(button_style());

    let exit_button_builder = ComponentInjectorBuilder::new(exit_button_builder, ExitButton);

    commands
        .spawn(Respawn)
        .insert(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(0.0),
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                },
                align_content: AlignContent::Center,
                flex_direction: FlexDirection::Column,
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Center,

                ..Default::default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
            ..Default::default()
        })
        .build_child_entity(&respawn_button_builder)
        .build_child_entity(&exit_button_builder);
}

fn despawn_menu(mut commands: Commands, root: Query<Entity, With<Respawn>>) {
    let Ok(entity) = root.get_single() else {
        return;
    };

    commands.entity(entity).despawn_recursive();
}

fn check_player_death(
    q_ships: Query<Entity, (With<Dead>, With<Ship>, With<Player>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    let Ok(entity) = q_ships.get_single() else {
        // Return if player is alive or is not exists
        return;
    };

    commands.entity(entity).remove::<Dead>();
    next_state.set(GameState::Respawn);
}

fn on_respawn_pressed(
    button_query: Query<&Interaction, (With<Button>, With<RespawnButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in button_query.iter() {
        let Interaction::Clicked = *interaction else {continue;};
        next_state.set(GameState::InGame);
    }
}

fn on_exit_pressed(
    button_query: Query<&Interaction, (With<Button>, With<ExitButton>)>,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in button_query.iter() {
        let Interaction::Clicked = *interaction else {continue;};

        exit.send(AppExit);
    }
}
