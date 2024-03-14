use bevy::prelude::*;

mod camera;
mod game;
mod menu;

const MAP_SIZE: Vec2 = Vec2::new(80., 50.);
const SNAKE_FAT_STEPS: i32 = 5;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum GameState {
    MainMenu,
    InGame,
    GameOver,
}

fn main() {
    App::new()
        .insert_state(GameState::MainMenu)
        .add_plugins((
            DefaultPlugins
                .set(bevy::window::WindowPlugin {
                    primary_window: Some(bevy::window::Window {
                        prevent_default_event_handling: true,
                        canvas: Some("#snake-rs-canvas".into()),
                        ..default()
                    }),
                    ..default()
                })
                .set(bevy::log::LogPlugin {
                    level: bevy::log::Level::DEBUG,
                    ..default()
                }),
            camera::CameraPlugin,
            game::GamePlugin,
            menu::MenuPlugin,
        ))
        .insert_resource(TestSwitchStateTimer(Timer::from_seconds(
            2.,
            TimerMode::Repeating,
        )))
        .add_systems(Update, test_switch_state)
        .run();
}

#[derive(Resource)]
struct TestSwitchStateTimer(Timer);

fn test_switch_state(
    time: Res<Time>,
    mut timer: ResMut<TestSwitchStateTimer>,
    mut next_state: ResMut<NextState<GameState>>,
    game_state: Res<State<GameState>>,
) {
    // if timer.0.tick(time.delta()).just_finished() {
    //     match game_state.get() {
    //         GameState::MainMenu => next_state.set(GameState::InGame),
    //         GameState::InGame => (),
    //         GameState::GameOver => next_state.set(GameState::MainMenu),
    //     }
    // }
}
