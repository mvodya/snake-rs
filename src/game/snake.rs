use bevy::prelude::*;
use bevy_spatial::SpatialAccess;

use crate::GameState;

use super::{CollisionTracker, GameTickTimer, Movable, MovementStages, NNTree};
pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SnakeSpawnedEvent>()
            .add_event::<SnakeCatastrophicEvent>()
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
                    (spawn_snake_body, snake_collision_with_snakes).after(MovementStages::Commit),
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

/// Snake head component
#[derive(Component)]
struct Snake(SnakeDirection);

/// Reference on snake head
///
/// Used for snake body elements
#[derive(Component)]
struct SnakeRef(Entity);

/// Snake body
///
/// Contains next snake body element
#[derive(Component)]
struct SnakeBody(Option<Entity>);

/// Marker for last snake element
#[derive(Component)]
struct SnakeTail;

/// Called when snake head spawned
///
/// Attention! Snake spawned without SnakeTail & SnakeRef
#[derive(Event)]
struct SnakeSpawnedEvent(Entity);

/// Called when snake collides with other snake
#[derive(Event)]
struct SnakeCatastrophicEvent(Entity);

/// Spawn snake head
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
        CollisionTracker,
    ));

    ev_snake_spawned.send(SnakeSpawnedEvent(snake.id()));
}

/// Called for snake after snake spawn
fn on_snake_spawn(
    mut ev_snake_spawned: EventReader<SnakeSpawnedEvent>,
    snakes: Query<Entity, With<Snake>>,
    mut commands: Commands,
) {
    for ev in ev_snake_spawned.read() {
        // Search snake
        let snake = snakes.get(ev.0).unwrap();
        // Add reference to snake head and tail component for new snake
        commands.entity(snake).insert((SnakeRef(snake), SnakeTail));
        debug!("Setup snake {:?} after spawn", snake);
    }
}

/// Calculate movement of snake head
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

/// Calculate movement of snake body elements
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

/// Player input handler
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

/// Creates new snake body element
fn spawn_snake_body(
    mut commands: Commands,
    timer: ResMut<GameTickTimer>,
    mut tails: Query<(Entity, &SnakeBody, &SnakeRef, Option<&Snake>, &Transform), With<SnakeTail>>,
    bodies: Query<&Transform, (With<SnakeBody>, Without<SnakeTail>)>,
) {
    // TODO: Remove this
    // Call spawn only when we have game tick
    if !timer.0.just_finished() {
        return;
    };

    for (entity, body, snake_ref, snake, transform) in &mut tails {
        // Get current position
        let current_pos = transform.translation;

        // Get next snake body entity
        let delta;
        if let Some(next_body) = body.0 {
            // Search transformation component for next snake body
            let next_body_pos = bodies.get(next_body).unwrap().translation;
            // Calculate delta of current position and next position
            delta = (current_pos - next_body_pos);
        } else if let Some(snake) = snake {
            // Get delta from direction of snake head moving
            delta = snake.0.get_vector().extend(0.) * Vec3::new(-1., -1., -1.);
        } else {
            continue;
        }
        // Calculate new position for tail
        let new_pos = current_pos + delta;

        // Spawn new snake tail
        let tail = commands.spawn((
            SnakeBody(Some(entity)),
            Movable(None),
            SnakeRef(snake_ref.0),
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
            CollisionTracker,
        ));
        debug!("Spawn new snake body {:?}", tail.id());

        // Remove snake tail from pervious body
        commands.entity(entity).remove::<SnakeTail>();
    }
}

/// TODO: Remove snake head and snake elements
fn despawn_all_snakes(mut commands: Commands, query: Query<(Entity, &Snake)>) {
    for (entity, snake) in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Check snake collision... with snakes?
fn snake_collision_with_snakes(
    mut ev_snake_catastrophic: EventWriter<SnakeCatastrophicEvent>,
    timer: ResMut<GameTickTimer>,
    tree: Res<NNTree>,
    query: Query<(Entity, &Transform), With<Snake>>,
    bodies: Query<(Entity, &Transform), (With<SnakeBody>, With<CollisionTracker>)>,
) {
    if timer.0.just_finished() {
        // Run search for all snakes (heads)
        for (entity, transform) in query.iter() {
            // Get snake position
            let pos = transform.translation.truncate();
            // Find nearest entities
            for (other_pos, other_entity) in tree.within_distance(pos, 1.) {
                if let Some(other_entity) = other_entity {
                    // Do not react on self & react only on same position
                    // check is other_entity is SnakeBody
                    if entity != other_entity && pos == other_pos && bodies.contains(other_entity) {
                        // Send event
                        ev_snake_catastrophic.send(SnakeCatastrophicEvent(entity));
                        debug!(
                            "Snake {:?} collision detected at {:?} with {:?}",
                            entity, pos, other_entity
                        );
                    }
                }
            }
        }
    }
}
