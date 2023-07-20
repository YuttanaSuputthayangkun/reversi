use bevy::prelude::*;

use super::GameState;

pub struct ResultPlugin;

#[derive(Clone, Copy)]
pub struct ResultData {
    // add score here
}

#[derive(Event)]
pub struct ResultEvent(ResultData);

impl Plugin for ResultPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ResultEvent>()
            .add_systems(OnEnter(GameState::Result), show_result_screen)
            .add_systems(
                Update,
                proceed_button_click.run_if(in_state(GameState::Result)),
            )
            .add_systems(OnExit(GameState::Result), clear_result_screen);
    }
}

fn show_result_screen(mut event_reader: EventReader<ResultEvent>) {
    for _event in event_reader.iter() {
        // setup here
    }
}

fn clear_result_screen() {}

fn proceed_button_click() {}
