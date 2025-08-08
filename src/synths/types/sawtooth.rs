use crate::synths::traits::Synthesizer;

// Sawtooth wave synthesizer
#[derive(Clone, Copy, Debug)]
pub struct SawtoothSynth {
    pub amplitude: f64,
    pub smoothness: f64, // Smoothness factor
}

impl SawtoothSynth {
    pub fn new() -> Self {
        Self {
            amplitude: 1.0,
            smoothness: 1.0,
        }
    }
}

impl Synthesizer for SawtoothSynth {
    fn generate_sample(&self, phase: f64, _frequency: f64) -> f64 {
        let normalized = phase / (2.0 * std::f64::consts::PI);
        let sawtooth = 2.0 * (normalized - (normalized + 0.5).floor()) - 1.0;
        sawtooth * self.amplitude * self.smoothness
    }

    fn name(&self) -> &'static str {
        "Sawtooth"
    }
}