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
