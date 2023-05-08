//! Main implementation of the simulation

#![allow(clippy::type_complexity)]

use bevy::prelude::*;

pub mod game;
pub mod menu;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
pub enum AppState {
    #[default]
    Menu,
    Game,
}

pub struct RootPlugin;

impl Plugin for RootPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_plugin(game::GamePlugin)
            .add_plugin(menu::MenuPlugin)
            .add_startup_system(spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    info!("creating camera");
    commands.spawn(Camera2dBundle::default());
}
