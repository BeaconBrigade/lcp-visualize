use bevy::prelude::*;

use super::components::PauseMenuMarker;

const PAUSE_BACKGROUND: Color = Color::hsl(0.0, 0.0, 0.1);

pub fn spawn_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::all(Val::Percent(100.)),
                    ..default()
                },
                ..default()
            },
            PauseMenuMarker,
        ))
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                background_color: PAUSE_BACKGROUND.into(),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size {
                        width: Val::Percent(80.),
                        height: Val::Percent(90.),
                    },
                    ..default()
                },
                ..default()
            });
        });
}

pub fn despawn_ui(mut commands: Commands, query: Query<Entity, With<PauseMenuMarker>>) {
    commands.entity(query.single()).despawn_recursive();
}
