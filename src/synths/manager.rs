use crate::consts::constants;
use crate::synths::modular::ModularSynth;
use crate::synths::modules::compressor::Compressor;
use crate::synths::modules::filter::LowPassFilter;
use crate::synths::modules::gain::Gain;
use crate::synths::modules::lfo::LFO;
use crate::synths::modules::noise::Noise;
use crate::synths::modules::reverb::Reverb;
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

    pub fn get_current_gain(&self) -> f64 {
        match self {
            SynthType::Sine(synth) => self.get_gain_from_synth(synth),
            SynthType::Square(synth) => self.get_gain_from_synth(synth),
            SynthType::Sawtooth(synth) => self.get_gain_from_synth(synth),
            SynthType::FM(synth) => self.get_gain_from_synth(synth),
            SynthType::Hammond(synth) => self.get_gain_from_synth(synth),
        }
    }

    pub fn set_current_gain(&mut self, new_gain: f64) {
        match self {
            SynthType::Sine(synth) => {
                Self::set_gain_in_synth_static(synth, new_gain);
            }
            SynthType::Square(synth) => {
                Self::set_gain_in_synth_static(synth, new_gain);
            }
            SynthType::Sawtooth(synth) => {
                Self::set_gain_in_synth_static(synth, new_gain);
            }
            SynthType::FM(synth) => {
                Self::set_gain_in_synth_static(synth, new_gain);
            }
            SynthType::Hammond(synth) => {
                Self::set_gain_in_synth_static(synth, new_gain);
            }
        }
    }

    /// Helper pour récupérer le gain d'un synthétiseur modulaire
    fn get_gain_from_synth<O: crate::synths::traits::Oscillator>(
        &self,
        synth: &ModularSynth<O>,
    ) -> f64 {
        // Parcourir les modules pour trouver le module Gain
        for module in &synth.modules {
            if module.name() == "Gain" {
                // Utiliser le downcasting en lecture seule
                if let Some(gain_module) = module.as_any().downcast_ref::<Gain>() {
                    println!("Gain module found: {}", gain_module.get_gain());
                    return gain_module.get_gain();
                }
            }
        }
        // Si pas de module gain trouvé, retourner la valeur par défaut
        constants::CURRENT_GAIN
    }

    /// Helper pour mettre à jour le gain d'un synthétiseur modulaire
    fn set_gain_in_synth_static<O: crate::synths::traits::Oscillator>(
        synth: &mut ModularSynth<O>,
        new_gain: f64,
    ) {
        // Parcourir les modules pour trouver le module Gain
        for module in &mut synth.modules {
            if module.name() == "Gain" {
                // Utiliser le downcasting mutable pour modifier le gain
                if let Some(gain_module) = module.as_any_mut().downcast_mut::<Gain>() {
                    gain_module.set_gain(new_gain);
                    return;
                }
            }
        }
        println!("Module Gain non trouvé pour mise à jour");
    }
}

impl std::fmt::Debug for SynthType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SynthType::Sine(_) => write!(f, "Sine"),
            SynthType::Square(_) => write!(f, "Square"),
            SynthType::Sawtooth(_) => write!(f, "Sawtooth"),
            SynthType::FM(_) => write!(f, "FM"),
            SynthType::Hammond(_) => write!(f, "Hammond"),
        }
    }
}

