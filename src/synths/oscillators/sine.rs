use crate::synths::traits::Oscillator;

#[derive(Clone, Copy)]
pub struct SineOscillator;

impl Oscillator for SineOscillator {
    fn sample(&self, phase: f64) -> f64 {
        phase.sin()
    }
    fn name(&self) -> &'static str {
        "Sine"
    }
}
