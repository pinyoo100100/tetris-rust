use bevy::prelude::*;
use rand::Rng;

/// The seven standard Tetris piece types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl TetrominoType {
    /// Returns all tetromino types.
    pub fn all() -> &'static [TetrominoType] {
        &[
            TetrominoType::I,
            TetrominoType::O,
            TetrominoType::T,
            TetrominoType::S,
            TetrominoType::Z,
            TetrominoType::J,
            TetrominoType::L,
        ]
    }

    /// Returns a random tetromino type.
    pub fn random() -> Self {
        let types = Self::all();
        let mut rng = rand::thread_rng();
        types[rng.gen_range(0..types.len())]
    }

    /// Returns the color associated with each piece type.
    pub fn color(&self) -> Color {
        match self {
            TetrominoType::I => Color::srgb(0.0, 1.0, 1.0),   // Cyan
            TetrominoType::O => Color::srgb(1.0, 1.0, 0.0),   // Yellow
            TetrominoType::T => Color::srgb(0.6, 0.0, 0.8),   // Purple
            TetrominoType::S => Color::srgb(0.0, 1.0, 0.0),   // Green
            TetrominoType::Z => Color::srgb(1.0, 0.0, 0.0),   // Red
            TetrominoType::J => Color::srgb(0.0, 0.0, 1.0),   // Blue
            TetrominoType::L => Color::srgb(1.0, 0.5, 0.0),   // Orange
        }
    }

    /// Returns the ghost piece color (same hue but more transparent/darker).
    pub fn ghost_color(&self) -> Color {
        match self {
            TetrominoType::I => Color::srgba(0.0, 1.0, 1.0, 0.3),
            TetrominoType::O => Color::srgba(1.0, 1.0, 0.0, 0.3),
            TetrominoType::T => Color::srgba(0.6, 0.0, 0.8, 0.3),
            TetrominoType::S => Color::srgba(0.0, 1.0, 0.0, 0.3),
            TetrominoType::Z => Color::srgba(1.0, 0.0, 0.0, 0.3),
            TetrominoType::J => Color::srgba(0.0, 0.0, 1.0, 0.3),
            TetrominoType::L => Color::srgba(1.0, 0.5, 0.0, 0.3),
        }
    }

    /// Returns the block offsets for each rotation state.
    /// Each tetromino has 4 rotation states (0, 1, 2, 3).
    /// Offsets are relative to the pivot point.
    pub fn blocks(&self, rotation: usize) -> [(i32, i32); 4] {
        let rotation = rotation % 4;
        match self {
            TetrominoType::I => match rotation {
                0 => [(-1, 0), (0, 0), (1, 0), (2, 0)],
                1 => [(0, -1), (0, 0), (0, 1), (0, 2)],
                2 => [(-1, 1), (0, 1), (1, 1), (2, 1)],
                3 => [(1, -1), (1, 0), (1, 1), (1, 2)],
                _ => unreachable!(),
            },
            TetrominoType::O => [
                (0, 0), (1, 0), (0, 1), (1, 1),
            ],
            TetrominoType::T => match rotation {
                0 => [(-1, 0), (0, 0), (1, 0), (0, 1)],
                1 => [(0, -1), (0, 0), (0, 1), (1, 0)],
                2 => [(-1, 0), (0, 0), (1, 0), (0, -1)],
                3 => [(0, -1), (0, 0), (0, 1), (-1, 0)],
                _ => unreachable!(),
            },
            TetrominoType::S => match rotation {
                0 => [(-1, 0), (0, 0), (0, 1), (1, 1)],
                1 => [(0, 1), (0, 0), (1, 0), (1, -1)],
                2 => [(-1, -1), (0, -1), (0, 0), (1, 0)],
                3 => [(-1, 1), (-1, 0), (0, 0), (0, -1)],
                _ => unreachable!(),
            },
            TetrominoType::Z => match rotation {
                0 => [(-1, 1), (0, 1), (0, 0), (1, 0)],
                1 => [(0, -1), (0, 0), (1, 0), (1, 1)],
                2 => [(-1, 0), (0, 0), (0, -1), (1, -1)],
                3 => [(-1, -1), (-1, 0), (0, 0), (0, 1)],
                _ => unreachable!(),
            },
            TetrominoType::J => match rotation {
                0 => [(-1, 1), (-1, 0), (0, 0), (1, 0)],
                1 => [(0, -1), (0, 0), (0, 1), (1, 1)],
                2 => [(-1, 0), (0, 0), (1, 0), (1, -1)],
                3 => [(-1, -1), (0, -1), (0, 0), (0, 1)],
                _ => unreachable!(),
            },
            TetrominoType::L => match rotation {
                0 => [(-1, 0), (0, 0), (1, 0), (1, 1)],
                1 => [(0, -1), (0, 0), (0, 1), (1, -1)],
                2 => [(-1, -1), (-1, 0), (0, 0), (1, 0)],
                3 => [(-1, 1), (0, -1), (0, 0), (0, 1)],
                _ => unreachable!(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_types_count() {
        assert_eq!(TetrominoType::all().len(), 7);
    }

    #[test]
    fn test_each_piece_has_4_blocks() {
        for piece_type in TetrominoType::all() {
            for rotation in 0..4 {
                let blocks = piece_type.blocks(rotation);
                assert_eq!(blocks.len(), 4, "{piece_type:?} rotation {rotation} should have 4 blocks");
            }
        }
    }

    #[test]
    fn test_o_piece_rotation_invariant() {
        let o = TetrominoType::O;
        let base = o.blocks(0);
        for rotation in 1..4 {
            assert_eq!(base, o.blocks(rotation), "O piece should be same in all rotations");
        }
    }

    #[test]
    fn test_rotation_wraps_around() {
        for piece_type in TetrominoType::all() {
            assert_eq!(
                piece_type.blocks(0),
                piece_type.blocks(4),
                "{piece_type:?} rotation 4 should equal rotation 0"
            );
        }
    }
}
