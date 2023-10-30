use std::default::Default;
use bevy::input::common_conditions::input_toggle_active;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use crate::pig::PigPlugin;

use crate::res::sprite::PLANET;
use crate::res::size::SPRITE;
use crate::ui::GameUI;

mod ui;
mod pig;
mod res;

#[derive(Resource)]
pub struct Money(pub f32);

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    #[inspector(min = 0.0)]
    pub speed: f32,
}



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
        .register_type::<Player>()
        .add_systems(Startup, set_up)
        .add_systems(Update, (bevy::window::close_on_esc, move_character))
        .add_plugins((PigPlugin, GameUI))
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::M)))
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

    let texture = asset_server.load(PLANET);

    command.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2 { x: SPRITE, y: SPRITE }),
            ..default()
        },
        texture,
        ..default()
    },
                   Player {
                       speed: 100.0
                   },
                   Name::new("Player"),
    ));
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