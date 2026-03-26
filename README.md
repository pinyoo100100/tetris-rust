# Tetris - Rust & Bevy

A fully functional Tetris game built with Rust and the Bevy game engine, following ECS (Entity Component System) architecture principles.

## Features

- All 7 standard tetromino pieces (I, O, T, S, Z, J, L)
- 10x20 grid-based board
- Piece rotation with wall kick support
- Collision detection (walls, floor, piece-to-piece)
- Line clearing with row shifting
- Score system with level multipliers
- Increasing speed over time (level-based)
- Game over detection and restart
- Ghost piece preview (shows landing position)
- Next piece preview
- Clean UI with score, level, and lines display

## Controls

| Key | Action |
|-----|--------|
| Left Arrow | Move piece left |
| Right Arrow | Move piece right |
| Up Arrow | Rotate piece clockwise |
| Down Arrow | Soft drop (move down one row) |
| Space | Hard drop (instantly drop to bottom) |
| R | Restart game |

## Prerequisites

- [Rust](https://rustup.rs/) (stable, 1.82+)
- System dependencies (Linux):
  ```bash
  sudo apt-get install pkg-config libasound2-dev libudev-dev libxkbcommon-dev
  ```

## Build & Run

```bash
# Build the project
cargo build --release

# Run the game
cargo run --release
```

## Run Tests

```bash
cargo test
```

## Project Structure

```
src/
├── main.rs              # App setup, state management, Bevy plugin configuration
├── constants.rs         # Game constants (grid size, speeds, scoring, colors)
├── components.rs        # ECS components (FallingBlock, LockedBlock, GhostBlock, etc.)
├── resources.rs         # ECS resources (Board, CurrentPiece, Score, DropTimer)
├── tetromino.rs         # Tetromino type definitions, shapes, rotations, colors
├── board.rs             # Grid-to-world coordinate mapping, z-layer constants
└── systems/
    ├── mod.rs           # Module declarations
    ├── setup.rs         # Camera, grid, and UI initialization systems
    ├── spawn.rs         # Piece spawning, ghost piece, next piece preview
    ├── gravity.rs       # Automatic piece falling (tick-based gravity)
    ├── input.rs         # Keyboard input handling (movement, rotation, drops)
    ├── line_clear.rs    # Line detection, clearing, and score calculation
    └── rendering.rs     # Score UI updates, game over overlay, cleanup
```

## Architecture

The game follows Bevy's ECS architecture:

- **Components**: `FallingBlock`, `LockedBlock`, `GhostBlock`, `GridPosition`, `ScoreText`, etc.
- **Resources**: `Board` (grid state), `CurrentPiece`, `NextPiece`, `Score`, `DropTimer`
- **Systems**: Modular systems for gravity, input, rendering, spawning, and line clearing
- **States**: `Playing`, `GameOver`, `Restarting` with proper state transitions

## Scoring

| Lines Cleared | Base Points |
|---------------|-------------|
| 1 (Single) | 100 |
| 2 (Double) | 300 |
| 3 (Triple) | 500 |
| 4 (Tetris) | 800 |

Points are multiplied by `(level + 1)`. Soft drops earn 1 point per row, hard drops earn 2 points per row.

## License

MIT
