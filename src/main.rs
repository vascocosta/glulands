#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
    window::WindowResolution,
};
use bevy_ecs_ldtk::prelude::*;

use collisions::{CollisionBundle, LevelCollisions};
use consts::*;
use enemies::CowBundle;
use gameplay::{BackgroundMusic, GameState, GoalBundle, PortalEntryBundle, PortalExitBundle};
use items::{BronzeBundle, CarrotBundle, KeyBundle};
use player::{MainCamera, PlayerBundle, PlayerStats};

mod collisions;
mod consts;
mod enemies;
mod gameplay;
mod items;
mod player;
mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some(CANVAS_NAME.into()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        resolution: WindowResolution::new(PHYSICAL_WIDTH, PHYSICAL_HEIGHT),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(LdtkPlugin)
        .insert_state(GameState::Menu)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(PlayerStats::default())
        .insert_resource(LevelCollisions::default())
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<GoalBundle>("Goal")
        .register_ldtk_entity::<KeyBundle>("Key")
        .register_ldtk_entity::<CarrotBundle>("Carrot")
        .register_ldtk_entity::<BronzeBundle>("Bronze")
        .register_ldtk_entity::<CowBundle>("Cow")
        .register_ldtk_entity::<PortalEntryBundle>("Portal_Entry")
        .register_ldtk_entity::<PortalExitBundle>("Portal_Exit")
        .register_ldtk_int_cell::<CollisionBundle>(1)
        .add_systems(Startup, setup)
        .add_systems(Update, (gameplay::toggle_state, gameplay::toggle_music))
        .add_systems(
            OnEnter(GameState::Menu),
            ui::setup_menu.run_if(in_state(GameState::Menu)),
        )
        .add_systems(
            OnExit(GameState::Menu),
            (ui::despawn_menu, setup_ldtk_world, ui::setup_status_bar).chain(),
        )
        .add_systems(OnEnter(GameState::PauseMenu), ui::setup_menu)
        .add_systems(OnExit(GameState::PauseMenu), ui::despawn_menu)
        .add_systems(OnEnter(GameState::GameOver), ui::setup_game_over)
        .add_systems(
            Update,
            (
                collisions::cache_collision_locations,
                (player::move_player, player::center_camera).chain(),
                player::update_player_stats,
                items::check_keys,
                items::check_carrots,
                items::check_bronze,
                enemies::patrol,
                gameplay::check_goal,
                gameplay::check_game_over,
                gameplay::check_cheats,
                ui::update_status_bar,
            )
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(
            Update,
            gameplay::check_portal_entry
                .run_if(not(in_state(GameState::Menu)))
                .run_if(not(in_state(GameState::PauseMenu)))
                .run_if(not(in_state(GameState::GameOver))),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = SCALE;
    camera.transform.translation.x = PHYSICAL_WIDTH / 8.0;
    camera.transform.translation.y = PHYSICAL_HEIGHT / 8.0;
    commands.spawn((camera, MainCamera));

    commands.spawn((
        AudioBundle {
            source: asset_server.load(BACKGROUND_MUSIC_PATH),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(BACKGROUND_MUSIC_VOLUME),
                paused: true,
                ..default()
            },
        },
        BackgroundMusic,
    ));
}

fn setup_ldtk_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load(LDTK_PROJECT_PATH),
        ..default()
    });
}
