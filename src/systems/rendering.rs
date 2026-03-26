use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;

/// Query filter type aliases for score UI to reduce type complexity.
type ScoreFilter = (With<ScoreText>, Without<LevelText>, Without<LinesText>);
type LevelFilter = (With<LevelText>, Without<ScoreText>, Without<LinesText>);
type LinesFilter = (With<LinesText>, Without<ScoreText>, Without<LevelText>);

/// System to update the score UI text.
pub fn update_score_ui(
    score: Res<Score>,
    mut score_query: Query<&mut Text, ScoreFilter>,
    mut level_query: Query<&mut Text, LevelFilter>,
    mut lines_query: Query<&mut Text, LinesFilter>,
) {
    if !score.is_changed() {
        return;
    }

    for mut text in &mut score_query {
        **text = score.value.to_string();
    }

    for mut text in &mut level_query {
        **text = score.level.to_string();
    }

    for mut text in &mut lines_query {
        **text = score.lines_cleared.to_string();
    }
}

/// System to show the game over overlay.
pub fn show_game_over_overlay(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            GameOverOverlay,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.2, 0.2)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new("Press R to Restart"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

/// System to remove the game over overlay.
pub fn hide_game_over_overlay(
    mut commands: Commands,
    overlay_query: Query<Entity, With<GameOverOverlay>>,
) {
    for entity in &overlay_query {
        commands.entity(entity).despawn_recursive();
    }
}

/// System to clean up all game entities when restarting.
pub fn cleanup_game(
    mut commands: Commands,
    falling_query: Query<Entity, With<FallingBlock>>,
    locked_query: Query<Entity, With<LockedBlock>>,
    ghost_query: Query<Entity, With<GhostBlock>>,
    next_preview_query: Query<Entity, With<NextPieceBlock>>,
    overlay_query: Query<Entity, With<GameOverOverlay>>,
    mut board: ResMut<Board>,
    mut score: ResMut<Score>,
    mut current_piece: ResMut<CurrentPiece>,
    mut next_piece: ResMut<NextPiece>,
    mut drop_timer: ResMut<DropTimer>,
) {
    // Despawn all game entities
    for entity in falling_query.iter().chain(locked_query.iter()).chain(ghost_query.iter()).chain(next_preview_query.iter()) {
        commands.entity(entity).despawn();
    }

    for entity in &overlay_query {
        commands.entity(entity).despawn_recursive();
    }

    // Reset resources
    board.reset();
    score.reset();
    *current_piece = CurrentPiece::new(crate::tetromino::TetrominoType::random());
    *next_piece = NextPiece::default();
    *drop_timer = DropTimer::default();
}
