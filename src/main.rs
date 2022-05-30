use bevy::prelude::*;
use syncing_circuits::graphics::{DLCPlugin, CircuitTimer, SideBarPlugin, MIN_CIRCUIT_TIME, MAX_CIRCUIT_TIME, CircuitTimerMode};

fn main() {
    App::new()
        .insert_resource( Msaa { samples: 4 })
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
    if keyboard_input.pressed(KeyCode::Right) || time.mode == CircuitTimerMode::Play { time.time += 0.1; }

    if time.time > MAX_CIRCUIT_TIME {
        time.time = MAX_CIRCUIT_TIME;
        time.mode = CircuitTimerMode::Pause;
    }
    time.time = time.time.max(MIN_CIRCUIT_TIME);
}
