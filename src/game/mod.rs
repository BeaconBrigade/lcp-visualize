use bevy::prelude::*;

use self::ui::GameUiPlugin;

mod ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(|| warn!("I'm awake"))
            .add_plugin(GameUiPlugin);
    }
}
