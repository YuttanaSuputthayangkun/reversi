use bevy::prelude::*;

use crate::board::BoardPosition;

pub use plugin::GamePlugin;
pub use resource::BoardSettings;

use super::util::{despawn_entities_and_clear_resource, IterEntity};
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
                    despawn_entities_and_clear_resource::<resource::BoardEntityList>,
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

    #[derive(Resource, Default)]
    pub struct BoardEntityList(pub Vec<Entity>);

    impl IterEntity for BoardEntityList {
        fn iter_entity(&self) -> Box<dyn Iterator<Item = Entity> + '_> {
            let iter = self.0.iter().map(|x| x.clone());
            Box::new(iter)
        }
    }

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
}

mod event {
    use super::*;

    #[derive(Event)]
    pub struct CellClicked(pub BoardPosition);
}

mod system {
    use super::*;

    pub fn spawn_board_ui(mut commands: Commands, board_settings: Res<resource::BoardSettings>) {
        let mut board_entity_list = resource::BoardEntityList::default();
        let camera = commands.spawn(Camera2dBundle::default()).id();
        board_entity_list.0.push(camera);
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
        board_entity_list.0.push(board);
        commands.insert_resource(board_entity_list);
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
            .insert(component::BoardPositionComponent(pos));
    }

    pub fn button_system(
        mut interaction_query: Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &component::BoardPositionComponent,
            ),
            (
                Changed<Interaction>,
                With<Button>,
                With<component::BoardPositionComponent>,
            ),
        >,
        mut event_writer: EventWriter<event::CellClicked>,
        board_settings: Res<resource::BoardSettings>,
    ) {
        for (interaction, mut color, board_pos) in &mut interaction_query {
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
