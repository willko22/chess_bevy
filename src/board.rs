use bevy::prelude::*;

use crate::config::*;



#[derive(Component)]
pub struct ChessBoard {
    pub board: [[Option<Entity>; 8]; 8],
}


pub fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create single shared mesh and materials
    let tile_mesh = meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE));
    let white_material = materials.add(BOARD_LIGHT_COLOR);
    let black_material = materials.add(BOARD_DARK_COLOR);

    // Create board parent
    let board = commands.spawn((
        Transform::default(), 
        Visibility::default(), 
        ChessBoard { board: [[None; 8]; 8] }
    )).id();

    // Generate board squares
    for row in 0..8 {
        for col in 0..8 {
            let position = Vec3::new(
                (col as f32 * TILE_SIZE) - MARGIN + (TILE_SIZE/2.0),
                (row as f32 * TILE_SIZE) - MARGIN + (TILE_SIZE/2.0),
                0.0
            );

            commands.spawn((
                Mesh2d(tile_mesh.clone()),
                MeshMaterial2d(if (1 + row + col) % 2 == 0 { white_material.clone() } else { black_material.clone() }),
                Transform::from_translation(position),
            )).set_parent(board);
        }
    }
}



pub fn scale_board(
    mut board_query: Query<&mut Transform, With<ChessBoard>>,
    change : f32,
    cieling : f32,
    min : bool
) {
    // increase the size of the board
    let mut transform = board_query.single_mut();
    let old_scale = transform.scale.x;
    let new_scale;  
    if min {
        new_scale = (old_scale + change).min(cieling);
    } else {
        new_scale = (old_scale + change).max(cieling);
    }    
    transform.scale = Vec3::splat(new_scale);

    // Calculate and apply offset to keep board centered
    center_board(board_query);
}

fn center_board(
    mut board_query: Query<&mut Transform, With<ChessBoard>>,
) {
    // center the board
    let mut transform = board_query.single_mut();
    let current_board_size = BOARD_SIZE * transform.scale.x;

    transform.translation.x = WINDOW_WIDTH / 2.0 - current_board_size / 2.0;
    transform.translation.y = WINDOW_HEIGHT / 2.0 - current_board_size / 2.0;
}

pub fn reset_board_scale(
    mut board_query: Query<&mut Transform, With<ChessBoard>>
) {
    // reset the board to its original size and position
    let mut transform = board_query.single_mut();
    transform.scale = Vec3::splat(1.0);
    transform.translation = Vec3::splat(0.0);
}
