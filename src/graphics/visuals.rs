use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::{GeometryBuilder, DrawMode, FillMode, StrokeMode, Path, ShapePath}, shapes, plugin::ShapePlugin};

use crate::DisconnectLightCircuitCalculator;
use std::cmp::PartialEq;

#[derive(Component)]
/// A component representing the circuit calculator, rather than the visual part.
pub struct DLRCCircuit(pub DisconnectLightCircuitCalculator);

#[derive(Component)]
/// A marker component to indicate this shape is a light.
pub struct Light;

#[derive(Component)]
pub struct CircleRadius(pub f32);

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

#[derive(Bundle)]
pub struct CircleBundle {
    pub radius: CircleRadius,
    #[bundle]
    pub shape_bundle: ShapeBundle
}
/// This plugin spawns all disconnected lightbulb circuits, adds a shared manipulable timer to the resources, and updates the lightbulb brightness.
pub struct DLCPlugin;

impl Plugin for DLCPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugin(ShapePlugin)
            .add_startup_system(spawn_dlc)
            .insert_resource(CircuitTimer {
                time: MIN_CIRCUIT_TIME,
                mode: CircuitTimerMode::Pause,
            })
            .add_system(update_lightbulb)
            .add_system(expand_circles);
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
    pub mode: CircuitTimerMode,
}

/// Spawns all circuit + light entities
fn spawn_dlc(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let dlcc = DLRCCircuit(DisconnectLightCircuitCalculator::with_constants(
        300.0, 0.2, 4.0, 6.0,
    ));

    let light_texture_handle = asset_server.load("dino-rino-flame-animation.png");
    //let light_texture_atlas = TextureAtlas::from_grid_with_padding(light_texture_handle, Vec2::new(14.0, 48.0), 4, 1, Vec2::new(20.0, 48.0));
    let light_texture_atlas =
        TextureAtlas::from_grid(light_texture_handle, Vec2::new(16.0, 48.0), 4, 1);
    let light_texture_atlus_handle = texture_atlases.add(light_texture_atlas);

    commands
        .spawn_bundle(CircuitBundle {
            circuit: dlcc,
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("dino-rino-mouth-open-small.png"),
                transform: Transform::from_scale(Vec3::splat(5.0))
                    .with_translation(Vec3::new(-75.0, 0.0, 5.0)),
                ..default()
            },
        })
        .with_children(|parent| {
            parent.spawn_bundle(LightBundle {
                light: Light {},
                sprite_sheet_bundle: SpriteSheetBundle {
                    texture_atlas: light_texture_atlus_handle,
                    transform: Transform::from_scale(Vec3::splat(1.0))
                        .with_translation(Vec3::new(0.0, 27.0, -1.0)),
                    ..default()
                },
            });
        });
        
}

/// Updates the colors of all light entities based on the time provided by CircuitTimer.
fn update_lightbulb(
    mut commands: Commands,
    circuit_timer: ResMut<CircuitTimer>,
    mut query_lights: Query<(&Parent, &mut TextureAtlasSprite), With<Light>>,
    query_circs: Query<&DLRCCircuit>,
) {
    for (parent, mut sprite) in query_lights.iter_mut() {
        let parent_circuit = query_circs
            .get(parent.0)
            .expect("couldn't find child to light");
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

        // Check if the current time (phase shifted) is a multiple of a half period
        let epsilon = 0.003;
        let period = std::f32::consts::TAU / parent_circuit.0.circuit.angular_freq();
        let time_to_peaks = period / 2.0;
        let time_multiple = (circuit_timer.time + period / 4.0) / time_to_peaks;
        let closest_integer_multiple = time_multiple.round();
        if (time_multiple - closest_integer_multiple).abs() < epsilon && closest_integer_multiple != 0.0 {
            info!("Circle spawned");
            let starting_radius = 0.0;
            let circle_builder = GeometryBuilder::new().add(&shapes::Circle {
                radius: starting_radius,
                ..shapes::Circle::default()
            });
            commands
                .entity(**parent)
                .with_children(|parent| {
                    parent.spawn_bundle(
                        CircleBundle {
                            radius: CircleRadius(starting_radius),
                            shape_bundle: circle_builder.build(
                                DrawMode::Outlined {
                                    fill_mode: FillMode::color(Color::hsla(0.0, 0.0, 0.0, 0.0)),
                                    outline_mode: StrokeMode::new(Color::hsla(0.0, 0.0, 1.0, calculate_circle_alpha(starting_radius)), 1.0),
                                },
                                Transform::from_translation(Vec3::new(0.0, 25.0, 20.0))
                            )
                        }
                    );
                });
        }
        
    }
}

fn calculate_circle_alpha(radius: f32) -> f32 {
    (-radius / 100.0) + 1.0
}

fn expand_circles(mut query: Query<(&mut CircleRadius, &mut Path, &mut DrawMode)>) {
    for (mut radius, mut path, mut draw_mode) in query.iter_mut() {
        *radius = CircleRadius(radius.0 + 0.2);
        let new_circle = shapes::Circle {
            radius: radius.0,
            ..shapes::Circle::default()
        };
        *draw_mode = DrawMode::Outlined {
            fill_mode: FillMode::color(Color::hsla(0.0, 0.0, 0.0, 0.0)),
            outline_mode: StrokeMode::new(Color::hsla(0.0, 0.0, 1.0, calculate_circle_alpha(radius.0)), 1.0),
        };
        *path = ShapePath::build_as(&new_circle);
    }
}
