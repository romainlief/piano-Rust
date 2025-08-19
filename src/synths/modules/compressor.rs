use crate::synths::traits::Module;

#[derive(Clone)]
pub struct Compressor {
    threshold_db: f64,
    ratio: f64,
    attack_coeff: f64,
    release_coeff: f64,
    makeup_gain_db: f64,
    _sample_rate: f64,

    // Pour calcul RMS
    rms_buffer: Vec<f64>,
    rms_sum: f64,
    rms_index: usize,
    rms_window_size: usize,

    envelope_db: f64,
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

        let rms_window_size = (0.03 * sample_rate) as usize; // 30 ms window

        Self {
            threshold_db,
            ratio,
            attack_coeff,
            release_coeff,
            makeup_gain_db,
            _sample_rate: sample_rate,
            rms_buffer: vec![0.0; rms_window_size],
            rms_sum: 0.0,
            rms_index: 0,
            rms_window_size,
            envelope_db: -120.0,
        }
    }

    fn linear_to_db(linear: f64) -> f64 {
        20.0 * linear.max(1e-12).log10()
    }

    fn db_to_linear(db: f64) -> f64 {
        10f64.powf(db / 20.0)
    }

    // Met à jour le RMS avec le nouvel échantillon et retourne le RMS courant
    fn update_rms(&mut self, sample: f64) -> f64 {
        let sq = sample * sample;
        self.rms_sum -= self.rms_buffer[self.rms_index];
        self.rms_sum += sq;
        self.rms_buffer[self.rms_index] = sq;
        self.rms_index = (self.rms_index + 1) % self.rms_window_size;

        (self.rms_sum / self.rms_window_size as f64).sqrt()
    }

    fn gain_reduction(&self, input_db: f64) -> f64 {
        if input_db > self.threshold_db {
            (input_db - self.threshold_db) * (1.0 - 1.0 / self.ratio)
        } else {
            0.0
        }
    }

    // #### Getters ####
    pub fn get_threshold(&self) -> f64 {
        self.threshold_db
    }

    // #### Setters ####
    pub fn set_threshold(&mut self, new_threshold: f64) {
        self.threshold_db = new_threshold;
    }
}

impl Module for Compressor {
    fn process(&mut self, input: f64, _time: f64) -> f64 {
        // Calcul du niveau RMS sur la fenêtre glissante
        let rms = self.update_rms(input.abs());
        let input_db = Self::linear_to_db(rms);

        let target_reduction = self.gain_reduction(input_db);

        // Attack/Release smoothing de la réduction de gain
        if target_reduction > self.envelope_db {
            self.envelope_db =
                target_reduction + self.attack_coeff * (self.envelope_db - target_reduction);
        } else {
            self.envelope_db =
                target_reduction + self.release_coeff * (self.envelope_db - target_reduction);
        }

        // Gain final avec make-up
        let gain_db = -self.envelope_db + self.makeup_gain_db;
        let gain = Self::db_to_linear(gain_db);

        // Applique le gain sur le signal d'entrée (préserve la phase)
        input * gain
    }

    fn name(&self) -> &'static str {
        "SimpleRMSCompressor"
    }

    fn clone_box(&self) -> Box<dyn Module> {
        Box::new(self.clone())
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
