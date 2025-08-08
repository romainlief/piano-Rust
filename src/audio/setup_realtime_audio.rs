use crate::synths;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, SampleFormat, SizedSample, StreamConfig};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Polyphonic real-time version
pub fn run_output_polyphonic_realtime(
    frequencies: Arc<Mutex<HashSet<u64>>>,
    synth_type: Arc<Mutex<synths::manager::SynthType>>,
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
    synth_type: Arc<Mutex<synths::manager::SynthType>>,
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
    synth_type: &Arc<Mutex<synths::manager::SynthType>>,
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
