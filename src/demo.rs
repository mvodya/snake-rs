use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_spiders).add_systems(
            Update,
            (hello_world, (greet_spiders, update_spiders).chain()),
        );
    }
}

#[derive(Component)]
struct Spider;

#[derive(Component)]
struct Name(String);

fn hello_world() {
    println!("test by test");
}

fn add_spiders(mut commands: Commands) {
    commands.spawn((Spider, Name("Vasya".to_string())));
    commands.spawn((Spider, Name("Momoto".to_string())));
    commands.spawn((Spider, Name("Hatiko".to_string())));
}

fn greet_spiders(query: Query<&Name, With<Spider>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn update_spiders(mut query: Query<&mut Name, With<Spider>>) {
    for mut name in &mut query {
        if name.0 == "Vasya" {
            name.0 = "Petya".to_string();
            break;
        }
    }
}
