use bevy::prelude::*;
use data::*;

use super::util;
use super::GameState;

pub mod plugin {
    use crate::game_state::util::despawn_entities_and_clear_resource;

    use super::*;

    #[derive(Clone)]
    pub struct ResultPlugin {
        pub settings: data::Settings,
    }

    impl Plugin for ResultPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.insert_resource(resource::Settings(self.settings.clone()))
                .add_event::<event::ResultEvent>()
                .add_event::<event::ButtonClicked>()
                .add_systems(
                    OnEnter(GameState::Result),
                    util::init_resource::<resource::Entities>,
                )
                .add_systems(
                    Update,
                    (
                        // show_result_screen should run only once and doesn't need to be in update schedule.
                        // But it seems, sometimes the run condition is not triggered properly.
                        // Probably because of how event is sent. further investigation required.
                        system::show_result_screen
                            .pipe(system_adapter::error)
                            .run_if(
                                in_state(GameState::Result)
                                    .and_then(on_event::<event::ResultEvent>()),
                            ),
                        system::check_button_click,
                        system::change_state.run_if(on_event::<event::ButtonClicked>()),
                    )
                        .chain()
                        .run_if(
                            in_state(GameState::Result)
                                .and_then(resource_exists::<resource::Entities>()), // check if show_result_screen has been called
                        ),
                )
                .add_systems(
                    OnExit(GameState::Result),
                    despawn_entities_and_clear_resource::<resource::Entities>,
                );

            #[cfg(feature = "debug")]
            {
                app.add_systems(
                    OnEnter(GameState::Result),
                    system::debug::try_send_debug_result_event
                        .run_if(not(on_event::<event::ResultEvent>())),
                );
            }
        }
    }
}

pub mod data {
    use bevy::{prelude::Color, utils::HashMap};

    pub(super) const FONT_SIZE: f32 = 100.;

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub enum PlayerType {
        Black,
        White,
    }

    impl std::fmt::Display for PlayerType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let str = match self {
                PlayerType::Black => "Black",
                PlayerType::White => "White",
            };
            f.write_str(str)
        }
    }

    pub type CellCount = u16;

    #[derive(Clone, Debug)]
    pub struct ResultData {
        pub scores: HashMap<PlayerType, CellCount>,
    }

    #[derive(Debug, Clone)]
    pub(super) enum ButtonType {
        Proceed,
    }

    #[derive(Debug, Clone)]
    pub struct Settings {
        pub text_color: Color,
        pub player_color_map: HashMap<PlayerType, Color>,
    }
}

mod component {
    use bevy::prelude::{Component, Deref};

    use super::data;

    #[derive(Component, Deref)]
    pub(super) struct ButtonType(pub(super) data::ButtonType);
}

mod resource {
    use bevy::prelude::{Deref, DerefMut, Resource};

    use super::{data, util};

    #[derive(Default)]
    pub struct ResultEntities;

    pub(super) type Entities = util::Entities<ResultEntities>;

    #[derive(Resource, Deref, DerefMut)]
    pub(super) struct Settings(pub data::Settings);
}

pub mod event {
    use super::*;

    #[derive(Event, Deref, DerefMut)]
    pub struct ResultEvent(pub ResultData);

    #[derive(Event, Deref, DerefMut)]
    pub(super) struct ButtonClicked(data::ButtonType);

    impl From<data::ButtonType> for ButtonClicked {
        fn from(value: data::ButtonType) -> Self {
            ButtonClicked(value)
        }
    }
}

mod system {
    use super::*;
    use std::ops::Deref;

    #[derive(Debug, Clone, Copy)]
    pub enum ShowResultScreenError {
        NoResultEvent,
    }

    pub(super) fn show_result_screen(
        mut commands: Commands,
        settings: Res<resource::Settings>,
        mut entities: ResMut<resource::Entities>,
        mut event_reader: EventReader<event::ResultEvent>,
        asset_server: Res<AssetServer>,
    ) -> Result<(), ShowResultScreenError> {
        if let Some(event) = event_reader.iter().next() {
            let camera = commands.spawn(Camera2dBundle::default()).id();
            entities.push(camera);

            // spawn ui here
            let mut parent = commands.spawn(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            });
            entities.push(parent.id());

            parent.with_children(|builder| {
                let font = asset_server.load::<Font, _>("fonts/NotoSans-Regular.ttf");
                let button_data = [
                    (data::PlayerType::Black, data::ButtonType::Proceed),
                    (data::PlayerType::White, data::ButtonType::Proceed),
                ];
                for (player_type, button_type) in button_data.into_iter() {
                    let score = event.scores.get(&player_type).unwrap();
                    let color = settings.player_color_map.get(&player_type).unwrap().clone();
                    let button_bundle = ButtonBundle {
                        background_color: color.into(),
                        style: Style {
                            display: Display::Flex,
                            flex_grow: score.clone() as f32,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    };
                    let text_bundle = TextBundle::from_section(
                        format!("{}", score).to_string(),
                        TextStyle {
                            font: font.clone(),
                            font_size: data::FONT_SIZE,
                            color: settings.text_color.into(),
                        },
                    );
                    let new_button = builder
                        .spawn(button_bundle)
                        .with_children(|x| {
                            x.spawn(text_bundle);
                        })
                        .insert(component::ButtonType(button_type))
                        .id();
                    entities.push(new_button);
                }
            });

            Ok(())
        } else {
            Err(ShowResultScreenError::NoResultEvent)
        }
    }

    pub(super) fn check_button_click(
        query: Query<(&Interaction, &component::ButtonType), With<Button>>,
        mut event_writer: EventWriter<event::ButtonClicked>,
    ) {
        let button_press = query
            .iter()
            .find(|(ref interaction, _)| (**interaction).eq(&Interaction::Pressed));
        if let Some((_, button_type)) = button_press {
            event_writer.send(button_type.deref().to_owned().into());
        }
    }

    pub(super) fn change_state(
        mut next_game_state: ResMut<NextState<GameState>>,
        mut button_click_event_reader: EventReader<event::ButtonClicked>,
    ) {
        let event = button_click_event_reader.iter().next();
        if event.is_none() {
            return;
        }

        match event.unwrap().deref() {
            data::ButtonType::Proceed => {
                next_game_state.set(GameState::Game);
            }
        }
    }

    #[cfg(feature = "debug")]
    pub mod debug {
        use super::*;

        pub fn try_send_debug_result_event(mut writer: EventWriter<event::ResultEvent>) {
            let data = data::ResultData {
                scores: [(data::PlayerType::Black, 1), (data::PlayerType::White, 2)]
                    .into_iter()
                    .collect(),
            };
            writer.send(event::ResultEvent(data));
        }
    }
}
