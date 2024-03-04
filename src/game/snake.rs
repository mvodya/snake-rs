use bevy::prelude::*;

use crate::GameState;

use super::GameTickTimer;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SnakeSpawnedEvent>()
            .add_systems(OnEnter(GameState::InGame), spawn_snake)
            .add_systems(OnExit(GameState::InGame), despawn_all_snakes)
            .add_systems(
                Update,
                (
                    snake_input,
                    move_snake_head,
                    (move_snake_body).after(move_snake_head),
                ),
            );
    }
}

#[derive(Debug, Clone)]
enum SnakeDirection {
    Up,
    Right,
    Down,
    Left,
}

impl SnakeDirection {
    fn get_vector(&self) -> Vec2 {
        match self {
            SnakeDirection::Up => Vec2::new(0., 1.),
            SnakeDirection::Right => Vec2::new(1., 0.),
            SnakeDirection::Down => Vec2::new(0., -1.),
            SnakeDirection::Left => Vec2::new(-1., 0.),
        }
    }

    fn is_oposite(&self, other: &SnakeDirection) -> bool {
        let a = self.get_vector();
        let b = other.get_vector();
        a.dot(b) == -1.
    }
}

#[derive(Component)]
struct Snake(SnakeDirection);

#[derive(Component)]
struct SnakeBody(Option<Entity>);

#[derive(Event)]
struct SnakeSpawnedEvent(Entity);

fn setup() {
    // TODO
    info!("Snake setup runs!");
}

fn spawn_snake(mut ev_snake_spawned: EventWriter<SnakeSpawnedEvent>, mut commands: Commands) {
    let snake = commands.spawn((
        Snake(SnakeDirection::Right),
        SnakeBody(None),
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

fn move_snake_head(mut snakes: Query<(&mut Transform, &Snake)>, timer: ResMut<GameTickTimer>) {
    if timer.0.just_finished() {
        for (mut transform, snake) in &mut snakes {
            let movement = snake.0.get_vector();
            transform.translation += movement.extend(0.);
        }
    }
}

fn move_snake_body() {}

fn snake_input(mut snakes: Query<&mut Snake>, keys: Res<ButtonInput<KeyCode>>, time: Res<Time>) {
    let mut direction: Option<SnakeDirection> = None;
    if keys.any_just_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        direction = SnakeDirection::Up.into();
    }
    if keys.any_just_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        direction = SnakeDirection::Down.into();
    }
    if keys.any_just_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        direction = SnakeDirection::Right.into();
    }
    if keys.any_just_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        direction = SnakeDirection::Left.into();
    }
    if let Some(direction) = direction {
        for mut snake in &mut snakes {
            if !snake.0.is_oposite(&direction) {
                snake.0 = direction.clone();
            }
        }
    }
}

fn spawn_snake_body(mut ev_snake_spawned: EventReader<SnakeSpawnedEvent>, mut commands: Commands) {
    for ev in ev_snake_spawned.read() {}
}

fn despawn_all_snakes(mut commands: Commands, query: Query<(Entity, &Snake)>) {
    for (entity, snake) in query.iter() {
        commands.entity(entity).despawn();
    }
}
