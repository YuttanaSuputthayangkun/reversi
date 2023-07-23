use bevy::prelude::*;

pub use data::{PlayerType, ResultData};
pub use event::ResultEvent;
pub use plugin::ResultPlugin;

use super::util::*;
use super::GameState;

mod plugin {
    use crate::game_state::util::despawn_entities_and_clear_resource;

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
                .add_systems(
                    OnExit(GameState::Result),
                    despawn_entities_and_clear_resource::<resource::Entities>,
                );
        }
    }
}

mod data {
    use bevy::utils::HashMap;

    #[derive(Clone, Copy)]
    pub enum PlayerType {
        Black,
        White,
    }

    #[derive(Clone)]
    pub struct ResultData {
        scores: HashMap<PlayerType, u16>,
    }
}

mod resource {

    #[derive(Default)]
    pub struct ResultEntities;

    pub type Entities = super::Entities<ResultEntities>;
}

mod event {
    use super::*;

    #[derive(Event)]
    pub struct ResultEvent(ResultData);
}

mod system {
    use super::*;

    pub fn show_result_screen(
        mut commands: Commands,
        mut event_reader: EventReader<event::ResultEvent>,
    ) {
        let entity_list = resource::Entities::default();
        for _event in event_reader.iter() {
            // setup here
        }
        commands.insert_resource(entity_list);
    }

    pub fn proceed_button_click() {}
}
