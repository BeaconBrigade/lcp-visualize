#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "lcp-visualizer".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_ui)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("starting application");
    let image = asset_server.load("textures/bevy.png");
    let fira_sans = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn(NodeBundle {
            background_color: Color::WHITE.into(),
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size {
                        width: Val::Px(200.),
                        height: Val::Px(200.),
                    },
                    ..default()
                },
                image: UiImage {
                    texture: image,
                    ..default()
                },
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Hello World!".into(),
                        style: TextStyle {
                            color: Color::WHITE,
                            font_size: 40.,
                            font: fira_sans,
                        },
                    }],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                style: Style {
                    padding: UiRect::all(Val::Px(40.0)),
                    margin: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                background_color: Color::hsl(0.0, 0.0, 0.3).into(),
                ..default()
            });
        });
}
