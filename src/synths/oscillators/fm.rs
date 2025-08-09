use crate::synths::traits::Oscillator;

#[derive(Clone, Copy)]
pub struct FmOscillator {
    mod_index: f64, // Intensité modulation
    mod_ratio: f64, // Ratio fréquence modulateur/porteur
}

impl FmOscillator {
    pub fn new(mod_index: f64, mod_ratio: f64) -> Self {
        Self {
            mod_index,
            mod_ratio,
        }
    }

    // #### Setters ####
    pub fn set_mod_index(&mut self, mod_index: f64) {
        self.mod_index = mod_index;
    }

    pub fn set_mod_ratio(&mut self, mod_ratio: f64) {
        self.mod_ratio = mod_ratio;
    }

    // #### Getters ####
    pub fn get_mod_index(&self) -> f64 {
        self.mod_index
    }

    pub fn get_mod_ratio(&self) -> f64 {
        self.mod_ratio
    }
}

impl Oscillator for FmOscillator {
    fn sample(&self, phase: f64) -> f64 {
        // Module phase du carrier (pas l'amplitude)
        let modulator_phase = phase * self.mod_ratio;
        let modulation = (modulator_phase).sin() * self.mod_index;

        // La phase modulée du carrier
        let modulated_phase = phase + modulation;

        // Signal carrier avec modulation de phase
        let carrier = modulated_phase.sin();

        let harmonic = (modulated_phase * 2.0).sin() * 0.3;

        // Mélange avec saturation douce
        let output = carrier + harmonic;

        if output.abs() > 0.7 {
            output.signum() * (0.7 + (output.abs() - 0.7) * 0.3)
        } else {
            output
        }
    }

    fn name(&self) -> &'static str {
        "FM"
    }
}
