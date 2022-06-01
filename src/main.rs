use bevy::prelude::*;
use syncing_circuits::graphics::{update_time, DLCPlugin, SideBarPlugin};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(SideBarPlugin)
        .add_startup_system(start_camera)
        .add_plugin(DLCPlugin)
        .add_system(update_time)
        .run();
}

fn start_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
