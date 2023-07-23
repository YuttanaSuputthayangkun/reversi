use bevy::prelude::*;

use super::util::Entities;
use crate::board::BoardPosition;

pub use plugin::GamePlugin;
pub use resource::BoardSettings;

use super::util::despawn_entities_and_clear_resource;
use super::{position_pairs, GameState};

mod plugin {
    use super::*;

    #[derive(Clone)]
    pub struct GamePlugin {
        pub board_settings: resource::BoardSettings,
    }

    impl Plugin for GamePlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.insert_resource(self.board_settings.clone())
                .add_systems(OnEnter(GameState::Game), system::spawn_board_ui)
                .add_systems(
                    OnExit(GameState::Game),
                    despawn_entities_and_clear_resource::<resource::Entities>,
                )
                .add_systems(
                    Update,
                    (system::button_system, system::read_event_system)
                        .run_if(in_state(GameState::Game)),
                )
                .add_event::<event::CellClicked>();
        }
    }
}

mod resource {
    use super::*;

    #[derive(Resource, Clone)]
    pub struct BoardSettings {
        pub board_size_x: u16,
        pub board_size_y: u16,
        pub cell_color: Color,
        pub cell_hovered_color: Color,
        pub background_color: Color,
    }

    #[derive(Default)]
    pub struct BoardEntities;

    pub type Entities = super::Entities<BoardEntities>;

    #[allow(dead_code)]
    pub enum Turn {
        Black,
        White,
    }

    #[allow(dead_code)]
    pub struct TurnData {
        turn: Turn,
    }
}

mod component {
    use super::*;

    #[derive(Component)]
    pub struct BoardPositionComponent(pub BoardPosition);

    #[derive(Component, Deref, DerefMut)]
    pub struct Clickable(#[deref] pub bool);
}

mod event {
    use super::*;

    #[derive(Event)]
    pub struct CellClicked(pub BoardPosition);
}

mod system {
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
                            let pos = BoardPosition {
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
        pos: BoardPosition,
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
            .insert(component::BoardPositionComponent(pos))
            .insert(component::Clickable(true)); // change this to false when the clickable button system is complete
    }

    pub fn button_system(
        mut interaction_query: Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &component::BoardPositionComponent,
                &component::Clickable,
            ),
            (
                Changed<Interaction>,
                With<Button>,
                With<component::BoardPositionComponent>,
                With<component::Clickable>,
            ),
        >,
        mut event_writer: EventWriter<event::CellClicked>,
        board_settings: Res<resource::BoardSettings>,
    ) {
        for (interaction, mut color, board_pos, clickable) in &mut interaction_query {
            if !**clickable {
                return;
            }

            match *interaction {
                Interaction::Hovered => {
                    *color = board_settings.cell_hovered_color.into();
                }
                other => {
                    *color = board_settings.cell_color.into();

                    if other == Interaction::Pressed {
                        let event = event::CellClicked(board_pos.0);
                        event_writer.send(event);
                    }
                }
            }
        }
    }

    pub fn read_event_system(mut event_reader: EventReader<event::CellClicked>) {
        for e in event_reader.iter() {
            info!("CellClicked {:?}", e.0);
        }
    }
}
