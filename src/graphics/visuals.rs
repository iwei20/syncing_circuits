use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;

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
    pub shape: ShapeBundle
}

#[derive(Bundle)]
pub struct LightBundle {
    pub light: Light,
    #[bundle] pub shape: ShapeBundle }

/// This plugin spawns all disconnected lightbulb circuits, adds a shared manipulable timer to the resources, and updates the lightbulb brightness. 
pub struct DLCPlugin;

impl Plugin for DLCPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_dlc)
            .insert_resource( CircuitTimer { time: MIN_CIRCUIT_TIME, mode: CircuitTimerMode::Pause } )
            .add_plugin(ShapePlugin)
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
) { 
    let dlcc = DLRCCircuit(DisconnectLightCircuitCalculator::with_constants(300.0, 0.2, 4.0, 6.0));
    let light_color = Color::hsl(0.0, 0.0, 20.0 * dlcc.0.lightbulb_power(0.0));
    let square = shapes::Rectangle {
        extents: Vec2::splat(100.0),
        ..shapes::Rectangle::default()
    };
    let circuit_builder = GeometryBuilder::new().add(&square);
    let light_builder = GeometryBuilder::new().add(&square);

    commands.spawn_bundle(
        CircuitBundle {
            circuit: dlcc,
            shape: 
                circuit_builder.build(
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(Color::WHITE),
                        outline_mode: StrokeMode::new(Color::BLACK, 5.0),
                    },
                    Transform::from_translation(Vec3::new(250.0, -250.0, 0.0)),
                )
        }
    ).with_children(|parent| {
        parent.spawn_bundle(
            LightBundle {
                light: Light { },
                shape: 
                    light_builder.build(
                        DrawMode::Outlined {
                            fill_mode: FillMode::color(light_color),
                            outline_mode: StrokeMode::new(Color::BLACK, 5.0),
                        },
                        Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),
                    )
            }
        );
    });
}

/// Updates the colors of all light entities based on the time provided by CircuitTimer.
fn update_lightbulb(
    circuit_timer: ResMut<CircuitTimer>,
    mut query_lights: Query<(&Light, &Parent, &mut DrawMode)>,
    query_circs: Query<&DLRCCircuit>,
) {
    for (_, parent, mut draw_mode) in query_lights.iter_mut() {
        let parent_circuit = query_circs.get(parent.0).expect("couldn't find child to light");
        let new_power = parent_circuit.0.lightbulb_power(circuit_timer.time);
        let new_color = Color::hsl(0.0, 0.0, 20.0 * new_power);
        if let DrawMode::Outlined {
            ref mut fill_mode,
            outline_mode: _,
        } = *draw_mode
        {
            fill_mode.color = new_color;
        }
    }
}
