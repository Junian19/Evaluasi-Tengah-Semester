use eframe::egui;
use rand::Rng;
use std::{thread, time::Duration};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 500.0])
            .with_title("Rain Monitoring System"),
        ..Default::default()
    };

    eframe::run_native(
        "Rain Monitoring System",
        options,
        Box::new(|_cc| Box::new(RainApp::default())),
    )
}

struct RainApp {
    rain_value: f32,
    status: String,
    history: Vec<f32>,
}

impl Default for RainApp {
    fn default() -> Self {
        Self {
            rain_value: 0.0,
            status: "Tidak Hujan".to_string(),
            history: vec![0.0; 20],
        }
    }
}

// =========================
// FUNGSI VALIDASI DATA
// =========================
fn validate_sensor(value: f32) -> f32 {
    // Validasi data sensor
    if value < 0.0 {
        0.0
    } else if value > 100.0 {
        100.0
    } else {
        value
    }
}

// =========================
// FUNGSI STATUS HUJAN
// =========================
fn rain_status(value: f32) -> String {
    // Percabangan
    if value < 30.0 {
        "Tidak Hujan".to_string()
    } else if value < 70.0 {
        "Gerimis".to_string()
    } else {
        "Hujan Deras".to_string()
    }
}

// =========================
// FUNGSI UPDATE HISTORY
// =========================
fn update_history(history: &mut Vec<f32>, value: f32) {
    history.push(value);

    if history.len() > 20 {
        history.remove(0);
    }
}

// =========================
// MOVING AVERAGE
// =========================
fn moving_average(data: &Vec<f32>) -> f32 {
    let mut total = 0.0;

    // Perulangan penjumlahan data
    for value in data {
        total += value;
    }

    total / data.len() as f32
}

impl eframe::App for RainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // =========================
        // INPUT DATA SENSOR DUMMY
        // =========================
        let mut rng = rand::thread_rng();
        let sensor_input = rng.gen_range(0.0..120.0);

        // =========================
        // VALIDASI DATA
        // =========================
        self.rain_value = validate_sensor(sensor_input);

        // =========================
        // PERCABANGAN STATUS
        // =========================
        self.status = rain_status(self.rain_value);

        // =========================
        // UPDATE HISTORY
        // =========================
        update_history(&mut self.history, self.rain_value);

        // =========================
        // MOVING AVERAGE FILTER
        // =========================
        let average = moving_average(&self.history);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🌧 Rain Monitoring Dashboard");
            ui.separator();

            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(250.0, 150.0));
                    ui.heading("Status Hujan");
                    ui.add_space(20.0);

                    ui.label(
                        egui::RichText::new(&self.status)
                            .size(30.0)
                            .strong(),
                    );
                });

                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(250.0, 150.0));
                    ui.heading("Intensitas Sensor");
                    ui.add_space(20.0);

                    ui.label(
                        egui::RichText::new(format!("{:.2}%", self.rain_value))
                            .size(35.0)
                            .color(egui::Color32::LIGHT_BLUE)
                            .strong(),
                    );
                });
            });

            ui.add_space(20.0);

            ui.group(|ui| {
                ui.heading("Moving Average Sensor");
                ui.add_space(10.0);

                ui.label(
                    egui::RichText::new(format!("Rata-rata: {:.2}%", average))
                        .size(28.0)
                        .color(egui::Color32::GREEN)
                        .strong(),
                );
            });

            ui.add_space(20.0);
            ui.separator();
            ui.heading("Grafik Intensitas Hujan");
            ui.add_space(10.0);

            let desired_size = egui::vec2(800.0, 200.0);
            let (response, painter) =
                ui.allocate_painter(desired_size, egui::Sense::hover());

            let rect = response.rect;
            let width = rect.width();
            let height = rect.height();

            let max_points = self.history.len();

            // =========================
            // PERULANGAN UNTUK GRAFIK
            // =========================
            for i in 1..max_points {
                let x1 = rect.left() + (i as f32 - 1.0) * width / max_points as f32;
                let y1 = rect.bottom()
                    - (self.history[i - 1] / 100.0) * height;

                let x2 = rect.left() + i as f32 * width / max_points as f32;
                let y2 = rect.bottom()
                    - (self.history[i] / 100.0) * height;

                painter.line_segment(
                    [egui::pos2(x1, y1), egui::pos2(x2, y2)],
                    egui::Stroke::new(3.0, egui::Color32::LIGHT_BLUE),
                );
            }
        });

        // Refresh GUI tiap 1 detik
        thread::sleep(Duration::from_millis(1000));
        ctx.request_repaint();
    }
}