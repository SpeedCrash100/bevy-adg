use bevy::{prelude::*, text::TextLayoutInfo};

use crate::components::health::Health;

#[derive(Component)]
pub struct HealthDisplay;

pub struct HealthDisplayPlugin;

impl Plugin for HealthDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(insert_text_boxes)
            .add_system(update_health_boxes)
            .add_system(remove_orphaned_health_boxes);
    }
}

fn insert_text_boxes(
    mut commands: Commands,
    q_living: Query<(Entity, &Health), (With<Health>, Without<HealthDisplay>)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, health) in q_living.iter() {
        let health_value = health.health();
        let health_str = format!("{:.2}", health_value);

        commands
            .entity(entity)
            .insert(HealthDisplay)
            .with_children(|cb| {
                cb.spawn(Text2dBundle {
                    text: Text::from_section(
                        health_str,
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ),
                    transform: Transform::from_translation(Vec3::Z),
                    ..default()
                })
                .insert(TextLayoutInfo {
                    size: [80.0, 30.0].into(),
                    ..default()
                });
            });
    }
}

fn update_health_boxes(
    q_living: Query<(&Health, &Children), (With<HealthDisplay>, Changed<Health>)>,
    mut q_text: Query<&mut Text, Without<HealthDisplay>>,
) {
    for (health, children) in q_living.iter() {
        let health_value = health.health();
        let health_str = format!("{:.2}", health_value);

        let text = children
            .iter()
            .find(|el| q_text.contains(**el))
            .and_then(|el| Some(q_text.get_mut(*el).unwrap()));

        let Some(mut text) = text else {
            continue;
        };

        text.sections[0].value = health_str;
    }
}

fn remove_orphaned_health_boxes(
    mut commands: Commands,
    q_entity: Query<Entity>,
    q_text: Query<(&Parent, Entity), With<Text>>,
) {
    for (parent, entity) in q_text.iter() {
        if q_entity.contains(parent.get()) {
            continue;
        }

        commands.entity(entity).despawn();
    }
}
