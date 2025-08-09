use crate::synths::modules::adsr::ADSR;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Structure pour gérer une note active avec son ADSR individuel
#[derive(Clone)]
pub struct ActiveNote {
    pub frequency: f64,
    pub adsr: ADSR,
    pub is_released: bool, // true quand la touche est relâchée mais l'ADSR est en release
}

impl ActiveNote {
    pub fn new(frequency: f64, sample_rate: f64) -> Self {
        let mut adsr = ADSR::new(sample_rate);
        // Configuration ADSR plus audible et plus dynamique
        adsr.set_attack(1.0);   // 5ms attack très rapide
        adsr.set_decay(0.1);      // 100ms decay
        adsr.set_sustain(0.9);    // 90% sustain level (plus fort)
        adsr.set_release(0.3);    // 300ms release plus long
        
        adsr.note_on(); // Démarrer immédiatement l'ADSR
        
        Self {
            frequency,
            adsr,
            is_released: false,
        }
    }
    
    pub fn note_off(&mut self) {
        self.adsr.note_off();
        self.is_released = true;
    }
    
    pub fn is_finished(&self) -> bool {
        use crate::synths::modules::adsr::EnvelopeStage;
        matches!(self.adsr.get_stage(), EnvelopeStage::Idle)
    }
    
    pub fn get_amplitude(&mut self) -> f64 {
        self.adsr.get_amplitude()
    }
}

/// Gestionnaire des notes actives avec ADSR individuels
pub type ActiveNoteManager = Arc<Mutex<HashMap<u64, ActiveNote>>>;

pub fn create_note_manager() -> ActiveNoteManager {
    Arc::new(Mutex::new(HashMap::new()))
}

/// Ajoute une nouvelle note avec son ADSR
pub fn add_note(manager: &ActiveNoteManager, frequency: f64, sample_rate: f64) {
    let frequency_key = (frequency * 1000.0) as u64; // Convertir en clé entière
    let mut notes = manager.lock().unwrap();
    
    if let Some(existing_note) = notes.get_mut(&frequency_key) {
        // Si la note existe déjà, vérifier si elle est en cours de release
        if existing_note.is_released {
            // Redémarrer l'ADSR depuis le début
            existing_note.adsr.note_on();
            existing_note.is_released = false;
            println!("Note redémarrée: {:.2} Hz", frequency);
        }
        // Si la note n'est pas relâchée, ne rien faire (évite les note_on multiples)
    } else {
        // Créer une nouvelle note
        let note = ActiveNote::new(frequency, sample_rate);
        notes.insert(frequency_key, note);
        println!("Note ajoutée: {:.2} Hz", frequency);
    }
}

/// Marque une note pour release (note_off)
pub fn  release_note(manager: &ActiveNoteManager, frequency: f64) {
    let frequency_key = (frequency * 1000.0) as u64;
    let mut notes = manager.lock().unwrap();
    
    if let Some(note) = notes.get_mut(&frequency_key) {
        if !note.is_released {
            note.note_off();
            println!("Note relâchée: {:.2} Hz", frequency);
        }
    }
}

/// Nettoie les notes finies (ADSR en Idle)
pub fn cleanup_finished_notes(manager: &ActiveNoteManager) {
    let mut notes = manager.lock().unwrap();
    notes.retain(|_key, note| !note.is_finished());
}

/// Arrête toutes les notes
pub fn stop_all_notes(manager: &ActiveNoteManager) {
    let mut notes = manager.lock().unwrap();
    for note in notes.values_mut() {
        if !note.is_released {
            note.note_off();
        }
    }
}
