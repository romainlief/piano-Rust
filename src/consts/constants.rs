use std::sync::atomic::AtomicUsize;

/// Consts
pub const VECTEUR_NOTES: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9]; // The 9 octaves
pub const SAMPLE_RATE: f64 = 44100.0;

/// Variables
pub static CURRENT_OCTAVE_INDEX: AtomicUsize = AtomicUsize::new(4); // Current index in VECTEUR_NOTES (thread-safe)

/// ADSR
pub static ADSR_ATTACK: f64 = 1.0;
pub static ADSR_DECAY: f64 = 0.1;
pub static ADSR_SUSTAIN: f64 = 0.9;
pub static ADSR_RELEASE: f64 = 0.3;

/// SINE
pub static SINE_CURRENT_GAIN: f64 = 4.1; // Current gain value
pub static SINE_CURRENT_LFO_RATE: f64 = 5.0; // Current LFO freq

pub static SINE_CURRENT_THRESHOLD: f64 = -10.0; // en dB : seuil élevé - compression seulement sur les pics
pub static SINE_CURRENT_RATIO: f64 = 4.0; // ratio très doux : 1.2:1 - compression très légère
pub static SINE_CURRENT_ATTACK: f64 = 1.01; // attaque encore plus lente : 200 ms
pub static SINE_CURRENT_RELEASE: f64 = 0.2; // release très lente : 1 seconde
pub static SINE_CURRENT_MAKEUP_GAIN: f64 = 0.0; // gain neutre

/// FM
pub static FM_CURRENT_GAIN: f64 = 0.6; // Current gain value
pub static FM_CURRENT_LFO_RATE: f64 = 5.0; // Current LFO freq

pub static FM_CURRENT_THRESHOLD: f64 = -24.0; // en dB : seuil de déclenchement de la compression, -24 dB est assez standard
pub static FM_CURRENT_RATIO: f64 = 4.0; // ratio de compression : 4:1 est un bon compromis pour un compresseur généraliste
pub static FM_CURRENT_ATTACK: f64 = 0.01; // attaque en secondes : 10 ms, assez rapide pour attraper les transitoires
pub static FM_CURRENT_RELEASE: f64 = 0.1; // release en secondes : 100 ms, permet un relâchement naturel
pub static FM_CURRENT_MAKEUP_GAIN: f64 = 0.0; // gain de compensation en dB, à ajuster selon besoin (0 dB = pas de gain ajouté)
pub static FM_CURRENT_KNEE: f64 = 6.0; // soft knee en dB, une transition douce de 6 dB est classique
pub static FM_CURRENT_LOOKAHEAD_TIME: f64 = 0.005; // 5 ms de look-ahead pour anticiper les pics

/// HAMMOND
pub static HAMMOND_CURRENT_GAIN: f64 = 0.6; // Current gain value
pub static HAMMOND_CURRENT_LFO_RATE: f64 = 5.0; // Current LFO freq

pub static HAMMOND_CURRENT_THRESHOLD: f64 = -24.0; // en dB : seuil de déclenchement de la compression, -24 dB est assez standard
pub static HAMMOND_CURRENT_RATIO: f64 = 4.0; // ratio de compression : 4:1 est un bon compromis pour un compresseur généraliste
pub static HAMMOND_CURRENT_ATTACK: f64 = 0.01; // attaque en secondes : 10 ms, assez rapide pour attraper les transitoires
pub static HAMMOND_CURRENT_RELEASE: f64 = 0.1; // release en secondes : 100 ms, permet un relâchement naturel
pub static HAMMOND_CURRENT_MAKEUP_GAIN: f64 = 0.0; // gain de compensation en dB, à ajuster selon besoin (0 dB = pas de gain ajouté)
pub static HAMMOND_CURRENT_KNEE: f64 = 6.0; // soft knee en dB, une transition douce de 6 dB est classique
pub static HAMMOND_CURRENT_LOOKAHEAD_TIME: f64 = 0.005; // 5 ms de look-ahead pour anticiper les pics

/// Sawtooth
pub static SAWTOOTH_CURRENT_GAIN: f64 = 0.6; // Current gain value
pub static SAWTOOTH_CURRENT_LFO_RATE: f64 = 5.0; // Current LFO freq

pub static SAWTOOTH_CURRENT_THRESHOLD: f64 = -24.0; // en dB : seuil de déclenchement de la compression, -24 dB est assez standard
pub static SAWTOOTH_CURRENT_RATIO: f64 = 4.0; // ratio de compression : 4:1 est un bon compromis pour un compresseur généraliste
pub static SAWTOOTH_CURRENT_ATTACK: f64 = 0.01; // attaque en secondes : 10 ms, assez rapide pour attraper les transitoires
pub static SAWTOOTH_CURRENT_RELEASE: f64 = 0.1; // release en secondes : 100 ms, permet un relâchement naturel
pub static SAWTOOTH_CURRENT_MAKEUP_GAIN: f64 = 0.0; // gain de compensation en dB, à ajuster selon besoin (0 dB = pas de gain ajouté)
pub static SAWTOOTH_CURRENT_KNEE: f64 = 6.0; // soft knee en dB, une transition douce de 6 dB est classique
pub static SAWTOOTH_CURRENT_LOOKAHEAD_TIME: f64 = 0.005; // 5 ms de look-ahead pour anticiper les pics

/// Square
pub static SQUARE_CURRENT_GAIN: f64 = 0.6; // Current gain value
pub static SQUARE_CURRENT_LFO_RATE: f64 = 5.0; // Current LFO freq

pub static SQUARE_CURRENT_THRESHOLD: f64 = -24.0; // en dB : seuil de déclenchement de la compression, -24 dB est assez standard
pub static SQUARE_CURRENT_RATIO: f64 = 4.0; // ratio de compression : 4:1 est un bon compromis pour un compresseur généraliste
pub static SQUARE_CURRENT_ATTACK: f64 = 0.01; // attaque en secondes : 10 ms, assez rapide pour attraper les transitoires
pub static SQUARE_CURRENT_RELEASE: f64 = 0.1; // release en secondes : 100 ms, permet un relâchement naturel
pub static SQUARE_CURRENT_MAKEUP_GAIN: f64 = 0.0; // gain de compensation en dB, à ajuster selon besoin (0 dB = pas de gain ajouté)
pub static SQUARE_CURRENT_KNEE: f64 = 6.0; // soft knee en dB, une transition douce de 6 dB est classique
pub static SQUARE_CURRENT_LOOKAHEAD_TIME: f64 = 0.005; // 5 ms de look-ahead pour anticiper les pics
