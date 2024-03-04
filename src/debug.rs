use bevy::prelude::*;

use crate::GameState;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_state_switch);
    }
}

fn debug_state_switch(game_state: Res<State<GameState>>) {
    if game_state.is_changed() {
        let state = game_state.get();
        debug!("GameState switched to {:?}", state);
    }
}
