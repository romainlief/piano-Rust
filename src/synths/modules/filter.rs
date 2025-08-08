use crate::synths::traits::Module;

#[derive(Clone, Copy)]
pub struct Filter {
    cutoff: f64,
    resonance: f64,
}
