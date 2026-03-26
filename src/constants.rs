/// Grid dimensions
pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 20;

/// Size of each block in pixels
pub const BLOCK_SIZE: f32 = 30.0;

/// Window dimensions
pub const WINDOW_WIDTH: f32 = 600.0;
pub const WINDOW_HEIGHT: f32 = 700.0;

/// Board offset (pixels from center) to position the grid
pub const BOARD_OFFSET_X: f32 = -((GRID_WIDTH as f32) * BLOCK_SIZE) / 2.0;
pub const BOARD_OFFSET_Y: f32 = -((GRID_HEIGHT as f32) * BLOCK_SIZE) / 2.0;

/// Sidebar offset for UI elements (next piece, score)
pub const SIDEBAR_X: f32 = (GRID_WIDTH as f32) * BLOCK_SIZE / 2.0 + 80.0;

/// Initial drop interval in seconds
pub const INITIAL_DROP_INTERVAL: f32 = 0.8;

/// Minimum drop interval
pub const MIN_DROP_INTERVAL: f32 = 0.05;

/// Speed increase per level (seconds subtracted from interval)
pub const SPEED_INCREASE_PER_LEVEL: f32 = 0.05;

/// Lines needed to advance one level
pub const LINES_PER_LEVEL: u32 = 10;

/// Spawn position for new pieces
/// Y is set to GRID_HEIGHT - 2 so pieces with upward offsets (e.g. T, J, L) stay in bounds.
pub const SPAWN_X: i32 = (GRID_WIDTH as i32) / 2 - 1;
pub const SPAWN_Y: i32 = (GRID_HEIGHT as i32) - 2;

/// Scoring values
pub const SCORE_SINGLE: u32 = 100;
pub const SCORE_DOUBLE: u32 = 300;
pub const SCORE_TRIPLE: u32 = 500;
pub const SCORE_TETRIS: u32 = 800;
pub const SCORE_SOFT_DROP: u32 = 1;
pub const SCORE_HARD_DROP: u32 = 2;

/// Grid border color (RGBA)
pub const GRID_BORDER_COLOR: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

/// Background color for empty cells
pub const EMPTY_CELL_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
