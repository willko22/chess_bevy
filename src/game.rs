use std::process::exit;

use bevy::prelude::*;

use crate::resource_allocation::*;
use crate::board::*;
use crate::pieces::*;


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
    mut board_query: Query<(&mut ChessBoard, &Transform), (With<ChessBoard>, Without<Piece>)>,
    mut pieces_query: Query<&mut Transform, (With<Piece>, Without<ChessBoard>)>,
    mut game_state: ResMut<GameState>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = window.single();
        let mouse_position = window.cursor_position().unwrap();
        let mouse_position = Vec2::new(mouse_position.x as f32, mouse_position.y as f32);

        // Get board information from first query in ParamSet
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

        // Handle piece movement if a piece is already selected
        if let Some(selected_piece) = game_state.selected_piece {
            // Move piece using the third query in ParamSet
            if let Ok(mut transform) = pieces_query.get_mut(selected_piece.0) {
                // Calculate new position based on clicked tile
                let new_x = board_position.x + (tile_x as f32 * tile_size) + (tile_size / 2.0);
                let new_y = board_position.y + (tile_y as f32 * tile_size) + (tile_size / 2.0);
                transform.translation.x = new_x;
                transform.translation.y = new_y;

                // Update board state
                let (old_y, old_x) = selected_piece.1;
                board_component.pieces[old_y][old_x] = None;
                board_component.pieces[tile_y][tile_x] = Some(selected_piece.0);
                game_state.selected_piece = None;

            }
            game_state.selected_piece = None;

            
        } else {
            if let Some(piece_entity) = board_component.pieces[tile_y][tile_x] {
                game_state.selected_piece = Some((piece_entity, (tile_y, tile_x)));
            } else {
                game_state.selected_piece = None;
            }    

        }
        // Select new piece if one exists at clicked position
        

        // println!("Mouse position on board: row = {:?}, col = {:?}", tile_y, tile_x);
        // println!("Selected piece: {:?}", game_state.selected_piece);
        return;
    }
}