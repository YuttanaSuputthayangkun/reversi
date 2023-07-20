use bevy::prelude::*;

use crate::board::BoardPosition;

use super::{position_pairs, GameState};

#[derive(Clone)]
pub struct GamePlugin {
    pub board_size_x: u16,
    pub board_size_y: u16,
    pub cell_color: Color,
    pub cell_hovered_color: Color,
    pub background_color: Color,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(BoardResource::from(self))
            .add_systems(OnEnter(GameState::Game), spawn_board_ui)
            .add_systems(OnExit(GameState::Game), despawn_board_ui)
            .add_systems(
                Update,
                (button_system, read_event_system).run_if(in_state(GameState::Game)),
            )
            .add_event::<CellClicked>();
    }
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

#[derive(Resource)]
struct BoardResource {
    entity_list: Vec<Entity>,
    board_size_x: u16,
    board_size_y: u16,
    cell_color: Color,
    cell_hovered_color: Color,
    background_color: Color,
}

impl<'a> From<&'a GamePlugin> for BoardResource {
    fn from(value: &'a GamePlugin) -> Self {
        BoardResource {
            entity_list: vec![],
            board_size_x: value.board_size_x,
            board_size_y: value.board_size_y,
            cell_color: value.cell_color,
            cell_hovered_color: value.cell_hovered_color,
            background_color: value.background_color,
        }
    }
}

fn spawn_board_ui(mut commands: Commands, mut board_resource: ResMut<BoardResource>) {
    let camera = commands.spawn(Camera2dBundle::default()).id();
    board_resource.entity_list.push(camera);
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
                            board_resource.board_size_y,
                            1.0,
                        ),
                        grid_template_rows: RepeatedGridTrack::flex(
                            board_resource.board_size_x,
                            1.0,
                        ),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    for (x, y) in
                        position_pairs(board_resource.board_size_x, board_resource.board_size_y)
                    {
                        let pos = BoardPosition {
                            x: x.into(),
                            y: y.into(),
                        };
                        spawn_cell(builder, pos, &board_resource);
                    }
                });
        })
        .id();
    board_resource.entity_list.push(board);
}

fn spawn_cell(builder: &mut ChildBuilder, pos: BoardPosition, board_resource: &BoardResource) {
    builder
        .spawn(ButtonBundle {
            style: Style {
                aspect_ratio: Some(1.0),
                ..default()
            },
            background_color: board_resource.background_color.into(),
            ..default()
        })
        .insert(BoardPositionComponent(pos));
}

fn despawn_board_ui(mut commands: Commands, mut board_resource: ResMut<BoardResource>) {
    for id in board_resource.entity_list.iter() {
        commands.entity(id.clone()).despawn_recursive();
    }
    board_resource.entity_list.clear();
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
    board_resource: Res<BoardResource>,
) {
    for (interaction, mut color, board_pos) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = board_resource.cell_hovered_color.into();
            }
            other => {
                *color = board_resource.cell_color.into();

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
