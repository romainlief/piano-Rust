use crate::synths::traits::Synthesizer;

// Hammond synthesizer with harmonic control
#[derive(Clone, Copy, Debug)]
pub struct HammondSynth {
    fundamental: f64,
    harmonic2: f64,
    harmonic3: f64,
    harmonic4: f64,
    harmonic5: f64,
}

impl HammondSynth {
    pub fn new() -> Self {
        Self {
            fundamental: 1.0,
            harmonic2: 0.5,
            harmonic3: 0.3,
            harmonic4: 0.2,
            harmonic5: 0.1,
        }
    }
    pub fn get_fundamental(&self) -> f64 {
        self.fundamental
    }

    pub fn set_fundamental(&mut self, value: f64) {
        self.fundamental = value;
    }

    pub fn get_harmonic2(&self) -> f64 {
        self.harmonic2
    }

    pub fn set_harmonic2(&mut self, value: f64) {
        self.harmonic2 = value;
    }

    pub fn get_harmonic3(&self) -> f64 {
        self.harmonic3
    }

    pub fn set_harmonic3(&mut self, value: f64) {
        self.harmonic3 = value;
    }

    pub fn get_harmonic4(&self) -> f64 {
        self.harmonic4
    }

    pub fn set_harmonic4(&mut self, value: f64) {
        self.harmonic4 = value;
    }

    pub fn get_harmonic5(&self) -> f64 {
        self.harmonic5
    }

    pub fn set_harmonic5(&mut self, value: f64) {
        self.harmonic5 = value;
    }
}

impl Synthesizer for HammondSynth {
    fn generate_sample(&self, phase: f64, _frequency: f64) -> f64 {
        let fund = phase.sin() * self.fundamental;
        let harm2 = (phase * 2.0).sin() * self.harmonic2;
        let harm3 = (phase * 3.0).sin() * self.harmonic3;
        let harm4 = (phase * 4.0).sin() * self.harmonic4;
        let harm5 = (phase * 5.0).sin() * self.harmonic5;

        fund + harm2 + harm3 + harm4 + harm5
    }

    fn name(&self) -> &'static str {
        "Hammond"
    }
}
