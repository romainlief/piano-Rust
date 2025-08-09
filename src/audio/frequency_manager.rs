use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub type FrequencySet = Arc<Mutex<HashSet<u64>>>;

pub fn create_frequency_set() -> FrequencySet {
    Arc::new(Mutex::new(HashSet::new()))
}

pub fn add_frequency(set: &FrequencySet, frequency: f64) {
    let frequency_key = (frequency * 1000.0) as u64;
    let mut frequencies = set.lock().unwrap();
    frequencies.insert(frequency_key);
    println!("Fréquence ajoutée: {:.2} Hz", frequency);
}

pub fn remove_frequency(set: &FrequencySet, frequency: f64) {
    let frequency_key = (frequency * 1000.0) as u64;
    let mut frequencies = set.lock().unwrap();
    frequencies.remove(&frequency_key);
    println!("Fréquence supprimée: {:.2} Hz", frequency);
}

pub fn clear_frequencies(set: &FrequencySet) {
    let mut frequencies = set.lock().unwrap();
    frequencies.clear();
    println!("Toutes les fréquences supprimées");
}

pub fn convert_to_frequencies(set: &FrequencySet) -> Vec<f64> {
    let frequencies = set.lock().unwrap();
    frequencies
        .iter()
        .map(|&freq_key| freq_key as f64 / 1000.0)
        .collect()
}
