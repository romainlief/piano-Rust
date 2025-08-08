use crate::synths::traits::Synthesizer;

// FM synthesizer (Frequency Modulation)
#[derive(Clone, Copy, Debug)]
pub struct FMSynth {
    carrier_amplitude: f64,
    modulator_frequency_ratio: f64,
    modulation_index: f64,
}

impl FMSynth {
    pub fn new(mod_freq_ratio: f64, mod_index: f64) -> Self {
        Self {
            carrier_amplitude: 1.0,
            modulator_frequency_ratio: mod_freq_ratio,
            modulation_index: mod_index,
        }
    }

    pub fn get_carrier_amplitude(&self) -> f64 {
        self.carrier_amplitude
    }

    pub fn set_carrier_amplitude(&mut self, value: f64) {
        self.carrier_amplitude = value;
    }

    pub fn get_modulator_frequency_ratio(&self) -> f64 {
        self.modulator_frequency_ratio
    }

    pub fn set_modulator_frequency_ratio(&mut self, value: f64) {
        self.modulator_frequency_ratio = value;
    }

    pub fn get_modulation_index(&self) -> f64 {
        self.modulation_index
    }

    pub fn set_modulation_index(&mut self, value: f64) {
        self.modulation_index = value;
    }
}

impl Synthesizer for FMSynth {
    fn generate_sample(&self, phase: f64, _frequency: f64) -> f64 {
        let modulator_phase = phase * self.modulator_frequency_ratio;
        let modulator = modulator_phase.sin() * self.modulation_index;

        (phase + modulator).sin() * self.carrier_amplitude
    }

    fn name(&self) -> &'static str {
        "FM"
    }
}
