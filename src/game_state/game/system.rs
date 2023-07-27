use bevy::prelude::*;

use super::util;
use crate::board;

pub use data::{Board, BoardCell, BoardSettings, Player};
pub use plugin::GamePlugin;

use super::util::despawn_entities_and_clear_resource;
use super::{position_pairs, GameState};

mod plugin {
    use super::*;

    #[derive(Clone)]
    pub struct GamePlugin {
        pub first_turn: Player,
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
}

mod data {
    use std::ops::Rem;

    use super::*;

    pub type BoardCell = Option<data::Player>;
    pub type Board = board::Board<BoardCell>;

    #[derive(Clone, Copy, Debug, Deref)]
    pub struct BoardSize(u16);

    impl BoardSize {
        pub fn size(&self) -> u16 {
            self.0
        }
    }

    impl Into<u16> for BoardSize {
        fn into(self) -> u16 {
            self.0.into()
        }
    }

    impl TryFrom<u16> for BoardSize {
        type Error = &'static str;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value.rem(2) {
                0 => Ok(BoardSize(value)),
                _ => Err("BoardSize can only be even."),
            }
        }
    }

    #[derive(Clone)]
    pub struct BoardSettings {
        pub board_size_x: BoardSize,
        pub board_size_y: BoardSize,
        pub cell_color: Color,
        pub cell_hovered_color: Color,
        pub background_color: Color,
    }

    #[derive(Debug)]
    pub struct CellData {
        pub position: board::BoardPosition,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum Player {
        #[default]
        None,
        Black,
        White,
    }

    impl Player {
        pub fn next(&self) -> Self {
            use Player::*;
            match self {
                Black => White,
                White => Black,
                None => None,
            }
        }

        pub fn next_mut(&mut self) {
            *self = self.next();
        }
    }

    #[derive(Debug)]
    pub struct GameData {
        pub turn: Player,
        pub turn_count: u16,
        pub board: Board,
    }

    impl GameData {
        pub fn new(first_turn: Player, board_size_x: BoardSize, board_size_y: BoardSize) -> Self {
            let size =
                board::Size::new(board_size_x.size().into(), board_size_y.size().into()).unwrap();
            let board = Board::new(size);
            GameData {
                turn: first_turn,
                turn_count: 0,
                board: board,
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
    pub struct BoardPosition(pub board::BoardPosition);

    #[derive(Component)]
    pub struct Cell;

    #[derive(Component, Deref, DerefMut)]
    pub struct Clickable(pub bool);

    #[derive(Component, Deref, DerefMut, Debug)]
    pub struct Player(pub data::Player);
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

    use bevy::utils::HashMap;

    use super::*;

    pub fn spawn_board_ui(mut commands: Commands, board_settings: Res<resource::BoardSettings>) {
        info!("spawn_board_ui");

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
                                board_settings.board_size_y.into(),
                                1.0,
                            ),
                            grid_template_rows: RepeatedGridTrack::flex(
                                board_settings.board_size_x.into(),
                                1.0,
                            ),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        for (x, y) in position_pairs::<u16>(
                            board_settings.board_size_x.into(),
                            board_settings.board_size_y.into(),
                        ) {
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
            .insert(component::Cell)
            .insert(component::BoardPosition(pos))
            .insert(component::Clickable(false))
            .insert(component::Player(data::Player::default()));
    }

    pub fn set_initial_player_cells(
        mut cells: Query<
            (&component::BoardPosition, &mut component::Player),
            Added<component::Cell>,
        >,
        board_settings: Res<resource::BoardSettings>,
        mut game_data: ResMut<resource::GameData>,
    ) {
        if cells.is_empty() {
            return;
        }

        const INDEX_OFFSET: u16 = 1;
        let starting_point = (
            ((board_settings.board_size_x.size() / 2) - INDEX_OFFSET) as usize,
            ((board_settings.board_size_y.size() / 2) - INDEX_OFFSET) as usize,
        );
        let initial_cell_positions: [(board::BoardPosition, data::Player); 4] = [
            (
                (starting_point.0, starting_point.1).into(),
                data::Player::Black,
            ),
            (
                (starting_point.0 + 1, starting_point.1).into(),
                data::Player::White,
            ),
            (
                (starting_point.0, starting_point.1 + 1).into(),
                data::Player::White,
            ),
            (
                (starting_point.0 + 1, starting_point.1 + 1).into(),
                data::Player::Black,
            ),
        ];
        info!("{:?}", initial_cell_positions);
        let initial_cell_positions = initial_cell_positions
            .into_iter()
            .collect::<HashMap<board::BoardPosition, data::Player>>();

        for (pos, mut player) in cells.iter_mut() {
            if let Some(set_player) = initial_cell_positions.get(pos.deref()) {
                **player = set_player.clone();
            }
        }

        for (pos, player) in initial_cell_positions.iter() {
            let cell_mut = game_data.board.cell_mut(pos.deref()).unwrap();
            cell_mut.replace(player.clone());
        }
    }

    pub fn update_cell_clickable(
        mut cells: Query<
            (
                &mut component::Clickable,
                &component::BoardPosition,
                &component::Player,
            ),
            Changed<component::Clickable>,
        >,
        mut _game_data: ResMut<resource::GameData>,
    ) {
        let game_data = &**_game_data;
        let _current_turn = &game_data.turn;

        for (_clickable, _board_position, _player) in cells.iter_mut() {
            // update clickable component of cells
        }
    }

    pub fn update_player_cell_color(
        mut cells: Query<
            (
                &mut BackgroundColor,
                &component::Player,
                &Interaction,
                &component::Clickable,
            ),
            With<component::Cell>,
        >,
        board_settings: Res<resource::BoardSettings>,
    ) {
        for (mut background_color, player, interaction, clickable) in cells.iter_mut() {
            if !**clickable {
                break;
            }

            match interaction {
                Interaction::None => {
                    *background_color = match **player {
                        data::Player::Black => Color::BLACK,
                        data::Player::White => Color::WHITE,
                        data::Player::None => board_settings.cell_color,
                    }
                    .into();
                }
                _ => (),
            }
        }
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
                            info!("clicked: {:?}", board_pos.deref());
                            cell_click_event.send(event::CellClick(board_pos.deref().clone()));
                        }
                    }
                }
            }
        }
    }

    pub fn turn_cells(_cells: Query<&component::Cell>, mut _game_data: ResMut<resource::GameData>) {
        // cells.iter_many(f)

        // implement
    }

    pub fn update_turn(
        // clickable_cells: Query<&component::Clickable>,
        mut game_data: ResMut<resource::GameData>,
    ) {
        // clickable_cells.

        // todo: check if the board can be clicked on this turn
        game_data.turn.next_mut();
        game_data.turn_count += 1;
        info!("update_turn: {:?}", game_data);
    }
}
