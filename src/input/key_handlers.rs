use crate::audio::note_manager;
use crate::synths;
use device_query::Keycode;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize)]
pub struct Notes(pub HashMap<u8, HashMap<String, f64>>);
const VECTEUR_NOTES: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

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

    match key {
        Keycode::Q => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "A") {
                println!("Touche Q pressée - fréquence: {}", freq);
                note_manager::add_note(note_manager, freq, 44100.0);
            }
        }
        Keycode::B => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "B") {
                println!("Touche B pressée - fréquence: {}", freq);
                note_manager::add_note(note_manager, freq, 44100.0);
            }
        }
        Keycode::C => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "C") {
                println!("Touche C pressée - fréquence: {}", freq);
                note_manager::add_note(note_manager, freq, 44100.0);
            }
        }
        Keycode::D => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "D") {
                println!("Touche D pressée - fréquence: {}", freq);
                note_manager::add_note(note_manager, freq, 44100.0);
            }
        }
        Keycode::E => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "E") {
                println!("Touche E pressée - fréquence: {}", freq);
                note_manager::add_note(note_manager, freq, 44100.0);
            }
        }
        Keycode::F => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "F") {
                println!("Touche F pressée - fréquence: {}", freq);
                note_manager::add_note(note_manager, freq, 44100.0);
            }
        }
        Keycode::G => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "G") {
                println!("Touche G pressée - fréquence: {}", freq);
                note_manager::add_note(note_manager, freq, 44100.0);
            }
        }
        Keycode::Key1 => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "ASHARP") {
                println!("Touche 1 pressée - fréquence: {}", freq);
                note_manager::add_note(note_manager, freq, 44100.0);
            }
        }
        Keycode::Key2 => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "CSHARP") {
                println!("Touche 2 pressée - fréquence: {}", freq);
                note_manager::add_note(note_manager, freq, 44100.0);
            }
        }
        Keycode::Key3 => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "DSHARP") {
                println!("Touche 3 pressée - fréquence: {}", freq);
                note_manager::add_note(note_manager, freq, 44100.0);
            }
        }
        Keycode::Key4 => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "FSHARP") {
                println!("Touche 4 pressée - fréquence: {}", freq);
                note_manager::add_note(note_manager, freq, 44100.0);
            }
        }
        Keycode::Key5 => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "GSHARP") {
                println!("Touche 5 pressée - fréquence: {}", freq);
                note_manager::add_note(note_manager, freq, 44100.0);
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
            println!("Touche Flèche Gauche pressée");
        }
        Keycode::Right => {
            println!("Touche Flèche Droite pressée");
        }
        _ => {}
    }
}

pub fn matching_key_released(
    key: Keycode,
    _current_synth_type: &Arc<Mutex<synths::manager::SynthType>>,
    note_manager: &note_manager::ActiveNoteManager,
) {
    // Fonction helper pour récupérer la fréquence depuis le JSON
    let get_frequency =
        |octave: u8, note: &str| -> Option<f64> { NOTES.0.get(&octave)?.get(note).copied() };

    match key {
        Keycode::Q => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "A") {
                println!("Touche Q relâchée - fréquence: {}", freq);
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::B => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "B") {
                println!("Touche B relâchée - fréquence: {}", freq);
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::C => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "C") {
                println!("Touche C relâchée - fréquence: {}", freq);
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::D => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "D") {
                println!("Touche D relâchée - fréquence: {}", freq);
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::E => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "E") {
                println!("Touche E relâchée - fréquence: {}", freq);
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::F => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "F") {
                println!("Touche F relâchée - fréquence: {}", freq);
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::G => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "G") {
                println!("Touche G relâchée - fréquence: {}", freq);
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::Key1 => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "ASHARP") {
                println!("Touche 1 relâchée - fréquence: {}", freq);
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::Key2 => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "CSHARP") {
                println!("Touche 2 relâchée - fréquence: {}", freq);
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::Key3 => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "DSHARP") {
                println!("Touche 3 relâchée - fréquence: {}", freq);
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::Key4 => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "FSHARP") {
                println!("Touche 4 relâchée - fréquence: {}", freq);
                note_manager::release_note(note_manager, freq);
            }
        }
        Keycode::Key5 => {
            if let Some(freq) = get_frequency(VECTEUR_NOTES[3], "GSHARP") {
                println!("Touche 5 relâchée - fréquence: {}", freq);
                note_manager::release_note(note_manager, freq);
            }
        }
        _ => {}
    }
}
