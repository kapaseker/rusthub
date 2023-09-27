use std::f32::consts::PI;

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins)
        .add_systems(Startup, set_up)
        .insert_resource(RotateSpeed(10.0))
        .add_systems(Update, (bevy::window::close_on_esc, camera_by_controller, camera_by_self))
        .run()
}

#[derive(Resource)]
struct RotateSpeed(f32);

#[derive(Component)]
struct Controller;

#[derive(Component)]
struct CameraController;

fn set_up(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((PbrBundle {
        mesh: meshes.add(shape::Cube::new(2.0).into()),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }, Controller)).with_children(|parent| {
        parent.spawn((Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }, CameraController));
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(4.0).into()),
        material: materials.add(Color::BLUE.into()),
        transform: Transform::from_xyz(0.0, -2.0, 0.0),
        ..default()
    });
}

fn camera_by_self(time: Res<Time>,
                  speed: Res<RotateSpeed>,
                  keyboard_input: Res<Input<KeyCode>>,
                  mut controller_query: Query<(&mut Transform), With<CameraController>>, ) {
    let mut trans = controller_query.single_mut();
    let mut rotate_direction = 0.0;
    if keyboard_input.pressed(KeyCode::Down) {
        rotate_direction = -1.0;
    } else if keyboard_input.pressed(KeyCode::Up) {
        rotate_direction = 1.0;
    }
    trans.rotate_around(Vec3::ZERO, Quat::from_rotation_x(rotate_direction * speed.0 * time.delta_seconds()));
}

fn camera_by_controller(
    time: Res<Time>,
    speed: Res<RotateSpeed>,
    keyboard_input: Res<Input<KeyCode>>,
    mut controller_query: Query<(&mut Transform), With<Controller>>,
) {
    let mut trans = controller_query.single_mut();
    let mut rotate_direction = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        rotate_direction = -1.0;
    } else if keyboard_input.pressed(KeyCode::Right) {
        rotate_direction = 1.0;
    }
    trans.rotate_around(Vec3::ZERO, Quat::from_rotation_y(rotate_direction * speed.0 * time.delta_seconds()));
}