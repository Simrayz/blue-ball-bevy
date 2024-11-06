pub mod components;
pub mod events;
pub mod resources;
mod soundtrack;
mod systems;

use events::*;
use resources::*;
use soundtrack::*;
use systems::*;

use bevy::prelude::*;

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
            (
                spawn_player,
                spawn_enemies,
                spawn_camera,
                spawn_stars,
                play_soundtrack,
            ),
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
