use crate::consts::constants;
use crate::synths::modular::ModularSynth;
use crate::synths::modules::compressor::Compressor;
use crate::synths::modules::filter::LowPassFilter;
use crate::synths::modules::gain::Gain;
use crate::synths::modules::lfo::{LFO, LfoWaveform};
use crate::synths::modules::noise::Noise;
use crate::synths::oscillators::{
    FmOscillator, HammondOscillator, SawtoothOscillator, SineOscillator, SquareOscillator,
};

#[derive(Clone)]
/// Enum representing different types of modular synthesizers
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
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    pub fn n_sine() -> Self {
        let oscillator: SineOscillator = SineOscillator;

        let compressor = Compressor::new(
            constants::SINE_CURRENT_THRESHOLD,
            constants::SINE_CURRENT_RATIO,
            constants::SINE_CURRENT_ATTACK,
            constants::SINE_CURRENT_RELEASE,
            constants::SINE_CURRENT_GAIN,
            constants::SAMPLE_RATE,
        );

        let noise = Noise::new(constants::SINE_CURRENT_NOISE);

        let gain = Gain::new(constants::SINE_CURRENT_GAIN);

        let lfo = LFO::new(
            LfoWaveform::Sine,
            constants::SINE_CURRENT_LFO_FREQ,
            constants::SAMPLE_RATE,
        );

        let filter = LowPassFilter::new(
            constants::SINE_CURRENT_FILTER_CUTOFF,
            constants::SINE_CURRENT_FILTER_RESONANCE,
            constants::SAMPLE_RATE,
        );

        let mut synth: ModularSynth<SineOscillator> = ModularSynth::new(oscillator);
        if constants::SINE_ACTIVATION_NOISE {
            synth.add_module(noise);
        }
        if constants::SINE_ACTIVATION_LFO {
            synth.add_module(lfo);
        }
        if constants::SINE_ACTIVATION_FILTER {
            synth.add_module(filter);
        }
        if constants::SINE_ACTIVATION_GAIN {
            synth.add_module(gain);
        }
        if constants::SINE_ACTIVATION_COMPRESSOR {
            synth.add_module(compressor);
        }
        SynthType::Sine(synth)
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    pub fn n_square() -> Self {
        let oscillator = SquareOscillator;

        let gain = Gain::new(constants::SQUARE_CURRENT_GAIN);

        let noise: Noise = Noise::new(constants::SQUARE_CURRENT_NOISE);

        let compressor = Compressor::new(
            constants::SQUARE_CURRENT_THRESHOLD,
            constants::SQUARE_CURRENT_RATIO,
            constants::SQUARE_CURRENT_ATTACK,
            constants::SQUARE_CURRENT_RELEASE,
            constants::SQUARE_CURRENT_GAIN,
            constants::SAMPLE_RATE,
        );

        let lfo = LFO::new(
            LfoWaveform::Square,
            constants::SQUARE_CURRENT_LFO_FREQ,
            constants::SAMPLE_RATE,
        );

        let filter = LowPassFilter::new(
            constants::SQUARE_CURRENT_FILTER_CUTOFF,
            constants::SQUARE_CURRENT_FILTER_RESONANCE,
            constants::SAMPLE_RATE,
        );

        let mut synth = ModularSynth::new(oscillator);
        if constants::SQUARE_ACTIVATION_NOISE {
            synth.add_module(noise);
        }
        if constants::SQUARE_ACTIVATION_LFO {
            synth.add_module(lfo);
        }
        if constants::SQUARE_ACTIVATION_FILTER {
            synth.add_module(filter);
        }
        if constants::SQUARE_ACTIVATION_GAIN {
            synth.add_module(gain);
        }
        if constants::SQUARE_ACTIVATION_COMPRESSOR {
            synth.add_module(compressor);
        }
        SynthType::Square(synth)
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    pub fn n_sawtooth() -> Self {
        let oscillator = SawtoothOscillator;

        let gain = Gain::new(constants::SAWTOOTH_CURRENT_GAIN);

        let noise = Noise::new(constants::SAWTOOTH_CURRENT_NOISE);

        let compressor = Compressor::new(
            constants::SAWTOOTH_CURRENT_THRESHOLD,
            constants::SAWTOOTH_CURRENT_RATIO,
            constants::SAWTOOTH_CURRENT_ATTACK,
            constants::SAWTOOTH_CURRENT_RELEASE,
            constants::SAWTOOTH_CURRENT_GAIN,
            constants::SAMPLE_RATE,
        );

        let filter = LowPassFilter::new(
            constants::SAWTOOTH_CURRENT_FILTER_CUTOFF,
            constants::SAWTOOTH_CURRENT_FILTER_RESONANCE,
            constants::SAMPLE_RATE,
        );

        let lfo = LFO::new(
            LfoWaveform::SawUp,
            constants::SAWTOOTH_CURRENT_LFO_FREQ,
            constants::SAMPLE_RATE,
        );

        let mut synth = ModularSynth::new(oscillator);
        if constants::SAWTOOTH_ACTIVATION_NOISE {
            synth.add_module(noise);
        }
        if constants::SAWTOOTH_ACTIVATION_LFO {
            synth.add_module(lfo);
        }
        if constants::SAWTOOTH_ACTIVATION_FILTER {
            synth.add_module(filter);
        }
        if constants::SAWTOOTH_ACTIVATION_GAIN {
            synth.add_module(gain);
        }
        if constants::SAWTOOTH_ACTIVATION_COMPRESSOR {
            synth.add_module(compressor);
        }
        SynthType::Sawtooth(synth)
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    pub fn n_fm() -> Self {
        let oscillator = FmOscillator::new(3.5, 1.414);

        let gain = Gain::new(constants::FM_CURRENT_GAIN);

        let noise = Noise::new(constants::FM_CURRENT_NOISE);

        let compressor = Compressor::new(
            constants::FM_CURRENT_THRESHOLD,
            constants::FM_CURRENT_RATIO,
            constants::FM_CURRENT_ATTACK,
            constants::FM_CURRENT_RELEASE,
            constants::FM_CURRENT_GAIN,
            constants::SAMPLE_RATE,
        );

        let filter = LowPassFilter::new(
            constants::FM_CURRENT_FILTER_CUTOFF,
            constants::FM_CURRENT_FILTER_RESONANCE,
            constants::SAMPLE_RATE,
        );

        let lfo = LFO::new(
            LfoWaveform::SawDown,
            constants::FM_CURRENT_LFO_FREQ,
            constants::SAMPLE_RATE,
        );

        let mut synth = ModularSynth::new(oscillator);
        if constants::FM_ACTIVATION_NOISE {
            synth.add_module(noise);
        }
        if constants::FM_ACTIVATION_LFO {
            synth.add_module(lfo);
        }
        if constants::FM_ACTIVATION_FILTER {
            synth.add_module(filter);
        }
        if constants::FM_ACTIVATION_GAIN {
            synth.add_module(gain);
        }
        if constants::FM_ACTIVATION_COMPRESSOR {
            synth.add_module(compressor);
        }
        SynthType::FM(synth)
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    pub fn n_hammond() -> Self {
        let oscillator = HammondOscillator;

        let gain = Gain::new(constants::HAMMOND_CURRENT_GAIN);

        let noise = Noise::new(constants::HAMMOND_CURRENT_NOISE);

        let compressor = Compressor::new(
            constants::HAMMOND_CURRENT_THRESHOLD,
            constants::HAMMOND_CURRENT_RATIO,
            constants::HAMMOND_CURRENT_ATTACK,
            constants::HAMMOND_CURRENT_RELEASE,
            constants::HAMMOND_CURRENT_GAIN,
            constants::SAMPLE_RATE,
        );

        let filter = LowPassFilter::new(
            constants::HAMMOND_CURRENT_FILTER_CUTOFF,
            constants::HAMMOND_CURRENT_FILTER_RESONANCE,
            constants::SAMPLE_RATE,
        );

        let lfo = LFO::new(
            LfoWaveform::Sine,
            constants::HAMMOND_CURRENT_LFO_FREQ,
            constants::SAMPLE_RATE,
        );

        let mut synth = ModularSynth::new(oscillator);
        if constants::HAMMOND_ACTIVATION_NOISE {
            synth.add_module(noise);
        }
        if constants::HAMMOND_ACTIVATION_LFO {
            synth.add_module(lfo);
        }
        if constants::HAMMOND_ACTIVATION_FILTER {
            synth.add_module(filter);
        }
        if constants::HAMMOND_ACTIVATION_GAIN {
            synth.add_module(gain);
        }
        if constants::HAMMOND_ACTIVATION_COMPRESSOR {
            synth.add_module(compressor);
        }
        SynthType::Hammond(synth)
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
}
