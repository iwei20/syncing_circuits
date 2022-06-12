use bevy::prelude::*;
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    plugin::ShapePlugin,
    prelude::{DrawMode, FillMode, GeometryBuilder, Path, ShapePath, StrokeMode},
    shapes,
};

use crate::DisconnectLightCircuitCalculator;
use std::cmp::PartialEq;

/// This plugin spawns all disconnected lightbulb circuits, adds a shared manipulable timer to the resources, and updates the lightbulb brightness.
pub struct DLCPlugin;

impl Plugin for DLCPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(ShapePlugin)
            .add_startup_system(spawn_dlc)
            .insert_resource(CircuitTimer {
                time: MIN_CIRCUIT_TIME,
                mode: CircuitTimerMode::Pause,
            })
            .add_system(update_lightbulb)
            .add_system(expand_circles)
            .add_system(update_current_arrow);
    }
}

/// at what time the simulation ends
pub const MAX_CIRCUIT_TIME: f64 = 100.0;

/// the minimum time of the simulation, the start
/// there isn't much reason I can see for this to not always be zero
pub const MIN_CIRCUIT_TIME: f64 = 0.0;

/* CIRCUIT ENTITY */
#[derive(Bundle)]
/// A bundle of components defining a circuit
pub struct CircuitBundle {
    pub circuit: DLRCCircuit,
    pub plot: CurrentTimePlot,
    pub sts: SpawnedThisSignum,
    pub lcrs: LastCurrentRateSignum,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

#[derive(Component)]
/// A component representing the circuit calculator, rather than the visual part.
pub struct DLRCCircuit(pub DisconnectLightCircuitCalculator);

#[derive(Component)]
/// A component to store the current computed current, time pairs related to a circuit which has been
pub struct CurrentTimePlot(pub Vec<(f64, f64)>);

#[derive(Component)]
pub struct SpawnedThisSignum(pub f64, pub bool);

#[derive(Component)]
pub struct LastCurrentRateSignum(pub f64);

const ARROW_SPRITE_SCALE: f32 = 0.5;

/// Spawns all circuit + light entities
fn spawn_dlc(mut commands: Commands, asset_server: Res<AssetServer>) {
    let dlcc = DLRCCircuit(DisconnectLightCircuitCalculator::with_constants(
        10.0, 0.2, 4.0, 6.0,
    ));

    let circle_builder = GeometryBuilder::new().add(&shapes::Circle {
        radius: 10.0,
        ..shapes::Circle::default()
    });

    commands
        .spawn_bundle(CircuitBundle {
            circuit: dlcc,
            //initialized with MIN_CIRCUIT_TIME, 0.0, because that is what it starts as
            plot: CurrentTimePlot(vec![(MIN_CIRCUIT_TIME, 0.0)]),
            sts: SpawnedThisSignum(0.0, false),
            lcrs: LastCurrentRateSignum(1.0),
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("series-edited.png"),
                transform: Transform::from_scale(Vec3::splat(0.3))
                    .with_translation(Vec3::new(-75.0, 0.0, 5.0)),
                ..default()
            },
        })
        .with_children(|parent| {
            parent.spawn_bundle(LightBundle {
                light: Light,
                shape_bundle: circle_builder.build(
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(Color::hsla(0.0, 0.0, 0.0, 0.7)),
                        outline_mode: StrokeMode::new(Color::hsla(0.0, 0.0, 1.0, 1.0), 1.0),
                    },
                    Transform::from_scale(Vec3::splat(18.0))
                        .with_translation(Vec3::new(-505.0, 335.0, 15.0)),
                ),
            });
        })
        .with_children(|parent| {
            parent.spawn_bundle(CurrentArrowBundle {
                current_arrow: CurrentArrow,
                sprite_bundle: SpriteBundle {
                    texture: asset_server.load("whitearrow_box.png"),
                    transform: Transform::from_scale(Vec3::splat(ARROW_SPRITE_SCALE))
                        //.with_translation(Vec3::new(-505.0, 600.0, 10.0)),
                        .with_translation(Vec3::new(0.0, 75.0, 0.0)),
                    ..default()
                },
            });
        });
}

/* LIGHTBULB ENTITY */
#[derive(Bundle)]
/// A bundle of components defining a lightbulb
pub struct LightBundle {
    pub light: Light,
    #[bundle]
    pub shape_bundle: ShapeBundle,
}

#[derive(Component)]
/// A marker component to indicate this shape is a light.
pub struct Light;

