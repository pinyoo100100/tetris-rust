use bevy::prelude::*;

use crate::board::*;
use crate::components::*;
use crate::resources::*;
use crate::tetromino::TetrominoType;

/// Spawn a new falling piece. Returns false if the spawn position is blocked (game over).
pub fn spawn_piece(
    commands: &mut Commands,
    current_piece: &CurrentPiece,
) {
    let color = current_piece.piece_type.color();
    let positions = current_piece.block_positions();

    for &(x, y) in &positions {
        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(BLOCK_SIZE - 2.0, BLOCK_SIZE - 2.0)),
                ..default()
            },
            Transform::from_translation(block_world_position(x, y, Z_FALLING)),
            FallingBlock,
            GridPosition::new(x, y),
        ));
    }
}

/// Remove all falling block entities.
pub fn despawn_falling_blocks(commands: &mut Commands, query: &Query<Entity, With<FallingBlock>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Remove all ghost block entities.
pub fn despawn_ghost_blocks(commands: &mut Commands, query: &Query<Entity, With<GhostBlock>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Spawn ghost piece blocks showing where the piece will land.
pub fn spawn_ghost_piece(
    commands: &mut Commands,
    current_piece: &CurrentPiece,
    board: &Board,
) {
    let ghost_y = current_piece.hard_drop_y(board);
    let ghost_color = current_piece.piece_type.ghost_color();
    let offsets = current_piece.piece_type.blocks(current_piece.rotation);

    for &(dx, dy) in &offsets {
        let x = current_piece.x + dx;
        let y = ghost_y + dy;
        commands.spawn((
            Sprite {
                color: ghost_color,
                custom_size: Some(Vec2::new(BLOCK_SIZE - 2.0, BLOCK_SIZE - 2.0)),
                ..default()
            },
            Transform::from_translation(block_world_position(x, y, Z_GHOST)),
            GhostBlock,
            GridPosition::new(x, y),
        ));
    }
}

/// Spawn the next piece preview in the UI area.
pub fn spawn_next_piece_preview(
    commands: &mut Commands,
    next_piece: &NextPiece,
) {
    let color = next_piece.piece_type.color();
    let offsets = next_piece.piece_type.blocks(0);

    // Position the preview in the sidebar area
    let preview_base_x = SIDEBAR_X;
    let preview_base_y = 150.0;
    let preview_block_size = 20.0;

    for &(dx, dy) in &offsets {
        let x = preview_base_x + dx as f32 * preview_block_size;
        let y = preview_base_y + dy as f32 * preview_block_size;
        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(preview_block_size - 2.0, preview_block_size - 2.0)),
                ..default()
            },
            Transform::from_xyz(x, y, Z_FALLING),
            NextPieceBlock,
        ));
    }
}

/// Remove all next piece preview entities.
pub fn despawn_next_piece_preview(
    commands: &mut Commands,
    query: &Query<Entity, With<NextPieceBlock>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// System to initialize the first piece when entering Playing state.
pub fn spawn_initial_piece(
    mut commands: Commands,
    current_piece: Res<CurrentPiece>,
    next_piece: Res<NextPiece>,
) {
    spawn_piece(&mut commands, &current_piece);
    spawn_ghost_piece(&mut commands, &current_piece, &Board::default());
    spawn_next_piece_preview(&mut commands, &next_piece);
}

/// Lock the current piece onto the board and spawn a new one.
/// Returns true if the game should continue, false if game over.
pub fn lock_and_spawn(
    commands: &mut Commands,
    board: &mut Board,
    current_piece: &mut CurrentPiece,
    next_piece: &mut NextPiece,
    falling_query: &Query<Entity, With<FallingBlock>>,
    ghost_query: &Query<Entity, With<GhostBlock>>,
    next_preview_query: &Query<Entity, With<NextPieceBlock>>,
) -> bool {
    let color = current_piece.piece_type.color();
    let positions = current_piece.block_positions();

    // Lock current piece blocks onto the board
    for &(x, y) in &positions {
        board.set(x, y, color);
    }

    // Remove falling block entities
    despawn_falling_blocks(commands, falling_query);
    despawn_ghost_blocks(commands, ghost_query);
    despawn_next_piece_preview(commands, next_preview_query);

    // Set up next piece
    let new_type = next_piece.piece_type;
    next_piece.piece_type = TetrominoType::random();

    *current_piece = CurrentPiece::new(new_type);

    // Check if the new piece can be placed (game over check)
    if !current_piece.can_place(board, current_piece.x, current_piece.y, current_piece.rotation) {
        return false;
    }

    // Spawn the new falling piece
    spawn_piece(commands, current_piece);
    spawn_ghost_piece(commands, current_piece, board);
    spawn_next_piece_preview(commands, next_piece);

    true
}

use crate::constants::{BLOCK_SIZE, SIDEBAR_X};
