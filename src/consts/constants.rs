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
pub static SINE_ACTIVATION_GAIN: bool = false; // true to activate the gain effect
pub static SINE_ACTIVATION_LFO: bool = false; // true to activate the LFO effect
pub static SINE_ACTIVATION_COMPRESSOR: bool = false; // true to activate the compressor effect
pub static SINE_ACTIVATION_NOISE: bool = false; // true to activate the noise effect
pub static SINE_ACTIVATION_FILTER: bool = false; // true to activate the filter effect
pub static SINE_ACTIVATION_REVERB: bool = true; // true to activate the reverb effect
////////////////////////////////////////////////////////////////////////////////////////////////////////////
// GAIN
pub static SINE_CURRENT_GAIN: f64 = 4.1; // Current gain value
// LFO
pub static SINE_CURRENT_LFO_FREQ: f64 = 725.0; // Current LFO freq
pub static SINE_CURRENT_LFO_WAVEFORM: LfoWaveform = LfoWaveform::Sine; // Current LFO waveform
// COMPRESSOR
pub static SINE_CURRENT_THRESHOLD: f64 = -10.0; // en dB : seuil élevé - compression seulement sur les pics
pub static SINE_CURRENT_RATIO: f64 = 4.0; // ratio très doux : 1.2:1 - compression très légère
pub static SINE_CURRENT_ATTACK: f64 = 1.01; // attaque encore plus lente : 200 ms
pub static SINE_CURRENT_RELEASE: f64 = 0.2; // release très lente : 1 seconde
pub static SINE_CURRENT_MAKEUP_GAIN: f64 = 0.0; // gain neutre
// NOISE
pub static SINE_CURRENT_NOISE: f64 = 0.2; // niveau de bruit à ajouter
// LOW PASS
pub static SINE_CURRENT_FILTER_CUTOFF: f64 = 100.0; // fréquence de coupure en Hz
pub static SINE_CURRENT_FILTER_RESONANCE: f64 = 1.2; // résonance (Q)
// REVERB
pub static SINE_CURRENT_DRY_WET: f64 = 0.7; // niveau de réverbération à ajouter
pub static SINE_CURRENT_REVERB_TYPE: ReverbType = ReverbType::Plate; // Type de réverbération par défaut
pub static SINE_CURRENT_REVERB_EARLY_GAIN: f64 = 0.9; // Gain des premières réflexions
pub static SINE_CURRENT_REVERB_TAIL_GAIN: f64 = 0.9; // Gain de la queue
pub static SINE_CURRENT_REVERB_PRE_DELAY_MS: f64 = 100.0; // Pré-délai en millisecondes
////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// FM
///
/// ACTIVATION EFFECT
pub static FM_ACTIVATION_GAIN: bool = true; // true to activate the gain effect
pub static FM_ACTIVATION_LFO: bool = true; // true to activate the LFO effect
pub static FM_ACTIVATION_COMPRESSOR: bool = true; // true to activate the compressor effect
pub static FM_ACTIVATION_NOISE: bool = true; // true to activate the noise effect
pub static FM_ACTIVATION_FILTER: bool = true; // true to activate the filter effect
pub static FM_ACTIVATION_REVERB: bool = true; // true to activate the reverb effect
////////////////////////////////////////////////////////////////////////////////////////////////////////////
// GAIN
pub static FM_CURRENT_GAIN: f64 = 0.6; // Current gain value
// LFO
pub static FM_CURRENT_LFO_FREQ: f64 = 5.0; // Current LFO freq
pub static FM_CURRENT_LFO_WAVEFORM: LfoWaveform = LfoWaveform::SawDown; // Current LFO waveform
// COMPRESSOR
pub static FM_CURRENT_THRESHOLD: f64 = -24.0; // en dB : seuil de déclenchement de la compression, -24 dB est assez standard
pub static FM_CURRENT_RATIO: f64 = 4.0; // ratio de compression : 4:1 est un bon compromis pour un compresseur généraliste
pub static FM_CURRENT_ATTACK: f64 = 0.01; // attaque en secondes : 10 ms, assez rapide pour attraper les transitoires
pub static FM_CURRENT_RELEASE: f64 = 0.1; // release en secondes : 100 ms, permet un relâchement naturel
pub static FM_CURRENT_MAKEUP_GAIN: f64 = 0.0; // gain de compensation en dB, à ajuster selon besoin (0 dB = pas de gain ajouté)
pub static FM_CURRENT_KNEE: f64 = 6.0; // soft knee en dB, une transition douce de 6 dB est classique
pub static FM_CURRENT_LOOKAHEAD_TIME: f64 = 0.005; // 5 ms de look-ahead pour anticiper les pics
// NOISE
pub static FM_CURRENT_NOISE: f64 = 0.2; // niveau de bruit à ajouter
// LOW PASS
pub static FM_CURRENT_FILTER_CUTOFF: f64 = 100.0; // fréquence de coupure en Hz
pub static FM_CURRENT_FILTER_RESONANCE: f64 = 1.2; // résonance (Q)
// REVERB
pub static FM_CURRENT_DRY_WET: f64 = 0.2; // niveau de réverbération à ajouter
pub static FM_CURRENT_REVERB_TYPE: ReverbType = ReverbType::Spring; // Type de réverbération par défaut
pub static FM_CURRENT_REVERB_EARLY_GAIN: f64 = 0.9; // Gain des premières réflexions
pub static FM_CURRENT_REVERB_TAIL_GAIN: f64 = 0.9; // Gain de la queue
pub static FM_CURRENT_REVERB_PRE_DELAY_MS: f64 = 10.0; // Pré-délai en millisecondes
////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// HAMMOND
///
/// ACTIVATION EFFECT
pub static HAMMOND_ACTIVATION_GAIN: bool = true; // true to activate the gain effect
pub static HAMMOND_ACTIVATION_LFO: bool = true; // true to activate the LFO effect
pub static HAMMOND_ACTIVATION_COMPRESSOR: bool = true; // true to activate the compressor effect
pub static HAMMOND_ACTIVATION_NOISE: bool = true; // true to activate the noise effect
pub static HAMMOND_ACTIVATION_FILTER: bool = true; // true to activate the filter effect
pub static HAMMOND_ACTIVATION_REVERB: bool = true; // true to activate the reverb effect
////////////////////////////////////////////////////////////////////////////////////////////////////////////
// GAIN
pub static HAMMOND_CURRENT_GAIN: f64 = 0.6; // Current gain value
// LFO
pub static HAMMOND_CURRENT_LFO_FREQ: f64 = 5.0; // Current LFO freq
pub static HAMMOND_CURRENT_LFO_WAVEFORM: LfoWaveform = LfoWaveform::Sine; // Current LFO waveform
// COMPRESSOR
pub static HAMMOND_CURRENT_THRESHOLD: f64 = -24.0; // en dB : seuil de déclenchement de la compression, -24 dB est assez standard
pub static HAMMOND_CURRENT_RATIO: f64 = 4.0; // ratio de compression : 4:1 est un bon compromis pour un compresseur généraliste
pub static HAMMOND_CURRENT_ATTACK: f64 = 0.01; // attaque en secondes : 10 ms, assez rapide pour attraper les transitoires
pub static HAMMOND_CURRENT_RELEASE: f64 = 0.1; // release en secondes : 100 ms, permet un relâchement naturel
pub static HAMMOND_CURRENT_MAKEUP_GAIN: f64 = 0.0; // gain de compensation en dB, à ajuster selon besoin (0 dB = pas de gain ajouté)
pub static HAMMOND_CURRENT_KNEE: f64 = 6.0; // soft knee en dB, une transition douce de 6 dB est classique
pub static HAMMOND_CURRENT_LOOKAHEAD_TIME: f64 = 0.005; // 5 ms de look-ahead pour anticiper les pics
// NOISE
pub static HAMMOND_CURRENT_NOISE: f64 = 0.2; // niveau de bruit à ajouter
// LOW PASS
pub static HAMMOND_CURRENT_FILTER_CUTOFF: f64 = 100.0; // fréquence de coupure en Hz
pub static HAMMOND_CURRENT_FILTER_RESONANCE: f64 = 1.2; // résonance (Q)
// REVERB
pub static HAMMOND_CURRENT_DRY_WET: f64 = 0.2; // niveau de réverbération à ajouter
pub static HAMMOND_CURRENT_REVERB_TYPE: ReverbType = ReverbType::Spring; // Type de réverbération par défaut
pub static HAMMOND_CURRENT_REVERB_EARLY_GAIN: f64 = 0.9; // Gain des premières réflexions
pub static HAMMOND_CURRENT_REVERB_TAIL_GAIN: f64 = 0.9; // Gain de la queue
pub static HAMMOND_CURRENT_REVERB_PRE_DELAY_MS: f64 = 10.0; // Pré-délai en millisecondes
////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Sawtooth
///
/// ACTIVATION EFFECT
pub static SAWTOOTH_ACTIVATION_GAIN: bool = true; // true to activate the gain effect
pub static SAWTOOTH_ACTIVATION_LFO: bool = true; // true to activate the LFO effect
pub static SAWTOOTH_ACTIVATION_COMPRESSOR: bool = true; // true to activate the compressor effect
pub static SAWTOOTH_ACTIVATION_NOISE: bool = true; // true to activate the noise effect
pub static SAWTOOTH_ACTIVATION_FILTER: bool = true; // true to activate the filter effect
pub static SAWTOOTH_ACTIVATION_REVERB: bool = true; // true to activate the reverb effect
////////////////////////////////////////////////////////////////////////////////////////////////////////////
// GAIN
pub static SAWTOOTH_CURRENT_GAIN: f64 = 0.6; // Current gain value
// LFO
pub static SAWTOOTH_CURRENT_LFO_FREQ: f64 = 5.0; // Current LFO freq
pub static SAWTOOTH_CURRENT_LFO_WAVEFORM: LfoWaveform = LfoWaveform::SawUp; // Current LFO waveform
// COMPRESSOR
pub static SAWTOOTH_CURRENT_THRESHOLD: f64 = -24.0; // en dB : seuil de déclenchement de la compression, -24 dB est assez standard
pub static SAWTOOTH_CURRENT_RATIO: f64 = 4.0; // ratio de compression : 4:1 est un bon compromis pour un compresseur généraliste
pub static SAWTOOTH_CURRENT_ATTACK: f64 = 0.01; // attaque en secondes : 10 ms, assez rapide pour attraper les transitoires
pub static SAWTOOTH_CURRENT_RELEASE: f64 = 0.1; // release en secondes : 100 ms, permet un relâchement naturel
pub static SAWTOOTH_CURRENT_MAKEUP_GAIN: f64 = 0.0; // gain de compensation en dB, à ajuster selon besoin (0 dB = pas de gain ajouté)
pub static SAWTOOTH_CURRENT_KNEE: f64 = 6.0; // soft knee en dB, une transition douce de 6 dB est classique
pub static SAWTOOTH_CURRENT_LOOKAHEAD_TIME: f64 = 0.005; // 5 ms de look-ahead pour anticiper les pics
// NOISE
pub static SAWTOOTH_CURRENT_NOISE: f64 = 0.2; // niveau de bruit à ajouter
// LOW PASS
pub static SAWTOOTH_CURRENT_FILTER_CUTOFF: f64 = 100.0; // fréquence de coupure en Hz
pub static SAWTOOTH_CURRENT_FILTER_RESONANCE: f64 = 1.2; // résonance (Q)
// REVERB
pub static SAWTOOTH_CURRENT_DRY_WET: f64 = 0.2; // niveau de réverbération à ajouter
pub static SAWTOOTH_CURRENT_REVERB_TYPE: ReverbType = ReverbType::Spring; // Type de réverbération par défaut
pub static SAWTOOTH_CURRENT_REVERB_EARLY_GAIN: f64 = 0.9; // Gain des premières réflexions
pub static SAWTOOTH_CURRENT_REVERB_TAIL_GAIN: f64 = 0.9; // Gain de la queue
pub static SAWTOOTH_CURRENT_REVERB_PRE_DELAY_MS: f64 = 10.0; // Pré-délai en millisecondes
////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Square
/// ACTIVATION EFFECT
pub static SQUARE_ACTIVATION_GAIN: bool = true; // true to activate the gain effect
pub static SQUARE_ACTIVATION_LFO: bool = true; // true to activate the LFO effect
pub static SQUARE_ACTIVATION_COMPRESSOR: bool = true; // true to activate the compressor effect
pub static SQUARE_ACTIVATION_NOISE: bool = true; // true to activate the noise effect
pub static SQUARE_ACTIVATION_FILTER: bool = true; // true to activate the filter effect
pub static SQUARE_ACTIVATION_REVERB: bool = true; // true to activate the reverb effect
////////////////////////////////////////////////////////////////////////////////////////////////////////////
// GAIN
pub static SQUARE_CURRENT_GAIN: f64 = 0.6; // Current gain value
// LFO
pub static SQUARE_CURRENT_LFO_FREQ: f64 = 5.0; // Current LFO freq
pub static SQUARE_CURRENT_LFO_WAVEFORM: LfoWaveform = LfoWaveform::Square; // Current LFO waveform
// COMPRESSOR
pub static SQUARE_CURRENT_THRESHOLD: f64 = -24.0; // en dB : seuil de déclenchement de la compression, -24 dB est assez standard
pub static SQUARE_CURRENT_RATIO: f64 = 4.0; // ratio de compression : 4:1 est un bon compromis pour un compresseur généraliste
pub static SQUARE_CURRENT_ATTACK: f64 = 0.01; // attaque en secondes : 10 ms, assez rapide pour attraper les transitoires
pub static SQUARE_CURRENT_RELEASE: f64 = 0.1; // release en secondes : 100 ms, permet un relâchement naturel
pub static SQUARE_CURRENT_MAKEUP_GAIN: f64 = 0.0; // gain de compensation en dB, à ajuster selon besoin (0 dB = pas de gain ajouté)
pub static SQUARE_CURRENT_KNEE: f64 = 6.0; // soft knee en dB, une transition douce de 6 dB est classique
pub static SQUARE_CURRENT_LOOKAHEAD_TIME: f64 = 0.005; // 5 ms de look-ahead pour anticiper les pics
// NOISE
pub static SQUARE_CURRENT_NOISE: f64 = 0.2; // niveau de bruit à ajouter
// LOW PASS
pub static SQUARE_CURRENT_FILTER_CUTOFF: f64 = 100.0; // fréquence de coupure en Hz
pub static SQUARE_CURRENT_FILTER_RESONANCE: f64 = 1.2; // résonance (Q)
// REVERB
pub static SQUARE_CURRENT_DRY_WET: f64 = 0.2; // niveau de réverbération à ajouter
pub static SQUARE_CURRENT_REVERB_TYPE: ReverbType = ReverbType::Spring; // Type de réverbération par défaut
pub static SQUARE_CURRENT_REVERB_EARLY_GAIN: f64 = 0.9; // Gain des premières réflexions
pub static SQUARE_CURRENT_REVERB_TAIL_GAIN: f64 = 0.9; // Gain de la queue
pub static SQUARE_CURRENT_REVERB_PRE_DELAY_MS: f64 = 10.0; // Pré-délai en millisecondes
////////////////////////////////////////////////////////////////////////////////////////////////////////////
