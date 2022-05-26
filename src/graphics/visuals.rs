use std::ops::DerefMut;

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle}};

use crate::DisconnectLightCircuitCalculator;

#[derive(Bundle)]
pub struct DLCBundle {
    pub dlc: DisconnectLightCircuit,
    pub circuit: CircuitMesh,
    pub light: LightMesh
}

/// Disconnected lightbulb circuit, graphic component
#[derive(Component)]
pub struct DisconnectLightCircuit(pub DisconnectLightCircuitCalculator);

#[derive(Component, Clone)]
pub struct DLCSize(pub Vec2);

#[derive(Component)]
pub struct CircuitMesh(pub MaterialMesh2dBundle<ColorMaterial>);

#[derive(Component)]
pub struct LightMesh(pub MaterialMesh2dBundle<ColorMaterial>);

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
            circuit: CircuitMesh(MaterialMesh2dBundle { 
                mesh: meshes.add(shape::Quad::new(size.0).into()).into(), 
                material: materials.add(ColorMaterial::from(Color::WHITE)), 
                transform: position, 
                ..default()
            }),
            light: LightMesh(MaterialMesh2dBundle { 
                mesh: meshes.add(shape::Quad::new(size.0 / 4.0).into()).into(), 
                material: materials.add(light_color), 
                transform: position * Transform::from_translation((size.0 / 2.0, 0.0).into()), 
                ..default()
            })
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