impl SynthType {
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    pub fn n_sine() -> Self {
        let oscillator: SineOscillator = SineOscillator;

        let compressor = Compressor::new(
            constants::CURRENT_THRESHOLD,
            constants::CURRENT_RATIO,
            constants::CURRENT_ATTACK,
            constants::CURRENT_RELEASE,
            constants::CURRENT_GAIN,
            constants::SAMPLE_RATE,
        );

        let noise = Noise::new(constants::CURRENT_NOISE);

        let gain = Gain::new(constants::CURRENT_GAIN);

        let lfo = LFO::new(
            constants::CURRENT_LFO_WAVEFORM,
            constants::CURRENT_LFO_FREQ,
            constants::SAMPLE_RATE,
        );

        let filter = LowPassFilter::new(
            constants::CURRENT_FILTER_CUTOFF,
            constants::CURRENT_FILTER_RESONANCE,
            constants::SAMPLE_RATE,
        );

        let reverb = Reverb::new(
            constants::SAMPLE_RATE,
            constants::CURRENT_REVERB_TYPE,
            constants::CURRENT_DRY_WET,
            constants::CURRENT_REVERB_EARLY_GAIN,
            constants::CURRENT_REVERB_TAIL_GAIN,
            constants::CURRENT_REVERB_PRE_DELAY_MS,
        );

        let mut synth: ModularSynth<SineOscillator> = ModularSynth::new(oscillator);
        if constants::ACTIVATION_NOISE {
            synth.add_module(noise);
        }
        if constants::ACTIVATION_LFO {
            synth.add_module(lfo);
        }
        if constants::ACTIVATION_FILTER {
            synth.add_module(filter);
        }
        if constants::ACTIVATION_GAIN {
            synth.add_module(gain);
        }
        if constants::ACTIVATION_COMPRESSOR {
            synth.add_module(compressor);
        }
        if constants::ACTIVATION_REVERB {
            synth.add_module(reverb);
        }
        SynthType::Sine(synth)
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    pub fn n_square() -> Self {
        let oscillator = SquareOscillator;

        let gain = Gain::new(constants::CURRENT_GAIN);

        let noise: Noise = Noise::new(constants::CURRENT_NOISE);

        let compressor = Compressor::new(
            constants::CURRENT_THRESHOLD,
            constants::CURRENT_RATIO,
            constants::CURRENT_ATTACK,
            constants::CURRENT_RELEASE,
            constants::CURRENT_GAIN,
            constants::SAMPLE_RATE,
        );

        let lfo = LFO::new(
            constants::CURRENT_LFO_WAVEFORM,
            constants::CURRENT_LFO_FREQ,
            constants::SAMPLE_RATE,
        );

        let filter = LowPassFilter::new(
            constants::CURRENT_FILTER_CUTOFF,
            constants::CURRENT_FILTER_RESONANCE,
            constants::SAMPLE_RATE,
        );

        let reverb = Reverb::new(
            constants::SAMPLE_RATE,
            constants::CURRENT_REVERB_TYPE,
            constants::CURRENT_DRY_WET,
            constants::CURRENT_REVERB_EARLY_GAIN,
            constants::CURRENT_REVERB_TAIL_GAIN,
            constants::CURRENT_REVERB_PRE_DELAY_MS,
        );

        let mut synth = ModularSynth::new(oscillator);
        if constants::ACTIVATION_NOISE {
            synth.add_module(noise);
        }
        if constants::ACTIVATION_LFO {
            synth.add_module(lfo);
        }
        if constants::ACTIVATION_FILTER {
            synth.add_module(filter);
        }
        if constants::ACTIVATION_GAIN {
            synth.add_module(gain);
        }
        if constants::ACTIVATION_COMPRESSOR {
            synth.add_module(compressor);
        }
        if constants::ACTIVATION_REVERB {
            synth.add_module(reverb);
        }
        SynthType::Square(synth)
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    pub fn n_sawtooth() -> Self {
        let oscillator = SawtoothOscillator;

        let gain = Gain::new(constants::CURRENT_GAIN);

        let noise = Noise::new(constants::CURRENT_NOISE);

        let compressor = Compressor::new(
            constants::CURRENT_THRESHOLD,
            constants::CURRENT_RATIO,
            constants::CURRENT_ATTACK,
            constants::CURRENT_RELEASE,
            constants::CURRENT_GAIN,
            constants::SAMPLE_RATE,
        );

        let filter = LowPassFilter::new(
            constants::CURRENT_FILTER_CUTOFF,
            constants::CURRENT_FILTER_RESONANCE,
            constants::SAMPLE_RATE,
        );

        let lfo = LFO::new(
            constants::CURRENT_LFO_WAVEFORM,
            constants::CURRENT_LFO_FREQ,
            constants::SAMPLE_RATE,
        );

        let reverb = Reverb::new(
            constants::SAMPLE_RATE,
            constants::CURRENT_REVERB_TYPE,
            constants::CURRENT_DRY_WET,
            constants::CURRENT_REVERB_EARLY_GAIN,
            constants::CURRENT_REVERB_TAIL_GAIN,
            constants::CURRENT_REVERB_PRE_DELAY_MS,
        );

        let mut synth = ModularSynth::new(oscillator);
        if constants::ACTIVATION_NOISE {
            synth.add_module(noise);
        }
        if constants::ACTIVATION_LFO {
            synth.add_module(lfo);
        }
        if constants::ACTIVATION_FILTER {
            synth.add_module(filter);
        }
        if constants::ACTIVATION_GAIN {
            synth.add_module(gain);
        }
        if constants::ACTIVATION_COMPRESSOR {
            synth.add_module(compressor);
        }
        if constants::ACTIVATION_REVERB {
            synth.add_module(reverb);
        }
        SynthType::Sawtooth(synth)
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    pub fn n_fm() -> Self {
        let oscillator = FmOscillator::new(3.5, 1.414);

        let gain = Gain::new(constants::CURRENT_GAIN);

        let noise = Noise::new(constants::CURRENT_NOISE);

        let compressor = Compressor::new(
            constants::CURRENT_THRESHOLD,
            constants::CURRENT_RATIO,
            constants::CURRENT_ATTACK,
            constants::CURRENT_RELEASE,
            constants::CURRENT_GAIN,
            constants::SAMPLE_RATE,
        );

        let filter = LowPassFilter::new(
            constants::CURRENT_FILTER_CUTOFF,
            constants::CURRENT_FILTER_RESONANCE,
            constants::SAMPLE_RATE,
        );

        let lfo = LFO::new(
            constants::CURRENT_LFO_WAVEFORM,
            constants::CURRENT_LFO_FREQ,
            constants::SAMPLE_RATE,
        );

        let reverb = Reverb::new(
            constants::SAMPLE_RATE,
            constants::CURRENT_REVERB_TYPE,
            constants::CURRENT_DRY_WET,
            constants::CURRENT_REVERB_EARLY_GAIN,
            constants::CURRENT_REVERB_TAIL_GAIN,
            constants::CURRENT_REVERB_PRE_DELAY_MS,
        );

        let mut synth = ModularSynth::new(oscillator);
        if constants::ACTIVATION_NOISE {
            synth.add_module(noise);
        }
        if constants::ACTIVATION_LFO {
            synth.add_module(lfo);
        }
        if constants::ACTIVATION_FILTER {
            synth.add_module(filter);
        }
        if constants::ACTIVATION_GAIN {
            synth.add_module(gain);
        }
        if constants::ACTIVATION_COMPRESSOR {
            synth.add_module(compressor);
        }
        if constants::ACTIVATION_REVERB {
            synth.add_module(reverb);
        }
        SynthType::FM(synth)
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    pub fn n_hammond() -> Self {
        let oscillator = HammondOscillator;

        let gain = Gain::new(constants::CURRENT_GAIN);

        let noise = Noise::new(constants::CURRENT_NOISE);

        let compressor = Compressor::new(
            constants::CURRENT_THRESHOLD,
            constants::CURRENT_RATIO,
            constants::CURRENT_ATTACK,
            constants::CURRENT_RELEASE,
            constants::CURRENT_GAIN,
            constants::SAMPLE_RATE,
        );

        let filter = LowPassFilter::new(
            constants::CURRENT_FILTER_CUTOFF,
            constants::CURRENT_FILTER_RESONANCE,
            constants::SAMPLE_RATE,
        );

        let lfo = LFO::new(
            constants::CURRENT_LFO_WAVEFORM,
            constants::CURRENT_LFO_FREQ,
            constants::SAMPLE_RATE,
        );

        let reverb = Reverb::new(
            constants::SAMPLE_RATE,
            constants::CURRENT_REVERB_TYPE,
            constants::CURRENT_DRY_WET,
            constants::CURRENT_REVERB_EARLY_GAIN,
            constants::CURRENT_REVERB_TAIL_GAIN,
            constants::CURRENT_REVERB_PRE_DELAY_MS,
        );

        let mut synth = ModularSynth::new(oscillator);
        if constants::ACTIVATION_NOISE {
            synth.add_module(noise);
        }
        if constants::ACTIVATION_LFO {
            synth.add_module(lfo);
        }
        if constants::ACTIVATION_FILTER {
            synth.add_module(filter);
        }
        if constants::ACTIVATION_GAIN {
            synth.add_module(gain);
        }
        if constants::ACTIVATION_COMPRESSOR {
            synth.add_module(compressor);
        }
        if constants::ACTIVATION_REVERB {
            synth.add_module(reverb);
        }
        SynthType::Hammond(synth)
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
}
