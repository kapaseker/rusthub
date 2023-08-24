use bevy::prelude::*;
use bevy_mod_picking::*;
use bevy_mod_picking::prelude::{RaycastPickCamera, RaycastPickTarget};
use crate::constant::COUNT_CHESS_BLOCK;

#[derive(Default, Component)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

#[derive(Default, Resource)]
pub struct SelectedSquare {
    entity: Option<Entity>,
}

#[derive(Default, Resource)]
struct SquareMaterials {
    highlight_color: Handle<StandardMaterial>,
    selected_color: Handle<StandardMaterial>,
    black_color: Handle<StandardMaterial>,
    white_color: Handle<StandardMaterial>,
}

fn color_squares(
    selected_square: Res<SelectedSquare>,
    materials: Res<SquareMaterials>,
    mut query: Query<(Entity, &Square, &mut Handle<StandardMaterial>)>,
    picking_camera_query: Query<&RaycastPickCamera>,
) {
    let top_entity = match picking_camera_query.iter().last() {
        Some(pick_camera) => {
            Some(pick_camera.clone())
        }
        None => { None }
    };

    for (entity, square, mut material) in query.iter_mut() {
        *material = if Some(entity) == top_entity {
            materials.highlight_color.clone()
        } else if Some(entity) == selected_square.entity {
            materials.selected_color.clone()
        } else if square.is_white() {
            materials.white_color.clone()
        } else {
            materials.black_color.clone()
        }
    }
}

pub fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 2.0, ..default() }));
    let white_block = materials.add(Color::rgb(1.0, 0.9, 0.9).into());
    let black_block = materials.add(Color::rgb(0.0, 0.1, 0.1).into());

    for i in 0..COUNT_CHESS_BLOCK {
        for j in 0..COUNT_CHESS_BLOCK {
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: if (i + j + 1) % 2 == 0 {
                        white_block.clone()
                    } else {
                        black_block.clone()
                    },
                    transform: Transform::from_translation(Vec3::new((i * 2) as f32, 0.0, (j * 2) as f32)),
                    ..default()
                },
                PickableBundle::default(),
                RaycastPickTarget::default(),
                Square {
                    x: i,
                    y: j,
                }
            ));
        }
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedSquare>()
            .add_systems(Startup, create_board)
            .add_systems(Update, color_squares);
    }
}