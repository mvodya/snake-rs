use bevy::prelude::*;

mod demo;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, demo::HelloPlugin))
        .run();
}
