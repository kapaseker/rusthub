use std::default::Default;

use bevy::prelude::*;

#[derive(Resource)]
struct FarmingConfig {
    move_speed: f32,
}

fn main() {
    App::new().add_plugins(DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Farming".into(),
                resolution: (800.0, 640.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        })
    )
        .add_systems(Startup, set_up)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, move_character)
        .insert_resource(FarmingConfig { move_speed: 100.0 })
        .run();
}

fn set_up(mut command: Commands, asset_server: Res<AssetServer>) {
    command.spawn(Camera2dBundle::default());

    let texture = asset_server.load("images/Planets/Black_hole.png");

    command.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        ..default()
    });
}

fn move_character(
    mut character: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    config: Res<FarmingConfig>,
    time: Res<Time>,
) {
    for (mut transfer, _) in &mut character {
        let mut movement = Vec3::new(0.0, 0.0, 0.0);
        let move_distance = config.move_speed * time.delta_seconds();

        if input.pressed(KeyCode::W) {
            movement.y = move_distance;
        }
        if input.pressed(KeyCode::S) {
            movement.y = -move_distance;
        }
        if input.pressed(KeyCode::A) {
            movement.x = -move_distance;
        }
        if input.pressed(KeyCode::D) {
            movement.x = move_distance;
        }

        transfer.translation += movement
    }
}