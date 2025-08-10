use crate::audio::note_manager;
use crate::consts::constants::{CURRENT_OCTAVE_INDEX, VECTEUR_NOTES, SAMPLE_RATE};
use crate::synths;
use device_query::Keycode;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize)]
pub struct Notes(pub HashMap<u8, HashMap<String, f64>>);

pub static NOTES: Lazy<Notes> = Lazy::new(|| {
    let json_str = include_str!("../../res/notes.json");
    serde_json::from_str(json_str).expect("JSON invalide")
});

pub fn matching_key_pressed(
    key: Keycode,
    current_synth_type: &Arc<Mutex<synths::manager::SynthType>>,
    note_manager: &note_manager::ActiveNoteManager,
) {
    // Catch the octave and note to get the frequency in NOTES
    let get_frequency =
        |octave: u8, note: &str| -> Option<f64> { NOTES.0.get(&octave)?.get(note).copied() };

    let current_index = CURRENT_OCTAVE_INDEX.load(Ordering::Relaxed);
    let current_octave = VECTEUR_NOTES[current_index];

    match key {
        Keycode::Q => {
            if let Some(freq) = get_frequency(current_octave, "A") {
                println!(
                    "Touche Q pressée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::add_note(note_manager, freq, SAMPLE_RATE);
            }
        }
        Keycode::B => {
            if let Some(freq) = get_frequency(current_octave, "B") {
                println!(
                    "Touche B pressée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::add_note(note_manager, freq, SAMPLE_RATE);
            }
        }
        Keycode::C => {
            if let Some(freq) = get_frequency(current_octave, "C") {
                println!(
                    "Touche C pressée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::add_note(note_manager, freq, SAMPLE_RATE);
            }
        }
        Keycode::D => {
            if let Some(freq) = get_frequency(current_octave, "D") {
                println!(
                    "Touche D pressée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::add_note(note_manager, freq, SAMPLE_RATE);
            }
        }
        Keycode::E => {
            if let Some(freq) = get_frequency(current_octave, "E") {
                println!(
                    "Touche E pressée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::add_note(note_manager, freq, SAMPLE_RATE);
            }
        }
        Keycode::F => {
            if let Some(freq) = get_frequency(current_octave, "F") {
                println!(
                    "Touche F pressée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::add_note(note_manager, freq, SAMPLE_RATE);
            }
        }
        Keycode::G => {
            if let Some(freq) = get_frequency(current_octave, "G") {
                println!(
                    "Touche G pressée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::add_note(note_manager, freq, SAMPLE_RATE);
            }
        }
        Keycode::Key1 => {
            if let Some(freq) = get_frequency(current_octave, "ASHARP") {
                println!(
                    "Touche 1 pressée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::add_note(note_manager, freq, SAMPLE_RATE);
            }
        }
        Keycode::Key2 => {
            if let Some(freq) = get_frequency(current_octave, "CSHARP") {
                println!(
                    "Touche 2 pressée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::add_note(note_manager, freq, SAMPLE_RATE);
            }
        }
        Keycode::Key3 => {
            if let Some(freq) = get_frequency(current_octave, "DSHARP") {
                println!(
                    "Touche 3 pressée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::add_note(note_manager, freq, SAMPLE_RATE);
            }
        }
        Keycode::Key4 => {
            if let Some(freq) = get_frequency(current_octave, "FSHARP") {
                println!(
                    "Touche 4 pressée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::add_note(note_manager, freq, SAMPLE_RATE);
            }
        }
        Keycode::Key5 => {
            if let Some(freq) = get_frequency(current_octave, "GSHARP") {
                println!(
                    "Touche 5 pressée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::add_note(note_manager, freq, SAMPLE_RATE);
            }
        }
        Keycode::Space => {
            println!("Espace pressé - arrêt de toutes les notes");
            note_manager::stop_all_notes(note_manager);
        }
        Keycode::Z => {
            *current_synth_type.lock().unwrap() = synths::manager::SynthType::n_sine();
            println!("Synthétiseur changé: Modular Sine");
        }
        Keycode::X => {
            *current_synth_type.lock().unwrap() = synths::manager::SynthType::n_square();
            println!("Synthétiseur changé: Modular Square");
        }
        Keycode::S => {
            *current_synth_type.lock().unwrap() = synths::manager::SynthType::n_sawtooth();
            println!("Synthétiseur changé: Modular Sawtooth");
        }
        Keycode::K => {
            *current_synth_type.lock().unwrap() = synths::manager::SynthType::n_fm();
            println!("Synthétiseur changé: FM");
        }
        Keycode::H => {
            *current_synth_type.lock().unwrap() = synths::manager::SynthType::n_hammond();
            println!("Synthétiseur changé: Hammond Organ");
        }
        Keycode::Escape => {
            println!("\rAu revoir !");
            std::process::exit(0);
        }
        Keycode::Left => {
            // Octave - 1
            let current_index = CURRENT_OCTAVE_INDEX.load(Ordering::Relaxed);
            if current_index > 0 {
                CURRENT_OCTAVE_INDEX.store(current_index - 1, Ordering::Relaxed);
                let new_octave = VECTEUR_NOTES[current_index - 1];
                println!("Octave changée vers: {}", new_octave);
            } else {
                println!("Octave minimum atteinte ({})", VECTEUR_NOTES[0]);
            }
        }
        Keycode::Right => {
            // Octave + 1
            let current_index = CURRENT_OCTAVE_INDEX.load(Ordering::Relaxed);
            if current_index < VECTEUR_NOTES.len() - 1 {
                CURRENT_OCTAVE_INDEX.store(current_index + 1, Ordering::Relaxed);
                let new_octave = VECTEUR_NOTES[current_index + 1];
                println!("Octave changée vers: {})", new_octave);
            } else {
                println!(
                    "Octave maximum atteinte ({})",
                    VECTEUR_NOTES[VECTEUR_NOTES.len() - 1]
                );
            }
        }
        _ => {}
    }
}

pub fn matching_key_released(
    key: Keycode,
    _current_synth_type: &Arc<Mutex<synths::manager::SynthType>>,
    note_manager: &note_manager::ActiveNoteManager,
) {
    let get_frequency =
        |octave: u8, note: &str| -> Option<f64> { NOTES.0.get(&octave)?.get(note).copied() };

    let current_index = CURRENT_OCTAVE_INDEX.load(Ordering::Relaxed);
    let current_octave = VECTEUR_NOTES[current_index];

    match key {
        Keycode::Q => {
            if let Some(freq) = get_frequency(current_octave, "A") {
                println!(
                    "Touche Q relâchée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::B => {
            if let Some(freq) = get_frequency(current_octave, "B") {
                println!(
                    "Touche B relâchée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::C => {
            if let Some(freq) = get_frequency(current_octave, "C") {
                println!(
                    "Touche C relâchée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::D => {
            if let Some(freq) = get_frequency(current_octave, "D") {
                println!(
                    "Touche D relâchée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::E => {
            if let Some(freq) = get_frequency(current_octave, "E") {
                println!(
                    "Touche E relâchée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::F => {
            if let Some(freq) = get_frequency(current_octave, "F") {
                println!(
                    "Touche F relâchée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::G => {
            if let Some(freq) = get_frequency(current_octave, "G") {
                println!(
                    "Touche G relâchée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::Key1 => {
            if let Some(freq) = get_frequency(current_octave, "ASHARP") {
                println!(
                    "Touche 1 relâchée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::Key2 => {
            if let Some(freq) = get_frequency(current_octave, "CSHARP") {
                println!(
                    "Touche 2 relâchée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::Key3 => {
            if let Some(freq) = get_frequency(current_octave, "DSHARP") {
                println!(
                    "Touche 3 relâchée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::Key4 => {
            if let Some(freq) = get_frequency(current_octave, "FSHARP") {
                println!(
                    "Touche 4 relâchée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::Key5 => {
            if let Some(freq) = get_frequency(current_octave, "GSHARP") {
                println!(
                    "Touche 5 relâchée - octave: {} - fréquence: {}",
                    current_octave, freq
                );
                note_manager::release_note(note_manager, freq);
            }
        }
        _ => {}
    }
}
