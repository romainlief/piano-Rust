use crate::audio::note_manager;
use crate::consts;
use crate::synths;
use device_query::Keycode;
use std::sync::{Arc, Mutex};

pub fn matching_key_pressed(
    key: Keycode,
    current_synth_type: &Arc<Mutex<synths::manager::SynthType>>,
    note_manager: &note_manager::ActiveNoteManager,
) {
    match key {
        Keycode::Q => {
            println!("Touche Q pressée - fréquence: {}", consts::constants::A4);
            note_manager::add_note(note_manager, consts::constants::A4, 44100.0);
        }
        Keycode::B => {
            println!("Touche B pressée - fréquence: {}", consts::constants::B4);
            note_manager::add_note(note_manager, consts::constants::B4, 44100.0);
        }
        Keycode::C => {
            println!("Touche C pressée - fréquence: {}", consts::constants::C5);
            note_manager::add_note(note_manager, consts::constants::C5, 44100.0);
        }
        Keycode::D => {
            println!("Touche D pressée - fréquence: {}", consts::constants::D5);
            note_manager::add_note(note_manager, consts::constants::D5, 44100.0);
        }
        Keycode::E => {
            println!("Touche E pressée - fréquence: {}", consts::constants::E5);
            note_manager::add_note(note_manager, consts::constants::E5, 44100.0);
        }
        Keycode::F => {
            println!("Touche F pressée - fréquence: {}", consts::constants::F5);
            note_manager::add_note(note_manager, consts::constants::F5, 44100.0);
        }
        Keycode::G => {
            println!("Touche G pressée - fréquence: {}", consts::constants::G5);
            note_manager::add_note(note_manager, consts::constants::G5, 44100.0);
        }
        Keycode::Key1 => {
            println!(
                "Touche 1 pressée - fréquence: {}",
                consts::constants::ASHARP4
            );
            note_manager::add_note(note_manager, consts::constants::ASHARP4, 44100.0);
        }
        Keycode::Key2 => {
            println!(
                "Touche 2 pressée - fréquence: {}",
                consts::constants::CSHARP5
            );
            note_manager::add_note(note_manager, consts::constants::CSHARP5, 44100.0);
        }
        Keycode::Key3 => {
            println!(
                "Touche 3 pressée - fréquence: {}",
                consts::constants::DSHARP5
            );
            note_manager::add_note(note_manager, consts::constants::DSHARP5, 44100.0);
        }
        Keycode::Key4 => {
            println!(
                "Touche 4 pressée - fréquence: {}",
                consts::constants::FSHARP5
            );
            note_manager::add_note(note_manager, consts::constants::FSHARP5, 44100.0);
        }
        Keycode::Key5 => {
            println!(
                "Touche 5 pressée - fréquence: {}",
                consts::constants::GSHARP5
            );
            note_manager::add_note(note_manager, consts::constants::GSHARP5, 44100.0);
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
        _ => {}
    }
}

pub fn matching_key_released(
    key: Keycode,
    _current_synth_type: &Arc<Mutex<synths::manager::SynthType>>,
    note_manager: &note_manager::ActiveNoteManager,
) {
    match key {
        Keycode::Q => {
            println!("Touche Q relâchée - fréquence: {}", consts::constants::A4);
            note_manager::release_note(note_manager, consts::constants::A4);
        }
        Keycode::B => {
            println!("Touche B relâchée - fréquence: {}", consts::constants::B4);
            note_manager::release_note(note_manager, consts::constants::B4);
        }
        Keycode::C => {
            println!("Touche C relâchée - fréquence: {}", consts::constants::C5);
            note_manager::release_note(note_manager, consts::constants::C5);
        }
        Keycode::D => {
            println!("Touche D relâchée - fréquence: {}", consts::constants::D5);
            note_manager::release_note(note_manager, consts::constants::D5);
        }
        Keycode::E => {
            println!("Touche E relâchée - fréquence: {}", consts::constants::E5);
            note_manager::release_note(note_manager, consts::constants::E5);
        }
        Keycode::F => {
            println!("Touche F relâchée - fréquence: {}", consts::constants::F5);
            note_manager::release_note(note_manager, consts::constants::F5);
        }
        Keycode::G => {
            println!("Touche G relâchée - fréquence: {}", consts::constants::G5);
            note_manager::release_note(note_manager, consts::constants::G5);
        }
        Keycode::Key1 => {
            println!(
                "Touche 1 relâchée - fréquence: {}",
                consts::constants::ASHARP4
            );
            note_manager::release_note(note_manager, consts::constants::ASHARP4);
        }
        Keycode::Key2 => {
            println!(
                "Touche 2 relâchée - fréquence: {}",
                consts::constants::CSHARP5
            );
            note_manager::release_note(note_manager, consts::constants::CSHARP5);
        }
        Keycode::Key3 => {
            println!(
                "Touche 3 relâchée - fréquence: {}",
                consts::constants::DSHARP5
            );
            note_manager::release_note(note_manager, consts::constants::DSHARP5);
        }
        Keycode::Key4 => {
            println!(
                "Touche 4 relâchée - fréquence: {}",
                consts::constants::FSHARP5
            );
            note_manager::release_note(note_manager, consts::constants::FSHARP5);
        }
        Keycode::Key5 => {
            println!(
                "Touche 5 relâchée - fréquence: {}",
                consts::constants::GSHARP5
            );
            note_manager::release_note(note_manager, consts::constants::GSHARP5);
        }
        _ => {}
    }
}
