mod app;
mod audio;
mod utils;

use app::App;
use cpal::traits::{DeviceTrait, HostTrait};
use audio::run_audio_processing;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let frequency = Arc::new(Mutex::new(0.0));

    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .expect("Nie znaleziono mikrofonu");
    let config = device.default_input_config().unwrap();

    let sample_rate = config.sample_rate().0 as usize;
    let frame_size = 2048; // to change according to needs

    let opoznienie: f32 = 1000.0 * (frame_size as f32) / (sample_rate as f32);
    println!("Opóźnienie ≈ {:.2}ms", opoznienie);

    println!("Mikrofon: {}", device.name().unwrap());
    println!("Konfiguracja: {:?}", config);

    let app = App::new(Arc::clone(&frequency));

    let freq_clone = Arc::clone(&frequency);
    let threshold_clone = Arc::clone(&app.threshold);

    thread::spawn(move || {
        run_audio_processing(freq_clone, sample_rate, frame_size, threshold_clone);
    });

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1000.0, 600.0)),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Pitch detector",
        options,
        Box::new(|_cc| Box::new(app)),
    );
}
