mod board;
mod components;
mod constants;
mod resources;
mod systems;
mod tetromino;

use bevy::prelude::*;
use bevy::window::WindowResolution;

use resources::*;
use tetromino::TetrominoType;

/// Game states.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
    Restarting,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris - Rust & Bevy".to_string(),
                resolution: WindowResolution::new(
                    constants::WINDOW_WIDTH,
                    constants::WINDOW_HEIGHT,
                ),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .init_state::<GameState>()
        // Initialize resources
        .insert_resource(Board::default())
        .insert_resource(CurrentPiece::new(TetrominoType::random()))
        .insert_resource(NextPiece::default())
        .insert_resource(Score::default())
        .insert_resource(DropTimer::default())
        // Startup systems (run once)
        .add_systems(
            Startup,
            (
                systems::setup::setup_camera,
                systems::setup::setup_grid,
                systems::setup::setup_ui,
                systems::spawn::spawn_initial_piece,
            ),
        )
        // Playing state systems
        .add_systems(
            Update,
            (
                systems::input::input_system,
                systems::gravity::gravity_system,
                systems::rendering::update_score_ui,
                systems::input::restart_during_play_system,
            )
                .run_if(in_state(GameState::Playing)),
        )
        // Game over state systems
        .add_systems(
            Update,
            systems::input::restart_input_system.run_if(in_state(GameState::GameOver)),
        )
        // State transition handlers
        .add_systems(
            OnEnter(GameState::GameOver),
            systems::rendering::show_game_over_overlay,
        )
        .add_systems(
            OnEnter(GameState::Restarting),
            (systems::rendering::cleanup_game, transition_to_playing).chain(),
        )
        .add_systems(OnEnter(GameState::Playing), on_enter_playing)
        .run();
}

/// Transition from Restarting to Playing state.
fn transition_to_playing(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}

/// Called when entering the Playing state (handles fresh start and restart).
fn on_enter_playing(
    mut commands: Commands,
    current_piece: Res<CurrentPiece>,
    next_piece: Res<NextPiece>,
    board: Res<Board>,
    falling_query: Query<Entity, With<components::FallingBlock>>,
) {
    // Only spawn pieces if there are none (restart scenario)
    if falling_query.iter().count() == 0 {
        systems::spawn::spawn_piece(&mut commands, &current_piece);
        systems::spawn::spawn_ghost_piece(&mut commands, &current_piece, &board);
        systems::spawn::spawn_next_piece_preview(&mut commands, &next_piece);
    }
}
