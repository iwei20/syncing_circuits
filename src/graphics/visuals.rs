use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;

use crate::DisconnectLightCircuitCalculator;

#[derive(Component)]
pub struct DLRCCircuit(pub DisconnectLightCircuitCalculator);
#[derive(Component)]
pub struct Light;

#[derive(Bundle)]
pub struct CircuitBundle {
    pub circuit: DLRCCircuit,
    #[bundle]
    pub shape: ShapeBundle,
    //pub wrapped: MaterialMesh2dBundle<ColorMaterial>
}

#[derive(Bundle)]
pub struct LightBundle {
    pub light: Light,
    #[bundle]
    pub shape: ShapeBundle,
    //pub wrapped: MaterialMesh2dBundle<ColorMaterial>
}

pub struct DLCPlugin;

impl Plugin for DLCPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_dlc)
            .insert_resource( CircuitTimer { time: 0.0 } )
            .add_plugin(ShapePlugin)
            .add_system(update_lightbulb);
    }
}

pub struct CircuitTimer {
    pub time: f32,
}


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
            ref mut outline_mode,
        } = *draw_mode
        {
            fill_mode.color = new_color;
        }
    }
}
