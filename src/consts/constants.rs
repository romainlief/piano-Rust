use std::sync::atomic::AtomicUsize;

/// Consts
pub const VECTEUR_NOTES: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9]; // The 9 octaves
pub const SAMPLE_RATE: f64 = 44100.0;

/// Variables
pub static CURRENT_OCTAVE_INDEX: AtomicUsize = AtomicUsize::new(4); // Current index in VECTEUR_NOTES (thread-safe)

pub static ADSR_ATTACK: f64 = 1.0;
pub static ADSR_DECAY: f64 = 0.1;
pub static ADSR_SUSTAIN: f64 = 0.9;
pub static ADSR_RELEASE: f64 = 0.3;

/// SINE
pub static SINE_CURRENT_GAIN: f64 = 0.6; // Current gain value
pub static SINE_CURRENT_LFO_RATE: f64 = 5.0; // Current LFO freq

/// FM
pub static FM_CURRENT_GAIN: f64 = 0.6; // Current gain value
pub static FM_CURRENT_LFO_RATE: f64 = 5.0; // Current LFO freq

/// HAMMOND
pub static HAMMOND_CURRENT_GAIN: f64 = 0.6; // Current gain value
pub static HAMMOND_CURRENT_LFO_RATE: f64 = 5.0; // Current LFO freq

/// Sawtooth
pub static SAWTOOTH_CURRENT_GAIN: f64 = 0.6; // Current gain value
pub static SAWTOOTH_CURRENT_LFO_RATE: f64 = 5.0; // Current LFO freq

/// Square
pub static SQUARE_CURRENT_GAIN: f64 = 0.6; // Current gain value
pub static SQUARE_CURRENT_LFO_RATE: f64 = 5.0; // Current LFO freq
