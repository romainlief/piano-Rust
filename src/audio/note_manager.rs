use crate::consts::constants;
use crate::synths::modules::adsr::ADSR;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Struct that represents an active note with its frequency and ADSR envelope
#[derive(Clone)]
pub struct ActiveNote {
    pub frequency: f64,
    pub adsr: ADSR,
    pub is_released: bool, // true quand la touche est relâchée mais l'ADSR est en release
}

impl ActiveNote {
    pub fn new(frequency: f64, sample_rate: f64) -> Self {
        let mut adsr = ADSR::new(sample_rate);
        adsr.set_attack(constants::ADSR_ATTACK);
        adsr.set_decay(constants::ADSR_DECAY);
        adsr.set_sustain(constants::ADSR_SUSTAIN);
        adsr.set_release(constants::ADSR_RELEASE);
        adsr.note_on();

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
    if let Ok(mut notes) = manager.lock() {
        let frequency_key = (frequency * 1000.0) as u64; // Convertir en clé entière

        if let Some(existing_note) = notes.get_mut(&frequency_key) {
            if existing_note.is_released {
                // TODO: code ne servant a rien
                existing_note.is_released = false;
                println!("Note redémarrée: {:.2} Hz", frequency);
            }
        } else {
            // Créer une nouvelle note
            let note = ActiveNote::new(frequency, sample_rate);
            notes.insert(frequency_key, note);
            println!("Note ajoutée: {:.2} Hz", frequency);
        }
    } else {
        eprintln!("Warning: Failed to lock note manager to add note");
    }
}

pub fn release_note(manager: &ActiveNoteManager, frequency: f64) {
    if let Ok(mut notes) = manager.lock() {
        let frequency_key = (frequency * 1000.0) as u64;

        if let Some(note) = notes.get_mut(&frequency_key) {
            if !note.is_released {
                note.note_off();
                println!("Note relâchée: {:.2} Hz", frequency);
            }
        }
    } else {
        eprintln!("Warning: Failed to lock note manager to release note");
    }
}

/// Nettoie les notes finies (ADSR en Idle)
pub fn cleanup_finished_notes(manager: &ActiveNoteManager) {
    if let Ok(mut notes) = manager.lock() {
        notes.retain(|_key, note| !note.is_finished());
    } else {
        eprintln!("Warning: Failed to lock note manager for cleanup");
    }
}

/// Arrête toutes les notes
pub fn stop_all_notes(manager: &ActiveNoteManager) {
    if let Ok(mut notes) = manager.lock() {
        for note in notes.values_mut() {
            if !note.is_released {
                note.note_off();
            }
        }
    } else {
        eprintln!("Warning: Failed to lock note manager to stop notes");
    }
}
