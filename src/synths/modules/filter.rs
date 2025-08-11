use crate::synths::traits::Module;

#[derive(Clone, Copy)]
pub struct LowPassFilter {
    sample_rate: f64,
    cutoff_freq: f64,
    resonance: f64,
    // coefficients
    a0: f64,
    a1: f64,
    a2: f64,
    b1: f64,
    b2: f64,
    // mémoire des échantillons
    z1: f64,
    z2: f64,
}

impl LowPassFilter {
    pub fn new(sample_rate: f64, cutoff_freq: f64, resonance: f64) -> Self {
        let mut filter = LowPassFilter {
            sample_rate,
            cutoff_freq,
            resonance,
            a0: 0.0,
            a1: 0.0,
            a2: 0.0,
            b1: 0.0,
            b2: 0.0,
            z1: 0.0,
            z2: 0.0,
        };
        filter.calc_coefficients();
        filter
    }

    fn calc_coefficients(&mut self) {
        let omega = 2.0 * std::f64::consts::PI * self.cutoff_freq / self.sample_rate;
        let alpha = omega.sin() / (2.0 * self.resonance);
        let cos_omega = omega.cos();

        let b0 = (1.0 - cos_omega) / 2.0;
        let b1 = 1.0 - cos_omega;
        let b2 = (1.0 - cos_omega) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_omega;
        let a2 = 1.0 - alpha;

        self.a0 = b0 / a0;
        self.a1 = b1 / a0;
        self.a2 = b2 / a0;
        self.b1 = a1 / a0;
        self.b2 = a2 / a0;
    }

    // #### Setters ####

    pub fn set_cutoff_freq(&mut self, cutoff_freq: f64) {
        self.cutoff_freq = cutoff_freq;
        self.calc_coefficients();
    }

    pub fn set_resonance(&mut self, resonance: f64) {
        self.resonance = resonance;
        self.calc_coefficients();
    }

    pub fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
        self.calc_coefficients();
    }

    // #### Getters ####

    pub fn get_cutoff_freq(&self) -> f64 {
        self.cutoff_freq
    }

    pub fn get_resonance(&self) -> f64 {
        self.resonance
    }

    pub fn get_sample_rate(&self) -> f64 {
        self.sample_rate
    }
}

impl Module for LowPassFilter {
    fn process(&mut self, input: f64, _time: f64) -> f64 {
        let output = self.a0 * input + self.a1 * self.z1 + self.a2 * self.z2
            - self.b1 * self.z1
            - self.b2 * self.z2;
        self.z2 = self.z1;
        self.z1 = output;
        output
    }

    fn name(&self) -> &'static str {
        "LowPassFilter"
    }

    fn clone_box(&self) -> Box<dyn Module> {
        Box::new(*self)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
