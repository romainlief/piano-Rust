use crate::synths::traits::Oscillator;

#[derive(Clone, Copy)]
pub struct SawtoothOscillator;

impl Oscillator for SawtoothOscillator {
    fn sample(&self, phase: f64) -> f64 {
        let normalized_phase = (phase % (2.0 * std::f64::consts::PI)) / (2.0 * std::f64::consts::PI);
        2.0 * normalized_phase - 1.0
    }

    fn name(&self) -> &'static str {
        "Sawtooth"
    }
}
