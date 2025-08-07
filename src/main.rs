use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, SampleFormat, SizedSample, StreamConfig};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use synthesizer_emulation::consts;
use synthesizer_emulation::synth;
use synthesizer_emulation::synth::types::{
    FMSynth, HammondSynth, SawtoothSynth, SineSynth, SquareSynth,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let active_frequencies = Arc::new(Mutex::new(HashSet::<u64>::new()));
    let current_synth_type = Arc::new(Mutex::new(
        synth::manager::SynthType::Sine(SineSynth::new()),
    ));

    // Clone for the audio thread
    let frequencies_clone = Arc::clone(&active_frequencies);
    let synth_type_clone = Arc::clone(&current_synth_type);

    // Run the audio output in a separate thread
    run_output_polyphonic_realtime(frequencies_clone, synth_type_clone);

    println!("Piano en temps réel avec synthétiseurs avancés !");
    println!("Touches musicales :");
    println!("Q-B-C-D-E-F-G - Notes naturelles");
    println!("1-2-3-4-5 - Notes dièses");
    println!();
    println!("Synthétiseurs :");
    println!("W - Sine basique        X - Square (50%)");
    println!("S - Sawtooth");
    println!("N - Hammond");
    println!("K - FM léger");
    println!();
    println!("ESPACE - Arrêter toutes les notes");
    println!("ESC - Quitter");

    let device_state = DeviceState::new();
    let mut previous_keys = HashSet::new();

    loop {
        let keys: HashSet<Keycode> = device_state.get_keys().into_iter().collect();

        // Check for pressed keys
        for key in keys.difference(&previous_keys) {
            match key {
                Keycode::Q => {
                    println!("Touche Q pressée - fréquence: {}", consts::constants::A4);
                    add_frequency_realtime(&active_frequencies, consts::constants::A4);
                }
                Keycode::B => {
                    println!("Touche B pressée - fréquence: {}", consts::constants::B4);
                    add_frequency_realtime(&active_frequencies, consts::constants::B4);
                }
                Keycode::C => {
                    println!("Touche C pressée - fréquence: {}", consts::constants::C5);
                    add_frequency_realtime(&active_frequencies, consts::constants::C5);
                }
                Keycode::D => {
                    println!("Touche D pressée - fréquence: {}", consts::constants::D5);
                    add_frequency_realtime(&active_frequencies, consts::constants::D5);
                }
                Keycode::E => {
                    println!("Touche E pressée - fréquence: {}", consts::constants::E5);
                    add_frequency_realtime(&active_frequencies, consts::constants::E5);
                }
                Keycode::F => {
                    println!("Touche F pressée - fréquence: {}", consts::constants::F5);
                    add_frequency_realtime(&active_frequencies, consts::constants::F5);
                }
                Keycode::G => {
                    println!("Touche G pressée - fréquence: {}", consts::constants::G5);
                    add_frequency_realtime(&active_frequencies, consts::constants::G5);
                }
                Keycode::Key1 => {
                    println!(
                        "Touche 1 pressée - fréquence: {}",
                        consts::constants::ASharp4
                    );
                    add_frequency_realtime(&active_frequencies, consts::constants::ASharp4);
                }
                Keycode::Key2 => {
                    println!(
                        "Touche 2 pressée - fréquence: {}",
                        consts::constants::CSharp5
                    );
                    add_frequency_realtime(&active_frequencies, consts::constants::CSharp5);
                }
                Keycode::Key3 => {
                    println!(
                        "Touche 3 pressée - fréquence: {}",
                        consts::constants::DSharp5
                    );
                    add_frequency_realtime(&active_frequencies, consts::constants::DSharp5);
                }
                Keycode::Key4 => {
                    println!(
                        "Touche 4 pressée - fréquence: {}",
                        consts::constants::FSharp5
                    );
                    add_frequency_realtime(&active_frequencies, consts::constants::FSharp5);
                }
                Keycode::Key5 => {
                    println!(
                        "Touche 5 pressée - fréquence: {}",
                        consts::constants::GSharp5
                    );
                    add_frequency_realtime(&active_frequencies, consts::constants::GSharp5);
                }
                Keycode::Space => {
                    println!("Espace pressé - arrêt de toutes les notes");
                    stop_all_frequencies_realtime(&active_frequencies);
                }
                Keycode::Z => {
                    *current_synth_type.lock().unwrap() =
                        synth::manager::SynthType::Sine(SineSynth::new());
                    println!("Synthétiseur changé: Sine");
                }
                Keycode::X => {
                    *current_synth_type.lock().unwrap() =
                        synth::manager::SynthType::Square(SquareSynth::new(0.5));
                    println!("Synthétiseur changé: Square 50%");
                }
                Keycode::S => {
                    *current_synth_type.lock().unwrap() =
                        synth::manager::SynthType::Sawtooth(SawtoothSynth::new());
                    println!("Synthétiseur changé: Sawtooth");
                }
                Keycode::N => {
                    *current_synth_type.lock().unwrap() =
                        synth::manager::SynthType::Hammond(HammondSynth::new());
                    println!("Synthétiseur changé: Hammond");
                }
                Keycode::K => {
                    *current_synth_type.lock().unwrap() =
                        synth::manager::SynthType::FM(FMSynth::new(1.5, 2.0));
                    println!("Synthétiseur changé: FM léger");
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
                    println!("Touche Q relâchée - fréquence: {}", consts::constants::A4);
                    remove_frequency_realtime(&active_frequencies, consts::constants::A4);
                }
                Keycode::B => {
                    println!("Touche B relâchée - fréquence: {}", consts::constants::B4);
                    remove_frequency_realtime(&active_frequencies, consts::constants::B4);
                }
                Keycode::C => {
                    println!("Touche C relâchée - fréquence: {}", consts::constants::C5);
                    remove_frequency_realtime(&active_frequencies, consts::constants::C5);
                }
                Keycode::D => {
                    println!("Touche D relâchée - fréquence: {}", consts::constants::D5);
                    remove_frequency_realtime(&active_frequencies, consts::constants::D5);
                }
                Keycode::E => {
                    println!("Touche E relâchée - fréquence: {}", consts::constants::E5);
                    remove_frequency_realtime(&active_frequencies, consts::constants::E5);
                }
                Keycode::F => {
                    println!("Touche F relâchée - fréquence: {}", consts::constants::F5);
                    remove_frequency_realtime(&active_frequencies, consts::constants::F5);
                }
                Keycode::G => {
                    println!("Touche G relâchée - fréquence: {}", consts::constants::G5);
                    remove_frequency_realtime(&active_frequencies, consts::constants::G5);
                }
                Keycode::Key1 => {
                    println!(
                        "Touche 1 relâchée - fréquence: {}",
                        consts::constants::ASharp4
                    );
                    remove_frequency_realtime(&active_frequencies, consts::constants::ASharp4);
                }
                Keycode::Key2 => {
                    println!(
                        "Touche 2 relâchée - fréquence: {}",
                        consts::constants::CSharp5
                    );
                    remove_frequency_realtime(&active_frequencies, consts::constants::CSharp5);
                }
                Keycode::Key3 => {
                    println!(
                        "Touche 3 relâchée - fréquence: {}",
                        consts::constants::DSharp5
                    );
                    remove_frequency_realtime(&active_frequencies, consts::constants::DSharp5);
                }
                Keycode::Key4 => {
                    println!(
                        "Touche 4 relâchée - fréquence: {}",
                        consts::constants::FSharp5
                    );
                    remove_frequency_realtime(&active_frequencies, consts::constants::FSharp5);
                }
                Keycode::Key5 => {
                    println!(
                        "Touche 5 relâchée - fréquence: {}",
                        consts::constants::GSharp5
                    );
                    remove_frequency_realtime(&active_frequencies, consts::constants::GSharp5);
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
fn run_output_polyphonic_realtime(
    frequencies: Arc<Mutex<HashSet<u64>>>,
    synth_type: Arc<Mutex<synth::manager::SynthType>>,
) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device.default_output_config().unwrap();
    match config.sample_format() {
        SampleFormat::F32 => {
            run_synth_polyphonic_realtime::<f32>(frequencies, synth_type, device, config.into())
        }
        SampleFormat::I16 => {
            run_synth_polyphonic_realtime::<i16>(frequencies, synth_type, device, config.into())
        }
        SampleFormat::U16 => {
            run_synth_polyphonic_realtime::<u16>(frequencies, synth_type, device, config.into())
        }

        _ => panic!("Unsupported format"),
    }
}

/// Real-time polyphonic synthesizer
fn run_synth_polyphonic_realtime<T: SizedSample + FromSample<f64>>(
    frequencies: Arc<Mutex<HashSet<u64>>>,
    synth_type: Arc<Mutex<synth::manager::SynthType>>,
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
        let synth_type_clone = Arc::clone(&synth_type);

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    write_data_polyphonic_realtime(
                        data,
                        channels,
                        &frequencies_clone,
                        &synth_type_clone,
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
    synth_type: &Arc<Mutex<synth::manager::SynthType>>,
    phases: &mut HashMap<u64, f64>,
    sample_rate: f64,
) {
    // Get the active frequencies
    let active_freq_keys = {
        let freqs = frequencies.lock().unwrap();
        freqs.clone()
    };

    // Get the current synthesizer type
    let current_synth_type = {
        let synth = synth_type.lock().unwrap();
        *synth
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
                println!(
                    "Debug: {} fréquences actives - Synth: {}",
                    active_freqs.len(),
                    current_synth_type.name()
                );
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

            // Use the selected synthesizer to generate the sample
            let wave_value = current_synth_type.generate_sample(*phase, freq);

            // Reduce the amplitude when there are multiple notes (avoid saturation)
            let amplitude = if active_freqs.len() > 1 {
                0.2 / active_freqs.len() as f64
            } else {
                0.2
            };

            sample_left += wave_value * amplitude;
            sample_right += wave_value * amplitude;

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
