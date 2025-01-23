mod app;
mod audio;
mod utils;

use app::App;
use cpal::traits::{DeviceTrait, HostTrait};
use audio::run_audio_processing;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Trzeba tak, bo frequency będzie rozpatrywane w dwóch wątkach
    let frequency = Arc::new(Mutex::new(0.0));

    let host = cpal::default_host();

    // Wybór domyślnego mikrofonu
    let device = host
        .default_input_device()
        .expect("Nie znaleziono mikrofonu");

    // Jedyne co tak naprawdę potrzebujemy z configu to sample rate
    let config = device.default_input_config().unwrap();

    let sample_rate = config.sample_rate().0 as usize;
    let frame_size = 2048; // DO ZMIANY DO WYBORU

    let opoznienie: f32 = 1000.0 * (frame_size as f32) / (sample_rate as f32);
    println!("Opóźnienie ≈ {:.2}ms", opoznienie);

    println!("Mikrofon: {}", device.name().unwrap());
    println!("Konfiguracja: {:?}", config);

    // Tworzy instancje app z współdzieloną frequency
    let app = App::new(Arc::clone(&frequency));

    // Klonuje żeby zaraz przenieść do wątku
    let freq_clone = Arc::clone(&frequency);
    let threshold_clone = Arc::clone(&app.threshold);

    // Uruchom przetwarzanie audio w osobnym wątku
    thread::spawn(move || {
        run_audio_processing(freq_clone, sample_rate, frame_size, threshold_clone);
    });

    // Ustawienia okna aplikacji
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1000.0, 600.0)),
        ..Default::default()
    };

    // Uruchom aplikację!!!!!!!
    let _ = eframe::run_native(
        "Pitch detector",
        options,
        Box::new(|_cc| Box::new(app)),
    );
}
