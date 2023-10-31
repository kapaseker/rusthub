// hide console window
#![windows_subsystem = "windows"]

use std::f32::consts::PI;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use chrono::prelude::*;

struct MouseDragPlugin;

#[derive(Default)]
struct CanMoveWindow {
    movable: bool,
}

impl Plugin for MouseDragPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_move);
    }
}

fn mouse_move(
    mouse: Res<Input<MouseButton>>,
    mut window_move: bevy::prelude::Local<CanMoveWindow>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        info!("Mouse press");
        window_move.movable = true;
    }

    if mouse.just_released(MouseButton::Left) {
        info!("Mouse release");
        window_move.movable = false;
    }

    if window_move.movable {
        let mut window = window.single_mut();
        if let WindowPosition::At(window_position) = window.position {
            let mut sum_delta = Vec2::ZERO;
            for motion in mouse_motion.iter() {
                sum_delta += motion.delta;
            }
            window.position = WindowPosition::At(IVec2::new(window_position.x + sum_delta.x as i32, window_position.y + sum_delta.y as i32))
        }
    }
}

fn main() {
    App::new().add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (600.0, 600.0).into(),
            transparent: true,
            decorations: false,
            position: WindowPosition::At(IVec2::new(0, 0)),
            ..default()
        }),
        ..default()
    }))
        .insert_resource(ClearColor(Color::NONE))
        .add_systems(Startup, set_up)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, draw_watch_dial)
        .add_plugins(MouseDragPlugin)
        .run();
}

fn set_up(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let font = assets.load("fonts/open-sans.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 48.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::Center;
    commands.spawn(
        Text2dBundle {
            text: Text::from_section("12", text_style.clone())
                .with_alignment(text_alignment),
            transform: Transform::from_translation(Vec3::Y * 220f32),
            ..default()
        });

    commands.spawn(
        Text2dBundle {
            text: Text::from_section("3", text_style.clone())
                .with_alignment(text_alignment),
            transform: Transform::from_translation(Vec3::X * 220f32),
            ..default()
        });

    commands.spawn(
        Text2dBundle {
            text: Text::from_section("6", text_style.clone())
                .with_alignment(text_alignment),
            transform: Transform::from_translation(-Vec3::Y * 220f32),
            ..default()
        });

    commands.spawn(
        Text2dBundle {
            text: Text::from_section("9", text_style.clone())
                .with_alignment(text_alignment),
            transform: Transform::from_translation(-Vec3::X * 220f32),
            ..default()
        });
}

/// 看下Gizmos的文档，Gizmos的绘制发生在每一帧，所以不能使用fixtime更新
fn draw_watch_dial(mut gizmo: Gizmos) {
    let local: DateTime<chrono::offset::Local> = chrono::offset::Local::now();
    let hour = local.hour12();
    let minute = local.minute();
    let sec = local.second();

    // out circle
    gizmo.circle_2d(Vec2::ZERO, 260.0, Color::BLUE).segments(360);

    // second hand
    gizmo.line_2d(Vec2::ZERO, Vec2::from_angle(second_to_angle(sec)) * 220.0, Color::RED);

    // minute hand
    gizmo.line_2d(Vec2::ZERO, Vec2::from_angle(minute_to_angle(minute, sec)) * 160.0, Color::GREEN);

    // hour hand
    gizmo.line_2d(Vec2::ZERO, Vec2::from_angle(hour_to_angle(hour.1, minute)) * 120.0, Color::YELLOW);
}

fn second_to_angle(second: u32) -> f32 {
    return second as f32 / 60.0 * 2.0 * (-PI) + (PI / 2.0);
}

fn minute_to_angle(minute: u32, second: u32) -> f32 {
    return (minute as f32 + second as f32 / 60.0) / 60.0 * 2.0 * (-PI) + (PI / 2.0);
}

fn hour_to_angle(hour: u32, min: u32) -> f32 {
    return (hour as f32 + (min as f32) / 60.0) / 12.0 * 2.0 * (-PI) + (PI / 2.0);
}