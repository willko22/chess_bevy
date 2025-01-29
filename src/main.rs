#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod config;
pub mod resource_allocation;
pub mod pieces;
pub mod board;
pub mod game;


use bevy::prelude::*;
use config::*;

// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// enum ChessSystemSet {
//     BoardSetup,
//     PieceSpawn,
// }

fn main() {
    App::new()
        .add_plugins(
            (DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: WINDOW_TITLE.into(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        resizable: false,
                        decorations: true,
                        ..default()
                    }),
                    ..default()
                }),
                game::GamePlugin

            )
        )
       
        
        .run();
}

