use bevy::{app::AppExit, prelude::*};

use crate::{
    menu::components::{MenuMarker, QuitMarker, StartGameMarker},
    AppState,
};

const APP_BACKGROUND: Color = Color::hsl(0.0, 0.0, 0.1);
const BUTTON_BACKGROUND: Color = Color::hsl(0.0, 0.0, 0.15);
const BUTTON_HOVER: Color = Color::hsl(0.0, 0.0, 0.3);

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("starting application");
    let image = asset_server.load("textures/lcp-sim-logo.png");
    let fira_sans = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn((
            NodeBundle {
                background_color: APP_BACKGROUND.into(),
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
            },
            MenuMarker,
        ))
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
            parent
                .spawn(NodeBundle {
                    background_color: Color::hsl(0., 0.0, 0.2).into(),
                    style: Style {
                        margin: UiRect::all(Val::Px(10.)),
                        padding: UiRect::all(Val::Px(40.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Le Chatelier's Principle Visualizer!".into(),
                                style: TextStyle {
                                    color: Color::WHITE,
                                    font_size: 40.,
                                    font: fira_sans.clone(),
                                },
                            }],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(15.)),
                            padding: UiRect {
                                left: Val::Px(20.),
                                right: Val::Px(20.),
                                ..default()
                            },
                            ..default()
                        },
                        background_color: BUTTON_BACKGROUND.into(),
                        ..default()
                    },
                    StartGameMarker,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Start Game".to_string(),
                                style: TextStyle {
                                    font: fira_sans.clone(),
                                    font_size: 35.,
                                    color: Color::WHITE,
                                },
                            }],
                            ..default()
                        },
                        style: Style {
                            margin: UiRect::all(Val::Px(10.)),
                            ..default()
                        },
                        ..default()
                    });
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(15.)),
                            padding: UiRect {
                                left: Val::Px(20.),
                                right: Val::Px(20.),
                                ..default()
                            },
                            ..default()
                        },
                        background_color: BUTTON_BACKGROUND.into(),
                        ..default()
                    },
                    QuitMarker,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Quit".to_string(),
                                style: TextStyle {
                                    font: fira_sans,
                                    font_size: 35.,
                                    color: Color::WHITE,
                                },
                            }],
                            ..default()
                        },
                        style: Style {
                            margin: UiRect::all(Val::Px(10.)),
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

pub fn despawn_ui(mut commands: Commands, query: Query<Entity, With<MenuMarker>>) {
    info!("despawning menu");
    let menu = query.single();

    commands.entity(menu).despawn_recursive();
}

pub fn handle_start_game(
    mut commands: Commands,
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartGameMarker>),
    >,
) {
    let Ok((interaction, mut bg)) = query.get_single_mut() else {
        return;
    };

    match *interaction {
        Interaction::Clicked => commands.insert_resource(NextState(Some(AppState::Game))),
        Interaction::Hovered => *bg = BUTTON_HOVER.into(),
        Interaction::None => *bg = BUTTON_BACKGROUND.into(),
    }
}

pub fn handle_quit_game(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitMarker>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    let Ok((interaction, mut bg)) = query.get_single_mut() else {
        return;
    };

    match *interaction {
        Interaction::Clicked => exit.send(AppExit),
        Interaction::Hovered => *bg = BUTTON_HOVER.into(),
        Interaction::None => *bg = BUTTON_BACKGROUND.into(),
    }
}
