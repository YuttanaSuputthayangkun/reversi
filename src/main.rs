#[allow(dead_code)]
mod board;
mod game_state;

#[cfg(test)]
mod bevy_test;

#[cfg(test)]
mod iterator_test;

use bevy::prelude::*;
use game_state::plugin::GameStatePlugin;

const GAME_TITLE: &str = "Reversi";
const WINDOW_RESOLUTION_X: f32 = 1280.;
const WINDOW_RESOLUTION_Y: f32 = 720.;
const GAME_SETTINGS_PATH: &str = "game_settings.json";

fn main() {
    setup_game();
}

#[derive(Debug)]
enum LoadPluginError {
    IO(std::io::Error),
    Read(serde_json::Error),
}

impl From<std::io::Error> for LoadPluginError {
    fn from(value: std::io::Error) -> Self {
        LoadPluginError::IO(value)
    }
}

impl From<serde_json::Error> for LoadPluginError {
    fn from(value: serde_json::Error) -> Self {
        LoadPluginError::Read(value)
    }
}

fn load_game_state_plugin() -> Result<GameStatePlugin, LoadPluginError> {
    let file = std::fs::File::open(GAME_SETTINGS_PATH)?;
    let reader = std::io::BufReader::new(file);
    let game_state_plugin = serde_json::from_reader(reader)?;
    Ok(game_state_plugin)
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
        .add_plugins(load_game_state_plugin().unwrap())
        .run();
}
