use crate::synths::traits::Module;

#[derive(Clone, Copy)]
/// Types of reverb
pub enum ReverbType {
    Hall,
    Room,
    Plate,
    Spring,
    Shimmer, // a faire mieux si possible
}

/// Comb filter amorti (damping) pour la queue
#[derive(Clone)]
struct DampedComb {
    buf: Vec<f64>,
    idx: usize,
    feedback: f64,
    damping: f64, // (0..1) low-pass dans la boucle
    lowpass_state: f64,
}

impl DampedComb {
    fn new(len_samples: usize, feedback: f64, damping: f64) -> Self {
        Self {
            buf: vec![0.0; len_samples.max(1)],
            idx: 0,
            feedback,
            damping,
            lowpass_state: 0.0,
        }
    }

    #[inline]
    fn process(&mut self, input: f64) -> f64 {
        let y = self.buf[self.idx];
        // Low-pass dans la boucle de feedback (damping)
        self.lowpass_state = (1.0 - self.damping) * y + self.damping * self.lowpass_state;
        let fb = self.lowpass_state * self.feedback + input;
        self.buf[self.idx] = fb;
        self.idx += 1;
        if self.idx >= self.buf.len() {
            self.idx = 0;
        }
        y
    }
}

/// All-pass diffuser pour lisser/épaissir la queue
#[derive(Clone)]
struct Allpass {
    buf: Vec<f64>,
    idx: usize,
    feedback: f64, // généralement ~0.5..0.8
}

impl Allpass {
    fn new(len_samples: usize, feedback: f64) -> Self {
        Self {
            buf: vec![0.0; len_samples.max(1)],
            idx: 0,
            feedback,
        }
    }

    #[inline]
    fn process(&mut self, input: f64) -> f64 {
        let buf_out = self.buf[self.idx];
        let y = -input + buf_out;
        let new_val = input + buf_out * self.feedback;
        self.buf[self.idx] = new_val;
        self.idx += 1;
        if self.idx >= self.buf.len() {
            self.idx = 0;
        }
        y
    }
}

#[derive(Clone)]
pub struct Reverb {
    reverb_type: ReverbType,

    // Mix & paramètres globaux
    dry_wet: f64,      // 0 = dry, 1 = wet
    early_gain: f64,   // gain des early reflections (convolution)
    tail_gain: f64,    // gain de la queue (algo)
    pre_delay_ms: f64, // pré-délai avant early+tail
    sample_rate: f64,

    // --- Convolution (early reflections) ---
    ir: Vec<f64>,      // impulse response (mono)
    ir_hist: Vec<f64>, // tampon circulaire pour la convolution directe
    ir_pos: usize,

    // pré-délai (buffer circulaire)
    predelay_buf: Vec<f64>,
    predelay_pos: usize,

    combs: Vec<DampedComb>,  // combs en parallèle
    allpasses: Vec<Allpass>, // allpass en série
}

impl Reverb {
    pub fn new(
        sample_rate: f64,
        reverb_type: ReverbType,
        dry_wet: f64,
        early_gain: f64,
        tail_gain: f64,
        pre_delay_ms: f64,
    ) -> Self {
        let mut reverb = Self {
            reverb_type,
            dry_wet, // mix de base
            early_gain,
            tail_gain,
            pre_delay_ms, // pré-délai léger par défaut
            sample_rate,

            ir: Vec::new(),
            ir_hist: vec![0.0; 1],
            ir_pos: 0,

            predelay_buf: vec![0.0; 1],
            predelay_pos: 0,

            combs: Vec::new(),
            allpasses: Vec::new(),
        };
        reverb.configure_by_type(reverb_type);
        reverb.rebuild_predelay();
        reverb
    }

