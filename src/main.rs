pub mod config;
pub mod pieces;
pub mod board;

use bevy::prelude::*;

use config::*;
use board::*;
use pieces::*;

// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// enum ChessSystemSet {
//     BoardSetup,
//     PieceSpawn,
// }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: WINDOW_TITLE.into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                decorations: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (
            create_camera, 
            create_board, 
            create_pieces
        ).chain())
        .add_systems(Update, keyboard_input)
        .run();
}

#[derive(Component)]
pub struct Camera;

fn create_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 5.0),
        Camera,
    ));
}

pub fn keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    board_query: Query<&mut Transform, With<ChessBoard>>
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        // reset the board to its original size and position
        reset_board_scale(board_query);
    } else if keyboard.just_pressed(KeyCode::Equal) && keyboard.just_pressed(KeyCode::ShiftLeft) ||
       keyboard.just_pressed(KeyCode::Equal) && keyboard.just_pressed(KeyCode::ShiftRight) ||
       keyboard.pressed(KeyCode::NumpadAdd) {

        scale_board(board_query, BOARD_SCALE_CHANGE, BOARD_SCALE_MAX, true);
        
    }  else if keyboard.just_pressed(KeyCode::Minus) ||
       keyboard.pressed(KeyCode::NumpadSubtract) {

        scale_board(board_query,-BOARD_SCALE_CHANGE, BOARD_SCALE_MIN, false);

    }

    if keyboard.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}
