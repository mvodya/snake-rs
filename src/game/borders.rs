use bevy::prelude::*;

use crate::{GameState, MAP_SIZE};

pub struct BordersPlugin;

impl Plugin for BordersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_borders)
            .add_systems(OnExit(GameState::InGame), despawn_borders);
    }
}

#[derive(Component)]
struct Border;

fn spawn_borders(mut commands: Commands) {
    let color = Color::rgb(0.5, 0.5, 0.5);

    // Up
    commands.spawn((
        Border,
        SpriteBundle {
            sprite: Sprite {
                color: color,
                custom_size: Vec2::new(MAP_SIZE.x, 0.5).into(),
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: MAP_SIZE.y / 2.,
                    z: -1.,
                },
                ..default()
            },
            ..default()
        },
    ));

    // Down
    commands.spawn((
        Border,
        SpriteBundle {
            sprite: Sprite {
                color: color,
                custom_size: Vec2::new(MAP_SIZE.x, 0.5).into(),
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: -MAP_SIZE.y / 2.,
                    z: -1.,
                },
                ..default()
            },
            ..default()
        },
    ));

    // Left
    commands.spawn((
        Border,
        SpriteBundle {
            sprite: Sprite {
                color: color,
                custom_size: Vec2::new(0.5, MAP_SIZE.y).into(),
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: -MAP_SIZE.x / 2.,
                    y: 0.,
                    z: -1.,
                },
                ..default()
            },
            ..default()
        },
    ));

    // Right
    commands.spawn((
        Border,
        SpriteBundle {
            sprite: Sprite {
                color: color,
                custom_size: Vec2::new(0.5, MAP_SIZE.y).into(),
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: MAP_SIZE.x / 2.,
                    y: 0.,
                    z: -1.,
                },
                ..default()
            },
            ..default()
        },
    ));
}

fn despawn_borders(mut commands: Commands, query: Query<Entity, With<Border>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
