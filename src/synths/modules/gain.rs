use crate::synths::traits::Module;

#[derive(Clone, Copy)]
pub struct Gain {
    pub gain: f64,
}

impl Gain {
    pub fn new(gain: f64) -> Self {
        Self { gain }
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
}
