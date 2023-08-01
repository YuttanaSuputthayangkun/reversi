use crate::board;
use bevy::prelude::*;
use itertools::Itertools;

use super::position_pairs;
pub use data::{Board, BoardCell, BoardSettings, Player};

use std::ops::{Deref, DerefMut};

use bevy::utils::HashMap;

use super::*;

pub fn spawn_board_ui(mut commands: Commands, board_settings: Res<resource::BoardSettings>) {
    info!("spawn_board_ui");

    let mut entities = resource::Entities::default();
    let mut cell_entities = resource::BoardCellEntities::default();
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
                            board_settings.board_size_y().into(),
                            1.0,
                        ),
                        grid_template_rows: RepeatedGridTrack::flex(
                            board_settings.board_size_x().into(),
                            1.0,
                        ),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    for (x, y) in position_pairs::<u16>(
                        board_settings.board_size_x().into(),
                        board_settings.board_size_y().into(),
                    ) {
                        let pos = board::BoardPosition {
                            x: x.into(),
                            y: y.into(),
                        };
                        let cell_entity = spawn_cell(builder, pos, &board_settings);
                        cell_entities.deref_mut().insert(pos, cell_entity);
                    }
                });
        })
        .id();
    entities.push(board);
    commands.insert_resource(entities);
    commands.insert_resource(cell_entities);
}

pub fn spawn_cell(
    builder: &mut ChildBuilder,
    pos: board::BoardPosition,
    board_settings: &resource::BoardSettings,
) -> Entity {
    builder
        .spawn(ButtonBundle {
            style: Style {
                aspect_ratio: Some(1.0),
                ..default()
            },
            background_color: board_settings.background_color().into(),
            ..default()
        })
        .insert(component::Cell)
        .insert(component::BoardPosition(pos))
        .insert(component::Clickable(false))
        .insert(component::Player(data::Player::default()))
        .id()
}

pub fn set_initial_player_cells(
    mut cells: Query<(&component::BoardPosition, &mut component::Player), Added<component::Cell>>,
    board_settings: Res<resource::BoardSettings>,
    mut game_data: ResMut<resource::GameData>,
) {
    if cells.is_empty() {
        return;
    }

    const INDEX_OFFSET: u16 = 1;
    let starting_point = (
        ((board_settings.board_size_x().size() / 2) - INDEX_OFFSET) as board::PositionUnit,
        ((board_settings.board_size_y().size() / 2) - INDEX_OFFSET) as board::PositionUnit,
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
        let cell_mut = game_data.board_mut().cell_mut(pos.deref()).unwrap();
        *cell_mut = player.clone();
    }
}

pub fn button_interaction_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &component::BoardPosition,
            &component::Clickable,
        ),
        Changed<Interaction>,
    >,
    mut cell_click_event: EventWriter<event::CellClick>,
) {
    for (interaction, board_pos, clickable) in interaction_query.iter_mut() {
        if interaction == &Interaction::Pressed && **clickable {
            info!("clicked: {:?}", board_pos.deref());
            cell_click_event.send(event::CellClick(board_pos.deref().clone()));
        }
    }
}

pub fn change_clicked_player_cell(
    mut query: Query<(&component::BoardPosition, &mut component::Player)>,
    mut game_data: ResMut<resource::GameData>,
    mut cell_click_reader: EventReader<event::CellClick>,
    mut player_cell_changed_writer: EventWriter<event::PlayerCellChanged>,
) {
    if let Some(cell_click) = cell_click_reader.iter().next() {
        // find the entity and change the player component
        let (board_position, mut matched_cell_player) = query
            .iter_mut()
            .find(|(pos, _)| pos.eq(cell_click))
            .unwrap();
        let new_player = game_data.current_player();
        info!("change_clicked_player_cell new_player({:?})", &new_player);
        **matched_cell_player = new_player;

        // update player on the game data
        let board_player_mut = game_data.board_mut().cell_mut(board_position).unwrap();
        *board_player_mut = new_player;

        player_cell_changed_writer.send(event::PlayerCellChanged {
            player: new_player,
            board_position: **board_position,
        })
    }
}

pub fn change_opposite_player_cells(
    mut query: Query<(&component::BoardPosition, &mut component::Player)>,
    mut game_data: ResMut<resource::GameData>,
    mut player_cell_changed_reader: EventReader<event::PlayerCellChanged>,
) {
    if let Some(player_cell_changed) = player_cell_changed_reader.iter().next() {
        let current_player = game_data.current_player();
        let opposite_player = game_data.opposite_player();

        // info!(
        //     "current_player({:?}) opposite_player({:?})",
        //     &current_player, &opposite_player
        // );

        info!(
            "change_opposite_player_cells player_cell_changed position({:?}) to player({:?})",
            &player_cell_changed.board_position, &player_cell_changed.player
        );

        // try to get opposite player cell's position that connects with current player in all directions
        let opposite_positions = board::DIRECTIONS
            .iter()
            .flat_map(|direction| {
                let mut iter = board::Iter::new(
                    &game_data.board(),
                    player_cell_changed.board_position,
                    direction.clone(),
                    1,
                );

                let mut opposite_positions: Vec<board::BoardPosition> = vec![];
                // todo: implement function for iterator. somthing like take_until...?
                let found_same_player_on_other_side = loop {
                    match iter.next() {
                        Some((pos, player)) if player.eq(&opposite_player) => {
                            opposite_positions.push(pos);
                        }
                        Some((_, player)) if player.eq(&current_player) => {
                            break true;
                        }
                        _ => break false,
                    }
                };

                info!(
                    "direction({:?}) opposite_positions({:?})",
                    direction, &opposite_positions
                );

                if found_same_player_on_other_side {
                    opposite_positions
                } else {
                    vec![]
                }
            })
            .map(|pos| (pos, ()))
            .collect::<HashMap<board::BoardPosition, ()>>();

        info!(
            "change_opposite_player_cells opposite_positions({:?})",
            &opposite_positions
        );

        // update entities
        for (board_position, mut player) in query.iter_mut() {
            if opposite_positions.contains_key(board_position.deref()) {
                **player = current_player;
            }
        }

        // update board in game_data
        for position in opposite_positions.keys() {
            let cell_mut = game_data.board_mut().cell_mut(position).unwrap();
            *cell_mut = current_player;
        }
    }
}

