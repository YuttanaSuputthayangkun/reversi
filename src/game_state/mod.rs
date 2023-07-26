use bevy::prelude::{Plugin, States};

#[allow(dead_code)]
pub mod util;

mod game;
mod result;

#[cfg(feature = "debug")]
mod debug;

use game::GamePlugin;

pub mod plugin {
    pub use super::game::BoardSettings;
    pub use super::game::GamePlugin;
    pub use super::GameStatePlugin;
}

pub mod data {
    pub use super::game::Player;
}

pub struct GameStatePlugin {
    pub game_plugin: GamePlugin,
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameState>()
            .add_plugins(self.game_plugin.clone())
            .add_plugins(result::ResultPlugin);

        #[cfg(feature = "debug")]
        debug::add_debug(app);
    }
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum GameState {
    #[default]
    Game,
    Result,
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
