use bevy::{audio::AudioSink, prelude::*};
use crate::graphics::DLRCCircuit;

pub struct MusicController(Handle<AudioSink>);

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
    let music = asset_server.load("still_woozy_x_los_retros_style_instrumental_daydreams.ogg");
    let handle = audio_sinks.get_handle(
        audio.play_with_settings(
            music,
            PlaybackSettings::LOOP,
        ));
    commands.insert_resource(MusicController(handle));
}

fn volume(
    query_circs: Query<&DLRCCircuit>,
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
        sink.set_volume((power_avg as f32 * 5.0).min(0.8));
    }
}

