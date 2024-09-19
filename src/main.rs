use eframe::{egui, epi};

struct AspectRatioCalculator {
    width: f32,
    aspect_width: f32,
    aspect_height: f32,
    calculated_height: f32,
}

impl Default for AspectRatioCalculator {
    fn default() -> Self {
        Self {
            width: 1000.0,
            aspect_width: 3.0,
            aspect_height: 2.0,
            calculated_height: 0.0,
        }
    }
}

impl epi::App for AspectRatioCalculator {
    fn name(&self) -> &str {
        "Aspect Ratio Calculator"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Höhe berechnen");

            ui.horizontal(|ui| {
                ui.label("Breite:");
                ui.add(egui::DragValue::new(&mut self.width).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("Breite (Verhältnis):");
                ui.add(egui::DragValue::new(&mut self.aspect_width).speed(0.1));
            });

            ui.horizontal(|ui| {
                ui.label("Höhe (Verhältnis):");
                ui.add(egui::DragValue::new(&mut self.aspect_height).speed(0.1));
            });

            if ui.button("Berechnen").clicked() {
                self.calculated_height = self.width * (self.aspect_height / self.aspect_width);
            }

            ui.horizontal(|ui| {
                ui.label("Höhe (wird berechnet):");
                ui.label(format!("{:.2}", self.calculated_height));
            });
        });
    }
}

fn main() {
    let app = AspectRatioCalculator::default();
    let native_options = eframe::NativeOptions {
        always_on_top: true,
        initial_window_size: Some(egui::vec2(200.0, 200.0)),
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}