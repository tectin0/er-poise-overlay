#![windows_subsystem = "windows"]

use std::{collections::BTreeSet, time::Duration};

use eframe::egui;
use egui_plot::{uniform_grid_spacer, Line, Plot, PlotPoints, VLine};
use er_toughness_overlay::{
    error_to_cmd, get_toughness, is_key_event, is_mouse_event, RESOLUTION,
    TOUGHNESS_UPDATE_INTERVAL,
};

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 150.0])
            .with_position([RESOLUTION[0] as f32 - 400.0, 0.0])
            .with_transparent(true)
            .with_window_level(egui::WindowLevel::AlwaysOnTop)
            .with_decorations(false),

        ..Default::default()
    };

    match eframe::run_native(
        "Toughness Live",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            error_to_cmd(&format!("Failed to run GUI: {:?}", e));
            Err(e)
        }
    }
}

const TOUGHNESS_HASH_MULTIPLIER: f32 = 100000f32;
const TOUGHNESS_DISPLAY_MULTIPLIER: f64 = 10f64;

const HISTORY_SIZE: usize = 100;

#[derive(Default)]
struct App {
    current_toughness: f32,
    toughness_history: Vec<f64>,
    lmb_pressed_history: Vec<bool>,
    lmb_was_pressed: bool,
    unique_values: BTreeSet<i64>,
    last_update: Option<std::time::Instant>,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut unique_values = BTreeSet::new();
        unique_values.insert(0);

        let start_toughness = match get_toughness() {
            Ok(toughness) => toughness,
            Err(_error) => 0.0,
        };

        let toughness_history = vec![start_toughness as f64; HISTORY_SIZE];

        let lmb_pressed_history = vec![false; HISTORY_SIZE];

        App {
            current_toughness: start_toughness,
            unique_values,
            toughness_history,
            lmb_pressed_history,
            last_update: Some(std::time::Instant::now()),
            ..Default::default()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if is_key_event(&device_query::Keycode::F5) {
            self.unique_values.clear();
            self.unique_values.insert(0);
        }

        self.lmb_was_pressed = is_mouse_event(&1) || is_key_event(&device_query::Keycode::F);

        if self.last_update.unwrap().elapsed() > TOUGHNESS_UPDATE_INTERVAL {
            self.current_toughness = match get_toughness() {
                Ok(toughness) => toughness,
                Err(_error) => self.current_toughness,
            };

            self.unique_values
                .insert((self.current_toughness * TOUGHNESS_HASH_MULTIPLIER) as i64);

            self.toughness_history.push(self.current_toughness as f64);
            self.lmb_pressed_history.push(self.lmb_was_pressed);

            self.lmb_was_pressed = false;

            if self.toughness_history.len() > HISTORY_SIZE {
                self.toughness_history.remove(0);
                self.lmb_pressed_history.remove(0);
            }

            self.last_update = Some(std::time::Instant::now());
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::none().outer_margin(egui::Margin::symmetric(15.0, 5.0)))
            .show(ctx, |ui| {
                let plot = Plot::new("Toughness Plot")
                    .y_grid_spacer(uniform_grid_spacer(|_grid_input| [10f64, 50f64, 100f64]))
                    .x_axis_formatter(|_gridmark, _range| String::new());

                plot.show(ui, |plot_ui| {
                    let lmb_pressed_positions = self
                        .lmb_pressed_history
                        .iter()
                        .enumerate()
                        .filter(|(_, &pressed)| pressed)
                        .map(|(i, _)| i as f64)
                        .collect::<Vec<f64>>();

                    for &pos in &lmb_pressed_positions {
                        let vline = VLine::new(pos)
                            .color(egui::Color32::from_rgb(200, 0, 0))
                            .width(0.5);

                        plot_ui.vline(vline);
                    }

                    let line = Line::new(
                        self.toughness_history
                            .iter()
                            .enumerate()
                            .map(|(i, &t)| [i as f64, t * TOUGHNESS_DISPLAY_MULTIPLIER])
                            .collect::<PlotPoints>(),
                    );

                    plot_ui.line(line);

                    for &toughness in &self.unique_values {
                        let y = toughness as f64 / TOUGHNESS_HASH_MULTIPLIER as f64
                            * TOUGHNESS_DISPLAY_MULTIPLIER;
                        plot_ui.line(
                            Line::new(PlotPoints::new(vec![[0.0, y], [100.0, y]]))
                                .color(egui::Color32::from_rgb(255, 0, 255))
                                .width(1.0),
                        );
                    }
                });
            });

        ctx.request_repaint_after(Duration::from_millis(25));
    }
}
