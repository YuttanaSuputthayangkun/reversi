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
                    self.board_settings.board_size_x(),
                    self.board_settings.board_size_y(),
                )
                .into(),
            )
            .add_event::<event::PlayerCellChanged>()
            .add_event::<event::CellClick>()
            .add_event::<event::TurnChange>()
            .add_event::<event::TurnStuck>()
            .add_systems(OnEnter(GameState::Game), system::spawn_board_ui.chain())
            .add_systems(
                OnExit(GameState::Game),
                despawn_entities_and_clear_resource::<resource::Entities>,
            )
            .add_systems(
                Update,
                (
                    system::set_initial_player_cells, // todo: find out how to not run this in update loop
                    system::button_interaction_system, // send event::CellClick
                    (
                        system::change_clicked_player_cell,
                        system::change_opposite_player_cells,
                    )
                        .chain()
                        .run_if(
                            on_event::<event::CellClick>()
                                .or_else(resource_added::<resource::BoardCellEntities>()), // just for after init
                        ),
                    util::send_default_event::<event::TurnChange>
                        .run_if(on_event::<event::CellClick>()),
                    util::send_default_event::<event::TurnStuck>.run_if(system::is_turn_stuck),
                    system::update_turn.run_if(
                        on_event::<event::TurnChange>().or_else(on_event::<event::TurnStuck>()),
                    ),
                    (system::clear_cell_clickable, system::update_cell_clickable)
                        .chain()
                        .run_if(
                            on_event::<event::TurnChange>()
                                .or_else(resource_added::<resource::BoardCellEntities>()), // just for after init
                        ),
                    system::change_cell_color,
                    system::check_win_condition.run_if(on_event::<event::TurnStuck>()),
                )
                    .chain()
                    .run_if(in_state(GameState::Game)),
            );
    }
}
