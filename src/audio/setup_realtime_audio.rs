use crate::audio::note_manager;
use crate::synths;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, SampleFormat, SizedSample, StreamConfig};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Polyphonic real-time version using note_manager
pub fn run_output_polyphonic_realtime(
    notes: note_manager::ActiveNoteManager,
    synth_type: Arc<Mutex<synths::manager::SynthType>>,
) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device.default_output_config().unwrap();
    match config.sample_format() {
        SampleFormat::F32 => {
            run_synth_polyphonic_realtime::<f32>(notes, synth_type, device, config.into())
        }
        SampleFormat::I16 => {
            run_synth_polyphonic_realtime::<i16>(notes, synth_type, device, config.into())
        }
        SampleFormat::U16 => {
            run_synth_polyphonic_realtime::<u16>(notes, synth_type, device, config.into())
        }

        _ => panic!("Unsupported format"),
    }
}

/// Real-time polyphonic synthesizer using note_manager
fn run_synth_polyphonic_realtime<T: SizedSample + FromSample<f64>>(
    notes: note_manager::ActiveNoteManager,
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

        let notes_clone = Arc::clone(&notes);
        let synth_type_clone = Arc::clone(&synth_type);

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    write_data_polyphonic_realtime(
                        data,
                        channels,
                        &notes_clone,
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

/// Generate polyphonic audio samples using note_manager
fn write_data_polyphonic_realtime<T: SizedSample + FromSample<f64>>(
    output: &mut [T],
    channels: usize,
    notes: &note_manager::ActiveNoteManager,
    synth_type: &Arc<Mutex<synths::manager::SynthType>>,
    phases: &mut HashMap<u64, f64>,
    sample_rate: f64,
) {
    // Get the current synthesizer type
    let mut current_synth_type = {
        let synth = synth_type.lock().unwrap();
        synth.clone()
    };

    for frame in output.chunks_mut(channels) {
        let mut sample = 0.0;

        // Lock notes for the entire frame processing to avoid multiple locks
        {
            let mut notes_guard = notes.lock().unwrap();

            // Process each active note with its individual ADSR
            for (frequency_key, active_note) in notes_guard.iter_mut() {
                let frequency = active_note.frequency;

                // Get or initialize phase for this frequency
                let phase = phases.entry(*frequency_key).or_insert(0.0);

                // Generate the base oscillator sample
                // Convertir la phase [0,1] vers [0,2π] pour les oscillateurs
                let phase_radians = *phase * 2.0 * std::f64::consts::PI;
                let oscillator_sample = current_synth_type.generate_sample(phase_radians, frequency);

                // Apply the individual ADSR envelope - THIS is the crucial fix!
                let adsr_amplitude = active_note.get_amplitude();
                let final_sample = oscillator_sample * adsr_amplitude;

                // Add to the mix
                sample += final_sample;

                // Update phase
                *phase += frequency / sample_rate;
                if *phase >= 1.0 {
                    *phase -= 1.0;
                }
            }

            // Normalize by number of active notes to prevent clipping but keep good volume
            if !notes_guard.is_empty() {
                // Moins de division pour un son plus fort
                let note_count = notes_guard.len() as f64;
                if note_count > 1.0 {
                    sample /= note_count.sqrt(); // Division par racine carrée pour préserver le volume
                }
                // Amplification finale
                sample *= 1.5; // Boost le volume final
            }
        } // Release the lock here

        // Fill all channels with the same sample
        for sample_slot in frame.iter_mut() {
            *sample_slot = T::from_sample(sample);
        }
    }

    // Clean up finished notes phases - separate lock to avoid holding too long
    {
        let notes_guard = notes.lock().unwrap();
        let finished_notes: Vec<u64> = notes_guard
            .iter()
            .filter(|(_, note)| note.is_finished())
            .map(|(&key, _)| key)
            .collect();

        for key in finished_notes {
            phases.remove(&key);
        }
    }
}
