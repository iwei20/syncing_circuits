use bevy::{audio::AudioSink, prelude::*};
use crate::graphics::DLRCCircuit;

pub struct MusicController(Handle<AudioSink>, Handle<AudioSink>);

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup_audio)
            .add_system(volume);
    }
}

fn setup_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let static_noise = asset_server.load("01-White-Noise-10min-popgone.ogg");
    let music = asset_server.load("taishi-reverie-loop.ogg");
    let static_noise_handle = audio_sinks.get_handle(
        audio.play_with_settings(
            static_noise,
            PlaybackSettings::LOOP,
        ));
    let music_handle = audio_sinks.get_handle(
        audio.play_with_settings(
            music,
            PlaybackSettings::LOOP,
        ));
    commands.insert_resource(MusicController(static_noise_handle, music_handle));
}

fn volume(
    query_circs: Query<&DLRCCircuit>,
    time: Res<Time>,
    audio_sinks: Res<Assets<AudioSink>>,
    music_controller: Res<MusicController>,
) {
    let mut circ_amount = 1;
    let mut power_avg = 0.0;
    for ddlc in query_circs.iter() {
        circ_amount += 1;
        power_avg += ddlc.0.lightbulb_power()
    }
    power_avg /= circ_amount as f64;
    if let Some(sink) = audio_sinks.get(&music_controller.0) {
        let mut max_vol = 0.03f32;
        max_vol = max_vol - max_vol.powf(0.05f32 * time.seconds_since_startup() as f32 + 1f32);
        sink.set_volume(max_vol - (power_avg as f32).min(max_vol));
    }
}

