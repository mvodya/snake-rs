use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::GameState;

use super::PlayerStats;

pub struct GameInterfacePlugin;

impl Plugin for GameInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(OnEnter(GameState::InGame), setup)
            .add_systems(OnExit(GameState::InGame), despawn_all_ui)
            .add_systems(
                Update,
                (fps_text_update, score_text_update).run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Component)]
struct HUD;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct ScoreText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "SCORE: ",
                TextStyle {
                    font: asset_server.load("fonts/Minimal3x5.ttf"),
                    font_size: 30.0,
                    ..default()
                },
            ),
            TextSection::new(
                "???",
                TextStyle {
                    font: asset_server.load("fonts/Minimal3x5.ttf"),
                    font_size: 30.0,
                    ..default()
                },
            ),
        ])
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        HUD,
        ScoreText,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/Minimal3x5.ttf"),
                    font_size: 30.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/Minimal3x5.ttf"),
                font_size: 30.0,
                ..default()
            }),
        ])
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        }),
        HUD,
        FpsText,
    ));
}

fn despawn_all_ui(mut commands: Commands, query: Query<Entity, With<HUD>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn fps_text_update(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

fn score_text_update(stats: Res<PlayerStats>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in &mut query {
        text.sections[1].value = format!("{}", stats.score);
    }
}
