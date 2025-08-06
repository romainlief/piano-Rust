use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, SampleFormat, SizedSample, StreamConfig};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;
mod consts;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let active_frequencies = Arc::new(Mutex::new(HashSet::<u64>::new()));

    // Clone for the audio thread
    let frequencies_clone = Arc::clone(&active_frequencies);

    // Run the audio output in a separate thread
    run_output_polyphonic_realtime(frequencies_clone);

    println!("Piano en temps réel ! Maintenez les touches pour jouer :");
    println!("a - A 440Hz         1 - A#/Bb 466.16Hz");
    println!("b - B 493.88Hz      2 - C#/Db 554.37Hz");
    println!("c - C 523.25Hz      3 - D#/Eb 622.25Hz");
    println!("d - D 587.33Hz      4 - F#/Gb 739.99Hz");
    println!("e - E 659.26Hz      5 - G#/Ab 830.61Hz");
    println!("f - F 698.46Hz");
    println!("g - G 783.99Hz");
    println!();
    println!("ESPACE - Arrêter toutes les notes");
    println!("ESC - Quitter");
    println!("Appuyez sur ESC pour quitter...");

    let device_state = DeviceState::new();
    let mut previous_keys = HashSet::new();

    loop {
        let keys: HashSet<Keycode> = device_state.get_keys().into_iter().collect();

        // Check for pressed keys
        for key in keys.difference(&previous_keys) {
            match key {
                Keycode::Q => {
                    println!("Touche Q pressée - fréquence: {}", consts::A4);
                    add_frequency_realtime(&active_frequencies, consts::A4);
                }
                Keycode::B => {
                    println!("Touche B pressée - fréquence: {}", consts::B4);
                    add_frequency_realtime(&active_frequencies, consts::B4);
                }
                Keycode::C => {
                    println!("Touche C pressée - fréquence: {}", consts::C5);
                    add_frequency_realtime(&active_frequencies, consts::C5);
                }
                Keycode::D => {
                    println!("Touche D pressée - fréquence: {}", consts::D5);
                    add_frequency_realtime(&active_frequencies, consts::D5);
                }
                Keycode::E => {
                    println!("Touche E pressée - fréquence: {}", consts::E5);
                    add_frequency_realtime(&active_frequencies, consts::E5);
                }
                Keycode::F => {
                    println!("Touche F pressée - fréquence: {}", consts::F5);
                    add_frequency_realtime(&active_frequencies, consts::F5);
                }
                Keycode::G => {
                    println!("Touche G pressée - fréquence: {}", consts::G5);
                    add_frequency_realtime(&active_frequencies, consts::G5);
                }
                Keycode::Key1 => {
                    println!("Touche 1 pressée - fréquence: {}", consts::ASharp4);
                    add_frequency_realtime(&active_frequencies, consts::ASharp4);
                }
                Keycode::Key2 => {
                    println!("Touche 2 pressée - fréquence: {}", consts::CSharp5);
                    add_frequency_realtime(&active_frequencies, consts::CSharp5);
                }
                Keycode::Key3 => {
                    println!("Touche 3 pressée - fréquence: {}", consts::DSharp5);
                    add_frequency_realtime(&active_frequencies, consts::DSharp5);
                }
                Keycode::Key4 => {
                    println!("Touche 4 pressée - fréquence: {}", consts::FSharp5);
                    add_frequency_realtime(&active_frequencies, consts::FSharp5);
                }
                Keycode::Key5 => {
                    println!("Touche 5 pressée - fréquence: {}", consts::GSharp5);
                    add_frequency_realtime(&active_frequencies, consts::GSharp5);
                }
                Keycode::Space => {
                    println!("Espace pressé - arrêt de toutes les notes");
                    stop_all_frequencies_realtime(&active_frequencies);
                }
                Keycode::Escape => {
                    println!("\rAu revoir !");
                    return Ok(());
                }
                _ => {}
            }
        }

        // Check for released keys
        for key in previous_keys.difference(&keys) {
            match key {
                Keycode::Q => {
                    println!("Touche Q relâchée - fréquence: {}", consts::A4);
                    remove_frequency_realtime(&active_frequencies, consts::A4);
                }
                Keycode::B => {
                    println!("Touche B relâchée - fréquence: {}", consts::B4);
                    remove_frequency_realtime(&active_frequencies, consts::B4);
                }
                Keycode::C => {
                    println!("Touche C relâchée - fréquence: {}", consts::C5);
                    remove_frequency_realtime(&active_frequencies, consts::C5);
                }
                Keycode::D => {
                    println!("Touche D relâchée - fréquence: {}", consts::D5);
                    remove_frequency_realtime(&active_frequencies, consts::D5);
                }
                Keycode::E => {
                    println!("Touche E relâchée - fréquence: {}", consts::E5);
                    remove_frequency_realtime(&active_frequencies, consts::E5);
                }
                Keycode::F => {
                    println!("Touche F relâchée - fréquence: {}", consts::F5);
                    remove_frequency_realtime(&active_frequencies, consts::F5);
                }
                Keycode::G => {
                    println!("Touche G relâchée - fréquence: {}", consts::G5);
                    remove_frequency_realtime(&active_frequencies, consts::G5);
                }
                Keycode::Key1 => {
                    println!("Touche 1 relâchée - fréquence: {}", consts::ASharp4);
                    remove_frequency_realtime(&active_frequencies, consts::ASharp4);
                }
                Keycode::Key2 => {
                    println!("Touche 2 relâchée - fréquence: {}", consts::CSharp5);
                    remove_frequency_realtime(&active_frequencies, consts::CSharp5);
                }
                Keycode::Key3 => {
                    println!("Touche 3 relâchée - fréquence: {}", consts::DSharp5);
                    remove_frequency_realtime(&active_frequencies, consts::DSharp5);
                }
                Keycode::Key4 => {
                    println!("Touche 4 relâchée - fréquence: {}", consts::FSharp5);
                    remove_frequency_realtime(&active_frequencies, consts::FSharp5);
                }
                Keycode::Key5 => {
                    println!("Touche 5 relâchée - fréquence: {}", consts::GSharp5);
                    remove_frequency_realtime(&active_frequencies, consts::GSharp5);
                }
                _ => {}
            }
        }

        previous_keys = keys;

        // Small sleep to avoid busy-waiting
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn add_frequency_realtime(frequencies: &Arc<Mutex<HashSet<u64>>>, freq: f64) {
    let mut freqs = frequencies.lock().unwrap();
    let freq_key = freq.to_bits();
    if freqs.insert(freq_key) {
        println!("\rNote ON: {:.2} Hz. Notes actives: {}", freq, freqs.len());
    }
}

fn remove_frequency_realtime(frequencies: &Arc<Mutex<HashSet<u64>>>, freq: f64) {
    let mut freqs = frequencies.lock().unwrap();
    let freq_key = freq.to_bits();
    if freqs.remove(&freq_key) {
        println!("\rNote OFF: {:.2} Hz. Notes actives: {}", freq, freqs.len());
    }
}

fn stop_all_frequencies_realtime(frequencies: &Arc<Mutex<HashSet<u64>>>) {
    let mut freqs = frequencies.lock().unwrap();
    freqs.clear();
    println!("\rToutes les notes arrêtées.");
}

/// Polyphonic real-time version
fn run_output_polyphonic_realtime(frequencies: Arc<Mutex<HashSet<u64>>>) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device.default_output_config().unwrap();
    match config.sample_format() {
        SampleFormat::F32 => {
            run_synth_polyphonic_realtime::<f32>(frequencies, device, config.into())
        }
        SampleFormat::I16 => {
            run_synth_polyphonic_realtime::<i16>(frequencies, device, config.into())
        }
        SampleFormat::U16 => {
            run_synth_polyphonic_realtime::<u16>(frequencies, device, config.into())
        }

        _ => panic!("Unsupported format"),
    }
}

