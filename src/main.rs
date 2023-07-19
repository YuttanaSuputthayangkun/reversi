#[allow(dead_code)]
mod board;
mod game_state;

#[cfg(test)]
mod bevy_test;

use bevy::prelude::*;
use game_state::{plugin::GamePlugin, GameStatePlugin};

const GAME_TITLE: &'static str = "Reversi";
const WINDOW_RESOLUTION_X: f32 = 1280.;
const WINDOW_RESOLUTION_Y: f32 = 720.;
const BOARD_SIZE_X: u16 = 8;
const BOARD_SIZE_Y: u16 = 8;
const CELL_COLOR: Color = Color::Rgba {
    red: 30. / 256.,
    green: 128. / 256.,
    blue: 0.,
    alpha: 1.,
};
const CELL_HOVERED_COLOR: Color = Color::Rgba {
    red: 17. / 256.,
    green: 66. / 256.,
    blue: 1.,
    alpha: 1.,
};
const BACKGROUND_COLOR: Color = Color::Rgba {
    red: 145. / 256.,
    green: 145. / 256.,
    blue: 145. / 256.,
    alpha: 1.,
};

fn main() {
    setup_game();
}

fn setup_game() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [WINDOW_RESOLUTION_X, WINDOW_RESOLUTION_Y].into(),
                title: GAME_TITLE.to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GameStatePlugin {
            game_plugin: GamePlugin {
                board_size_x: BOARD_SIZE_X,
                board_size_y: BOARD_SIZE_Y,
                cell_color: CELL_COLOR,
                cell_hovered_color: CELL_HOVERED_COLOR,
                background_color: BACKGROUND_COLOR,
            },
        })
        .run();
}
