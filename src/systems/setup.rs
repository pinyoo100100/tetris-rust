use bevy::prelude::*;

use crate::board::*;
use crate::components::*;
use crate::constants::*;

/// Spawn the 2D camera.
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Spawn the grid background cells and border.
pub fn setup_grid(mut commands: Commands) {
    // Spawn background cells
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            commands.spawn((
                Sprite {
                    color: Color::srgba(
                        EMPTY_CELL_COLOR[0],
                        EMPTY_CELL_COLOR[1],
                        EMPTY_CELL_COLOR[2],
                        EMPTY_CELL_COLOR[3],
                    ),
                    custom_size: Some(Vec2::new(BLOCK_SIZE - 1.0, BLOCK_SIZE - 1.0)),
                    ..default()
                },
                Transform::from_translation(block_world_position(x as i32, y as i32, Z_GRID)),
                GridCell,
            ));
        }
    }

    // Spawn grid border (4 thin rectangles)
    let border_color = Color::srgba(
        GRID_BORDER_COLOR[0],
        GRID_BORDER_COLOR[1],
        GRID_BORDER_COLOR[2],
        GRID_BORDER_COLOR[3],
    );
    let board_width = GRID_WIDTH as f32 * BLOCK_SIZE;
    let board_height = GRID_HEIGHT as f32 * BLOCK_SIZE;
    let center_x = BOARD_OFFSET_X + board_width / 2.0;
    let center_y = BOARD_OFFSET_Y + board_height / 2.0;
    let border_thickness = 2.0;

    // Left border
    commands.spawn((
        Sprite {
            color: border_color,
            custom_size: Some(Vec2::new(border_thickness, board_height + border_thickness)),
            ..default()
        },
        Transform::from_xyz(BOARD_OFFSET_X - border_thickness / 2.0, center_y, Z_GRID + 0.1),
        GridBorder,
    ));

    // Right border
    commands.spawn((
        Sprite {
            color: border_color,
            custom_size: Some(Vec2::new(border_thickness, board_height + border_thickness)),
            ..default()
        },
        Transform::from_xyz(
            BOARD_OFFSET_X + board_width + border_thickness / 2.0,
            center_y,
            Z_GRID + 0.1,
        ),
        GridBorder,
    ));

    // Bottom border
    commands.spawn((
        Sprite {
            color: border_color,
            custom_size: Some(Vec2::new(board_width + border_thickness * 2.0, border_thickness)),
            ..default()
        },
        Transform::from_xyz(center_x, BOARD_OFFSET_Y - border_thickness / 2.0, Z_GRID + 0.1),
        GridBorder,
    ));

    // Top border
    commands.spawn((
        Sprite {
            color: border_color,
            custom_size: Some(Vec2::new(board_width + border_thickness * 2.0, border_thickness)),
            ..default()
        },
        Transform::from_xyz(
            center_x,
            BOARD_OFFSET_Y + board_height + border_thickness / 2.0,
            Z_GRID + 0.1,
        ),
        GridBorder,
    ));
}

/// Spawn the UI text elements for score, level, and lines.
pub fn setup_ui(mut commands: Commands) {
    // Root UI node
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexEnd,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("TETRIS"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Score label
            parent.spawn((
                Text::new("Score"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));

            // Score value
            parent.spawn((
                Text::new("0"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
                ScoreText,
            ));

            // Level label
            parent.spawn((
                Text::new("Level"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));

            // Level value
            parent.spawn((
                Text::new("0"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
                LevelText,
            ));

            // Lines label
            parent.spawn((
                Text::new("Lines"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));

            // Lines value
            parent.spawn((
                Text::new("0"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                LinesText,
            ));

            // Next piece label
            parent.spawn((
                Text::new("Next"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Controls info
            parent.spawn((
                Text::new("Controls"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect {
                        top: Val::Px(120.0),
                        bottom: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                },
            ));

            let control_texts = [
                "← → : Move",
                "↑ : Rotate",
                "↓ : Soft Drop",
                "Space : Hard Drop",
                "R : Restart",
            ];

            for text in control_texts {
                parent.spawn((
                    Text::new(text),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.5, 0.5, 0.5)),
                    Node {
                        margin: UiRect::bottom(Val::Px(3.0)),
                        ..default()
                    },
                ));
            }
        });
}
