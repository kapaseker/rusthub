use bevy::math::vec3;
use bevy::prelude::*;
use rand::Rng;

const REGION_BOUNDARY: f32 = 100.0;

#[derive(Event)]
struct SpawnPlanetEvent;

#[derive(Resource)]
struct Planets {
    resources: Vec<String>,
}

#[derive(Component)]
struct Gravity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Birth(f32);

#[derive(Component)]
struct Planet;

fn main() {
    App::new().add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (1200.0, 1200.0).into(),
            title: "Shoot".to_string(),
            ..default()
        }),
        ..default()
    })).add_systems(Startup, set_up)
        .add_systems(Update, (bevy::window::close_on_esc, process_spawn_input, spawn_planet, process_planet_fall))
        .add_systems(Last, out_of_window_destroy)
        .add_event::<SpawnPlanetEvent>()
        .insert_resource(Planets {
            resources: vec![
                "images/Planets/Terran.png".into(),
                "images/Planets/Baren.png".into(),
                "images/Planets/Black_hole.png".into(),
                "images/Planets/Ice.png".into(),
                "images/Planets/Lava.png".into(),
            ]
        })
        .run();
}

fn out_of_window_destroy(mut command: Commands, window: Query<&Window>,
                         planets: Query<(Entity, &Transform), With<Planet>>,
) {
    if let Ok(window) = window.get_single() {
        let boundary_horizontal = window.resolution.width() / 2.0 - REGION_BOUNDARY;
        let boundary_vertical = window.resolution.height() / 2.0 - REGION_BOUNDARY;

        for (entity, transform) in &planets {
            let out_boundary = transform.translation.x < -boundary_horizontal || transform.translation.x > boundary_horizontal || transform.translation.y < -boundary_vertical || transform.translation.y > boundary_vertical;
            if out_boundary
            {
                command.entity(entity).despawn();
            }
        }
    }
}

fn process_spawn_input(
    mut event_writer: EventWriter<SpawnPlanetEvent>,
    keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        event_writer.send(SpawnPlanetEvent);
        println!("trigger space");
    }
}

fn spawn_planet(
    time: Res<Time>,
    mut event: EventReader<SpawnPlanetEvent>,
    mut command: Commands,
    window: Query<&Window>,
    asset_server: Res<AssetServer>,
    planets: Res<Planets>,
) {
    if !event.is_empty() {
        event.clear();
        println!("spawn planet");
        let window = window.single();
        let boundary_horizontal = window.resolution.width() / 2.0 - REGION_BOUNDARY;
        let boundary_vertical = window.resolution.height() / 2.0 - REGION_BOUNDARY;

        let mut rng = rand::thread_rng();
        let res: String = planets.resources[rng.gen::<usize>() % planets.resources.len()].clone();
        command.spawn((SpriteBundle {
            texture: asset_server.load(res),
            transform: Transform::from_translation(vec3(-boundary_horizontal, boundary_vertical, 0.0)),
            ..default()
        }, Gravity { x: 0.2, y: -0.98 }, Birth(time.elapsed_seconds()), Planet));
    }
}

fn process_planet_fall(
    time: Res<Time>,
    mut planets: Query<(&mut Transform, &Gravity, &Birth), With<Planet>>,
) {
    for x in planets.iter_mut() {
        let elapse_time = time.elapsed_seconds() - x.2.0;
        let mut motion = x.0;
        let gravity = x.1;
        motion.translation.x += gravity.x * elapse_time * elapse_time;
        motion.translation.y += gravity.y * elapse_time * elapse_time;
    }
}

fn set_up(mut command: Commands) {
    command.spawn(Camera2dBundle { ..default() });
}