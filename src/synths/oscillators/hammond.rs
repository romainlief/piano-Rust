use crate::synths::traits::Oscillator;

#[derive(Clone, Copy)]
pub struct HammondOscillator;

impl Oscillator for HammondOscillator {
    fn sample(&self, phase: f64) -> f64 {
        let mut sample = 0.0;
        // 16' - Sub-harmonic (octave plus bas)
        sample += (phase * 0.5).sin() * 0.6;
        // 5 1/3' - 3ème harmonique
        sample += (phase * 3.0).sin() * 0.4;
        // 8' - Fondamentale
        sample += phase.sin() * 0.8;
        // 4' - 2ème harmonique (octave plus haut)
        sample += (phase * 2.0).sin() * 0.5;
        // 2 2/3' - 3ème harmonique octave
        sample += (phase * 6.0).sin() * 0.3;
        // 2' - 4ème harmonique
        sample += (phase * 4.0).sin() * 0.25;
        // 1 3/5' - 5ème harmonique
        sample += (phase * 5.0).sin() * 0.2;
        // 1 1/3' - 6ème harmonique
        sample += (phase * 8.0).sin() * 0.15;
        // 1' - 8ème harmonique
        sample += (phase * 8.0).sin() * 0.1;
        sample * 0.3 // Evite la saturation
    }

    fn name(&self) -> &'static str {
        "Hammond"
    }
}
