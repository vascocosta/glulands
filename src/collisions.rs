use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::collections::HashSet;

use crate::consts::*;

#[derive(Default, Component)]
pub(crate) struct Collision;

#[derive(Default, Bundle, LdtkIntCell)]
pub(crate) struct CollisionBundle {
    collision: Collision,
}

#[derive(Default, Resource)]
pub(crate) struct LevelCollisions {
    collision_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}

impl LevelCollisions {
    pub(crate) fn collision(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.collision_locations.contains(grid_coords)
    }
}

pub(crate) fn cache_collision_locations(
    mut level_collisions: ResMut<LevelCollisions>,
    mut level_events: EventReader<LevelEvent>,
    collision_grid_pos: Query<&GridCoords, With<Collision>>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for level_event in level_events.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project_entities = ldtk_project_entities
                .get_single()
                .expect("LdtkProject should be loaded when level is spawned");
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entities)
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");

            *level_collisions = LevelCollisions {
                collision_locations: collision_grid_pos.iter().copied().collect(),
                level_width: level.px_wid / GRID_SIZE,
                level_height: level.px_hei / GRID_SIZE,
            };
        }
    }
}
