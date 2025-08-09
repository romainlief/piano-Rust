use crate::synths::traits::Module;
use std::f64::consts::PI;

/// Type of LFO waveforms
#[derive(Debug, Clone, Copy)]
pub enum LfoWaveform {
    Sine,
    Triangle,
    Square,
    SawUp,
    SawDown,
}

#[derive(Clone, Copy)]
pub struct LFO {
    waveform: LfoWaveform,
    freq: f64,        // en Hz
    sample_rate: f64, // échantillons par seconde
    phase: f64,       // 0..1
    amplitude: f64,   // amplitude de sortie
    offset: f64,      // décalage de sortie
    bipolar: bool,    // true = -1..1, false = 0..1
}

impl LFO {
    pub fn new(waveform: LfoWaveform, freq: f64, sample_rate: f64) -> Self {
        Self {
            waveform,
            freq,
            sample_rate,
            phase: 0.0,
            amplitude: 1.0,
            offset: 0.0,
            bipolar: true,
        }
    }

    /// Calculates the current waveform value based on the phase and waveform type.
    fn waveform_value(&self) -> f64 {
        match self.waveform {
            LfoWaveform::Sine => (self.phase * 2.0 * PI).sin(),
            LfoWaveform::Triangle => {
                if self.phase < 0.25 {
                    self.phase * 4.0
                } else if self.phase < 0.75 {
                    2.0 - self.phase * 4.0
                } else {
                    self.phase * 4.0 - 4.0
                }
            }
            LfoWaveform::Square => {
                if self.phase < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            LfoWaveform::SawUp => 2.0 * self.phase - 1.0,
            LfoWaveform::SawDown => 1.0 - 2.0 * self.phase,
        }
    }

    /// #### Setters ####
    pub fn set_waveform(&mut self, waveform: LfoWaveform) {
        self.waveform = waveform;
    }

    pub fn set_freq(&mut self, freq: f64) {
        self.freq = freq;
    }

    pub fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
    }

    pub fn set_amplitude(&mut self, amplitude: f64) {
        self.amplitude = amplitude;
    }

    pub fn set_offset(&mut self, offset: f64) {
        self.offset = offset;
    }

    pub fn set_bipolar(&mut self, bipolar: bool) {
        self.bipolar = bipolar;
    }

    /// #### Getters ####
    pub fn get_waveform(&self) -> LfoWaveform {
        self.waveform
    }

    pub fn get_freq(&self) -> f64 {
        self.freq
    }

    pub fn get_sample_rate(&self) -> f64 {
        self.sample_rate
    }

    pub fn get_amplitude(&self) -> f64 {
        self.amplitude
    }

    pub fn get_offset(&self) -> f64 {
        self.offset
    }

    pub fn get_bipolar(&self) -> bool {
        self.bipolar
    }
}

impl Module for LFO {
    fn process(&mut self, input: f64, _time: f64) -> f64 {
        let mut signal = self.waveform_value();

        if !self.bipolar {
            signal = (signal + 1.0) * 0.5;
        }

        // Adding offset and scaling by amplitude
        let output = signal * self.amplitude + self.offset;

        // Phase advancement
        self.phase += self.freq / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        // Applying the LFO modulation to the input signal
        input * (1.0 + output)
    }

    fn name(&self) -> &'static str {
        "LFO"
    }

    fn clone_box(&self) -> Box<dyn Module> {
        Box::new(*self)
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
