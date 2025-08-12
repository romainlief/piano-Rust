use crate::synths::traits::Module;

pub enum ReverbType {
    Hall,
    Room,
    Plate,
    Spring,
    Chamber,
}

#[derive(Clone, Copy)]
pub struct Reverb {
    reverb_type: ReverbType,
}

impl Reverb {
    pub fn new(reverb_type: ReverbType) -> Self {
        Self { reverb_type }
    }

    /// #### Setters ####
    pub fn set_reverb_type(&mut self, reverb_type: ReverbType) {
        self.reverb_type = reverb_type;
    }

    /// #### Getters ####
    pub fn reverb_type(&self) -> ReverbType {
        self.reverb_type
    }
}

impl Module for Reverb {
    fn process(&mut self, input: f64, _time: f64) -> f64 {
        input
    }

    fn name(&self) -> &'static str {
        "Reverb"
    }

    fn clone_box(&self) -> Box<dyn Module> {
        Box::new(*self)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
