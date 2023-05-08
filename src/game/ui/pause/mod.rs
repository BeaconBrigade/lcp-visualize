use bevy::prelude::*;

mod components;
mod systems;

use super::GameUiState;
use systems::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ui.in_schedule(OnEnter(GameUiState::Paused)))
            .add_system(despawn_ui.in_schedule(OnExit(GameUiState::Paused)));
    }
}
