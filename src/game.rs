use std::process::exit;

use bevy::prelude::*;

use crate::resource_allocation::*;
use crate::board::*;
use crate::pieces::*;

use std::time::Instant;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_systems(PreStartup, resource_allocation)
            .add_systems(Startup, (create_camera, create_board, create_pieces).chain())
            .add_systems(Update, (keyboard_input, mouse_input));
    }
}


#[derive(Component)]
pub struct Camera;

pub fn create_camera(
    mut commands: Commands,
    window_settings: Res<WindowSettings>
) {
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(window_settings.window_width / 2.0, window_settings.window_height / 2.0, 5.0),
        Camera,
    ));
}

pub fn keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut board_query: Query<&mut Transform, With<ChessBoard>>,
    board_settings: Res<BoardSettings>
) {
    let board_transform = board_query.single_mut();
    if keyboard.just_pressed(KeyCode::KeyR) {
        // reset the board to its original size and position
        reset_board_scale(board_transform);
    } else if keyboard.just_pressed(KeyCode::Equal) && keyboard.just_pressed(KeyCode::ShiftLeft) ||
       keyboard.just_pressed(KeyCode::Equal) && keyboard.just_pressed(KeyCode::ShiftRight) ||
       keyboard.just_pressed(KeyCode::NumpadAdd) {

        scale_board(board_transform, board_settings.scale_change, board_settings.max_scale, false);
        
    }  else if keyboard.just_pressed(KeyCode::Minus) ||
       keyboard.just_pressed(KeyCode::NumpadSubtract) {

        scale_board(board_transform,-board_settings.scale_change, board_settings.min_scale, true);

    }

    if keyboard.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

}

pub fn mouse_input(
    window: Query<&Window>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands, // Add commands for deferred operations
    mut board_query: Query<(&mut ChessBoard, &Transform), (With<ChessBoard>, Without<Piece>)>,
    mut pieces_query: Query<(&mut Transform, &mut Piece, &mut Visibility), With<Piece>>, // Only query for read access initially
    mut game_state: ResMut<GameState>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }
    let start = Instant::now();

    let window = window.single();
    let Some(mouse_position) = window.cursor_position() else { return };
    let mouse_position = Vec2::new(mouse_position.x as f32, mouse_position.y as f32);

    // Get board information
    let (mut board_component, board_transform) = board_query.single_mut();
    let board_size = board_component.current_size;
    let board_position = Vec2::new(board_transform.translation.x, board_transform.translation.y);

    // Calculate tile position
    let tile_size = board_size / 8.0;
    let mouse_pos_board = mouse_position - board_position;

    // Check if click is within board bounds
    if mouse_pos_board.x < 0.0 || 
       mouse_pos_board.y < 0.0 || 
       mouse_pos_board.x > board_size || 
       mouse_pos_board.y > board_size {
        game_state.selected_piece = None;
        return;
    }

    let tile_x: usize = (mouse_pos_board.x / tile_size).floor() as usize;
    let tile_y: usize = 7 - (mouse_pos_board.y / tile_size).floor() as usize;

    if let Some(selected_piece) = game_state.selected_piece {
        process_piece_movement(
            &mut commands,
            &mut board_component,
            &mut pieces_query,
            selected_piece,
            (tile_y, tile_x),
            board_position,
            tile_size,
        );
        game_state.selected_piece = None;
    } else if let Some(piece_entity) = board_component.pieces[tile_y][tile_x] {
        game_state.selected_piece = Some((piece_entity, (tile_y, tile_x)));
    }

    println!("Time taken: {:?}", start.elapsed());
}

fn process_piece_movement(
    commands: &mut Commands,
    board_component: &mut ChessBoard,
    pieces_query: &mut Query<(&mut Transform, &mut Piece, &mut Visibility), With<Piece>>,
    selected_piece: (Entity, (usize, usize)),
    new_pos: (usize, usize),
    board_position: Vec2,
    tile_size: f32,
) {
    let (selected_entity, old_pos) = selected_piece;
    let (tile_y, tile_x) = new_pos;
    
    // First, validate the move without mutable borrows
    let can_move = if let Ok((_, piece, _)) = pieces_query.get(selected_entity) {
        let dst_entity = &board_component.pieces[tile_y][tile_x];
        is_valid_move(&piece, old_pos, (tile_y, tile_x), dst_entity)
    } else {
        false
    };
    
    if !can_move {
        return;
    }
    
    // Handle capture if needed
    let dst_entity = board_component.pieces[tile_y][tile_x];
    if let Some(capture_entity) = dst_entity {
        if let Ok((_, mut capture_piece, mut capture_visibility)) = pieces_query.get_mut(capture_entity) {
            capture_piece.is_captured = true;
            *capture_visibility = Visibility::Hidden;
        }
    }
    
    // Now update the moving piece
    if let Ok((mut transform, mut piece, _)) = pieces_query.get_mut(selected_entity) {
        // Calculate new position
        let new_x = board_position.x + (tile_x as f32 * tile_size) + (tile_size / 2.0);
        let new_y = board_position.y + (tile_y as f32 * tile_size) + (tile_size / 2.0);
        
        // Update the transform and piece directly
        transform.translation = Vec3::new(new_x, new_y, 0.0);
        piece.has_moved = true;
        piece.position = (tile_y, tile_x);
        
        // Update the board data structure
        let (old_y, old_x) = old_pos;
        board_component.pieces[old_y][old_x] = None;
        board_component.pieces[tile_y][tile_x] = Some(selected_entity);
    }
}

pub fn is_valid_move(
    piece: &Piece,
    old_pos: (usize, usize),
    new_pos: (usize, usize),
    dst_entity: &Option<Entity>,
) -> bool {
    //// if the piece is not moved, return false
    if old_pos == new_pos {
        return false;
    }


    //// if piece is pawn
    if piece.piece_type == PieceType::Pawn {
        let (old_y, old_x) = old_pos;
        let (new_y, new_x) = new_pos;
        let y_diff = new_y as i32 - old_y as i32;
        let x_diff = new_x as i32 - old_x as i32;
        
        //// white pawn
        println!("Checking move for piece at {:?} from {:?} to {:?}", piece.piece_color, old_pos, new_pos);
        if piece.piece_color == PieceColor::White {
            return match (y_diff, x_diff, dst_entity.is_some(), piece.has_moved) {
                (1, 0, false, _) => true,  // Forward one step (empty square)
                (2, 0, false, false) => true,  // Forward two steps (empty square, first move)
                (1, x, true, _) if x.abs() == 1 => true,  // Capture diagonally
                _ => false
            };
        }
        //// black pawn
        else {
            return match (y_diff, x_diff, dst_entity.is_some(), piece.has_moved) {
                (-1, 0, false, _) => true,  // Forward one step (empty square)
                (-2, 0, false, false) => true,  // Forward two steps (empty square, first move)
                (-1, x, true, _) if x.abs() == 1 => true,  // Capture diagonally
                _ => false
            };
        }
    }
    //// if piece is rook
    //// if piece is knight
    //// if piece is bishop
    //// if piece is queen
    //// if piece is king

    // println!("Checking move for piece at {:?} from {:?} to {:?}", piece_entity, old_pos, new_pos);
    // true
    false
}