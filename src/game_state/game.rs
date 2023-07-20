use bevy::prelude::*;

use crate::board::BoardPosition;

use super::{position_pairs, GameState};

#[derive(Clone)]
pub struct GamePlugin {
    pub board_settings: BoardSettings,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(self.board_settings.clone())
            .add_systems(OnEnter(GameState::Game), spawn_board_ui)
            .add_systems(OnExit(GameState::Game), despawn_board_ui)
            .add_systems(
                Update,
                (button_system, read_event_system).run_if(in_state(GameState::Game)),
            )
            .add_event::<CellClicked>();
    }
}

#[derive(Resource, Clone)]
pub struct BoardSettings {
    pub board_size_x: u16,
    pub board_size_y: u16,
    pub cell_color: Color,
    pub cell_hovered_color: Color,
    pub background_color: Color,
}

#[allow(dead_code)]
enum Turn {
    Black,
    White,
}

#[allow(dead_code)]
struct TurnData {
    turn: Turn,
}

#[derive(Component)]
struct BoardPositionComponent(BoardPosition);

#[derive(Event)]
struct CellClicked(BoardPosition);

#[derive(Resource, Default)]
struct BoardEntityList {
    entity_list: Vec<Entity>,
}

fn spawn_board_ui(mut commands: Commands, board_settings: Res<BoardSettings>) {
    let mut board_entity_list = BoardEntityList::default();
    let camera = commands.spawn(Camera2dBundle::default()).id();
    board_entity_list.entity_list.push(camera);
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
                        let pos = BoardPosition {
                            x: x.into(),
                            y: y.into(),
                        };
                        spawn_cell(builder, pos, &board_settings);
                    }
                });
        })
        .id();
    board_entity_list.entity_list.push(board);
    commands.insert_resource(board_entity_list);
}

fn spawn_cell(builder: &mut ChildBuilder, pos: BoardPosition, board_settings: &BoardSettings) {
    builder
        .spawn(ButtonBundle {
            style: Style {
                aspect_ratio: Some(1.0),
                ..default()
            },
            background_color: board_settings.background_color.into(),
            ..default()
        })
        .insert(BoardPositionComponent(pos));
}

fn despawn_board_ui(mut commands: Commands, board_entity_list: Res<BoardEntityList>) {
    for id in board_entity_list.entity_list.iter() {
        commands.entity(id.clone()).despawn_recursive();
    }
    commands.remove_resource::<BoardEntityList>();
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &BoardPositionComponent),
        (
            Changed<Interaction>,
            With<Button>,
            With<BoardPositionComponent>,
        ),
    >,
    mut event_writer: EventWriter<CellClicked>,
    board_settings: Res<BoardSettings>,
) {
    for (interaction, mut color, board_pos) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = board_settings.cell_hovered_color.into();
            }
            other => {
                *color = board_settings.cell_color.into();

                if other == Interaction::Pressed {
                    let event = CellClicked(board_pos.0);
                    event_writer.send(event);
                }
            }
        }
    }
}

fn read_event_system(mut event_reader: EventReader<CellClicked>) {
    for e in event_reader.iter() {
        info!("CellClicked {:?}", e.0);
    }
}
