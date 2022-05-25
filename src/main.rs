use bevy::prelude::*;
use syncing_circuits::graphics::DLCPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(start_camera)
        .add_plugin(DLCPlugin)
        .run();

}

fn start_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
