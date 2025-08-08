use crate::synths::traits::Synthesizer;

// Hammond synthesizer with harmonic control
#[derive(Clone, Copy, Debug)]
pub struct HammondSynth {
    pub fundamental: f64,
    pub harmonic2: f64,
    pub harmonic3: f64,
    pub harmonic4: f64,
    pub harmonic5: f64,
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

    pub fn classic() -> Self {
        Self {
            fundamental: 1.0,
            harmonic2: 0.8,
            harmonic3: 0.6,
            harmonic4: 0.4,
            harmonic5: 0.2,
        }
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
