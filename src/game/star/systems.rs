use bevy::{prelude::*, window::PrimaryWindow};

use super::{components::Star, resources::StarSpawnTimer};

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    rand::random::<f32>() * window.width(),
                    rand::random::<f32>() * window.height(),
                    0.0,
                ),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn tick_star_spawn_timer(time: Res<Time>, mut star_spawn_timer: ResMut<StarSpawnTimer>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    rand::random::<f32>() * window.width(),
                    rand::random::<f32>() * window.height(),
                    0.0,
                ),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn despawn_stars(mut commands: Commands, stars_query: Query<Entity, With<Star>>) {
    for star in stars_query.iter() {
        commands.entity(star).despawn();
    }
}
