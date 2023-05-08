use bevy::prelude::*;

use self::{hud::HudPlugin, pause::PausePlugin};

mod hud;
mod pause;

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, States)]
pub enum GameUiState {
    #[default]
    Playing,
    Paused,
}

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameUiState>()
            .add_plugin(HudPlugin)
            .add_plugin(PausePlugin);
    }
}
