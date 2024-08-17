use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::collisions::LevelCollisions;
use crate::enemies::Cow;

const GRID_SIZE: i32 = 16;
const PLAYER_SPEED: f32 = 80.0;
const COW_HEALTH_HIT: f32 = 20.0;
const CORRECTION: f32 = 10.0;

#[derive(Default, Component)]
pub(crate) struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle("Characters/Basic Charakter Spritesheet.png", 16, 16, 64, 64, 0, 0, 68)]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Resource, Deref, DerefMut)]
pub(crate) struct PlayerStats {
    #[deref]
    pub(crate) health: f32,
    pub(crate) keys: usize,
    hit_timer: Timer,
    pub(crate) score: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        PlayerStats {
            health: 100.0,
            keys: 0,
            score: 0.0,
            hit_timer: Timer::from_seconds(0.250, TimerMode::Once),
        }
    }
}

#[derive(Default, Component)]
pub(crate) struct MainCamera;

pub(crate) fn move_player(
    mut player_grid_pos: Query<&mut GridCoords, With<Player>>,
    mut player_transform: Query<&mut Transform, With<Player>>,
    mut atlas_query: Query<&mut TextureAtlas, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    level_collisions: Res<LevelCollisions>,
    time: Res<Time>,
) {
    let mut direction = (0.0, 0.0);
    let mut x_correction = 0.0;
    let mut y_correction = 0.0;

    if let Ok(mut player_transform) = player_transform.get_single_mut() {
        let mut player_grid_pos = player_grid_pos.single_mut();
        let mut atlas = atlas_query.single_mut();

        if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
            direction.0 = -1.0;
            x_correction = -CORRECTION;
            atlas.index = 452;
        }

        if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
            direction.0 = 1.0;
            x_correction = CORRECTION;
            atlas.index = 644;
        }

        if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
            direction.1 = 1.0;
            y_correction = CORRECTION;
            atlas.index = 257;
        }

        if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
            direction.1 = -1.0;
            y_correction = -CORRECTION;
            atlas.index = 68;
        }

        if (direction.0 as i32).abs() + (direction.1 as i32).abs() > 1 {
            direction.0 *= 0.5;
            direction.1 *= 0.5;
        }

        let new_player_translation_x =
            player_transform.translation.x + direction.0 * PLAYER_SPEED * time.delta_seconds();
        let new_player_translation_y =
            player_transform.translation.y + direction.1 * PLAYER_SPEED * time.delta_seconds();
        let new_player_transform =
            Transform::from_xyz(new_player_translation_x, new_player_translation_y, 0.0);
        let new_player_grid_pos = bevy_ecs_ldtk::utils::translation_to_grid_coords(
            Vec2::new(
                new_player_transform.translation.truncate().x + x_correction,
                new_player_transform.translation.truncate().y + y_correction,
            ),
            IVec2::from((GRID_SIZE, GRID_SIZE)),
        );

        if !level_collisions.collision(&new_player_grid_pos) {
            *player_grid_pos = new_player_grid_pos;
            player_transform.translation = new_player_transform.translation;
        }
    }
}

pub(crate) fn update_player_stats(
    mut player_stats: ResMut<PlayerStats>,
    time: Res<Time>,
    player_grid_pos: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    cows_transforms: Query<&Transform, With<Cow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_selection: Res<LevelSelection>,
) {
    player_stats.hit_timer.tick(time.delta());

    if let Ok(player_grid_pos) = player_grid_pos.get_single() {
        for cow_transform in &cows_transforms {
            let cow_grid_pos = bevy_ecs_ldtk::utils::translation_to_grid_coords(
                cow_transform.translation.truncate(),
                IVec2::from((GRID_SIZE, GRID_SIZE)),
            );

            if *player_grid_pos == cow_grid_pos && player_stats.hit_timer.finished() {
                player_stats.health -= COW_HEALTH_HIT;
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/hit.ogg"),
                    ..default()
                });
                player_stats.hit_timer.reset();
            }
        }
    }

    let level = match level_selection.into_inner() {
        LevelSelection::Indices(indices) => indices.level,
        _ => 1,
    };

    player_stats.health -= time.delta_seconds() / (((level as f32 + 1.0) * 2.0) / 5.0);
    player_stats.score += time.delta_seconds();
}

pub(crate) fn center_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}
