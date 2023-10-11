use std::f32::consts::PI;

use bevy::app::{App, Startup};
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

#[derive(Resource)]
struct GltfPack(Handle<Gltf>);

#[derive(Component)]
struct Light;

#[derive(Resource)]
struct LoadSound(Handle<AudioSource>);

#[derive(Event)]
struct ModelLoaded;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum ModelState {
    #[default]
    Loading,
    Loaded,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1270.0, 960.0).into(),
                title: "Building Town".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(bevy_screen_diags::ScreenDiagsTextPlugin)
        .add_systems(Startup, (set_up_apartment, set_up_camera, set_up_plane, set_up_light, set_up_sound))
        .add_systems(OnEnter(ModelState::Loaded), load_apartment)
        .add_systems(Update,check_apartment.run_if(in_state(ModelState::Loading)))
        .add_systems(Update, rotate_light.run_if(in_state(ModelState::Loaded)))
        .add_systems(Update, play_load_sound)
        .add_event::<ModelLoaded>()
        .add_state::<ModelState>()
        .run();
}

fn set_up_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(LoadSound(asset_server.load("sound/parakeet.ogg")));
}

fn set_up_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 30.0, 30.0)),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}

fn set_up_plane(mut commands: Commands,
                mut meshes: ResMut<Assets<Mesh>>,
                mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(20000.0).into()),
        material: materials.add(Color::BLUE.into()),
        ..default()
    });
}

fn set_up_light(mut commands: Commands) {
    commands.spawn((DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            // translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    }, Light));
}

fn set_up_apartment(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let gltf = asset_server.load("models/k_11/GZ_K11.gltf");
    commands.insert_resource(GltfPack(gltf));
}

fn check_apartment(
    mut next_state: ResMut<NextState<ModelState>>,
    my: ResMut<GltfPack>,
    mut event_writer: EventWriter<ModelLoaded>,
    assets_gltf: Res<Assets<Gltf>>,
) {

    if assets_gltf.get(&my.0).is_none() {
        return;
    }

    next_state.set(ModelState::Loaded);
    event_writer.send(ModelLoaded);
}

fn load_apartment(
    mut commands: Commands,
    my: ResMut<GltfPack>,
    assets_gltf: Res<Assets<Gltf>>) {

    println!("load apartments");

    let gltf = assets_gltf.get(&my.0).unwrap();
    // 250 * 200
    for i in -10..10 {
        for j in -15..50 {
            commands.spawn(SceneBundle {
                transform: Transform {
                    scale: Vec3::new(0.01, 0.01, 0.01),
                    translation: Vec3::new(i as f32 * 4.0, 0.0, j as f32 * 4.0),
                    ..default()
                },
                scene: gltf.scenes[0].clone(),
                ..default()
            });
        }
    }
}

fn play_load_sound(
    mut commands: Commands,
    sound: Res<LoadSound>,
    mut event_reader: EventReader<ModelLoaded>,
) {
    if !event_reader.is_empty() {
        event_reader.clear();
        commands.spawn(AudioBundle {
            source: sound.0.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

fn rotate_light(time: Res<Time>, mut light_query: Query<&mut Transform, With<Light>>) {
    let mut light_transform = light_query.single_mut();
    light_transform.rotate_axis(Vec3::Y, time.delta_seconds() * PI);
}