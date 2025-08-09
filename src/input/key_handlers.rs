use crate::audio::frequency_manager;
use crate::consts;
use crate::synths;
use crate::synths::modules::adsr;
use device_query::Keycode;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub fn matching_key_pressed(
    key: Keycode,
    active_frequencies: &Arc<Mutex<HashSet<u64>>>,
    current_synth_type: &Arc<Mutex<synths::manager::SynthType>>,
) {
    match key {
        Keycode::Q => {
            println!("Touche Q pressée - fréquence: {}", consts::constants::A4);
            frequency_manager::add_frequency_realtime(&active_frequencies, consts::constants::A4);
        }
        Keycode::B => {
            println!("Touche B pressée - fréquence: {}", consts::constants::B4);
            frequency_manager::add_frequency_realtime(&active_frequencies, consts::constants::B4);
        }
        Keycode::C => {
            println!("Touche C pressée - fréquence: {}", consts::constants::C5);
            frequency_manager::add_frequency_realtime(&active_frequencies, consts::constants::C5);
        }
        Keycode::D => {
            println!("Touche D pressée - fréquence: {}", consts::constants::D5);
            frequency_manager::add_frequency_realtime(&active_frequencies, consts::constants::D5);
        }
        Keycode::E => {
            println!("Touche E pressée - fréquence: {}", consts::constants::E5);
            frequency_manager::add_frequency_realtime(&active_frequencies, consts::constants::E5);
        }
        Keycode::F => {
            println!("Touche F pressée - fréquence: {}", consts::constants::F5);
            frequency_manager::add_frequency_realtime(&active_frequencies, consts::constants::F5);
        }
        Keycode::G => {
            println!("Touche G pressée - fréquence: {}", consts::constants::G5);
            frequency_manager::add_frequency_realtime(&active_frequencies, consts::constants::G5);
        }
        Keycode::Key1 => {
            println!(
                "Touche 1 pressée - fréquence: {}",
                consts::constants::ASharp4
            );
            frequency_manager::add_frequency_realtime(
                &active_frequencies,
                consts::constants::ASharp4,
            );
        }
        Keycode::Key2 => {
            println!(
                "Touche 2 pressée - fréquence: {}",
                consts::constants::CSharp5
            );
            frequency_manager::add_frequency_realtime(
                &active_frequencies,
                consts::constants::CSharp5,
            );
        }
        Keycode::Key3 => {
            println!(
                "Touche 3 pressée - fréquence: {}",
                consts::constants::DSharp5
            );
            frequency_manager::add_frequency_realtime(
                &active_frequencies,
                consts::constants::DSharp5,
            );
        }
        Keycode::Key4 => {
            println!(
                "Touche 4 pressée - fréquence: {}",
                consts::constants::FSharp5
            );
            frequency_manager::add_frequency_realtime(
                &active_frequencies,
                consts::constants::FSharp5,
            );
        }
        Keycode::Key5 => {
            println!(
                "Touche 5 pressée - fréquence: {}",
                consts::constants::GSharp5
            );
            frequency_manager::add_frequency_realtime(
                &active_frequencies,
                consts::constants::GSharp5,
            );
        }
        Keycode::Space => {
            println!("Espace pressé - arrêt de toutes les notes");
            frequency_manager::stop_all_frequencies_realtime(&active_frequencies);
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
        Keycode::N => {
            *current_synth_type.lock().unwrap() = synths::manager::SynthType::lfo_sine();
            println!("Synthétiseur changé: Sine with LFO");
        }
        Keycode::K => {
            *current_synth_type.lock().unwrap() = synths::manager::SynthType::n_fm();
            println!("Synthétiseur changé: FM");
        }
        Keycode::H => {
            *current_synth_type.lock().unwrap() = synths::manager::SynthType::n_hammond();
            println!("Synthétiseur changé: Hammond");
        }
        Keycode::Escape => {
            println!("\rAu revoir !");
            std::process::exit(0);
        }
        _ => {}
    }
}

pub fn matching_key_released(key: Keycode, active_frequencies: &Arc<Mutex<HashSet<u64>>>) {
    match key {
        Keycode::Q => {
            println!("Touche Q relâchée - fréquence: {}", consts::constants::A4);
            frequency_manager::remove_frequency_realtime(
                &active_frequencies,
                consts::constants::A4,
            );
        }
        Keycode::B => {
            println!("Touche B relâchée - fréquence: {}", consts::constants::B4);
            frequency_manager::remove_frequency_realtime(
                &active_frequencies,
                consts::constants::B4,
            );
        }
        Keycode::C => {
            println!("Touche C relâchée - fréquence: {}", consts::constants::C5);
            frequency_manager::remove_frequency_realtime(
                &active_frequencies,
                consts::constants::C5,
            );
        }
        Keycode::D => {
            println!("Touche D relâchée - fréquence: {}", consts::constants::D5);
            frequency_manager::remove_frequency_realtime(
                &active_frequencies,
                consts::constants::D5,
            );
        }
        Keycode::E => {
            println!("Touche E relâchée - fréquence: {}", consts::constants::E5);
            frequency_manager::remove_frequency_realtime(
                &active_frequencies,
                consts::constants::E5,
            );
        }
        Keycode::F => {
            println!("Touche F relâchée - fréquence: {}", consts::constants::F5);
            frequency_manager::remove_frequency_realtime(
                &active_frequencies,
                consts::constants::F5,
            );
        }
        Keycode::G => {
            println!("Touche G relâchée - fréquence: {}", consts::constants::G5);
            frequency_manager::remove_frequency_realtime(
                &active_frequencies,
                consts::constants::G5,
            );
        }
        Keycode::Key1 => {
            println!(
                "Touche 1 relâchée - fréquence: {}",
                consts::constants::ASharp4
            );
            frequency_manager::remove_frequency_realtime(
                &active_frequencies,
                consts::constants::ASharp4,
            );
        }
        Keycode::Key2 => {
            println!(
                "Touche 2 relâchée - fréquence: {}",
                consts::constants::CSharp5
            );
            frequency_manager::remove_frequency_realtime(
                &active_frequencies,
                consts::constants::CSharp5,
            );
        }
        Keycode::Key3 => {
            println!(
                "Touche 3 relâchée - fréquence: {}",
                consts::constants::DSharp5
            );
            frequency_manager::remove_frequency_realtime(
                &active_frequencies,
                consts::constants::DSharp5,
            );
        }
        Keycode::Key4 => {
            println!(
                "Touche 4 relâchée - fréquence: {}",
                consts::constants::FSharp5
            );
            frequency_manager::remove_frequency_realtime(
                &active_frequencies,
                consts::constants::FSharp5,
            );
        }
        Keycode::Key5 => {
            println!(
                "Touche 5 relâchée - fréquence: {}",
                consts::constants::GSharp5
            );
            frequency_manager::remove_frequency_realtime(
                &active_frequencies,
                consts::constants::GSharp5,
            );
        }
        _ => {}
    }
}
