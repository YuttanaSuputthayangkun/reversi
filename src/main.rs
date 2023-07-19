#![allow(dead_code, unused_variables)]

// #[allow(dead_code)]
mod board;

use bevy::prelude::*;
use board::BoardPosition;

const GAME_TITLE: &'static str = "Reversi";
const WINDOW_RESOLUTION_X: f32 = 1280.;
const WINDOW_RESOLUTION_Y: f32 = 720.;
const BOARD_SIZE_X: u16 = 8;
const BOARD_SIZE_Y: u16 = 8;
const CELL_COLOR: Color = Color::Rgba {
    red: 30. / 256.,
    green: 128. / 256.,
    blue: 0.,
    alpha: 1.,
};
const CELL_COLOR_HOVERED: Color = Color::Rgba {
    red: 17. / 256.,
    green: 66. / 256.,
    blue: 1.,
    alpha: 1.,
};

#[cfg(test)]
mod bevy_test {
    use bevy::prelude::{Event, EventReader, Events, ResMut, Resource, Schedule, World};

    #[derive(Event)]
    struct Number(i32);

    #[test]
    fn world() {
        #[derive(Resource)]
        struct MyResource(i32);
        let mut world = World::new();
        world.insert_resource(MyResource(0));
        let mut schedule = Schedule::new();
        fn increase_number(mut res: ResMut<MyResource>) {
            res.0 = res.0 + 1;
        }
        schedule.add_systems(increase_number);
        schedule.run(&mut world);
        assert_eq!(1, world.get_resource::<MyResource>().unwrap().0);
        schedule.run(&mut world);
        assert_eq!(2, world.get_resource::<MyResource>().unwrap().0);
    }

    #[test]
    fn read_event_twice() {
        #[derive(Resource)]
        struct Counter(i32);
        #[derive(Event)]
        struct Event;
        let mut world = World::new();
        world.init_resource::<Events<Event>>();
        world.insert_resource(Counter(0));
        let mut schedule = Schedule::new();
        fn read_event(mut event_reader: EventReader<Event>, mut res: ResMut<Counter>) {
            for _ in event_reader.iter() {
                res.0 += 1;
            }
        }
        schedule
            .add_systems(Events::<Event>::update_system)
            .add_systems(read_event);

        // send first
        world.send_event(Event);
        schedule.run(&mut world);
        assert_eq!(1, world.get_resource::<Counter>().unwrap().0);

        // run again without sending
        schedule.run(&mut world);
        assert_eq!(1, world.get_resource::<Counter>().unwrap().0);

        // send second
        world.send_event(Event);
        schedule.run(&mut world);
        assert_eq!(2, world.get_resource::<Counter>().unwrap().0);
    }
}

fn main() {
    // setup_example();
    setup_game();
}

enum Turn {
    Black,
    White,
}

struct TurnData {
    turn: Turn,
}

#[derive(SystemSet, States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum GameState {
    #[default]
    Game,
    Result,
}

impl GameState {
    fn next(&self) -> Self {
        match self {
            GameState::Game => GameState::Result,
            GameState::Result => GameState::Game,
        }
    }
}

#[derive(Component)]
struct BoardPositionComponent(BoardPosition);

#[derive(Event)]
struct CellClicked(BoardPosition);

fn position_pairs() -> impl Iterator<Item = (u16, u16)> {
    (0..BOARD_SIZE_Y)
        .map(|y| (0..BOARD_SIZE_X).map(move |x| (x, y)))
        .flat_map(|x| x)
}

fn setup_game() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [WINDOW_RESOLUTION_X, WINDOW_RESOLUTION_Y].into(),
                title: GAME_TITLE.to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        // .add_systems(Startup, spawn_board_ui)
        .add_systems(OnEnter(GameState::Game), spawn_board_ui)
        .configure_sets(Startup, (GameState::Game, GameState::Result).chain())
        .configure_sets(Update, (GameState::Game, GameState::Result).chain())
        .add_systems(
            Update,
            (button_system, read_event_system)
                .in_set(GameState::Game)
                .run_if(in_state(GameState::Game)),
        )
        .add_event::<CellClicked>()
        .run();
}

fn spawn_board_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
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
                        grid_template_columns: RepeatedGridTrack::flex(BOARD_SIZE_Y, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(BOARD_SIZE_X, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    for (x, y) in position_pairs() {
                        let pos = BoardPosition {
                            x: x.into(),
                            y: y.into(),
                        };
                        spawn_cell(builder, pos);
                    }
                });
        });
}

fn spawn_cell(builder: &mut ChildBuilder, pos: BoardPosition) {
    let cell = builder
        .spawn(ButtonBundle {
            style: Style {
                aspect_ratio: Some(1.0),
                ..default()
            },
            background_color: CELL_COLOR.into(),
            ..default()
        })
        .insert(BoardPositionComponent(pos));
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
) {
    for (interaction, mut color, board_pos) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = CELL_COLOR_HOVERED.into();
            }
            other => {
                *color = CELL_COLOR.into();

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
