use bevy::prelude::{Plugin, States};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
mod util;

mod game;
mod result;

#[cfg(feature = "debug")]
mod debug;

use super::board;

pub mod plugin {
    pub use super::{
        game::{data::BoardSettings, plugin::GamePlugin},
        result::plugin::ResultPlugin,
        GameStatePlugin,
    };
}

pub mod data {
    pub use super::{
        game::data::{Player, Turn},
        result::data::{PlayerType as ResultPlayer, ResultData, Settings},
    };
}

pub mod event {
    pub use super::result::event::ResultEvent;
}

#[derive(Serialize, Deserialize)]
pub struct GameStatePlugin {
    pub game_plugin: game::plugin::GamePlugin,
    pub result_plugin: result::plugin::ResultPlugin,
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameState>()
            .add_plugins(self.game_plugin.clone())
            .add_plugins(self.result_plugin.clone());

        #[cfg(feature = "debug")]
        debug::add_debug(app);
    }
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    Game,
    Result,
}

fn position_pairs<Size: Into<board::PositionUnit> + Copy>(
    board_size_x: Size,
    board_size_y: Size,
) -> Vec<(board::PositionUnit, board::PositionUnit)> {
    (0..board_size_y.into())
        .flat_map(|y| (0..board_size_x.into()).map(move |x| (x, y)))
        .collect()
}
