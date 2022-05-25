use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::DisconnectedLightbulbCircuit;

/// Disconnected lightbulb circuit, graphic component
#[derive(Component)]
pub struct VisualDLC(DisconnectedLightbulbCircuit);

#[derive(Component)]
pub struct DLCPosition(Transform);

pub struct DLCPlugin;

impl Plugin for DLCPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(draw_dlc);
    }
}

fn draw_dlc(
    mut commands: Commands,
    mut query: Query<(&VisualDLC, &DLCPosition)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    for (visual, position) in query.iter() {
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::new(50., 6.)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            transform: position.0,
            ..default()
        });
    }
}