use bevy::prelude::*;

use crate::GameState;

pub struct MeatPlugin;

impl Plugin for MeatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component)]
struct Meat;

fn setup() {
    // TODO
    info!("Meat setup runs!");
}
