use bevy::prelude::*;
use syncing_circuits::graphics::{
    update_time, DLCPlugin, EffectsPlugin, MusicPlugin, UIWindowsPlugin,
};

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(UIWindowsPlugin)
        .add_plugin(MusicPlugin)
        .add_startup_system(start_camera)
        .add_plugin(EffectsPlugin)
        .add_plugin(DLCPlugin)
        .add_system(update_time)
        .run();
}

fn start_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
