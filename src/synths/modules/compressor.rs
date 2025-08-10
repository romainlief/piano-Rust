use crate::synths::traits::Module;

#[derive(Clone)]
pub struct Compressor {
    threshold_db: f64,
    ratio: f64,
    attack_coeff: f64,
    release_coeff: f64,
    makeup_gain_db: f64,
    sample_rate: f64,

    envelope_db: f64, // niveau d’enveloppe lissé (en dB)
}

impl Compressor {
    pub fn new(
        threshold_db: f64,
        ratio: f64,
        attack_sec: f64,
        release_sec: f64,
        makeup_gain_db: f64,
        sample_rate: f64,
    ) -> Self {
        let attack_coeff = (-1.0 / (attack_sec * sample_rate)).exp();
        let release_coeff = (-1.0 / (release_sec * sample_rate)).exp();
        Self {
            threshold_db,
            ratio,
            attack_coeff,
            release_coeff,
            makeup_gain_db,
            sample_rate,
            envelope_db: -120.0,
        }
    }

    #[inline]
    fn linear_to_db(linear: f64) -> f64 {
        20.0 * linear.max(1e-12).log10()
    }

    #[inline]
    fn db_to_linear(db: f64) -> f64 {
        10f64.powf(db / 20.0)
    }

    fn gain_reduction(&self, input_db: f64) -> f64 {
        if input_db > self.threshold_db {
            (input_db - self.threshold_db) * (1.0 - 1.0 / self.ratio)
        } else {
            0.0
        }
    }
}

impl Module for Compressor {
    fn process(&mut self, input: f64, _time: f64) -> f64 {
        let input_level = input.abs();
        let input_db = Self::linear_to_db(input_level);

        let target_reduction = self.gain_reduction(input_db);

        // Attack/Release smoothing correct sur l'enveloppe de réduction
        if target_reduction > self.envelope_db {
            // Attack - transition rapide vers plus de compression
            self.envelope_db = target_reduction + self.attack_coeff * (self.envelope_db - target_reduction);
        } else {
            // Release - transition lente vers moins de compression
            self.envelope_db = target_reduction + self.release_coeff * (self.envelope_db - target_reduction);
        }

        // Calcul gain linéaire final (compression + makeup)
        let gain_db = -self.envelope_db + self.makeup_gain_db;
        let gain = Self::db_to_linear(gain_db);

        // Limiter le gain pour éviter les explosions
        let safe_gain = gain.min(4.0).max(0.01); // Entre 0.01 et 4.0

        // Applique gain en préservant le signe
        input * safe_gain
    }

    fn name(&self) -> &'static str {
        "SimpleCompressor"
    }

    fn clone_box(&self) -> Box<dyn Module> {
        Box::new(self.clone())
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
