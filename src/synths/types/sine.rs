use crate::synths::traits::Synthesizer;

// Basic sine wave synthesizer
#[derive(Clone, Copy, Debug)]
pub struct SineSynth {
    amplitude: f64,
}

impl SineSynth {
    pub fn new() -> Self {
        Self { amplitude: 1.0 }
    }

    pub fn get_amplitude(&self) -> f64 {
        self.amplitude
    }

    pub fn set_amplitude(&mut self, value: f64) {
        self.amplitude = value;
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
