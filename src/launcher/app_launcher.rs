use device_query::DeviceState;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use crate::audio::{note_manager, setup_realtime_audio};
use crate::input::key_logic;
use crate::{prints, synths};

pub fn launch_application() -> Result<(), Box<dyn std::error::Error>> {
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
        key_logic::key_management(&device_state, &mut previous_keys, &current_synth_type, &note_manager);
    }
}
