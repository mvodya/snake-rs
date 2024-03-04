use bevy::prelude::*;

use crate::{game::snake::SnakePlugin, GameState};

mod meat;
mod snake;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTickTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .add_plugins((snake::SnakePlugin, meat::MeatPlugin))
            .add_systems(OnEnter(GameState::InGame), on_game_start)
            .add_systems(Update, game_tick_timer);
    }
}

#[derive(Resource)]
struct GameTickTimer(Timer);

fn on_game_start() {
    debug!("Init game state");
}

fn game_tick_timer(time: Res<Time>, mut timer: ResMut<GameTickTimer>) {
    timer.0.tick(time.delta());
}
