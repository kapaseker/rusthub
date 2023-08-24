use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (1200.0, 1200.0).into(),
            title: "Esc to close".to_string(),
            ..default()
        }),
        ..default()
    })).add_systems(Update, bevy::window::close_on_esc).run();
}