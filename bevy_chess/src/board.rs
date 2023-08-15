use bevy::prelude::*;
use bevy_mod_picking::*;
use bevy_mod_picking::prelude::RaycastPickTarget;

static COUNT_CHESS_BLOCK: u8 = 8;

pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

#[derive(Default)]
pub struct SelectedSquare {
    entity: Option<Entity>,
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