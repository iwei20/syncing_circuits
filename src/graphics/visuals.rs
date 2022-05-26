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

pub struct Time {
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
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    for (circuit, position, size) in query.iter() {
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(size.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: position.0,
            ..default()
        });
    }
}
