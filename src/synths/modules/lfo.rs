use crate::synths::traits::Module;

#[derive(Clone, Copy)]
pub struct LFO {
    pub freq: f64,  // Hz
    pub depth: f64, // %
    pub sample_rate: f64,
    phase: f64,
}

impl LFO {
    pub fn new(freq: f64, depth: f64, sample_rate: f64) -> Self {
        Self {
            freq,
            depth,
            sample_rate,
            phase: 0.0,
        }
    }
}

impl Module for LFO {
    fn process(&mut self, input: f64, _time: f64) -> f64 {
        let mod_signal = (self.phase * 2.0 * std::f64::consts::PI).sin() * self.depth;
        self.phase += self.freq / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        input * (1.0 + mod_signal)
    }

    fn name(&self) -> &'static str {
        "LFO"
    }

    fn clone_box(&self) -> Box<dyn Module> {
        Box::new(*self)
    }
}
