use crate::consts::constants;
use crate::synths::modular::ModularSynth;
use crate::synths::modules::compressor::Compressor;
use crate::synths::modules::filter::LowPassFilter;
use crate::synths::modules::gain::Gain;
use crate::synths::modules::lfo::{LFO, LfoWaveform};
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

    pub fn get_current_noise(&self) -> f64 {
        match self {
            SynthType::Sine(synth) => self.get_noise_from_synth(synth),
            SynthType::Square(synth) => self.get_noise_from_synth(synth),
            SynthType::Sawtooth(synth) => self.get_noise_from_synth(synth),
            SynthType::FM(synth) => self.get_noise_from_synth(synth),
            SynthType::Hammond(synth) => self.get_noise_from_synth(synth),
        }
    }

    pub fn set_current_cutoff(&mut self, new_cutoff: f64) {
        match self {
            SynthType::Sine(synth) => {
                Self::set_cutoff_in_synth_static(synth, new_cutoff);
            }
            SynthType::Square(synth) => {
                Self::set_cutoff_in_synth_static(synth, new_cutoff);
            }
            SynthType::Sawtooth(synth) => {
                Self::set_cutoff_in_synth_static(synth, new_cutoff);
            }
            SynthType::FM(synth) => {
                Self::set_cutoff_in_synth_static(synth, new_cutoff);
            }
            SynthType::Hammond(synth) => {
                Self::set_cutoff_in_synth_static(synth, new_cutoff);
            }
        }
    }

    pub fn set_current_resonance(&mut self, new_resonance: f64) {
        match self {
            SynthType::Sine(synth) => {
                Self::set_resonance_in_synth_static(synth, new_resonance);
            }
            SynthType::Square(synth) => {
                Self::set_resonance_in_synth_static(synth, new_resonance);
            }
            SynthType::Sawtooth(synth) => {
                Self::set_resonance_in_synth_static(synth, new_resonance);
            }
            SynthType::FM(synth) => {
                Self::set_resonance_in_synth_static(synth, new_resonance);
            }
            SynthType::Hammond(synth) => {
                Self::set_resonance_in_synth_static(synth, new_resonance);
            }
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

    pub fn set_current_noise(&mut self, new_noise: f64) {
        match self {
            SynthType::Sine(synth) => Self::set_noise_in_synth_static(synth, new_noise),
            SynthType::Square(synth) => Self::set_noise_in_synth_static(synth, new_noise),
            SynthType::Sawtooth(synth) => Self::set_noise_in_synth_static(synth, new_noise),
            SynthType::FM(synth) => Self::set_noise_in_synth_static(synth, new_noise),
            SynthType::Hammond(synth) => Self::set_noise_in_synth_static(synth, new_noise),
        }
    }

    pub fn set_current_lfo_frequency(&mut self, frequency: f64) {
        match self {
            SynthType::Sine(synth) => Self::set_lfo_frequency_in_synth_static(synth, frequency),
            SynthType::Square(synth) => Self::set_lfo_frequency_in_synth_static(synth, frequency),
            SynthType::Sawtooth(synth) => Self::set_lfo_frequency_in_synth_static(synth, frequency),
            SynthType::FM(synth) => Self::set_lfo_frequency_in_synth_static(synth, frequency),
            SynthType::Hammond(synth) => Self::set_lfo_frequency_in_synth_static(synth, frequency),
        }
    }

    pub fn set_current_lfo_waveform(&mut self, waveform: LfoWaveform) {
        match self {
            SynthType::Sine(synth) => Self::set_lfo_waveform_in_synth_static(synth, waveform),
            SynthType::Square(synth) => Self::set_lfo_waveform_in_synth_static(synth, waveform),
            SynthType::Sawtooth(synth) => Self::set_lfo_waveform_in_synth_static(synth, waveform),
            SynthType::FM(synth) => Self::set_lfo_waveform_in_synth_static(synth, waveform),
            SynthType::Hammond(synth) => Self::set_lfo_waveform_in_synth_static(synth, waveform),
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
        constants::CURRENT_GAIN
    }

    fn get_noise_from_synth<O: crate::synths::traits::Oscillator>(
        &self,
        synth: &ModularSynth<O>,
    ) -> f64 {
        // Parcourir les modules pour trouver le module Noise
        for module in &synth.modules {
            if module.name() == "NoiseEffect" {
                // Utiliser le downcasting en lecture seule
                if let Some(noise_module) = module.as_any().downcast_ref::<Noise>() {
                    println!("Noise module found: {}", noise_module.get_amount());
                    return noise_module.get_amount();
                }
            }
        }
        constants::CURRENT_NOISE
    }

    fn set_cutoff_in_synth_static<O: crate::synths::traits::Oscillator>(
        synth: &mut ModularSynth<O>,
        new_cutoff: f64,
    ) {
        for module in &mut synth.modules {
            if module.name() == "LowPassFilter" {
                if let Some(filter_module) = module.as_any_mut().downcast_mut::<LowPassFilter>() {
                    filter_module.set_cutoff_freq(new_cutoff);
                    return;
                }
            }
        }
        println!("Module Filter non trouvé pour mise à jour");
    }

    fn set_resonance_in_synth_static<O: crate::synths::traits::Oscillator>(
        synth: &mut ModularSynth<O>,
        new_resonance: f64,
    ) {
        for module in &mut synth.modules {
            if module.name() == "LowPassFilter" {
                if let Some(filter_module) = module.as_any_mut().downcast_mut::<LowPassFilter>() {
                    filter_module.set_resonance(new_resonance);
                    return;
                }
            }
        }
        println!("Module Filter non trouvé pour mise à jour");
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

    fn set_lfo_frequency_in_synth_static<O: crate::synths::traits::Oscillator>(
        synth: &mut ModularSynth<O>,
        new_frequency: f64,
    ) {
        for module in &mut synth.modules {
            if module.name() == "LFO" {
                if let Some(lfo_module) = module.as_any_mut().downcast_mut::<LFO>() {
                    lfo_module.set_freq(new_frequency);
                    return;
                }
            }
        }
        println!("Module LFO non trouvé pour mise à jour");
    }

    fn set_lfo_waveform_in_synth_static<O: crate::synths::traits::Oscillator>(
        synth: &mut ModularSynth<O>,
        new_waveform: LfoWaveform,
    ) {
        for module in &mut synth.modules {
            if module.name() == "LFO" {
                if let Some(lfo_module) = module.as_any_mut().downcast_mut::<LFO>() {
                    lfo_module.set_waveform(new_waveform);
                    return;
                }
            }
        }
        println!("Module LFO non trouvé pour mise à jour");
    }

    fn set_noise_in_synth_static<O: crate::synths::traits::Oscillator>(
        synth: &mut ModularSynth<O>,
        new_noise: f64,
    ) {
        // Parcourir les modules pour trouver le module Noise
        for module in &mut synth.modules {
            if module.name() == "NoiseEffect" {
                // Utiliser le downcasting mutable pour modifier le noise
                if let Some(noise_module) = module.as_any_mut().downcast_mut::<Noise>() {
                    noise_module.set_amount(new_noise);
                    return;
                }
            }
        }
        println!("Module Noise non trouvé pour mise à jour");
    }

    pub fn set_filter_activation(&mut self, active: bool) {
        match self {
            SynthType::Sine(synth) => Self::set_filter_activation_static(synth, active),
            SynthType::Square(synth) => Self::set_filter_activation_static(synth, active),
            SynthType::Sawtooth(synth) => Self::set_filter_activation_static(synth, active),
            SynthType::FM(synth) => Self::set_filter_activation_static(synth, active),
            SynthType::Hammond(synth) => Self::set_filter_activation_static(synth, active),
        }
    }

    /// Active ou désactive le module gain
    pub fn set_gain_activation(&mut self, active: bool) {
        match self {
            SynthType::Sine(synth) => {
                Self::set_gain_activation_static(synth, active);
            }
            SynthType::Square(synth) => {
                Self::set_gain_activation_static(synth, active);
            }
            SynthType::Sawtooth(synth) => {
                Self::set_gain_activation_static(synth, active);
            }
            SynthType::FM(synth) => {
                Self::set_gain_activation_static(synth, active);
            }
            SynthType::Hammond(synth) => {
                Self::set_gain_activation_static(synth, active);
            }
        }
    }

    pub fn set_compressor_activation(&mut self, active: bool) {
        match self {
            SynthType::Sine(synth) => Self::set_compressor_activation_static(synth, active),
            SynthType::Square(synth) => Self::set_compressor_activation_static(synth, active),
            SynthType::Sawtooth(synth) => Self::set_compressor_activation_static(synth, active),
            SynthType::FM(synth) => Self::set_compressor_activation_static(synth, active),
            SynthType::Hammond(synth) => Self::set_compressor_activation_static(synth, active),
        }
    }

    pub fn set_reverb_activation(&mut self, active: bool) {
        match self {
            SynthType::Sine(synth) => Self::set_reverb_activation_static(synth, active),
            SynthType::Square(synth) => Self::set_reverb_activation_static(synth, active),
            SynthType::Sawtooth(synth) => Self::set_reverb_activation_static(synth, active),
            SynthType::FM(synth) => Self::set_reverb_activation_static(synth, active),
            SynthType::Hammond(synth) => Self::set_reverb_activation_static(synth, active),
        }
    }

    pub fn set_noise_activation(&mut self, active: bool) {
        match self {
            SynthType::Sine(synth) => Self::set_noise_activation_static(synth, active),
            SynthType::Square(synth) => Self::set_noise_activation_static(synth, active),
            SynthType::Sawtooth(synth) => Self::set_noise_activation_static(synth, active),
            SynthType::FM(synth) => Self::set_noise_activation_static(synth, active),
            SynthType::Hammond(synth) => Self::set_noise_activation_static(synth, active),
        }
    }

    pub fn set_lfo_activation(&mut self, active: bool) {
        match self {
            SynthType::Sine(synth) => Self::set_lfo_activation_static(synth, active),
            SynthType::Square(synth) => Self::set_lfo_activation_static(synth, active),
            SynthType::Sawtooth(synth) => Self::set_lfo_activation_static(synth, active),
            SynthType::FM(synth) => Self::set_lfo_activation_static(synth, active),
            SynthType::Hammond(synth) => Self::set_lfo_activation_static(synth, active),
        }
    }

    /// Vérifie si le module gain est actif
    pub fn is_gain_active(&self) -> bool {
        match self {
            SynthType::Sine(synth) => Self::is_gain_active_static(synth),
            SynthType::Square(synth) => Self::is_gain_active_static(synth),
            SynthType::Sawtooth(synth) => Self::is_gain_active_static(synth),
            SynthType::FM(synth) => Self::is_gain_active_static(synth),
            SynthType::Hammond(synth) => Self::is_gain_active_static(synth),
        }
    }

    pub fn is_noise_active(&self) -> bool {
        match self {
            SynthType::Sine(synth) => Self::is_noise_active_static(synth),
            SynthType::Square(synth) => Self::is_noise_active_static(synth),
            SynthType::Sawtooth(synth) => Self::is_noise_active_static(synth),
            SynthType::FM(synth) => Self::is_noise_active_static(synth),
            SynthType::Hammond(synth) => Self::is_noise_active_static(synth),
        }
    }

    pub fn is_lfo_active(&self) -> bool {
        match self {
            SynthType::Sine(synth) => Self::is_lfo_active_static(synth),
            SynthType::Square(synth) => Self::is_lfo_active_static(synth),
            SynthType::Sawtooth(synth) => Self::is_lfo_active_static(synth),
            SynthType::FM(synth) => Self::is_lfo_active_static(synth),
            SynthType::Hammond(synth) => Self::is_lfo_active_static(synth),
        }
    }

    pub fn is_filter_active(&self) -> bool {
        match self {
            SynthType::Sine(synth) => Self::is_filter_active_static(synth),
            SynthType::Square(synth) => Self::is_filter_active_static(synth),
            SynthType::Sawtooth(synth) => Self::is_filter_active_static(synth),
            SynthType::FM(synth) => Self::is_filter_active_static(synth),
            SynthType::Hammond(synth) => Self::is_filter_active_static(synth),
        }
    }

    /// Obtient la forme d'onde actuelle du LFO
    pub fn get_current_lfo_waveform(&self) -> LfoWaveform {
        match self {
            SynthType::Sine(synth) => Self::get_lfo_waveform_from_synth(synth),
            SynthType::Square(synth) => Self::get_lfo_waveform_from_synth(synth),
            SynthType::Sawtooth(synth) => Self::get_lfo_waveform_from_synth(synth),
            SynthType::FM(synth) => Self::get_lfo_waveform_from_synth(synth),
            SynthType::Hammond(synth) => Self::get_lfo_waveform_from_synth(synth),
        }
    }

    pub fn get_current_lfo_frequency(&self) -> f64 {
        match self {
            SynthType::Sine(synth) => Self::get_lfo_frequency_from_synth(synth),
            SynthType::Square(synth) => Self::get_lfo_frequency_from_synth(synth),
            SynthType::Sawtooth(synth) => Self::get_lfo_frequency_from_synth(synth),
            SynthType::FM(synth) => Self::get_lfo_frequency_from_synth(synth),
            SynthType::Hammond(synth) => Self::get_lfo_frequency_from_synth(synth),
        }
    }

    pub fn get_current_cutoff(&self) -> f64 {
        match self {
            SynthType::Sine(synth) => Self::get_filter_cutoff_from_synth(synth),
            SynthType::Square(synth) => Self::get_filter_cutoff_from_synth(synth),
            SynthType::Sawtooth(synth) => Self::get_filter_cutoff_from_synth(synth),
            SynthType::FM(synth) => Self::get_filter_cutoff_from_synth(synth),
            SynthType::Hammond(synth) => Self::get_filter_cutoff_from_synth(synth),
        }
    }

    pub fn get_current_resonance(&self) -> f64 {
        match self {
            SynthType::Sine(synth) => Self::get_filter_resonance_from_synth(synth),
            SynthType::Square(synth) => Self::get_filter_resonance_from_synth(synth),
            SynthType::Sawtooth(synth) => Self::get_filter_resonance_from_synth(synth),
            SynthType::FM(synth) => Self::get_filter_resonance_from_synth(synth),
            SynthType::Hammond(synth) => Self::get_filter_resonance_from_synth(synth),
        }
    }

    fn set_filter_activation_static<O: crate::synths::traits::Oscillator>(
        synth: &mut ModularSynth<O>,
        active: bool,
    ) {
        let has_filter = synth.modules.iter().any(|m| m.name() == "LowPassFilter");

        if active && !has_filter {
            let filter = LowPassFilter::new(
                constants::CURRENT_FILTER_CUTOFF,
                constants::CURRENT_FILTER_RESONANCE,
                constants::SAMPLE_RATE,
            );
            synth.add_module(filter);
            println!("Module Filter ajouté");
        } else if !active && has_filter {
            synth.modules.retain(|m| m.name() != "LowPassFilter");
            println!("Module Filter retiré");
        } else {
            println!("Aucune action nécessaire pour le filtre");
        }
    }

    /// Helper pour activer/désactiver le gain dans un synthétiseur
    fn set_gain_activation_static<O: crate::synths::traits::Oscillator>(
        synth: &mut ModularSynth<O>,
        active: bool,
    ) {
        let has_gain = synth.modules.iter().any(|m| m.name() == "Gain");

        if active && !has_gain {
            let gain = Gain::new(constants::CURRENT_GAIN);
            synth.add_module(gain);
            println!("Module Gain ajouté");
        } else if !active && has_gain {
            synth.modules.retain(|m| m.name() != "Gain");
            println!("Module Gain retiré");
        }
    }

    fn set_compressor_activation_static<O: crate::synths::traits::Oscillator>(
        synth: &mut ModularSynth<O>,
        active: bool,
    ) {
        let has_compressor = synth
            .modules
            .iter()
            .any(|m| m.name() == "SimpleRMSCompressor");

        if active && !has_compressor {
            let compressor = Compressor::new(
                constants::CURRENT_THRESHOLD,
                constants::CURRENT_RATIO,
                constants::CURRENT_ATTACK,
                constants::CURRENT_RELEASE,
                constants::CURRENT_MAKEUP_GAIN,
                constants::SAMPLE_RATE,
            );
            synth.add_module(compressor);
            println!("Module Compressor ajouté");
        } else if !active && has_compressor {
            synth.modules.retain(|m| m.name() != "SimpleRMSCompressor");
            println!("Module Compressor retiré");
        } else {
            println!("Aucune action nécessaire pour le compresseur");
        }
    }

    fn set_reverb_activation_static<O: crate::synths::traits::Oscillator>(
        synth: &mut ModularSynth<O>,
        active: bool,
    ) {
        let has_reverb = synth.modules.iter().any(|m| m.name() == "Reverb");

        if active && !has_reverb {
            let reverb = Reverb::new(
                constants::SAMPLE_RATE,
                constants::CURRENT_REVERB_TYPE,
                constants::CURRENT_DRY_WET,
                constants::CURRENT_REVERB_EARLY_GAIN,
                constants::CURRENT_REVERB_TAIL_GAIN,
                constants::CURRENT_REVERB_PRE_DELAY_MS,
            );
            synth.add_module(reverb);
            println!("Module Reverb ajouté");
        } else if !active && has_reverb {
            synth.modules.retain(|m| m.name() != "Reverb");
            println!("Module Reverb retiré");
        } else {
            println!("Aucune action nécessaire pour le reverb");
        }
    }

    fn set_noise_activation_static<O: crate::synths::traits::Oscillator>(
        synth: &mut ModularSynth<O>,
        active: bool,
    ) {
        let has_noise = synth.modules.iter().any(|m| m.name() == "NoiseEffect");

        if active && !has_noise {
            let noise = Noise::new(constants::CURRENT_NOISE);
            synth.add_module(noise);
            println!("Module Noise ajouté");
        } else if !active && has_noise {
            synth.modules.retain(|m| m.name() != "NoiseEffect");
            println!("Module Noise retiré");
        } else {
            println!("Aucune action nécessaire pour le noise");
        }
    }

    fn set_lfo_activation_static<O: crate::synths::traits::Oscillator>(
        synth: &mut ModularSynth<O>,
        active: bool,
    ) {
        let has_lfo = synth.modules.iter().any(|m| m.name() == "LFO");

        if active && !has_lfo {
            let lfo = LFO::new(
                constants::CURRENT_LFO_WAVEFORM,
                constants::CURRENT_LFO_FREQ,
                constants::SAMPLE_RATE,
            );
            synth.add_module(lfo);
            println!("Module LFO ajouté");
        } else if !active && has_lfo {
            synth.modules.retain(|m| m.name() != "LFO");
            println!("Module LFO retiré");
        } else {
            println!("Aucune action nécessaire pour le LFO");
        }
    }

    /// Helper pour vérifier si le gain est actif
    fn is_gain_active_static<O: crate::synths::traits::Oscillator>(
        synth: &ModularSynth<O>,
    ) -> bool {
        synth.modules.iter().any(|m| m.name() == "Gain")
    }

    fn is_noise_active_static<O: crate::synths::traits::Oscillator>(
        synth: &ModularSynth<O>,
    ) -> bool {
        synth.modules.iter().any(|m| m.name() == "NoiseEffect")
    }

    fn is_lfo_active_static<O: crate::synths::traits::Oscillator>(synth: &ModularSynth<O>) -> bool {
        synth.modules.iter().any(|m| m.name() == "LFO")
    }

    fn is_filter_active_static<O: crate::synths::traits::Oscillator>(
        synth: &ModularSynth<O>,
    ) -> bool {
        synth.modules.iter().any(|m| m.name() == "LowPassFilter")
    }

    /// Helper pour récupérer la forme d'onde du LFO
    fn get_lfo_waveform_from_synth<O: crate::synths::traits::Oscillator>(
        synth: &ModularSynth<O>,
    ) -> LfoWaveform {
        for module in &synth.modules {
            if let Some(lfo) = module.as_any().downcast_ref::<LFO>() {
                return lfo.get_waveform();
            }
        }
        // Valeur par défaut si LFO pas trouvé
        constants::CURRENT_LFO_WAVEFORM
    }

    fn get_lfo_frequency_from_synth<O: crate::synths::traits::Oscillator>(
        synth: &ModularSynth<O>,
    ) -> f64 {
        for module in &synth.modules {
            if let Some(lfo) = module.as_any().downcast_ref::<LFO>() {
                return lfo.get_freq();
            }
        }
        constants::CURRENT_LFO_FREQ
    }

    fn get_filter_cutoff_from_synth<O: crate::synths::traits::Oscillator>(
        synth: &ModularSynth<O>,
    ) -> f64 {
        for module in &synth.modules {
            if let Some(filter) = module.as_any().downcast_ref::<LowPassFilter>() {
                return filter.get_cutoff_freq();
            }
        }
        constants::CURRENT_FILTER_CUTOFF
    }

    fn get_filter_resonance_from_synth<O: crate::synths::traits::Oscillator>(
        synth: &ModularSynth<O>,
    ) -> f64 {
        for module in &synth.modules {
            if let Some(filter) = module.as_any().downcast_ref::<LowPassFilter>() {
                return filter.get_resonance();
            }
        }
        constants::CURRENT_FILTER_RESONANCE
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

        let compressor: Compressor = Compressor::new(
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
        synth.add_module(noise);

        synth.add_module(lfo);

        if constants::ACTIVATION_FILTER {
            synth.add_module(filter);
        }
        synth.add_module(gain);

        if constants::ACTIVATION_COMPRESSOR {
            synth.add_module(compressor);
        }
        synth.add_module(reverb);

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
        synth.add_module(noise);

        synth.add_module(lfo);

        if constants::ACTIVATION_FILTER {
            synth.add_module(filter);
        }
        synth.add_module(gain);

        if constants::ACTIVATION_COMPRESSOR {
            synth.add_module(compressor);
        }
        synth.add_module(reverb);

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
        synth.add_module(noise);

        synth.add_module(lfo);

        if constants::ACTIVATION_FILTER {
            synth.add_module(filter);
        }
        synth.add_module(gain);

        if constants::ACTIVATION_COMPRESSOR {
            synth.add_module(compressor);
        }
        synth.add_module(reverb);

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
        synth.add_module(noise);

        synth.add_module(lfo);

        if constants::ACTIVATION_FILTER {
            synth.add_module(filter);
        }
        synth.add_module(gain);

        if constants::ACTIVATION_COMPRESSOR {
            synth.add_module(compressor);
        }
        synth.add_module(reverb);

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
        synth.add_module(noise);

        synth.add_module(lfo);

        if constants::ACTIVATION_FILTER {
            synth.add_module(filter);
        }
        synth.add_module(gain);

        if constants::ACTIVATION_COMPRESSOR {
            synth.add_module(compressor);
        }
        synth.add_module(reverb);

        SynthType::Hammond(synth)
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
}
