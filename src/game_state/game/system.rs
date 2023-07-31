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
            background_color: board_settings.background_color.into(),
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
        let cell_mut = game_data.board_mut().cell_mut(pos.deref()).unwrap();
        *cell_mut = player.clone();
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
        game_data.as_ref().turn,
        cell_log
    );

    let game_data = &**game_data;
    let current_turn = game_data.turn;

    // todo: remove after debug
    let mut clickable_positions: Vec<board::BoardPosition> = vec![];

    for (board_position, player) in cells.iter_mut() {
        // check only if the cell matches the current turn player
        let current_player: data::Player = current_turn.into();
        if player.ne(&current_player) {
            continue;
        }

        info!(
            "check on player({:?}) pos({:?})",
            &player,
            board_position.deref()
        );

        let opposite_player = player.next();
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
                    .insert(component::Clickable(true));
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
        if **clickable {
            *background_color = board_settings.cell_clickable_color.into();
        } else {
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

pub fn update_turn(mut game_data: ResMut<resource::GameData>) {
    game_data.turn = game_data.turn.next();
    game_data.turn_count += 1;
    {
        let turn = &game_data.turn;
        info!("update_turn to player({:?})", turn);
    }
}
