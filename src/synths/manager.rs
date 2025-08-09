use crate::synths::modular::ModularSynth;
use crate::synths::modules::adsr::{ADSR, EnvelopeCurve, EnvelopeStage};
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

    /// Déclenche note_on sur tous les modules ADSR
    pub fn note_on(&mut self) {
        match self {
            SynthType::Sine(synth) => synth.note_on(),
            SynthType::Square(synth) => synth.note_on(),
            SynthType::Sawtooth(synth) => synth.note_on(),
            SynthType::FM(synth) => synth.note_on(),
            SynthType::Hammond(synth) => synth.note_on(),
        }
    }

    /// Déclenche note_off sur tous les modules ADSR
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
    /// Create a basic sine synthesizer
    pub fn n_sine() -> Self {
        let oscillator = SineOscillator;
        let gain = Gain::new(0.5);
        let mut synth = ModularSynth::new(oscillator);
        synth.add_module(gain);
        SynthType::Sine(synth)
    }

    /// Create a sine synthesizer with LFO/GAIN/ADSR
    pub fn lfo_sine() -> Self {
        let oscillator = SineOscillator;
        let gain = Gain::new(0.5);
        let lfo = LFO::new(LfoWaveform::Sine, 5.0, 44100.0);
        let adsr = ADSR::new(44100.0);
        let mut synth = ModularSynth::new(oscillator);
        synth.add_module(gain);
        synth.add_module(lfo);
        synth.add_module(adsr);
        SynthType::Sine(synth)
    }

    /// Create a basic square synthesizer
    pub fn n_square() -> Self {
        let oscillator = SquareOscillator;
        let gain = Gain::new(0.3); // Square waves are louder
        let mut synth = ModularSynth::new(oscillator);
        synth.add_module(gain);
        SynthType::Square(synth)
    }

    /// Create a basic sawtooth synthesizer
    pub fn n_sawtooth() -> Self {
        let oscillator = SawtoothOscillator;
        let gain = Gain::new(0.3);
        let mut synth = ModularSynth::new(oscillator);
        synth.add_module(gain);
        SynthType::Sawtooth(synth)
    }

    pub fn n_fm() -> Self {
        let oscillator = FmOscillator::new(0.5, 2.0); // 0.5 modulation index, 2.0 frequency ratio
        let gain = Gain::new(0.3);
        let mut synth = ModularSynth::new(oscillator);
        synth.add_module(gain);
        SynthType::FM(synth)
    }

    pub fn n_hammond() -> Self {
        let oscillator = HammondOscillator;
        let gain = Gain::new(0.3);
        let mut synth = ModularSynth::new(oscillator);
        synth.add_module(gain);
        SynthType::Hammond(synth)
    }
}
