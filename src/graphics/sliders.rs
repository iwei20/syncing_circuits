use bevy::prelude::*;
use bevy_egui::{egui, EguiPlugin, EguiContext};
use bevy_egui::egui::{Align2, plot::{Value, Values, Plot, Line}};

use crate::graphics::{DLRCCircuit, CircuitTimer};

///Plugin to add slide bar of sliders
pub struct SideBarPlugin;

impl Plugin for SideBarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EguiPlugin)
            .add_system(left_slider_frame)
            .add_system(circuit_plot);
    }
}

///Create left side bar with the four desired sliders
fn left_slider_frame(
    mut egui_context: ResMut<EguiContext>,
    mut query_circs: Query<&mut DLRCCircuit>,
    mut time: ResMut<CircuitTimer>,
    ) {
    egui::Window::new("Circuit")
        .anchor(Align2::LEFT_CENTER, [0.0, -200.0])
        .fixed_size([200.0, 200.0])
        .collapsible(false)
        .frame(egui::Frame::dark_canvas(&egui_context.ctx_mut().style()))
        .show(egui_context.ctx_mut(), |ui| {
            for mut dlcc in query_circs.iter_mut() {
                let r = &mut dlcc.0.circuit.resistance;
                //TODO: loosen the limit on small R
                //We limit R to be very small compared to L and C
                //This is to let the angular frequency always be defined
                //meaning we always have oscilations
                //Possibly in the future we can figure out how to model bigger R
                //but it isn't in Halliday so right now this limitation stands
                ui.add(egui::Slider::new(r, 0.01..=0.2).text("R").fixed_decimals(2));
                let l = &mut dlcc.0.circuit.inductance;
                ui.add(egui::Slider::new(l, 1.0..=10.0).text("L").fixed_decimals(2));
                let c = &mut dlcc.0.circuit.capacitance;
                ui.add(egui::Slider::new(c, 1.0..=10.0).text("C").fixed_decimals(2));
            }
            //remeber to make this max value of time match with the graph in the function below
            ui.add(egui::Slider::new(&mut time.time, 0.0..=100.0).text("Time").fixed_decimals(0));
        });
}

fn circuit_plot(
    mut egui_ctx: ResMut<EguiContext>,
    query_circs: Query<&DLRCCircuit>,
    ) {
    egui::Window::new("Current")
        .anchor(Align2::RIGHT_TOP, [0.0, 100.0])
        .fixed_size([400.0, 400.0])
        .collapsible(false)
        .frame(egui::Frame::dark_canvas(&egui_ctx.ctx_mut().style()))
        .show(egui_ctx.ctx_mut(), |ui| {
            for dlcc in query_circs.iter() {
                let fidelity = 1000;
                let curvtime = (0..fidelity).map(|i| {
                    //remember to update this with the max value of time
                    let x = (i * 100) as f32 / fidelity as f32;
                    Value::new(x, dlcc.0.circuit.current(x))
                });
                let line = Line::new(Values::from_values_iter(curvtime));
                Plot::new("Current")
                    .view_aspect(1.0)
                    .data_aspect(5.0)
                    .allow_scroll(false)
                    .allow_zoom(false)
                    .allow_drag(false)
                    .allow_boxed_zoom(false)
                    .center_y_axis(true)
                    .show(ui, |plot_ui| plot_ui.line(line));
            }
        });
}
