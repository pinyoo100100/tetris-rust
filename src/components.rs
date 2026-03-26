use bevy::prelude::*;

/// Marker component for blocks that are part of the currently falling piece.
#[derive(Component)]
pub struct FallingBlock;

/// Marker component for blocks that have been locked onto the board.
#[derive(Component)]
pub struct LockedBlock;

/// Marker component for ghost piece blocks (preview of landing position).
#[derive(Component)]
pub struct GhostBlock;

/// Marker component for the grid background cells.
#[derive(Component)]
pub struct GridCell;

/// Marker component for the grid border lines.
#[derive(Component)]
pub struct GridBorder;

/// Marker component for the score text UI.
#[derive(Component)]
pub struct ScoreText;

/// Marker component for the level text UI.
#[derive(Component)]
pub struct LevelText;

/// Marker component for the lines text UI.
#[derive(Component)]
pub struct LinesText;

/// Marker component for the game over overlay.
#[derive(Component)]
pub struct GameOverOverlay;

/// Marker component for next piece preview blocks.
#[derive(Component)]
pub struct NextPieceBlock;

/// Grid position component (column, row).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
