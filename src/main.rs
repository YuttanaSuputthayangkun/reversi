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

fn main() {
    // setup_example();
    setup_game();
}

#[derive(Component)]
struct BoardPositionComponent(BoardPosition);

#[derive(Component)]
struct ClickedComponent;

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
        .add_systems(Startup, spawn_board_ui)
        .add_systems(Update, button_system)
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
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center, // check this
                grid_template_columns: vec![
                    GridTrack::flex(1.), // left
                    GridTrack::auto(),   // board
                    GridTrack::flex(1.), // right
                ],
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
                        justify_content: JustifyContent::Center, // check this
                        aspect_ratio: Some(1.0),
                        padding: UiRect::all(Val::Px(10.)),
                        column_gap: Val::Px(5.),
                        row_gap: Val::Px(5.),
                        grid_template_columns: RepeatedGridTrack::flex(BOARD_SIZE_Y, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(BOARD_SIZE_X, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    let pos_pairs = (0..BOARD_SIZE_X)
                        .map(|x| (0..BOARD_SIZE_Y).map(move |y| (x, y)))
                        .flat_map(|x| x);
                    for (x, y) in pos_pairs {
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
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = CELL_COLOR_HOVERED.into();
            }
            _ => {
                *color = CELL_COLOR.into();
            }
        }
    }
}

fn setup_example() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [1280., 720.].into(),
                title: "Reversi".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, spawn_layout)
        .run();
}

fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn(Camera2dBundle::default());

    // Top-level grid (app frame)
    commands
        .spawn(NodeBundle {
            style: Style {
                /// Use the CSS Grid algorithm for laying out this node
                display: Display::Grid,
                /// Make node fill the entirety it's parent (in this case the window)
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                /// Set the grid to have 2 columns with sizes [min-content, minmax(0, 1fr)]
                ///   - The first column will size to the size of it's contents
                ///   - The second column will take up the remaining available space
                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
                /// Set the grid to have 3 rows with sizes [auto, minmax(0, 1fr), 20px]
                ///  - The first row will size to the size of it's contents
                ///  - The second row take up remaining available space (after rows 1 and 3 have both been sized)
                ///  - The third row will be exactly 20px high
                grid_template_rows: vec![
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::px(20.),
                ],
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        })
        .with_children(|builder| {
            // Header
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        /// Make this node span two grid columns so that it takes up the entire top tow
                        grid_column: GridPlacement::span(2),
                        padding: UiRect::all(Val::Px(6.0)),
                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_nested_text_bundle(builder, font.clone(), "Bevy CSS Grid Layout Example");
                });

            // Main content grid (auto placed in row 2, column 1)
            builder
                .spawn(NodeBundle {
                    style: Style {
                        /// Make the height of the node fill its parent
                        height: Val::Percent(100.0),
                        /// Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
                        /// As the height is set explicitly, this means the width will adjust to match the height
                        aspect_ratio: Some(1.0),
                        /// Use grid layout for this node
                        display: Display::Grid,
                        // Add 24px of padding around the grid
                        padding: UiRect::all(Val::Px(24.0)),
                        /// Set the grid to have 4 columns all with sizes minmax(0, 1fr)
                        /// This creates 4 exactly evenly sized columns
                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                        /// Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                        /// This creates 4 exactly evenly sized rows
                        grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                        /// Set a 12px gap/gutter between rows and columns
                        row_gap: Val::Px(12.0),
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::DARK_GRAY),
                    ..default()
                })
                .with_children(|builder| {
                    // Note there is no need to specify the position for each grid item. Grid items that are
                    // not given an explicit position will be automatically positioned into the next available
                    // grid cell. The order in which this is performed can be controlled using the grid_auto_flow
                    // style property.

                    item_rect(builder, Color::ORANGE);
                    item_rect(builder, Color::BISQUE);
                    item_rect(builder, Color::BLUE);
                    item_rect(builder, Color::CRIMSON);

                    item_rect(builder, Color::CYAN);
                    item_rect(builder, Color::ORANGE_RED);
                    item_rect(builder, Color::DARK_GREEN);
                    item_rect(builder, Color::FUCHSIA);

                    item_rect(builder, Color::TEAL);
                    item_rect(builder, Color::ALICE_BLUE);
                    item_rect(builder, Color::CRIMSON);
                    item_rect(builder, Color::ANTIQUE_WHITE);

                    item_rect(builder, Color::YELLOW);
                    item_rect(builder, Color::PINK);
                    item_rect(builder, Color::YELLOW_GREEN);
                    item_rect(builder, Color::SALMON);
                });

            // Right side bar (auto placed in row 2, column 2)
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        // Align content towards the start (top) in the vertical axis
                        align_items: AlignItems::Start,
                        // Align content towards the center in the horizontal axis
                        justify_items: JustifyItems::Center,
                        // Add 10px padding
                        padding: UiRect::all(Val::Px(10.)),
                        // Add an fr track to take up all the available space at the bottom of the column so that the text nodes
                        // can be top-aligned. Normally you'd use flexbox for this, but this is the CSS Grid example so we're using grid.
                        grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::fr(1.0)],
                        // Add a 10px gap between rows
                        row_gap: Val::Px(10.),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Sidebar",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ));
                    builder.spawn(TextBundle::from_section(
                        "A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely.",
                        TextStyle {
                            font: font.clone(),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    ));
                    builder.spawn(NodeBundle::default());
                });

            // Footer / status bar
            builder.spawn(NodeBundle {
                style: Style {
                    // Make this node span two grid column so that it takes up the entire bottom row
                    grid_column: GridPlacement::span(2),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            });

            // Modal (absolutely positioned on top of content - currently hidden: to view it, change its visibility)
            builder.spawn(NodeBundle {
                visibility: Visibility::Hidden,
                style: Style {
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        top: Val::Px(100.),
                        bottom: Val::Auto,
                        left: Val::Auto,
                        right: Val::Auto,
                    },
                    width: Val::Percent(60.),
                    height: Val::Px(300.),
                    max_width: Val::Px(600.),
                    max_height: Val::Auto,
                    ..default()
                },
                background_color: BackgroundColor(Color::Rgba {
                    red: 255.0,
                    green: 255.0,
                    blue: 255.0,
                    alpha: 0.8,
                }),
                ..default()
            });
        });
}

/// Create a coloured rectangle node. The node has size as it is assumed that it will be
/// spawned as a child of a Grid container with `AlignItems::Stretch` and `JustifyItems::Stretch`
/// which will allow it to take it's size from the size of the grid area it occupies.
fn item_rect(builder: &mut ChildBuilder, color: Color) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            });
        });
}

fn spawn_nested_text_bundle(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
    builder.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 24.0,
            color: Color::BLACK,
        },
    ));
}
