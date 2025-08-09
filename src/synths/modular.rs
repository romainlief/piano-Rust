use crate::synths::traits::{Oscillator, Module};

pub struct ModularSynth<O: Oscillator> {
    pub oscillator: O,
    pub modules: Vec<Box<dyn Module>>,
    pub amplitude: f64,
}

impl<O: Oscillator> Clone for ModularSynth<O> {
    fn clone(&self) -> Self {
        Self {
            oscillator: self.oscillator,
            modules: self.modules.iter().map(|m| m.clone_box()).collect(),
            amplitude: self.amplitude,
        }
    }
}

impl<O: Oscillator> ModularSynth<O> {
    pub fn new(oscillator: O) -> Self {
        Self {
            oscillator,
            modules: Vec::new(),
            amplitude: 1.0,
        }
    }

    pub fn add_module<M: Module + 'static>(&mut self, module: M) {
        self.modules.push(Box::new(module));
    }

    pub fn generate_sample(&mut self, phase: f64, time: f64) -> f64 {
        let mut sample = self.oscillator.sample(phase) * self.amplitude;

        for module in &mut self.modules {
            sample = module.process(sample, time);
        }

        sample
    }

    /// Déclenche note_on sur tous les modules ADSR
    pub fn note_on(&mut self) {
        for module in &mut self.modules {
            // Utilise Any pour downcaster vers ADSR si possible
            if let Some(adsr) = module.as_any_mut().downcast_mut::<crate::synths::modules::adsr::ADSR>() {
                adsr.note_on();
            }
        }
    }

    /// Déclenche note_off sur tous les modules ADSR
    pub fn note_off(&mut self) {
        for module in &mut self.modules {
            if let Some(adsr) = module.as_any_mut().downcast_mut::<crate::synths::modules::adsr::ADSR>() {
                adsr.note_off();
            }
        }
    }
}
