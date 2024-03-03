use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_spiders)
            .add_systems(
                Update,
                (greet_spiders, update_spiders).chain(),
            );
    }
}

#[derive(Component)]
struct Spider;

#[derive(Component)]
struct Name(String);

// fn hello_world() {
//     println!("test by test");
// }

fn add_spiders(mut commands: Commands) {
    commands.spawn((Spider, Name("Vasya".to_string())));
    commands.spawn((Spider, Name("Momoto".to_string())));
    commands.spawn((Spider, Name("Hatiko".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_spiders(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Spider>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
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
