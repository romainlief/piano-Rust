use crate::synths::traits::Module;
use rand::Rng;

/// Noise struct
/// This struct adds noise to an audio signal.
#[derive(Clone, Copy)]
pub struct Noise {
    amount: f64, // niveau de bruit à ajouter
}

impl Noise {
    pub fn new(amount: f64) -> Self {
        Self { amount }
    }

    pub fn set_amount(&mut self, amount: f64) {
        self.amount = amount;
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }
}

impl Module for Noise {
    fn process(&mut self, input: f64, _time: f64) -> f64 {
        let mut rng = rand::rng();
        let noise: f64 = rng.random_range(-1.0..1.0) * self.amount;
        let mixed = input + noise;
        mixed.clamp(-1.0, 1.0) // évite la saturation
    }

    fn name(&self) -> &'static str {
        "NoiseEffect"
    }

    fn clone_box(&self) -> Box<dyn Module> {
        Box::new(*self)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
