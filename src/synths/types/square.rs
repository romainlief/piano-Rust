use crate::synths::traits::Synthesizer;

// Square wave synthesizer
#[derive(Clone, Copy, Debug)]
pub struct SquareSynth {
    amplitude: f64,
    duty_cycle: f64, // Duty cycle (0.5 = 50%)
}

impl SquareSynth {
    pub fn new(duty_cycle: f64) -> Self {
        Self {
            amplitude: 1.0,
            duty_cycle: duty_cycle.clamp(0.1, 0.9),
        }
    }

    pub fn get_amplitude(&self) -> f64 {
        self.amplitude
    }

    pub fn set_amplitude(&mut self, value: f64) {
        self.amplitude = value;
    }

    pub fn get_duty_cycle(&self) -> f64 {
        self.duty_cycle
    }

    pub fn set_duty_cycle(&mut self, value: f64) {
        self.duty_cycle = value.clamp(0.1, 0.9);
    }
}

impl Synthesizer for SquareSynth {
    fn generate_sample(&self, phase: f64, _frequency: f64) -> f64 {
        let normalized = (phase / (2.0 * std::f64::consts::PI)) % 1.0;
        if normalized < self.duty_cycle {
            self.amplitude
        } else {
            -self.amplitude
        }
    }

    fn name(&self) -> &'static str {
        "Square"
    }
}
