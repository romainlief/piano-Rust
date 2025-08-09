use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use crate::synths;

/// Ajoute une fréquence en temps réel avec gestion ADSR
pub fn add_frequency_realtime(
    frequencies: &Arc<Mutex<HashSet<u64>>>, 
    freq: f64,
    synth_type: &Arc<Mutex<synths::manager::SynthType>>
) {
    let mut freqs = frequencies.lock().unwrap();
    let freq_key = freq.to_bits();
    let was_empty = freqs.is_empty();
    
    if freqs.insert(freq_key) {
        println!("\rNote ON: {:.2} Hz. Notes actives: {}", freq, freqs.len());
        
        // Déclenche note_on seulement si c'est la première note
        if was_empty {
            drop(freqs); // Libère le lock avant de prendre celui du synth
            let mut synth = synth_type.lock().unwrap();
            synth.note_on();
            println!("🎵 ADSR: Note ON déclenché");
        }
    }
}

/// Retire une fréquence en temps réel avec gestion ADSR
pub fn remove_frequency_realtime(
    frequencies: &Arc<Mutex<HashSet<u64>>>, 
    freq: f64,
    synth_type: &Arc<Mutex<synths::manager::SynthType>>
) {
    let mut freqs = frequencies.lock().unwrap();
    let freq_key = freq.to_bits();
    
    if freqs.remove(&freq_key) {
        println!("\rNote OFF: {:.2} Hz. Notes actives: {}", freq, freqs.len());
        
        // Déclenche note_off seulement si c'était la dernière note
        if freqs.is_empty() {
            drop(freqs); // Libère le lock avant de prendre celui du synth
            let mut synth = synth_type.lock().unwrap();
            synth.note_off();
            println!("🎵 ADSR: Note OFF déclenché");
        }
    }
}

/// Arrête toutes les fréquences avec gestion ADSR
pub fn stop_all_frequencies_realtime(
    frequencies: &Arc<Mutex<HashSet<u64>>>,
    synth_type: &Arc<Mutex<synths::manager::SynthType>>
) {
    let mut freqs = frequencies.lock().unwrap();
    let had_notes = !freqs.is_empty();
    freqs.clear();
    println!("\rToutes les notes arrêtées.");
    
    if had_notes {
        drop(freqs); // Libère le lock avant de prendre celui du synth
        let mut synth = synth_type.lock().unwrap();
        synth.note_off();
        println!("🎵 ADSR: Note OFF déclenché (toutes les notes)");
    }
}
