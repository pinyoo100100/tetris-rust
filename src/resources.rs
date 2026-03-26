use bevy::prelude::*;

use crate::constants::*;
use crate::tetromino::TetrominoType;

/// The game board state - a 2D grid tracking which cells are occupied.
#[derive(Resource)]
pub struct Board {
    /// cells[y][x] = Some(Color) if occupied, None if empty.
    /// Row 0 is the bottom, row GRID_HEIGHT-1 is the top.
    pub cells: [[Option<Color>; GRID_WIDTH]; GRID_HEIGHT],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [[None; GRID_WIDTH]; GRID_HEIGHT],
        }
    }
}

impl Board {
    /// Check if a cell is within bounds and empty.
    pub fn is_free(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= GRID_WIDTH as i32 || y < 0 || y >= GRID_HEIGHT as i32 {
            return false;
        }
        self.cells[y as usize][x as usize].is_none()
    }

    /// Set a cell to a color.
    pub fn set(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < GRID_WIDTH as i32 && y >= 0 && y < GRID_HEIGHT as i32 {
            self.cells[y as usize][x as usize] = Some(color);
        }
    }

    /// Clear a row.
    pub fn clear_row(&mut self, row: usize) {
        if row < GRID_HEIGHT {
            self.cells[row] = [None; GRID_WIDTH];
        }
    }

    /// Check if a row is completely filled.
    pub fn is_row_full(&self, row: usize) -> bool {
        if row >= GRID_HEIGHT {
            return false;
        }
        self.cells[row].iter().all(|cell| cell.is_some())
    }

    /// Find all full rows and return their indices (bottom to top).
    pub fn find_full_rows(&self) -> Vec<usize> {
        (0..GRID_HEIGHT)
            .filter(|&row| self.is_row_full(row))
            .collect()
    }

    /// Clear full rows and shift rows above down. Returns the number of cleared rows.
    pub fn clear_full_rows(&mut self) -> u32 {
        let full_rows = self.find_full_rows();
        let count = full_rows.len() as u32;

        // Process from bottom to top, adjusting indices as rows are removed.
        // After removing a row at index `row`, all rows above shift down by 1,
        // so subsequent full row indices need to be decremented by the number
        // of rows already removed.
        for (i, &row) in full_rows.iter().enumerate() {
            let adjusted_row = row - i;
            // Shift all rows above this one down
            for y in adjusted_row..GRID_HEIGHT - 1 {
                self.cells[y] = self.cells[y + 1];
            }
            // Clear the top row
            self.cells[GRID_HEIGHT - 1] = [None; GRID_WIDTH];
        }

        count
    }

    /// Reset the board to empty.
    pub fn reset(&mut self) {
        self.cells = [[None; GRID_WIDTH]; GRID_HEIGHT];
    }
}

/// The currently falling piece state.
#[derive(Resource)]
pub struct CurrentPiece {
    pub piece_type: TetrominoType,
    pub x: i32,
    pub y: i32,
    pub rotation: usize,
}

impl CurrentPiece {
    pub fn new(piece_type: TetrominoType) -> Self {
        Self {
            piece_type,
            x: SPAWN_X,
            y: SPAWN_Y,
            rotation: 0,
        }
    }

    /// Get the absolute grid positions of all 4 blocks.
    pub fn block_positions(&self) -> [(i32, i32); 4] {
        let offsets = self.piece_type.blocks(self.rotation);
        [
            (self.x + offsets[0].0, self.y + offsets[0].1),
            (self.x + offsets[1].0, self.y + offsets[1].1),
            (self.x + offsets[2].0, self.y + offsets[2].1),
            (self.x + offsets[3].0, self.y + offsets[3].1),
        ]
    }

    /// Check if the piece can be placed at the given position/rotation on the board.
    pub fn can_place(&self, board: &Board, x: i32, y: i32, rotation: usize) -> bool {
        let offsets = self.piece_type.blocks(rotation);
        offsets.iter().all(|&(dx, dy)| board.is_free(x + dx, y + dy))
    }

