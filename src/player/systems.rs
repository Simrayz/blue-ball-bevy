use bevy::{prelude::*, window::PrimaryWindow};

use super::components::*;
use super::PLAYER_ROTATION_SPEED;
use super::PLAYER_SIZE;
use super::PLAYER_SPEED;
use crate::enemy::components::*;
use crate::enemy::ENEMY_SIZE;
use crate::events::GameOver;
use crate::score::resources::*;
use crate::soundtrack::*;
use crate::star::components::*;
use crate::star::STAR_SIZE;

pub fn rotation_system(mut query: Query<(&FaceInDirection, &mut Transform)>) {
    for (face, mut transform) in query.iter_mut() {
        *transform = transform.with_rotation(Quat::from_axis_angle(
            Vec3::Z,
            face.angle - std::f32::consts::FRAC_PI_2,
        ));
    }
}

pub fn control_player_system(
    mut query: Query<(&Player, &mut FaceInDirection)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for (player, mut face) in query.iter_mut() {
        let rotate_dir = match (
            keys.pressed(KeyCode::ArrowLeft),
            keys.pressed(KeyCode::ArrowRight),
        ) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };

        face.angle += rotate_dir * player.rotation_speed * dt;
    }
}

pub fn control_player_speed_system(
    mut query: Query<(&Player, &mut MoveInFacingDirection)>,
    keys: Res<ButtonInput<KeyCode>>,
    score: Res<Score>,
) {
    for (_player, mut move_facing) in query.iter_mut() {
        let score_increase = 1.0 + score.value as f32 / 100.0;
        let movement_speed = match (
            keys.pressed(KeyCode::ArrowUp),
            keys.pressed(KeyCode::ArrowDown),
        ) {
            (true, false) => PLAYER_SPEED * 1.5 * score_increase,
            (false, true) => PLAYER_SPEED * 0.5 * score_increase,
            _ => PLAYER_SPEED * score_increase,
        };
        move_facing.speed = movement_speed;
    }
}

pub fn move_in_facing_direction_system(
    mut query: Query<(&FaceInDirection, &MoveInFacingDirection, &mut Transform)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for (face, move_facing, mut transform) in query.iter_mut() {
        let mut delta_pos = transform.translation;
        delta_pos.x += dt * face.angle.cos() * move_facing.speed;
        delta_pos.y += dt * face.angle.sin() * move_facing.speed;
        *transform = transform.with_translation(delta_pos);
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                ..default()
            },
            texture: asset_server.load("sprites/ship.png"),
            ..default()
        },
        MoveInFacingDirection {
            speed: PLAYER_ROTATION_SPEED,
        },
        FaceInDirection { angle: 0.0 },
        Player {
            rotation_speed: PLAYER_SPEED.to_radians(),
        },
    ));
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    score: Res<Score>,
    sound_effects: Res<SoundEffectHandles>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance_squared(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < (player_radius + enemy_radius).powi(2) {
                println!("Enemy hit player! Game over!");
                commands.spawn(AudioBundle {
                    source: sound_effects.game_over.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
                commands.entity(player_entity).despawn();
                event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    sound_effects: Res<SoundEffectHandles>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance_squared(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;
            if distance < (player_radius + star_radius).powi(2) {
                println!("Player hit star!");
                score.value += 1;
                commands.spawn(AudioBundle {
                    source: sound_effects.star_sound.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
                commands.entity(star_entity).despawn();
            }
        }
    }
}
