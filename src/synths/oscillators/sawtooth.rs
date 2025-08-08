use crate::synths::traits::Oscillator;

#[derive(Clone, Copy)]
pub struct SawtoothOscillator;

impl Oscillator for SawtoothOscillator {
    fn sample(&self, phase: f64) -> f64 {
        // phase en radians => converti en [-1, 1]
        (phase / std::f64::consts::PI) % 2.0 - 1.0
    }

    fn name(&self) -> &'static str {
        "Sawtooth"
    }
}
