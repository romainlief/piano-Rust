use crate::synth::traits::Synthesizer;

// FM synthesizer (Frequency Modulation)
#[derive(Clone, Copy, Debug)]
pub struct FMSynth {
    pub carrier_amplitude: f64,
    pub modulator_frequency_ratio: f64,
    pub modulation_index: f64,
}

impl FMSynth {
    pub fn new(mod_freq_ratio: f64, mod_index: f64) -> Self {
        Self {
            carrier_amplitude: 1.0,
            modulator_frequency_ratio: mod_freq_ratio,
            modulation_index: mod_index,
        }
    }
}

impl Synthesizer for FMSynth {
    fn generate_sample(&self, phase: f64, frequency: f64) -> f64 {
        let modulator_phase = phase * self.modulator_frequency_ratio;
        let modulator = modulator_phase.sin() * self.modulation_index;

        (phase + modulator).sin() * self.carrier_amplitude
    }

    fn name(&self) -> &'static str {
        "FM"
    }
}
