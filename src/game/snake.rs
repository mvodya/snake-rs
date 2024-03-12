use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_spatial::SpatialAccess;

use crate::GameState;

use super::{
    meat::MeatEaten, CollisionTracker, GameTickTimer, Movable, MovementStages, NNTree, PlayerStats,
};
pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SnakeSpawnedEvent>()
            .add_event::<SnakeCollisionEvent>()
            .add_event::<SnakeCatastrophicEvent>()
            .insert_resource(SnakeInputBuffer(VecDeque::new()))
            .add_systems(OnEnter(GameState::InGame), spawn_snake)
            .add_systems(OnExit(GameState::InGame), despawn_all_snakes)
            .add_systems(
                Update,
                (
                    snake_input.in_set(MovementStages::Input),
                    (move_snake_head, move_snake_body)
                        .in_set(MovementStages::Calculate)
                        .after(MovementStages::Input),
                    spawn_snake_body,
                    on_snake_spawn,
                    player_score_collector,
                    (snake_collision, snake_collision_with_snakes).after(MovementStages::Commit),
                )
                    .run_if(in_state(GameState::InGame)),
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
/// Contains next and pervious snake body element
///
/// forward <-- HEAD <-- BODY[] <-- TAIL <-- backward
#[derive(Component)]
struct SnakeBody {
    forward: Option<Entity>,
    backward: Option<Entity>,
}

/// Marker for last snake element
#[derive(Component)]
struct SnakeTail;

/// Called when snake head spawned
///
/// Attention! Snake spawned without SnakeTail & SnakeRef
#[derive(Event)]
pub struct SnakeSpawnedEvent(Entity);

/// Called when snake collides with other entity
#[derive(Event)]
pub struct SnakeCollisionEvent {
    pub snake: Entity,
    pub other: Entity,
    pub position: Vec2,
}

#[derive(Resource)]
struct SnakeInputBuffer(VecDeque<SnakeDirection>);

/// Called when snake collides with other snake
#[derive(Event)]
pub struct SnakeCatastrophicEvent(Entity);

/// Spawn snake head
fn spawn_snake(mut ev_snake_spawned: EventWriter<SnakeSpawnedEvent>, mut commands: Commands) {
    // Spawn snake
    let snake = commands.spawn((
        Snake(SnakeDirection::Right),
        SnakeBody {
            forward: None,
            backward: None,
        },
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
    mut stats: ResMut<PlayerStats>,
) {
    if timer.0.just_finished() {
        for (mut movable, body) in &mut movable_bodies {
            if let Some(next_body) = body.forward {
                let next_pos = bodies.get(next_body).unwrap().translation;
                movable.0 = Some(next_pos.truncate());
            }
        }
        // Update stats
        stats.distance_traveled += 1;
    }
}

/// Player input handler
fn snake_input(
    mut snakes: Query<&mut Snake>,
    keys: Res<ButtonInput<KeyCode>>,
    mut buffer: ResMut<SnakeInputBuffer>,
    timer: ResMut<GameTickTimer>,
) {
    // Get input
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

    // Save input to buffer
    if buffer.0.len() < 2 {
        if let Some(direction) = direction {
            buffer.0.push_back(direction.clone());
        }
    }

    // Apply movement from buffer
    if timer.0.just_finished() {
        if let Some(direction) = buffer.0.pop_front() {
            for mut snake in &mut snakes {
                if !snake.0.is_oposite(&direction) {
                    snake.0 = direction.clone();
                }
            }
        }
    }
}

/// Creates new snake body element
fn spawn_snake_body(
    mut commands: Commands,
    mut ev_meat_eaten: EventReader<MeatEaten>,
    mut tails: Query<
        (
            Entity,
            &mut SnakeBody,
            &SnakeRef,
            Option<&Snake>,
            &Transform,
        ),
        With<SnakeTail>,
    >,
    bodies: Query<&Transform, (With<SnakeBody>, Without<SnakeTail>)>,
) {
    for ev in ev_meat_eaten.read() {
        for (entity, mut body, snake_ref, snake, transform) in &mut tails {
            // Check what snake requested
            if ev.snake != snake_ref.0 {
                continue;
            }

            // Get current position
            let current_pos = transform.translation;

            // Get next snake body entity
            let delta;
            if let Some(next_body) = body.forward {
                // Search transformation component for next snake body
                let next_body_pos = bodies.get(next_body).unwrap().translation;
                // Calculate delta of current position and next position
                delta = current_pos - next_body_pos;
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
                SnakeBody {
                    forward: Some(entity),
                    backward: None,
                },
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

            debug!("Snake new tail! {:?}, {:?}", tail.id(), new_pos);

            // Set backward for pervious body
            body.backward = Some(tail.id());

            // Remove snake tail from pervious body
            commands.entity(entity).remove::<SnakeTail>();
        }
    }
}

fn player_score_collector(
    mut ev_meat_eaten: EventReader<MeatEaten>,
    mut stats: ResMut<PlayerStats>,
) {
    for _ in ev_meat_eaten.read() {
        stats.score += 50;
        stats.food_eaten += 1;
    }
}

/// Remove snake head and snake elements
fn despawn_all_snakes(mut commands: Commands, query: Query<Entity, With<SnakeBody>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Check snake collision
fn snake_collision(
    mut ev_snake_collision: EventWriter<SnakeCollisionEvent>,
    timer: ResMut<GameTickTimer>,
    tree: Res<NNTree>,
    query: Query<(Entity, &Transform), With<Snake>>,
) {
    if timer.0.just_finished() {
        // Run search for all snakes (heads)
        for (entity, transform) in query.iter() {
            // Get snake position
            let pos = transform.translation.truncate();
            // Find nearest entities
            for (other_pos, other_entity) in tree.within_distance(pos, 1.) {
                // Is entity is available
                if let Some(other_entity) = other_entity {
                    // Do not react on self & react only on same position
                    if entity != other_entity && pos == other_pos {
                        // Send collision event
                        ev_snake_collision.send(SnakeCollisionEvent {
                            snake: entity,
                            other: other_entity,
                            position: pos,
                        });
                    }
                }
            }
        }
    }
}

fn snake_collision_with_snakes(
    mut ev_snake_collision: EventReader<SnakeCollisionEvent>,
    mut ev_snake_catastrophic: EventWriter<SnakeCatastrophicEvent>,
    bodies: Query<(Entity, &Transform), (With<SnakeBody>, With<CollisionTracker>)>,
) {
    for ev in ev_snake_collision.read() {
        // Check is other_entity is SnakeBody
        if bodies.contains(ev.other) {
            // Send event
            ev_snake_catastrophic.send(SnakeCatastrophicEvent(ev.snake));
            debug!(
                "Snake {:?} collision with snake body {:?} detected at {:?}",
                ev.snake, ev.position, ev.other
            );
        }
    }
}
