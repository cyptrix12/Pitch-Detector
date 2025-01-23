use crate::utils::closest_note_with_octave;
use eframe::egui;
use std::sync::{Arc, Mutex};

pub struct App {
    pub frequency: Arc<Mutex<f32>>,
    pub threshold: Arc<Mutex<f32>>,
}

impl App { //Tutaj praktycznie całość implementacji wprost z dokumentacji
    pub fn new(
        frequency: Arc<Mutex<f32>>
    ) -> Self {
        Self {
            frequency,
            threshold: Arc::new(Mutex::new(10.0)), // Domyślny próg na start
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Pobierz aktualną częstotliwość
        let freq = {
            let freq_lock = self.frequency.lock().unwrap();
            *freq_lock
        };

        // Pobierz aktualny próg
        let threshold = {
            let thresh_lock = self.threshold.lock().unwrap();
            *thresh_lock
        };

        // Sprawdź co to za nuta
        let (note, octave) = closest_note_with_octave(freq);
        let note_octave = format!("{}{}", note, octave);

        // Utwórz centralny panel
        // Szczerze to wybrałem akurat ten panel z dokumentacji,
        // ale pewnie lepiej by było to rozłożyć jakoś inaczej
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

                // Suwak do regulacji threshold
                // Walczyłem żeby był na środku ale nie podołałem
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
