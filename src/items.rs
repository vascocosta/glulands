use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::consts::*;
use crate::player::{Player, PlayerStats};

#[derive(Default, Clone, Component)]
pub(crate) struct Key;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub(crate) struct KeyBundle {
    pub key: Key,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Clone, Component)]
pub(crate) struct Carrot;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub(crate) struct CarrotBundle {
    pub carrot: Carrot,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Clone, Component)]
pub(crate) struct Bronze;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub(crate) struct BronzeBundle {
    pub carrot: Bronze,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

pub(crate) fn check_keys(
    player_grid_pos: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    key_entity_grid_pos: Query<(Entity, &GridCoords), With<Key>>,
    mut player_stats: ResMut<PlayerStats>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Ok(player_grid_pos) = player_grid_pos.get_single() {
        for (key_entity, key_grid_pos) in &key_entity_grid_pos {
            if player_grid_pos == key_grid_pos {
                player_stats.keys += 1;
                commands.entity(key_entity).despawn();
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/item.ogg"),
                    ..default()
                });
            }
        }
    }
}

pub(crate) fn check_carrots(
    player_grid_pos: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    carrot_entity_grid_pos: Query<(Entity, &GridCoords), With<Carrot>>,
    mut player_stats: ResMut<PlayerStats>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Ok(player_grid_pos) = player_grid_pos.get_single() {
        for (carrot_entity, carrot_grid_pos) in &carrot_entity_grid_pos {
            if player_grid_pos == carrot_grid_pos {
                player_stats.health = (player_stats.health + CARROT_HEALTH).min(PLAYER_MAX_HEALTH);
                commands.entity(carrot_entity).despawn();
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/item.ogg"),
                    ..default()
                });
            }
        }
    }
}

pub(crate) fn check_bronze(
    player_grid_pos: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    bronze_entity_grid_pos: Query<(Entity, &GridCoords), With<Bronze>>,
    mut player_stats: ResMut<PlayerStats>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Ok(player_grid_pos) = player_grid_pos.get_single() {
        for (bronze_entity, bronze_grid_pos) in &bronze_entity_grid_pos {
            if player_grid_pos == bronze_grid_pos {
                player_stats.score += BRONZE_SCORE;
                commands.entity(bronze_entity).despawn();
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/item.ogg"),
                    ..default()
                });
            }
        }
    }
}
