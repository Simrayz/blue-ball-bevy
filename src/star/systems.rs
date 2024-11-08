use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::components::Star;
use super::resources::StarSpawnTimer;
use super::{NUMBER_OF_STARS, STAR_SIZE};

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(STAR_SIZE, STAR_SIZE)),
                    color: get_sprite_color(),
                    ..default()
                },
                ..default()
            },
            Star {},
        ));
    }
}

pub fn tick_spawn_stars(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_star(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(STAR_SIZE, STAR_SIZE)),
                    color: get_sprite_color(),
                    ..default()
                },
                ..default()
            },
            Star {},
        ));
    }
}

fn get_sprite_color() -> Color {
    let color_hues: Vec<f32> = vec![60.0, 120.0, 180., 300.];
    let random_hue = color_hues.choose(&mut rand::thread_rng()).unwrap();

    Color::hsv(*random_hue, 0.7, 1.0)
}
