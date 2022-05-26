use bevy::{prelude::*, sprite::{MaterialMesh2dBundle}};

use crate::DisconnectLightCircuitCalculator;

#[derive(Bundle)]
pub struct DLCBundle {
    pub dlc: DisconnectLightCircuit
}

/// Disconnected lightbulb circuit, graphic component
#[derive(Component)]
pub struct DisconnectLightCircuit(pub usize, pub DisconnectLightCircuitCalculator);

#[derive(Component, Clone)]
pub struct DLCSize(pub Vec2);

#[derive(Bundle)]
pub struct CircuitMesh {
    pub id: CircuitID,
    #[bundle]
    pub wrapped: MaterialMesh2dBundle<ColorMaterial>
}

#[derive(Component)]
pub struct CircuitID(usize);

#[derive(Bundle)]
pub struct LightMesh {
    pub id: LightID,
    #[bundle]
    pub wrapped: MaterialMesh2dBundle<ColorMaterial>
}

#[derive(Component)]
pub struct LightID(usize);

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
            DisconnectLightCircuit(
                0,
                DisconnectLightCircuitCalculator::with_constants(0.2, 4.0, 6.0)
            ),
            Transform::from_translation(Vec3::new(250.0, -250.0, 0.0)),
            DLCSize(Vec2::new(100.0, 100.0))
        )
    ];

    for (circuit, position, size) in DLC_LIST {
        // Spawn circuit
        let light_color = ColorMaterial::from(Color::hsl(0.0, 0.0, 20.0 * circuit.1.lightbulb_power(300.0, 0.0)));
        commands.spawn_bundle(CircuitMesh {
            id: CircuitID(circuit.0),
            wrapped: MaterialMesh2dBundle { 
                mesh: meshes.add(shape::Quad::new(size.0).into()).into(), 
                material: materials.add(ColorMaterial::from(Color::WHITE)), 
                transform: position, 
                ..default()
            }
        });

        commands.spawn_bundle(LightMesh {
            id: LightID(circuit.0),
            wrapped: MaterialMesh2dBundle { 
                mesh: meshes.add(shape::Quad::new(size.0 / 4.0).into()).into(), 
                material: materials.add(light_color), 
                transform: position * Transform::from_translation((size.0 / 2.0, 0.0).into()), 
                ..default()
            }
        });

        commands.spawn_bundle(DLCBundle {
            dlc: circuit,
        });
    }
}

fn update_lightbulb(
    circuit_timer: ResMut<CircuitTimer>,
    query_circuits: Query<&DisconnectLightCircuit>,
    query_light_meshes: Query<(&LightID, &Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    query_circuits.for_each(|circuit| {
        query_light_meshes.for_each(|(id, handle)| {
            if id.0 == circuit.0 {
                materials.get_mut(handle).unwrap().color = 
                    Color::hsl(
                        0.0,
                        0.0,
                        20.0 * circuit.1.lightbulb_power(300.0, circuit_timer.t)
                    );
            }
        });
    });
}
