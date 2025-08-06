use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, SampleFormat, SizedSample, StreamConfig};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;
mod consts;

// Trait for synthesizers
pub trait Synthesizer {
    fn generate_sample(&self, phase: f64, frequency: f64) -> f64;
    fn name(&self) -> &'static str;
}

// Basic sine wave synthesizer
#[derive(Clone, Copy, Debug)]
pub struct SineSynth {
    pub amplitude: f64,
}

impl SineSynth {
    pub fn new() -> Self {
        Self { amplitude: 1.0 }
    }
}

impl Synthesizer for SineSynth {
    fn generate_sample(&self, phase: f64, _frequency: f64) -> f64 {
        phase.sin() * self.amplitude
    }

    fn name(&self) -> &'static str {
        "Sine"
    }
}

// Square wave synthesizer
#[derive(Clone, Copy, Debug)]
pub struct SquareSynth {
    pub amplitude: f64,
    pub duty_cycle: f64, // Duty cycle (0.5 = 50%)
}

impl SquareSynth {
    pub fn new(duty_cycle: f64) -> Self {
        Self {
            amplitude: 1.0,
            duty_cycle: duty_cycle.clamp(0.1, 0.9),
        }
    }
}

impl Synthesizer for SquareSynth {
    fn generate_sample(&self, phase: f64, _frequency: f64) -> f64 {
        let normalized = (phase / (2.0 * std::f64::consts::PI)) % 1.0;
        if normalized < self.duty_cycle {
            self.amplitude
        } else {
            -self.amplitude
        }
    }

    fn name(&self) -> &'static str {
        "Square"
    }
}

// Sawtooth wave synthesizer
#[derive(Clone, Copy, Debug)]
pub struct SawtoothSynth {
    pub amplitude: f64,
    pub smoothness: f64, // Smoothness factor
}

impl SawtoothSynth {
    pub fn new() -> Self {
        Self {
            amplitude: 1.0,
            smoothness: 1.0,
        }
    }
}

impl Synthesizer for SawtoothSynth {
    fn generate_sample(&self, phase: f64, _frequency: f64) -> f64 {
        let normalized = phase / (2.0 * std::f64::consts::PI);
        let sawtooth = 2.0 * (normalized - (normalized + 0.5).floor()) - 1.0;
        sawtooth * self.amplitude * self.smoothness
    }

    fn name(&self) -> &'static str {
        "Sawtooth"
    }
}

// Hammond synthesizer with harmonic control
#[derive(Clone, Copy, Debug)]
pub struct HammondSynth {
    pub fundamental: f64,
    pub harmonic2: f64,
    pub harmonic3: f64,
    pub harmonic4: f64,
    pub harmonic5: f64,
}

impl HammondSynth {
    pub fn new() -> Self {
        Self {
            fundamental: 1.0,
            harmonic2: 0.5,
            harmonic3: 0.3,
            harmonic4: 0.2,
            harmonic5: 0.1,
        }
    }

    pub fn classic() -> Self {
        Self {
            fundamental: 1.0,
            harmonic2: 0.8,
            harmonic3: 0.6,
            harmonic4: 0.4,
            harmonic5: 0.2,
        }
    }
}

impl Synthesizer for HammondSynth {
    fn generate_sample(&self, phase: f64, _frequency: f64) -> f64 {
        let fund = phase.sin() * self.fundamental;
        let harm2 = (phase * 2.0).sin() * self.harmonic2;
        let harm3 = (phase * 3.0).sin() * self.harmonic3;
        let harm4 = (phase * 4.0).sin() * self.harmonic4;
        let harm5 = (phase * 5.0).sin() * self.harmonic5;

        fund + harm2 + harm3 + harm4 + harm5
    }

    fn name(&self) -> &'static str {
        "Hammond"
    }
}

// FM synthesizer (Frequency Modulation)
#[derive(Clone, Copy, Debug)]
pub struct FMSynth {
    pub carrier_amplitude: f64,
    pub modulator_frequency_ratio: f64,
    pub modulation_index: f64,
}

