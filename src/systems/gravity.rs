use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;
use crate::systems::spawn;

/// System that handles automatic piece falling (gravity).
pub fn gravity_system(
    mut commands: Commands,
    time: Res<Time>,
    mut drop_timer: ResMut<DropTimer>,
    mut board: ResMut<Board>,
    mut current_piece: ResMut<CurrentPiece>,
    mut next_piece: ResMut<NextPiece>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<crate::GameState>>,
    falling_query: Query<Entity, With<FallingBlock>>,
    ghost_query: Query<Entity, With<GhostBlock>>,
    next_preview_query: Query<Entity, With<NextPieceBlock>>,
    locked_query: Query<Entity, With<LockedBlock>>,
) {
    drop_timer.timer.tick(time.delta());

    if !drop_timer.timer.just_finished() {
        return;
    }

    // Try to move the piece down
    if current_piece.try_move(&board, 0, -1) {
        // Update falling block positions
        update_falling_positions(&mut commands, &current_piece, &board, &falling_query, &ghost_query);
    } else {
        // Lock the piece and spawn a new one
        let can_continue = spawn::lock_and_spawn(
            &mut commands,
            &mut board,
            &mut current_piece,
            &mut next_piece,
            &falling_query,
            &ghost_query,
            &next_preview_query,
        );

        // Clear lines
        let lines = board.clear_full_rows();
        if lines > 0 {
            score.add_lines(lines);
            drop_timer.update_speed(score.level);
            // Rebuild locked block visuals
            rebuild_locked_blocks(&mut commands, &board, &locked_query);
        }

        if !can_continue {
            next_state.set(crate::GameState::GameOver);
        }
    }
}

/// Update the visual positions of falling blocks to match the current piece state.
fn update_falling_positions(
    commands: &mut Commands,
    current_piece: &CurrentPiece,
    board: &Board,
    falling_query: &Query<Entity, With<FallingBlock>>,
    ghost_query: &Query<Entity, With<GhostBlock>>,
) {
    // Despawn and respawn falling blocks at new positions
    spawn::despawn_falling_blocks(commands, falling_query);
    spawn::despawn_ghost_blocks(commands, ghost_query);
    spawn::spawn_piece(commands, current_piece);
    spawn::spawn_ghost_piece(commands, current_piece, board);
}

/// Rebuild all locked block entities to reflect the current board state.
pub fn rebuild_locked_blocks(
    commands: &mut Commands,
    board: &Board,
    locked_query: &Query<Entity, With<LockedBlock>>,
) {
    use crate::board::*;
    use crate::constants::*;

    // Remove existing locked blocks
    for entity in locked_query.iter() {
        commands.entity(entity).despawn();
    }

    // Spawn new locked blocks based on board state
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if let Some(color) = board.cells[y][x] {
                commands.spawn((
                    Sprite {
                        color,
                        custom_size: Some(Vec2::new(BLOCK_SIZE - 2.0, BLOCK_SIZE - 2.0)),
                        ..default()
                    },
                    Transform::from_translation(block_world_position(
                        x as i32, y as i32, Z_LOCKED,
                    )),
                    LockedBlock,
                    GridPosition::new(x as i32, y as i32),
                ));
            }
        }
    }
}
