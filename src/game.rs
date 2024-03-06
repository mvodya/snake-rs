use bevy::prelude::*;

use crate::GameState;

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
            0.3,
            TimerMode::Repeating,
        )))
        .add_plugins((
            snake::SnakePlugin,
            meat::MeatPlugin,
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
            ),
        );
    }
}

/// Game tick timer
#[derive(Resource)]
struct GameTickTimer(Timer);

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
fn game_tick_timer(time: Res<Time>, mut timer: ResMut<GameTickTimer>) {
    timer.0.tick(time.delta());
}

/// When InGame state enter
fn on_game_start() {
    debug!("Init game");
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
            if let Some(pos) = movable.0 {
                transform.translation = pos.extend(0.);
                movable.0 = None;
            }
        }
    }
}
