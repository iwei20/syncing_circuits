use bevy::prelude::*;

use crate::DisconnectLightCircuitCalculator;
use std::cmp::PartialEq;

#[derive(Component)]
/// A component representing the circuit calculator, rather than the visual part.
pub struct DLRCCircuit(pub DisconnectLightCircuitCalculator);

#[derive(Component)]
/// A marker component to indicate this shape is a light.
pub struct Light;

#[derive(Bundle)]
pub struct CircuitBundle {
    pub circuit: DLRCCircuit,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

#[derive(Bundle)]
pub struct LightBundle {
    pub light: Light,
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
}

/// This plugin spawns all disconnected lightbulb circuits, adds a shared manipulable timer to the resources, and updates the lightbulb brightness. 
pub struct DLCPlugin;

impl Plugin for DLCPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_dlc)
            .insert_resource( CircuitTimer { time: MIN_CIRCUIT_TIME, mode: CircuitTimerMode::Pause } )
            .add_system(update_lightbulb);
    }
}


pub const MAX_CIRCUIT_TIME: f32 = 100.0;
pub const MIN_CIRCUIT_TIME: f32 = 0.0;

#[derive(PartialEq)]
pub enum CircuitTimerMode {
    Play,
    Pause,
}

/// A shared resource for timers that provides the current *simulated* time, which can be changed freely.
pub struct CircuitTimer {
    pub time: f32,
    pub mode: CircuitTimerMode
}

/// Spawns all circuit + light entities
fn spawn_dlc(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) { 
    let dlcc = DLRCCircuit(DisconnectLightCircuitCalculator::with_constants(300.0, 0.2, 4.0, 6.0));

    let light_texture_handle = asset_server.load("dino-rino-flame-animation.png");
    //let light_texture_atlas = TextureAtlas::from_grid_with_padding(light_texture_handle, Vec2::new(14.0, 48.0), 4, 1, Vec2::new(20.0, 48.0));
    let light_texture_atlas = TextureAtlas::from_grid(light_texture_handle, Vec2::new(16.0, 48.0), 4, 1);
    let light_texture_atlus_handle = texture_atlases.add(light_texture_atlas);

    commands.spawn_bundle(
        CircuitBundle {
            circuit: dlcc,
            sprite_bundle:
                SpriteBundle {
                    texture: asset_server.load("dino-rino-mouth-open-small.png"),
                    transform: Transform::from_scale(Vec3::splat(5.0))
                        .with_translation(Vec3::new(-75.0, 0.0, 5.0)),
                    ..default()
                }
        }
    ).with_children(|parent| {
        parent.spawn_bundle(
            LightBundle {
                light: Light { },
                sprite_sheet_bundle: 
                    SpriteSheetBundle {
                        texture_atlas: light_texture_atlus_handle,
                        transform: Transform::from_scale(Vec3::splat(1.0))
                            .with_translation(Vec3::new(0.0, 27.0, -1.0)),
                        ..default()
                    }
            }
        );
    });
}

/// Updates the colors of all light entities based on the time provided by CircuitTimer.
fn update_lightbulb(
    circuit_timer: ResMut<CircuitTimer>,
    mut query_lights: Query<(&Parent, &mut TextureAtlasSprite), With<Light>>,
    query_circs: Query<&DLRCCircuit>,
) {
    for (parent, mut sprite) in query_lights.iter_mut() {
        let parent_circuit = query_circs.get(parent.0).expect("couldn't find child to light");
        let new_power = parent_circuit.0.lightbulb_power(circuit_timer.time);
        if new_power < 0.05 {
            sprite.index = 0;
        } else if new_power < 0.1 {
            sprite.index = 1;
        } else if new_power < 0.15 {
            sprite.index = 2;
        } else {
            sprite.index = 3;
        }
    }
}
