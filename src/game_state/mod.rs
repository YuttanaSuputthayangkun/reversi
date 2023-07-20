use bevy::prelude::{Plugin, States};

mod game;
mod result;

use game::GamePlugin;

pub mod plugin {
    pub use super::GameStatePlugin;
    use super::*;
    pub use game::GamePlugin;
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
        debug::add_plugin(app);
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

#[cfg(feature = "debug")]
mod debug {
    use super::*;
    use bevy::prelude::*;

    const DEBUG_KEYCODE: KeyCode = KeyCode::P;

    #[derive(Default)]
    struct DebugGameState;

    impl DebugGameState {
        fn next(&self, game_state: &GameState) -> GameState {
            match game_state {
                GameState::Game => GameState::Result,
                GameState::Result => GameState::Game,
            }
        }
    }

    pub(super) fn add_plugin(app: &mut App) {
        app.add_systems(Startup, || info!("Debug mode enabled."))
            .add_systems(Update, debug::next_debug_state_on_keyboard_press);
    }

    fn next_debug_state_on_keyboard_press(
        keyboard_input: Res<Input<KeyCode>>,
        current_game_state: Res<State<GameState>>,
        mut next_game_state: ResMut<NextState<GameState>>,
        debug_game_state: Local<DebugGameState>,
    ) {
        if keyboard_input.just_pressed(DEBUG_KEYCODE) {
            info!("debug keypress detected");
            let next_debug_state = debug_game_state.next(current_game_state.get());
            next_game_state.set(next_debug_state);
        }
    }
}
