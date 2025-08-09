use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use synthesizer_emulation::audio::{note_manager, setup_realtime_audio};
use synthesizer_emulation::input::key_handlers;
use synthesizer_emulation::{prints, synths};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Nouveau système : gestionnaire de notes avec ADSR individuels
    let note_manager = note_manager::create_note_manager();
    let current_synth_type: Arc<Mutex<synths::manager::SynthType>> =
        Arc::new(Mutex::new(synths::manager::SynthType::n_sine()));

    // Clone for the audio thread
    let notes_clone = Arc::clone(&note_manager);
    let synth_type_clone = Arc::clone(&current_synth_type);

    // Run the audio output in a separate thread
    setup_realtime_audio::run_output_polyphonic_realtime(notes_clone, synth_type_clone);

    prints::printfn::print_intro();

    let device_state = DeviceState::new();
    let mut previous_keys = HashSet::new();

    loop {
        let keys: HashSet<Keycode> = device_state.get_keys().into_iter().collect();
        // Check for pressed keys
        for key in keys.difference(&previous_keys) {
            key_handlers::matching_key_pressed(key.clone(), &current_synth_type, &note_manager);
        }
        // Check for released keys
        for key in previous_keys.difference(&keys) {
            key_handlers::matching_key_released(key.clone(), &current_synth_type, &note_manager);
        }

        // Nettoie les notes finies
        note_manager::cleanup_finished_notes(&note_manager);
        previous_keys = keys;
        // Small sleep to avoid busy-waiting
        std::thread::sleep(Duration::from_millis(10));
    }
}
