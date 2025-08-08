use crate::synths::traits::Oscillator;

#[derive(Clone, Copy)]
pub struct HammondOscillator;

impl Oscillator for HammondOscillator {
    fn sample(&self, phase: f64) -> f64 {
        // Additive synthesis harmonics
        (phase.sin() * 0.5)
        + ((2.0 * phase).sin() * 0.25)
        + ((3.0 * phase).sin() * 0.125)
        + ((4.0 * phase).sin() * 0.0625)
    }

    fn name(&self) -> &'static str {
        "Hammond"
    }
}