pub fn clear_cell_clickable(mut cells: Query<&mut component::Clickable>) {
    for mut clickable in cells.iter_mut() {
        **clickable = false;
    }
}

pub fn update_cell_clickable(
    mut commands: Commands,
    mut cells: Query<(&component::BoardPosition, &component::Player)>,
    game_data: Res<resource::GameData>,
    board_entities: Res<resource::BoardCellEntities>,
) {
    let cell_group = cells
        .iter()
        .sorted_by(|(_, a), (_, b)| a.cmp(b))
        .group_by(|x| x.1.deref());
    let cell_log = cell_group
        .into_iter()
        .map(|(player, cells)| {
            let cell_texts = cells
                .map(|(pos, _)| format!("({},{})", pos.x, pos.y))
                .collect::<Vec<_>>();
            let cell_count = cell_texts.iter().count();
            let cell_texts = cell_texts.into_iter().collect::<Vec<_>>().join(", ");
            format!(
                "(player({:?}), count({}) cells({}))",
                player, cell_count, cell_texts
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    info!(
        "update_cell_clickable cells count({}), turn({:?}) cells({})",
        cells.iter().count(),
        game_data.as_ref().turn(),
        cell_log
    );

    let game_data = &**game_data;
    let current_player = game_data.current_player();
    let opposite_player = game_data.opposite_player();
    info!(
        "update_cell_clickable current_player({:?}) opposite_player({:?})",
        &current_player, &opposite_player
    );

    // todo: remove after debug
    let mut clickable_positions: Vec<board::BoardPosition> = vec![];

    for (board_position, player) in cells.iter_mut() {
        // check only if the cell matches the current turn player
        if player.ne(&current_player) {
            continue;
        }

        info!(
            "check on player({:?}) pos({:?})",
            &player,
            board_position.deref()
        );

        for direction in board::DIRECTIONS.iter() {
            let mut iter = board::Iter::new(
                &game_data.board(),
                board_position.deref().clone(),
                direction.clone(),
                1,
            );

            {
                let board_position = board_position.deref().clone();
                info!(
                    "direction({:?}) from position({:?})",
                    &direction, board_position
                );
            }

            // the next cell has to be the opposite player, skip loop if not
            // "if let &&" pattern is not available here
            match iter.next() {
                Some((_, player)) if player.ne(&opposite_player) => {
                    info!(
                        "The next is not opposite player! It's player({:?})",
                        &player
                    );
                    continue;
                }
                _ => (),
            }

            // skip all opposite players
            let mut iter = iter.skip_while(|(_, &p)| p.eq(&opposite_player));

            // check if the cell has empty player
            if let Some((cell_position, data::Player::None)) = iter.next() {
                let entity = board_entities.get(&cell_position).unwrap();
                commands
                    .entity(entity.clone())
                    .insert(component::Clickable(true)); // todo: check if there's any way to change the component instead of insert
                clickable_positions.push(cell_position.clone());
            }
        }
    }

    // todo: remove after debug
    let cell_strings: Vec<String> = clickable_positions
        .iter()
        .map(|p| format!("({}, {})", p.x, p.y))
        .collect();
    let cell_string = cell_strings.join(", ");
    info!("clickable cells: {:?}", &cell_string);
}

pub fn change_cell_color(
    mut cells: Query<
        (
            &mut BackgroundColor,
            &component::Player,
            &component::Clickable,
        ),
        (
            With<component::Cell>,
            Or<(
                Changed<component::Clickable>,
                Added<component::Clickable>,
                Changed<component::Player>,
                Added<component::Player>,
            )>,
        ),
    >,
    board_settings: Res<resource::BoardSettings>,
) {
    for (mut background_color, player, clickable) in cells.iter_mut() {
        if **clickable {
            *background_color = board_settings.cell_color_clickable().into();
            continue;
        }

        *background_color = board_settings.player_cell_color(player).into();
    }
}

pub fn update_turn(mut game_data: ResMut<resource::GameData>) {
    game_data.update_turn();
    {
        let turn = game_data.turn();
        info!("update_turn to player({:?})", turn);
    }
}

pub fn check_win_condition(game_data: Res<resource::GameData>) {
    // todo
}
