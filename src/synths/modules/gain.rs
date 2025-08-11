use crate::synths::traits::Module;

#[derive(Clone, Copy)]
/// Gain struct:
/// This module implements a simple gain control for audio signals.
pub struct Gain {
    gain: f64,
}

impl Gain {
    pub fn new(gain: f64) -> Self {
        Self { gain }
    }

    pub fn set_gain(&mut self, gain: f64) {
        self.gain = gain;
    }

    pub fn get_gain(&self) -> f64 {
        self.gain
    }
}

impl Module for Gain {
    fn process(&mut self, input: f64, _time: f64) -> f64 {
        input * self.gain
    }

    fn name(&self) -> &'static str {
        "Gain"
    }

    fn clone_box(&self) -> Box<dyn Module> {
        Box::new(*self)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