/// Updates the colors of all light entities based on the time provided by CircuitTimer and the
/// current circuit. Also, if the lightbulb power is at a peak, a circle is spawned.
fn update_lightbulb(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    circuit_timer: ResMut<CircuitTimer>,
    mut query_lights: Query<(Entity, &Parent, &mut DrawMode), With<Light>>,
    mut query_circs: Query<(
        &mut DLRCCircuit,
        &mut SpawnedThisSignum,
        &mut LastCurrentRateSignum,
    )>,
) {
    for (entity, parent, mut draw_mode) in query_lights.iter_mut() {
        let mut parent_circuit = query_circs
            .get_mut(parent.0)
            .expect("couldn't find child to light");
        let new_power = parent_circuit.0 .0.lightbulb_power();

        *draw_mode = DrawMode::Outlined {
            fill_mode: FillMode::color(Color::hsla(0.0, 0.0, new_power as f32 * 6.0, 0.7)),
            outline_mode: StrokeMode::new(Color::hsla(0.0, 0.0, 1.0, 1.0), 1.0),
        };

        let epsilon = 0.2;
        if parent_circuit.0 .0.circuit.current_rate().signum() != parent_circuit.2 .0
            && circuit_timer.time > MIN_CIRCUIT_TIME + epsilon
            && circuit_timer.time < MAX_CIRCUIT_TIME - epsilon
            && !parent_circuit.1 .1
            && parent_circuit.0 .0.circuit.current().abs() > epsilon
        {
            //play circle pop sound
            let sound_effect = asset_server.load("pop.ogg");
            audio.play(sound_effect);

            info!("Circle spawned");
            let starting_radius = 10.0;
            let circle_builder = GeometryBuilder::new().add(&shapes::Circle {
                radius: starting_radius,
                ..shapes::Circle::default()
            });
            commands.entity(entity).with_children(|parent| {
                parent.spawn_bundle(CircleBundle {
                    radius: CircleRadius(starting_radius),
                    shape_bundle: circle_builder.build(
                        DrawMode::Outlined {
                            fill_mode: FillMode::color(Color::hsla(0.0, 0.0, 0.0, 0.0)),
                            outline_mode: StrokeMode::new(
                                Color::hsla(0.0, 0.0, 1.0, calculate_circle_alpha(starting_radius)),
                                1.0,
                            ),
                        },
                        Transform::identity(),
                    ),
                });
            });
            parent_circuit.1 .1 = true;
        }
    }
}

/* CIRCLE ENTITY */
#[derive(Bundle)]
/// A bundle of componenets defining circle
pub struct CircleBundle {
    pub radius: CircleRadius,
    #[bundle]
    pub shape_bundle: ShapeBundle,
}

#[derive(Component)]
/// A component to store the radius of an expanding circle
pub struct CircleRadius(pub f32);

/// Calculates what the alpha value of the circle should be to make it fade as the radius gets
/// larger
///
/// # Arguments
/// * 'radius' - the radius of the circle
///
/// # Returns
/// A floating point number representing what the alpha value should be
fn calculate_circle_alpha(radius: f32) -> f32 {
    (-radius / 20.0).exp()
}

/// Expands the radius of all circles, despawning them if they get too big
fn expand_circles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CircleRadius, &mut Path, &mut DrawMode)>,
) {
    for (entity, mut radius, mut path, mut draw_mode) in query.iter_mut() {
        if radius.0 > 100.0 {
            info!("Circle despawned");
            commands.entity(entity).despawn();
        }
        *radius = CircleRadius(radius.0 + 0.4);
        let new_circle = shapes::Circle {
            radius: radius.0,
            ..shapes::Circle::default()
        };
        *draw_mode = DrawMode::Outlined {
            fill_mode: FillMode::color(Color::hsla(0.0, 0.0, 0.0, 0.0)),
            outline_mode: StrokeMode::new(
                Color::hsla(0.0, 0.0, 1.0, calculate_circle_alpha(radius.0)),
                1.0,
            ),
        };
        *path = ShapePath::build_as(&new_circle);
    }
}

/* CURRENT ARROW ENTITY */
#[derive(Bundle)]
/// An entity meant to show a current arrow
pub struct CurrentArrowBundle {
    pub current_arrow: CurrentArrow,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

#[derive(Component)]
/// A marker for current arrow entities
pub struct CurrentArrow;

fn update_current_arrow(
    query_circs: Query<&DLRCCircuit>,
    mut query_arrows: Query<(&Parent, &CurrentArrow, &mut Transform)>,
) {
    for (parent, _currentarrow, mut transform) in query_arrows.iter_mut() {
        let parent_circuit = query_circs
            .get(parent.0)
            .expect("Couldn't find parent circuit of this arrow");

        let current = parent_circuit.0.circuit.current();
        transform.rotation = Quat::mul_quat(
            transform.rotation,
            Quat::from_rotation_z(-current as f32 * 0.1),
        );
        let epsilon = 0.01;
        if current.abs() > epsilon {
            *transform = transform.with_scale(Vec3::new(
                -ARROW_SPRITE_SCALE * current.signum() as f32,
                ARROW_SPRITE_SCALE,
                ARROW_SPRITE_SCALE,
            ));
        }
    }
}

/* Circuit Timer Resource */
#[derive(PartialEq)]
/// The two modes the simulation can be in, paused or playing
pub enum CircuitTimerMode {
    Play,
    Pause,
}

/// A timer keeping track of the current time in the simulation
pub struct CircuitTimer {
    pub time: f64,
    pub mode: CircuitTimerMode,
}

///the amount of simulation time passing every frame
///this intentionally doesn't make the simulation run in real time
const DELTA_T: f64 = 0.1;

/// Updates the timer and other time senstitive parts of the simulation
pub fn update_time(
    mut time: ResMut<CircuitTimer>,
    mut query_circs: Query<(
        &mut DLRCCircuit,
        &mut CurrentTimePlot,
        &mut SpawnedThisSignum,
        &mut LastCurrentRateSignum,
    )>,
) {
    if time.mode == CircuitTimerMode::Play {
        time.time += DELTA_T;
        for (mut circ, mut plot, mut sts, mut lcrs) in query_circs.iter_mut() {
            lcrs.0 = circ.0.circuit.current_rate().signum();
            circ.0.circuit.tick(DELTA_T);
            let new_current = circ.0.circuit.current();
            plot.0.push((time.time, new_current));
            if new_current.signum() != sts.0.signum() {
                sts.1 = false;
            }
            sts.0 = new_current;
        }
    }

    if time.time > MAX_CIRCUIT_TIME {
        time.time = MAX_CIRCUIT_TIME;
        time.mode = CircuitTimerMode::Pause;
    }
    time.time = time.time.max(MIN_CIRCUIT_TIME);
}
