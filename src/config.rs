use bevy::prelude::*;

pub const WINDOW_TITLE: &str = "Bevy Chess";

pub const WINDOW_HEIGHT: f32 = 600.0;
pub const WINDOW_WIDTH: f32 = 1000.0;

pub const TILE_SIZE: f32 = (if WINDOW_HEIGHT < WINDOW_WIDTH { WINDOW_HEIGHT } else { WINDOW_WIDTH } as i32 / 8 ) as f32;
pub const BOARD_SIZE: f32 = 8.0 * TILE_SIZE;
pub const MARGIN: f32 = (WINDOW_HEIGHT -BOARD_SIZE) / 2.0;

pub const BOARD_SCALE_CHANGE: f32 = 0.1;
pub const BOARD_SCALE_MAX: f32 = 1.0;
pub const BOARD_SCALE_MIN: f32 = 0.2;

pub const BOARD_COLOR_DARK: Color = Color::srgba(0.18, 0.15, 0.10, 1.0);
pub const BOARD_COLOR_LIGHT: Color = Color::srgba(0.85, 0.82, 0.75, 1.0);
pub const BOARD_SELECTED_TINT: Color = Color::srgba(0.5, 1.0, 0.5, 1.0);
// pub const BOARD_COLOR_SELECTED_DARK: Color = Color::srgba(0.1, 0.3, 0.1, 1.0);
// pub const BOARD_COLOR_SELECTED_LIGHT: Color = Color::srgba(0.8, 0.9, 0.7, 1.0);


const PIECE_ORIG_SIZE: f32 = 125.0;

pub const PIECE_SCALE: f32 = TILE_SIZE * 0.7 / PIECE_ORIG_SIZE;

