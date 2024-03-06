use bevy::prelude::*;

use crate::GameState;

use super::GameTickTimer;
use super::MovementStages;
use super::Movable;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SnakeSpawnedEvent>()
            .add_systems(OnEnter(GameState::InGame), spawn_snake)
            .add_systems(OnExit(GameState::InGame), despawn_all_snakes)
            .add_systems(
                Update,
                (
                    snake_input.in_set(MovementStages::Input),
                    (move_snake_head, move_snake_body)
                        .in_set(MovementStages::Calculate)
                        .after(MovementStages::Input),
                    on_snake_spawn,
                    (spawn_snake_body).after(MovementStages::Commit),
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
struct SnakeRef(Entity);

#[derive(Component)]
struct SnakeBody(Option<Entity>);

#[derive(Component)]
struct SnakeTail;

#[derive(Event)]
struct SnakeSpawnedEvent(Entity);

fn setup() {
    // TODO
    info!("Snake setup runs!");
}

fn spawn_snake(mut ev_snake_spawned: EventWriter<SnakeSpawnedEvent>, mut commands: Commands) {
    // Spawn snake
    let snake = commands.spawn((
        Snake(SnakeDirection::Right),
        SnakeBody(None),
        Movable(None),
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

fn on_snake_spawn(
    mut ev_snake_spawned: EventReader<SnakeSpawnedEvent>,
    snakes: Query<(Entity, &Transform), With<Snake>>,
    mut commands: Commands,
) {
    for ev in ev_snake_spawned.read() {
        // Search snake
        let (snake, snake_pos) = snakes.get(ev.0).unwrap();
        // Calculate position
        let new_pos = snake_pos.translation/* + Vec3::new(-1., 0., 0.)*/;
        // Spawn snake body
        commands.spawn((
            SnakeBody(Some(snake)),
            Movable(None),
            SnakeRef(snake),
            SnakeTail,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1., 1., 1.),
                    custom_size: Vec2::new(1., 1.).into(),
                    ..default()
                },
                transform: Transform {
                    translation: new_pos,
                    ..default()
                },
                ..default()
            },
        ));
    }
}

fn move_snake_head(
    mut snakes: Query<(&mut Movable, &Transform, &Snake)>,
    timer: ResMut<GameTickTimer>,
) {
    if timer.0.just_finished() {
        for (mut movable, transform, snake) in &mut snakes {
            let current_pos = transform.translation.truncate();
            let next_pos = snake.0.get_vector() + current_pos;
            movable.0 = Some(next_pos);
        }
    }
}

fn move_snake_body(
    mut movable_bodies: Query<(&mut Movable, &SnakeBody), Without<Snake>>,
    bodies: Query<&Transform, With<SnakeBody>>,
    timer: ResMut<GameTickTimer>,
) {
    if timer.0.just_finished() {
        for (mut movable, body) in &mut movable_bodies {
            if let Some(next_body) = body.0 {
                let next_pos = bodies.get(next_body).unwrap().translation;
                movable.0 = Some(next_pos.truncate());
            }
        }
    }
}

// fn move_snake_body(mut bodies: Query<(&mut Transform, &SnakeBody), Without<Snake>>, timer: ResMut<GameTickTimer>) {
//     if timer.0.just_finished() {
//         for (mut transform, body) in &mut bodies {
//             let next_pos = body.0;
//             transform.translation += movement.extend(0.);
//         }
//     }
// }

fn snake_input(mut snakes: Query<&mut Snake>, keys: Res<ButtonInput<KeyCode>>) {
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

fn spawn_snake_body(
    // mut ev_snake_spawned: EventReader<SnakeSpawnedEvent>,
    mut commands: Commands,
    timer: ResMut<GameTickTimer>,
    mut tails: Query<(Entity, &SnakeBody, &SnakeRef, &Transform), With<SnakeTail>>,
    bodies: Query<&Transform, (With<SnakeBody>, Without<SnakeTail>)>,
) {
    if !timer.0.just_finished() {
        return;
    };
    // debug!("Spawn new snake body");

    for (entity, body, snake, transform) in &mut tails {
        // Get current position
        let current_pos = transform.translation;

        // Get next snake body entity
        if let Some(next_body) = body.0 {
            // Search transformation component for next snake body
            let next_body_pos = bodies.get(next_body).unwrap().translation;
            // Calculate delta of current position and next position
            let delta = (current_pos - next_body_pos);
            // Calculate new position for tail
            let new_pos = current_pos + delta;

            // Spawn new snake tail
            commands.spawn((
                SnakeBody(Some(entity)),
                Movable(None),
                SnakeRef(snake.0),
                SnakeTail,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1., 1., 1.),
                        custom_size: Vec2::new(1., 1.).into(),
                        ..default()
                    },
                    transform: Transform {
                        translation: new_pos,
                        ..default()
                    },
                    ..default()
                },
            ));

            // Remove snake tail from pervious body
            commands.entity(entity).remove::<SnakeTail>();
        }
    }

    // for ev in ev_snake_spawned.read() {}
}

fn despawn_all_snakes(mut commands: Commands, query: Query<(Entity, &Snake)>) {
    for (entity, snake) in query.iter() {
        commands.entity(entity).despawn();
    }
}
