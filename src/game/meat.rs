use bevy::prelude::*;
use rand::Rng;

use crate::GameState;

use super::{snake::SnakeCollisionEvent, CollisionTracker, MovementStages};

pub struct MeatPlugin;

impl Plugin for MeatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MeatEaten>()
            .insert_resource(MeatSpawnerTickTimer(Timer::from_seconds(
                3.0,
                TimerMode::Repeating,
            )))
            .add_systems(
                Update,
                (
                    meat_spawner_tick_timer,
                    meat_spawner,
                    snake_collision_with_meat,
                )
                    .after(MovementStages::Commit)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Component)]
struct Meat;

// Meat spawn dealy
#[derive(Resource)]
struct MeatSpawnerTickTimer(Timer);

/// Called when snake eats the meat
#[derive(Event)]
pub struct MeatEaten {
    pub snake: Entity,
    pub position: Vec2,
}

/// Handle game tick
fn meat_spawner_tick_timer(time: Res<Time>, mut timer: ResMut<MeatSpawnerTickTimer>) {
    timer.0.tick(time.delta());
}

/// Meat spawner
fn meat_spawner(timer: ResMut<MeatSpawnerTickTimer>, meats: Query<&Meat>, mut commands: Commands) {
    if !timer.0.just_finished() {
        return;
    }

    // Get count of spawned meat
    let count = meats.iter().count();

    // There are no need spawn mode
    if count >= 9 {
        return;
    }

    // Calculate position
    let mut rng = rand::thread_rng();
    let hx = (crate::MAP_SIZE.x / 2.) as i32 - 1;
    let hy = (crate::MAP_SIZE.y / 2.) as i32 - 1;
    let x = rng.gen_range(-hx..=hx);
    let y = rng.gen_range(-hy..=hy);
    let pos = Vec3::new(x as f32, y as f32, 0.);

    // Spawn
    commands.spawn((
        Meat,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.),
                custom_size: Vec2::new(1., 1.).into(),
                ..default()
            },
            transform: Transform {
                translation: pos,
                ..default()
            },
            ..default()
        },
        CollisionTracker,
    ));
}

fn snake_collision_with_meat(
    mut ev_snake_collision: EventReader<SnakeCollisionEvent>,
    mut ev_meat_eaten: EventWriter<MeatEaten>,
    meats: Query<&Meat>,
    mut commands: Commands,
) {
    for ev in ev_snake_collision.read() {
        // Check is other_entity is SnakeBody
        if meats.contains(ev.other) {
            // Send event
            ev_meat_eaten.send(MeatEaten {
                snake: ev.snake,
                position: ev.position,
            });
            debug!(
                "Snake {:?} eats meat {:?} at {:?}",
                ev.snake, ev.position, ev.other
            );
            // Remove meat
            commands.entity(ev.other).despawn();
        }
    }
}
