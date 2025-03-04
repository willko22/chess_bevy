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
        .insert_resource(bevy::winit::WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Reactive {
                wait: std::time::Duration::from_secs_f64(1.0 / TARGET_FPS as f64),
                react_to_device_events: true,
                react_to_user_events: true,
                react_to_window_events: true,
            },
            unfocused_mode: bevy::winit::UpdateMode::Reactive {
                wait: std::time::Duration::from_secs_f64(1.0 / TARGET_FPS as f64 / 10.0),
                react_to_device_events: true,
                react_to_user_events: true,
                react_to_window_events: true,
            },
        })
       
        
        .run();
}

