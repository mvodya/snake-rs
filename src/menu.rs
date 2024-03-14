use bevy::prelude::*;

use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), show_main_menu)
            .add_systems(OnExit(GameState::MainMenu), hide_main_menu)
            .add_systems(OnEnter(GameState::GameOver), show_game_over)
            .add_systems(OnExit(GameState::GameOver), hide_game_over);
    }
}

#[derive(Component)]
struct UI;

fn show_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "SNAKE",
                TextStyle {
                    font: asset_server.load("fonts/Minimal3x5.ttf"),
                    font_size: 60.0,
                    ..default()
                },
            )
        ])
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            align_self: AlignSelf::Center,
            top: Val::Vh(30.),
            left: Val::Vw(50.),
            ..default()
        }),
        UI,
    ));
}

fn hide_main_menu() {}

fn show_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {}

fn hide_game_over() {}
