use bevy::prelude::*;
use bevy_mod_picking::*;
use bevy_mod_picking::prelude::RaycastPickCamera;
use crate::board::BoardPlugin;

mod pieces;
mod board;
mod constant;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample8)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1200.0, 1200.0).into(),
                title: "Chess".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(BoardPlugin)
        .add_systems(Startup, (setup, pieces::create_pieces))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane { size: 16.0, subdivisions: 0 })),
    //     material: materials.add(Color::SALMON.into()),
    //     transform: Transform::from_translation(Vec3::new(1.0, 0.0, 1.0)),
    //     ..default()
    // });
    commands.spawn((Camera3dBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7.0, 26.0, 7.0),
        )),
        ..default()
    }, RaycastPickCamera::default()));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
