use bevy::{app::AppExit, prelude::*};

use crate::states::GameState;

const MENU_FONT: &str = "fonts/FiraMono-Medium.ttf";

/// Mark for pause element
#[derive(Component)]
struct Pause;

/// Mark for unpause button
#[derive(Component)]
struct ContinueButton;

/// Mark for exit button
#[derive(Component)]
struct ExitButton;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::InGame)
            .add_system_set(Self::on_enter_pause())
            .add_system_set(Self::on_exit_pause())
            .add_system_set(Self::enter_pause_press())
            .add_system_set(Self::exit_pause_press())
            .add_system_set(Self::handle_buttons());
    }
}

impl PausePlugin {
    fn on_enter_pause() -> SystemSet {
        SystemSet::on_enter(GameState::Pause).with_system(build_menu)
    }

    fn on_exit_pause() -> SystemSet {
        SystemSet::on_exit(GameState::Pause).with_system(despawn_menu)
    }

    fn enter_pause_press() -> SystemSet {
        SystemSet::on_update(GameState::InGame).with_system(push_pause_state)
    }

    fn exit_pause_press() -> SystemSet {
        SystemSet::on_update(GameState::Pause).with_system(pop_pause_state)
    }

    fn handle_buttons() -> SystemSet {
        SystemSet::on_update(GameState::Pause)
            .with_system(button_effects::<ContinueButton>)
            .with_system(button_effects::<ExitButton>)
            .with_system(on_continue_pressed)
            .with_system(on_exit_pressed)
    }
}

fn standart_menu_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
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
        },
        ..Default::default()
    }
}

fn button_text_bundle(text: String, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            color: Color::WHITE,
            font: asset_server.load(MENU_FONT),
            font_size: 32.0,
            ..Default::default()
        },
    )
    .with_style(Style {
        align_self: AlignSelf::Center,
        ..Default::default()
    })
}

fn build_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        .with_children(|cs| {
            cs.spawn(standart_menu_button())
                .insert(ContinueButton)
                .with_children(|cs| {
                    cs.spawn(button_text_bundle("Continue".to_string(), &asset_server));
                });
        })
        .with_children(|cs| {
            cs.spawn(standart_menu_button())
                .insert(ExitButton)
                .with_children(|cs| {
                    cs.spawn(button_text_bundle("Exit".to_string(), &asset_server));
                });
        });
}

fn despawn_menu(mut commands: Commands, root: Query<Entity, With<Pause>>) {
    let Ok(entity) = root.get_single() else {
        return;
    };

    commands.entity(entity).despawn_recursive();
}

fn push_pause_state(mut key_state: ResMut<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if key_state.just_pressed(KeyCode::Escape) {
        state.push(GameState::Pause).ok();
        key_state.reset(KeyCode::Escape);
    }
}

fn pop_pause_state(mut key_state: ResMut<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if key_state.just_pressed(KeyCode::Escape) {
        state.pop().ok();
        key_state.reset(KeyCode::Escape);
    }
}

fn button_effects<T: Component>(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (With<Button>, With<T>)>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match interaction {
            Interaction::Clicked => *color = Color::rgb(0.7, 0.7, 0.7).into(),
            Interaction::Hovered => *color = Color::rgb(0.5, 0.5, 0.5).into(),
            Interaction::None => *color = Color::rgb(0.3, 0.3, 0.3).into(),
        }
    }
}

fn on_continue_pressed(
    button_query: Query<&Interaction, (With<Button>, With<ContinueButton>)>,
    mut state: ResMut<State<GameState>>,
) {
    for interaction in button_query.iter() {
        let Interaction::Clicked = *interaction else {continue;};
        state.pop().ok();
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