    /// Try to move the piece. Returns true if successful.
    pub fn try_move(&mut self, board: &Board, dx: i32, dy: i32) -> bool {
        let new_x = self.x + dx;
        let new_y = self.y + dy;
        if self.can_place(board, new_x, new_y, self.rotation) {
            self.x = new_x;
            self.y = new_y;
            true
        } else {
            false
        }
    }

    /// Try to rotate the piece clockwise. Returns true if successful.
    /// Implements basic wall kick: tries the rotation, then offset by +-1 on x.
    pub fn try_rotate(&mut self, board: &Board) -> bool {
        let new_rotation = (self.rotation + 1) % 4;

        // Try normal rotation
        if self.can_place(board, self.x, self.y, new_rotation) {
            self.rotation = new_rotation;
            return true;
        }

        // Wall kick: try shifting left
        if self.can_place(board, self.x - 1, self.y, new_rotation) {
            self.x -= 1;
            self.rotation = new_rotation;
            return true;
        }

        // Wall kick: try shifting right
        if self.can_place(board, self.x + 1, self.y, new_rotation) {
            self.x += 1;
            self.rotation = new_rotation;
            return true;
        }

        // Wall kick: try shifting left by 2 (for I piece)
        if self.piece_type == TetrominoType::I
            && self.can_place(board, self.x - 2, self.y, new_rotation)
        {
            self.x -= 2;
            self.rotation = new_rotation;
            return true;
        }

        // Wall kick: try shifting right by 2 (for I piece)
        if self.piece_type == TetrominoType::I
            && self.can_place(board, self.x + 2, self.y, new_rotation)
        {
            self.x += 2;
            self.rotation = new_rotation;
            return true;
        }

        false
    }

    /// Calculate the hard drop position (lowest valid Y).
    pub fn hard_drop_y(&self, board: &Board) -> i32 {
        let mut test_y = self.y;
        while self.can_place(board, self.x, test_y - 1, self.rotation) {
            test_y -= 1;
        }
        test_y
    }
}

/// The next piece to be spawned.
#[derive(Resource)]
pub struct NextPiece {
    pub piece_type: TetrominoType,
}

impl Default for NextPiece {
    fn default() -> Self {
        Self {
            piece_type: TetrominoType::random(),
        }
    }
}

/// Game score tracking.
#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
    pub lines_cleared: u32,
    pub level: u32,
}

impl Score {
    pub fn add_lines(&mut self, lines: u32) {
        self.lines_cleared += lines;
        self.value += match lines {
            1 => SCORE_SINGLE * (self.level + 1),
            2 => SCORE_DOUBLE * (self.level + 1),
            3 => SCORE_TRIPLE * (self.level + 1),
            4 => SCORE_TETRIS * (self.level + 1),
            _ => 0,
        };
        self.level = self.lines_cleared / LINES_PER_LEVEL;
    }

    pub fn add_soft_drop(&mut self) {
        self.value += SCORE_SOFT_DROP;
    }

    pub fn add_hard_drop(&mut self, rows: u32) {
        self.value += SCORE_HARD_DROP * rows;
    }

    pub fn reset(&mut self) {
        self.value = 0;
        self.lines_cleared = 0;
        self.level = 0;
    }
}

/// Timer resource for piece gravity.
#[derive(Resource)]
pub struct DropTimer {
    pub timer: Timer,
}

impl Default for DropTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(INITIAL_DROP_INTERVAL, TimerMode::Repeating),
        }
    }
}

