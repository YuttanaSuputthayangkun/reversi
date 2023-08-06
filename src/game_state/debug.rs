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

pub(super) fn add_debug(app: &mut App) {
    app.add_systems(Startup, || info!("Debug mode enabled."))
        .add_systems(Last, debug::next_debug_state_on_keyboard_press);
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
