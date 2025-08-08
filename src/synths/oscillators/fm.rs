use crate::synths::traits::Oscillator;

#[derive(Clone, Copy)]
pub struct FmOscillator {
    pub mod_index: f64, // Intensité de modulation
    pub mod_freq: f64,  // Fréquence du modulateur
}

impl FmOscillator {
    pub fn new(mod_index: f64, mod_freq: f64) -> Self {
        Self { mod_index, mod_freq }
    }
}

impl Oscillator for FmOscillator {
    fn sample(&self, phase: f64) -> f64 {
        let mod_signal = (phase * self.mod_freq).sin() * self.mod_index;
        (phase + mod_signal).sin()
    }

    fn name(&self) -> &'static str {
        "FM"
    }
}
