use crate::synths::traits::Module;

#[derive(Clone)]
pub struct Compressor {
    threshold: f64,   // in dB
    ratio: f64,       // compression ratio
    attack: f64,      // attack time in seconds
    release: f64,     // release time in seconds
    makeup_gain: f64, // gain applied after compression in dB
    knee: f64,        // dB, taille du "soft knee"
    sample_rate: f64,

    // États internes
    envelope_db: f64, // niveau RMS lissé en dB
    rms_sum: f64,     // somme des carrés pour RMS
    rms_window_size: usize,
    rms_buffer: Vec<f64>,
    rms_index: usize,
}

impl Compressor {
    pub fn new(
        threshold: f64,
        ratio: f64,
        attack: f64,
        release: f64,
        makeup_gain: f64,
        knee: f64,
        sample_rate: f64,
    ) -> Self {
        let window_size = (0.025 * sample_rate) as usize; // 25ms RMS window    
        Self {
            threshold,
            ratio,
            attack,
            release,
            makeup_gain,
            knee,
            sample_rate,
            envelope_db: -120.0,
            rms_sum: 0.0,
            rms_window_size: window_size,
            rms_buffer: vec![0.0; window_size],
            rms_index: 0,
        }
    }

    #[inline]
    fn db_to_linear(db: f64) -> f64 {
        10f64.powf(db / 20.0)
    }

    #[inline]
    fn linear_to_db(linear: f64) -> f64 {
        20.0 * linear.max(1e-12).log10()
    }

    // Calcul RMS sur un buffer circulaire
    fn update_rms(&mut self, sample: f64) -> f64 {
        let sq = sample * sample;
        self.rms_sum -= self.rms_buffer[self.rms_index];
        self.rms_sum += sq;
        self.rms_buffer[self.rms_index] = sq;
        self.rms_index = (self.rms_index + 1) % self.rms_window_size;

        let mean_square = self.rms_sum / self.rms_window_size as f64;
        mean_square.sqrt()
    }

    // Soft knee curve
    fn soft_knee_gain_reduction(&self, level_db: f64) -> f64 {
        let half_knee = self.knee / 2.0;
        if self.knee > 0.0 {
            if level_db < self.threshold - half_knee {
                0.0
            } else if level_db > self.threshold + half_knee {
                (level_db - self.threshold) * (1.0 - 1.0 / self.ratio)
            } else {
                // transition douce
                let x = level_db - (self.threshold - half_knee);
                let y = x / (self.knee);
                (y * y) * (1.0 - 1.0 / self.ratio) * (level_db - self.threshold + half_knee)
            }
        } else {
            if level_db > self.threshold {
                (level_db - self.threshold) * (1.0 - 1.0 / self.ratio)
            } else {
                0.0
            }
        }
    }

    /// #### Setters ####
    pub fn set_threshold(&mut self, threshold: f64) {
        self.threshold = threshold;
    }

    pub fn set_ratio(&mut self, ratio: f64) {
        self.ratio = ratio;
    }

    pub fn set_attack(&mut self, attack: f64) {
        self.attack = attack;
    }

    pub fn set_release(&mut self, release: f64) {
        self.release = release;
    }

    pub fn set_makeup_gain(&mut self, makeup_gain: f64) {
        self.makeup_gain = makeup_gain;
    }

    pub fn set_knee(&mut self, knee: f64) {
        self.knee = knee;
    }

    pub fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
    }

    /// #### Getters ####
    pub fn threshold(&self) -> f64 {
        self.threshold
    }

    pub fn ratio(&self) -> f64 {
        self.ratio
    }

    pub fn attack(&self) -> f64 {
        self.attack
    }

    pub fn release(&self) -> f64 {
        self.release
    }

    pub fn makeup_gain(&self) -> f64 {
        self.makeup_gain
    }

    pub fn knee(&self) -> f64 {
        self.knee
    }

    pub fn sample_rate(&self) -> f64 {
        self.sample_rate
    }
}

impl Module for Compressor {
    fn process(&mut self, input: f64, _time: f64) -> f64 {
        // RMS lissé
        let rms_level = self.update_rms(input);
        let level_db = Self::linear_to_db(rms_level);

        // Gain reduction avec soft knee
        let target_reduction = self.soft_knee_gain_reduction(level_db);

        // Attack/Release
        let coeff_attack = (-1.0 / (self.attack * self.sample_rate)).exp();
        let coeff_release = (-1.0 / (self.release * self.sample_rate)).exp();

        if target_reduction > self.envelope_db {
            self.envelope_db =
                coeff_attack * (self.envelope_db - target_reduction) + target_reduction;
        } else {
            self.envelope_db =
                coeff_release * (self.envelope_db - target_reduction) + target_reduction;
        }

        // Application du gain et make-up
        let output_gain = Self::db_to_linear(-self.envelope_db + self.makeup_gain);
        input * output_gain
    }

    fn name(&self) -> &'static str {
        "AdvancedCompressor"
    }

    fn clone_box(&self) -> Box<dyn Module> {
        Box::new(self.clone())
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
