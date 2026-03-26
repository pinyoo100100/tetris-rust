use crate::constants::*;
use crate::resources::Board;

/// Check and clear full lines from the board.
/// Returns the number of lines cleared.
pub fn check_and_clear_lines(board: &mut Board) -> u32 {
    board.clear_full_rows()
}

/// Calculate score for a given number of cleared lines at a given level.
pub fn calculate_line_score(lines: u32, level: u32) -> u32 {
    let base = match lines {
        1 => SCORE_SINGLE,
        2 => SCORE_DOUBLE,
        3 => SCORE_TRIPLE,
        4 => SCORE_TETRIS,
        _ => 0,
    };
    base * (level + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn test_no_lines_to_clear() {
        let mut board = Board::default();
        let cleared = check_and_clear_lines(&mut board);
        assert_eq!(cleared, 0);
    }

    #[test]
    fn test_single_line_clear() {
        let mut board = Board::default();
        for x in 0..GRID_WIDTH {
            board.set(x as i32, 0, Color::srgb(1.0, 0.0, 0.0));
        }
        let cleared = check_and_clear_lines(&mut board);
        assert_eq!(cleared, 1);
        // Row should now be empty
        assert!(board.is_free(0, 0));
    }

    #[test]
    fn test_double_line_clear() {
        let mut board = Board::default();
        for x in 0..GRID_WIDTH {
            board.set(x as i32, 0, Color::srgb(1.0, 0.0, 0.0));
            board.set(x as i32, 1, Color::srgb(0.0, 1.0, 0.0));
        }
        let cleared = check_and_clear_lines(&mut board);
        assert_eq!(cleared, 2);
    }

    #[test]
    fn test_tetris_clear() {
        let mut board = Board::default();
        for y in 0..4 {
            for x in 0..GRID_WIDTH {
                board.set(x as i32, y as i32, Color::srgb(1.0, 0.0, 0.0));
            }
        }
        let cleared = check_and_clear_lines(&mut board);
        assert_eq!(cleared, 4);
    }

    #[test]
    fn test_partial_line_not_cleared() {
        let mut board = Board::default();
        for x in 0..GRID_WIDTH - 1 {
            board.set(x as i32, 0, Color::srgb(1.0, 0.0, 0.0));
        }
        let cleared = check_and_clear_lines(&mut board);
        assert_eq!(cleared, 0);
    }

    #[test]
    fn test_blocks_shift_down_after_clear() {
        let mut board = Board::default();
        // Fill row 0 completely
        for x in 0..GRID_WIDTH {
            board.set(x as i32, 0, Color::srgb(1.0, 0.0, 0.0));
        }
        // Place a single block on row 1
        board.set(5, 1, Color::srgb(0.0, 1.0, 0.0));

        check_and_clear_lines(&mut board);

        // The block from row 1 should have shifted down to row 0
        assert!(!board.is_free(5, 0));
        assert!(board.is_free(5, 1));
    }

    #[test]
    fn test_score_calculation() {
        assert_eq!(calculate_line_score(1, 0), SCORE_SINGLE);
        assert_eq!(calculate_line_score(2, 0), SCORE_DOUBLE);
        assert_eq!(calculate_line_score(3, 0), SCORE_TRIPLE);
        assert_eq!(calculate_line_score(4, 0), SCORE_TETRIS);

        // Level multiplier
        assert_eq!(calculate_line_score(1, 1), SCORE_SINGLE * 2);
        assert_eq!(calculate_line_score(4, 2), SCORE_TETRIS * 3);
    }

    #[test]
    fn test_non_contiguous_line_clear() {
        let mut board = Board::default();
        // Fill rows 0 and 2 (skip row 1)
        for x in 0..GRID_WIDTH {
            board.set(x as i32, 0, Color::srgb(1.0, 0.0, 0.0));
            board.set(x as i32, 2, Color::srgb(0.0, 1.0, 0.0));
        }
        // Put a partial block on row 1
        board.set(3, 1, Color::srgb(0.0, 0.0, 1.0));

        let cleared = check_and_clear_lines(&mut board);
        assert_eq!(cleared, 2);
    }
}
