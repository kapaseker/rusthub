use bevy::app::{App, Startup};
use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_systems(Startup, set_up).run();
}

fn set_up(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(PbrBundle {
        mesh: asset_server.load("models/cube/cube.gltf#Mesh0/Primitive0"),
        transform: Transform::from_xyz(0.0, -2.0, 0.0),
        material: materials.add(StandardMaterial {
            base_color: Color::RED,
            ..default()
        }),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: asset_server.load("models/monkey/Monkey.gltf#Mesh0/Primitive0"),
        material: materials.add(StandardMaterial {
            base_color: Color::BLUE,
            ..default()
        }),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(6.0, 6.0, 0.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}