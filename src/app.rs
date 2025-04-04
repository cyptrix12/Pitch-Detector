use crate::utils::closest_note_with_octave;
use eframe::egui;
use std::sync::{Arc, Mutex};

pub struct App {
    pub frequency: Arc<Mutex<f32>>,
    pub threshold: Arc<Mutex<f32>>,
}

impl App {
    pub fn new(
        frequency: Arc<Mutex<f32>>
    ) -> Self {
        Self {
            frequency,
            threshold: Arc::new(Mutex::new(10.0)),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let freq = {
            let freq_lock = self.frequency.lock().unwrap();
            *freq_lock
        };

        let threshold = {
            let thresh_lock = self.threshold.lock().unwrap();
            *thresh_lock
        };

        let (note, octave) = closest_note_with_octave(freq);
        let note_octave = format!("{}{}", note, octave);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(150.0);

                ui.label(
                    egui::RichText::new("Wysokość dźwięku (Hz):")
                        .color(egui::Color32::WHITE)
                        .font(egui::FontId::proportional(50.0)),
                );

                ui.label(
                    egui::RichText::new(format!("{:.2}", freq))
                        .color(egui::Color32::WHITE)
                        .font(egui::FontId::proportional(60.0)),
                );

                ui.label(
                    egui::RichText::new(note_octave)
                        .color(egui::Color32::WHITE)
                        .font(egui::FontId::proportional(60.0)),
                );
                ui.add_space(20.0);

                ui.horizontal_centered(|ui| {
                    ui.label("Min dB:");
                    let mut thresh = threshold;
                    ui.add(egui::Slider::new(&mut thresh, 0.0..=100.0).text("dB"));
                    *self.threshold.lock().unwrap() = thresh;
                });
                ui.add_space(20.0);
            });

        });

        ctx.request_repaint();
    }
}
