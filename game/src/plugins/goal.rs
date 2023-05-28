use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes};

use crate::{
    components::{
        common::{Layer, PositionBundle},
        player::Player,
    },
    goal::{Goal, GoalChangeEvent, GOAL_REACH_RANGE, GOAL_UI_DISTANCE},
    math::{Position, RotateAroundZ},
    states::GameState,
};

#[derive(Component)]
struct NavigateUIMark;

#[derive(Component)]
struct NavigateWorldMark;

pub struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Goal>()
            .add_event::<GoalChangeEvent>()
            .add_system(goal_update_check)
            .add_system(goal_player_reached_check)
            .add_system(world_mark_update)
            .add_system(create_navigate_ui.on_startup())
            .add_system(navigate_ui_update)
            .add_system(goal_reset_on_respawn.in_schedule(OnEnter(GameState::Respawn)));
    }
}

fn goal_update_check(
    goal: Res<Goal>,
    mut prev_goal: Local<Goal>,
    mut ev_writer: EventWriter<GoalChangeEvent>,
) {
    if goal.position != prev_goal.position {
        *prev_goal = goal.clone();
        ev_writer.send(GoalChangeEvent::new(goal.position));
        info!("Point count: {}", goal.points);
    }
}

fn goal_player_reached_check(
    mut goal: ResMut<Goal>,
    player_transform: Query<&Transform, With<Player>>,
) {
    let player_position = player_transform.single().position();

    if goal.position.distance(player_position) <= GOAL_REACH_RANGE {
        goal.player_reached();
    }
}

fn goal_reset_on_respawn(mut goal: ResMut<Goal>) {
    goal.reset();
}

fn world_mark_update(
    mut ev_reader: EventReader<GoalChangeEvent>,
    mut commands: Commands,
    prev_mark: Query<Entity, With<NavigateWorldMark>>,
) {
    let Some(goal_changed) = ev_reader.iter().last() else {
        return;
    };

    if let Ok(entity) = prev_mark.get_single() {
        commands.entity(entity).despawn_recursive();
    }

    let shape = shapes::Circle {
        radius: GOAL_REACH_RANGE,
        center: goal_changed.position().clone(),
    };

    commands
        .spawn(NavigateWorldMark)
        .insert(ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        })
        .insert(Stroke {
            color: Color::GREEN,
            options: default(),
        });
}

fn create_navigate_ui(mut commands: Commands) {
    let arrow_radius = 10.0;
    let arrow_angles: [f32; 3] = [0.0, -135.0, 135.0];

    let mut points = [Vec2::ZERO; 3];
    for i in 0..arrow_angles.len() {
        let angle = arrow_angles[i];
        let vector = Vec2::X * arrow_radius;
        points[i] = vector.rotate_z(angle.to_radians());
    }

    let shape = shapes::Polygon {
        points: Vec::from(points),
        closed: true,
    };

    commands
        .spawn(NavigateUIMark)
        .insert(ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        })
        .insert(Fill {
            color: Color::GREEN,
            options: default(),
        })
        .insert(PositionBundle::new(
            Vec2::X * GOAL_UI_DISTANCE,
            Layer::Effects,
        ));
}

fn navigate_ui_update(
    goal: Res<Goal>,
    mut q_ui: Query<&mut Transform, With<NavigateUIMark>>,
    player_transform: Query<&Transform, (With<Player>, Without<NavigateUIMark>)>,
) {
    let mut ui_transform = q_ui.single_mut();
    let player_position = player_transform.single().position();

    let position_diff = goal.position - player_position;
    let direction = position_diff.normalize();
    let new_position = player_position + direction * GOAL_UI_DISTANCE;
    let new_position_extended = new_position.extend(Layer::Effects.into());

    let angle = direction.angle_between(Vec2::X);

    let mut transform = Transform::from_translation(new_position_extended);
    transform.rotate_around(new_position_extended, Quat::from_rotation_z(-angle));

    *ui_transform = transform;
}
