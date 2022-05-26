use bevy::prelude::*;
use syncing_circuits::{graphics::{DLCPlugin, DisconnectLightCircuit}, graphics::CircuitTimer};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(start_camera)
        .add_plugin(DLCPlugin)
        .insert_resource(CircuitTimer { t: 0.0 } )
        .add_system(update_time)
        .run();

}

fn start_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn update_time(
    mut time: ResMut<CircuitTimer>,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<&DisconnectLightCircuit>,
    ) {
    if keyboard_input.pressed(KeyCode::Left) { time.t -= 0.1; }
    if keyboard_input.pressed(KeyCode::Right) { time.t += 0.1; }
    info!("time is now: {}", time.t);
    for circuit in query.iter() {
        info!("power is now: {}", circuit.1.lightbulb_power(300.0, time.t));
     }
}