    /// Configure quelques paramètres & tailles de lignes selon le type
    fn configure_by_type(&mut self, rtype: ReverbType) {
        // Delais "classiques" (ms) inspirés de Freeverb/Schroeder
        let (comb_ms, ap_ms, feedback, damping, dry_wet, early_gain, tail_gain, pre_delay) =
            match rtype {
                ReverbType::Room => (
                    vec![29.7, 37.1, 41.1, 43.7], // plus court
                    vec![5.0, 1.7],
                    0.72,
                    0.25,
                    0.20,
                    0.8,
                    0.8,
                    8.0,
                ),
                ReverbType::Plate => (
                    vec![31.0, 36.0, 40.0, 44.0],
                    vec![6.3, 2.1],
                    0.78,
                    0.35,
                    0.25,
                    0.9,
                    0.95,
                    10.0,
                ),
                ReverbType::Spring => (
                    vec![25.0, 31.0, 34.0, 38.0],
                    vec![7.1, 2.3],
                    0.70,
                    0.45,
                    0.22,
                    0.85,
                    0.9,
                    12.0,
                ),
                ReverbType::Hall => (
                    vec![29.7, 37.1, 41.1, 43.7], // plus long = +tail_gain/feedback
                    vec![8.0, 3.0],
                    0.80,
                    0.30,
                    0.30,
                    0.9,
                    0.95,
                    20.0,
                ),
                ReverbType::Shimmer => (
                    vec![29.7, 37.1, 41.1, 43.7],
                    vec![8.0, 3.0],
                    0.82,
                    0.25,
                    0.35,
                    0.9,
                    1.0,
                    25.0,
                ),
            };

        self.dry_wet = dry_wet;
        self.early_gain = early_gain;
        self.tail_gain = tail_gain;
        self.pre_delay_ms = pre_delay;

        // Construire combs
        self.combs.clear();
        for ms in comb_ms {
            let len = ((ms / 1000.0) * self.sample_rate).round() as usize;
            self.combs
                .push(DampedComb::new(len.max(1), feedback, damping));
        }
        // Construire allpasses
        self.allpasses.clear();
        for ms in ap_ms {
            let len = ((ms / 1000.0) * self.sample_rate).round() as usize;
            self.allpasses.push(Allpass::new(len.max(1), 0.7));
        }
    }

    fn rebuild_predelay(&mut self) {
        let len = ((self.pre_delay_ms / 1000.0) * self.sample_rate).round() as usize;
        self.predelay_buf = vec![0.0; len.max(1)];
        self.predelay_pos = 0;
    }

    /// Convolution directe naive O(N) (OK pour petites IR)
    #[inline]
    fn convolve_early(&mut self, x: f64) -> f64 {
        if self.ir.is_empty() {
            return 0.0;
        }
        // push dans l’historique circulaire
        self.ir_hist[self.ir_pos] = x;

        // y = sum hist[pos - i] * ir[i]
        let mut acc = 0.0;
        let mut pos = self.ir_pos;
        for coeff in &self.ir {
            acc += self.ir_hist[pos] * *coeff;
            if pos == 0 {
                pos = self.ir_hist.len() - 1;
            } else {
                pos -= 1;
            }
        }

        self.ir_pos += 1;
        if self.ir_pos >= self.ir_hist.len() {
            self.ir_pos = 0;
        }
        acc
    }

    /// Pré-délai simple via buffer circulaire
    #[inline]
    fn predelay(&mut self, x: f64) -> f64 {
        let y = self.predelay_buf[self.predelay_pos];
        self.predelay_buf[self.predelay_pos] = x;
        self.predelay_pos += 1;
        if self.predelay_pos >= self.predelay_buf.len() {
            self.predelay_pos = 0;
        }
        y
    }

    /// Trajet algorithmique (queue) : combs en // puis allpass en série
    #[inline]
    fn algo_tail(&mut self, x: f64) -> f64 {
        let mut sum = 0.0;
        for c in &mut self.combs {
            sum += c.process(x);
        }
        // normalisation simple par nb de combs
        if !self.combs.is_empty() {
            sum /= self.combs.len() as f64;
        }

        let mut y = sum;
        for ap in &mut self.allpasses {
            y = ap.process(y);
        }
        y
    }

    /// #### Setters ####

    /// Charge/remplace l’IR (mono). Idéalement courte (early reflections).
    pub fn set_ir(&mut self, ir: Vec<f64>) {
        self.ir = ir;
        let n = self.ir.len().max(1);
        self.ir_hist = vec![0.0; n];
        self.ir_pos = 0;
    }

    pub fn set_type(&mut self, rtype: ReverbType) {
        self.reverb_type = rtype;
        self.configure_by_type(rtype);
    }

    pub fn set_dry_wet(&mut self, v: f64) {
        self.dry_wet = v.clamp(0.0, 1.0);
    }
    pub fn set_pre_delay_ms(&mut self, ms: f64) {
        self.pre_delay_ms = ms.max(0.0);
        self.rebuild_predelay();
    }

    pub fn set_tail_damping(&mut self, d: f64) {
        for c in &mut self.combs {
            c.damping = d.clamp(0.0, 1.0);
        }
    }

    /// #### Getters ####
    pub fn reverb_type(&self) -> ReverbType {
        self.reverb_type
    }
}

impl Module for Reverb {
    #[inline]
    fn process(&mut self, input: f64, _time: f64) -> f64 {
        // Pré-délai commun à early+tail
        let delayed = self.predelay(input);

        // Early reflections (convolution courte)
        let early = self.convolve_early(delayed) * self.early_gain;

        // Queue algorithmique
        let tail = self.algo_tail(delayed) * self.tail_gain;

        // Somme des composantes de réverbe
        let wet = early + tail;

        // Mix dry/wet
        (1.0 - self.dry_wet) * input + self.dry_wet * wet
    }

    fn name(&self) -> &'static str {
        "Reverb"
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
