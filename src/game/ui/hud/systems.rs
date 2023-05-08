use bevy::prelude::*;

use crate::game::ui::GameUiState;

use super::components::HudUiMarker;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("spawning hud");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position: UiRect {
                        left: Val::Px(20.),
                        right: Val::Px(0.),
                        top: Val::Px(15.),
                        bottom: Val::Px(0.),
                    },
                    ..default()
                },
                ..default()
            },
            HudUiMarker,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Particles".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                    }],
                    ..default()
                },
                ..default()
            });
        });
}

pub fn despawn_ui(mut commands: Commands, query: Query<Entity, With<HudUiMarker>>) {
    info!("despawning hud");
    let Ok(ui) = query.get_single() else {
        return;
    };

    commands.entity(ui).despawn_recursive();
}

pub fn pause_game(mut commands: Commands, key: Res<Input<KeyCode>>) {
    if key.just_pressed(KeyCode::Escape) {
        info!("pausing game");
        commands.insert_resource(NextState(Some(GameUiState::Paused)));
    }
}

pub fn start_game(mut commands: Commands, key: Res<Input<KeyCode>>) {
    if key.just_pressed(KeyCode::Return) {
        info!("starting game");
        commands.insert_resource(NextState(Some(GameUiState::Playing)));
    }
}
