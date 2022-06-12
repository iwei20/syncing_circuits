use bevy::prelude::*;
use bevy_egui::egui::{
    plot::{GridMark, Line, Plot, Points, Text, Value, Values},
    Align2, Color32, RichText,
};
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::graphics::{
    CircuitTimer, CircuitTimerMode, CurrentTimePlot, DLRCCircuit, MAX_CIRCUIT_TIME,
    MIN_CIRCUIT_TIME,
};

///Plugin to add sliders and plot to the game
pub struct UIWindowsPlugin;

impl Plugin for UIWindowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system(left_slider_frame)
            .add_system(circuit_plot);
    }
}

/// create a window with the desired sliders
fn left_slider_frame(
    mut egui_context: ResMut<EguiContext>,
    mut query_circs: Query<(&mut DLRCCircuit, &mut CurrentTimePlot)>,
    mut time: ResMut<CircuitTimer>,
) {
    egui::Window::new("Circuit")
        .anchor(Align2::LEFT_CENTER, [50.0, 200.0])
        .fixed_size([200.0, 200.0])
        .collapsible(false)
        .frame(
            egui::Frame::dark_canvas(&egui_context.ctx_mut().style())
                .fill(egui::Color32::TRANSPARENT)
                .stroke(egui::Stroke {
                    width: 1.0,
                    color: egui::Color32::TRANSPARENT,
                }),
        )
        .title_bar(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.add(
                //the f32 cast should be fine
                egui::ProgressBar::new((time.time / (MAX_CIRCUIT_TIME - MIN_CIRCUIT_TIME)) as f32)
                    .text(format!("time since start: {:.1} (s)", time.time)),
            );
            for (mut dlcc, _) in query_circs.iter_mut() {
                let r = &mut dlcc.0.circuit.resistance;
                ui.add(
                    egui::Slider::new(r, 0.00..=1.0)
                        .text("R (\u{03A9})")
                        .text_color(egui::Color32::WHITE)
                        .fixed_decimals(2),
                );
                let l = &mut dlcc.0.circuit.inductance;
                ui.add(
                    egui::Slider::new(l, 0.1..=10.0)
                        .text("L (H)")
                        .text_color(egui::Color32::WHITE)
                        .fixed_decimals(2),
                );
                let c = &mut dlcc.0.circuit.capacitance;
                ui.add(
                    egui::Slider::new(c, 0.1..=10.0)
                        .text("C (F)")
                        .text_color(egui::Color32::WHITE)
                        .fixed_decimals(2),
                );
                let start_q = &mut dlcc.0.circuit.startcharge;
                ui.add(
                    egui::Slider::new(start_q, 0.0..=50.0)
                        .text("starting Q (C)")
                        .text_color(egui::Color32::WHITE)
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

/// creates a window containing a plot of the current against time
fn circuit_plot(mut egui_ctx: ResMut<EguiContext>, query_circs: Query<&CurrentTimePlot>) {
    egui::Window::new("current")
        .title_bar(false)
        .anchor(Align2::LEFT_TOP, [0.0, 100.0])
        .fixed_size([400.0, 400.0])
        .collapsible(false)
        .frame(
            egui::Frame::dark_canvas(&egui_ctx.ctx_mut().style())
                .fill(egui::Color32::TRANSPARENT)
                .stroke(egui::Stroke {
                    width: 1.0,
                    color: egui::Color32::TRANSPARENT,
                }),
        )
        .show(egui_ctx.ctx_mut(), |ui| {
            for points in query_circs.iter() {
                let line = Line::new(Values::from_values_iter(
                    points.0.iter().map(|&(a, b)| Value::new(a, b)),
                ))
                .stroke(egui::Stroke {
                    color: Color32::from_rgb(0, 92, 128),
                    width: 3.0,
                });

                //stupid hack to get graph to have fixed axis
                //basically just add the boundry points to the graph
                let boundry_points = vec![
                    Value::new(MIN_CIRCUIT_TIME, -10.0),
                    Value::new(MIN_CIRCUIT_TIME, 10.0),
                    Value::new(MAX_CIRCUIT_TIME, -10.0),
                    Value::new(MAX_CIRCUIT_TIME, 10.0),
                ];
                Plot::new("")
                    .show_background(false)
                    .view_aspect(1.0)
                    .data_aspect(5.0)
                    .allow_scroll(false)
                    .allow_zoom(false)
                    .allow_drag(false)
                    .allow_boxed_zoom(false)
                    .center_y_axis(true)
                    .x_grid_spacer(|_| {
                        vec![GridMark {
                            value: 0.0,
                            step_size: f64::INFINITY,
                        }]
                    })
                    .y_grid_spacer(|_| {
                        vec![GridMark {
                            value: 0.0,
                            step_size: f64::INFINITY,
                        }]
                    })
                    .show_x(false)
                    .show_y(false)
                    .show(ui, |plot_ui| {
                        plot_ui.line(line);
                        plot_ui.points(
                            Points::new(Values::from_values(boundry_points))
                                .color(Color32::TRANSPARENT),
                        );
                        plot_ui.text(
                            Text::new(
                                Value::new(1.0, 10.0),
                                RichText::new("current").size(20.0).color(Color32::WHITE),
                            )
                            .anchor(Align2::LEFT_TOP),
                        );
                        plot_ui.text(
                            Text::new(
                                Value::new(90.0, -0.1),
                                RichText::new("time").size(20.0).color(Color32::WHITE),
                            )
                            .anchor(Align2::LEFT_TOP),
                        );
                    });
            }
        });
}
