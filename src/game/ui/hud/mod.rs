use bevy::prelude::*;

mod components;
mod systems;

use systems::*;

use crate::AppState;

use super::GameUiState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_ui.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_ui.in_schedule(OnExit(AppState::Game)))
            .add_system(pause_game.in_set(OnUpdate(GameUiState::Playing)))
            .add_system(start_game.in_set(OnUpdate(GameUiState::Paused)));
    }
}
