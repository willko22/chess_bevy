use bevy::{log::tracing_subscriber::field::debug, prelude::*};

use crate::config::*;

#[derive(Resource)]
pub struct BoardSettings{
    pub board_width: f32,
    pub board_height: f32,
    pub margin: f32,
    pub board_color_light: Color,
    pub board_color_dark: Color,
    pub square_size: f32,
    pub max_scale: f32,
    pub min_scale: f32,
    pub scale_change: f32
}

#[derive(Resource)]
pub struct WindowSettings{
    pub window_title: String,
    pub window_height: f32,
    pub window_width: f32
}

#[derive(Resource)]
pub struct PieceSettings{
    pub piece_scale: f32
}

#[derive(Resource)]
pub struct GameState {
    pub selected_tile: Option<Entity>,
    pub selected_piece: Option<(Entity, (usize, usize))>, // Entity, (row, col)>,
    pub turn: bool, // true for white, false for black
}


pub fn resource_allocation(mut commands: Commands){

    commands.insert_resource(WindowSettings{
        window_title: WINDOW_TITLE.to_string(),
        window_height: WINDOW_HEIGHT,
        window_width: WINDOW_WIDTH
    });

    commands.insert_resource(BoardSettings{
        board_width: BOARD_SIZE,
        board_height: BOARD_SIZE,
        margin: MARGIN,
        board_color_light: BOARD_COLOR_LIGHT,
        board_color_dark: BOARD_COLOR_DARK,
        square_size: TILE_SIZE,
        max_scale: BOARD_SCALE_MAX,
        min_scale: BOARD_SCALE_MIN,
        scale_change: BOARD_SCALE_CHANGE
    });

    commands.insert_resource(PieceSettings{
        piece_scale: PIECE_SCALE
    });

    commands.insert_resource(GameState{
        selected_tile: None,
        selected_piece: None,
        turn: true
    });

}