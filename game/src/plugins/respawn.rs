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
        app.add_system_to_stage(LivingStages::DeadProcessing, check_player_death);

        for system_set in Self::get_update_system_sets() {
            app.add_system_set(system_set);
        }
    }
}

impl RespawnPlugin {
    fn get_update_system_sets() -> Vec<SystemSet> {
        vec![
            Self::on_enter_respawn(),
            Self::on_exit_respawn(),
            Self::handle_buttons(),
        ]
    }

    fn on_enter_respawn() -> SystemSet {
        SystemSet::on_enter(GameState::Respawn).with_system(build_menu)
    }

    fn on_exit_respawn() -> SystemSet {
        SystemSet::on_exit(GameState::Respawn).with_system(despawn_menu)
    }

    fn handle_buttons() -> SystemSet {
        SystemSet::on_update(GameState::Respawn)
            .with_system(on_respawn_pressed)
            .with_system(on_exit_pressed)
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
    mut state: ResMut<State<GameState>>,
) {
    let Ok(_) = q_ships.get_single() else {
        // Return if player is alive or is not exists
        return;
    };

    state.set(GameState::Respawn).ok();
}

fn on_respawn_pressed(
    button_query: Query<&Interaction, (With<Button>, With<RespawnButton>)>,
    mut state: ResMut<State<GameState>>,
) {
    for interaction in button_query.iter() {
        let Interaction::Clicked = *interaction else {continue;};
        state.set(GameState::InGame).ok();
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
