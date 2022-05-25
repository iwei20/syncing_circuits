use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::DisconnectedLightbulbCircuit;

#[derive(Bundle)]
pub struct DLCBundle {
    pub vdlc: VisualDLC,
    pub position: DLCPosition,
    pub size: DLCSize
}

/// Disconnected lightbulb circuit, graphic component
#[derive(Component)]
pub struct VisualDLC(pub DisconnectedLightbulbCircuit);

#[derive(Component)]
pub struct DLCPosition(pub Transform);

#[derive(Component)]
pub struct DLCSize(pub Vec2);

pub struct DLCPlugin;

impl Plugin for DLCPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(draw_dlc);
    }
}

fn draw_dlc(
    mut commands: Commands,
    query: Query<(&VisualDLC, &DLCPosition, &DLCSize)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    for (visual, position, size) in query.iter() {
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(size.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: position.0,
            ..default()
        });
    }
}