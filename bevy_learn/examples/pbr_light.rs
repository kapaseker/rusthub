use bevy::app::{App, Startup};
use bevy::prelude::*;

const MOVE_SPEED_PER_SECOND: f32 = 2.0;

#[derive(Component)]
struct MarkerPbr;

#[derive(Component)]
struct MarkerLight;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_systems(Startup, set_up).add_systems(Update, zoom_camera).run();
}

fn zoom_camera(
    mut light: Query<&mut Transform, (Without<Camera>, Without<MarkerPbr>, With<MarkerLight>)>,
    mut model: Query<&mut Transform, (Without<Camera>, With<MarkerPbr>)>,
    mut camera: Query<(&mut Transform, &mut Camera), With<Camera3d>>,
    timer: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    let delta_time = timer.delta_seconds();

    let move_distance = if input.pressed(KeyCode::Up) {
        -delta_time
    } else if input.pressed(KeyCode::Down) {
        delta_time
    } else {
        0.0
    } * MOVE_SPEED_PER_SECOND;

    let light_move_distance = if input.pressed(KeyCode::W) {
        -delta_time
    } else if input.pressed(KeyCode::S) {
        delta_time
    } else {
        0.0
    } * MOVE_SPEED_PER_SECOND;

    let rotation_distance = if input.pressed(KeyCode::Left) {
        -delta_time
    } else if input.pressed(KeyCode::Right) {
        delta_time
    } else {
        0.0
    } * MOVE_SPEED_PER_SECOND;

    let (mut camera_transform, _) = camera.single_mut();

    camera_transform.translation += Vec3::new(0.0, move_distance, move_distance);
    // camera_transform.rotation *= Quat::from_rotation_y(rotation_distance);

    for mut transfor in model.iter_mut() {
        transfor.rotation *= Quat::from_rotation_y(rotation_distance);
    }

    for mut transfor in light.iter_mut() {
        transfor.translation += Vec3::new(0.0, light_move_distance, 0.0);
    }

    // let mut transfor = model.single_mut();
    // transfor.rotation *= Quat::from_rotation_y(rotation_distance);
}

fn set_up(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::BLUE.into()),
        ..default()
    }, MarkerPbr));

    commands.spawn((PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(2.0).into()),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_xyz(0.0, 0.01, 0.0),
        ..default()
    }, MarkerPbr));

    commands.spawn((PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            radius: 5.0,
            range: 10.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        ..default()
    }, MarkerLight));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}