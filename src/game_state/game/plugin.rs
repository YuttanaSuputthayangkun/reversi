use crate::game_state::util::*;
use bevy::prelude::*;

use super::*;

#[derive(Clone)]
pub struct GamePlugin {
    pub first_turn: data::Turn,
    pub board_settings: data::BoardSettings,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(resource::BoardSettings(self.board_settings.clone()))
            .insert_resource::<resource::GameData>(
                data::GameData::new(
                    self.first_turn,
                    self.board_settings.board_size_x,
                    self.board_settings.board_size_y,
                )
                .into(),
            )
            .add_event::<event::CellClick>()
            .add_event::<event::TurnChange>()
            .add_systems(OnEnter(GameState::Game), system::spawn_board_ui.chain())
            .add_systems(
                OnExit(GameState::Game),
                despawn_entities_and_clear_resource::<resource::Entities>,
            )
            .add_systems(
                Update,
                (
                    system::set_initial_player_cells, // todo: find out how to not run this in update loop
                    system::update_player_cell_color,
                    system::button_interaction_system,
                    (
                        system::turn_cells,
                        system::update_cell_clickable,
                        // util::send_default_event::<event::TurnChange>,  // todo: find a way to pipe this with turn_cells
                    )
                        .chain()
                        .run_if(on_event::<event::CellClick>()),
                    system::update_turn.run_if(on_event::<event::TurnChange>()),
                )
                    .chain()
                    .run_if(in_state(GameState::Game)),
            );
    }
}
