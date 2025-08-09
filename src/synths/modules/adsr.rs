use crate::synths::traits::Module;

#[derive(Debug, Clone, Copy)]
pub enum EnvelopeCurve {
    Linear,
    Exponential,
}

#[derive(Debug, Clone, Copy)]
pub enum EnvelopeStage {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Clone, Copy)]
pub struct ADSR {
    attack: f64,  // en secondes
    decay: f64,   // en secondes
    sustain: f64, // 0..1
    release: f64, // en secondes
    sample_rate: f64,
    curve: EnvelopeCurve,
    stage: EnvelopeStage,
    level: f64, // Niveau actuel de l'enveloppe
    step: f64,  // Incrément par échantillon
}

impl ADSR {
    pub fn new(sample_rate: f64) -> Self {
        Self {
            attack: 0.5,
            decay: 0.1,
            sustain: 0.7,
            release: 0.2,
            sample_rate,
            curve: EnvelopeCurve::Linear,
            stage: EnvelopeStage::Idle,
            level: 0.0,
            step: 0.0,
        }
    }

    fn calc_step(&mut self, target: f64, time: f64) {
        if time <= 0.0 {
            self.level = target;
            self.step = 1.0;
            return;
        }
        let samples = time * self.sample_rate;
        match self.curve {
            EnvelopeCurve::Linear => {
                self.step = (target - self.level) / samples;
            }
            EnvelopeCurve::Exponential => {
                // Assure-toi que level > 0 pour la puissance
                let start = self.level.max(1e-6);
                self.step = (target / start).powf(1.0 / samples);
            }
        }
    }

    pub fn note_on(&mut self) {
        self.stage = EnvelopeStage::Attack;
        // Reset à un petit niveau non nul si exponentiel pour éviter la stagnation
        if let EnvelopeCurve::Exponential = self.curve {
            if self.level < 1e-6 {
                self.level = 1e-6;
            }
        } else {
            self.level = 0.0;
        }
        self.calc_step(1.0, self.attack);
    }

    pub fn note_off(&mut self) {
        self.stage = EnvelopeStage::Release;
        self.calc_step(0.0, self.release);
    }

    fn advance(&mut self) {
        match self.stage {
            EnvelopeStage::Attack => match self.curve {
                EnvelopeCurve::Linear => {
                    self.level += self.step;
                    if self.level >= 1.0 {
                        self.level = 1.0;
                        self.stage = EnvelopeStage::Decay;
                        self.calc_step(self.sustain, self.decay);
                    }
                }
                EnvelopeCurve::Exponential => {
                    self.level *= self.step;
                    if self.level >= 1.0 {
                        self.level = 1.0;
                        self.stage = EnvelopeStage::Decay;
                        self.calc_step(self.sustain, self.decay);
                    }
                }
            },
            EnvelopeStage::Decay => match self.curve {
                EnvelopeCurve::Linear => {
                    self.level += self.step;
                    if self.level <= self.sustain {
                        self.level = self.sustain;
                        self.stage = EnvelopeStage::Sustain;
                    }
                }
                EnvelopeCurve::Exponential => {
                    self.level *= self.step;
                    if self.level <= self.sustain {
                        self.level = self.sustain;
                        self.stage = EnvelopeStage::Sustain;
                    }
                }
            },
            EnvelopeStage::Sustain => {
                // Stable
            }
            EnvelopeStage::Release => match self.curve {
                EnvelopeCurve::Linear => {
                    self.level += self.step;
                    if self.level <= 0.0 {
                        self.level = 0.0;
                        self.stage = EnvelopeStage::Idle;
                    }
                }
                EnvelopeCurve::Exponential => {
                    self.level *= self.step;
                    if self.level <= 0.0001 {
                        self.level = 0.0;
                        self.stage = EnvelopeStage::Idle;
                    }
                }
            },
            EnvelopeStage::Idle => {}
        }
    }
    // #### Setters ####
    pub fn set_attack(&mut self, seconds: f64) {
        self.attack = seconds.max(0.0);
    }

    pub fn set_decay(&mut self, seconds: f64) {
        self.decay = seconds.max(0.0);
    }

    pub fn set_sustain(&mut self, level: f64) {
        self.sustain = level.clamp(0.0, 1.0);
    }

    pub fn set_release(&mut self, seconds: f64) {
        self.release = seconds.max(0.0);
    }

    pub fn set_curve(&mut self, curve: EnvelopeCurve) {
        self.curve = curve;
    }

    pub fn set_sample_rate(&mut self, sr: f64) {
        self.sample_rate = sr;
    }

    // #### Getters ####
    pub fn get_attack(&self) -> f64 {
        self.attack
    }

    pub fn get_decay(&self) -> f64 {
        self.decay
    }

    pub fn get_sustain(&self) -> f64 {
        self.sustain
    }

    pub fn get_release(&self) -> f64 {
        self.release
    }

    pub fn get_curve(&self) -> EnvelopeCurve {
        self.curve
    }

    pub fn get_sample_rate(&self) -> f64 {
        self.sample_rate
    }

    pub fn get_stage(&self) -> EnvelopeStage {
        self.stage
    }

    pub fn get_level(&self) -> f64 {
        self.level
    }

    pub fn get_step(&self) -> f64 {
        self.step
    }
}

impl Module for ADSR {
    fn process(&mut self, input: f64, _time: f64) -> f64 {
        self.advance();
        input * self.level
    }

    fn name(&self) -> &'static str {
        "ADSR"
    }

    fn clone_box(&self) -> Box<dyn Module> {
        Box::new(*self)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
