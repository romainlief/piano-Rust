pub trait Synthesizer: Send + Sync {
    fn generate_sample(&self, phase: f64, frequency: f64) -> f64;
    fn name(&self) -> &'static str;
}

pub trait Oscillator: Send + Sync + Clone + Copy {
    fn sample(&self, phase: f64) -> f64;
    fn name(&self) -> &'static str;
}

pub trait Module: Send + Sync {
    fn process(&mut self, input: f64, time: f64) -> f64;
    fn name(&self) -> &'static str;
    fn clone_box(&self) -> Box<dyn Module>;
}
