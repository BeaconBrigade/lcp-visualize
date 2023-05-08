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
        .add_plugin(lcp_visualize::RootPlugin)
        .run();
}