/// Real-time polyphonic synthesizer
fn run_synth_polyphonic_realtime<T: SizedSample + FromSample<f64>>(
    frequencies: Arc<Mutex<HashSet<u64>>>,
    device: Device,
    config: StreamConfig,
) {
    std::thread::spawn(move || {
        let sample_rate = config.sample_rate.0 as f64;
        let channels = config.channels as usize;
        let err_fn = |err| eprintln!("an error occurred on stream: {err}");

        // Variables for oscillators
        let mut phases: HashMap<u64, f64> = HashMap::new();

        let frequencies_clone = Arc::clone(&frequencies);

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    write_data_polyphonic_realtime(
                        data,
                        channels,
                        &frequencies_clone,
                        &mut phases,
                        sample_rate,
                    )
                },
                err_fn,
                None,
            )
            .unwrap();

        stream.play().unwrap();
        loop {
            std::thread::sleep(Duration::from_millis(1));
        }
    });
}

/// Generate polyphonic audio samples in real-time
fn write_data_polyphonic_realtime<T: SizedSample + FromSample<f64>>(
    output: &mut [T],
    channels: usize,
    frequencies: &Arc<Mutex<HashSet<u64>>>,
    phases: &mut HashMap<u64, f64>,
    sample_rate: f64,
) {
    // Get the active frequencies
    let active_freq_keys = {
        let freqs = frequencies.lock().unwrap();
        freqs.clone()
    };

    // Convert the active frequency keys to f64
    let active_freqs: Vec<(u64, f64)> = active_freq_keys
        .iter()
        .map(|&key| (key, f64::from_bits(key)))
        .collect();

    static mut DEBUG_COUNTER: u64 = 0;
    unsafe {
        DEBUG_COUNTER += 1;
        if DEBUG_COUNTER % 44100 == 0 {
            // Once per second approximately
            if !active_freqs.is_empty() {
                println!("Debug: {} fréquences actives", active_freqs.len());
                for &(_, freq) in &active_freqs {
                    println!("  - {:.2} Hz", freq);
                }
            }
        }
    }

    for frame in output.chunks_mut(channels) {
        let mut sample_left = 0.0;
        let mut sample_right = 0.0;

        // Generate and sum all active oscillators
        for &(freq_key, freq) in &active_freqs {
            let phase = phases.entry(freq_key).or_insert(0.0);
            let phase_increment = 2.0 * std::f64::consts::PI * freq / sample_rate;
            let sine_value = phase.sin();

            // Reduce the amplitude when there are multiple notes (avoid saturation)
            let amplitude = if active_freqs.len() > 1 {
                0.3 / active_freqs.len() as f64
            } else {
                0.3
            };

            sample_left += sine_value * amplitude;
            sample_right += sine_value * amplitude;

            *phase += phase_increment;
            if *phase > 2.0 * std::f64::consts::PI {
                *phase -= 2.0 * std::f64::consts::PI;
            }
        }

        // Clean up the phases of frequencies that are no longer active
        phases.retain(|k, _| active_freq_keys.contains(k));

        let left: T = T::from_sample(sample_left);
        let right: T = T::from_sample(sample_right);

        for (channel, sample) in frame.iter_mut().enumerate() {
            *sample = if channel & 1 == 0 { left } else { right };
        }
    }
}
