#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_wasm_window_resize::WindowResizePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "lcp-visualizer".to_string(),
                canvas: Some("#bevy".to_string()),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(WindowResizePlugin)
        .add_plugin(lcp_visualize::RootPlugin)
        .run();
}
