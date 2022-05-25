use bevy::prelude::*;
use syncing_circuits::{graphics::{DLCPlugin, DLCBundle, VisualDLC, DLCPosition, DLCSize}, DisconnectedLightbulbCircuit};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(start_camera)
        .add_startup_system(spawn_circuits)
        .add_plugin(DLCPlugin)
        .run();

}

fn start_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_circuits(mut commands: Commands) {
    commands.spawn_bundle(DLCBundle {
        vdlc: VisualDLC(DisconnectedLightbulbCircuit::with_constants(5.0, 4.0, 3.0)),
        position: DLCPosition(Transform::from_translation(Vec3::new(250.0, -250.0, 0.0))),
        size: DLCSize(Vec2::new(100.0, 100.0))
    });
}