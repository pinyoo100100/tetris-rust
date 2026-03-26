use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;
use crate::systems::gravity::rebuild_locked_blocks;
use crate::systems::spawn;

/// System that handles keyboard input for piece movement.
pub fn input_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut board: ResMut<Board>,
    mut current_piece: ResMut<CurrentPiece>,
    mut next_piece: ResMut<NextPiece>,
    mut score: ResMut<Score>,
    mut drop_timer: ResMut<DropTimer>,
    mut next_state: ResMut<NextState<crate::GameState>>,
    falling_query: Query<Entity, With<FallingBlock>>,
    ghost_query: Query<Entity, With<GhostBlock>>,
    next_preview_query: Query<Entity, With<NextPieceBlock>>,
    locked_query: Query<Entity, With<LockedBlock>>,
) {
    let mut moved = false;

    // Move left
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        if current_piece.try_move(&board, -1, 0) {
            moved = true;
        }
    }

    // Move right
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        if current_piece.try_move(&board, 1, 0) {
            moved = true;
        }
    }

    // Rotate clockwise
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        if current_piece.try_rotate(&board) {
            moved = true;
        }
    }

    // Soft drop
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        if current_piece.try_move(&board, 0, -1) {
            score.add_soft_drop();
            moved = true;
        }
    }

    // Hard drop
    if keyboard.just_pressed(KeyCode::Space) {
        let drop_distance = (current_piece.y - current_piece.hard_drop_y(&board)).unsigned_abs();
        let target_y = current_piece.hard_drop_y(&board);
        current_piece.y = target_y;
        score.add_hard_drop(drop_distance);

        // Phase 1: Lock the piece onto the board
        spawn::lock_piece(
            &mut commands,
            &mut board,
            &current_piece,
            &falling_query,
            &ghost_query,
            &next_preview_query,
        );

        // Phase 2: Clear lines (before spawning next piece to avoid false game over)
        let lines = board.clear_full_rows();
        if lines > 0 {
            score.add_lines(lines);
            drop_timer.update_speed(score.level);
        }

        // Always rebuild locked block visuals after locking a piece
        rebuild_locked_blocks(&mut commands, &board, &locked_query);

        // Phase 3: Spawn next piece (game over check runs against cleared board)
        let can_continue = spawn::spawn_next_piece(
            &mut commands,
            &board,
            &mut current_piece,
            &mut next_piece,
        );

        if !can_continue {
            next_state.set(crate::GameState::GameOver);
        }

        // Reset the drop timer after hard drop
        drop_timer.timer.reset();
        return;
    }

    // Update visual positions if the piece moved
    if moved {
        spawn::despawn_falling_blocks(&mut commands, &falling_query);
        spawn::despawn_ghost_blocks(&mut commands, &ghost_query);
        spawn::spawn_piece(&mut commands, &current_piece);
        spawn::spawn_ghost_piece(&mut commands, &current_piece, &board);
    }
}

/// System that handles the restart key during game over.
pub fn restart_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        next_state.set(crate::GameState::Restarting);
    }
}

/// System that handles the restart key during gameplay.
pub fn restart_during_play_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        // Transition to Playing to reset
        next_state.set(crate::GameState::Restarting);
    }
}
