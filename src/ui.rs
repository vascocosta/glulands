use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::player::PlayerStats;

const APP_NAME: &str = "GLULANDS";
const VERSION: &str = "v0.4.0";
const BAR_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
const TEXT_COLOR: Color = Color::srgb(0.1, 1.0, 0.7);
const GAME_OVER_COLOR: Color = Color::srgb(0.7, 0.2, 0.3);

#[derive(Component)]
pub(crate) struct Menu;

#[derive(Component)]
pub(crate) struct ScoreText;

#[derive(Component)]
pub(crate) struct HealthText;

#[derive(Component)]
pub(crate) struct KeysText;

#[derive(Component)]
pub(crate) struct LevelText;

pub(crate) fn setup_status_bar(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                left: Val::Percent(0.0),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(5.0),
                ..default()
            },
            background_color: BAR_COLOR.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "󱉾 0000",
                    TextStyle {
                        font: asset_server.load("fonts/FiraCodeNerdFont-Regular.ttf"),
                        font_size: 40.0,
                        color: TEXT_COLOR,
                    },
                ))
                .insert(ScoreText);
            parent
                .spawn(TextBundle::from_section(
                    " 100",
                    TextStyle {
                        font: asset_server.load("fonts/FiraCodeNerdFont-Regular.ttf"),
                        font_size: 40.0,
                        color: TEXT_COLOR,
                    },
                ))
                .insert(HealthText);
            parent
                .spawn(TextBundle::from_section(
                    "󱕴 00/00",
                    TextStyle {
                        font: asset_server.load("fonts/FiraCodeNerdFont-Regular.ttf"),
                        font_size: 40.0,
                        color: TEXT_COLOR,
                    },
                ))
                .insert(KeysText);
            parent
                .spawn(TextBundle::from_section(
                    "󰬓 1",
                    TextStyle {
                        font: asset_server.load("fonts/FiraCodeNerdFont-Regular.ttf"),
                        font_size: 40.0,
                        color: TEXT_COLOR,
                    },
                ))
                .insert(LevelText);
        });
}

pub(crate) fn update_status_bar(
    player_stats: Res<PlayerStats>,
    level_selection: ResMut<LevelSelection>,
    mut score_query: Query<&mut Text, With<ScoreText>>,
    mut health_query: Query<&mut Text, (With<HealthText>, Without<ScoreText>)>,
    mut keys_query: Query<&mut Text, (With<KeysText>, Without<ScoreText>, Without<HealthText>)>,
    mut level_query: Query<
        &mut Text,
        (
            With<LevelText>,
            Without<ScoreText>,
            Without<HealthText>,
            Without<KeysText>,
        ),
    >,
) {
    if let Ok(mut text) = score_query.get_single_mut() {
        text.sections[0].value = format!("󱉾 {:05.0}", player_stats.score);
    }

    if let Ok(mut text) = health_query.get_single_mut() {
        text.sections[0].value = format!(
            " {:03.0}",
            if player_stats.health > 0.0 {
                player_stats.health
            } else {
                0.0
            }
        );
    }

    let level = match level_selection.into_inner() {
        LevelSelection::Indices(indices) => indices.level + 1,
        _ => 1,
    };

    if let Ok(mut text) = keys_query.get_single_mut() {
        text.sections[0].value = format!("󱕴 {:02}/{:02}", player_stats.keys, level)
    }

    if let Ok(mut text) = level_query.get_single_mut() {
        text.sections[0].value = format!("󰬓 {:02}", level);
    }
}

