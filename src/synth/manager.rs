use crate::synth::traits::Synthesizer;
use crate::synth::types::{FMSynth, HammondSynth, SawtoothSynth, SineSynth, SquareSynth};

#[derive(Clone, Copy, Debug)]
// Enum to manage different synthesizer types
pub enum SynthType {
    Sine(SineSynth),
    Square(SquareSynth),
    Sawtooth(SawtoothSynth),
    Hammond(HammondSynth),
    FM(FMSynth),
}

impl SynthType {
    pub fn generate_sample(&self, phase: f64, frequency: f64) -> f64 {
        match self {
            SynthType::Sine(synth) => synth.generate_sample(phase, frequency),
            SynthType::Square(synth) => synth.generate_sample(phase, frequency),
            SynthType::Sawtooth(synth) => synth.generate_sample(phase, frequency),
            SynthType::Hammond(synth) => synth.generate_sample(phase, frequency),
            SynthType::FM(synth) => synth.generate_sample(phase, frequency),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            SynthType::Sine(synth) => synth.name(),
            SynthType::Square(synth) => synth.name(),
            SynthType::Sawtooth(synth) => synth.name(),
            SynthType::Hammond(synth) => synth.name(),
            SynthType::FM(synth) => synth.name(),
        }
    }
}