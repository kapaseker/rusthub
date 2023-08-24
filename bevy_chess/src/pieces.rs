use bevy::prelude::*;
use crate::constant::COUNT_CHESS_BLOCK;

fn spawn_pieces(
    commands: &mut Commands,
    material: &Handle<StandardMaterial>,
    position: Vec3,
    offset: f32,
    children: Vec<&Handle<Mesh>>,
) {
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(position),
        ..default()
    }).with_children(|parent| {
        for child in children {
            parent.spawn(PbrBundle {
                mesh: child.clone(),
                material: material.clone(),
                transform: {
                    let mut transforms = Transform::from_translation(Vec3::new(-0.4, 0.0, offset));
                    transforms.scale *= Vec3::new(0.4, 0.4, 0.4);
                    transforms
                },
                ..default()
            });
        }
    });
}

pub fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let king_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0");
    let pawn_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh2/Primitive0");
    let knight_base_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh3/Primitive0");
    let knight_horse_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh4/Primitive0");
    let rook_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> = asset_server.load("models/chess_kit/pieces.glb#Mesh7/Primitive0");

    let white_block = materials.add(Color::rgb(1.0, 0.8, 0.8).into());
    let black_block = materials.add(Color::rgb(0.0, 0.2, 0.2).into());

    spawn_pieces(&mut commands, &white_block, Vec3::new(0.0, 0.0, 0.0), 3.6, vec![&rook_handle]);
    spawn_pieces(&mut commands, &white_block, Vec3::new(0.0, 0.0, 2.0), 1.8, vec![&knight_base_handle, &knight_horse_handle]);
    spawn_pieces(&mut commands, &white_block, Vec3::new(0.0, 0.0, 4.0), 0.0, vec![&bishop_handle]);
    spawn_pieces(&mut commands, &white_block, Vec3::new(0.0, 0.0, 6.0), -1.9, vec![&queen_handle]);
    spawn_pieces(&mut commands, &white_block, Vec3::new(0.0, 0.0, 8.0), -3.8, vec![&king_handle, &king_cross_handle]);
    spawn_pieces(&mut commands, &white_block, Vec3::new(0.0, 0.0, 10.0), 0.0, vec![&bishop_handle]);
    spawn_pieces(&mut commands, &white_block, Vec3::new(0.0, 0.0, 12.0), 1.8, vec![&knight_base_handle, &knight_horse_handle]);
    spawn_pieces(&mut commands, &white_block, Vec3::new(0.0, 0.0, 14.0), 3.6, vec![&rook_handle]);
    for i in 0..COUNT_CHESS_BLOCK {
        spawn_pieces(&mut commands, &white_block, Vec3::new(2.0, 0.0, (i as f32) * 2.0), 5.2, vec![&pawn_handle]);
    }

    spawn_pieces(&mut commands, &black_block, Vec3::new(14.0, 0.0, 0.0), 3.6, vec![&rook_handle]);
    spawn_pieces(&mut commands, &black_block, Vec3::new(14.0, 0.0, 2.0), 1.8, vec![&knight_base_handle, &knight_horse_handle]);
    spawn_pieces(&mut commands, &black_block, Vec3::new(14.0, 0.0, 4.0), 0.0, vec![&bishop_handle]);
    spawn_pieces(&mut commands, &black_block, Vec3::new(14.0, 0.0, 6.0), -1.9, vec![&queen_handle]);
    spawn_pieces(&mut commands, &black_block, Vec3::new(14.0, 0.0, 8.0), -3.8, vec![&king_handle, &king_cross_handle]);
    spawn_pieces(&mut commands, &black_block, Vec3::new(14.0, 0.0, 10.0), 0.0, vec![&bishop_handle]);
    spawn_pieces(&mut commands, &black_block, Vec3::new(14.0, 0.0, 12.0), 1.8, vec![&knight_base_handle, &knight_horse_handle]);
    spawn_pieces(&mut commands, &black_block, Vec3::new(14.0, 0.0, 14.0), 3.6, vec![&rook_handle]);
    for i in 0..COUNT_CHESS_BLOCK {
        spawn_pieces(&mut commands, &black_block, Vec3::new(12.0, 0.0, (i as f32) * 2.0), 5.2, vec![&pawn_handle]);
    }
}