pub(crate) fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                left: Val::Percent(25.0),
                top: Val::Percent(8.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(50.0),
                ..default()
            },
            background_color: BAR_COLOR.into(),
            border_radius: BorderRadius::all(Val::Percent(12.5)),
            ..default()
        })
        .insert(Menu)
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    format!("{} {}", APP_NAME, VERSION),
                    TextStyle {
                        font: asset_server.load("fonts/FiraCodeNerdFont-Regular.ttf"),
                        font_size: 80.0,
                        color: TEXT_COLOR,
                    },
                ))
                .insert(Menu);
        });
    commands
        .spawn(NodeBundle {
            style: Style {
                left: Val::Percent(25.0),
                top: Val::Percent(30.0),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                width: Val::Percent(50.0),
                padding: UiRect::all(Val::Percent(1.0)),
                ..default()
            },
            background_color: BAR_COLOR.into(),
            border_radius: BorderRadius::all(Val::Percent(12.5)),
            ..default()
        })
        .insert(Menu)
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "SPACE - START/PAUSE GAME",
                    TextStyle {
                        font: asset_server.load("fonts/FiraCodeNerdFont-Regular.ttf"),
                        font_size: 50.0,
                        color: TEXT_COLOR,
                    },
                ))
                .insert(Menu);
        });
    commands
        .spawn(NodeBundle {
            style: Style {
                left: Val::Percent(25.0),
                top: Val::Percent(40.0),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                width: Val::Percent(50.0),
                padding: UiRect::all(Val::Percent(1.0)),
                ..default()
            },
            background_color: BAR_COLOR.into(),
            border_radius: BorderRadius::all(Val::Percent(12.5)),
            ..default()
        })
        .insert(Menu)
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "AWSD - MOVE PLAYER AROUND",
                    TextStyle {
                        font: asset_server.load("fonts/FiraCodeNerdFont-Regular.ttf"),
                        font_size: 50.0,
                        color: TEXT_COLOR,
                    },
                ))
                .insert(Menu);
        });
    commands
        .spawn(NodeBundle {
            style: Style {
                left: Val::Percent(25.0),
                top: Val::Percent(50.0),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                width: Val::Percent(50.0),
                padding: UiRect::all(Val::Percent(1.0)),
                ..default()
            },
            background_color: BAR_COLOR.into(),
            border_radius: BorderRadius::all(Val::Percent(12.5)),
            ..default()
        })
        .insert(Menu)
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "M - TOGGLE MUSIC ON/OFF",
                    TextStyle {
                        font: asset_server.load("fonts/FiraCodeNerdFont-Regular.ttf"),
                        font_size: 50.0,
                        color: TEXT_COLOR,
                    },
                ))
                .insert(Menu);
        });
    commands
        .spawn(NodeBundle {
            style: Style {
                left: Val::Percent(25.0),
                top: Val::Percent(60.0),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                width: Val::Percent(50.0),
                padding: UiRect::all(Val::Percent(1.0)),
                ..default()
            },
            background_color: BAR_COLOR.into(),
            border_radius: BorderRadius::all(Val::Percent(12.5)),
            ..default()
        })
        .insert(Menu)
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "F5 - RESTART GAME",
                    TextStyle {
                        font: asset_server.load("fonts/FiraCodeNerdFont-Regular.ttf"),
                        font_size: 50.0,
                        color: TEXT_COLOR,
                    },
                ))
                .insert(Menu);
        });
    commands
        .spawn(NodeBundle {
            style: Style {
                left: Val::Percent(25.0),
                top: Val::Percent(70.0),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                width: Val::Percent(50.0),
                padding: UiRect::all(Val::Percent(1.0)),
                ..default()
            },
            background_color: BAR_COLOR.into(),
            border_radius: BorderRadius::all(Val::Percent(12.5)),
            ..default()
        })
        .insert(Menu)
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "Q - QUIT GAME",
                    TextStyle {
                        font: asset_server.load("fonts/FiraCodeNerdFont-Regular.ttf"),
                        font_size: 50.0,
                        color: TEXT_COLOR,
                    },
                ))
                .insert(Menu);
        });
}

pub(crate) fn despawn_menu(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub(crate) fn setup_game_over(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                left: Val::Percent(25.0),
                top: Val::Percent(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(50.0),
                ..default()
            },
            ..default()
        })
        .insert(Menu)
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "GAME OVER",
                    TextStyle {
                        font_size: 150.0,
                        color: GAME_OVER_COLOR,
                        ..default()
                    },
                ))
                .insert(Menu);
        });
}
