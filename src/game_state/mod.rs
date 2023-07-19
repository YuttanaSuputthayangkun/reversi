use bevy::prelude::{Plugin, States, SystemSet};

mod game;
mod result;

use game::GamePlugin;

pub mod plugin {
    pub use super::game::GamePlugin;
    pub use super::GameStatePlugin;
}

pub struct GameStatePlugin {
    pub game_plugin: GamePlugin,
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameState>()
            .add_plugins(self.game_plugin.clone());
    }
}

#[derive(SystemSet, States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum GameState {
    #[default]
    Game,
    Result,
}

impl GameState {
    fn next(&self) -> Self {
        match self {
            GameState::Game => GameState::Result,
            GameState::Result => GameState::Game,
        }
    }
}

fn position_pairs<Size: Into<usize> + Copy>(
    board_size_x: Size,
    board_size_y: Size,
) -> Vec<(usize, usize)> {
    (0..board_size_y.into())
        .map(|y| (0..board_size_x.into()).map(move |x| (x, y)))
        .flat_map(|x| x)
        .collect()
}
