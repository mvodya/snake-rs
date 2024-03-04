use bevy::prelude::*;

use crate::{game::snake::SnakePlugin, GameState};

mod meat;
mod snake;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((snake::SnakePlugin, meat::MeatPlugin))
            .add_systems(OnEnter(GameState::InGame), on_game_start);
    }
}

fn on_game_start() {
    debug!("Init game state");
}
