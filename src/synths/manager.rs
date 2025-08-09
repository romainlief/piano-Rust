use crate::synths::modular::ModularSynth;
use crate::synths::modules::gain::Gain;
use crate::synths::modules::lfo::{LFO, LfoWaveform};
use crate::synths::oscillators::{
    FmOscillator, HammondOscillator, SawtoothOscillator, SineOscillator, SquareOscillator,
};

#[derive(Clone)]
pub enum SynthType {
    Sine(ModularSynth<SineOscillator>),
    Square(ModularSynth<SquareOscillator>),
    Sawtooth(ModularSynth<SawtoothOscillator>),
    FM(ModularSynth<FmOscillator>),
    Hammond(ModularSynth<HammondOscillator>),
}

impl SynthType {
    pub fn generate_sample(&mut self, phase: f64, frequency: f64) -> f64 {
        match self {
            SynthType::Sine(synth) => synth.generate_sample(phase, frequency),
            SynthType::Square(synth) => synth.generate_sample(phase, frequency),
            SynthType::Sawtooth(synth) => synth.generate_sample(phase, frequency),
            SynthType::FM(synth) => synth.generate_sample(phase, frequency),
            SynthType::Hammond(synth) => synth.generate_sample(phase, frequency),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            SynthType::Sine(_) => "Modular Sine",
            SynthType::Square(_) => "Modular Square",
            SynthType::Sawtooth(_) => "Modular Sawtooth",
            SynthType::FM(_) => "Modular FM",
            SynthType::Hammond(_) => "Modular Hammond",
        }
    }

    pub fn note_on(&mut self) {
        match self {
            SynthType::Sine(synth) => synth.note_on(),
            SynthType::Square(synth) => synth.note_on(),
            SynthType::Sawtooth(synth) => synth.note_on(),
            SynthType::FM(synth) => synth.note_on(),
            SynthType::Hammond(synth) => synth.note_on(),
        }
    }

    pub fn note_off(&mut self) {
        match self {
            SynthType::Sine(synth) => synth.note_off(),
            SynthType::Square(synth) => synth.note_off(),
            SynthType::Sawtooth(synth) => synth.note_off(),
            SynthType::FM(synth) => synth.note_off(),
            SynthType::Hammond(synth) => synth.note_off(),
        }
    }
}

impl SynthType {
    pub fn n_sine() -> Self {
        let oscillator = SineOscillator;
        let gain = Gain::new(0.6);
        let lfo = LFO::new(LfoWaveform::Sine, 5.0, 44100.0);
        let mut synth = ModularSynth::new(oscillator);
        synth.add_module(gain);
        synth.add_module(lfo);
        SynthType::Sine(synth)
    }

    pub fn n_square() -> Self {
        let oscillator = SquareOscillator;
        let gain = Gain::new(0.6);
        let lfo = LFO::new(LfoWaveform::Square, 5.0, 44100.0);
        let mut synth = ModularSynth::new(oscillator);
        synth.add_module(gain);
        synth.add_module(lfo);
        SynthType::Square(synth)
    }

    pub fn n_sawtooth() -> Self {
        let oscillator = SawtoothOscillator;
        let gain = Gain::new(0.6);
        let lfo = LFO::new(LfoWaveform::SawUp, 5.0, 44100.0);
        let mut synth = ModularSynth::new(oscillator);
        synth.add_module(gain);
        synth.add_module(lfo);
        SynthType::Sawtooth(synth)
    }

    pub fn n_fm() -> Self {
        let oscillator = FmOscillator::new(3.5, 1.414);
        let gain = Gain::new(0.6); // Gain plus faible car FM peut être fort
        let lfo = LFO::new(LfoWaveform::SawDown, 5.0, 44100.0);
        let mut synth = ModularSynth::new(oscillator);
        synth.add_module(gain);
        synth.add_module(lfo);
        SynthType::FM(synth)
    }

    pub fn n_hammond() -> Self {
        let oscillator = HammondOscillator;
        let gain = Gain::new(0.6);
        let lfo = LFO::new(LfoWaveform::Sine, 5.0, 44100.0);
        let mut synth = ModularSynth::new(oscillator);
        synth.add_module(gain);
        synth.add_module(lfo);
        SynthType::Hammond(synth)
    }
}
