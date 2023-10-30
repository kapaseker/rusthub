use std::f32::consts::PI;
use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (600.0, 600.0).into(),
            title: "WatchDial".to_string(),
            ..default()
        }),
        ..default()
    }))
        .add_systems(Startup, set_up)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, draw_watch_dial)
        .run();
}

fn set_up(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn draw_watch_dial(mut gizmo: Gizmos, time: Res<Time>) {
    gizmo.circle_2d(Vec2::ZERO, 260.0, Color::BLUE).segments(360);
    gizmo.line_2d(Vec2::ZERO, Vec2::from_angle(time.elapsed_seconds() % 60.0 / 60.0 * 2.0 * -PI) * 240.0, Color::RED);
}