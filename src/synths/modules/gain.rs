use crate::synths::traits::Module;

#[derive(Clone, Copy)]
/// Gain struct:
/// This module implements a simple gain control for audio signals.
pub struct Gain {
    gain_db: f64, // Gain factor in dB
}

impl Gain {
    pub fn new(gain_db: f64) -> Self {
        Self { gain_db }
    }

    pub fn set_gain(&mut self, gain_db: f64) {
        self.gain_db = gain_db;
    }

    pub fn get_gain(&self) -> f64 {
        self.gain_db
    }

    /// Convertit les dB en facteur linéaire pour le traitement audio
    fn db_to_linear(&self) -> f64 {
        10.0_f64.powf(self.gain_db / 20.0)
    }
}

impl Module for Gain {
    fn process(&mut self, input: f64, _time: f64) -> f64 {
        input * self.db_to_linear()
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
