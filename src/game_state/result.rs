use bevy::prelude::*;

pub use plugin::ResultPlugin;

use super::GameState;

mod plugin {
    use super::*;

    pub struct ResultPlugin;

    impl Plugin for ResultPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_event::<event::ResultEvent>()
                .add_systems(OnEnter(GameState::Result), system::show_result_screen)
                .add_systems(
                    Update,
                    system::proceed_button_click.run_if(in_state(GameState::Result)),
                )
                .add_systems(OnExit(GameState::Result), system::clear_result_screen);
        }
    }
}

mod event {
    use super::*;

    #[derive(Clone, Copy)]
    pub struct ResultData {
        // add score here
    }

    #[derive(Event)]
    pub struct ResultEvent(ResultData);
}

mod system {
    use super::*;

    pub fn show_result_screen(mut event_reader: EventReader<event::ResultEvent>) {
        for _event in event_reader.iter() {
            // setup here
        }
    }

    pub fn clear_result_screen() {}

    pub fn proceed_button_click() {}
}
