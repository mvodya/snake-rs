use bevy::prelude::*;

use crate::{game::PlayerStats, GameState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), show_main_menu)
            .add_systems(
                Update,
                (animate_logo_text, anykey_check).run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), hide_ui)
            .add_systems(OnEnter(GameState::GameOver), show_game_over)
            .add_systems(
                Update,
                (animate_game_over_text, anykey_check_game_over)
                    .run_if(in_state(GameState::GameOver)),
            )
            .add_systems(OnExit(GameState::GameOver), hide_ui);
    }
}

#[derive(Component)]
struct UI;

#[derive(Component)]
struct LogoText(f64);

#[derive(Component)]
struct GameOverText(f64);

fn show_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    justify_self: JustifySelf::Center,
                    top: Val::Vh(40.),
                    ..default()
                },
                ..default()
            },
            UI,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/Minimal5x7.ttf"),
                            font_size: 120.,
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/Minimal3x5.ttf"),
                            font_size: 20.,
                            ..default()
                        },
                    ),
                ])
                .with_text_justify(JustifyText::Center),
                LogoText(0.),
            ));
        });
}

const LOGO_TEXT: &str = "SNAKE-RS";
const LOGO_SUB_TEXT: &str = "\n\npress any key to start playing";

fn animate_logo_text(
    time: Res<Time>,
    mut query: Query<(&mut LogoText, &mut Text), With<LogoText>>,
) {
    for (mut timer, mut text) in &mut query {
        // Calculate time
        timer.0 += time.delta_seconds_f64();

        // Start after 1s
        if timer.0 < 1. {
            continue;
        }

        if timer.0 < 2. {
            let logo_step_progress = ((timer.0 - 1.) * LOGO_TEXT.len() as f64) as usize;
            text.sections[0].value = LOGO_TEXT[0..logo_step_progress].into();
            text.sections[0].value += "_";
        } else {
            text.sections[0].value = LOGO_TEXT.into();
        }

        // Start after 3s
        if timer.0 < 3. {
            continue;
        }

        if timer.0 < 4. {
            let logo_step_progress = ((timer.0 - 3.) * LOGO_SUB_TEXT.len() as f64) as usize;
            text.sections[1].value = LOGO_SUB_TEXT[0..logo_step_progress].into();
            // Fix glitch
            if timer.0 > 3.1 {
                text.sections[1].value += "_";
            }
        } else {
            text.sections[1].value = LOGO_SUB_TEXT.into();
        }
    }
}

fn anykey_check(
    mut next_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepad_inputs: Res<ButtonInput<GamepadButton>>,
) {
    if keys.get_just_pressed().len() > 0 || gamepad_inputs.get_just_pressed().len() > 0 {
        next_state.set(GameState::InGame);
    }
}

fn show_game_over(mut commands: Commands, asset_server: Res<AssetServer>, stats: Res<PlayerStats>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    justify_self: JustifySelf::Center,
                    top: Val::Vh(40.),
                    ..default()
                },
                ..default()
            },
            UI,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "GAME OVER",
                        TextStyle {
                            font: asset_server.load("fonts/Minimal5x7.ttf"),
                            font_size: 120.,
                            color: Color::Rgba {
                                red: 1.0,
                                green: 0.,
                                blue: 0.,
                                alpha: 1.,
                            },
                            ..default()
                        },
                    ),
                    TextSection::new(
                        format!(
                            "\n\nscore: {}   //   food eaten: {}   //   distance traveled: {}",
                            stats.score, stats.food_eaten, stats.distance_traveled
                        ),
                        TextStyle {
                            font: asset_server.load("fonts/Minimal5x7.ttf"),
                            font_size: 20.,
                            ..default()
                        },
                    ),
                    TextSection::new(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/Minimal5x7.ttf"),
                            font_size: 20.,
                            ..default()
                        },
                    ),
                ])
                .with_text_justify(JustifyText::Center),
                GameOverText(0.),
            ));
        });
}

const GAME_OVER_SUB_TEXT: &str = "\n\npress any key to restart game...";

fn animate_game_over_text(
    time: Res<Time>,
    mut query: Query<(&mut GameOverText, &mut Text), With<GameOverText>>,
) {
    for (mut timer, mut text) in &mut query {
        // Calculate time
        timer.0 += time.delta_seconds_f64();

        // Start after 1s
        if timer.0 < 1. {
            continue;
        }

        if timer.0 < 2. {
            let step_progress = ((timer.0 - 1.) * GAME_OVER_SUB_TEXT.len() as f64) as usize;
            text.sections[2].value = GAME_OVER_SUB_TEXT[0..step_progress].into();
            // Fix glitch
            if timer.0 > 1.1 {
                text.sections[2].value += "_";
            }
        } else {
            text.sections[2].value = GAME_OVER_SUB_TEXT.into();
        }
    }
}

fn anykey_check_game_over(
    query: Query<&GameOverText>,
    mut next_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepad_inputs: Res<ButtonInput<GamepadButton>>,
) {
    for timer in &query {
        if timer.0 < 1.5 {
            continue;
        }
        if keys.get_just_pressed().len() > 0 || gamepad_inputs.get_just_pressed().len() > 0 {
            next_state.set(GameState::InGame);
        }
    }
}

fn hide_ui(mut commands: Commands, query: Query<Entity, With<UI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
