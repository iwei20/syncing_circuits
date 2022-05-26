use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::DisconnectLightCircuitCalculator;

#[derive(Bundle)]
pub struct DLCBundle {
    pub dlc: DisconnectLightCircuit,
    pub position: DLCPosition,
    pub size: DLCSize
}

/// Disconnected lightbulb circuit, graphic component
#[derive(Component)]
pub struct DisconnectLightCircuit(pub DisconnectLightCircuitCalculator);

#[derive(Component)]
pub struct DLCPosition(pub Transform);

#[derive(Component)]
pub struct DLCSize(pub Vec2);

pub struct DLCPlugin;

pub struct CircuitTimer {
    pub t: f32,
}

impl Plugin for DLCPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(draw_dlc);
    }
}

fn draw_dlc(
    mut commands: Commands,
    query: Query<(&DisconnectLightCircuit, &DLCPosition, &DLCSize)>,
    time: Res<CircuitTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    for (circuit, position, size) in query.iter() {
        // Draw circuit
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(size.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: position.0,
            ..default()
        });

        // Draw lightbulb
        let light_color = ColorMaterial::from(Color::hsl(0.0, 0.0, 20.0 * circuit.0.lightbulb_power(300.0, time.t)));
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(size.0 / 4.0).into()).into(),
            material: materials.add(light_color),
            transform: position.0 * Transform::from_translation((size.0 / 2.0, 0.0).into()),
            ..default()
        });
    }
}
