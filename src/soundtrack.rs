use bevy::{audio::*, prelude::*};

#[derive(Component)]
struct Soundtrack;

pub fn play_soundtrack(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/chiptronical.ogg"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: Volume::new(0.2),
                ..default()
            },
        },
        Soundtrack,
    ));
}

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
