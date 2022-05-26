use bevy::prelude::*;
use syncing_circuits::{graphics::{DLCPlugin, DLCBundle, DisconnectLightCircuit, DLCPosition, DLCSize}, DisconnectLightCircuitCalculator, graphics::Time};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(start_camera)
        .add_startup_system(spawn_circuits)
        .add_plugin(DLCPlugin)
        .insert_resource(Time { t: 0.0 } )
        .add_system(update_time)
        .run();

}

fn start_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_circuits(mut commands: Commands) {
    commands.spawn_bundle(DLCBundle {
        dlc: DisconnectLightCircuit(DisconnectLightCircuitCalculator::with_constants(5.0, 4.0, 3.0)),
        position: DLCPosition(Transform::from_translation(Vec3::new(250.0, -250.0, 0.0))),
        size: DLCSize(Vec2::new(100.0, 100.0))
    });
}


pub fn update_time(
    mut time: ResMut<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&DisconnectLightCircuit, &DLCPosition, &DLCSize)>,
    ) {
    if keyboard_input.pressed(KeyCode::Left) { time.t -= 1.0; }
    if keyboard_input.pressed(KeyCode::Right) { time.t += 1.0; }
    println!("time is now: {}", time.t);
    for (circuit, _, _) in query.iter() {
        println!("power is now: {}", circuit.0.lightbulb_power(10.0, time.t));
     }
}
