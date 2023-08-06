use crate::board;
use bevy::prelude::*;
use itertools::Itertools;

use super::position_pairs;
use bevy::utils::HashMap;
use std::ops::{Deref, DerefMut};

pub use data::{Board, BoardCell, BoardSettings, Player};

use super::*;

pub fn init_game_data(mut game_data: ResMut<resource::GameData>) {
    game_data.reset();
}

pub fn spawn_board_ui(
    mut commands: Commands,
    board_settings: Res<resource::BoardSettings>,
    game_data: Res<resource::GameData>,
) {
    let camera = commands.spawn(Camera2dBundle::default()).id();
    let mut entities = resource::Entities::default();
    entities.push(camera);
    let mut cell_entities = resource::BoardCellEntities::default();
    let ui = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                grid_template_columns: vec![GridTrack::max_content()],
                ..default()
            },
            background_color: board_settings
                .board_player_color(&Into::<data::Player>::into(game_data.turn().clone()))
                .into(),
            ..default()
        })
        .insert(component::BoardParent)
        .with_children(|builder| {
            // spawn the actual board
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
    entities.push(ui);
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
            background_color: board_settings.cell_color_background().into(),
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

                if found_same_player_on_other_side {
                    opposite_positions
                } else {
                    vec![]
                }
            })
            .map(|pos| (pos, ()))
            .collect::<HashMap<board::BoardPosition, ()>>();

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
    let game_data = &**game_data;
    let current_player = game_data.current_player();
    let opposite_player = game_data.opposite_player();
    for (board_position, player) in cells.iter_mut() {
        // check only if the cell matches the current turn player
        if player.ne(&current_player) {
            continue;
        }

        for direction in board::DIRECTIONS.iter() {
            let mut iter = board::Iter::new(
                &game_data.board(),
                board_position.deref().clone(),
                direction.clone(),
                1,
            );

            // the next cell has to be the opposite player, skip loop if not
            // "if let &&" pattern is not available here
            match iter.next() {
                Some((_, player)) if player.ne(&opposite_player) => {
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
            }
        }
    }
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

        *background_color = board_settings.cell_player_color(player).into();
    }
}

pub fn update_turn(
    mut game_data: ResMut<resource::GameData>,
    mut turn_stuck_reader: EventReader<event::TurnStuck>,
) {
    let is_turn_stuck = turn_stuck_reader.iter().next().is_some();
    if is_turn_stuck {
        game_data.notify_turn_stuck();
    }

    game_data.next_turn();

    {
        let turn = game_data.turn();
        info!("update_turn to player({:?})", turn);
    }
}

pub fn any_clickable_cell(query: Query<&component::Clickable>) -> bool {
    query.iter().find(|&clickable| **clickable).is_some()
}

pub fn check_win_condition(
    query: Query<&component::Player, With<component::Cell>>,
    game_data: Res<resource::GameData>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut result_event_writer: EventWriter<result::event::ResultEvent>,
) {
    if game_data.is_turn_stuck() {
        info!("The game has ended, there's no clickable cell for both players.");

        // go to the result game state
        next_game_state.set(GameState::Result);

        // calculate total score and send through the event
        let cell_count_map: HashMap<data::Player, usize> = query
            .iter()
            .map(|x| x.deref())
            .cloned()
            // .sorted_by(|a, b| a.cmp(b)) // TODO: check if sorting like this is okay
            .group_by(|x| x.clone())
            .into_iter()
            .map(|(player, group)| (player, group.count()))
            .collect();
        result_event_writer.send(result::event::ResultEvent(result::data::ResultData {
            scores: [
                (
                    result::data::PlayerType::White,
                    cell_count_map
                        .get(&data::Player::White)
                        .cloned()
                        .unwrap_or_default() as u16,
                ),
                (
                    result::data::PlayerType::Black,
                    cell_count_map
                        .get(&data::Player::Black)
                        .cloned()
                        .unwrap_or_default() as u16,
                ),
            ]
            .into_iter()
            .collect(),
        }));
    }
}

pub fn change_board_background_color(
    mut query: Query<&mut BackgroundColor, With<component::BoardParent>>,
    game_data: ResMut<resource::GameData>,
    board_settings: Res<resource::BoardSettings>,
    time: Res<Time>,
    mut begin_color: Local<Option<Color>>,
    mut timer: Local<Option<Timer>>,
    turn_change_reader: EventReader<event::TurnChange>,
) {
    let background_color = query.iter_mut().next();
    if background_color.is_none() {
        info!("Cannot find background color of the board");
        return;
    }
    let mut background_color = background_color.unwrap();

    // init transition on turn change: save color on begin and start new timer
    if !turn_change_reader.is_empty() {
        *begin_color = Some(background_color.deref().0);

        let duration = board_settings.board_player_color_change_duration();
        let new_timer = Timer::new(duration, TimerMode::Once);
        *timer = Some(new_timer);
    }

    // check if timer exist
    let is_finished: bool =
        if let (Some(timer), Some(begin_color)) = (timer.deref_mut(), begin_color.deref()) {
            // apply color, Bevy doesn't have color lerping now, so I use Vec4's
            let begin_color: Vec4 = begin_color.clone().into();
            let target_player: data::Player = game_data.turn().clone().into();
            let target_color: Vec4 = board_settings.board_player_color(&target_player).into();
            let (r, g, b, _) = begin_color.lerp(target_color, timer.percent()).into();
            *background_color = Color::rgba(r, g, b, 1.).into();

            timer.tick(time.delta());

            timer.just_finished()
        } else {
            false
        };

    if is_finished {
        *begin_color = None;
        *timer = None;
    }
}

#[cfg(feature = "debug")]
pub(super) mod debug {
    use std::time::Duration;

    use rand::Rng;

    use super::*;

    const DEBUG_KEYCODE: KeyCode = KeyCode::O;
    const AUTO_CELL_CLICK_DELAY: Duration = Duration::from_millis(100);

    pub fn auto_cell_click(
        query: Query<(&component::Clickable, &component::BoardPosition)>,
        key_press: Res<Input<KeyCode>>,
        time: Res<Time>,
        mut cell_click_event_writer: EventWriter<event::CellClick>,
        mut timer: Local<Option<Timer>>,
        mut is_enabled: Local<bool>,
    ) {
        // toggle on keypress
        if key_press.just_pressed(DEBUG_KEYCODE) {
            *is_enabled = !*is_enabled;
        }

        if !*is_enabled {
            return;
        }

        if timer.is_none() {
            let mut new_timer = Timer::default();
            new_timer.set_duration(AUTO_CELL_CLICK_DELAY);
            new_timer.set_mode(TimerMode::Once);
            timer.replace(new_timer);
        }

        let timer = timer.as_mut().unwrap();
        if timer.finished() {
            let clickable_positions: Vec<_> = query
                .iter()
                .filter(|(c, _)| ***c)
                .map(|(_, pos)| pos)
                .collect();
            let clickable_pos = if clickable_positions.is_empty() {
                None
            } else {
                // random pick
                // TODO: maybe move this into utils
                let pos =
                    clickable_positions[rand::thread_rng().gen_range(0..clickable_positions.len())];
                Some(pos)
            };

            if let Some(pos) = clickable_pos {
                cell_click_event_writer.send(event::CellClick(pos.deref().clone()));

                timer.reset();
            }
        }

        timer.tick(time.delta());
    }
}
