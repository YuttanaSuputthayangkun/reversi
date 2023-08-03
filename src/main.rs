#[allow(dead_code)]
mod board;
mod game_state;

#[cfg(test)]
mod bevy_test;

#[cfg(test)]
mod iterator_test;

use bevy::prelude::*;
use game_state::{
    data::{Player, Turn},
    plugin::{BoardSettings, GamePlugin, GameStatePlugin},
};

const GAME_TITLE: &'static str = "Reversi";
const WINDOW_RESOLUTION_X: f32 = 1280.;
const WINDOW_RESOLUTION_Y: f32 = 720.;
const FIRST_TURN: Turn = Turn::Black;
const BOARD_SIZE_X: u16 = 8;
const BOARD_SIZE_Y: u16 = 8;
const CELL_COLOR_NORMAL: Color = Color::Rgba {
    red: 30. / 256.,
    green: 128. / 256.,
    blue: 0.,
    alpha: 1.,
};
const CELL_COLOR_PLAYER_WHITE: Color = Color::WHITE;
const CELL_COLOR_PLAYER_BLACK: Color = Color::BLACK;
const CELL_COLOR_CLICKABLE: Color = Color::RED;
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
            game_plugin: GamePlugin::new(
                FIRST_TURN,
                BoardSettings::new(
                    BOARD_SIZE_X.try_into().unwrap(),
                    BOARD_SIZE_Y.try_into().unwrap(),
                    CELL_COLOR_CLICKABLE,
                    [
                        (Player::None, CELL_COLOR_NORMAL),
                        (Player::White, CELL_COLOR_PLAYER_WHITE),
                        (Player::Black, CELL_COLOR_PLAYER_BLACK),
                    ]
                    .into_iter(),
                    BACKGROUND_COLOR,
                ),
            ),
        })
        .run();
}
