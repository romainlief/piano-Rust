use crate::synth::traits::Synthesizer;

// Basic sine wave synthesizer
#[derive(Clone, Copy, Debug)]
pub struct SineSynth {
    pub amplitude: f64,
}

impl SineSynth {
    pub fn new() -> Self {
        Self { amplitude: 1.0 }
    }
}

impl Synthesizer for SineSynth {
    fn generate_sample(&self, phase: f64, _frequency: f64) -> f64 {
        phase.sin() * self.amplitude
    }

    fn name(&self) -> &'static str {
        "Sine"
    }
}