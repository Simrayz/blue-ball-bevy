pub mod events;
pub mod soundtrack;
mod systems;

pub mod enemy;
mod player;
mod score;
pub mod star;

use enemy::EnemyPlugin;
use events::*;
use player::PlayerPlugin;
use score::ScorePlugin;
use soundtrack::*;
use star::StarPlugin;

use bevy::prelude::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<SoundEffectHandles>()
        .add_event::<GameOver>()
        .add_plugins((EnemyPlugin, StarPlugin, PlayerPlugin, ScorePlugin))
        .add_systems(Startup, (spawn_camera, play_soundtrack))
        .add_systems(Update, (exit_game, handle_game_over))
        .run();
}
