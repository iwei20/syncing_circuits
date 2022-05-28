use bevy::prelude::*;
use syncing_circuits::{graphics::DLCPlugin, graphics::CircuitTimer, graphics::SideBarPlugin};


fn main() {
    App::new()
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

pub fn update_time(
    mut time: ResMut<CircuitTimer>,
    keyboard_input: Res<Input<KeyCode>>,
    ) {
    if keyboard_input.pressed(KeyCode::Left) { time.time -= 0.1; }
    if keyboard_input.pressed(KeyCode::Right) { time.time += 0.1; }
}
