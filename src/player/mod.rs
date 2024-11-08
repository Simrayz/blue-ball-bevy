use bevy::prelude::*;
use systems::*;

pub mod components;
mod systems;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 250.0;
pub const PLAYER_ROTATION_SPEED: f32 = 320.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                // player_movement,
                control_player_system,
                control_player_speed_system,
                rotation_system,
                move_in_facing_direction_system,
                confine_player_movement,
                enemy_hit_player,
                player_hit_star,
            ),
        );
    }
}