impl DropTimer {
    pub fn update_speed(&mut self, level: u32) {
        let interval = (INITIAL_DROP_INTERVAL - level as f32 * SPEED_INCREASE_PER_LEVEL)
            .max(MIN_DROP_INTERVAL);
        self.timer.set_duration(std::time::Duration::from_secs_f32(interval));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_is_free_empty() {
        let board = Board::default();
        assert!(board.is_free(0, 0));
        assert!(board.is_free(5, 10));
        assert!(board.is_free(9, 19));
    }

    #[test]
    fn test_board_is_free_out_of_bounds() {
        let board = Board::default();
        assert!(!board.is_free(-1, 0));
        assert!(!board.is_free(0, -1));
        assert!(!board.is_free(10, 0));
        assert!(!board.is_free(0, 20));
    }

    #[test]
    fn test_board_set_and_check() {
        let mut board = Board::default();
        board.set(3, 5, Color::srgb(1.0, 0.0, 0.0));
        assert!(!board.is_free(3, 5));
        assert!(board.is_free(3, 4));
    }

    #[test]
    fn test_row_full_detection() {
        let mut board = Board::default();
        for x in 0..GRID_WIDTH {
            board.set(x as i32, 0, Color::srgb(1.0, 0.0, 0.0));
        }
        assert!(board.is_row_full(0));
        assert!(!board.is_row_full(1));
    }

    #[test]
    fn test_clear_full_rows_single() {
        let mut board = Board::default();
        // Fill row 0
        for x in 0..GRID_WIDTH {
            board.set(x as i32, 0, Color::srgb(1.0, 0.0, 0.0));
        }
        // Place a block on row 1
        board.set(5, 1, Color::srgb(0.0, 1.0, 0.0));

        let cleared = board.clear_full_rows();
        assert_eq!(cleared, 1);
        // The block that was on row 1 should now be on row 0
        assert!(!board.is_free(5, 0));
    }

    #[test]
    fn test_clear_full_rows_multiple() {
        let mut board = Board::default();
        // Fill rows 0 and 1
        for x in 0..GRID_WIDTH {
            board.set(x as i32, 0, Color::srgb(1.0, 0.0, 0.0));
            board.set(x as i32, 1, Color::srgb(0.0, 1.0, 0.0));
        }
        // Place a block on row 2
        board.set(3, 2, Color::srgb(0.0, 0.0, 1.0));

        let cleared = board.clear_full_rows();
        assert_eq!(cleared, 2);
        // The block that was on row 2 should now be on row 0
        assert!(!board.is_free(3, 0));
    }

    #[test]
    fn test_collision_detection() {
        let board = Board::default();
        let piece = CurrentPiece::new(TetrominoType::O);
        // Should be placeable at spawn
        assert!(piece.can_place(&board, piece.x, piece.y, piece.rotation));
    }

    #[test]
    fn test_hard_drop() {
        let board = Board::default();
        let piece = CurrentPiece::new(TetrominoType::I);
        let drop_y = piece.hard_drop_y(&board);
        // I piece in rotation 0 has blocks at y offsets of 0
        // So hard drop should land at y=0
        assert_eq!(drop_y, 0);
    }

    #[test]
    fn test_score_lines() {
        let mut score = Score::default();
        score.add_lines(1);
        assert_eq!(score.value, SCORE_SINGLE);
        assert_eq!(score.lines_cleared, 1);
    }

    #[test]
    fn test_score_tetris() {
        let mut score = Score::default();
        score.add_lines(4);
        assert_eq!(score.value, SCORE_TETRIS);
        assert_eq!(score.lines_cleared, 4);
    }

    #[test]
    fn test_level_progression() {
        let mut score = Score::default();
        for _ in 0..LINES_PER_LEVEL {
            score.add_lines(1);
        }
        assert_eq!(score.level, 1);
    }

    #[test]
    fn test_board_reset() {
        let mut board = Board::default();
        board.set(5, 5, Color::srgb(1.0, 0.0, 0.0));
        board.reset();
        assert!(board.is_free(5, 5));
    }

    #[test]
    fn test_try_move() {
        let board = Board::default();
        let mut piece = CurrentPiece::new(TetrominoType::O);
        let start_x = piece.x;
        assert!(piece.try_move(&board, 1, 0));
        assert_eq!(piece.x, start_x + 1);
    }

    #[test]
    fn test_try_rotate() {
        let board = Board::default();
        let mut piece = CurrentPiece::new(TetrominoType::T);
        let start_rotation = piece.rotation;
        assert!(piece.try_rotate(&board));
        assert_eq!(piece.rotation, (start_rotation + 1) % 4);
    }
}
