use crate::synths::traits::Synthesizer;

// Sawtooth wave synthesizer
#[derive(Clone, Copy, Debug)]
pub struct SawtoothSynth {
    amplitude: f64,
    smoothness: f64,
}

impl SawtoothSynth {
    pub fn new() -> Self {
        Self {
            amplitude: 1.0,
            smoothness: 1.0,
        }
    }

    pub fn get_amplitude(&self) -> f64 {
        self.amplitude
    }

    pub fn set_amplitude(&mut self, value: f64) {
        self.amplitude = value;
    }

    pub fn get_smoothness(&self) -> f64 {
        self.smoothness
    }

    pub fn set_smoothness(&mut self, value: f64) {
        self.smoothness = value;
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