impl FMSynth {
    pub fn new(mod_freq_ratio: f64, mod_index: f64) -> Self {
        Self {
            carrier_amplitude: 1.0,
            modulator_frequency_ratio: mod_freq_ratio,
            modulation_index: mod_index,
        }
    }
}

impl Synthesizer for FMSynth {
    fn generate_sample(&self, phase: f64, frequency: f64) -> f64 {
        let modulator_phase = phase * self.modulator_frequency_ratio;
        let modulator = modulator_phase.sin() * self.modulation_index;

        (phase + modulator).sin() * self.carrier_amplitude
    }

    fn name(&self) -> &'static str {
        "FM"
    }
}

#[derive(Clone, Copy, Debug)]
// Enum to manage different synthesizer types
pub enum SynthType {
    Sine(SineSynth),
    Square(SquareSynth),
    Sawtooth(SawtoothSynth),
    Hammond(HammondSynth),
    FM(FMSynth),
}

impl SynthType {
    pub fn generate_sample(&self, phase: f64, frequency: f64) -> f64 {
        match self {
            SynthType::Sine(synth) => synth.generate_sample(phase, frequency),
            SynthType::Square(synth) => synth.generate_sample(phase, frequency),
            SynthType::Sawtooth(synth) => synth.generate_sample(phase, frequency),
            SynthType::Hammond(synth) => synth.generate_sample(phase, frequency),
            SynthType::FM(synth) => synth.generate_sample(phase, frequency),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            SynthType::Sine(synth) => synth.name(),
            SynthType::Square(synth) => synth.name(),
            SynthType::Sawtooth(synth) => synth.name(),
            SynthType::Hammond(synth) => synth.name(),
            SynthType::FM(synth) => synth.name(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let active_frequencies = Arc::new(Mutex::new(HashSet::<u64>::new()));
    let current_synth_type = Arc::new(Mutex::new(SynthType::Sine(SineSynth::new())));

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
    println!("S - Sawtooth           V - Square (25%)");
    println!("N - Hammond classique   H - Hammond moderne");
    println!(", - FM léger           K - FM intense");
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
                Keycode::Z => {
                    *current_synth_type.lock().unwrap() = SynthType::Sine(SineSynth::new());
                    println!("Synthétiseur changé: Sine");
                }
                Keycode::X => {
                    *current_synth_type.lock().unwrap() = SynthType::Square(SquareSynth::new(0.5));
                    println!("Synthétiseur changé: Square 50%");
                }
                Keycode::V => {
                    *current_synth_type.lock().unwrap() = SynthType::Square(SquareSynth::new(0.25));
                    println!("Synthétiseur changé: Square 25%");
                }
                Keycode::S => {
                    *current_synth_type.lock().unwrap() = SynthType::Sawtooth(SawtoothSynth::new());
                    println!("Synthétiseur changé: Sawtooth");
                }
                Keycode::N => {
                    *current_synth_type.lock().unwrap() = SynthType::Hammond(HammondSynth::new());
                    println!("Synthétiseur changé: Hammond");
                }
                Keycode::H => {
                    *current_synth_type.lock().unwrap() =
                        SynthType::Hammond(HammondSynth::classic());
                    println!("Synthétiseur changé: Hammond Classic");
                }
                Keycode::M => {
                    *current_synth_type.lock().unwrap() = SynthType::FM(FMSynth::new(1.5, 2.0));
                    println!("Synthétiseur changé: FM léger");
                }
                Keycode::K => {
                    *current_synth_type.lock().unwrap() = SynthType::FM(FMSynth::new(2.0, 5.0));
                    println!("Synthétiseur changé: FM intense");
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
fn run_output_polyphonic_realtime(
    frequencies: Arc<Mutex<HashSet<u64>>>,
    synth_type: Arc<Mutex<SynthType>>,
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
    synth_type: Arc<Mutex<SynthType>>,
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
    synth_type: &Arc<Mutex<SynthType>>,
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
