use bevy::prelude::*;

use super::util;
use crate::board;

pub use data::BoardSettings;
pub use data::Turn;
pub use plugin::GamePlugin;

use super::util::despawn_entities_and_clear_resource;
use super::{position_pairs, GameState};

mod plugin {
    use super::*;

    #[derive(Clone)]
    pub struct GamePlugin {
        pub first_turn: Turn,
        pub board_settings: data::BoardSettings,
    }

    impl Plugin for GamePlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.insert_resource(resource::BoardSettings(self.board_settings.clone()))
                .insert_resource::<resource::GameData>(data::GameData::new(self.first_turn).into())
                .add_event::<event::CellClick>()
                .add_event::<event::TurnChange>()
                .add_systems(OnEnter(GameState::Game), system::spawn_board_ui)
                .add_systems(
                    OnExit(GameState::Game),
                    despawn_entities_and_clear_resource::<resource::Entities>,
                )
                .add_systems(
                    Update,
                    (
                        system::button_interaction_system,
                        (
                            system::turn_cells,
                            util::send_default_event::<event::TurnChange>,
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
}

mod data {
    use super::*;

    #[derive(Clone)]
    pub struct BoardSettings {
        pub board_size_x: u16,
        pub board_size_y: u16,
        pub cell_color: Color,
        pub cell_hovered_color: Color,
        pub background_color: Color,
    }

    #[derive(Debug)]
    pub struct CellData {
        pub position: board::BoardPosition,
    }

    #[derive(Clone, Copy, Debug)]
    pub enum Turn {
        Black,
        White,
    }

    impl Turn {
        pub fn next(&self) -> Self {
            use Turn::*;
            match self {
                Black => White,
                White => Black,
            }
        }

        pub fn next_mut(&mut self) {
            *self = self.next();
        }
    }

    #[derive(Debug)]
    pub struct GameData {
        pub turn: Turn,
        pub turn_count: u16,
    }

    impl GameData {
        pub fn new(first_turn: Turn) -> Self {
            GameData {
                turn: first_turn,
                turn_count: 0,
            }
        }
    }
}

mod resource {
    use super::*;

    #[derive(Resource, Clone, Deref)]
    pub struct BoardSettings(#[deref] pub data::BoardSettings);

    #[derive(Default)]
    pub struct BoardEntities;

    pub type Entities = super::util::Entities<BoardEntities>;

    #[derive(Resource, Deref, DerefMut, Debug)]
    pub struct GameData(#[deref] pub data::GameData);

    impl From<data::GameData> for GameData {
        fn from(value: data::GameData) -> Self {
            GameData(value)
        }
    }
}

mod component {
    use super::board;
    use super::*;

    #[derive(Component, Deref)]
    pub struct BoardPosition(#[deref] pub board::BoardPosition);

    #[derive(Component, Deref, DerefMut)]
    pub struct Clickable(#[deref] pub bool);
}

mod event {
    use super::*;

    #[derive(Event, Deref)]
    pub struct CellClick(#[deref] pub board::BoardPosition);

    #[derive(Event, Default)]
    pub struct TurnChange;
}

mod system {
    use std::ops::Deref;

    use super::*;

    pub fn spawn_board_ui(mut commands: Commands, board_settings: Res<resource::BoardSettings>) {
        let mut entities = resource::Entities::default();
        let camera = commands.spawn(Camera2dBundle::default()).id();
        entities.push(camera);
        let board = commands
            .spawn(NodeBundle {
                style: Style {
                    display: Display::Grid,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    grid_template_columns: vec![GridTrack::max_content()],
                    ..default()
                },
                background_color: Color::GRAY.into(),
                ..default()
            })
            .with_children(|builder| {
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            justify_content: JustifyContent::Center,
                            aspect_ratio: Some(1.0),
                            padding: UiRect::all(Val::Percent(1.)),
                            column_gap: Val::Percent(1.),
                            row_gap: Val::Percent(1.),
                            grid_template_columns: RepeatedGridTrack::flex(
                                board_settings.board_size_y,
                                1.0,
                            ),
                            grid_template_rows: RepeatedGridTrack::flex(
                                board_settings.board_size_x,
                                1.0,
                            ),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        for (x, y) in
                            position_pairs(board_settings.board_size_x, board_settings.board_size_y)
                        {
                            let pos = board::BoardPosition {
                                x: x.into(),
                                y: y.into(),
                            };
                            spawn_cell(builder, pos, &board_settings);
                        }
                    });
            })
            .id();
        entities.push(board);
        commands.insert_resource(entities);
    }

    pub fn spawn_cell(
        builder: &mut ChildBuilder,
        pos: board::BoardPosition,
        board_settings: &resource::BoardSettings,
    ) {
        builder
            .spawn(ButtonBundle {
                style: Style {
                    aspect_ratio: Some(1.0),
                    ..default()
                },
                background_color: board_settings.background_color.into(),
                ..default()
            })
            .insert(component::BoardPosition(pos))
            .insert(component::Clickable(true)); // change this to false when the clickable button system is complete
    }

    pub fn button_interaction_system(
        mut interaction_query: Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &component::BoardPosition,
                &component::Clickable,
            ),
            (
                Changed<Interaction>,
                With<Button>,
                With<component::BoardPosition>,
                With<component::Clickable>,
            ),
        >,
        board_settings: Res<resource::BoardSettings>,
        mut cell_click_event: EventWriter<event::CellClick>,
    ) {
        for (interaction, mut color, board_pos, clickable) in &mut interaction_query {
            match *interaction {
                Interaction::Hovered => {
                    *color = board_settings.cell_hovered_color.into();
                }
                other => {
                    *color = board_settings.cell_color.into();

                    if other == Interaction::Pressed {
                        if **clickable {
                            cell_click_event.send(event::CellClick(board_pos.deref().clone()));
                        }
                    }
                }
            }
        }
    }

    pub fn turn_cells(mut _game_data: ResMut<resource::GameData>) {
        // implement
    }

    pub fn update_turn(mut game_data: ResMut<resource::GameData>) {
        // todo: check if the board can be clicked on this turn
        game_data.turn.next_mut();
        game_data.turn_count += 1;
        info!("update_turn: {:?}", game_data);
    }
}
