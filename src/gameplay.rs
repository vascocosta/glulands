use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::player::{Player, PlayerStats};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum GameState {
    Menu,
    PauseMenu,
    Running,
    GameOver,
}

#[derive(Default, Component)]
pub(crate) struct Goal;

#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct GoalBundle {
    goal: Goal,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Component)]
pub(crate) struct BackgroundMusic;

pub(crate) fn check_goal(
    players: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    goals: Query<&GridCoords, With<Goal>>,
    mut player_stats: ResMut<PlayerStats>,
    level_selection: ResMut<LevelSelection>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if players
        .iter()
        .zip(goals.iter())
        .any(|(player_grid_coords, goal_grid_coords)| player_grid_coords == goal_grid_coords)
    {
        let indices = match level_selection.into_inner() {
            LevelSelection::Indices(indices) => indices,
            _ => panic!("level selection should always be Indices in this game"),
        };

        if player_stats.keys == indices.level + 1 {
            player_stats.keys = 0;
            player_stats.health = 100.0;
            indices.level += 1;
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/level.ogg"),
                ..default()
            });
        }
    }
}

pub(crate) fn check_game_over(
    player_state: Res<PlayerStats>,
    mut next_state: ResMut<NextState<GameState>>,
    mut music_query: Query<&AudioSink, With<BackgroundMusic>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if player_state.health <= 0.0 {
        next_state.set(GameState::GameOver);
        let audio_sink = music_query.single_mut();

        if !audio_sink.is_paused() {
            audio_sink.pause();
        }

        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/lost.ogg"),
            ..default()
        });
    }
}

pub(crate) fn toggle_state(
    input: Res<ButtonInput<KeyCode>>,
    game_state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut music_query: Query<&AudioSink, With<BackgroundMusic>>,
) {
    if input.just_released(KeyCode::KeyP) || input.just_released(KeyCode::Space) {
        let audio_sink = music_query.single_mut();

        match game_state.get() {
            GameState::Menu => {
                next_state.set(GameState::Running);
                audio_sink.play();
            }
            GameState::Running => {
                next_state.set(GameState::PauseMenu);

                if !audio_sink.is_paused() {
                    audio_sink.pause();
                }
            }
            GameState::PauseMenu => {
                next_state.set(GameState::Running);

                if audio_sink.is_paused() {
                    audio_sink.play();
                }
            }
            _ => (),
        }
    }
}

pub(crate) fn toggle_music(
    input: Res<ButtonInput<KeyCode>>,
    mut music_query: Query<&AudioSink, With<BackgroundMusic>>,
) {
    if input.just_pressed(KeyCode::KeyM) {
        let audio_sink = music_query.single_mut();
        audio_sink.toggle();
    }
}
