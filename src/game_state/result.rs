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
                    First,
                    system::show_result_screen
                        .pipe(system_adapter::error)
                        .run_if(
                            in_state(GameState::Result).and_then(on_event::<event::ResultEvent>()),
                        ),
                )
                .add_systems(
                    Update,
                    (
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
    use bevy::{prelude::Color, ui::BackgroundColor, utils::HashMap};

    pub(super) const FONT_SIZE: f32 = 20.;
    pub(super) const TEXT_COLOR: Color = Color::GREEN;

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
        pub color_background: BackgroundColor,
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
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: settings.color_background,
                ..default()
            });
            entities.push(parent.id());

            let font = asset_server.load::<Font, _>("fonts/NotoSans-Regular.ttf");

            let mut content = parent.with_children(|builder| {
                builder.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column, //check if this works
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                });
            });

            // add scores
            const SCORE_MARGIN: UiRect = UiRect {
                left: Val::Px(30.),
                right: Val::Px(30.),
                top: Val::Px(30.),
                bottom: Val::Px(30.),
            };
            for (player, score) in event.scores.iter() {
                content.with_children(|builder| {
                    builder.spawn(
                        TextBundle::from_section(
                            format!("{} : {}", player.deref(), score.deref()),
                            TextStyle {
                                font: font.clone(),
                                font_size: data::FONT_SIZE,
                                color: data::TEXT_COLOR,
                            },
                        )
                        .with_style(Style {
                            margin: SCORE_MARGIN,
                            ..default()
                        }),
                    );
                });
            }

            const BUTTON_BACKGROUND_COLOR: BackgroundColor = BackgroundColor(Color::ORANGE);
            content.with_children(|builder| {
                builder
                    .spawn(ButtonBundle {
                        background_color: BUTTON_BACKGROUND_COLOR,
                        style: Style {
                            width: Val::Px(100.), // add const
                            height: Val::Px(30.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(component::ButtonType(data::ButtonType::Proceed));
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
            .find(|(ref interaction, _)| interaction.deref().eq(&Interaction::Pressed));
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
