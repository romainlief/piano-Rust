use std::sync::atomic::AtomicUsize;

/// Consts
pub const VECTEUR_NOTES: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
pub const SAMPLE_RATE: f64 = 44100.0;

/// Variables
pub static CURRENT_OCTAVE_INDEX: AtomicUsize = AtomicUsize::new(4); // Index actuel dans VECTEUR_NOTES (thread-safe)
