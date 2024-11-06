pub mod components;
pub mod events;
pub mod resources;
mod systems;

use events::*;
use resources::*;
use systems::*;

use bevy::prelude::*;

// Players
pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;
// Enemies
pub const NUMBER_OF_ENEMIES: usize = 5;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;

// Misc
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<SoundEffectHandles>()
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .add_systems(
            Startup,
            (spawn_player, spawn_enemies, spawn_camera, spawn_stars),
        )
        .add_systems(Update, (player_movement, confine_player_movement))
        .add_systems(
            Update,
            (
                enemy_movement,
                update_enemy_direction,
                confine_enemy_movement,
                enemy_hit_player,
                tick_enemy_spawn_timer,
                spawn_enemy,
            ),
        )
        .add_systems(
            Update,
            (player_hit_star, update_score, tick_spawn_stars, spawn_star),
        )
        .add_systems(
            Update,
            (
                exit_game,
                handle_game_over,
                update_high_scores,
                high_scores_updated,
            ),
        )
        .run();
}
