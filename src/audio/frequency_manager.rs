use std::collections::HashSet;
use std::sync::{Arc, Mutex};

/// Ajoute une fréquence en temps réel
pub fn add_frequency_realtime(frequencies: &Arc<Mutex<HashSet<u64>>>, freq: f64) {
    let mut freqs = frequencies.lock().unwrap();
    let freq_key = freq.to_bits();
    if freqs.insert(freq_key) {
        println!("\rNote ON: {:.2} Hz. Notes actives: {}", freq, freqs.len());
    }
}

/// Retire une fréquence en temps réel
pub fn remove_frequency_realtime(frequencies: &Arc<Mutex<HashSet<u64>>>, freq: f64) {
    let mut freqs = frequencies.lock().unwrap();
    let freq_key = freq.to_bits();
    if freqs.remove(&freq_key) {
        println!("\rNote OFF: {:.2} Hz. Notes actives: {}", freq, freqs.len());
    }
}

/// Arrête toutes les fréquences
pub fn stop_all_frequencies_realtime(frequencies: &Arc<Mutex<HashSet<u64>>>) {
    let mut freqs = frequencies.lock().unwrap();
    freqs.clear();
    println!("\rToutes les notes arrêtées.");
}
