use crate::synths::traits::Module;

#[derive(Clone, Copy)]
pub struct ADSR {
    pub attack: f64,  // seconds
    pub decay: f64,   // seconds
    pub sustain: f64, // 0..1
    pub release: f64, // seconds
    pub gate: bool, 
    pub time: f64, // seconds since note_on or since release
    pub released_at: Option<f64>,
}
