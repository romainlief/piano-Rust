use crate::audio::note_manager;
use crate::input::key_handlers;
use crate::synths;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Manages key inputs and synthesizer actions based on key events.
pub fn key_management(
    device_state: &DeviceState,
    previous_keys: &mut HashSet<Keycode>,
    current_synth_type: &Arc<Mutex<synths::manager::SynthType>>,
    note_manager: &note_manager::ActiveNoteManager,
) {
    let keys: HashSet<Keycode> = device_state.get_keys().into_iter().collect();

    // Check for pressed keys
    for key in keys.difference(previous_keys) {
        key_handlers::matching_key_pressed(key.clone(), current_synth_type, note_manager);
    }

    // Check for released keys
    for key in previous_keys.difference(&keys) {
        key_handlers::matching_key_released(key.clone(), current_synth_type, note_manager);
    }

    // Clean up finished notes
    note_manager::cleanup_finished_notes(note_manager);
    *previous_keys = keys;

    // Small sleep to avoid busy-waiting
    std::thread::sleep(Duration::from_millis(10));
}
