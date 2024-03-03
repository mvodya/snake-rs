use bevy::{prelude::*, render::camera::ScalingMode};

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, (setup, spawn_player))
        .add_systems(Update, move_player);
    }
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.0);
    commands.spawn(camera_bundle);
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0., 0.5, 1.),
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        ..default()
    }));
}

fn move_player(
  mut players: Query<&mut Transform, With<Player>>,
  keys: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
) {
    let mut direction = Vec2::ZERO;
    if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        direction.y += 1.;
    }
    if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        direction.y -= 1.;
    }
    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        direction.x += 1.;
    }
    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        direction.x -= 1.;
    }
    if direction == Vec2::ZERO {
        return;
    }
    direction = direction.normalize();

    let move_speed = 7.;
    let move_delta = direction * move_speed * time.delta_seconds();

    for mut transform in &mut players {
        transform.translation += move_delta.extend(0.);
    }
}