use std::default::Default;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

#[derive(Resource)]
pub struct Money(pub f32);

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

const SPRITE_SIZE: f32 = 20.0f32;

fn main() {
    App::new().add_plugins(DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Farming".into(),
                resolution: (800.0, 600.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        })
    )
        .add_systems(Startup, set_up)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, (move_character, spawn_pig))
        .insert_resource(Money(100.0))
        .run();
}

fn set_up(mut command: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 200.0,
        min_height: 150.0,
    };

    command.spawn(camera);

    let texture = asset_server.load("images/Planets/Black_hole.png");

    command.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2 { x: SPRITE_SIZE, y: SPRITE_SIZE }),
            ..default()
        },
        texture,
        ..default()
    },
                   Player {
                       speed: 100.0
                   }
    ));
}

fn spawn_pig(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();

    if money.0 >= 10.0 {

        money.0 -= 10.0;

        info!("Spent $10 on a pig, remaining: {}", money.0);

        let pig_texture = asset_server.load("images/monster/nose_red.png");
        let mut pig_transform = *player_transform;
        pig_transform.translation.z = -1.0;
        command.spawn(
            (
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2 { x: SPRITE_SIZE, y: SPRITE_SIZE }),
                        ..default()
                    },
                    texture: pig_texture,
                    transform: pig_transform,
                    ..default()
                }
            )
        );
    }
}

fn move_character(
    mut character: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transfer, player) in &mut character {
        let mut movement = Vec3::new(0.0, 0.0, 0.0);
        let move_distance = player.speed * time.delta_seconds();

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