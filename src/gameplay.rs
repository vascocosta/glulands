use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::consts::*;
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
pub(crate) struct PortalEntry;

#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct PortalEntryBundle {
    portal_entry: PortalEntry,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Component)]
pub(crate) struct PortalExit;

#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct PortalExitBundle {
    portal_exit: PortalExit,
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

pub(crate) fn check_portal_entry(
    mut player_transform: Query<&mut Transform, With<Player>>,
    mut player_grid_pos: Query<&mut GridCoords, With<Player>>,
    portal_entries: Query<&GridCoords, (With<PortalEntry>, Without<Player>)>,
    portal_exits: Query<&GridCoords, (With<PortalExit>, Without<Player>)>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for mut player_grid_pos in &mut player_grid_pos {
        if let Ok(portal_entry_grid_pos) = portal_entries.get_single() {
            let portal_exit_grid_pos = portal_exits.single();

            if *player_grid_pos == *portal_entry_grid_pos {
                *player_grid_pos = *portal_exit_grid_pos;

                let mut player_tranform = player_transform.single_mut();
                player_tranform.translation = bevy_ecs_ldtk::utils::grid_coords_to_translation(
                    *player_grid_pos,
                    IVec2::new(GRID_SIZE, GRID_SIZE),
                )
                .extend(0.0);

                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/teleport.ogg"),
                    ..default()
                });
            }
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

pub(crate) fn check_cheats(
    input: Res<ButtonInput<KeyCode>>,
    level_selection: ResMut<LevelSelection>,
) {
    if input.just_released(KeyCode::ControlRight) && input.just_released(KeyCode::KeyL) {
        let indices = match level_selection.into_inner() {
            LevelSelection::Indices(indices) => indices,
            _ => panic!("level selection should always be Indices in this game"),
        };
        indices.level += 1;
    }
}
