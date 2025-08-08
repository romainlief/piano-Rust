pub trait Synthesizer {
    fn generate_sample(&self, phase: f64, frequency: f64) -> f64;
    fn name(&self) -> &'static str;
}