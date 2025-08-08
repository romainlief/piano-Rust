use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use synthesizer_emulation::audio::setup_realtime_audio;
use synthesizer_emulation::input::key_handlers;
use synthesizer_emulation::{prints, synths};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let active_frequencies: Arc<Mutex<HashSet<u64>>> = Arc::new(Mutex::new(HashSet::<u64>::new()));
    let current_synth_type: Arc<Mutex<synths::manager::SynthType>> = Arc::new(Mutex::new(
        synths::manager::SynthType::n_sine(),
    ));

    // Clone for the audio thread
    let frequencies_clone = Arc::clone(&active_frequencies);
    let synth_type_clone = Arc::clone(&current_synth_type);

    // Run the audio output in a separate thread
    setup_realtime_audio::run_output_polyphonic_realtime(frequencies_clone, synth_type_clone);

    prints::printfn::print_intro();

    let device_state = DeviceState::new();
    let mut previous_keys = HashSet::new();

    loop {
        let keys: HashSet<Keycode> = device_state.get_keys().into_iter().collect();
        // Check for pressed keys
        for key in keys.difference(&previous_keys) {
            key_handlers::matching_key_pressed(
                key.clone(),
                &active_frequencies,
                &current_synth_type,
            );
        }
        // Check for released keys
        for key in previous_keys.difference(&keys) {
            key_handlers::matching_key_released(key.clone(), &active_frequencies);
        }
        previous_keys = keys;
        // Small sleep to avoid busy-waiting
        std::thread::sleep(Duration::from_millis(10));
    }
}
