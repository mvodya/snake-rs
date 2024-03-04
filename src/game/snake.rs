use bevy::prelude::*;

use crate::GameState;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SnakeSpawnedEvent>()
            .add_systems(OnEnter(GameState::InGame), spawn_snake)
            .add_systems(OnExit(GameState::InGame), despawn_all_snakes);
    }
}

#[derive(Component)]
struct Snake;

#[derive(Event)]
struct SnakeSpawnedEvent(Entity);

fn setup() {
    // TODO
    info!("Snake setup runs!");
}

pub fn spawn_snake(mut ev_snake_spawned: EventWriter<SnakeSpawnedEvent>, mut commands: Commands) {
    let snake = commands.spawn((
        Snake,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 1., 1.),
                custom_size: Vec2::new(1., 1.).into(),
                ..default()
            },
            ..default()
        },
    ));
    ev_snake_spawned.send(SnakeSpawnedEvent(snake.id()));
}

pub fn despawn_all_snakes(mut commands: Commands, quety: Query<(Entity, &Snake)>) {
    for (entity, snake) in quety.iter() {
        commands.entity(entity).despawn();
    }
}
