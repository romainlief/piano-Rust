use crate::synths::modules::lfo::LfoWaveform;
use crate::synths::modules::reverb::ReverbType;
use std::sync::atomic::AtomicUsize;

/// Consts
pub const VECTEUR_NOTES: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9]; // The 9 octaves
pub const SAMPLE_RATE: f64 = 44100.0;

/// Variables
pub static CURRENT_OCTAVE_INDEX: AtomicUsize = AtomicUsize::new(4); // Current index in VECTEUR_NOTES (thread-safe)

/// ADSR
pub static ADSR_ATTACK: f64 = 1.0; // Attack time in seconds
pub static ADSR_DECAY: f64 = 0.1; // Decay time in seconds
pub static ADSR_SUSTAIN: f64 = 1.0; // Sustain level (0.0 to 1.0)
pub static ADSR_RELEASE: f64 = 0.7; // Release time in seconds

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// SINE
///
/// ACTIVATION EFFECT
pub static ACTIVATION_GAIN: bool = false; // true to activate the gain effect
pub static ACTIVATION_LFO: bool = false; // true to activate the LFO effect
pub static ACTIVATION_COMPRESSOR: bool = false; // true to activate the compressor effect
pub static ACTIVATION_NOISE: bool = false; // true to activate the noise effect
pub static ACTIVATION_FILTER: bool = false; // true to activate the filter effect
pub static ACTIVATION_REVERB: bool = false; // true to activate the reverb effect
////////////////////////////////////////////////////////////////////////////////////////////////////////////
// GAIN
pub static CURRENT_GAIN: f64 = 4.1; // Current gain value
// LFO
pub static CURRENT_LFO_FREQ: f64 = 725.0; // Current LFO freq
pub static CURRENT_LFO_WAVEFORM: LfoWaveform = LfoWaveform::Sine; // Current LFO waveform
// COMPRESSOR
pub static CURRENT_THRESHOLD: f64 = -10.0; // en dB : seuil élevé - compression seulement sur les pics
pub static CURRENT_RATIO: f64 = 4.0; // ratio très doux : 1.2:1 - compression très légère
pub static CURRENT_ATTACK: f64 = 1.01; // attaque encore plus lente : 200 ms
pub static CURRENT_RELEASE: f64 = 0.2; // release très lente : 1 seconde
pub static CURRENT_MAKEUP_GAIN: f64 = 0.0; // gain neutre
// NOISE
pub static CURRENT_NOISE: f64 = 0.2; // niveau de bruit à ajouter
// LOW PASS
pub static CURRENT_FILTER_CUTOFF: f64 = 8000.0; // fréquence de coupure en Hz (était 100Hz - trop bas!)
pub static CURRENT_FILTER_RESONANCE: f64 = 1.2; // résonance (Q)
// REVERB
pub static CURRENT_DRY_WET: f64 = 0.7; // niveau de réverbération à ajouter
pub static CURRENT_REVERB_TYPE: ReverbType = ReverbType::Plate; // Type de réverbération par défaut
pub static CURRENT_REVERB_EARLY_GAIN: f64 = 0.9; // Gain des premières réflexions
pub static CURRENT_REVERB_TAIL_GAIN: f64 = 0.9; // Gain de la queue
pub static CURRENT_REVERB_PRE_DELAY_MS: f64 = 100.0; // Pré-délai en millisecondes
