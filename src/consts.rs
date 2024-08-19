use bevy::color::Color;

pub(crate) const APP_NAME: &str = "GLULANDS";
pub(crate) const VERSION: &str = "v0.4.2";
pub(crate) const CANVAS_NAME: &str = "#glulands-canvas";

pub(crate) const PHYSICAL_WIDTH: f32 = 1600.0;
pub(crate) const PHYSICAL_HEIGHT: f32 = 900.0;
pub(crate) const SCALE: f32 = 0.2;
pub(crate) const GRID_SIZE: i32 = 16;
pub(crate) const LDTK_PROJECT_PATH: &str = "Glulands.ldtk";

pub(crate) const BACKGROUND_MUSIC_PATH: &str = "sounds/Intergalactic Odyssey.ogg";
pub(crate) const BACKGROUND_MUSIC_VOLUME: f32 = 0.3;
pub(crate) const HIT_SOUND_PATH: &str = "sounds/hit.ogg";

pub(crate) const BACKGROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub(crate) const BAR_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub(crate) const TEXT_COLOR: Color = Color::srgb(0.1, 1.0, 0.7);
pub(crate) const GAME_OVER_COLOR: Color = Color::srgb(0.7, 0.2, 0.3);

pub(crate) const PLAYER_ATLAS_INDEX_LEFT: usize = 452;
pub(crate) const PLAYER_ATLAS_INDEX_RIGHT: usize = 644;
pub(crate) const PLAYER_ATLAS_INDEX_UP: usize = 257;
pub(crate) const PLAYER_ATLAS_INDEX_DOWN: usize = 68;
pub(crate) const PLAYER_SPEED: f32 = 80.0;
pub(crate) const PLAYER_MAX_HEALTH: f32 = 100.0;
pub(crate) const COW_SPEED: f32 = 70.0;
pub(crate) const COW_HEALTH_HIT: f32 = 20.0;
pub(crate) const COW_SCORE_HIT: f32 = 100.0;
pub(crate) const BRONZE_SCORE: f32 = 50.0;
pub(crate) const CARROT_HEALTH: f32 = 25.0;

pub(crate) const CORRECTION: f32 = 10.0;
