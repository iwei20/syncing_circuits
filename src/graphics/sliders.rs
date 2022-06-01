use bevy::prelude::*;
use bevy_egui::egui::{
    plot::{Line, Plot, Points, Value, Values},
    Align2,
};
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::graphics::{
    CircuitTimer, CircuitTimerMode, CurrentTimePlot, DLRCCircuit, MAX_CIRCUIT_TIME,
    MIN_CIRCUIT_TIME,
};

///Plugin to add slide bar of sliders
pub struct SideBarPlugin;

impl Plugin for SideBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system(left_slider_frame)
            .add_system(circuit_plot);
    }
}

///Create left side bar with the four desired sliders
fn left_slider_frame(
    mut egui_context: ResMut<EguiContext>,
    mut query_circs: Query<(&mut DLRCCircuit, &mut CurrentTimePlot)>,
    mut time: ResMut<CircuitTimer>,
) {
    egui::Window::new("Circuit")
        .anchor(Align2::LEFT_CENTER, [0.0, -200.0])
        .fixed_size([200.0, 200.0])
        .collapsible(false)
        .frame(egui::Frame::dark_canvas(&egui_context.ctx_mut().style()))
        .show(egui_context.ctx_mut(), |ui| {
            ui.add(
                egui::ProgressBar::new(time.time / (MAX_CIRCUIT_TIME - MIN_CIRCUIT_TIME))
                    .text(format!("time since start: {:.1}", time.time)),
            );
            for (mut dlcc, _) in query_circs.iter_mut() {
                let r = &mut dlcc.0.circuit.resistance;
                //TODO: loosen the limit on small R
                //We limit R to be very small compared to L and C
                //This is to let the angular frequency always be defined
                //meaning we always have oscilations
                //Possibly in the future we can figure out how to model bigger R
                //but it isn't in Halliday so right now this limitation stands
                //This might have been fixed with the change to the simulation
                ui.add(egui::Slider::new(r, 0.01..=0.2).text("R").fixed_decimals(2));
                let l = &mut dlcc.0.circuit.inductance;
                ui.add(egui::Slider::new(l, 1.0..=10.0).text("L").fixed_decimals(2));
                let c = &mut dlcc.0.circuit.capacitance;
                ui.add(egui::Slider::new(c, 1.0..=10.0).text("C").fixed_decimals(2));
                let start_q = &mut dlcc.0.circuit.startcharge;
                ui.add(
                    egui::Slider::new(start_q, 0.0..=300.0)
                        .text("starting Q")
                        .fixed_decimals(2),
                );
            }
            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                //start, stop, and rewind buttons
                ui.selectable_value(&mut time.mode, CircuitTimerMode::Play, "Play");
                ui.selectable_value(&mut time.mode, CircuitTimerMode::Pause, "Pause");
                if ui.button("Reset").clicked() {
                    time.time = MIN_CIRCUIT_TIME;
                    time.mode = CircuitTimerMode::Pause;
                    for (mut dlcc, mut plot) in query_circs.iter_mut() {
                        dlcc.0.circuit.reset();
                        plot.0.clear();
                        plot.0.push((MIN_CIRCUIT_TIME, 0.0));
                    }
                }
            });
        });
}

fn circuit_plot(mut egui_ctx: ResMut<EguiContext>, query_circs: Query<&CurrentTimePlot>) {
    egui::Window::new("Current")
        .anchor(Align2::RIGHT_TOP, [0.0, 100.0])
        .fixed_size([400.0, 400.0])
        .collapsible(false)
        .frame(egui::Frame::dark_canvas(&egui_ctx.ctx_mut().style()))
        .show(egui_ctx.ctx_mut(), |ui| {
            for points in query_circs.iter() {
                let line = Line::new(Values::from_values_iter(
                    points.0.iter().map(|&(a, b)| Value::new(a, b)),
                ));

                //stupid hack to get graph to have fixed axis
                //basically just add the boundry points to the graph
                let boundry_points = vec![
                    Value::new(MIN_CIRCUIT_TIME, -10.0),
                    Value::new(MIN_CIRCUIT_TIME, 10.0),
                    Value::new(MAX_CIRCUIT_TIME, -10.0),
                    Value::new(MAX_CIRCUIT_TIME, 10.0),
                ];
                Plot::new("Current")
                    .view_aspect(1.0)
                    .data_aspect(5.0)
                    .allow_scroll(false)
                    .allow_zoom(false)
                    .allow_drag(false)
                    .allow_boxed_zoom(false)
                    .center_y_axis(true)
                    .show(ui, |plot_ui| {
                        plot_ui.line(line);
                        plot_ui.points(Points::new(Values::from_values(boundry_points)));
                    });
            }
        });
}
