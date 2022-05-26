use std::ops::DerefMut;

use bevy::{prelude::*, sprite::{Mesh2dHandle, SpecializedMaterial2d}};

use crate::DisconnectLightCircuitCalculator;

#[derive(Bundle)]
pub struct DLCBundle<M: SpecializedMaterial2d> {
    pub dlc: DisconnectLightCircuit,
    pub position: Transform,
    pub size: DLCSize,
    pub circuit_light_mesh: (Mesh2dHandle, Mesh2dHandle),
    pub circuit_light_material: (Handle<M>, Handle<M>)

}

/// Disconnected lightbulb circuit, graphic component
#[derive(Component)]
pub struct DisconnectLightCircuit(pub DisconnectLightCircuitCalculator);

#[derive(Component, Clone)]
pub struct DLCSize(pub Vec2);

pub struct DLCPlugin;

pub struct CircuitTimer {
    pub t: f32,
}


impl Plugin for DLCPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {

        app.add_startup_system(spawn_dlc)
           .add_system(update_lightbulb);
    }
}


fn spawn_dlc(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) { 

    let DLC_LIST: [(DisconnectLightCircuit, Transform, DLCSize); 1] = [
        (
            DisconnectLightCircuit(DisconnectLightCircuitCalculator::with_constants(0.2, 4.0, 6.0)),
            Transform::from_translation(Vec3::new(250.0, -250.0, 0.0)),
            DLCSize(Vec2::new(100.0, 100.0))
        )
    ];

    for (circuit, position, size) in DLC_LIST {
        // Spawn circuit
        let light_color = ColorMaterial::from(Color::hsl(0.0, 0.0, 20.0 * circuit.0.lightbulb_power(300.0, 0.0)));
        commands.spawn_bundle(DLCBundle {
            dlc: circuit,
            position,
            size: size.clone(),
            circuit_light_mesh: (meshes.add(shape::Quad::new(size.0).into()).into(), meshes.add(shape::Quad::new(size.0 / 4.0).into()).into()),
            circuit_light_material: (materials.add(ColorMaterial::from(Color::WHITE)), materials.add(light_color))
        });
    }
}

fn update_lightbulb(
    mut circuit_timer: ResMut<CircuitTimer>,
    mut query: Query<(&DisconnectLightCircuit, &Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    query.for_each(|(circuit, color)| {
        materials.get_mut(color).unwrap().color = Color::hsl(0.0, 0.0, 20.0 * circuit.0.lightbulb_power(300.0, circuit_timer.t));
    });
}
