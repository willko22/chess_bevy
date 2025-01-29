use bevy::prelude::*;

use crate::config::*;



#[derive(Component)]
pub struct ChessBoard {
    pub pieces: [[Option<Entity>; 8]; 8],
    pub tiles: [[Option<Entity>; 8]; 8],
    pub current_size: f32,
}

#[derive(Component)]

pub struct Tile {
    pub row: usize,
    pub col: usize,
    pub color: Color,
}

// fn tint_color(base: Color, tint: Color) -> Color {
//     let color_vec = BOARD_COLOR_DARK.as_rgba_f32();
//     Color::srgba(
//         base.r() * tint.r(),
//         base.g() * tint.g(),
//         base.b() * tint.b(),
//         base.a() * tint.a()
//     )
// }


pub fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create single shared mesh and materials
    let tile_mesh = meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE));
    let white_material = materials.add(BOARD_COLOR_LIGHT);
    let black_material = materials.add(BOARD_COLOR_DARK);
    // println!("{:?}", BOARD_COLOR_DARK);
    // materials.add(BOARD_COLOR_SELECTED_DARK);
    // materials.add(BOARD_COLOR_SELECTED_LIGHT);


    let mut tiles: [[Option<Entity>; 8]; 8] = [[None; 8]; 8];
    
    

    // Generate board squares
    for row in 0..8 {
        for col in 0..8 {
            let position = Vec3::new(
                (col as f32 * TILE_SIZE) - MARGIN + (TILE_SIZE/2.0),
                (row as f32 * TILE_SIZE) - MARGIN + (TILE_SIZE/2.0),
                0.0
            );

            let calc = (1 + row + col) % 2;

            let tile_id =commands.spawn((
                Mesh2d(tile_mesh.clone()),
                MeshMaterial2d(if calc == 0 { white_material.clone() } else { black_material.clone() }),
                Transform::from_translation(position),
                Tile { row, col, color: if calc == 0 { BOARD_COLOR_LIGHT } else { BOARD_COLOR_DARK } },
            )).id();

            tiles[row][col] = Some(tile_id);
        }
    }


    // Create board parent
    let board = commands.spawn((
        Transform::default(), 
        Visibility::default(), 
        ChessBoard { pieces: [[None; 8]; 8], tiles: tiles , current_size: TILE_SIZE * 8.0 }
    )).id();

    // set board as parent to all tiles

    for row in 0..8 {
        for col in 0..8 {
            if let Some(tile_id) = tiles[row][col] {
                commands.entity(tile_id).set_parent(board);
            }
        }
    }
}



pub fn scale_board(
    mut board_transform: Mut<Transform>,
    change : f32,
    cieling : f32,
    min : bool
) {
    // increase the size of the board
    // let (board_component, mut board_transform) = board_query.single_mut();
    // let mut board_transform = board_query.single_mut();
    let old_scale = board_transform.scale.x;
    let new_scale;  
    if min {
        new_scale = (old_scale + change).max(cieling); 
    } else {
        new_scale = (old_scale + change).min(cieling);
    }    
    board_transform.scale = Vec3::splat(new_scale);

    

    // Calculate and apply offset to keep board centered
    center_board(board_transform);
}

fn center_board(
    mut board_transform: Mut<Transform>,
) {
    // let mut transform: Mut<'_, Transform> = board_query.single_mut();

    let half_board_size: f32 = BOARD_SIZE / 2.0;
  
    // Apply offset to maintain center point
    board_transform.translation.x = half_board_size * (1.0 - board_transform.scale.x);
    board_transform.translation.y = half_board_size * (1.0 - board_transform.scale.y);
}


pub fn reset_board_scale(
    mut board_transform: Mut<Transform>
) {
    // reset the board to its original size and position
    // let mut transform = board_query.single_mut();
    board_transform.scale = Vec3::splat(1.0);
    board_transform.translation = Vec3::splat(0.0);
}
