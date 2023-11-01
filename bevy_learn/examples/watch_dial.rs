// hide console window
#![windows_subsystem = "windows"]

use std::f32::consts::PI;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use chrono::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

struct MouseDragPlugin;

#[derive(Default)]
struct CanMoveWindow {
    movable: bool,
}

#[derive(Resource)]
struct JumpSecond(bool);

impl Plugin for MouseDragPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_drag_move_window);
    }
}

fn mouse_drag_move_window(
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
            mouse_motion.clear();
            window.position = WindowPosition::At(IVec2::new(window_position.x + sum_delta.x as i32, window_position.y + sum_delta.y as i32))
        }
    }
}

const NUM_WORD_DISTANCE: f32 = 220f32;

const SHORT_HAND: f32 = 20f32;
const SECOND_HAND_LEN: f32 = 220f32;
const MINUTE_HAND_LEN: f32 = 160f32;
const HOUR_HAND_LEN: f32 = 120f32;

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
        .insert_resource(JumpSecond(true))
        .insert_resource(ClearColor(Color::NONE))
        .add_systems(Startup, set_up)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, (draw_watch_dial, draw_watch_hand, ui_control_panel))
        .add_plugins(MouseDragPlugin)
        .add_plugins(EguiPlugin)
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

    for i in 1..=12 {
        let angle_vec = Vec2::from_angle((90f32 - i as f32 * 30f32).to_radians()) * NUM_WORD_DISTANCE;
        commands.spawn(
            Text2dBundle {
                text: Text::from_section(format!("{}", i), text_style.clone()).with_alignment(text_alignment),
                transform: Transform::from_translation(Vec3::new(angle_vec.x, angle_vec.y, 0.0)),
                ..default()
            });
    }
}

fn draw_watch_hand(mut gizmo: Gizmos, mut jump: ResMut<JumpSecond>) {
    let local: DateTime<chrono::offset::Local> = chrono::offset::Local::now();
    let hour = local.hour12();
    let minute = local.minute();

    // out circle
    gizmo.circle_2d(Vec2::ZERO, 260.0, Color::BLUE).segments(360);

    let mut sec = local.second() as f32;
    if !jump.0 {
        sec += local.nanosecond() as f32 / 1000000000f32
    }
    let second_hand = Vec2::from_angle(second_to_angle(sec));
    gizmo.line_2d(-second_hand * SHORT_HAND, second_hand * SECOND_HAND_LEN, Color::RED);

    // minute hand
    let minute_hand = Vec2::from_angle(minute_to_angle(minute, sec as u32));
    gizmo.line_2d(-minute_hand * SHORT_HAND, minute_hand * MINUTE_HAND_LEN, Color::GREEN);

    // hour hand
    let hour_hand = Vec2::from_angle(hour_to_angle(hour.1, minute));
    gizmo.line_2d(-hour_hand * SHORT_HAND, hour_hand * HOUR_HAND_LEN, Color::YELLOW);
}

fn ui_control_panel(mut contexts: EguiContexts, mut jump: ResMut<JumpSecond>) {
    egui::Window::new("").movable(false).resizable(false).title_bar(false).show(contexts.ctx_mut(), |ui| {
        ui.checkbox(&mut (jump.0), "jump second");
    });
}

/// 看下Gizmos的文档，Gizmos的绘制发生在每一帧，所以不能使用fixtime更新
fn draw_watch_dial(mut gizmo: Gizmos) {
    for i in 0..60 {
        let mut len = 250f32;
        if i % 5 == 0 {
            len = 240f32;
        }
        // gizmo.line_2d(Vec2::from_angle((i as f32 * 6.0).to_degrees()) * 210f32, Vec2::from_angle((i as f32).to_degrees()) * 260f32, Color::BLUE);
        gizmo.line_2d(Vec2::from_angle((i as f32 * 6.0).to_radians()) * len,
                      Vec2::from_angle((i as f32 * 6.0).to_radians()) * 260f32, Color::BLUE);
    }

    // out circle
    gizmo.circle_2d(Vec2::ZERO, 260.0, Color::BLUE).segments(360);
}

fn second_to_angle(second: f32) -> f32 {
    return second / 60.0 * 2.0 * (-PI) + (PI / 2.0);
}

fn minute_to_angle(minute: u32, second: u32) -> f32 {
    return (minute as f32 + second as f32 / 60.0) / 60.0 * 2.0 * (-PI) + (PI / 2.0);
}

fn hour_to_angle(hour: u32, min: u32) -> f32 {
    return (hour as f32 + (min as f32) / 60.0) / 12.0 * 2.0 * (-PI) + (PI / 2.0);
}