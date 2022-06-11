use bevy::{audio::AudioSink, prelude::*};

pub struct MusicController(Handle<AudioSink>);

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup_audio);
    }
}

fn setup_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let music = asset_server.load("still_woozy_x_los_retros_style_instrumental_daydreams.ogg");
    let handle = audio_sinks.get_handle(audio.play_with_settings(music, PlaybackSettings::LOOP));
    commands.insert_resource(MusicController(handle));
}
