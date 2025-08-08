use crate::synths::traits::Oscillator;

#[derive(Clone, Copy)]
pub struct SquareOscillator;

impl Oscillator for SquareOscillator {
    fn sample(&self, phase: f64) -> f64 {
        if phase.sin() >= 0.0 { 1.0 } else { -1.0 }
    }

    fn name(&self) -> &'static str {
        "Square"
    }
}
