use bevy::prelude::*;

use crate::config::{TILE_SIZE, PIECE_SCALE};
use crate::board::*;



pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Empty,
}

pub enum PieceColor {
    White,
    Black,
}

#[derive(Component)]
pub struct Piece {
    pub piece_type: PieceType,
    pub piece_color: PieceColor,
    pub position: (u8, u8),
}


pub fn create_pieces(
    mut commands: Commands,
    mut board_query: Query<(Entity, &mut ChessBoard), With<ChessBoard>>,
    asset_server: Res<AssetServer>
){
    if let Ok((board_entity, mut chess_board)) = board_query.get_single_mut() {

        let white_pawn: Handle<Image> = asset_server.load("pieces/w_pawn.png");
        let white_rook: Handle<Image> = asset_server.load("pieces/w_rook.png");
        let white_knight: Handle<Image> = asset_server.load("pieces/w_knight.png");
        let white_bishop: Handle<Image> = asset_server.load("pieces/w_bishop.png");
        let white_queen: Handle<Image> = asset_server.load("pieces/w_queen.png");
        let white_king: Handle<Image> = asset_server.load("pieces/w_king.png");
        

        let black_pawn: Handle<Image> = asset_server.load("pieces/b_pawn.png");
        let black_rook: Handle<Image> = asset_server.load("pieces/b_rook.png");
        let black_knight: Handle<Image> = asset_server.load("pieces/b_knight.png");
        let black_bishop: Handle<Image> = asset_server.load("pieces/b_bishop.png");
        let black_queen: Handle<Image> = asset_server.load("pieces/b_queen.png");
        let black_king: Handle<Image> = asset_server.load("pieces/b_king.png");
        

        let tile_center = TILE_SIZE / 2.0;
        let white_pos_y = tile_center;
        let white_pawn_pos_y = TILE_SIZE + (tile_center);

        let black_pos_y = 7.0 * TILE_SIZE + (tile_center);
        let black_pawn_pos_y = 6.0 * TILE_SIZE + (tile_center);

        let piece_scale = Vec3::new(PIECE_SCALE, PIECE_SCALE, 1.0);
        //// White pieces ////
        // Pawns
        {
        for i in 0..8 {
            let pawn = commands.spawn((
                Sprite {
                    image: white_pawn.clone(),
                    ..default()
                },
                Transform{
                    translation: Vec3::new(i as f32 * TILE_SIZE + (TILE_SIZE / 2.0), white_pawn_pos_y, 1.0),
                    scale: piece_scale,
                    ..default()
                },
                Piece {
                    piece_type: PieceType::Pawn,
                    piece_color: PieceColor::White,
                    position: (2, i+1),
                } 
            )).set_parent(board_entity)
            .id();
            chess_board.board[1][i as usize] = Some(pawn);
        }
        }
           
        // Rooks
        {
        let rook1 = commands.spawn((
            Sprite {
                image: white_rook.clone(),
                ..default()
            },
            Transform{
                translation: Vec3::new(white_pos_y, white_pos_y, 1.0),
                scale: piece_scale,
                ..default()
            },
            Piece {
                piece_type: PieceType::Rook,
                piece_color: PieceColor::White,
                position: (1, 1),
            } 
        )).set_parent(board_entity)
        .id();
        chess_board.board[0][0] = Some(rook1);

        let rook2 = commands.spawn((
            Sprite {
                image: white_rook.clone(),
                ..default()
            },
            Transform{
                translation: Vec3::new(7.0 * TILE_SIZE + (white_pos_y), white_pos_y, 1.0),
                scale: piece_scale,
                ..default()
            },
            Piece {
                piece_type: PieceType::Rook,
                piece_color: PieceColor::White,
                position: (1, 8),
            } 
        )).set_parent(board_entity)
        .id();
        chess_board.board[0][7] = Some(rook2);
        }

        // Knights
        {
        let knight1 = commands.spawn((
            Sprite {
                image: white_knight.clone(),
                ..default()
            },
            Transform{
                translation: Vec3::new(TILE_SIZE + white_pos_y, white_pos_y, 1.0),
                scale: piece_scale,
                ..default()
            },
            Piece {
                piece_type: PieceType::Knight,
                piece_color: PieceColor::White,
                position: (1, 2),
            } 
        )).set_parent(board_entity)
        .id();
        chess_board.board[0][1] = Some(knight1);

        let knight2 = commands.spawn((
            Sprite {
                image: white_knight.clone(),
                ..default()
            },
            Transform{
                translation: Vec3::new(6.0 * TILE_SIZE + white_pos_y, white_pos_y, 1.0),
                scale: piece_scale,
                ..default()
            },
            Piece {
                piece_type: PieceType::Knight,
                piece_color: PieceColor::White,
                position: (1, 7),
            } 
        )).set_parent(board_entity)
        .id();
        chess_board.board[0][6] = Some(knight2);
        }

        // Bishops
        {
        let bishop1 = commands.spawn((
            Sprite {
                image: white_bishop.clone(),
                ..default()
            },
            Transform{
                translation: Vec3::new(2.0 * TILE_SIZE + white_pos_y, white_pos_y, 1.0),
                scale: piece_scale,
                ..default()
            },
            Piece {
                piece_type: PieceType::Bishop,
                piece_color: PieceColor::White,
                position: (1, 3),
            } 
        )).set_parent(board_entity)
        .id();
        chess_board.board[0][2] = Some(bishop1);

        let bishop2 = commands.spawn((
            Sprite {
                image: white_bishop.clone(),
                ..default()
            },
            Transform{
                translation: Vec3::new(5.0 * TILE_SIZE + white_pos_y, white_pos_y, 1.0),
                scale: piece_scale,
                ..default()
            },
            Piece {
                piece_type: PieceType::Bishop,
                piece_color: PieceColor::White,
                position: (1, 6),
            } 
        )).set_parent(board_entity)
        .id();
        chess_board.board[0][5] = Some(bishop2);

        }
        
        // Queen
        {
        let queen = commands.spawn((
            Sprite {
                image: white_queen.clone(),
                ..default()
            },
            Transform{
                translation: Vec3::new(3.0 * TILE_SIZE + white_pos_y, white_pos_y, 1.0),
                scale: piece_scale,
                ..default()
            },
            Piece {
                piece_type: PieceType::Queen,
                piece_color: PieceColor::White,
                position: (1, 4),
            } 
        )).set_parent(board_entity)
        .id();
        chess_board.board[0][3] = Some(queen);
        }
        
        // King
        {
        let king = commands.spawn((
            Sprite {
                image: white_king.clone(),
                ..default()
            },
            Transform{
                translation: Vec3::new(4.0 * TILE_SIZE + white_pos_y, white_pos_y, 1.0),
                scale: piece_scale,
                ..default()
            },
            Piece {
                piece_type: PieceType::King,
                piece_color: PieceColor::White,
                position: (1, 5),
            } 
        )).set_parent(board_entity)
        .id();
        chess_board.board[0][4] = Some(king);
        }
        
        
        //// Black pieces ////
        // Pawns
        {     
        for i in 0..8 {
            let pawn = commands.spawn((
                Sprite {
                    image: black_pawn.clone(),
                    ..default()
                },
                Transform{
                    translation: Vec3::new(i as f32 * TILE_SIZE + (TILE_SIZE / 2.0), black_pawn_pos_y, 1.0),
                    scale: piece_scale,
                    ..default()
                },
                Piece {
                    piece_type: PieceType::Pawn,
                    piece_color: PieceColor::Black,
                    position: (7, i+1),
                } 
            )).set_parent(board_entity)
            .id();
            chess_board.board[6][i as usize] = Some(pawn);
        }
        }      

        // Rooks
        {
        let rook1 = commands.spawn((
            Sprite {
                image: black_rook.clone(),
                ..default()
            },
            Transform{
                translation: Vec3::new(tile_center, black_pos_y, 1.0),
                scale: piece_scale,
                ..default()
            },
            Piece {
                piece_type: PieceType::Rook,
                piece_color: PieceColor::Black,
                position: (7, 1),
            } 
        )).set_parent(board_entity)
        .id();
        chess_board.board[7][0] = Some(rook1);

        let rook2 = commands.spawn((
            Sprite {
                image: black_rook.clone(),
                ..default()
            },
            Transform{
                translation: Vec3::new(7.0 * TILE_SIZE + (tile_center), black_pos_y, 1.0),
                scale: piece_scale,
                ..default()
            },
            Piece {
                piece_type: PieceType::Rook,
                piece_color: PieceColor::Black,
                position: (7, 8),
            } 
        )).set_parent(board_entity)
        .id();
        chess_board.board[7][7] = Some(rook2);
        }

        // Knights
        {
        let knight1 = commands.spawn((
            Sprite {
                image: black_knight.clone(),
                ..default()
            },
            Transform{
                translation: Vec3::new(TILE_SIZE + tile_center, black_pos_y, 1.0),
                scale: piece_scale,
                ..default()
            },
            Piece {
                piece_type: PieceType::Knight,
                piece_color: PieceColor::Black,
                position: (7, 2),
            } 
        )).set_parent(board_entity)
        .id();
        chess_board.board[7][1] = Some(knight1);

        let knight2 = commands.spawn((
            Sprite {
                image: black_knight.clone(),
                ..default()
            },
            Transform{
                translation: Vec3::new(6.0 * TILE_SIZE + tile_center, black_pos_y, 1.0),
                scale: piece_scale,
                ..default()
            },
            Piece {
                piece_type: PieceType::Knight,
                piece_color: PieceColor::Black,
                position: (7, 7),
            } 
        )).set_parent(board_entity)
        .id();
        chess_board.board[7][6] = Some(knight2);
        }

        // Bishops
        {
        let bishop1 = commands.spawn((
            Sprite {
                image: black_bishop.clone(),
                ..default()
            },
            Transform{
                translation: Vec3::new(2.0 * TILE_SIZE + tile_center, black_pos_y, 1.0),
                scale: piece_scale,
                ..default()
            },
            Piece {
                piece_type: PieceType::Bishop,
                piece_color: PieceColor::Black,
                position: (7, 3),
            } 
        )).set_parent(board_entity)
        .id();
        chess_board.board[7][2] = Some(bishop1);

        let bishop2 = commands.spawn((
            Sprite {
                image: black_bishop.clone(),
                ..default()
            },
            Transform{
                translation: Vec3::new(5.0 * TILE_SIZE + tile_center, black_pos_y, 1.0),
                scale: piece_scale,
                ..default()
            },
            Piece {
                piece_type: PieceType::Bishop,
                piece_color: PieceColor::Black,
                position: (7, 6),
            } 
        )).set_parent(board_entity)
        .id();
        chess_board.board[7][5] = Some(bishop2);

        }
        
        // Queen
        {
        let queen = commands.spawn((
            Sprite {
                image: black_queen.clone(),
                ..default()
            },
            Transform{
                translation: Vec3::new(3.0 * TILE_SIZE + tile_center, black_pos_y, 1.0),
                scale: piece_scale,
                ..default()
            },
            Piece {
                piece_type: PieceType::Queen,
                piece_color: PieceColor::Black,
                position: (7, 4),
            } 
        )).set_parent(board_entity)
        .id();
        chess_board.board[7][3] = Some(queen);
        }
        
        // King
        {
        let king = commands.spawn((
            Sprite {
                image: black_king.clone(),
                ..default()
            },
            Transform{
                translation: Vec3::new(4.0 * TILE_SIZE + tile_center, black_pos_y, 1.0),
                scale: piece_scale,
                ..default()
            },
            Piece {
                piece_type: PieceType::King,
                piece_color: PieceColor::White,
                position: (7, 5),
            } 
        )).set_parent(board_entity)
        .id();
        chess_board.board[7][4] = Some(king);
        }
            



    }

}
