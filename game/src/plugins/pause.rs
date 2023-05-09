use bevy::{app::AppExit, prelude::*};

use crate::components::ui::button::ButtonBuilder;
use crate::entity::{ComponentInjectorBuilder, EntityChildBuildDirector};
use crate::states::GameState;

const MENU_FONT: &str = "fonts/FiraMono-Medium.ttf";

/// Mark for pause element
#[derive(Component)]
struct Pause;

/// Mark for unpause button
#[derive(Component, Clone)]
struct ContinueButton;

/// Mark for exit button
#[derive(Component, Clone)]
struct ExitButton;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            build_menu.in_schedule(OnEnter(GameState::Pause)),
            despawn_menu.in_schedule(OnExit(GameState::Pause)),
            push_pause_state.in_set(OnUpdate(GameState::InGame)),
            pop_pause_state.in_set(OnUpdate(GameState::Pause)),
        ))
        .add_systems((on_continue_pressed, on_exit_pressed).in_set(OnUpdate(GameState::Pause)));
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

    // Build continue button
    let mut continue_button_builder = ButtonBuilder::default();
    continue_button_builder
        .text("Continue".to_string())
        .font(font.clone())
        .style(button_style());

    let continue_button_builder =
        ComponentInjectorBuilder::new(continue_button_builder, ContinueButton);

    let mut exit_button_builder = ButtonBuilder::default();
    exit_button_builder
        .text("Exit".to_string())
        .font(font)
        .style(button_style());

    let exit_button_builder = ComponentInjectorBuilder::new(exit_button_builder, ExitButton);

    commands
        .spawn(Pause)
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
        .build_child_entity(&continue_button_builder)
        .build_child_entity(&exit_button_builder);
}

fn despawn_menu(mut commands: Commands, root: Query<Entity, With<Pause>>) {
    let Ok(entity) = root.get_single() else {
        return;
    };

    commands.entity(entity).despawn_recursive();
}

fn push_pause_state(
    mut key_state: ResMut<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if key_state.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Pause);
        key_state.reset(KeyCode::Escape);
    }
}

fn pop_pause_state(
    mut key_state: ResMut<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if key_state.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::InGame);
        key_state.reset(KeyCode::Escape);
    }
}

fn on_continue_pressed(
    button_query: Query<&Interaction, (With<Button>, With<ContinueButton>)>,
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
