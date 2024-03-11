use bevy::prelude::*;

use crate::GameState;

mod interface;
mod meat;
mod snake;

/// Stages for control movement game entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
enum MovementStages {
    /// Input handlers
    Input,
    /// Movement calculations
    Calculate,
    /// Apply movement
    Commit,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTickTimer(Timer::from_seconds(
            0.2,
            TimerMode::Repeating,
        )))
        .insert_resource(PlayerStats::default())
        .add_plugins((
            snake::SnakePlugin,
            meat::MeatPlugin,
            interface::GameInterfacePlugin,
            bevy_spatial::AutomaticUpdate::<CollisionTracker>::new()
                .with_spatial_ds(bevy_spatial::SpatialStructure::KDTree2)
                .with_frequency(std::time::Duration::from_millis(1)),
        ))
        .add_systems(OnEnter(GameState::InGame), on_game_start)
        .add_systems(OnExit(GameState::InGame), on_game_stop)
        .add_systems(
            Update,
            (
                game_tick_timer,
                move_all_movable
                    .in_set(MovementStages::Commit)
                    .after(MovementStages::Calculate),
                test_game_over,
            ),
        );
    }
}

/// Game tick timer
#[derive(Resource)]
struct GameTickTimer(Timer);

/// Player statistics
#[derive(Resource, Clone, Copy)]
struct PlayerStats {
    pub score: u32,
    pub food_eaten: u32,
    pub distance_traveled: u32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            score: 0,
            food_eaten: 0,
            distance_traveled: 0,
        }
    }
}

/// Movable component
///
/// Used for delayed movement (after MovementStages:Calculate stage)
#[derive(Component)]
struct Movable(Option<Vec2>);

/// Track collision with KDTree
#[derive(Component, Default)]
struct CollisionTracker;

/// Alias for KDTree
type NNTree = bevy_spatial::kdtree::KDTree2<CollisionTracker>;

/// Handle game tick
fn game_tick_timer(
    time: Res<Time>,
    mut timer: ResMut<GameTickTimer>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    // Speed up movement on shift key pressed
    let speed_up;
    if keys.any_pressed([KeyCode::ShiftLeft]) {
        speed_up = 2;
    } else {
        speed_up = 1;
    }
    // Update game tick
    timer.0.tick(time.delta() * speed_up);
}

/// When InGame state enter
fn on_game_start(mut stats: ResMut<PlayerStats>) {
    debug!("Init game");
    // Reset player statistics
    *stats = PlayerStats::default();
}

/// When InGame state exit
fn on_game_stop() {
    debug!("Stop game");
}

/// Apply all movement
fn move_all_movable(
    mut movable: Query<(&mut Movable, &mut Transform)>,
    timer: ResMut<GameTickTimer>,
) {
    if timer.0.just_finished() {
        for (mut movable, mut transform) in &mut movable {
            if let Some(mut pos) = movable.0 {
                // Portal on borders of world
                let hx = crate::MAP_SIZE.x / 2.;
                let hy = crate::MAP_SIZE.y / 2.;
                pos.x = ((pos.x + hx) % crate::MAP_SIZE.x) - hx;
                if pos.x < -hx {
                    pos.x = hx - 1.;
                }
                pos.y = ((pos.y + hy) % crate::MAP_SIZE.y) - hy;
                if pos.y < -hy {
                    pos.y = hy - 1.;
                }
                // Make transform
                transform.translation = pos.extend(0.);
                movable.0 = None;
            }
        }
    }
}

fn test_game_over(
    mut ev_snake_catastrophic: EventReader<snake::SnakeCatastrophicEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ev in ev_snake_catastrophic.read() {
        next_state.set(GameState::GameOver);
    }
}
