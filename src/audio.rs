use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Sample;
use rustfft::num_complex::Complex;
use rustfft::FftPlanner;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run_audio_processing(
    frequency: Arc<Mutex<f32>>,
    sample_rate: usize,
    frame_size: usize,
    threshold: Arc<Mutex<f32>>,
) {
    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .expect("Nie znaleziono mikrofonu");

    let config = device.default_input_config().unwrap();

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => run_stream::<f32>(
            &device,
            &config.into(),
            frequency,
            sample_rate,
            frame_size,
            threshold,
        ),
        cpal::SampleFormat::I16 => run_stream::<i16>(
            &device,
            &config.into(),
            frequency,
            sample_rate,
            frame_size,
            threshold,
        ),
        cpal::SampleFormat::U16 => run_stream::<u16>(
            &device,
            &config.into(),
            frequency,
            sample_rate,
            frame_size,
            threshold,
        ),
    };

    stream.play().unwrap();

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

pub fn run_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    frequency: Arc<Mutex<f32>>,
    sample_rate: usize,
    frame_size: usize,
    threshold: Arc<Mutex<f32>>,
) -> cpal::Stream
where
    T: Sample + Send + 'static,
    f32: From<T>,
{
    let mut samples = Vec::with_capacity(frame_size);

    // Hanning window
    let window: Vec<f32> = (0..frame_size)
        .map(|i| {
            0.5 - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / frame_size as f32).cos()
        })
        .collect();

    

    let err_fn = |err| eprintln!("Błąd: {}", err);

    device
        .build_input_stream(
            config,
            move |data: &[T], _: &cpal::InputCallbackInfo| {
                for &sample in data {
                    let sample: f32 = Sample::to_f32(&sample);
                    samples.push(sample);
                    if samples.len() == frame_size {
                        let windowed_samples: Vec<f32> = samples
                            .iter()
                            .zip(window.iter())
                            .map(|(&s, &w)| s * w)
                            .collect();

                        let mut planner = FftPlanner::new();
                        let fft = planner.plan_fft_forward(frame_size);

                        let mut buffer: Vec<Complex<f32>> = windowed_samples
                            .iter()
                            .map(|&s| Complex { re: s, im: 0.0 })
                            .collect();

                        fft.process(&mut buffer);

                        let magnitudes_calc: Vec<f32> = buffer
                            .iter()
                            .take(frame_size / 2)
                            .map(|c| c.norm())
                            .collect();

                        let current_threshold = {
                            let thresh_lock = threshold.lock().unwrap();
                            *thresh_lock
                        };

                        let mut peak_indices = Vec::new();
                        for i in 1..(frame_size / 2 - 1) {
                            if magnitudes_calc[i] > magnitudes_calc[i - 1]
                                && magnitudes_calc[i] > magnitudes_calc[i + 1]
                                && magnitudes_calc[i] > current_threshold
                            {
                                peak_indices.push(i);
                            }
                        }

                        if let Some(&fundamental_index) = peak_indices.first() {
                            let alpha = magnitudes_calc[fundamental_index - 1];
                            let beta = magnitudes_calc[fundamental_index];
                            let gamma = magnitudes_calc[fundamental_index + 1];
                            let p = 0.5 * (alpha - gamma) / (alpha - 2.0 * beta + gamma);
                            let freq_bin = fundamental_index as f32 + p;

                            let freq = 2.0 * freq_bin * sample_rate as f32 / frame_size as f32;

                            {
                                let mut freq_lock = frequency.lock().unwrap();
                                *freq_lock = freq;
                            }

                        }

                        samples.clear();
                    }
                }
            },
            err_fn,
        )
        .unwrap()
}
