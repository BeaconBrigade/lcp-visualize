use bevy::prelude::*;

use crate::AppState;

mod components;
mod systems;

use systems::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ui.in_schedule(OnEnter(AppState::Menu)))
            .add_system(despawn_ui.in_schedule(OnExit(AppState::Menu)))
            .add_systems((handle_start_game, handle_quit_game).in_set(OnUpdate(AppState::Menu)));
    }
}
