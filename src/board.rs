use bevy::prelude::*;

use crate::constants::*;

/// Convert grid coordinates to world pixel coordinates.
/// Grid (0,0) is bottom-left of the board.
pub fn grid_to_world(grid_x: i32, grid_y: i32) -> Vec3 {
    let x = BOARD_OFFSET_X + (grid_x as f32 + 0.5) * BLOCK_SIZE;
    let y = BOARD_OFFSET_Y + (grid_y as f32 + 0.5) * BLOCK_SIZE;
    Vec3::new(x, y, 0.0)
}

/// Get the world position for a block sprite (with z-ordering).
pub fn block_world_position(grid_x: i32, grid_y: i32, z: f32) -> Vec3 {
    let mut pos = grid_to_world(grid_x, grid_y);
    pos.z = z;
    pos
}

/// Z-layer constants for rendering order.
pub const Z_GRID: f32 = 0.0;
pub const Z_GHOST: f32 = 1.0;
pub const Z_LOCKED: f32 = 2.0;
pub const Z_FALLING: f32 = 3.0;
