use bevy::audio::*;
use bevy::prelude::*;

pub const ENEMY_SPAWN_TIME: f32 = 5.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;

#[derive(Resource)]
pub struct SoundEffectHandles {
    pub game_over: Handle<AudioSource>,
    pub star_sound: Handle<AudioSource>,
}

impl FromWorld for SoundEffectHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            game_over: asset_server.load("audio/jingles_NES00.ogg"),
            star_sound: asset_server.load("audio/powerUp4.ogg"),
        }
    }
}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> Self {
        Self { scores: Vec::new() }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
